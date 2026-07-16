//! Regression: proof irrelevance must use the *normalized* Prop (Sort-zero)
//! test, not a syntactic `Level::is_zero` match (audit item, def_eq/types.rs).
//!
//! A proposition whose sort level is e.g. `imax(u, 0)` normalizes to `0` (it is
//! genuinely a `Prop`), but `imax(u, 0)` is not the *literal* `Level::Zero`.
//! With the old syntactic check such a proof would be denied proof irrelevance;
//! with the fix it is recognised as living in `Prop` and two of its inhabitants
//! are definitionally equal.

#![allow(clippy::all)]

use oxilean_kernel::{Declaration, DefEqChecker, Environment, Expr, Level, Name};

fn axiom(env: &mut Environment, name: &str, univ_params: Vec<Name>, ty: Expr) {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .expect("axiom should be added");
}

#[test]
fn proof_irrelevance_fires_for_imax_u_zero_prop() {
    let mut env = Environment::new();
    let u = Name::str("u");
    // `P.{u} : Sort (imax u 0)`  — a Prop whose sort is NOT the literal Zero.
    let prop_sort = Expr::Sort(Level::imax(Level::param(u.clone()), Level::Zero));
    axiom(&mut env, "P", vec![u.clone()], prop_sort);

    // Two proofs `a b : P.{0}`.
    let p_at_0 = Expr::Const(Name::str("P"), vec![Level::Zero]);
    axiom(&mut env, "a", vec![], p_at_0.clone());
    axiom(&mut env, "b", vec![], p_at_0);

    let mut checker = DefEqChecker::new(&env);
    let a = Expr::Const(Name::str("a"), vec![]);
    let b = Expr::Const(Name::str("b"), vec![]);
    assert!(
        checker.is_def_eq(&a, &b),
        "two proofs of a Prop with sort imax(u,0) must be def-eq by proof irrelevance"
    );
}

#[test]
fn proof_irrelevance_still_excludes_genuine_types() {
    // Control: a term living in `Sort 1` (Type) must NOT be collapsed, even
    // though the sort could be written awkwardly. `T : Sort 1` and two elements
    // are distinct.
    let mut env = Environment::new();
    axiom(&mut env, "T", vec![], Expr::Sort(Level::succ(Level::Zero)));
    axiom(&mut env, "x", vec![], Expr::Const(Name::str("T"), vec![]));
    axiom(&mut env, "y", vec![], Expr::Const(Name::str("T"), vec![]));

    let mut checker = DefEqChecker::new(&env);
    let x = Expr::Const(Name::str("x"), vec![]);
    let y = Expr::Const(Name::str("y"), vec![]);
    assert!(
        !checker.is_def_eq(&x, &y),
        "distinct elements of a Type must not be identified by proof irrelevance"
    );
}
