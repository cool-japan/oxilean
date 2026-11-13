//! No data out of a proof: the side condition on the type of a projection
//! (Lean 4 kernel `infer_proj`), driven through `TypeChecker::infer_type`,
//! `check_constant_info` and `DefEqChecker::is_def_eq` — the paths a
//! declaration is checked on.
//!
//! When the structure's type is a proposition, the projected field's type
//! must be a proposition, and so must the type of every earlier field a
//! later field's type depends on. Without the condition `h.0` reads the
//! witness out of `h : ∃ n : Nat, True`; proof irrelevance then makes the
//! witnesses of any two proofs definitionally equal, which is `0 = 1`.
//!
//! Every verdict below is the one Lean 4's kernel (v4.32.0-rc1) gives on the
//! same term; the export-level counterpart is the fixture
//! `tests/fixtures/lean4export/ProjOutOfProp.ndjson`.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, ConstructorVal, DefinitionSafety, DefinitionVal,
    InductiveVal,
};
use oxilean_kernel::Node;
use oxilean_kernel::{
    check_constant_info, BinderInfo, DefEqChecker, Environment, Expr, KernelError, Level, Name,
    ReducibilityHint, TypeChecker,
};

// ---------------------------------------------------------------------------
// Expression-building helpers
// ---------------------------------------------------------------------------

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn lvl1() -> Level {
    Level::succ(Level::zero())
}

fn type0() -> Expr {
    Expr::Sort(lvl1())
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn cnst_lvls(name: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(name), levels)
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
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

fn bvar(i: u32) -> Expr {
    Expr::BVar(i)
}

// ---------------------------------------------------------------------------
// Environment-building helpers
// ---------------------------------------------------------------------------

fn common(name: &str, level_params: &[&str], ty: Expr) -> ConstantVal {
    ConstantVal {
        name: Name::str(name),
        level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
        ty,
    }
}

/// A structure-like inductive (one constructor, no indices, not recursive)
/// with `num_params` parameters and `num_fields` fields.
#[allow(clippy::too_many_arguments)]
fn add_structure(
    env: &mut Environment,
    name: &str,
    level_params: &[&str],
    ty: Expr,
    is_prop: bool,
    ctor_ty: Expr,
    num_params: u32,
    num_fields: u32,
) {
    let ctor = format!("{name}.mk");
    env.add_constant(ConstantInfo::Inductive(InductiveVal {
        common: common(name, level_params, ty),
        num_params,
        num_indices: 0,
        all: vec![Name::str(name)],
        ctors: vec![Name::str(&ctor)],
        num_nested: 0,
        is_rec: false,
        is_unsafe: false,
        is_reflexive: false,
        is_prop,
    }))
    .expect("add inductive");
    env.add_constant(ConstantInfo::Constructor(ConstructorVal {
        common: common(&ctor, level_params, ctor_ty),
        induct: Name::str(name),
        cidx: 0,
        num_params,
        num_fields,
        is_unsafe: false,
    }))
    .expect("add constructor");
}

fn add_axiom(env: &mut Environment, name: &str, ty: Expr) {
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: common(name, &[], ty),
        is_unsafe: false,
    }))
    .expect("add axiom");
}

/// `fun _ : N => T` — the predicate of the existential used below.
fn const_true() -> Expr {
    lam("_n", cnst("N"), cnst("T"))
}

/// `Ex.{1} N (fun _ => T)` — `∃ n : N, T`.
fn ex_n_t() -> Expr {
    apps(cnst_lvls("Ex", vec![lvl1()]), vec![cnst("N"), const_true()])
}

/// `Ex.{0} A (fun _ => T)` — `∃ _ : A, T`, an existential over a proposition.
fn ex_a_t() -> Expr {
    apps(
        cnst_lvls("Ex", vec![Level::zero()]),
        vec![cnst("A"), lam("_a", cnst("A"), cnst("T"))],
    )
}

/// The environment of every test:
///
/// * `N : Type` (data) with `n0 : N`, `T : Prop` with `t0 : T`,
///   `A B : Prop`;
/// * `Ex.{u} (α : Sort u) (p : α → Prop) : Prop` with
///   `Ex.mk : (w : α) → p w → Ex α p` — `Exists`: a data field, then a proof
///   field whose type depends on it;
/// * `Conj (a b : Prop) : Prop` with `Conj.mk : a → b → Conj a b` — `And`;
/// * `PD : Prop` with `PD.mk : N → T → PD` — a data field, then a proof
///   field that does not depend on it;
/// * `Sig (α : Type) (β : α → Type) : Type` with
///   `Sig.mk : (fst : α) → β fst → Sig α β` — a structure in `Type` whose
///   second field depends on its first;
/// * `W.{u} (α : Sort u) : Sort u` with `W.mk : α → W α` — a proposition at
///   `u := 0`, a type at `u := 1`;
/// * operands: `he : Ex N (fun _ => T)`, `hq : Ex A (fun _ => T)`,
///   `hc : Conj A B`, `hp : PD`,
///   `F : N → Type`, `s : Sig N F`, `w0 : W.{0} A`, `w1 : W.{1} N`.
fn env() -> Environment {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    add_axiom(&mut env, "N", type0());
    add_axiom(&mut env, "n0", cnst("N"));
    add_axiom(&mut env, "T", prop());
    add_axiom(&mut env, "t0", cnst("T"));
    add_axiom(&mut env, "A", prop());
    add_axiom(&mut env, "B", prop());

    // Ex.{u} : (α : Sort u) → (α → Prop) → Prop
    add_structure(
        &mut env,
        "Ex",
        &["u"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi("p", pi("_a", bvar(0), prop()), prop()),
        ),
        true,
        // (α : Sort u) → (p : α → Prop) → (w : α) → (h : p w) → Ex α p
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "p",
                pi("_a", bvar(0), prop()),
                pi(
                    "w",
                    bvar(1),
                    pi(
                        "h",
                        app(bvar(1), bvar(0)),
                        apps(cnst_lvls("Ex", vec![u.clone()]), vec![bvar(3), bvar(2)]),
                    ),
                ),
            ),
        ),
        2,
        2,
    );

    // Conj : Prop → Prop → Prop
    add_structure(
        &mut env,
        "Conj",
        &[],
        pi("a", prop(), pi("b", prop(), prop())),
        true,
        // (a b : Prop) → (left : a) → (right : b) → Conj a b
        pi(
            "a",
            prop(),
            pi(
                "b",
                prop(),
                pi(
                    "left",
                    bvar(1),
                    pi("right", bvar(1), apps(cnst("Conj"), vec![bvar(3), bvar(2)])),
                ),
            ),
        ),
        2,
        2,
    );

    // PD : Prop, PD.mk : N → T → PD
    add_structure(
        &mut env,
        "PD",
        &[],
        prop(),
        true,
        pi("n", cnst("N"), pi("h", cnst("T"), cnst("PD"))),
        0,
        2,
    );

    // Sig : (α : Type) → (α → Type) → Type
    add_structure(
        &mut env,
        "Sig",
        &[],
        pi(
            "alpha",
            type0(),
            pi("beta", pi("_a", bvar(0), type0()), type0()),
        ),
        false,
        // (α : Type) → (β : α → Type) → (fst : α) → (snd : β fst) → Sig α β
        pi(
            "alpha",
            type0(),
            pi(
                "beta",
                pi("_a", bvar(0), type0()),
                pi(
                    "fst",
                    bvar(1),
                    pi(
                        "snd",
                        app(bvar(1), bvar(0)),
                        apps(cnst("Sig"), vec![bvar(3), bvar(2)]),
                    ),
                ),
            ),
        ),
        2,
        2,
    );

    // W.{u} : Sort u → Sort u, W.mk : (α : Sort u) → α → W α
    add_structure(
        &mut env,
        "W",
        &["u"],
        pi("alpha", Expr::Sort(u.clone()), Expr::Sort(u.clone())),
        false,
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "val",
                bvar(0),
                app(cnst_lvls("W", vec![u.clone()]), bvar(1)),
            ),
        ),
        1,
        1,
    );

    add_axiom(&mut env, "he", ex_n_t());
    add_axiom(&mut env, "hq", ex_a_t());
    add_axiom(
        &mut env,
        "hc",
        apps(cnst("Conj"), vec![cnst("A"), cnst("B")]),
    );
    add_axiom(&mut env, "hp", cnst("PD"));
    add_axiom(&mut env, "F", pi("_a", cnst("N"), type0()));
    add_axiom(&mut env, "s", apps(cnst("Sig"), vec![cnst("N"), cnst("F")]));
    add_axiom(
        &mut env,
        "w0",
        app(cnst_lvls("W", vec![Level::zero()]), cnst("A")),
    );
    add_axiom(&mut env, "w1", app(cnst_lvls("W", vec![lvl1()]), cnst("N")));
    env
}

/// The error of a projection the side condition refuses.
fn assert_refused(result: Result<Expr, KernelError>, what: &str) {
    match result {
        Err(e) => {
            let text = e.to_string();
            assert!(
                text.contains("invalid projection") && text.contains("proposition"),
                "{what}: refused, but not by the side condition on projections: {text}"
            );
        }
        Ok(ty) => panic!("{what}: data projected out of a proof was given the type {ty}"),
    }
}

// ---------------------------------------------------------------------------
// Refused
// ---------------------------------------------------------------------------

/// `he.0` — the witness of an existential: data out of a proof.
#[test]
fn the_witness_of_an_existential_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_refused(tc.infer_type(&proj("Ex", 0, cnst("he"))), "Ex field 0");
}

/// `he.1` — a proof, but its type `p he.0` depends on the data field.
#[test]
fn a_proof_field_whose_type_depends_on_a_data_field_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_refused(tc.infer_type(&proj("Ex", 1, cnst("he"))), "Ex field 1");
}

/// The same in an infer-only checker, which does not type the arguments of
/// an application: there the type `(fun _ => T) he.0` of field 1 is a
/// proposition as far as inference looks, and only the condition on the
/// EARLIER field — the one the type depends on — refuses the projection.
#[test]
fn the_dependent_field_condition_holds_without_checking_arguments() {
    let env = env();
    let mut tc = TypeChecker::new_infer_only(&env);
    assert_refused(
        tc.infer_type(&proj("Ex", 1, cnst("he"))),
        "Ex field 1, infer-only",
    );
    assert_refused(
        tc.infer_type(&proj("Ex", 0, cnst("he"))),
        "Ex field 0, infer-only",
    );
    tc.infer_type(&proj("Ex", 1, cnst("hq")))
        .expect("over a proposition the earlier field is a proof: accepted");
}

/// `hp.0` — the data field of a proposition whose later field does not
/// depend on it is still data.
#[test]
fn the_data_field_of_a_proposition_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_refused(tc.infer_type(&proj("PD", 0, cnst("hp"))), "PD field 0");
}

/// The same under a binder: `fun (h : ∃ n : N, T) => h.0`.
#[test]
fn the_witness_function_is_refused_under_its_binder() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let wit = lam("h", ex_n_t(), proj("Ex", 0, bvar(0)));
    assert_refused(tc.infer_type(&wit), "fun h => h.0");
}

/// The declaration-checking path: the definition
/// `wit : (∃ n : N, T) → N := fun h => h.0` is rejected and not installed.
#[test]
fn the_witness_function_is_rejected_as_a_declaration() {
    let mut env = env();
    let wit = ConstantInfo::Definition(DefinitionVal {
        common: common("wit", &[], pi("h", ex_n_t(), cnst("N"))),
        value: lam("h", ex_n_t(), proj("Ex", 0, bvar(0))),
        hints: ReducibilityHint::Abbrev,
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("wit")],
    });
    let result = check_constant_info(&mut env, wit);
    assert!(
        result.is_err(),
        "a function that reads the witness out of a proof must be rejected"
    );
    assert!(
        env.find(&Name::str("wit")).is_none(),
        "a rejected declaration must not be installed"
    );
}

/// The statement the proof of `False` goes through:
/// `∀ h1 h2 : ∃ n : N, T, h1.0 = h2.0` is not a well-formed type — and with
/// it gone, proof irrelevance has no data to identify.
#[test]
fn a_type_that_mentions_the_witness_is_not_a_type() {
    let mut env = env();
    // Eq' : N → N → Prop (an axiom is enough: only the binder types matter).
    add_axiom(
        &mut env,
        "Eq'",
        pi("_a", cnst("N"), pi("_b", cnst("N"), prop())),
    );
    let stmt = pi(
        "h1",
        ex_n_t(),
        pi(
            "h2",
            ex_n_t(),
            apps(
                cnst("Eq'"),
                vec![proj("Ex", 0, bvar(1)), proj("Ex", 0, bvar(0))],
            ),
        ),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.ensure_sort(&stmt).is_err(),
        "a statement about the witnesses of two proofs must not type-check"
    );
}

/// The def-eq checker's own inference agrees with the type checker's: it
/// gives no type to a projection the type checker refuses, so none of its
/// type-directed rules can fire on one. `he.1` used to be typed
/// `(fun _ => T) he.0`, a proposition, which made it definitionally equal —
/// by proof irrelevance — to any proof of `T`.
#[test]
fn the_def_eq_checker_gives_a_refused_projection_no_type() {
    let env = env();
    let mut dc = DefEqChecker::new(&env);
    assert!(
        !dc.is_def_eq(&proj("Ex", 1, cnst("he")), &cnst("t0")),
        "proof irrelevance fired on a projection the type checker refuses"
    );
}

// ---------------------------------------------------------------------------
// Accepted
// ---------------------------------------------------------------------------

/// `hc.0 : A` and `hc.1 : B` — the proof fields of a conjunction.
#[test]
fn both_fields_of_a_conjunction_are_accepted() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let left = tc
        .infer_type(&proj("Conj", 0, cnst("hc")))
        .expect("Conj field 0 is a proof of a proposition");
    assert_eq!(left, cnst("A"));
    let right = tc
        .infer_type(&proj("Conj", 1, cnst("hc")))
        .expect("Conj field 1 is a proof of a proposition");
    assert_eq!(right, cnst("B"));
}

/// `hq.0 : A` and `hq.1 : (fun _ => T) hq.0` — an existential over a
/// PROPOSITION: its witness is itself a proof, so the very fields refused
/// for `∃ n : N, T` are accepted (Lean core's `Init` carries such
/// projections).
#[test]
fn both_fields_of_an_existential_over_a_proposition_are_accepted() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let witness = tc
        .infer_type(&proj("Ex", 0, cnst("hq")))
        .expect("the witness of an existential over a proposition is a proof");
    assert_eq!(witness, cnst("A"));
    let property = tc
        .infer_type(&proj("Ex", 1, cnst("hq")))
        .expect("the earlier field the property's type depends on is a proof");
    assert_eq!(
        property,
        app(lam("_a", cnst("A"), cnst("T")), proj("Ex", 0, cnst("hq")))
    );
}

/// `s.0 : N` and `s.1 : F s.0` — a structure in `Type` is untouched, data
/// fields and a field that depends on one included.
#[test]
fn every_field_of_a_structure_in_type_is_accepted() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let fst = tc
        .infer_type(&proj("Sig", 0, cnst("s")))
        .expect("Sig field 0 is a field of a structure in Type");
    assert_eq!(fst, cnst("N"));
    let snd = tc
        .infer_type(&proj("Sig", 1, cnst("s")))
        .expect("Sig field 1 is a field of a structure in Type");
    assert_eq!(snd, app(cnst("F"), proj("Sig", 0, cnst("s"))));
}

/// `hp.1 : T` — a proof field after a data field it does not depend on.
#[test]
fn a_proof_field_after_an_independent_data_field_is_accepted() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let h = tc
        .infer_type(&proj("PD", 1, cnst("hp")))
        .expect("PD field 1 is a proof and its type mentions no data field");
    assert_eq!(h, cnst("T"));
}

/// A structure whose sort is a universe parameter: at `u := 0` it is a
/// proposition and its field a proof (`w0.0 : A`), at `u := 1` it is a type
/// and its field data (`w1.0 : N`), and at the parameter itself `Sort u` is
/// not `Prop` (`fun (α : Sort u) (w : W α) => w.0`).
#[test]
fn a_sort_polymorphic_structure_is_accepted_at_every_level() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let at_prop = tc
        .infer_type(&proj("W", 0, cnst("w0")))
        .expect("W.{0} A is a proposition and its field is a proof of A");
    assert_eq!(at_prop, cnst("A"));
    let at_type = tc
        .infer_type(&proj("W", 0, cnst("w1")))
        .expect("W.{1} N is a type");
    assert_eq!(at_type, cnst("N"));
    let u = Level::param(Name::str("u"));
    let poly = lam(
        "alpha",
        Expr::Sort(u.clone()),
        lam(
            "w",
            app(cnst_lvls("W", vec![u]), bvar(0)),
            proj("W", 0, bvar(0)),
        ),
    );
    tc.infer_type(&poly)
        .expect("Sort u is not Prop: the projection is accepted at the parameter");
}

/// The def-eq checker still types the projections that are accepted: two
/// proofs `hc.0` and an axiom of `A` are equal by proof irrelevance.
#[test]
fn the_def_eq_checker_still_types_an_accepted_projection() {
    let mut env = env();
    add_axiom(&mut env, "a0", cnst("A"));
    let mut dc = DefEqChecker::new(&env);
    assert!(
        dc.is_def_eq(&proj("Conj", 0, cnst("hc")), &cnst("a0")),
        "proof irrelevance must still see that hc.0 is a proof of A"
    );
}
