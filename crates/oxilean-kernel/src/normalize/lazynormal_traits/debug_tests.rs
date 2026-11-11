//! `LazyNormal`'s `Debug` output before and after the normal form is computed.

use super::LazyNormal;
use crate::{BinderInfo, Expr, Level, Name, Node};

#[test]
fn debug_shows_the_pending_original_then_the_evaluated_normal_form() {
    let identity = Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(Expr::Sort(Level::zero())),
        Node::new(Expr::BVar(0)),
    );
    let redex = Expr::App(Node::new(identity), Node::new(Expr::Sort(Level::zero())));
    let lazy = LazyNormal::new(redex.clone());
    assert_eq!(
        format!("{lazy:?}"),
        format!("LazyNormal::Pending({redex:?})")
    );
    let normal = lazy.normalized().clone();
    assert_eq!(
        format!("{lazy:?}"),
        format!("LazyNormal::Evaluated({normal:?})")
    );
    assert_ne!(normal, redex);
}
