//! `abstract_div` against a verbatim copy of its earlier body.

use super::*;

/// The division as the earlier body computed it.
fn earlier_abstract_div(a: &Interval, b: &Interval) -> Interval {
    if a.is_bottom() || b.is_bottom() {
        return Interval::bottom();
    }
    if b.contains(0) {
        return Interval::top();
    }
    let combos = [a.lo / b.lo, a.lo / b.hi, a.hi / b.lo, a.hi / b.hi];
    let lo = *combos
        .iter()
        .min()
        .expect("combos iterator must be non-empty");
    let hi = *combos
        .iter()
        .max()
        .expect("combos iterator must be non-empty");
    Interval::new(lo, hi)
}

#[test]
fn agrees_with_the_earlier_body_on_every_pair_of_bounds() {
    let bounds = [i64::MIN + 1, -9, -2, -1, 0, 1, 3, 7, i64::MAX];
    let mut dividends = vec![Interval::bottom()];
    for &lo in &bounds {
        for &hi in &bounds {
            if lo <= hi {
                dividends.push(Interval::new(lo, hi));
            }
        }
    }
    // `top` reaches `i64::MIN`, whose quotient by -1 overflows in either body,
    // so it is tried as a divisor only (it contains 0).
    let mut divisors = dividends.clone();
    divisors.push(Interval::top());
    for a in &dividends {
        for b in &divisors {
            assert_eq!(
                abstract_div(a, b),
                earlier_abstract_div(a, b),
                "{a:?} / {b:?}"
            );
        }
    }
}
