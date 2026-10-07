//! `ScalarMult::windowed` against a verbatim copy of the previous code, which
//! read the last table entry back with `expect`.

use super::{EllipticCurvePoint, ScalarMult};

fn old_windowed(m: &ScalarMult, a: f64) -> EllipticCurvePoint {
    let w: u64 = 4;
    let window = 1u64 << w;
    let base = EllipticCurvePoint::Affine(m.point.0, m.point.1);
    let mut table = vec![EllipticCurvePoint::Infinity];
    for _ in 1..window {
        let last = table.last().expect("starts with Infinity").clone();
        table.push(last.add_points(&base, a));
    }
    let mut result = EllipticCurvePoint::Infinity;
    let bits = 64u32;
    let mut i = bits;
    while i > 0 {
        i = i.saturating_sub(w as u32);
        for _ in 0..w {
            result = result.double_point(a);
        }
        let digit = ((m.k >> i) & (window - 1)) as usize;
        result = result.add_points(&table[digit], a);
    }
    result
}

fn same(p: &EllipticCurvePoint, q: &EllipticCurvePoint) -> bool {
    match (p, q) {
        (EllipticCurvePoint::Infinity, EllipticCurvePoint::Infinity) => true,
        (EllipticCurvePoint::Affine(x1, y1), EllipticCurvePoint::Affine(x2, y2)) => {
            x1.to_bits() == x2.to_bits() && y1.to_bits() == y2.to_bits()
        }
        _ => false,
    }
}

#[test]
fn windowed_multiplication_is_bit_identical() {
    for k in [0u64, 1, 2, 3, 7, 15, 16, 255, 1 << 40, u64::MAX] {
        for (point, a) in [((0.0, 1.0), -1.0), ((2.0, 3.0), 0.0), ((-1.0, 0.0), 1.0)] {
            let m = ScalarMult::new(point, k);
            assert!(same(&m.windowed(a), &old_windowed(&m, a)), "k {k}");
        }
    }
}
