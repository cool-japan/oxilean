//! `match_pattern_in_egraph` for an exact-expression pattern.

use super::*;

fn constant(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

#[test]
fn exact_pattern_matches_the_class_of_a_known_expression() {
    let mut cc = CongruenceClosure::new();
    let a = constant("a");
    let _b = cc.add_term(&constant("b"));
    let a_id = cc.add_term(&a);
    let found = match_pattern_in_egraph(&cc, &EPatternNode::Exact(a), 0);
    assert_eq!(found.len(), 1);
    let expected_class = match cc.get_node(a_id) {
        Some(node) => cc.find_immut(node.eclass),
        None => panic!("a node returned by add_term must exist"),
    };
    assert_eq!(found[0].matched_class, expected_class);
}

#[test]
fn exact_pattern_matches_nothing_for_an_unknown_expression() {
    let mut cc = CongruenceClosure::new();
    let _a = cc.add_term(&constant("a"));
    let found = match_pattern_in_egraph(&cc, &EPatternNode::Exact(constant("missing")), 0);
    assert!(found.is_empty());
}

#[test]
fn exact_pattern_follows_a_merge() {
    let mut cc = CongruenceClosure::new();
    let a_id = cc.add_term(&constant("a"));
    let b_id = cc.add_term(&constant("b"));
    cc.merge_with_reason(a_id, b_id, MergeReason::Assertion);
    let found_a = match_pattern_in_egraph(&cc, &EPatternNode::Exact(constant("a")), 0);
    let found_b = match_pattern_in_egraph(&cc, &EPatternNode::Exact(constant("b")), 0);
    assert_eq!(found_a.len(), 1);
    assert_eq!(found_b.len(), 1);
    assert_eq!(found_a[0].matched_class, found_b[0].matched_class);
}
