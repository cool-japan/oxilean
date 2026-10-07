//! Compares `build_match` with its previous form for zero to four arms.

use super::{build_ite, build_match};
use oxilean_kernel::{Expr, Name, Node};

fn previous_build_match(scrutinee: &Expr, arms: &[(Name, Expr)]) -> Expr {
    if arms.is_empty() {
        return Expr::Const(Name::str("absurd"), vec![]);
    }
    if arms.len() == 1 {
        let (_pat, body) = &arms[0];
        return Expr::Let(
            Name::str("_"),
            Node::new(Expr::Const(Name::str("_"), vec![])),
            Node::new(scrutinee.clone()),
            Node::new(body.clone()),
        );
    }
    let match_name = Name::str("_match");
    let mut result = arms
        .last()
        .expect("arms is non-empty (checked above)")
        .1
        .clone();
    for (pat, body) in arms.iter().rev().skip(1) {
        let eq_check = Expr::App(
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("BEq.beq"), vec![])),
                Node::new(scrutinee.clone()),
            )),
            Node::new(Expr::Const(pat.clone(), vec![])),
        );
        result = build_ite(&eq_check, body, &result);
    }
    Expr::Let(
        match_name,
        Node::new(Expr::Const(Name::str("_"), vec![])),
        Node::new(scrutinee.clone()),
        Node::new(result),
    )
}

#[test]
fn matches_the_previous_form_for_zero_to_four_arms() {
    let scrutinee = Expr::Const(Name::str("s"), vec![]);
    let arms: Vec<(Name, Expr)> = (0..4)
        .map(|i| {
            (
                Name::str(format!("P{}", i)),
                Expr::Const(Name::str(format!("body{}", i)), vec![]),
            )
        })
        .collect();
    for count in 0..=arms.len() {
        assert_eq!(
            build_match(&scrutinee, &arms[..count]),
            previous_build_match(&scrutinee, &arms[..count]),
            "{} arms",
            count
        );
    }
}
