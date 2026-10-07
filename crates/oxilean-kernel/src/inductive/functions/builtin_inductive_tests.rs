//! `mk_bool_inductive`, `mk_nat_inductive` and `mk_unit_inductive` build
//! exactly what the builder produced for the same name, type and
//! constructors.

use super::*;

#[test]
fn bool_nat_and_unit_equal_the_builder_result() {
    let bool_built = InductiveTypeBuilder::new()
        .name(Name::str("Bool"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(
            Name::str("Bool.true"),
            Expr::Const(Name::str("Bool"), vec![]),
        )
        .intro_rule(
            Name::str("Bool.false"),
            Expr::Const(Name::str("Bool"), vec![]),
        )
        .build();
    assert_eq!(bool_built, Ok(mk_bool_inductive()));

    let nat_built = InductiveTypeBuilder::new()
        .name(Name::str("Nat"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(Name::str("Nat.zero"), Expr::Const(Name::str("Nat"), vec![]))
        .intro_rule(
            Name::str("Nat.succ"),
            Expr::Pi(
                crate::BinderInfo::Default,
                Name::str("n"),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
                Node::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        )
        .build();
    assert_eq!(nat_built, Ok(mk_nat_inductive()));

    let unit_built = InductiveTypeBuilder::new()
        .name(Name::str("Unit"))
        .ty(Expr::Sort(Level::succ(Level::zero())))
        .intro_rule(
            Name::str("Unit.unit"),
            Expr::Const(Name::str("Unit"), vec![]),
        )
        .build();
    assert_eq!(unit_built, Ok(mk_unit_inductive()));

    for ind in [mk_bool_inductive(), mk_nat_inductive(), mk_unit_inductive()] {
        assert!(!ind.is_prop && !ind.is_nested);
        assert_eq!(
            ind.recursor,
            Name::mk_str(ind.name.clone(), "rec".to_string())
        );
    }
}
