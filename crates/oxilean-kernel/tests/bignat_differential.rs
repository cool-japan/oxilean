//! Differential tests: kernel `BigNat` vs `num-bigint` (dev-dependency only).
//!
//! Every arithmetic operation of the hand-written TCB bignum is compared
//! against the community reference implementation on directed edge cases and
//! on proptest-generated random values (single-limb, boundary, and
//! multi-limb, crossing the Karatsuba threshold from both sides).
//!
//! Lean-specific semantics (`x / 0 = 0`, `x % 0 = x`, truncated `sub`) are
//! mapped explicitly where the reference differs.

use num_bigint::BigUint;
use oxilean_kernel::bignat::BigNat;
use proptest::collection::vec as prop_vec;
use proptest::prelude::*;

// ── helpers ──────────────────────────────────────────────────────────────────

/// Convert kernel BigNat -> reference BigUint via little-endian u64 limbs.
fn to_ref(n: &BigNat) -> BigUint {
    let mut out = BigUint::default();
    for (i, limb) in n.as_limbs().iter().enumerate() {
        out += BigUint::from(*limb) << (64 * i);
    }
    out
}

/// Convert little-endian u64 limbs to both implementations.
fn both(limbs: &[u64]) -> (BigNat, BigUint) {
    let ours = BigNat::from_limbs(limbs.to_vec());
    let theirs = to_ref(&ours);
    (ours, theirs)
}

/// Directed edge-case values: 0, 1, 2, u64 boundaries, and multi-limb values
/// below/at/above the Karatsuba threshold (32 limbs).
fn edge_cases() -> Vec<Vec<u64>> {
    let mut cases: Vec<Vec<u64>> = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![u64::MAX - 1],
        vec![u64::MAX],
        vec![0, 1],               // 2^64
        vec![1, 1],               // 2^64 + 1
        vec![u64::MAX, u64::MAX], // 2^128 - 1
        vec![0, 0, 1],            // 2^128
        vec![5, 0, 0, 7],         // sparse multi-limb
    ];
    // Karatsuba threshold crossings: 31, 32, 33, 64 limbs.
    for len in [31usize, 32, 33, 64] {
        cases.push((0..len).map(|i| u64::MAX - i as u64).collect());
        cases.push(
            (0..len)
                .map(|i| 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1) | 1)
                .collect(),
        );
    }
    cases
}

// ── directed differential tests ──────────────────────────────────────────────

#[test]
fn diff_add_sub_mul_edge_cases() {
    let cases = edge_cases();
    for a_limbs in &cases {
        for b_limbs in &cases {
            let (a, ra) = both(a_limbs);
            let (b, rb) = both(b_limbs);
            assert_eq!(to_ref(&a.add(&b)), &ra + &rb, "add {a} {b}");
            assert_eq!(to_ref(&a.mul(&b)), &ra * &rb, "mul {a} {b}");
            // Lean sub is truncated.
            let expected_sub = if ra >= rb {
                &ra - &rb
            } else {
                BigUint::default()
            };
            assert_eq!(to_ref(&a.sub(&b)), expected_sub, "sub {a} {b}");
        }
    }
}

#[test]
fn diff_div_mod_edge_cases() {
    let cases = edge_cases();
    for a_limbs in &cases {
        for b_limbs in &cases {
            let (a, ra) = both(a_limbs);
            let (b, rb) = both(b_limbs);
            if b.is_zero() {
                // Lean: x / 0 = 0, x % 0 = x.
                assert!(a.div(&b).is_zero(), "div-by-zero {a}");
                assert_eq!(a.rem(&b), a, "mod-by-zero {a}");
            } else {
                assert_eq!(to_ref(&a.div(&b)), &ra / &rb, "div {a} {b}");
                assert_eq!(to_ref(&a.rem(&b)), &ra % &rb, "mod {a} {b}");
            }
        }
    }
}

#[test]
fn diff_bitwise_and_shifts_edge_cases() {
    let cases = edge_cases();
    for a_limbs in &cases {
        for b_limbs in &cases {
            let (a, ra) = both(a_limbs);
            let (b, rb) = both(b_limbs);
            assert_eq!(to_ref(&a.land(&b)), &ra & &rb, "land {a} {b}");
            assert_eq!(to_ref(&a.lor(&b)), &ra | &rb, "lor {a} {b}");
            assert_eq!(to_ref(&a.lxor(&b)), &ra ^ &rb, "lxor {a} {b}");
        }
        let (a, ra) = both(a_limbs);
        for sh in [0u64, 1, 63, 64, 65, 127, 128, 1000] {
            let shifted = a.checked_shl(&BigNat::from(sh));
            match shifted {
                Some(v) => assert_eq!(to_ref(&v), &ra << sh, "shl {a} {sh}"),
                None => unreachable!("shift within bound must succeed: {a} <<< {sh}"),
            }
            assert_eq!(to_ref(&a.shr(&BigNat::from(sh))), &ra >> sh, "shr {a} {sh}");
        }
    }
}

#[test]
fn diff_gcd_edge_cases() {
    let cases = edge_cases();
    for a_limbs in &cases {
        for b_limbs in &cases {
            let (a, ra) = both(a_limbs);
            let (b, rb) = both(b_limbs);
            let g = a.gcd(&b);
            assert_eq!(to_ref(&g), gcd_ref(&ra, &rb), "gcd {a} {b}");
        }
    }
}

fn gcd_ref(a: &BigUint, b: &BigUint) -> BigUint {
    let (mut a, mut b) = (a.clone(), b.clone());
    let zero = BigUint::default();
    while b != zero {
        let r = &a % &b;
        a = b;
        b = r;
    }
    a
}

#[test]
fn diff_pow_edge_cases() {
    // pow within the memory guard, cross-checked against the reference.
    let bases: [u64; 6] = [0, 1, 2, 3, 10, u64::MAX];
    let exps: [u32; 7] = [0, 1, 2, 3, 10, 64, 100];
    for &base in &bases {
        for &exp in &exps {
            let ours = BigNat::from(base).pow(&BigNat::from(u64::from(exp)));
            let theirs = BigUint::from(base).pow(exp);
            match ours {
                Some(v) => assert_eq!(to_ref(&v), theirs, "pow {base} {exp}"),
                None => unreachable!("pow within bound must succeed: {base}^{exp}"),
            }
        }
    }
    // Multi-limb base.
    let base = BigNat::from(u64::MAX).succ(); // 2^64
    let ours = base.pow(&BigNat::from(10u64));
    let theirs = (BigUint::from(1u32) << 64u32).pow(10);
    match ours {
        Some(v) => assert_eq!(to_ref(&v), theirs),
        None => unreachable!("(2^64)^10 is within the bound"),
    }
}

#[test]
fn diff_decimal_display_edge_cases() {
    for limbs in edge_cases() {
        let (a, ra) = both(&limbs);
        assert_eq!(a.to_string(), ra.to_string(), "display {:?}", limbs);
        // Round-trip through decimal parse.
        assert_eq!(BigNat::from_decimal_str(&a.to_string()), Some(a));
    }
}

// ── property-based differential tests ────────────────────────────────────────

/// Random limb vectors spanning 0..=40 limbs (crosses the Karatsuba
/// threshold at 32 from both sides).
fn arb_limbs() -> impl Strategy<Value = Vec<u64>> {
    prop_vec(any::<u64>(), 0..=40)
}

/// Small-ish limb vectors for the quadratic-cost ops (div, gcd).
fn arb_limbs_small() -> impl Strategy<Value = Vec<u64>> {
    prop_vec(any::<u64>(), 0..=12)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn prop_diff_add(a in arb_limbs(), b in arb_limbs()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        prop_assert_eq!(to_ref(&x.add(&y)), rx + ry);
    }

    #[test]
    fn prop_diff_sub_truncated(a in arb_limbs(), b in arb_limbs()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        let expected = if rx >= ry { rx - ry } else { BigUint::default() };
        prop_assert_eq!(to_ref(&x.sub(&y)), expected);
    }

    #[test]
    fn prop_diff_mul(a in arb_limbs(), b in arb_limbs()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        prop_assert_eq!(to_ref(&x.mul(&y)), rx * ry);
    }

    #[test]
    fn prop_diff_div_mod(a in arb_limbs_small(), b in arb_limbs_small()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        if y.is_zero() {
            prop_assert!(x.div(&y).is_zero());
            prop_assert_eq!(x.rem(&y), x);
        } else {
            prop_assert_eq!(to_ref(&x.div(&y)), &rx / &ry);
            prop_assert_eq!(to_ref(&x.rem(&y)), &rx % &ry);
            // Euclidean identity.
            let (q, r) = x.div_rem(&y);
            prop_assert_eq!(q.mul(&y).add(&r), x.clone());
            prop_assert!(r < y);
        }
    }

    #[test]
    fn prop_diff_div_mod_near_limb_boundaries(
        a in arb_limbs_small(),
        top in prop_oneof![Just(1u64), Just(2), Just(u64::MAX), Just(u64::MAX - 1), Just(1u64 << 63)],
        lows in prop_vec(prop_oneof![Just(0u64), Just(1), Just(u64::MAX)], 0..=3),
    ) {
        // Divisors whose top limb sits at normalization boundaries exercise
        // the Knuth-D qhat estimation and add-back paths.
        let mut divisor = lows.clone();
        divisor.push(top);
        let (x, rx) = both(&a);
        let (y, ry) = both(&divisor);
        prop_assume!(!y.is_zero());
        prop_assert_eq!(to_ref(&x.div(&y)), &rx / &ry);
        prop_assert_eq!(to_ref(&x.rem(&y)), &rx % &ry);
    }

    #[test]
    fn prop_diff_bitwise(a in arb_limbs(), b in arb_limbs()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        prop_assert_eq!(to_ref(&x.land(&y)), &rx & &ry);
        prop_assert_eq!(to_ref(&x.lor(&y)), &rx | &ry);
        prop_assert_eq!(to_ref(&x.lxor(&y)), &rx ^ &ry);
    }

    #[test]
    fn prop_diff_shifts(a in arb_limbs(), sh in 0u64..=300) {
        let (x, rx) = both(&a);
        match x.checked_shl(&BigNat::from(sh)) {
            Some(v) => prop_assert_eq!(to_ref(&v), &rx << sh),
            None => prop_assert!(false, "shift within bound must succeed"),
        }
        prop_assert_eq!(to_ref(&x.shr(&BigNat::from(sh))), &rx >> sh);
    }

    #[test]
    fn prop_diff_gcd(a in arb_limbs_small(), b in arb_limbs_small()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        prop_assert_eq!(to_ref(&x.gcd(&y)), gcd_ref(&rx, &ry));
    }

    #[test]
    fn prop_diff_pow_small(base in 0u64..=1000, exp in 0u32..=40) {
        let ours = BigNat::from(base).pow(&BigNat::from(u64::from(exp)));
        let theirs = BigUint::from(base).pow(exp);
        match ours {
            Some(v) => prop_assert_eq!(to_ref(&v), theirs),
            None => prop_assert!(false, "pow within bound must succeed"),
        }
    }

    #[test]
    fn prop_diff_ordering(a in arb_limbs(), b in arb_limbs()) {
        let (x, rx) = both(&a);
        let (y, ry) = both(&b);
        prop_assert_eq!(x.cmp(&y), rx.cmp(&ry));
        prop_assert_eq!(x.beq(&y), rx == ry);
        prop_assert_eq!(x.ble(&y), rx <= ry);
    }

    #[test]
    fn prop_diff_log2(a in arb_limbs()) {
        let (x, rx) = both(&a);
        // Lean: log2 0 = 0; otherwise floor(log2 n) = bits - 1.
        let expected = if x.is_zero() { 0 } else { rx.bits() - 1 };
        prop_assert_eq!(x.log2(), BigNat::from(expected));
    }

    #[test]
    fn prop_diff_decimal_roundtrip(a in arb_limbs()) {
        let (x, rx) = both(&a);
        prop_assert_eq!(x.to_string(), rx.to_string());
        prop_assert_eq!(BigNat::from_decimal_str(&x.to_string()), Some(x));
    }

    #[test]
    fn prop_diff_parse_decimal_strings(s in "[0-9]{1,80}") {
        let ours = BigNat::from_decimal_str(&s);
        let theirs = s.parse::<BigUint>().ok();
        match (ours, theirs) {
            (Some(v), Some(r)) => prop_assert_eq!(to_ref(&v), r),
            _ => prop_assert!(false, "digit strings must parse in both implementations"),
        }
    }

    #[test]
    fn prop_diff_succ_pred(a in arb_limbs()) {
        let (x, rx) = both(&a);
        prop_assert_eq!(to_ref(&x.succ()), &rx + 1u32);
        let expected_pred = if x.is_zero() { BigUint::default() } else { &rx - 1u32 };
        prop_assert_eq!(to_ref(&x.pred()), expected_pred);
    }
}
