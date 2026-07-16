//! Wave-2 interaction sweep: the four kernel hard parts (universe levels,
//! quotients, struct eta, recursors) landed in the same release and interact.
//! Each test here drives a CROSSING of at least two of them end-to-end
//! through the public kernel API (`TypeChecker` / `Reducer` /
//! `add_inductive_family` / `check_constant_info`).
//!
//! (a) struct eta under a quotient (`Quot.mk r ⟨a,b⟩` patterns);
//! (b) K-reduction + proof irrelevance + struct eta ordering in def_eq
//!     (must terminate and agree with Lean);
//! (c) `Nat.rec` iota on a `NatLit` inside a universe-polymorphic
//!     definition (levels × recursors × literals);
//! (d) large-elimination decision consulting NORMALIZED levels (Prop
//!     detection via `is_zero` only after `imax(_, 0) → 0` normalization).

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal,
};
use oxilean_kernel::reduce::{Reducer, ReducibilityHint};
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_constant_info, init_builtin_env, BinderInfo, Environment, Expr,
    InductiveSpec, Level, Literal, Name, TypeChecker,
};

// ---------------------------------------------------------------------------
// Helpers (builtin names are FLAT single atoms; quotient names installed by
// `add_quot` are HIERARCHICAL — use `Name::from_str` for those).
// ---------------------------------------------------------------------------

fn builtin_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env must initialize");
    env
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn quot_const(name: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::from_str(name), levels)
}

fn app(f: Expr, args: &[Expr]) -> Expr {
    let mut r = f;
    for a in args {
        r = Expr::App(Node::new(r), Node::new(a.clone()));
    }
    r
}

fn pi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(cod),
    )
}

fn lam(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(body),
    )
}

fn proj(struct_name: &str, idx: u32, e: Expr) -> Expr {
    Expr::Proj(Name::str(struct_name), idx, Node::new(e))
}

fn nat() -> Expr {
    cnst("Nat")
}

fn nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn one() -> Level {
    Level::succ(Level::zero())
}

fn add_axiom(env: &mut Environment, name: &str, level_params: &[&str], ty: Expr) {
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        is_unsafe: false,
    }))
    .expect("add axiom");
}

fn whnf(env: &Environment, e: &Expr) -> Expr {
    Reducer::new().whnf_env(e, env)
}

/// `structure MyPair : Type := mk (fst snd : Nat)` derived through the
/// CHECKED recursor machinery (`add_inductive_family`), so struct eta runs
/// against a Wave-2-derived `InductiveVal`, not a hand-rolled one.
fn add_mypair(env: &mut Environment) {
    let spec = InductiveSpec::new(
        Name::str("MyPair"),
        Expr::Sort(one()),
        vec![(
            Name::str("MyPair.mk"),
            pi("fst", nat(), pi("snd", nat(), cnst("MyPair"))),
        )],
    );
    add_inductive_family(env, vec![], 0, vec![spec]).expect("MyPair must be accepted");
    assert!(
        env.is_structure_like(&Name::str("MyPair")),
        "derived MyPair must satisfy the strict structure-like predicate"
    );
}

// ---------------------------------------------------------------------------
// (a) struct eta under a quotient.
// ---------------------------------------------------------------------------

/// `Quot.mk r w ≡ Quot.mk r ⟨w.0, w.1⟩` — app congruence over `Quot.mk`
/// descends into the quotiented element, where `isDefEqEtaStruct` must fire
/// on a structure derived by the checked inductive machinery.
#[test]
fn quot_mk_of_eta_expanded_struct_is_def_eq() {
    let mut env = builtin_env();
    add_mypair(&mut env);
    add_axiom(
        &mut env,
        "r",
        &[],
        pi("a", cnst("MyPair"), pi("b", cnst("MyPair"), prop())),
    );
    add_axiom(&mut env, "w", &[], cnst("MyPair"));

    // MyPair : Sort 1, so Quot.mk is instantiated at u := 1.
    let quot_mk = |elem: Expr| {
        app(
            quot_const("Quot.mk", vec![one()]),
            &[cnst("MyPair"), cnst("r"), elem],
        )
    };
    let eta = app(
        cnst("MyPair.mk"),
        &[proj("MyPair", 0, cnst("w")), proj("MyPair", 1, cnst("w"))],
    );
    let t = quot_mk(cnst("w"));
    let s = quot_mk(eta);
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&t, &s),
        "struct eta must fire underneath Quot.mk application congruence"
    );
    // Negative control: swapped fields are NOT the eta expansion of w.
    let swapped = app(
        cnst("MyPair.mk"),
        &[proj("MyPair", 1, cnst("w")), proj("MyPair", 0, cnst("w"))],
    );
    let s_bad = quot_mk(swapped);
    let mut tc = TypeChecker::new(&env);
    assert!(
        !tc.is_def_eq(&t, &s_bad),
        "swapped projections must not be definitionally equal"
    );
}

/// `Quot.lift f h (Quot.mk r ⟨5, 7⟩)` iota-reduces through the quotient,
/// beta-reduces the lifted function, and finally proj-reduces the
/// constructor pattern: the whole pipeline lands on the literal `5`.
#[test]
fn quot_lift_of_ctor_pattern_reduces_through_proj() {
    let mut env = builtin_env();
    add_mypair(&mut env);
    add_axiom(
        &mut env,
        "r",
        &[],
        pi("a", cnst("MyPair"), pi("b", cnst("MyPair"), prop())),
    );
    // f : MyPair → Nat := fun p => p.0
    let f = lam("p", cnst("MyPair"), proj("MyPair", 0, Expr::BVar(0)));
    // h : ∀ (a b : MyPair), r a b → Eq Nat (f a) (f b) (opaque axiom).
    let h_ty = pi(
        "a",
        cnst("MyPair"),
        pi(
            "b",
            cnst("MyPair"),
            pi(
                "hr",
                app(cnst("r"), &[Expr::BVar(1), Expr::BVar(0)]),
                app(
                    Expr::Const(Name::str("Eq"), vec![one()]),
                    &[
                        nat(),
                        app(f.clone(), &[Expr::BVar(2)]),
                        app(f.clone(), &[Expr::BVar(1)]),
                    ],
                ),
            ),
        ),
    );
    add_axiom(&mut env, "h", &[], h_ty);
    let pair = app(cnst("MyPair.mk"), &[nat_lit(5), nat_lit(7)]);
    let q = app(
        quot_const("Quot.mk", vec![one()]),
        &[cnst("MyPair"), cnst("r"), pair],
    );
    let lift = app(
        quot_const("Quot.lift", vec![one(), one()]),
        &[cnst("MyPair"), cnst("r"), nat(), f, cnst("h"), q],
    );
    assert_eq!(
        whnf(&env, &lift),
        nat_lit(5),
        "Quot.lift ∘ Quot.mk ∘ ctor must reduce all the way to the field"
    );
}

// ---------------------------------------------------------------------------
// (b) K-reduction + proof irrelevance + struct eta ordering.
// ---------------------------------------------------------------------------

/// Two DIFFERENT stuck proofs of the same (defeq-refl) equation: each side
/// K-reduces to the minor premise, and def_eq must both terminate and
/// return true (proof irrelevance on the majors would also justify it —
/// either route must agree).
#[test]
fn k_reduction_and_proof_irrelevance_agree_and_terminate() {
    let mut env = builtin_env();
    let eq_nat = |a: Expr, b: Expr| app(Expr::Const(Name::str("Eq"), vec![one()]), &[nat(), a, b]);
    add_axiom(&mut env, "h1", &[], eq_nat(nat_lit(0), nat_lit(0)));
    add_axiom(&mut env, "h2", &[], eq_nat(nat_lit(0), nat_lit(0)));
    add_axiom(&mut env, "mval", &[], nat());
    let motive = lam(
        "b",
        nat(),
        lam("t", eq_nat(nat_lit(0), Expr::BVar(0)), nat()),
    );
    let rec_app = |h: Expr| {
        app(
            Expr::Const(Name::str("Eq.rec"), vec![one(), one()]),
            &[
                nat(),
                nat_lit(0),
                motive.clone(),
                cnst("mval"),
                nat_lit(0),
                h,
            ],
        )
    };
    let lhs = rec_app(cnst("h1"));
    let rhs = rec_app(cnst("h2"));
    assert_eq!(whnf(&env, &lhs), cnst("mval"), "K-reduction fires on h1");
    assert_eq!(whnf(&env, &rhs), cnst("mval"), "K-reduction fires on h2");
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&lhs, &rhs),
        "Eq.rec over distinct stuck proofs must be def-eq (K + proof irrelevance)"
    );
}

/// A Type-valued structure whose FIELDS are proofs: struct eta must fire on
/// the whole (the outer type is not a Prop, so proof irrelevance cannot
/// short-circuit), and proof irrelevance must then decide the projected
/// proof obligations. Pins the ordering (irrelevance → structural → eta)
/// without looping.
#[test]
fn struct_eta_over_proof_fields_uses_proof_irrelevance() {
    let mut env = builtin_env();
    add_axiom(&mut env, "A", &[], prop());
    add_axiom(&mut env, "B", &[], prop());
    add_axiom(&mut env, "pa", &[], cnst("A"));
    add_axiom(&mut env, "pb", &[], cnst("B"));
    // structure S : Type := mk (a : A) (b : B) — proofs as fields.
    let spec = InductiveSpec::new(
        Name::str("S"),
        Expr::Sort(one()),
        vec![(
            Name::str("S.mk"),
            pi("a", cnst("A"), pi("b", cnst("B"), cnst("S"))),
        )],
    );
    add_inductive_family(&mut env, vec![], 0, vec![spec]).expect("S must be accepted");
    add_axiom(&mut env, "z", &[], cnst("S"));

    // z ≡ S.mk z.0 z.1 (pure struct eta; fields match syntactically).
    let eta = app(
        cnst("S.mk"),
        &[proj("S", 0, cnst("z")), proj("S", 1, cnst("z"))],
    );
    let mut tc = TypeChecker::new(&env);
    assert!(tc.is_def_eq(&cnst("z"), &eta), "plain struct eta must hold");

    // S.mk pa pb ≡ S.mk z.0 z.1: constructor congruence descends into the
    // fields, where pa =?= z.0 is decided by PROOF IRRELEVANCE on a
    // projected proof (needs the Proj arm of quick type inference).
    let built = app(cnst("S.mk"), &[cnst("pa"), cnst("pb")]);
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&built, &eta),
        "proof irrelevance must decide projected proof fields"
    );
    // And therefore z itself is def-eq to a ctor of arbitrary proofs.
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("z"), &built),
        "eta expansion + proof irrelevance must compose"
    );
}

/// Prop-valued structure: proof irrelevance fires FIRST (before eta) on the
/// whole comparison — and must not loop with the eta rule also applicable.
#[test]
fn prop_struct_proof_irrelevance_beats_eta_without_looping() {
    let mut env = builtin_env();
    add_axiom(&mut env, "A", &[], prop());
    add_axiom(&mut env, "B", &[], prop());
    add_axiom(&mut env, "pa", &[], cnst("A"));
    add_axiom(&mut env, "pb", &[], cnst("B"));
    let spec = InductiveSpec::new(
        Name::str("MyAnd"),
        prop(),
        vec![(
            Name::str("MyAnd.intro"),
            pi("a", cnst("A"), pi("b", cnst("B"), cnst("MyAnd"))),
        )],
    );
    add_inductive_family(&mut env, vec![], 0, vec![spec]).expect("MyAnd must be accepted");
    add_axiom(&mut env, "w", &[], cnst("MyAnd"));
    let intro = app(cnst("MyAnd.intro"), &[cnst("pa"), cnst("pb")]);
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("w"), &intro),
        "opaque proof vs constructor proof of the same Prop must be def-eq"
    );
    let eta = app(
        cnst("MyAnd.intro"),
        &[proj("MyAnd", 0, cnst("w")), proj("MyAnd", 1, cnst("w"))],
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("w"), &eta),
        "eta-shaped Prop comparison must terminate and hold"
    );
}

// ---------------------------------------------------------------------------
// (c) Nat.rec on a NatLit inside a universe-polymorphic definition.
// ---------------------------------------------------------------------------

/// `F 2` where `F := Nat.rec (motive := fun _ => Sort (u+1)) A (fun _ _ => B)`
/// computes a TYPE from a literal, under a universe parameter `u`. The
/// definition below typechecks only if literal iota (`2 ↦ Nat.succ 1`),
/// recursor reduction, and level handling all cooperate.
#[test]
fn nat_rec_on_literal_inside_polymorphic_definition() {
    let mut env = builtin_env();
    let u = Level::param(Name::str("u"));
    let su = Level::succ(u.clone());
    let ssu = Level::succ(su.clone());
    // A, B : Sort (u+1) — universe-polymorphic opaque types.
    add_axiom(&mut env, "A", &["u"], Expr::Sort(su.clone()));
    add_axiom(&mut env, "B", &["u"], Expr::Sort(su.clone()));
    let a_c = Expr::Const(Name::str("A"), vec![u.clone()]);
    let b_c = Expr::Const(Name::str("B"), vec![u.clone()]);
    // motive : Nat → Sort (u+2) — each `motive n` is `Sort (u+1)`.
    let motive = lam("_", nat(), Expr::Sort(su.clone()));
    let succ_case = lam("n", nat(), lam("ih", Expr::Sort(su.clone()), b_c.clone()));
    let f_two = app(
        Expr::Const(Name::str("Nat.rec"), vec![ssu]),
        &[motive, a_c, succ_case, nat_lit(2)],
    );
    // whnf: 2 ↦ succ 1, the succ rule fires, the minor discards the IH → B.
    assert_eq!(
        whnf(&env, &f_two),
        b_c,
        "type-level Nat.rec on a literal must compute under a level param"
    );
    // def d.{u} : F 2 → B := fun x => x — the inferred codomain is `F 2`,
    // so acceptance requires def_eq to unfold the literal recursor call
    // inside a polymorphic declaration.
    let d = ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str("d"),
            level_params: vec![Name::str("u")],
            ty: pi("x", f_two.clone(), b_c.clone()),
        },
        value: lam("x", f_two.clone(), Expr::BVar(0)),
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("d")],
    });
    check_constant_info(&mut env, d)
        .expect("polymorphic definition using Nat.rec on a literal must typecheck");
}

// ---------------------------------------------------------------------------
// (d) large-elimination decision on NORMALIZED levels.
// ---------------------------------------------------------------------------

/// A single-constructor Prop inductive whose field's sort is the RAW level
/// `imax u 0` (obtained via level instantiation, so no smart constructor
/// collapses it). `imax u 0` normalizes to `0`, so the field IS a
/// proposition and the subsingleton rule must grant LARGE elimination — a
/// syntactic `is_zero` on the un-normalized level would wrongly deny it.
#[test]
fn subsingleton_large_elimination_sees_through_raw_imax() {
    let mut env = builtin_env();
    // Rel.{v, w} : Sort (imax v w) — instantiating w := 0 yields the raw
    // `Sort (imax u 0)` as the FIELD's sort.
    add_axiom(
        &mut env,
        "Rel",
        &["v", "w"],
        Expr::Sort(Level::imax(
            Level::param(Name::str("v")),
            Level::param(Name::str("w")),
        )),
    );
    let u = Level::param(Name::str("u"));
    let field_ty = Expr::Const(Name::str("Rel"), vec![u.clone(), Level::zero()]);
    let spec = InductiveSpec::new(
        Name::str("P"),
        prop(),
        vec![(
            Name::str("P.intro"),
            pi("f", field_ty, Expr::Const(Name::str("P"), vec![u.clone()])),
        )],
    )
    .with_rec_name(Name::str("P.rec"));
    add_inductive_family(&mut env, vec![Name::str("u")], 0, vec![spec])
        .expect("P must be accepted");
    let rv = env
        .get_recursor_val(&Name::str("P.rec"))
        .expect("P.rec must be derived")
        .clone();
    assert_eq!(
        rv.common.level_params,
        vec![Name::str("u_1"), Name::str("u")],
        "field sorted at raw imax(u,0) ≡ Prop ⇒ subsingleton ⇒ LARGE elimination \
         (fresh motive universe prepended)"
    );
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("derived P.rec type must typecheck");
}

/// Negative control for the same code path: a field whose sort normalizes
/// to something PROVABLY non-zero (`imax u 1 → max u 1`) is not a
/// proposition, so the Prop inductive only eliminates into Prop.
#[test]
fn non_prop_field_restricts_to_small_elimination() {
    let mut env = builtin_env();
    add_axiom(
        &mut env,
        "Rel",
        &["v", "w"],
        Expr::Sort(Level::imax(
            Level::param(Name::str("v")),
            Level::param(Name::str("w")),
        )),
    );
    let u = Level::param(Name::str("u"));
    // Rel.{u, 1} : Sort (imax u 1) ~ Sort (max u 1) — never a proposition.
    let field_ty = Expr::Const(Name::str("Rel"), vec![u.clone(), one()]);
    let spec = InductiveSpec::new(
        Name::str("Q"),
        prop(),
        vec![(
            Name::str("Q.intro"),
            pi("f", field_ty, Expr::Const(Name::str("Q"), vec![u.clone()])),
        )],
    )
    .with_rec_name(Name::str("Q.rec"));
    add_inductive_family(&mut env, vec![Name::str("u")], 0, vec![spec])
        .expect("Q must be accepted");
    let rv = env
        .get_recursor_val(&Name::str("Q.rec"))
        .expect("Q.rec must be derived")
        .clone();
    assert_eq!(
        rv.common.level_params,
        vec![Name::str("u")],
        "a non-Prop field in a Prop inductive must force small elimination"
    );
}
