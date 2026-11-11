//! `HeapPred::sep_star` against a verbatim copy of the previous code, which
//! read every address of `Heap::domain` back with `expect`.

use super::{Heap, HeapPred};

fn old_sep_star(p: HeapPred, q: HeapPred) -> HeapPred {
    HeapPred::new(move |h| {
        let domain: Vec<u64> = h.domain().into_iter().collect();
        let n = domain.len();
        for mask in 0u64..(1u64 << n) {
            let mut h1 = Heap::empty();
            let mut h2 = Heap::empty();
            for (i, &addr) in domain.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    h1.write(addr, h.read(addr).expect("addr is in the domain"));
                } else {
                    h2.write(addr, h.read(addr).expect("addr is in the domain"));
                }
            }
            if p.satisfies(&h1) && q.satisfies(&h2) {
                return true;
            }
        }
        false
    })
}

/// A family of heap predicates; `k` selects one.
fn pred(k: u64) -> HeapPred {
    match k % 5 {
        0 => HeapPred::new(|h| h.size() == 0),
        1 => HeapPred::new(move |h| h.size() == (k % 3) as usize),
        2 => HeapPred::new(move |h| h.read(k % 4) == Some(k % 3)),
        3 => HeapPred::new(|h| h.domain().iter().all(|a| a % 2 == 0)),
        _ => HeapPred::new(|h| {
            h.domain()
                .iter()
                .map(|&a| h.read(a).unwrap_or(0))
                .sum::<u64>()
                >= 2
        }),
    }
}

#[test]
fn separating_conjunction_answers_as_before() {
    let mut holds = 0usize;
    let mut fails = 0usize;
    // Every heap over addresses 0..4 with values 0..3 in each cell.
    for cells in 0u64..(4u64.pow(4)) {
        let mut h = Heap::empty();
        for addr in 0..4u64 {
            let digit = (cells / 4u64.pow(addr as u32)) % 4;
            if digit > 0 {
                h.write(addr, digit - 1);
            }
        }
        for kp in 0..10u64 {
            for kq in [0u64, 3, 6, 7, 9] {
                let new = HeapPred::sep_star(pred(kp), pred(kq)).satisfies(&h);
                let old = old_sep_star(pred(kp), pred(kq)).satisfies(&h);
                assert_eq!(new, old, "cells {cells}, p {kp}, q {kq}");
                if new {
                    holds += 1;
                } else {
                    fails += 1;
                }
            }
        }
    }
    assert!(holds > 0 && fails > 0);
}
