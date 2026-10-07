//! `Zipper::new`, `extend` and `move_left` against verbatim copies of the
//! previous code, which rebuilt the zipper at every position with `expect`.

use super::Zipper;
use std::cell::RefCell;

fn old_new(data: Vec<u32>, i: usize) -> Option<Zipper<u32>> {
    if data.is_empty() || i >= data.len() {
        return None;
    }
    let mut v = data;
    let right = v.split_off(i + 1);
    let focus = v.pop().expect("v is data[..=i]");
    Some(Zipper {
        left: v,
        focus,
        right,
    })
}
fn old_extend<B: Clone>(z: &Zipper<u32>, f: impl Fn(&Zipper<u32>) -> B) -> Zipper<B> {
    let all: Vec<u32> = z
        .left
        .iter()
        .cloned()
        .chain(std::iter::once(z.focus))
        .chain(z.right.iter().cloned())
        .collect();
    let focus_idx = z.left.len();
    let results: Vec<B> = (0..all.len())
        .map(|i| {
            let z = Zipper::new(all.clone(), i).expect("i < all.len()");
            f(&z)
        })
        .collect();
    Zipper::new(results, focus_idx).expect("focus_idx < results.len()")
}
fn old_move_left(z: &Zipper<u32>) -> Option<Zipper<u32>> {
    if z.left.is_empty() {
        return None;
    }
    let mut new_left = z.left.clone();
    let new_focus = new_left.pop().expect("left is non-empty");
    let mut new_right = vec![z.focus];
    new_right.extend(z.right.iter().cloned());
    Some(Zipper {
        left: new_left,
        focus: new_focus,
        right: new_right,
    })
}
fn parts<A: Clone>(z: &Zipper<A>) -> (Vec<A>, A, Vec<A>) {
    (z.left.clone(), z.focus.clone(), z.right.clone())
}

#[test]
fn new_equals_the_previous_constructor() {
    for len in 0..7u32 {
        let data: Vec<u32> = (0..len).map(|x| 10 + x).collect();
        for i in 0..(len as usize + 2) {
            let new = Zipper::new(data.clone(), i).map(|z| parts(&z));
            let old = old_new(data.clone(), i).map(|z| parts(&z));
            assert_eq!(new, old, "len {len}, i {i}");
        }
    }
}

#[test]
fn extend_visits_the_same_zippers_in_the_same_order() {
    for len in 1..7u32 {
        let data: Vec<u32> = (0..len).map(|x| 3 * x + 1).collect();
        for i in 0..len as usize {
            let z = Zipper::new(data.clone(), i).expect("i < len");
            // `f` records every zipper it is given, so the order of the calls
            // and their arguments are compared, not only the results.
            let new_log = RefCell::new(Vec::new());
            let old_log = RefCell::new(Vec::new());
            let new = z.extend(|w| {
                new_log.borrow_mut().push(parts(w));
                w.left.len() as u64 * 1000 + u64::from(w.focus) + w.right.iter().sum::<u32>() as u64
            });
            let old = old_extend(&z, |w| {
                old_log.borrow_mut().push(parts(w));
                w.left.len() as u64 * 1000 + u64::from(w.focus) + w.right.iter().sum::<u32>() as u64
            });
            assert_eq!(parts(&new), parts(&old), "len {len}, focus {i}");
            assert_eq!(new_log.into_inner(), old_log.into_inner());
        }
    }
}

#[test]
fn move_left_equals_the_previous_one() {
    for len in 1..7u32 {
        let data: Vec<u32> = (0..len).collect();
        for i in 0..len as usize {
            let z = Zipper::new(data.clone(), i).expect("i < len");
            assert_eq!(
                z.move_left().map(|w| parts(&w)),
                old_move_left(&z).map(|w| parts(&w))
            );
        }
    }
}
