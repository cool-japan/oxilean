//! Recursor derivation, iota reduction, and inductive-checking tests.
//!
//! Pins the Wave-2 semantics (audit items S3-ind, S4, S5, S6, C6, C7, C9,
//! C10, C11, S2-rec, C5-rec):
//! * derivation golden tests (Nat, List, a 2-index family, PLift, Prop
//!   small elimination, the Eq/Acc/False subsingleton exceptions, K flag);
//! * iota (NatLit expansion, over-application, WHNF-needed major premise,
//!   K-like reduction, StrLit expansion, mutual Tree/Forest);
//! * checking (re-derived recursors reject tampered types/rules/K flags,
//!   strict positivity incl. definition-hidden negativity, universe rule,
//!   the `is_prop` lie, nested-inductive rejection).

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal, InductiveVal,
};
use oxilean_kernel::reduce::{Reducer, ReducibilityHint};
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_and_derive_family, check_constant_info, init_builtin_env,
    BinderInfo, Environment, Expr, InductiveSpec, KernelError, Level, Literal, Name, TypeChecker,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn builtin_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env must initialize");
    env
}

fn app(f: Expr, args: &[Expr]) -> Expr {
    let mut r = f;
    for a in args {
        r = Expr::App(Node::new(r), Node::new(a.clone()));
    }
    r
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn nat() -> Expr {
    cnst("Nat")
}

fn nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

fn pi(bi: BinderInfo, name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Node::new(dom), Node::new(cod))
}

fn lam(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(body),
    )
}

fn whnf(env: &Environment, e: &Expr) -> Expr {
    Reducer::new().whnf_env(e, env)
}

/// The constant motive `fun (_ : dom) => Nat`.
fn const_nat_motive(dom: Expr) -> Expr {
    lam("_", dom, nat())
}

// ---------------------------------------------------------------------------
// 1. Derivation golden tests
// ---------------------------------------------------------------------------

/// Derived `Nat.rec` has Lean's exact type:
/// `{motive : Nat → Sort u} → motive Nat.zero →
///  ((n : Nat) → motive n → motive (Nat.succ n)) → (t : Nat) → motive t`.
#[test]
fn nat_rec_golden_type() {
    let env = builtin_env();
    let rv = env
        .get_recursor_val(&Name::str("Nat.rec"))
        .expect("Nat.rec")
        .clone();
    assert_eq!(rv.common.level_params, vec![Name::str("u")]);
    assert_eq!(
        (rv.num_params, rv.num_indices, rv.num_motives, rv.num_minors),
        (0, 0, 1, 2)
    );
    assert!(!rv.k);
    let u = Level::param(Name::str("u"));
    let expected = pi(
        BinderInfo::Implicit,
        "motive",
        pi(BinderInfo::Default, "t", nat(), Expr::Sort(u)),
        pi(
            BinderInfo::Default,
            "zero",
            app(Expr::BVar(0), &[cnst("Nat.zero")]),
            pi(
                BinderInfo::Default,
                "succ",
                pi(
                    BinderInfo::Default,
                    "n",
                    nat(),
                    pi(
                        BinderInfo::Default,
                        "ih",
                        app(Expr::BVar(2), &[Expr::BVar(0)]),
                        app(Expr::BVar(3), &[app(cnst("Nat.succ"), &[Expr::BVar(1)])]),
                    ),
                ),
                pi(
                    BinderInfo::Default,
                    "t",
                    nat(),
                    app(Expr::BVar(3), &[Expr::BVar(0)]),
                ),
            ),
        ),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&rv.common.ty, &expected),
        "derived Nat.rec type must match Lean's:\n got {}\n want {}",
        rv.common.ty,
        expected
    );
    // The derived type itself typechecks.
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("Nat.rec type is a type");
}

/// Derived `List.rec` (parameterized, universe-polymorphic):
/// `List.rec.{u_1, u} : {α : Type u} → {motive : List α → Sort u_1} →
///  motive (List.nil α) → ((head : α) → (tail : List α) → motive tail →
///  motive (List.cons α head tail)) → (t : List α) → motive t`.
#[test]
fn list_rec_golden_type() {
    let env = builtin_env();
    let rv = env
        .get_recursor_val(&Name::str("List.rec"))
        .expect("List.rec")
        .clone();
    // Fresh elimination universe prepended: "u" is taken, so "u_1".
    assert_eq!(
        rv.common.level_params,
        vec![Name::str("u_1"), Name::str("u")]
    );
    let u = Level::param(Name::str("u"));
    let u1 = Level::param(Name::str("u_1"));
    let list = |a: Expr| app(Expr::Const(Name::str("List"), vec![u.clone()]), &[a]);
    let nil = Expr::Const(Name::str("List.nil"), vec![u.clone()]);
    let cons = Expr::Const(Name::str("List.cons"), vec![u.clone()]);
    let expected = pi(
        BinderInfo::Implicit,
        "α",
        Expr::Sort(Level::succ(u.clone())),
        pi(
            BinderInfo::Implicit,
            "motive",
            pi(
                BinderInfo::Default,
                "t",
                list(Expr::BVar(0)),
                Expr::Sort(u1),
            ),
            pi(
                BinderInfo::Default,
                "nil",
                app(Expr::BVar(0), &[app(nil, &[Expr::BVar(1)])]),
                pi(
                    BinderInfo::Default,
                    "cons",
                    pi(
                        BinderInfo::Default,
                        "head",
                        Expr::BVar(2),
                        pi(
                            BinderInfo::Default,
                            "tail",
                            list(Expr::BVar(3)),
                            pi(
                                BinderInfo::Default,
                                "ih",
                                app(Expr::BVar(3), &[Expr::BVar(0)]),
                                app(
                                    Expr::BVar(4),
                                    &[app(cons, &[Expr::BVar(5), Expr::BVar(2), Expr::BVar(1)])],
                                ),
                            ),
                        ),
                    ),
                    pi(
                        BinderInfo::Default,
                        "t",
                        list(Expr::BVar(3)),
                        app(Expr::BVar(3), &[Expr::BVar(0)]),
                    ),
                ),
            ),
        ),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&rv.common.ty, &expected),
        "derived List.rec type must match Lean's:\n got {}\n want {}",
        rv.common.ty,
        expected
    );
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("List.rec type is a type");
}

/// A 2-index family: `Vec2 (α : Type) : Nat → Nat → Type` with
/// `nil : Vec2 α 0 0` and `grow : (n m : Nat) → Vec2 α n m →
/// Vec2 α (n+1) (m+1)` — the derived recursor typechecks, has the right
/// counts, and iota reduces the `grow` case with its IH.
#[test]
fn two_index_family_derivation_and_iota() {
    let mut env = builtin_env();
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let vec2 = |a: Expr, i: Expr, j: Expr| app(cnst("Vec2"), &[a, i, j]);
    let vec2_ty = pi(
        BinderInfo::Default,
        "α",
        type0.clone(),
        pi(
            BinderInfo::Default,
            "i",
            nat(),
            pi(BinderInfo::Default, "j", nat(), type0.clone()),
        ),
    );
    let nil_ty = pi(
        BinderInfo::Default,
        "α",
        type0.clone(),
        vec2(Expr::BVar(0), cnst("Nat.zero"), cnst("Nat.zero")),
    );
    let grow_ty = pi(
        BinderInfo::Default,
        "α",
        type0,
        pi(
            BinderInfo::Default,
            "n",
            nat(),
            pi(
                BinderInfo::Default,
                "m",
                nat(),
                pi(
                    BinderInfo::Default,
                    "v",
                    vec2(Expr::BVar(2), Expr::BVar(1), Expr::BVar(0)),
                    vec2(
                        Expr::BVar(3),
                        app(cnst("Nat.succ"), &[Expr::BVar(2)]),
                        app(cnst("Nat.succ"), &[Expr::BVar(1)]),
                    ),
                ),
            ),
        ),
    );
    let spec = InductiveSpec::new(
        Name::str("Vec2"),
        vec2_ty,
        vec![
            (Name::str("Vec2.nil"), nil_ty),
            (Name::str("Vec2.grow"), grow_ty),
        ],
    )
    .with_rec_name(Name::str("Vec2.rec"));
    add_inductive_family(&mut env, vec![], 1, vec![spec]).expect("Vec2 must be accepted");

    let rv = env
        .get_recursor_val(&Name::str("Vec2.rec"))
        .expect("Vec2.rec")
        .clone();
    assert_eq!(
        (rv.num_params, rv.num_indices, rv.num_motives, rv.num_minors),
        (1, 2, 1, 2)
    );
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("Vec2.rec type must typecheck");

    // iota: count `grow` layers of `grow n m (grow n' m' nil)` = 2.
    let motive = lam(
        "i",
        nat(),
        lam(
            "j",
            nat(),
            lam("_", vec2(nat(), Expr::BVar(1), Expr::BVar(0)), nat()),
        ),
    );
    let m_nil = nat_lit(0);
    let m_grow = lam(
        "n",
        nat(),
        lam(
            "m",
            nat(),
            lam(
                "v",
                vec2(nat(), Expr::BVar(1), Expr::BVar(0)),
                lam("ih", nat(), app(cnst("Nat.succ"), &[Expr::BVar(0)])),
            ),
        ),
    );
    let v1 = app(
        cnst("Vec2.grow"),
        &[
            nat(),
            cnst("Nat.zero"),
            cnst("Nat.zero"),
            app(cnst("Vec2.nil"), &[nat()]),
        ],
    );
    let v2 = app(
        cnst("Vec2.grow"),
        &[
            nat(),
            app(cnst("Nat.succ"), &[cnst("Nat.zero")]),
            app(cnst("Nat.succ"), &[cnst("Nat.zero")]),
            v1,
        ],
    );
    let two = app(
        cnst("Nat.succ"),
        &[app(cnst("Nat.succ"), &[cnst("Nat.zero")])],
    );
    let rec_app = app(
        Expr::Const(Name::str("Vec2.rec"), vec![Level::succ(Level::zero())]),
        &[nat(), motive, m_nil, m_grow, two.clone(), two, v2],
    );
    let result = whnf(&env, &rec_app);
    assert_eq!(result, nat_lit(2), "two grow layers must count to 2");
}

/// PLift-like universe-polymorphic inductive:
/// `PLift.{u} (α : Sort u) : Type u | up : α → PLift α` — the level
/// parameter propagates into all generated declarations, and the fresh
/// elimination universe is prepended.
#[test]
fn plift_universe_polymorphism() {
    let mut env = builtin_env();
    let u = Level::param(Name::str("u"));
    let plift_ty = pi(
        BinderInfo::Default,
        "α",
        Expr::Sort(u.clone()),
        Expr::Sort(Level::succ(u.clone())),
    );
    let up_ty = pi(
        BinderInfo::Default,
        "α",
        Expr::Sort(u.clone()),
        pi(
            BinderInfo::Default,
            "a",
            Expr::BVar(0),
            app(
                Expr::Const(Name::str("PLift"), vec![u.clone()]),
                &[Expr::BVar(1)],
            ),
        ),
    );
    let spec = InductiveSpec::new(
        Name::str("PLift"),
        plift_ty,
        vec![(Name::str("PLift.up"), up_ty)],
    )
    .with_rec_name(Name::str("PLift.rec"));
    add_inductive_family(&mut env, vec![Name::str("u")], 1, vec![spec])
        .expect("PLift must be accepted");
    let rv = env
        .get_recursor_val(&Name::str("PLift.rec"))
        .expect("PLift.rec")
        .clone();
    assert_eq!(
        rv.common.level_params,
        vec![Name::str("u_1"), Name::str("u")],
        "fresh elim universe prepended to the inductive's own params"
    );
    let iv = env
        .get_inductive_val(&Name::str("PLift"))
        .expect("PLift inductive");
    assert_eq!(iv.common.level_params, vec![Name::str("u")]);
    let cv = env
        .get_constructor_val(&Name::str("PLift.up"))
        .expect("PLift.up");
    assert_eq!(cv.common.level_params, vec![Name::str("u")]);
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("PLift.rec type must typecheck");
    // iota at a concrete instantiation (u := 1, α := Nat).
    let one = Level::succ(Level::zero());
    let motive = lam(
        "_",
        app(Expr::Const(Name::str("PLift"), vec![one.clone()]), &[nat()]),
        nat(),
    );
    let m_up = lam("a", nat(), Expr::BVar(0));
    let major = app(
        Expr::Const(Name::str("PLift.up"), vec![one.clone()]),
        &[nat(), nat_lit(9)],
    );
    let rec_app = app(
        Expr::Const(Name::str("PLift.rec"), vec![one.clone(), one]),
        &[nat(), motive, m_up, major],
    );
    assert_eq!(whnf(&env, &rec_app), nat_lit(9));
}

/// A 2-constructor Prop inductive only eliminates into Prop: no fresh
/// elimination universe is added and the motive lands in `Sort 0`.
#[test]
fn prop_inductive_small_elimination_enforced() {
    let mut env = builtin_env();
    let prop = Expr::Sort(Level::zero());
    let or2 = |a: Expr, b: Expr| app(cnst("Or2"), &[a, b]);
    let or2_ty = pi(
        BinderInfo::Default,
        "a",
        prop.clone(),
        pi(BinderInfo::Default, "b", prop.clone(), prop.clone()),
    );
    let inl_ty = pi(
        BinderInfo::Default,
        "a",
        prop.clone(),
        pi(
            BinderInfo::Default,
            "b",
            prop.clone(),
            pi(
                BinderInfo::Default,
                "h",
                Expr::BVar(1),
                or2(Expr::BVar(2), Expr::BVar(1)),
            ),
        ),
    );
    let inr_ty = pi(
        BinderInfo::Default,
        "a",
        prop.clone(),
        pi(
            BinderInfo::Default,
            "b",
            prop.clone(),
            pi(
                BinderInfo::Default,
                "h",
                Expr::BVar(0),
                or2(Expr::BVar(2), Expr::BVar(1)),
            ),
        ),
    );
    let spec = InductiveSpec::new(
        Name::str("Or2"),
        or2_ty,
        vec![
            (Name::str("Or2.inl"), inl_ty),
            (Name::str("Or2.inr"), inr_ty),
        ],
    )
    .with_rec_name(Name::str("Or2.rec"));
    add_inductive_family(&mut env, vec![], 2, vec![spec]).expect("Or2 must be accepted");
    let rv = env
        .get_recursor_val(&Name::str("Or2.rec"))
        .expect("Or2.rec")
        .clone();
    assert!(
        rv.common.level_params.is_empty(),
        "small elimination: no fresh universe parameter, got {:?}",
        rv.common.level_params
    );
    assert!(!rv.k, "multi-constructor Prop is not K-like");
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("Or2.rec type must typecheck");
    // The inductive's is_prop is computed by the kernel.
    assert!(
        env.get_inductive_val(&Name::str("Or2"))
            .expect("Or2")
            .is_prop
    );
}

/// Subsingleton exceptions: `Eq` (single ctor, all args params) and a
/// `False`-like empty Prop eliminate into any universe; `Eq` is K-like.
#[test]
fn subsingleton_large_elimination_eq_false() {
    let mut env = builtin_env();
    // Eq: derived by the builtin path with Lean's exact shape.
    let rv = env
        .get_recursor_val(&Name::str("Eq.rec"))
        .expect("Eq.rec")
        .clone();
    assert_eq!(rv.num_params, 2, "Eq has params α and a");
    assert_eq!(rv.num_indices, 1, "Eq has index b");
    assert!(rv.k, "Eq is K-like");
    assert_eq!(
        rv.common.level_params.len(),
        2,
        "Eq.rec large-eliminates: fresh universe + u"
    );
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("Eq.rec type must typecheck");

    // False: empty Prop inductive large-eliminates (ex falso) but is not K.
    let spec = InductiveSpec::new(Name::str("False2"), Expr::Sort(Level::zero()), vec![])
        .with_rec_name(Name::str("False2.rec"));
    add_inductive_family(&mut env, vec![], 0, vec![spec]).expect("False2 must be accepted");
    let rv = env
        .get_recursor_val(&Name::str("False2.rec"))
        .expect("False2.rec")
        .clone();
    assert_eq!(rv.common.level_params, vec![Name::str("u")]);
    assert_eq!(rv.num_minors, 0);
    assert!(!rv.k, "empty inductives are not K-like");
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("False2.rec type must typecheck");
}

/// `Acc`-like reflexive inductive: the recursive field is a Pi
/// (`(y : α) → r y x → Acc r y`), so the minor premise gets a Pi-wrapped
/// induction hypothesis; the family is flagged reflexive and — being a
/// single-constructor Prop whose fields are props or indices —
/// large-eliminates.
#[test]
fn acc_reflexive_derivation_and_iota() {
    let mut env = builtin_env();
    let u = Level::param(Name::str("u"));
    let prop = Expr::Sort(Level::zero());
    // Acc.{u} (α : Sort u) (r : α → α → Prop) : α → Prop
    let acc_ty = pi(
        BinderInfo::Default,
        "α",
        Expr::Sort(u.clone()),
        pi(
            BinderInfo::Default,
            "r",
            pi(
                BinderInfo::Default,
                "x",
                Expr::BVar(0),
                pi(BinderInfo::Default, "y", Expr::BVar(1), prop.clone()),
            ),
            pi(BinderInfo::Default, "x", Expr::BVar(1), prop.clone()),
        ),
    );
    let acc = |a: Expr, r: Expr, x: Expr| {
        app(Expr::Const(Name::str("Acc2"), vec![u.clone()]), &[a, r, x])
    };
    // intro : (x : α) → ((y : α) → r y x → Acc α r y) → Acc α r x
    let intro_ty = pi(
        BinderInfo::Default,
        "α",
        Expr::Sort(u.clone()),
        pi(
            BinderInfo::Default,
            "r",
            pi(
                BinderInfo::Default,
                "x",
                Expr::BVar(0),
                pi(BinderInfo::Default, "y", Expr::BVar(1), prop.clone()),
            ),
            pi(
                BinderInfo::Default,
                "x",
                Expr::BVar(1),
                pi(
                    BinderInfo::Default,
                    "h",
                    pi(
                        BinderInfo::Default,
                        "y",
                        Expr::BVar(2),
                        pi(
                            BinderInfo::Default,
                            "hr",
                            app(Expr::BVar(2), &[Expr::BVar(0), Expr::BVar(1)]),
                            acc(Expr::BVar(4), Expr::BVar(3), Expr::BVar(1)),
                        ),
                    ),
                    acc(Expr::BVar(3), Expr::BVar(2), Expr::BVar(1)),
                ),
            ),
        ),
    );
    let spec = InductiveSpec::new(
        Name::str("Acc2"),
        acc_ty,
        vec![(Name::str("Acc2.intro"), intro_ty)],
    )
    .with_rec_name(Name::str("Acc2.rec"));
    add_inductive_family(&mut env, vec![Name::str("u")], 2, vec![spec])
        .expect("Acc2 must be accepted");
    let iv = env
        .get_inductive_val(&Name::str("Acc2"))
        .expect("Acc2")
        .clone();
    assert!(iv.is_rec);
    assert!(iv.is_reflexive, "Acc has a reflexive (function) field");
    let rv = env
        .get_recursor_val(&Name::str("Acc2.rec"))
        .expect("Acc2.rec")
        .clone();
    assert_eq!(
        rv.common.level_params,
        vec![Name::str("u_1"), Name::str("u")],
        "Acc large-eliminates (subsingleton exception)"
    );
    assert!(!rv.k, "Acc has a non-param field, not K-like");
    let mut tc = TypeChecker::new(&env);
    tc.ensure_sort(&rv.common.ty)
        .expect("Acc2.rec type must typecheck");
}

// ---------------------------------------------------------------------------
// 2. Iota reduction
// ---------------------------------------------------------------------------

/// `Nat.rec` on Nat *literals* 0/1/5 (pins C6): the kernel expands the
/// literal one constructor layer at a time.
#[test]
fn nat_rec_on_literals() {
    let env = builtin_env();
    let u = Level::succ(Level::zero());
    // rec_add m k = m + k via Nat.rec on k.
    let rec_add = |m: u64, k: u64| {
        app(
            Expr::Const(Name::str("Nat.rec"), vec![u.clone()]),
            &[
                const_nat_motive(nat()),
                nat_lit(m),
                lam(
                    "n",
                    nat(),
                    lam("ih", nat(), app(cnst("Nat.succ"), &[Expr::BVar(0)])),
                ),
                nat_lit(k),
            ],
        )
    };
    assert_eq!(whnf(&env, &rec_add(10, 0)), nat_lit(10), "NatLit 0 case");
    assert_eq!(whnf(&env, &rec_add(10, 1)), nat_lit(11), "NatLit 1 case");
    assert_eq!(whnf(&env, &rec_add(10, 5)), nat_lit(15), "NatLit 5 case");
}

/// Over-application: a motive returning a function type — the trailing
/// argument must be re-applied to the reduct, never dropped.
#[test]
fn nat_rec_over_application_keeps_args() {
    let env = builtin_env();
    let u = Level::succ(Level::zero());
    let motive = lam("_", nat(), pi(BinderInfo::Default, "y", nat(), nat()));
    // zero case: identity; succ case: constant successor of the extra arg.
    let z = lam("y", nat(), Expr::BVar(0));
    let s = lam(
        "n",
        nat(),
        lam(
            "ih",
            pi(BinderInfo::Default, "y", nat(), nat()),
            lam("y", nat(), app(cnst("Nat.succ"), &[Expr::BVar(0)])),
        ),
    );
    let over0 = app(
        Expr::Const(Name::str("Nat.rec"), vec![u.clone()]),
        &[
            motive.clone(),
            z.clone(),
            s.clone(),
            nat_lit(0),
            nat_lit(41),
        ],
    );
    assert_eq!(whnf(&env, &over0), nat_lit(41));
    let over1 = app(
        Expr::Const(Name::str("Nat.rec"), vec![u]),
        &[motive, z, s, nat_lit(1), nat_lit(41)],
    );
    assert_eq!(whnf(&env, &over1), nat_lit(42));
}

/// The major premise is WHNF'd before matching: a definition unfolding to
/// a constructor application still triggers iota.
#[test]
fn iota_whnf_major_premise() {
    let mut env = builtin_env();
    let two = app(
        cnst("Nat.succ"),
        &[app(cnst("Nat.succ"), &[cnst("Nat.zero")])],
    );
    env.add_constant(ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str("two"),
            level_params: vec![],
            ty: nat(),
        },
        value: two,
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("two")],
    }))
    .expect("add definition");
    let u = Level::succ(Level::zero());
    let rec_app = app(
        Expr::Const(Name::str("Nat.rec"), vec![u]),
        &[
            const_nat_motive(nat()),
            nat_lit(100),
            lam(
                "n",
                nat(),
                lam("ih", nat(), app(cnst("Nat.succ"), &[Expr::BVar(0)])),
            ),
            cnst("two"),
        ],
    );
    assert_eq!(whnf(&env, &rec_app), nat_lit(102));
}

/// K-like reduction (pins C7): `Eq.rec` reduces both on a literal
/// `Eq.refl` major premise and on a *stuck* proof term (an axiom) whose
/// type is a definitionally-refl equation.
#[test]
fn k_reduction_eq() {
    let mut env = builtin_env();
    let one = Level::succ(Level::zero());
    let eq_nat = |a: Expr, b: Expr| {
        app(
            Expr::Const(Name::str("Eq"), vec![one.clone()]),
            &[nat(), a, b],
        )
    };
    // h : Eq Nat 0 0 — a stuck constant, NOT syntactically Eq.refl.
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("h0"),
            level_params: vec![],
            ty: eq_nat(nat_lit(0), nat_lit(0)),
        },
        is_unsafe: false,
    }))
    .expect("add axiom h0");
    // mval : some result constant to observe.
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("mval"),
            level_params: vec![],
            ty: nat(),
        },
        is_unsafe: false,
    }))
    .expect("add axiom mval");
    let motive = lam(
        "b",
        nat(),
        lam("t", eq_nat(nat_lit(0), Expr::BVar(0)), nat()),
    );
    let eq_rec = Expr::Const(Name::str("Eq.rec"), vec![one.clone(), one.clone()]);
    // Eq.rec α a motive minor b major
    let via_axiom = app(
        eq_rec.clone(),
        &[
            nat(),
            nat_lit(0),
            motive.clone(),
            cnst("mval"),
            nat_lit(0),
            cnst("h0"),
        ],
    );
    assert_eq!(
        whnf(&env, &via_axiom),
        cnst("mval"),
        "K-reduction must fire on a stuck proof of a defeq-refl equation"
    );
    // And on a literal refl.
    let refl = app(
        Expr::Const(Name::str("Eq.refl"), vec![one]),
        &[nat(), nat_lit(0)],
    );
    let via_refl = app(
        eq_rec,
        &[nat(), nat_lit(0), motive, cnst("mval"), nat_lit(0), refl],
    );
    assert_eq!(whnf(&env, &via_refl), cnst("mval"));
}

/// K-reduction must NOT fire when the equation's endpoints are not
/// definitionally equal — the term stays stuck.
#[test]
fn k_reduction_does_not_fire_on_non_refl_type() {
    let mut env = builtin_env();
    let one = Level::succ(Level::zero());
    let eq_nat = |a: Expr, b: Expr| {
        app(
            Expr::Const(Name::str("Eq"), vec![one.clone()]),
            &[nat(), a, b],
        )
    };
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("h01"),
            level_params: vec![],
            ty: eq_nat(nat_lit(0), nat_lit(1)),
        },
        is_unsafe: false,
    }))
    .expect("add axiom h01");
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("mval"),
            level_params: vec![],
            ty: nat(),
        },
        is_unsafe: false,
    }))
    .expect("add axiom mval");
    let motive = lam(
        "b",
        nat(),
        lam("t", eq_nat(nat_lit(0), Expr::BVar(0)), nat()),
    );
    let rec_app = app(
        Expr::Const(Name::str("Eq.rec"), vec![one.clone(), one]),
        &[
            nat(),
            nat_lit(0),
            motive,
            cnst("mval"),
            nat_lit(1),
            cnst("h01"),
        ],
    );
    let result = whnf(&env, &rec_app);
    assert_ne!(
        result,
        cnst("mval"),
        "0 = 1 is not definitionally refl; the term must stay stuck"
    );
}

/// `String.rec` on a string literal (StrLit expansion): the literal
/// expands to `String.mk (List.cons Char (Char.ofNat 97) …)`.
#[test]
fn string_rec_on_literal() {
    let mut env = builtin_env();
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str("consume"),
            level_params: vec![],
            ty: pi(
                BinderInfo::Default,
                "l",
                app(
                    Expr::Const(Name::str("List"), vec![Level::zero()]),
                    &[cnst("Char")],
                ),
                nat(),
            ),
        },
        is_unsafe: false,
    }))
    .expect("add axiom consume");
    let u = Level::succ(Level::zero());
    let motive = lam("_", cnst("String"), nat());
    let m_mk = lam(
        "data",
        app(
            Expr::Const(Name::str("List"), vec![Level::zero()]),
            &[cnst("Char")],
        ),
        app(cnst("consume"), &[Expr::BVar(0)]),
    );
    let rec_app = app(
        Expr::Const(Name::str("String.rec"), vec![u]),
        &[motive, m_mk, Expr::Lit(Literal::Str("ab".to_string()))],
    );
    let result = whnf(&env, &rec_app);
    // Expected: consume (List.cons Char (Char.ofNat 97) (List.cons Char (Char.ofNat 98) (List.nil Char)))
    let char_ty = cnst("Char");
    let cons = |c: u64, rest: Expr| {
        app(
            Expr::Const(Name::str("List.cons"), vec![Level::zero()]),
            &[
                char_ty.clone(),
                app(cnst("Char.ofNat"), &[nat_lit(c)]),
                rest,
            ],
        )
    };
    let nil = app(
        Expr::Const(Name::str("List.nil"), vec![Level::zero()]),
        std::slice::from_ref(&char_ty),
    );
    let expected = app(cnst("consume"), &[cons(97, cons(98, nil))]);
    assert_eq!(result, expected);
}

/// Mutual Tree/Forest (pins C9), mirroring the lean4export fixture
/// `tests/fixtures/lean4export/Tree_Forest.ndjson`: family declaration,
/// recursors with two motives and all four minor premises, and iota
/// across the family (leaf count of a small tree).
#[test]
fn mutual_tree_forest() {
    let mut env = builtin_env();
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let tree = Name::str("Tree");
    let forest = Name::str("Forest");
    let tree_of = |a: Expr| app(Expr::Const(tree.clone(), vec![]), &[a]);
    let forest_of = |a: Expr| app(Expr::Const(forest.clone(), vec![]), &[a]);
    // Tree (α : Type) | leaf : Tree α | node : α → Forest α → Tree α
    let tree_spec = InductiveSpec::new(
        tree.clone(),
        pi(BinderInfo::Default, "α", type0.clone(), type0.clone()),
        vec![
            (
                Name::str("Tree.leaf"),
                pi(
                    BinderInfo::Implicit,
                    "α",
                    type0.clone(),
                    tree_of(Expr::BVar(0)),
                ),
            ),
            (
                Name::str("Tree.node"),
                pi(
                    BinderInfo::Implicit,
                    "α",
                    type0.clone(),
                    pi(
                        BinderInfo::Default,
                        "a",
                        Expr::BVar(0),
                        pi(
                            BinderInfo::Default,
                            "children",
                            forest_of(Expr::BVar(1)),
                            tree_of(Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        ],
    );
    // Forest (α : Type) | nil : Forest α | cons : Tree α → Forest α → Forest α
    let forest_spec = InductiveSpec::new(
        forest.clone(),
        pi(BinderInfo::Default, "α", type0.clone(), type0.clone()),
        vec![
            (
                Name::str("Forest.nil"),
                pi(
                    BinderInfo::Implicit,
                    "α",
                    type0.clone(),
                    forest_of(Expr::BVar(0)),
                ),
            ),
            (
                Name::str("Forest.cons"),
                pi(
                    BinderInfo::Implicit,
                    "α",
                    type0.clone(),
                    pi(
                        BinderInfo::Default,
                        "t",
                        tree_of(Expr::BVar(0)),
                        pi(
                            BinderInfo::Default,
                            "f",
                            forest_of(Expr::BVar(1)),
                            forest_of(Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        ],
    );
    add_inductive_family(&mut env, vec![], 1, vec![tree_spec, forest_spec])
        .expect("mutual Tree/Forest must be accepted");

    // Family metadata mirrors the fixture: 2 motives, 4 minors, per-type rules.
    let tree_rec_name = Name::mk_str(tree.clone(), "rec".to_string());
    let forest_rec_name = Name::mk_str(forest.clone(), "rec".to_string());
    let trec = env
        .get_recursor_val(&tree_rec_name)
        .expect("Tree.rec")
        .clone();
    let frec = env
        .get_recursor_val(&forest_rec_name)
        .expect("Forest.rec")
        .clone();
    for rv in [&trec, &frec] {
        assert_eq!(rv.num_motives, 2);
        assert_eq!(rv.num_minors, 4);
        assert_eq!(rv.num_params, 1);
        assert_eq!(rv.all, vec![tree.clone(), forest.clone()]);
        assert_eq!(rv.common.level_params, vec![Name::str("u")]);
        assert_eq!(rv.rules.len(), 2, "each recursor carries its own rules");
        let mut tc = TypeChecker::new(&env);
        tc.ensure_sort(&rv.common.ty)
            .expect("mutual recursor type must typecheck");
    }
    let iv = env.get_inductive_val(&tree).expect("Tree").clone();
    assert!(iv.is_rec);
    assert_eq!(iv.all, vec![tree.clone(), forest.clone()]);

    // Leaf count via mutual recursion:
    //   count(leaf) = 1, count(node a f) = countF(f),
    //   countF(nil) = 0, countF(cons t f) = count(t) + countF(f).
    let one = Level::succ(Level::zero());
    let m1 = lam("_", tree_of(nat()), nat());
    let m2 = lam("_", forest_of(nat()), nat());
    let m_leaf = nat_lit(1);
    let m_node = lam(
        "a",
        nat(),
        lam(
            "children",
            forest_of(nat()),
            lam("ihf", nat(), Expr::BVar(0)),
        ),
    );
    let m_nil = nat_lit(0);
    let m_cons = lam(
        "t",
        tree_of(nat()),
        lam(
            "f",
            forest_of(nat()),
            lam(
                "iht",
                nat(),
                lam(
                    "ihf",
                    nat(),
                    app(cnst("Nat.add"), &[Expr::BVar(1), Expr::BVar(0)]),
                ),
            ),
        ),
    );
    let leaf = app(Expr::Const(Name::str("Tree.leaf"), vec![]), &[nat()]);
    let fnil = app(Expr::Const(Name::str("Forest.nil"), vec![]), &[nat()]);
    let fcons = |t: Expr, f: Expr| {
        app(
            Expr::Const(Name::str("Forest.cons"), vec![]),
            &[nat(), t, f],
        )
    };
    // node 7 [leaf, leaf] — two leaves.
    let the_tree = app(
        Expr::Const(Name::str("Tree.node"), vec![]),
        &[
            nat(),
            nat_lit(7),
            fcons(leaf.clone(), fcons(leaf.clone(), fnil.clone())),
        ],
    );
    let premises = |major: Expr, rec: &Name| {
        app(
            Expr::Const(rec.clone(), vec![one.clone()]),
            &[
                nat(),
                m1.clone(),
                m2.clone(),
                m_leaf.clone(),
                m_node.clone(),
                m_nil.clone(),
                m_cons.clone(),
                major,
            ],
        )
    };
    assert_eq!(
        whnf(&env, &premises(the_tree, &tree_rec_name)),
        nat_lit(2),
        "node [leaf, leaf] has two leaves"
    );
    assert_eq!(
        whnf(&env, &premises(leaf, &tree_rec_name)),
        nat_lit(1),
        "a single leaf counts 1"
    );
    let two_forest = fcons(
        app(Expr::Const(Name::str("Tree.leaf"), vec![]), &[nat()]),
        fnil,
    );
    assert_eq!(
        whnf(&env, &premises(two_forest, &forest_rec_name)),
        nat_lit(1),
        "Forest.rec reduces across the family"
    );
}

// ---------------------------------------------------------------------------
// 3. Checking: re-derive, don't trust
// ---------------------------------------------------------------------------

/// Replaying Nat through `check_constant_info`: the correct derived
/// recursor is accepted; tampered variants (wrong type / wrong rule RHS /
/// flipped K flag) are rejected (pins S3).
#[test]
fn replay_accepts_derived_and_rejects_tampered_recursor() {
    let nat_spec = || {
        InductiveSpec::new(
            Name::str("Nat"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![
                (Name::str("Nat.zero"), nat()),
                (
                    Name::str("Nat.succ"),
                    pi(BinderInfo::Default, "n", nat(), nat()),
                ),
            ],
        )
        .with_rec_name(Name::str("Nat.rec"))
    };
    let derive = |env: &Environment| {
        check_and_derive_family(env, &[], 0, &[nat_spec()]).expect("derive Nat")
    };
    let fresh_env_with_nat = || {
        let mut env = Environment::new();
        let fam = derive(&env);
        for ci in fam
            .inductives
            .iter()
            .chain(fam.constructors.iter())
            .cloned()
        {
            check_constant_info(&mut env, ci).expect("inductive + ctors accepted");
        }
        (env, fam)
    };

    // Positive control: the kernel-derived recursor is accepted.
    {
        let (mut env, fam) = fresh_env_with_nat();
        let rec = fam.recursors[0].clone();
        check_constant_info(&mut env, rec).expect("derived recursor must be accepted");
        assert!(env.is_recursor(&Name::str("Nat.rec")));
    }
    // Wrong type: the old placeholder `Sort 0` recursor type is rejected.
    {
        let (mut env, fam) = fresh_env_with_nat();
        let mut rv = match &fam.recursors[0] {
            ConstantInfo::Recursor(rv) => rv.clone(),
            _ => unreachable!("derivation produces a recursor"),
        };
        rv.common.ty = Expr::Sort(Level::zero());
        let err = check_constant_info(&mut env, ConstantInfo::Recursor(rv))
            .expect_err("wrong recursor type must be rejected");
        assert!(matches!(err, KernelError::InvalidRecursor(_)));
    }
    // Wrong rule RHS: swap the zero/succ right-hand sides.
    {
        let (mut env, fam) = fresh_env_with_nat();
        let mut rv = match &fam.recursors[0] {
            ConstantInfo::Recursor(rv) => rv.clone(),
            _ => unreachable!(),
        };
        let rhs0 = rv.rules[0].rhs.clone();
        let rhs1 = rv.rules[1].rhs.clone();
        rv.rules[0].rhs = rhs1;
        rv.rules[1].rhs = rhs0;
        let err = check_constant_info(&mut env, ConstantInfo::Recursor(rv))
            .expect_err("wrong rule RHS must be rejected");
        assert!(matches!(err, KernelError::InvalidRecursor(_)));
    }
    // Lying K flag: Nat is not K-like.
    {
        let (mut env, fam) = fresh_env_with_nat();
        let mut rv = match &fam.recursors[0] {
            ConstantInfo::Recursor(rv) => rv.clone(),
            _ => unreachable!(),
        };
        rv.k = true;
        let err = check_constant_info(&mut env, ConstantInfo::Recursor(rv))
            .expect_err("flipped K flag must be rejected");
        assert!(matches!(err, KernelError::InvalidRecursor(_)));
    }
}

/// Non-strictly-positive inductives are rejected (pins C11), including a
/// negative occurrence hidden behind a definition (WHNF-hardened check).
#[test]
fn positivity_rejections() {
    // inductive Bad | mk : (Bad → Bool) → Bad
    {
        let mut env = builtin_env();
        let bad = cnst("Bad");
        let spec = InductiveSpec::new(
            Name::str("Bad"),
            Expr::Sort(Level::succ(Level::zero())),
            vec![(
                Name::str("Bad.mk"),
                pi(
                    BinderInfo::Default,
                    "f",
                    pi(BinderInfo::Default, "x", bad.clone(), cnst("Bool")),
                    bad,
                ),
            )],
        );
        let err = add_inductive_family(&mut env, vec![], 0, vec![spec])
            .expect_err("negative occurrence must be rejected");
        assert!(matches!(err, KernelError::InvalidInductive(_)));
    }
    // def F (X : Type) : Type := X → Bool; inductive Bad2 | mk : F Bad2 → Bad2
    {
        let mut env = builtin_env();
        let type0 = Expr::Sort(Level::succ(Level::zero()));
        env.add_constant(ConstantInfo::Definition(DefinitionVal {
            common: ConstantVal {
                name: Name::str("F"),
                level_params: vec![],
                ty: pi(BinderInfo::Default, "X", type0.clone(), type0.clone()),
            },
            value: Expr::Lam(
                BinderInfo::Default,
                Name::str("X"),
                Node::new(type0.clone()),
                Node::new(pi(BinderInfo::Default, "x", Expr::BVar(0), cnst("Bool"))),
            ),
            hints: ReducibilityHint::Regular(1),
            safety: DefinitionSafety::Safe,
            all: vec![Name::str("F")],
        }))
        .expect("add F");
        let bad2 = cnst("Bad2");
        let spec = InductiveSpec::new(
            Name::str("Bad2"),
            type0,
            vec![(
                Name::str("Bad2.mk"),
                pi(
                    BinderInfo::Default,
                    "f",
                    app(cnst("F"), std::slice::from_ref(&bad2)),
                    bad2,
                ),
            )],
        );
        let err = add_inductive_family(&mut env, vec![], 0, vec![spec])
            .expect_err("definition-hidden negative occurrence must be rejected");
        assert!(
            matches!(err, KernelError::InvalidInductive(_)),
            "got {:?}",
            err
        );
    }
    // Mutual negativity: Even's ctor uses Odd negatively.
    {
        let mut env = builtin_env();
        let type0 = Expr::Sort(Level::succ(Level::zero()));
        let even_spec = InductiveSpec::new(
            Name::str("EvenT"),
            type0.clone(),
            vec![(
                Name::str("EvenT.mk"),
                pi(
                    BinderInfo::Default,
                    "f",
                    pi(BinderInfo::Default, "x", cnst("OddT"), cnst("Bool")),
                    cnst("EvenT"),
                ),
            )],
        );
        let odd_spec = InductiveSpec::new(
            Name::str("OddT"),
            type0,
            vec![(
                Name::str("OddT.mk"),
                pi(BinderInfo::Default, "e", cnst("EvenT"), cnst("OddT")),
            )],
        );
        let err = add_inductive_family(&mut env, vec![], 0, vec![even_spec, odd_spec])
            .expect_err("negative sibling occurrence must be rejected");
        assert!(matches!(err, KernelError::InvalidInductive(_)));
    }
}

/// Nested inductives are rejected with the dedicated typed error (C10).
#[test]
fn nested_inductive_typed_rejection() {
    let mut env = builtin_env();
    let t = cnst("RoseTree");
    let list_t = app(
        Expr::Const(Name::str("List"), vec![Level::zero()]),
        std::slice::from_ref(&t),
    );
    let spec = InductiveSpec::new(
        Name::str("RoseTree"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![(
            Name::str("RoseTree.mk"),
            pi(BinderInfo::Default, "children", list_t, t),
        )],
    );
    let err = add_inductive_family(&mut env, vec![], 0, vec![spec])
        .expect_err("nested inductive must be rejected");
    match err {
        KernelError::UnsupportedNestedInductive(name) => {
            assert_eq!(name, Name::str("RoseTree"));
        }
        other => panic!("expected UnsupportedNestedInductive, got {:?}", other),
    }
}

/// Constructor universe violation: a `Type`-valued inductive cannot store
/// a `Type`-sized field (`mk : Type → SmallT` needs `Type 1 ≤ Type 0`).
#[test]
fn constructor_universe_violation_rejected() {
    let mut env = builtin_env();
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let spec = InductiveSpec::new(
        Name::str("SmallT"),
        type0.clone(),
        vec![(
            Name::str("SmallT.mk"),
            pi(BinderInfo::Default, "x", type0, cnst("SmallT")),
        )],
    );
    let err = add_inductive_family(&mut env, vec![], 0, vec![spec])
        .expect_err("universe violation must be rejected");
    assert!(matches!(err, KernelError::InvalidInductive(_)));
}

/// Prop fields are exempt from the universe rule (impredicativity):
/// a Prop-valued field fits in any inductive.
#[test]
fn prop_field_universe_exemption() {
    let mut env = builtin_env();
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    // Boxes a proposition *value*: mk : (p : Prop) → p → HoldsT is NOT
    // allowed in Type 0 (Prop itself is Type-sized), but a Prop *proof*
    // field is: mk : Eq Nat 0 0 → HoldsT.
    let one = Level::succ(Level::zero());
    let eq00 = app(
        Expr::Const(Name::str("Eq"), vec![one]),
        &[nat(), nat_lit(0), nat_lit(0)],
    );
    let spec = InductiveSpec::new(
        Name::str("HoldsT"),
        type0,
        vec![(
            Name::str("HoldsT.mk"),
            pi(BinderInfo::Default, "h", eq00, cnst("HoldsT")),
        )],
    );
    add_inductive_family(&mut env, vec![], 0, vec![spec])
        .expect("Prop-sized fields fit everywhere");
    let _ = prop;
}

/// An `is_prop` lie on a supplied `InductiveVal` is rejected (pins S4):
/// the kernel recomputes propositionality from the declared type.
#[test]
fn is_prop_lie_rejected() {
    let mut env = Environment::new();
    let iv = InductiveVal {
        common: ConstantVal {
            name: Name::str("LyingProp"),
            level_params: vec![],
            ty: Expr::Sort(Level::zero()), // lands in Prop...
        },
        num_params: 0,
        num_indices: 0,
        all: vec![Name::str("LyingProp")],
        ctors: vec![],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false, // ...but claims otherwise.
    };
    let err = check_constant_info(&mut env, ConstantInfo::Inductive(iv))
        .expect_err("is_prop lie must be rejected");
    assert!(matches!(err, KernelError::InvalidInductive(_)));
}

/// A hand-supplied constructor whose metadata or telescope disagrees with
/// the parent inductive is rejected.
#[test]
fn bogus_constructor_rejected() {
    let mut env = builtin_env();
    // Claims to construct Nat but returns Bool.
    let cv = oxilean_kernel::declaration::ConstructorVal {
        common: ConstantVal {
            name: Name::str("Nat.bogus"),
            level_params: vec![],
            ty: cnst("Bool"),
        },
        induct: Name::str("Nat"),
        cidx: 0,
        num_params: 0,
        num_fields: 0,
        is_unsafe: false,
    };
    let err = check_constant_info(&mut env, ConstantInfo::Constructor(cv))
        .expect_err("bogus constructor must be rejected");
    assert!(matches!(err, KernelError::InvalidInductive(_)));
}

/// `RecursorVal::get_major_idx` positions and quotient over-application
/// still behave (regression guard for the shared reducer path).
#[test]
fn recursor_metadata_sanity() {
    let env = builtin_env();
    let nat_rec = env
        .get_recursor_val(&Name::str("Nat.rec"))
        .expect("Nat.rec");
    assert_eq!(nat_rec.get_major_idx(), 3);
    let eq_rec = env.get_recursor_val(&Name::str("Eq.rec")).expect("Eq.rec");
    // 2 params + 1 motive + 1 minor + 1 index = major at 5.
    assert_eq!(eq_rec.get_major_idx(), 5);
    let list_rec = env
        .get_recursor_val(&Name::str("List.rec"))
        .expect("List.rec");
    assert_eq!(list_rec.get_major_idx(), 4);
}

/// The derived `Bool.rec` actually computes (the audit's placeholder
/// `Nat.rec m z s Nat.zero ↦ s` mis-reduction is gone; pins S6).
#[test]
fn builtin_bool_and_nat_rec_compute() {
    let env = builtin_env();
    let u = Level::succ(Level::zero());
    // Bool.rec motive m_true m_false Bool.false ↦ m_false (ctor order: true, false).
    let motive = lam("_", cnst("Bool"), nat());
    let rec_app = app(
        Expr::Const(Name::str("Bool.rec"), vec![u.clone()]),
        &[motive, nat_lit(1), nat_lit(0), cnst("Bool.false")],
    );
    assert_eq!(whnf(&env, &rec_app), nat_lit(0));
    // Nat.rec m z s Nat.zero ↦ z (NOT s, as the old placeholder did).
    let rec_app = app(
        Expr::Const(Name::str("Nat.rec"), vec![u]),
        &[
            const_nat_motive(nat()),
            nat_lit(77),
            lam("n", nat(), lam("ih", nat(), nat_lit(0))),
            cnst("Nat.zero"),
        ],
    );
    assert_eq!(whnf(&env, &rec_app), nat_lit(77));
}
