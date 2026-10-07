//! `monomial_term_to_expr`: the expression built for `coeff * x1^e1 * ...`.

use super::*;
use crate::tactic::ring::Monomial;

fn app2(f: &str, a: Expr, b: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str(f), vec![])),
            Node::new(a),
        )),
        Node::new(b),
    )
}

fn var(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn nat(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

#[test]
fn constant_monomial_is_just_its_coefficient() {
    let one = monomial_term_to_expr(&Monomial::constant(), 1, 1);
    assert_eq!(one, nat(1));
    let seven = monomial_term_to_expr(&Monomial::constant(), 7, 1);
    assert_eq!(seven, nat(7));
    let neg = monomial_term_to_expr(&Monomial::constant(), -3, 1);
    assert_eq!(
        neg,
        Expr::App(
            Node::new(Expr::Const(Name::str("Neg.neg"), vec![])),
            Node::new(nat(3)),
        )
    );
    let frac = monomial_term_to_expr(&Monomial::constant(), 1, 2);
    assert_eq!(frac, app2("HDiv.hDiv", nat(1), nat(2)));
}

#[test]
fn single_variable_with_unit_coefficient_is_the_variable() {
    let mono = Monomial::var(Name::str("x"));
    assert_eq!(monomial_term_to_expr(&mono, 1, 1), var("x"));
}

#[test]
fn single_variable_power_uses_hpow() {
    let mono = Monomial::var_exp(Name::str("x"), 3);
    assert_eq!(
        monomial_term_to_expr(&mono, 1, 1),
        app2("HPow.hPow", var("x"), nat(3))
    );
}

#[test]
fn coefficient_multiplies_the_variable_part() {
    let mono = Monomial::var(Name::str("x"));
    assert_eq!(
        monomial_term_to_expr(&mono, 4, 1),
        app2("Nat.mul", nat(4), var("x"))
    );
}

#[test]
fn several_variables_are_multiplied_left_to_right() {
    let mono = Monomial {
        exponents: vec![
            (Name::str("x"), 1),
            (Name::str("y"), 2),
            (Name::str("z"), 1),
        ],
    };
    let expected = app2(
        "Nat.mul",
        app2("Nat.mul", var("x"), app2("HPow.hPow", var("y"), nat(2))),
        var("z"),
    );
    assert_eq!(monomial_term_to_expr(&mono, 1, 1), expected);
}
