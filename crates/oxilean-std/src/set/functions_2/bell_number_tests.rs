//! `bell_number` against a verbatim copy of the previous Bell triangle, which
//! read the last entry of each row with `expect`.

use super::bell_number;

fn old_bell_number(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut row = vec![1u64];
    for _ in 1..=n {
        let mut new_row = vec![*row.last().expect("starts with [1]")];
        for j in 0..row.len() {
            new_row.push(new_row[j] + row[j]);
        }
        row = new_row;
    }
    row[0]
}

#[test]
fn bell_number_is_unchanged() {
    // Row n of the triangle is bounded by B(n + 1), and B(25) < u64::MAX.
    for n in 0..=24 {
        assert_eq!(bell_number(n), old_bell_number(n), "{n}");
    }
    assert_eq!(bell_number(6), 203);
}
