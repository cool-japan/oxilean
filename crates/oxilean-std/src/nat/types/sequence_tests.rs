//! `FibonacciUtil::zeckendorf` and `CollatzUtil::stopping_time` against
//! verbatim copies of the previous code, which read the last entry of a
//! vector with `expect`.

use super::{CollatzUtil, FibonacciUtil};

fn old_zeckendorf(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut all_fibs = vec![1u64, 1];
    while *all_fibs.last().expect("starts with [1, 1]") < n {
        let len = all_fibs.len();
        all_fibs.push(all_fibs[len - 1].saturating_add(all_fibs[len - 2]));
    }
    all_fibs.retain(|&x| x <= n);
    all_fibs.dedup();
    let mut result = Vec::new();
    for &fib in all_fibs.iter().rev() {
        if fib <= n {
            result.push(fib);
            n -= fib;
        }
    }
    result
}
fn old_stopping_time(n: u64) -> Option<usize> {
    let seq = CollatzUtil::sequence(n);
    if *seq.last().expect("sequence(n) starts with n") == 1 {
        Some(seq.len() - 1)
    } else {
        None
    }
}

#[test]
fn zeckendorf_representations_are_unchanged() {
    // Up to u64::MAX, where the generated Fibonacci numbers saturate.
    for n in (0..3000u64).chain([u64::MAX / 3, u64::MAX - 1, u64::MAX]) {
        assert_eq!(FibonacciUtil::zeckendorf(n), old_zeckendorf(n), "{n}");
    }
}

#[test]
fn stopping_times_are_unchanged() {
    for n in 0..3000u64 {
        assert_eq!(CollatzUtil::stopping_time(n), old_stopping_time(n), "{n}");
    }
}
