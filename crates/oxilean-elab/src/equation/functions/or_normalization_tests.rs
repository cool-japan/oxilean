//! Compares `normalize_or_patterns` with its previous form, which flattened
//! the alternatives and folded them from the right, on every or-tree with
//! up to five leaves.

use super::super::types::Pattern;
use super::{flatten_or, normalize_or_patterns};
use oxilean_kernel::Name;

fn previous_normalize_or_patterns(pat: &Pattern) -> Pattern {
    let alts = flatten_or(pat);
    if alts.len() == 1 {
        return alts
            .into_iter()
            .next()
            .expect("alts has exactly one element");
    }
    let mut iter = alts.into_iter().rev();
    let mut result = iter.next().expect("alts is non-empty after flatten_or");
    for p in iter {
        result = Pattern::Or(Box::new(p), Box::new(result));
    }
    result
}

fn leaf(i: usize) -> Pattern {
    match i % 4 {
        0 => Pattern::Var(Name::str(format!("x{}", i))),
        1 => Pattern::Wild,
        2 => Pattern::Ctor(Name::str(format!("C{}", i)), vec![Pattern::Wild]),
        _ => Pattern::As(
            Name::str(format!("a{}", i)),
            Box::new(Pattern::Var(Name::str("y"))),
        ),
    }
}

/// Every binary or-tree over the leaves `first..first + count`.
fn or_trees(first: usize, count: usize) -> Vec<Pattern> {
    if count == 1 {
        return vec![leaf(first)];
    }
    let mut trees = Vec::new();
    for left_count in 1..count {
        for left in or_trees(first, left_count) {
            for right in or_trees(first + left_count, count - left_count) {
                trees.push(Pattern::Or(Box::new(left.clone()), Box::new(right)));
            }
        }
    }
    trees
}

#[test]
fn matches_the_previous_form_on_every_small_or_tree() {
    let mut checked = 0;
    for count in 1..=5 {
        for tree in or_trees(0, count) {
            assert_eq!(
                normalize_or_patterns(&tree),
                previous_normalize_or_patterns(&tree),
                "{:?}",
                tree
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 1 + 1 + 2 + 5 + 14);
}

#[test]
fn nests_alternatives_to_the_right() {
    let tree = Pattern::Or(
        Box::new(Pattern::Or(Box::new(leaf(0)), Box::new(leaf(1)))),
        Box::new(leaf(2)),
    );
    let expected = Pattern::Or(
        Box::new(leaf(0)),
        Box::new(Pattern::Or(Box::new(leaf(1)), Box::new(leaf(2)))),
    );
    assert_eq!(normalize_or_patterns(&tree), expected);
    assert_eq!(normalize_or_patterns(&leaf(3)), leaf(3));
}
