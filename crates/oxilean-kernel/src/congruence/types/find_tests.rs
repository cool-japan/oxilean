//! `CongruenceClosure::find` on first sight, after a union and through a
//! chain of parents.

use super::CongruenceClosure;
use crate::{Expr, Name};

fn constant(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

#[test]
fn find_registers_a_new_expression_and_compresses_paths() {
    let mut cc = CongruenceClosure::new();
    let (a, b, c) = (constant("a"), constant("b"), constant("c"));
    assert_eq!(cc.find(&a), a);
    assert_eq!(cc.parent.get(&a), Some(&a));
    assert_eq!(cc.rank.get(&a), Some(&0));

    cc.union(&a, &b);
    let root = cc.find(&a);
    assert_eq!(cc.find(&b), root);

    // A chain c -> b -> a, written directly, is compressed by one `find`.
    let mut chain = CongruenceClosure::new();
    chain.parent.insert(a.clone(), a.clone());
    chain.parent.insert(b.clone(), a.clone());
    chain.parent.insert(c.clone(), b.clone());
    assert_eq!(chain.find(&c), a);
    assert_eq!(chain.parent.get(&c), Some(&a));
    assert_eq!(chain.rank.get(&c), None);
}
