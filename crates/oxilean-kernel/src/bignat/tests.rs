//! Unit tests for the kernel BigNat.
//!
//! Edge-case coverage: 0, 1, `u64::MAX` boundaries, multi-limb values, and
//! Karatsuba threshold crossings on both sides. Differential tests against
//! `num-bigint` live in `tests/bignat_differential.rs` (dev-dependency only).

use super::*;

fn n(v: u64) -> BigNat {
    BigNat::from(v)
}

fn parse(s: &str) -> BigNat {
    match BigNat::from_decimal_str(s) {
        Some(v) => v,
        None => unreachable!("test literal must parse: {s}"),
    }
}

// ── Construction / conversion ────────────────────────────────────────────────

#[test]
fn test_zero_one_normalization() {
    assert!(BigNat::zero().is_zero());
    assert!(BigNat::one().is_one());
    assert_eq!(BigNat::from(0u64), BigNat::zero());
    assert_eq!(BigNat::from_limbs(vec![0, 0, 0]), BigNat::zero());
    assert_eq!(BigNat::from_limbs(vec![5, 0, 0]), n(5));
    assert_eq!(BigNat::from_limbs(vec![5, 0, 0]).as_limbs(), &[5]);
}

#[test]
fn test_from_u128_roundtrip() {
    let v = BigNat::from(u128::from(u64::MAX) + 1);
    assert_eq!(v.as_limbs(), &[0, 1]);
    assert_eq!(v.to_u64(), None);
    assert_eq!(BigNat::from(42u128), n(42));
}

#[test]
fn test_to_u64_boundaries() {
    assert_eq!(BigNat::zero().to_u64(), Some(0));
    assert_eq!(n(u64::MAX).to_u64(), Some(u64::MAX));
    assert_eq!(n(u64::MAX).succ().to_u64(), None);
    assert_eq!(n(u64::MAX).succ().pred().to_u64(), Some(u64::MAX));
}

#[test]
fn test_to_u32() {
    assert_eq!(n(42).to_u32(), Some(42));
    assert_eq!(n(u64::from(u32::MAX)).to_u32(), Some(u32::MAX));
    assert_eq!(n(u64::from(u32::MAX) + 1).to_u32(), None);
}

#[test]
fn test_bit_length() {
    assert_eq!(BigNat::zero().bit_length(), 0);
    assert_eq!(n(1).bit_length(), 1);
    assert_eq!(n(2).bit_length(), 2);
    assert_eq!(n(u64::MAX).bit_length(), 64);
    assert_eq!(n(u64::MAX).succ().bit_length(), 65);
}

// ── Ordering / equality ──────────────────────────────────────────────────────

#[test]
fn test_ordering() {
    assert!(BigNat::zero() < n(1));
    assert!(n(u64::MAX) < n(u64::MAX).succ());
    assert!(n(u64::MAX).succ() > n(u64::MAX));
    assert_eq!(n(7).cmp(&n(7)), std::cmp::Ordering::Equal);
    let big = parse("340282366920938463463374607431768211456"); // 2^128
    assert!(big > n(u64::MAX));
}

#[test]
fn test_partial_eq_ord_u64() {
    assert!(n(5) == 5u64);
    assert!(n(5) != 6u64);
    assert!(n(u64::MAX).succ() != 0u64);
    assert!(n(u64::MAX).succ() > u64::MAX);
    assert!(n(3) < 4u64);
}

// ── succ / pred ──────────────────────────────────────────────────────────────

#[test]
fn test_succ_pred() {
    assert_eq!(BigNat::zero().succ(), n(1));
    assert_eq!(BigNat::zero().pred(), BigNat::zero()); // Lean: pred 0 = 0
    assert_eq!(n(1).pred(), BigNat::zero());
    // u64::MAX + 1 crosses the limb boundary correctly.
    let big = n(u64::MAX).succ();
    assert_eq!(big.as_limbs(), &[0, 1]);
    assert_eq!(big.pred(), n(u64::MAX));
}

// ── add / sub ────────────────────────────────────────────────────────────────

#[test]
fn test_add_carry_chain() {
    // (2^128 - 1) + 1 = 2^128
    let a = BigNat::from_limbs(vec![u64::MAX, u64::MAX]);
    assert_eq!(a.add(&n(1)).as_limbs(), &[0, 0, 1]);
    assert_eq!(n(0).add(&n(0)), BigNat::zero());
}

#[test]
fn test_add_u64_max_regression() {
    // The old u64 kernel wrapped (release) or panicked (debug) here.
    let r = n(u64::MAX).add(&n(1));
    assert_eq!(r.to_u64(), None);
    assert_eq!(r.to_string(), "18446744073709551616");
}

#[test]
fn test_sub_truncated() {
    assert_eq!(n(5).sub(&n(3)), n(2));
    assert_eq!(n(3).sub(&n(5)), BigNat::zero()); // Lean: a - b = 0 for b >= a
    assert_eq!(n(3).sub(&n(3)), BigNat::zero());
    assert_eq!(BigNat::zero().sub(&n(1)), BigNat::zero());
    // Multi-limb borrow: 2^128 - 1
    let p128 = parse("340282366920938463463374607431768211456");
    assert_eq!(
        p128.sub(&n(1)).to_string(),
        "340282366920938463463374607431768211455"
    );
}

// ── mul ──────────────────────────────────────────────────────────────────────

#[test]
fn test_mul_small() {
    assert_eq!(n(6).mul(&n(7)), n(42));
    assert_eq!(n(0).mul(&n(7)), BigNat::zero());
    assert_eq!(n(7).mul(&n(0)), BigNat::zero());
    assert_eq!(n(1).mul(&n(7)), n(7));
}

#[test]
fn test_mul_overflow_regression() {
    // 2^32 * 2^32 = 2^64: the old u64 kernel silently produced 0 in release
    // builds. Pin the fix forever.
    let x = n(1 << 32);
    let r = x.mul(&x);
    assert_ne!(r, BigNat::zero());
    assert_eq!(r.to_string(), "18446744073709551616");
    assert_eq!(r.as_limbs(), &[0, 1]);
    // (2^64 - 1)^2 = 2^128 - 2^65 + 1
    let m = n(u64::MAX);
    assert_eq!(
        m.mul(&m).to_string(),
        "340282366920938463426481119284349108225"
    );
}

#[test]
fn test_mul_karatsuba_threshold_crossings() {
    // Exercise limb counts just below, at, and above KARATSUBA_THRESHOLD (32)
    // and verify against the schoolbook path directly.
    for &len in &[30usize, 31, 32, 33, 40, 64, 65] {
        let a: Vec<u64> = (0..len).map(|i| u64::MAX - i as u64).collect();
        let b: Vec<u64> = (0..len)
            .map(|i| 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1))
            .collect();
        let expected = BigNat::from_limbs(mul_schoolbook(&a, &b));
        let got = BigNat::from_limbs(a.clone()).mul(&BigNat::from_limbs(b.clone()));
        assert_eq!(got, expected, "limb count {len}");
    }
}

#[test]
fn test_mul_karatsuba_unbalanced() {
    // Very unbalanced operand lengths through the Karatsuba path.
    let a: Vec<u64> = (0..33).map(|i| u64::MAX - i as u64).collect();
    let b: Vec<u64> = (0..200)
        .map(|i| 0xDEAD_BEEF_u64.wrapping_mul(i as u64 + 3))
        .collect();
    let expected = BigNat::from_limbs(mul_schoolbook(&a, &b));
    let got = BigNat::from_limbs(a).mul(&BigNat::from_limbs(b));
    assert_eq!(got, expected);
}

#[test]
fn test_mul_commutative_multi_limb() {
    let a = parse("123456789012345678901234567890123456789");
    let b = parse("987654321098765432109876543210");
    assert_eq!(a.mul(&b), b.mul(&a));
    assert_eq!(
        a.mul(&b).to_string(),
        "121932631137021795226185032733744855963362292333223746380111126352690"
    );
}

// ── div / mod ────────────────────────────────────────────────────────────────

#[test]
fn test_div_mod_by_zero_lean_semantics() {
    // Lean: x / 0 = 0 and x % 0 = x.
    assert_eq!(n(7).div(&BigNat::zero()), BigNat::zero());
    assert_eq!(n(7).rem(&BigNat::zero()), n(7));
    assert_eq!(BigNat::zero().div(&BigNat::zero()), BigNat::zero());
    assert_eq!(BigNat::zero().rem(&BigNat::zero()), BigNat::zero());
    let big = parse("340282366920938463463374607431768211456");
    assert_eq!(big.rem(&BigNat::zero()), big);
}

#[test]
fn test_div_mod_small() {
    assert_eq!(n(10).div(&n(3)), n(3));
    assert_eq!(n(10).rem(&n(3)), n(1));
    assert_eq!(n(10).div(&n(10)), n(1));
    assert_eq!(n(10).rem(&n(10)), BigNat::zero());
    assert_eq!(n(3).div(&n(10)), BigNat::zero());
    assert_eq!(n(3).rem(&n(10)), n(3));
}

#[test]
fn test_div_mod_multi_limb() {
    // 2^192 / (2^64 + 1), verified externally.
    let a = parse("6277101735386680763835789423207666416102355444464034512896");
    let b = parse("18446744073709551617");
    let q = a.div(&b);
    let r = a.rem(&b);
    assert_eq!(q.mul(&b).add(&r), a);
    assert!(r < b);
    assert_eq!(q.to_string(), "340282366920938463444927863358058659840");
    assert_eq!(r.to_string(), "18446744073709551616");
}

#[test]
fn test_div_knuth_add_back_case() {
    // Divisor with small second limb forces qhat over-estimation paths.
    let u = BigNat::from_limbs(vec![0, 0, 0x8000_0000_0000_0000]);
    let v = BigNat::from_limbs(vec![1, 0x8000_0000_0000_0000]);
    let (q, r) = u.div_rem(&v);
    assert_eq!(q.mul(&v).add(&r), u);
    assert!(r < v);
}

#[test]
fn test_div_rem_u64() {
    let a = parse("340282366920938463463374607431768211456"); // 2^128
    let (q, r) = a.div_rem_u64(10);
    assert_eq!(q.to_string(), "34028236692093846346337460743176821145");
    assert_eq!(r, 6);
    assert_eq!(a.div_rem_u64(0), (BigNat::zero(), 0));
}

// ── pow ──────────────────────────────────────────────────────────────────────

#[test]
fn test_pow_lean_edge_cases() {
    // Lean: 0 ^ 0 = 1.
    assert_eq!(BigNat::zero().pow(&BigNat::zero()), Some(BigNat::one()));
    assert_eq!(BigNat::zero().pow(&n(5)), Some(BigNat::zero()));
    assert_eq!(n(1).pow(&n(1_000_000_000)), Some(BigNat::one()));
    assert_eq!(n(7).pow(&BigNat::zero()), Some(BigNat::one()));
    assert_eq!(n(2).pow(&n(10)), Some(n(1024)));
}

#[test]
fn test_pow_beyond_u64() {
    // 2^64 and 2^100 must be exact, not wrapped/saturated.
    let p64 = match n(2).pow(&n(64)) {
        Some(v) => v,
        None => unreachable!("2^64 is within the bound"),
    };
    assert_eq!(p64.to_string(), "18446744073709551616");
    let p100 = match n(2).pow(&n(100)) {
        Some(v) => v,
        None => unreachable!("2^100 is within the bound"),
    };
    assert_eq!(p100.to_string(), "1267650600228229401496703205376");
    // 3^200, cross-checked externally.
    let p = match n(3).pow(&n(200)) {
        Some(v) => v,
        None => unreachable!("3^200 is within the bound"),
    };
    assert_eq!(
        p.to_string(),
        "265613988875874769338781322035779626829233452653394495974574961739092490901302182994384699044001"
    );
}

#[test]
fn test_pow_exponent_not_truncated() {
    // The old kernel truncated the exponent with `as u32`, so
    // 2 ^ 2^32 evaluated to 2^0 = 1. Now the guard refuses (None => stuck),
    // never a wrong value.
    let e = n(1u64 << 32);
    assert_eq!(n(2).pow(&e), None);
    // Base 1 with the same exponent is fine (result is small).
    assert_eq!(n(1).pow(&e), Some(BigNat::one()));
    // Exponent beyond u64 with base >= 2: stuck, not wrong.
    let huge = n(u64::MAX).succ();
    assert_eq!(n(2).pow(&huge), None);
    assert_eq!(BigNat::zero().pow(&huge), Some(BigNat::zero()));
}

#[test]
fn test_pow_memory_guard_boundary() {
    // 2^(MAX_RESULT_BITS) is exactly at the bound (bit_length(2) = 2, so the
    // check is 2*e <= MAX_RESULT_BITS); e = MAX_RESULT_BITS/2 passes,
    // e = MAX_RESULT_BITS + 1 refuses.
    assert!(n(2).pow(&n(MAX_RESULT_BITS / 2)).is_some());
    assert_eq!(n(2).pow(&n(MAX_RESULT_BITS + 1)), None);
}

// ── gcd ──────────────────────────────────────────────────────────────────────

#[test]
fn test_gcd() {
    assert_eq!(n(12).gcd(&n(8)), n(4));
    assert_eq!(n(8).gcd(&n(12)), n(4));
    assert_eq!(BigNat::zero().gcd(&n(5)), n(5));
    assert_eq!(n(5).gcd(&BigNat::zero()), n(5));
    assert_eq!(BigNat::zero().gcd(&BigNat::zero()), BigNat::zero());
    assert_eq!(n(17).gcd(&n(13)), n(1));
    // Multi-limb: gcd(2^128, 2^64) = 2^64.
    let p128 = parse("340282366920938463463374607431768211456");
    let p64 = parse("18446744073709551616");
    assert_eq!(p128.gcd(&p64), p64);
}

// ── beq / ble ────────────────────────────────────────────────────────────────

#[test]
fn test_beq_ble() {
    assert!(n(5).beq(&n(5)));
    assert!(!n(5).beq(&n(6)));
    assert!(n(5).ble(&n(5)));
    assert!(n(5).ble(&n(6)));
    assert!(!n(6).ble(&n(5)));
    let big = n(u64::MAX).succ();
    assert!(!big.beq(&n(0)));
    assert!(n(u64::MAX).ble(&big));
    assert!(!big.ble(&n(u64::MAX)));
}

// ── bitwise ──────────────────────────────────────────────────────────────────

#[test]
fn test_bitwise_small() {
    assert_eq!(n(0b1100).land(&n(0b1010)), n(0b1000));
    assert_eq!(n(0b1100).lor(&n(0b1010)), n(0b1110));
    assert_eq!(n(0b1100).lxor(&n(0b1010)), n(0b0110));
}

#[test]
fn test_bitwise_mixed_widths() {
    let big = n(u64::MAX).succ(); // 2^64 = limbs [0, 1]
    assert_eq!(big.land(&n(u64::MAX)), BigNat::zero());
    assert_eq!(big.lor(&n(1)).as_limbs(), &[1, 1]);
    assert_eq!(big.lxor(&big), BigNat::zero()); // normalization after xor
    assert_eq!(big.lxor(&n(1)).as_limbs(), &[1, 1]);
}

// ── shifts ───────────────────────────────────────────────────────────────────

#[test]
fn test_shl_never_drops_bits() {
    // The old kernel computed (2^63) << 1 = 0 in release builds.
    let r = match n(1u64 << 63).checked_shl(&n(1)) {
        Some(v) => v,
        None => unreachable!("small shift is within the bound"),
    };
    assert_eq!(r.to_string(), "18446744073709551616");
    // Shift by 64 and 65 across the limb boundary.
    let r64 = match n(1).checked_shl(&n(64)) {
        Some(v) => v,
        None => unreachable!("shift by 64 is within the bound"),
    };
    assert_eq!(r64.as_limbs(), &[0, 1]);
    let r65 = match n(3).checked_shl(&n(65)) {
        Some(v) => v,
        None => unreachable!("shift by 65 is within the bound"),
    };
    assert_eq!(r65.as_limbs(), &[0, 6]);
    assert_eq!(n(5).checked_shl(&BigNat::zero()), Some(n(5)));
}

#[test]
fn test_shl_guard() {
    // 0 <<< anything = 0, even for absurd shift amounts.
    let huge = n(u64::MAX).succ();
    assert_eq!(BigNat::zero().checked_shl(&huge), Some(BigNat::zero()));
    // Non-zero shifted beyond the bound: stuck, not wrong.
    assert_eq!(n(1).checked_shl(&n(MAX_RESULT_BITS + 1)), None);
    assert_eq!(n(1).checked_shl(&huge), None);
    // Exactly at the bound is allowed (1 bit + (MAX-1) shift = MAX bits).
    assert!(n(1).checked_shl(&n(MAX_RESULT_BITS - 1)).is_some());
}

#[test]
fn test_shr() {
    assert_eq!(n(0b1100).shr(&n(2)), n(0b11));
    assert_eq!(n(1).shr(&n(1)), BigNat::zero());
    assert_eq!(n(u64::MAX).shr(&n(64)), BigNat::zero());
    assert_eq!(n(u64::MAX).shr(&n(63)), n(1));
    // Multi-limb: 2^128 >> 64 = 2^64; >> beyond u64 = 0.
    let p128 = parse("340282366920938463463374607431768211456");
    assert_eq!(p128.shr(&n(64)).as_limbs(), &[0, 1]);
    assert_eq!(p128.shr(&n(u64::MAX)), BigNat::zero());
    assert_eq!(p128.shr(&n(u64::MAX).succ()), BigNat::zero());
    assert_eq!(p128.shr(&BigNat::zero()), p128);
}

// ── log2 ─────────────────────────────────────────────────────────────────────

#[test]
fn test_log2_lean_semantics() {
    assert_eq!(BigNat::zero().log2(), BigNat::zero()); // Lean: log2 0 = 0
    assert_eq!(n(1).log2(), BigNat::zero());
    assert_eq!(n(2).log2(), n(1));
    assert_eq!(n(3).log2(), n(1));
    assert_eq!(n(4).log2(), n(2));
    assert_eq!(n(u64::MAX).log2(), n(63));
    assert_eq!(n(u64::MAX).succ().log2(), n(64));
}

// ── decimal parse / display ──────────────────────────────────────────────────

#[test]
fn test_decimal_parse_basics() {
    assert_eq!(BigNat::from_decimal_str("0"), Some(BigNat::zero()));
    assert_eq!(BigNat::from_decimal_str("00007"), Some(n(7)));
    assert_eq!(
        BigNat::from_decimal_str("18446744073709551615"),
        Some(n(u64::MAX))
    );
    assert_eq!(
        BigNat::from_decimal_str("18446744073709551616"),
        Some(n(u64::MAX).succ())
    );
    assert_eq!(BigNat::from_decimal_str(""), None);
    assert_eq!(BigNat::from_decimal_str("12a3"), None);
    assert_eq!(BigNat::from_decimal_str("-1"), None);
    assert_eq!(BigNat::from_decimal_str("+1"), None);
    assert_eq!(BigNat::from_decimal_str("1 2"), None);
}

#[test]
fn test_decimal_display_roundtrip() {
    let cases = [
        "0",
        "1",
        "9",
        "10",
        "18446744073709551615",
        "18446744073709551616",
        "10000000000000000000000000000000000000000",
        "340282366920938463463374607431768211455",
        "115792089237316195423570985008687907853269984665640564039457584007913129639936",
    ];
    for s in cases {
        assert_eq!(parse(s).to_string(), s, "roundtrip of {s}");
    }
}

#[test]
fn test_from_str_trait() {
    let v: Result<BigNat, ParseBigNatError> = "12345".parse();
    assert_eq!(v, Ok(n(12345)));
    let bad: Result<BigNat, ParseBigNatError> = "x".parse();
    assert_eq!(bad, Err(ParseBigNatError));
}

#[test]
fn test_display_zero_padding_between_chunks() {
    // 10^19 (one full chunk of zeros after the leading "1").
    let v = parse("10000000000000000000");
    assert_eq!(v.to_string(), "10000000000000000000");
    // 2^64 * 10^19 + 5 exercises interior zero-padded chunks.
    let v2 = n(u64::MAX)
        .succ()
        .mul(&parse("10000000000000000000"))
        .add(&n(5));
    assert_eq!(v2.to_string(), "184467440737095516160000000000000000005");
}

#[test]
fn test_debug_format() {
    assert_eq!(format!("{:?}", n(42)), "BigNat(42)");
}

// ── hash coherence ───────────────────────────────────────────────────────────

#[test]
fn test_hash_eq_coherence() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(parse("18446744073709551616"));
    assert!(set.contains(&n(u64::MAX).succ()));
    assert!(!set.contains(&n(u64::MAX)));
}
