//! `bell_numbers` against a verbatim copy of the previous Bell triangle,
//! which read the last entry of each row with `expect`.

use super::bell_numbers;

fn old_bell_numbers(max_n: usize) -> Vec<u128> {
    if max_n == 0 {
        return vec![1];
    }
    let mut row: Vec<u128> = vec![1];
    let mut bells = vec![1u128];
    for _ in 1..=max_n {
        let mut next = vec![0u128; row.len() + 1];
        next[0] = *row.last().expect("starts with [1]");
        for j in 1..=row.len() {
            next[j] = next[j - 1] + row[j - 1];
        }
        bells.push(next[0]);
        row = next;
    }
    bells
}

#[test]
fn bell_numbers_are_unchanged() {
    // Row n of the triangle is bounded by B(n + 1), far below u128::MAX here.
    for max_n in 0..=30 {
        assert_eq!(bell_numbers(max_n), old_bell_numbers(max_n), "{max_n}");
    }
    assert_eq!(bell_numbers(5), vec![1, 1, 2, 5, 15, 52]);
}
