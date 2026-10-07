//! `avl_rebalance` and `avl_insert` on ordinary inputs (trees built by
//! insertion, and one rebalance of an unbalanced tree whose stored heights are
//! correct) against a verbatim copy of the rebalancing in this module.

use super::{avl_balance_factor, avl_insert, avl_rebalance, avl_update_height, AvlNode};

fn copy_rotate_right(mut y: Box<AvlNode<u32>>) -> Box<AvlNode<u32>> {
    let mut x = y.left.take().expect("left child");
    y.left = x.right.take();
    avl_update_height(&mut y);
    x.right = Some(y);
    avl_update_height(&mut x);
    x
}
fn copy_rotate_left(mut x: Box<AvlNode<u32>>) -> Box<AvlNode<u32>> {
    let mut y = x.right.take().expect("right child");
    x.right = y.left.take();
    avl_update_height(&mut x);
    y.left = Some(x);
    avl_update_height(&mut y);
    y
}
fn copy_rebalance(mut node: Box<AvlNode<u32>>) -> Box<AvlNode<u32>> {
    avl_update_height(&mut node);
    let bf = avl_balance_factor(&node);
    if bf > 1 {
        if avl_balance_factor(node.left.as_ref().expect("left child")) < 0 {
            let left = node.left.take().expect("left child");
            node.left = Some(copy_rotate_left(left));
        }
        return copy_rotate_right(node);
    }
    if bf < -1 {
        if avl_balance_factor(node.right.as_ref().expect("right child")) > 0 {
            let right = node.right.take().expect("right child");
            node.right = Some(copy_rotate_right(right));
        }
        return copy_rotate_left(node);
    }
    node
}
fn copy_insert(node: Option<Box<AvlNode<u32>>>, value: u32) -> Box<AvlNode<u32>> {
    match node {
        None => AvlNode::new(value),
        Some(mut n) => {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => n.left = Some(copy_insert(n.left.take(), value)),
                std::cmp::Ordering::Greater => n.right = Some(copy_insert(n.right.take(), value)),
                std::cmp::Ordering::Equal => {}
            }
            copy_rebalance(n)
        }
    }
}
/// An unbalanced binary search tree with correct stored heights.
fn plain_insert(node: Option<Box<AvlNode<u32>>>, value: u32) -> Box<AvlNode<u32>> {
    match node {
        None => AvlNode::new(value),
        Some(mut n) => {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => n.left = Some(plain_insert(n.left.take(), value)),
                std::cmp::Ordering::Greater => n.right = Some(plain_insert(n.right.take(), value)),
                std::cmp::Ordering::Equal => {}
            }
            avl_update_height(&mut n);
            n
        }
    }
}
/// Pre-order (value, height) with an explicit marker for a missing child.
fn shape(node: &Option<Box<AvlNode<u32>>>, out: &mut Vec<Option<(u32, usize)>>) {
    match node {
        None => out.push(None),
        Some(n) => {
            out.push(Some((n.value, n.height)));
            shape(&n.left, out);
            shape(&n.right, out);
        }
    }
}
fn shape_of(node: Box<AvlNode<u32>>) -> Vec<Option<(u32, usize)>> {
    let mut out = Vec::new();
    shape(&Some(node), &mut out);
    out
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

#[test]
fn insertion_builds_the_same_trees_as_the_copy() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for round in 0..400 {
        let len = (round % 40) + 1;
        let values: Vec<u32> = (0..len).map(|_| (rng.next() % 64) as u32).collect();
        let mut new_tree: Option<Box<AvlNode<u32>>> = None;
        let mut copy_tree: Option<Box<AvlNode<u32>>> = None;
        for &v in &values {
            new_tree = Some(avl_insert(new_tree.take(), v));
            copy_tree = Some(copy_insert(copy_tree.take(), v));
            let mut a = Vec::new();
            let mut b = Vec::new();
            shape(&new_tree, &mut a);
            shape(&copy_tree, &mut b);
            assert_eq!(a, b, "after inserting {v} of {values:?}");
        }
    }
    // Sorted runs exercise the single rotations, zig-zags the double ones.
    for values in [
        (0..32).collect::<Vec<u32>>(),
        (0..32).rev().collect(),
        vec![10, 5, 7, 20, 25, 22, 1, 3, 2],
    ] {
        let mut new_tree = None;
        let mut copy_tree = None;
        for &v in &values {
            new_tree = Some(avl_insert(new_tree, v));
            copy_tree = Some(copy_insert(copy_tree, v));
        }
        let mut a = Vec::new();
        let mut b = Vec::new();
        shape(&new_tree, &mut a);
        shape(&copy_tree, &mut b);
        assert_eq!(a, b);
    }
}

#[test]
fn one_rebalance_of_an_unbalanced_tree_matches_the_copy() {
    let mut rng = Rng(0x0123_4567_89ab_cdef);
    let mut rotated = 0usize;
    for round in 0..2000 {
        let len = (round % 12) + 1;
        let values: Vec<u32> = (0..len).map(|_| (rng.next() % 32) as u32).collect();
        let mut tree = None;
        for &v in &values {
            tree = Some(plain_insert(tree, v));
        }
        let mut copy = None;
        for &v in &values {
            copy = Some(plain_insert(copy, v));
        }
        let (Some(tree), Some(copy)) = (tree, copy) else {
            continue;
        };
        if avl_balance_factor(&tree).abs() > 1 {
            rotated += 1;
        }
        assert_eq!(
            shape_of(avl_rebalance(tree)),
            shape_of(copy_rebalance(copy)),
            "{values:?}"
        );
    }
    assert!(rotated > 100);
}
