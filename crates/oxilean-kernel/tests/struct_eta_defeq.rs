//! End-to-end tests for definitional eta for structures, unit-like
//! definitional equality, multi-binder function eta, and strict projection
//! typing — all driven through `TypeChecker::is_def_eq` / `infer_type`
//! (the declaration-checking path), not module-level unit helpers.
//!
//! Semantics target: Lean 4 kernel `isDefEqEtaStruct`, `isDefEqUnitLike`,
//! `tryEtaExpansionCore`, and `inferProj` (audit items C1, C2, S11).

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, ConstructorVal, InductiveVal,
};
use oxilean_kernel::Node;
use oxilean_kernel::{BinderInfo, DefEqChecker, Environment, Expr, Level, Name, TypeChecker};

// ---------------------------------------------------------------------------
// Expression-building helpers
// ---------------------------------------------------------------------------

fn sort0() -> Expr {
    Expr::Sort(Level::zero())
}

fn sort1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

fn lvl1() -> Level {
    Level::succ(Level::zero())
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

#[allow(clippy::too_many_arguments)]
fn add_inductive(
    env: &mut Environment,
    name: &str,
    level_params: &[&str],
    ty: Expr,
    num_params: u32,
    num_indices: u32,
    is_rec: bool,
    ctors: &[&str],
) {
    env.add_constant(ConstantInfo::Inductive(InductiveVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        num_params,
        num_indices,
        all: vec![Name::str(name)],
        ctors: ctors.iter().map(|c| Name::str(*c)).collect(),
        num_nested: 0,
        is_rec,
        is_unsafe: false,
        is_reflexive: false,
        is_prop: false,
    }))
    .expect("add inductive");
}

#[allow(clippy::too_many_arguments)]
fn add_ctor(
    env: &mut Environment,
    name: &str,
    level_params: &[&str],
    ty: Expr,
    induct: &str,
    cidx: u32,
    num_params: u32,
    num_fields: u32,
) {
    env.add_constant(ConstantInfo::Constructor(ConstructorVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: level_params.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        induct: Name::str(induct),
        cidx,
        num_params,
        num_fields,
        is_unsafe: false,
    }))
    .expect("add constructor");
}

fn add_axiom(env: &mut Environment, name: &str, ty: Expr) {
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    }))
    .expect("add axiom");
}

/// The audit probe's 0-parameter structure: `MyPair` with two `Sort 0` fields
/// and an axiom `e : MyPair`.
fn env_mypair() -> Environment {
    let mut env = Environment::new();
    add_inductive(
        &mut env,
        "MyPair",
        &[],
        sort1(),
        0,
        0,
        false,
        &["MyPair.mk"],
    );
    add_ctor(
        &mut env,
        "MyPair.mk",
        &[],
        pi("fst", sort0(), pi("snd", sort0(), cnst("MyPair"))),
        "MyPair",
        0,
        0,
        2,
    );
    add_axiom(&mut env, "e", cnst("MyPair"));
    env
}

/// Universe-polymorphic two-parameter structure:
/// `MyProd.{u,v} (α : Sort u) (β : Sort v)` with fields `fst : α`, `snd : β`,
/// instantiated at `{1,1}` with axioms `A B : Sort 1`, `p : MyProd.{1,1} A B`.
fn env_myprod() -> Environment {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let v = Level::param(Name::str("v"));
    add_inductive(
        &mut env,
        "MyProd",
        &["u", "v"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "beta",
                Expr::Sort(v.clone()),
                Expr::Sort(Level::max(u.clone(), v.clone())),
            ),
        ),
        2,
        0,
        false,
        &["MyProd.mk"],
    );
    add_ctor(
        &mut env,
        "MyProd.mk",
        &["u", "v"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "beta",
                Expr::Sort(v.clone()),
                pi(
                    "fst",
                    bvar(1),
                    pi(
                        "snd",
                        bvar(1),
                        apps(cnst_lvls("MyProd", vec![u, v]), vec![bvar(3), bvar(2)]),
                    ),
                ),
            ),
        ),
        "MyProd",
        0,
        2,
        2,
    );
    add_axiom(&mut env, "A", sort1());
    add_axiom(&mut env, "B", sort1());
    add_axiom(
        &mut env,
        "p",
        apps(
            cnst_lvls("MyProd", vec![lvl1(), lvl1()]),
            vec![cnst("A"), cnst("B")],
        ),
    );
    add_axiom(&mut env, "qa", cnst("A"));
    env
}

/// The eta-expansion of `p` at `MyProd.{1,1} A B`.
fn myprod_expansion() -> Expr {
    apps(
        cnst_lvls("MyProd.mk", vec![lvl1(), lvl1()]),
        vec![
            cnst("A"),
            cnst("B"),
            proj("MyProd", 0, cnst("p")),
            proj("MyProd", 1, cnst("p")),
        ],
    )
}

/// Nested structures: `Outer` with field 0 of structure type `Inner`.
fn env_nested() -> Environment {
    let mut env = Environment::new();
    add_inductive(&mut env, "Inner", &[], sort1(), 0, 0, false, &["Inner.mk"]);
    add_ctor(
        &mut env,
        "Inner.mk",
        &[],
        pi("a", sort0(), pi("b", sort0(), cnst("Inner"))),
        "Inner",
        0,
        0,
        2,
    );
    add_inductive(&mut env, "Outer", &[], sort1(), 0, 0, false, &["Outer.mk"]);
    add_ctor(
        &mut env,
        "Outer.mk",
        &[],
        pi("x", cnst("Inner"), pi("y", sort0(), cnst("Outer"))),
        "Outer",
        0,
        0,
        2,
    );
    add_axiom(&mut env, "w", cnst("Outer"));
    env
}

/// Sigma-like dependent structure:
/// `MySig.{u,v} (α : Sort u) (β : α → Sort v)` with `fst : α`, `snd : β fst`,
/// instantiated with `A : Sort 1`, `Bfam : A → Sort 1`, `z : MySig.{1,1} A Bfam`.
fn env_mysig() -> Environment {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    let v = Level::param(Name::str("v"));
    add_inductive(
        &mut env,
        "MySig",
        &["u", "v"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "beta",
                pi("_x", bvar(0), Expr::Sort(v.clone())),
                Expr::Sort(Level::max(u.clone(), v.clone())),
            ),
        ),
        2,
        0,
        false,
        &["MySig.mk"],
    );
    add_ctor(
        &mut env,
        "MySig.mk",
        &["u", "v"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            pi(
                "beta",
                pi("_x", bvar(0), Expr::Sort(v.clone())),
                pi(
                    "fst",
                    bvar(1),
                    pi(
                        "snd",
                        app(bvar(1), bvar(0)),
                        apps(cnst_lvls("MySig", vec![u, v]), vec![bvar(3), bvar(2)]),
                    ),
                ),
            ),
        ),
        "MySig",
        0,
        2,
        2,
    );
    add_axiom(&mut env, "A", sort1());
    add_axiom(&mut env, "Bfam", pi("_x", cnst("A"), sort1()));
    add_axiom(
        &mut env,
        "z",
        apps(
            cnst_lvls("MySig", vec![lvl1(), lvl1()]),
            vec![cnst("A"), cnst("Bfam")],
        ),
    );
    env
}

fn mysig_expansion() -> Expr {
    apps(
        cnst_lvls("MySig.mk", vec![lvl1(), lvl1()]),
        vec![
            cnst("A"),
            cnst("Bfam"),
            proj("MySig", 0, cnst("z")),
            proj("MySig", 1, cnst("z")),
        ],
    )
}

/// Unit-like types: `MyUnit` (0 fields, 0 params) with axioms `u1 u2 : MyUnit`,
/// and the parameterized `MyWrap.{u} (α : Sort u)` (0 fields, 1 param) with
/// `w1 w2 : MyWrap.{1} A` and `x2 : MyWrap.{1} A2`.
fn env_unitlike() -> Environment {
    let mut env = Environment::new();
    add_inductive(
        &mut env,
        "MyUnit",
        &[],
        sort1(),
        0,
        0,
        false,
        &["MyUnit.unit"],
    );
    add_ctor(
        &mut env,
        "MyUnit.unit",
        &[],
        cnst("MyUnit"),
        "MyUnit",
        0,
        0,
        0,
    );
    add_axiom(&mut env, "u1", cnst("MyUnit"));
    add_axiom(&mut env, "u2", cnst("MyUnit"));
    let u = Level::param(Name::str("u"));
    add_inductive(
        &mut env,
        "MyWrap",
        &["u"],
        pi("alpha", Expr::Sort(u.clone()), Expr::Sort(u.clone())),
        1,
        0,
        false,
        &["MyWrap.mk"],
    );
    add_ctor(
        &mut env,
        "MyWrap.mk",
        &["u"],
        pi(
            "alpha",
            Expr::Sort(u.clone()),
            app(cnst_lvls("MyWrap", vec![u]), bvar(0)),
        ),
        "MyWrap",
        0,
        1,
        0,
    );
    add_axiom(&mut env, "A", sort1());
    add_axiom(&mut env, "A2", sort1());
    add_axiom(
        &mut env,
        "w1",
        app(cnst_lvls("MyWrap", vec![lvl1()]), cnst("A")),
    );
    add_axiom(
        &mut env,
        "w2",
        app(cnst_lvls("MyWrap", vec![lvl1()]), cnst("A")),
    );
    add_axiom(
        &mut env,
        "x2",
        app(cnst_lvls("MyWrap", vec![lvl1()]), cnst("A2")),
    );
    env
}

/// Negative-test environment: a 2-constructor inductive (`MySum`), a
/// recursive single-constructor type (`MyStream`), and a single-constructor
/// indexed family (`MyIdx`). None of them may eta.
fn env_negatives() -> Environment {
    let mut env = Environment::new();
    add_inductive(
        &mut env,
        "MySum",
        &[],
        sort1(),
        0,
        0,
        false,
        &["MySum.inl", "MySum.inr"],
    );
    add_ctor(
        &mut env,
        "MySum.inl",
        &[],
        pi("a", sort0(), cnst("MySum")),
        "MySum",
        0,
        0,
        1,
    );
    add_ctor(
        &mut env,
        "MySum.inr",
        &[],
        pi("b", sort0(), cnst("MySum")),
        "MySum",
        1,
        0,
        1,
    );
    add_axiom(&mut env, "m", cnst("MySum"));
    add_axiom(&mut env, "m2", cnst("MySum"));
    add_axiom(&mut env, "a", sort0());
    // Recursive single-constructor type (a Stream-like): must NOT eta.
    add_inductive(
        &mut env,
        "MyStream",
        &[],
        sort1(),
        0,
        0,
        true,
        &["MyStream.cons"],
    );
    add_ctor(
        &mut env,
        "MyStream.cons",
        &[],
        pi("h", sort0(), pi("t", cnst("MyStream"), cnst("MyStream"))),
        "MyStream",
        0,
        0,
        2,
    );
    add_axiom(&mut env, "st", cnst("MyStream"));
    // Single-constructor indexed family: must NOT eta.
    add_inductive(
        &mut env,
        "MyIdx",
        &[],
        pi("i", sort0(), sort1()),
        0,
        1,
        false,
        &["MyIdx.mk"],
    );
    add_ctor(
        &mut env,
        "MyIdx.mk",
        &[],
        pi("i", sort0(), pi("x", sort0(), app(cnst("MyIdx"), bvar(1)))),
        "MyIdx",
        0,
        0,
        2,
    );
    add_axiom(&mut env, "i0", sort0());
    add_axiom(&mut env, "ix", app(cnst("MyIdx"), cnst("i0")));
    env
}

// ---------------------------------------------------------------------------
// (a)/(b) Parameterized, universe-polymorphic structure eta, both orientations
// ---------------------------------------------------------------------------

#[test]
fn struct_eta_polymorphic_two_field() {
    let env = env_myprod();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("p"), &myprod_expansion()),
        "p must be def-eq to MyProd.mk.{{1,1}} A B p.0 p.1"
    );
}

#[test]
fn struct_eta_polymorphic_two_field_reverse() {
    let env = env_myprod();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&myprod_expansion(), &cnst("p")),
        "MyProd.mk.{{1,1}} A B p.0 p.1 must be def-eq to p (symmetric orientation)"
    );
}

#[test]
fn struct_eta_swapped_fields_not_def_eq() {
    let env = env_myprod();
    let mut tc = TypeChecker::new(&env);
    let swapped = apps(
        cnst_lvls("MyProd.mk", vec![lvl1(), lvl1()]),
        vec![
            cnst("A"),
            cnst("B"),
            proj("MyProd", 1, cnst("p")),
            proj("MyProd", 0, cnst("p")),
        ],
    );
    assert!(
        !tc.is_def_eq(&cnst("p"), &swapped),
        "p must NOT be def-eq to MyProd.mk A B p.1 p.0"
    );
}

// ---------------------------------------------------------------------------
// (c) Nested structures
// ---------------------------------------------------------------------------

#[test]
fn struct_eta_nested() {
    let env = env_nested();
    let mut tc = TypeChecker::new(&env);
    let w = cnst("w");
    let w0 = proj("Outer", 0, w.clone());
    let inner_expansion = apps(
        cnst("Inner.mk"),
        vec![proj("Inner", 0, w0.clone()), proj("Inner", 1, w0)],
    );
    let outer_expansion = apps(
        cnst("Outer.mk"),
        vec![inner_expansion, proj("Outer", 1, w.clone())],
    );
    assert!(
        tc.is_def_eq(&w, &outer_expansion),
        "w must be def-eq to Outer.mk (Inner.mk w.0.0 w.0.1) w.1"
    );
    assert!(
        tc.is_def_eq(&outer_expansion, &w),
        "nested eta must also hold in the reverse orientation"
    );
}

// ---------------------------------------------------------------------------
// (d) Dependent second field (Sigma-like)
// ---------------------------------------------------------------------------

#[test]
fn struct_eta_dependent_sigma() {
    let env = env_mysig();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("z"), &mysig_expansion()),
        "z must be def-eq to MySig.mk.{{1,1}} A Bfam z.0 z.1"
    );
    assert!(
        tc.is_def_eq(&mysig_expansion(), &cnst("z")),
        "Sigma-like eta must also hold in the reverse orientation"
    );
}

#[test]
fn infer_dependent_field_type() {
    let env = env_mysig();
    let mut tc = TypeChecker::new(&env);
    let snd_ty = tc
        .infer_type(&proj("MySig", 1, cnst("z")))
        .expect("z.1 must typecheck");
    let expected = app(cnst("Bfam"), proj("MySig", 0, cnst("z")));
    assert!(
        tc.is_def_eq(&snd_ty, &expected),
        "type of z.1 must be Bfam z.0, got {snd_ty}"
    );
}

// ---------------------------------------------------------------------------
// (e) Unit-like types
// ---------------------------------------------------------------------------

#[test]
fn unit_like_two_axioms_def_eq() {
    let env = env_unitlike();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("u1"), &cnst("u2")),
        "two elements of a unit-like type must be def-eq"
    );
}

#[test]
fn unit_like_axiom_vs_constructor() {
    let env = env_unitlike();
    let mut tc = TypeChecker::new(&env);
    assert!(tc.is_def_eq(&cnst("u1"), &cnst("MyUnit.unit")));
    assert!(tc.is_def_eq(&cnst("MyUnit.unit"), &cnst("u1")));
}

#[test]
fn unit_like_parameterized() {
    let env = env_unitlike();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("w1"), &cnst("w2")),
        "two elements of MyWrap.{{1}} A must be def-eq"
    );
    assert!(
        !tc.is_def_eq(&cnst("w1"), &cnst("x2")),
        "elements of MyWrap A and MyWrap A2 must NOT be def-eq (types differ)"
    );
}

// ---------------------------------------------------------------------------
// (f) Negative gates: 2-constructor, recursive, indexed
// ---------------------------------------------------------------------------

#[test]
fn no_eta_for_two_constructor_inductive() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    assert!(
        !tc.is_def_eq(&cnst("m"), &app(cnst("MySum.inl"), cnst("a"))),
        "a 2-constructor inductive must not eta"
    );
    assert!(
        !tc.is_def_eq(&cnst("m"), &cnst("m2")),
        "unit-like equality must not fire on a 2-constructor inductive"
    );
}

#[test]
fn no_eta_for_recursive_single_constructor() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    let st = cnst("st");
    let expansion = apps(
        cnst("MyStream.cons"),
        vec![
            proj("MyStream", 0, st.clone()),
            proj("MyStream", 1, st.clone()),
        ],
    );
    assert!(
        !tc.is_def_eq(&st, &expansion),
        "a recursive single-constructor type must not eta (is_rec gate)"
    );
}

#[test]
fn no_eta_for_indexed_family() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    let ix = cnst("ix");
    let expansion = apps(
        cnst("MyIdx.mk"),
        vec![proj("MyIdx", 0, ix.clone()), proj("MyIdx", 1, ix.clone())],
    );
    assert!(
        !tc.is_def_eq(&ix, &expansion),
        "an indexed single-constructor family must not eta (num_indices gate)"
    );
}

// ---------------------------------------------------------------------------
// (g) Multi-binder function eta
// ---------------------------------------------------------------------------

#[test]
fn fun_eta_two_binders() {
    let mut env = Environment::new();
    add_axiom(&mut env, "g", pi("x", sort0(), pi("y", sort0(), sort0())));
    let g = cnst("g");
    let lam2 = lam(
        "x",
        sort0(),
        lam("y", sort0(), apps(g.clone(), vec![bvar(1), bvar(0)])),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&lam2, &g),
        "(fun x y => g x y) must be def-eq to g"
    );
    assert!(
        tc.is_def_eq(&g, &lam2),
        "g must be def-eq to (fun x y => g x y)"
    );
}

#[test]
fn fun_eta_three_binders() {
    let mut env = Environment::new();
    add_axiom(
        &mut env,
        "h",
        pi("x", sort0(), pi("y", sort0(), pi("z", sort0(), sort0()))),
    );
    let h = cnst("h");
    let lam3 = lam(
        "x",
        sort0(),
        lam(
            "y",
            sort0(),
            lam(
                "z",
                sort0(),
                apps(h.clone(), vec![bvar(2), bvar(1), bvar(0)]),
            ),
        ),
    );
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&lam3, &h),
        "(fun x y z => h x y z) must be def-eq to h"
    );
    assert!(
        tc.is_def_eq(&h, &lam3),
        "h must be def-eq to (fun x y z => h x y z)"
    );
}

// ---------------------------------------------------------------------------
// (h) S11: projection typing is strict — typed errors, no fabricated types
// ---------------------------------------------------------------------------

#[test]
fn infer_proj_wrong_operand_type_is_error() {
    let env = env_myprod();
    let mut tc = TypeChecker::new(&env);
    // qa : A, but the projection claims MyProd.
    assert!(
        tc.infer_type(&proj("MyProd", 0, cnst("qa"))).is_err(),
        "projecting from a non-MyProd operand must be a typed error"
    );
}

#[test]
fn infer_proj_two_constructor_is_error() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.infer_type(&proj("MySum", 0, cnst("m"))).is_err(),
        "projections on a 2-constructor inductive must be rejected"
    );
}

#[test]
fn infer_proj_recursive_is_error() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.infer_type(&proj("MyStream", 0, cnst("st"))).is_err(),
        "projections on a recursive single-constructor type must be rejected"
    );
}

#[test]
fn infer_proj_indexed_is_error() {
    let env = env_negatives();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.infer_type(&proj("MyIdx", 0, cnst("ix"))).is_err(),
        "projections on an indexed family must be rejected"
    );
}

#[test]
fn infer_proj_out_of_range_is_error() {
    let env = env_mypair();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.infer_type(&proj("MyPair", 5, cnst("e"))).is_err(),
        "out-of-range field index must be a typed error"
    );
}

#[test]
fn infer_proj_positive_control() {
    let env = env_myprod();
    let mut tc = TypeChecker::new(&env);
    let fst_ty = tc
        .infer_type(&proj("MyProd", 0, cnst("p")))
        .expect("p.0 must typecheck");
    assert!(tc.is_def_eq(&fst_ty, &cnst("A")), "type of p.0 must be A");
    let snd_ty = tc
        .infer_type(&proj("MyProd", 1, cnst("p")))
        .expect("p.1 must typecheck");
    assert!(tc.is_def_eq(&snd_ty, &cnst("B")), "type of p.1 must be B");
}

// ---------------------------------------------------------------------------
// (i) Regression: the audit's empirical probe, via both public entry points
// ---------------------------------------------------------------------------

fn mypair_expansion() -> Expr {
    apps(
        cnst("MyPair.mk"),
        vec![proj("MyPair", 0, cnst("e")), proj("MyPair", 1, cnst("e"))],
    )
}

#[test]
fn probe_struct_eta_defeqchecker() {
    let env = env_mypair();
    let mut checker = DefEqChecker::new(&env);
    assert!(
        checker.is_def_eq(&cnst("e"), &mypair_expansion()),
        "PROBE: e == MyPair.mk e.0 e.1 must hold"
    );
    let mut checker2 = DefEqChecker::new(&env);
    assert!(
        checker2.is_def_eq(&mypair_expansion(), &cnst("e")),
        "PROBE: MyPair.mk e.0 e.1 == e (symmetric) must hold"
    );
}

#[test]
fn probe_struct_eta_typechecker_path() {
    let env = env_mypair();
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&cnst("e"), &mypair_expansion()),
        "PROBE: TC path e == MyPair.mk e.0 e.1 must hold"
    );
}

#[test]
fn probe_proj_ctor_iota_still_works() {
    let env = env_mypair();
    let mut tc = TypeChecker::new(&env);
    let ctor_app = apps(cnst("MyPair.mk"), vec![cnst("e"), cnst("e")]);
    assert!(
        tc.is_def_eq(&proj("MyPair", 0, ctor_app), &cnst("e")),
        "PROBE: (MyPair.mk e e).0 == e must still hold"
    );
}

#[test]
fn probe_single_binder_fun_eta_still_works() {
    let mut env = env_mypair();
    add_axiom(&mut env, "f", pi("x", sort0(), sort0()));
    let f = cnst("f");
    let lam1 = lam("x", sort0(), app(f.clone(), bvar(0)));
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&lam1, &f),
        "PROBE: (fun x => f x) == f must hold"
    );
    assert!(
        tc.is_def_eq(&f, &lam1),
        "PROBE: f == (fun x => f x) must hold"
    );
}

#[test]
fn probe_unit_like_vs_ctor() {
    let env = env_unitlike();
    let mut checker = DefEqChecker::new(&env);
    assert!(
        checker.is_def_eq(&cnst("u1"), &cnst("MyUnit.unit")),
        "PROBE: u1 == MyUnit.unit must hold (isDefEqUnitLike)"
    );
}
