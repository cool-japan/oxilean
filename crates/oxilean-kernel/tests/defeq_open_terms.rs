//! Regression tests for definitional equality on OPEN terms (loose bound
//! variables under binders), exposed by Wave-3b export replay:
//!
//! 1. `subst::instantiate` must lift a substituted open argument past the
//!    binders it is inserted under (de Bruijn capture avoidance) — without
//!    this, iota/delta reduction inside `Pi` bodies rewrites wrong variables
//!    (seen as a wrong REJECTION of `Nat.brecOn.go` in `simple_add.ndjson`).
//! 2. `Nat` literal ↔ constructor bridging: `Lit 0 ≡ Nat.zero`,
//!    `Lit (n+1) ≡ Nat.succ t` when `Lit n ≡ t` (audit §5.2; seen in
//!    `Parity.isEven` via `OfNat.ofNat Nat 0 (instOfNatNat 0)`).
//! 3. K-like reduction with a Pi-bound major premise: the def-eq checker can
//!    type loose BVars from its binder stack and fire `Eq.rec` K-reduction
//!    where the Reducer's fresh `TypeChecker` cannot (seen as a wrong
//!    REJECTION of `eq_of_heq` in `Parity.isEven`).

use oxilean_kernel::Node;
use oxilean_kernel::{
    check_constant_info, init_builtin_env, instantiate, AxiomVal, BigNat, BinderInfo, ConstantInfo,
    ConstantVal, Environment, Expr, Level, Literal, Name, TypeChecker,
};

fn env_with(axioms: &[(&str, Expr)]) -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    for (name, ty) in axioms {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str(*name),
                level_params: vec![],
                ty: ty.clone(),
            },
            is_unsafe: false,
        });
        check_constant_info(&mut env, ci).unwrap_or_else(|e| panic!("axiom {name}: {e:?}"));
    }
    env
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}

fn lam(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(ty),
        Node::new(body),
    )
}

fn pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(ty),
        Node::new(body),
    )
}

// ---------------------------------------------------------------------------
// 1. Capture avoidance in `instantiate`.
// ---------------------------------------------------------------------------

#[test]
fn instantiate_lifts_open_arg_under_binder() {
    // body = λ y : Prop. x   (x = BVar(1) inside the lambda)
    // instantiate x := #0 (an OPEN variable of the surrounding context):
    // the argument must be lifted past the `y` binder → λ y : Prop. #1.
    let body = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    let result = instantiate(&body, &Expr::BVar(0));
    let expected = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    assert_eq!(result, expected, "open arg must not be captured by `y`");
}

#[test]
fn instantiate_closed_arg_behavior_unchanged() {
    // Closed arguments are unaffected by lifting.
    let body = lam("y", Expr::Sort(Level::zero()), Expr::BVar(1));
    let arg = Expr::Const(Name::str("Nat"), vec![]);
    let result = instantiate(&body, &arg);
    let expected = lam("y", Expr::Sort(Level::zero()), arg);
    assert_eq!(result, expected);
}

// ---------------------------------------------------------------------------
// 2. Nat literal ↔ constructor bridging.
// ---------------------------------------------------------------------------

#[test]
fn nat_literal_constructor_bridge() {
    let env = env_with(&[]);
    let mut tc = TypeChecker::new(&env);
    let lit = |n: u64| Expr::Lit(Literal::Nat(BigNat::from(n)));
    let zero = Expr::Const(Name::str("Nat.zero"), vec![]);
    let succ = |e: Expr| app(Expr::Const(Name::str("Nat.succ"), vec![]), e);

    assert!(tc.is_def_eq(&lit(0), &zero), "0 ≡ Nat.zero");
    assert!(tc.is_def_eq(&zero, &lit(0)), "Nat.zero ≡ 0 (symmetric)");
    assert!(
        tc.is_def_eq(&lit(2), &succ(succ(zero.clone()))),
        "2 ≡ Nat.succ (Nat.succ Nat.zero)"
    );
    assert!(
        tc.is_def_eq(&succ(lit(1)), &lit(2)),
        "Nat.succ 1 ≡ 2 (mixed)"
    );
    assert!(!tc.is_def_eq(&lit(1), &zero), "1 ≢ Nat.zero");
    assert!(
        !tc.is_def_eq(&lit(3), &succ(succ(zero))),
        "3 ≢ Nat.succ (Nat.succ Nat.zero)"
    );
}

#[test]
fn nat_literal_bridge_under_stuck_congruence() {
    // F 0 ≡ F Nat.zero for an opaque F : Nat → Prop (argument congruence
    // must bridge the literal).
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let env = env_with(&[("F", pi("n", nat, Expr::Sort(Level::zero())))]);
    let mut tc = TypeChecker::new(&env);
    let f = Expr::Const(Name::str("F"), vec![]);
    let t = app(f.clone(), Expr::Lit(Literal::Nat(BigNat::from(0u64))));
    let s = app(f, Expr::Const(Name::str("Nat.zero"), vec![]));
    assert!(tc.is_def_eq(&t, &s), "F 0 ≡ F Nat.zero");
}

// ---------------------------------------------------------------------------
// 3. K-reduction with a Pi-bound major premise.
// ---------------------------------------------------------------------------

#[test]
fn k_reduction_fires_on_pi_bound_major() {
    // A : Type, a : A, P : A → Prop.
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let a_ty = Expr::Const(Name::str("A"), vec![]);
    let env = env_with(&[
        ("A", type0),
        ("a", Expr::Const(Name::str("A"), vec![])),
        (
            "P",
            pi(
                "x",
                Expr::Const(Name::str("A"), vec![]),
                Expr::Sort(Level::zero()),
            ),
        ),
    ]);
    let u1 = Level::succ(Level::zero());
    let a = Expr::Const(Name::str("a"), vec![]);
    let p = Expr::Const(Name::str("P"), vec![]);
    let eq_a_a = apps(
        Expr::Const(Name::str("Eq"), vec![u1.clone()]),
        vec![a_ty.clone(), a.clone(), a.clone()],
    );
    // motive : (b : A) → Eq A a b → Sort 1  :=  λ b h, A
    let motive = lam(
        "b",
        a_ty.clone(),
        lam(
            "h",
            apps(
                Expr::Const(Name::str("Eq"), vec![u1.clone()]),
                vec![a_ty.clone(), a.clone(), Expr::BVar(0)],
            ),
            a_ty.clone(),
        ),
    );
    // @Eq.rec.{1, 1} A a motive (minor := a) (b := a) (h := #0, Pi-bound).
    let rec_app = apps(
        Expr::Const(Name::str("Eq.rec"), vec![u1.clone(), u1]),
        vec![a_ty, a.clone(), motive, a.clone(), a.clone(), Expr::BVar(0)],
    );
    // Π h : Eq A a a, P (Eq.rec … h)  ≡  Π h : Eq A a a, P a
    // requires K-reduction on the STUCK major `h` (typed from the binder).
    let t = pi("h", eq_a_a.clone(), app(p.clone(), rec_app));
    let s = pi("h", eq_a_a, app(p, a));
    let mut tc = TypeChecker::new(&env);
    assert!(
        tc.is_def_eq(&t, &s),
        "K-reduction must fire under the binder"
    );
}
