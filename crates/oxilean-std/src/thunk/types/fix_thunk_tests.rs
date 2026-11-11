//! `FixThunk::compute`: the step function reads earlier results through the
//! lookup it is given while the memo table is borrowed, and every result is
//! cached.

use super::FixThunk;
use std::cell::Cell;

#[test]
fn computes_in_order_reading_earlier_results_from_the_memo() {
    let calls = std::rc::Rc::new(Cell::new(0usize));
    let counter = calls.clone();
    let mut fib = FixThunk::new(move |n, lookup: &dyn Fn(usize) -> u64| {
        counter.set(counter.get() + 1);
        if n < 2 {
            n as u64
        } else {
            lookup(n - 1) + lookup(n - 2)
        }
    });
    let values: Vec<u64> = (0..30).map(|n| fib.compute(n)).collect();
    assert_eq!(values[10], 55);
    assert_eq!(values[29], 514_229);
    assert_eq!(calls.get(), 30);
    // Cached: no further call of the step function.
    assert_eq!(fib.compute(29), 514_229);
    assert_eq!(fib.compute(3), 2);
    assert_eq!(calls.get(), 30);
}

#[test]
fn a_step_may_read_many_entries_of_the_memo() {
    let mut sums = FixThunk::new(|n, lookup: &dyn Fn(usize) -> String| {
        let earlier: Vec<String> = (0..n).map(lookup).collect();
        format!("{}[{}]", n, earlier.join(","))
    });
    for n in 0..5 {
        sums.compute(n);
    }
    assert_eq!(sums.compute(2), "2[0[],1[0[]]]");
    assert_eq!(sums.compute(4).matches('[').count(), 16);
}
