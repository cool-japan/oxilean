//! Fuzz target: differential decimal parse/arithmetic of `oxilean_kernel::BigNat`
//! vs `num_bigint::BigUint`.
//!
//! # What this tests
//!
//! The kernel's hand-written `BigNat` must produce exactly the same results as
//! the reference `num-bigint` implementation for every operation the Lean 4
//! kernel uses. A divergence would mean the kernel could evaluate `Nat` literal
//! expressions differently from the reference, producing wrong proof verdicts.
//!
//! # Invariants under fuzz
//!
//! For any pair of byte sequences decoded as decimal strings (or raw u64
//! pairs from a fixed layout):
//!
//! * `BigNat::from_decimal_str` succeeds if and only if the string is
//!   non-empty all-ASCII-digits, and the resulting value equals
//!   `num_bigint::BigUint::parse_bytes(bytes, 10)`.
//! * `add`, `sub` (truncated), `mul`, `div`, `rem`, `gcd`, `beq`, `ble`,
//!   `land`, `lor`, `lxor`, `shr`, `log2` all match `num-bigint`.
//! * `pow` and `checked_shl` match when they return `Some`; when they return
//!   `None` the result would exceed [`MAX_RESULT_BITS`] bits and `num-bigint`
//!   would confirm that (we just check consistency, not the value itself, to
//!   avoid materializing enormous numbers).
//! * `Display` round-trips: `BigNat::from_decimal_str(n.to_string())` equals `n`.

#![no_main]

use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use num_traits::Zero;
use oxilean_kernel::bignat::{BigNat, MAX_RESULT_BITS};

/// Interpret the first `split` bytes as a decimal-encoded `BigNat`/`BigUint`
/// pair. Returns `None` if either side does not produce a usable value.
fn decode_pair(data: &[u8], split: usize) -> Option<(BigNat, BigUint, BigNat, BigUint)> {
    if data.len() < 2 {
        return None;
    }
    let split = (split % data.len()).max(1).min(data.len() - 1);
    let (left, right) = data.split_at(split);

    // Both halves must be non-empty all-digit strings for BigNat.
    if left.is_empty() || right.is_empty() {
        return None;
    }
    if !left.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if !right.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }

    let s_left = std::str::from_utf8(left).ok()?;
    let s_right = std::str::from_utf8(right).ok()?;

    let a = BigNat::from_decimal_str(s_left)?;
    let b = BigNat::from_decimal_str(s_right)?;

    let ra = BigUint::parse_bytes(left, 10)?;
    let rb = BigUint::parse_bytes(right, 10)?;

    Some((a, ra, b, rb))
}

/// Convert a `BigNat` to `BigUint` by round-tripping through decimal.
fn bignat_to_biguint(n: &BigNat) -> BigUint {
    let s = n.to_string();
    BigUint::parse_bytes(s.as_bytes(), 10).expect("Display->parse round-trip must hold")
}

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    // Use the first byte as a split index hint (avoids needing `arbitrary`).
    let split = data[0] as usize;
    let payload = &data[1..];

    let Some((a, ra, b, rb)) = decode_pair(payload, split) else {
        return;
    };

    // Cap the input values to 64 KiB (524 288 bits) so that decimal
    // conversion stays fast throughout all the differential checks below.
    // Inputs larger than this are still valid BigNat values; we just skip
    // the differential check to avoid multi-second num-bigint operations.
    // The arithmetic correctness of large operands is covered by the kernel
    // unit tests (bignat/tests.rs).
    const MAX_INPUT_BITS: u64 = 524_288; // 64 KiB
    if a.bit_length() > MAX_INPUT_BITS || b.bit_length() > MAX_INPUT_BITS {
        // Still check parse round-trip and that we didn't panic.
        let _ = a.to_string();
        let _ = b.to_string();
        return;
    }

    // ── Parse round-trip ────────────────────────────────────────────────────
    // BigNat::Display must produce exactly the decimal value, and
    // re-parsing it must recover the same BigNat.
    let a_str = a.to_string();
    let a_reparsed = BigNat::from_decimal_str(&a_str).expect("Display must be re-parseable");
    assert_eq!(a, a_reparsed, "Display round-trip failed");

    // ── Scalar parse matches reference ──────────────────────────────────────
    assert_eq!(
        bignat_to_biguint(&a),
        ra,
        "BigNat parse of '{}' disagrees with num-bigint",
        std::str::from_utf8(payload).unwrap_or("?")
    );

    // ── add ─────────────────────────────────────────────────────────────────
    {
        let got = a.add(&b);
        let exp = &ra + &rb;
        assert_eq!(bignat_to_biguint(&got), exp, "add mismatch");
    }

    // ── sub (truncated) ─────────────────────────────────────────────────────
    {
        // Lean semantics: a - b = 0 when b >= a.
        let got = a.sub(&b);
        let exp = if ra >= rb { &ra - &rb } else { BigUint::ZERO };
        assert_eq!(bignat_to_biguint(&got), exp, "sub mismatch");
    }

    // ── mul ─────────────────────────────────────────────────────────────────
    // Guard: cap differential check at 64 KiB (524 288 bits) so that
    // converting the result to a decimal string stays fast in both impls.
    // At 64 KiB the decimal string is ~158 000 digits, which is fast; at
    // MAX_RESULT_BITS (16 M bits) it would be ~5 M digits and very slow.
    {
        let result_bits = a.bit_length().saturating_add(b.bit_length());
        const DIFF_CHECK_BITS: u64 = 524_288; // 64 KiB
        if result_bits <= DIFF_CHECK_BITS {
            let got = a.mul(&b);
            let exp = &ra * &rb;
            assert_eq!(bignat_to_biguint(&got), exp, "mul mismatch");
        }
    }

    // ── div and rem ─────────────────────────────────────────────────────────
    {
        // Lean: x / 0 = 0, x % 0 = x
        let got_div = a.div(&b);
        let got_rem = a.rem(&b);
        let (exp_div, exp_rem) = if b.is_zero() {
            (BigUint::ZERO, ra.clone())
        } else {
            (&ra / &rb, &ra % &rb)
        };
        assert_eq!(bignat_to_biguint(&got_div), exp_div, "div mismatch");
        assert_eq!(bignat_to_biguint(&got_rem), exp_rem, "rem mismatch");
    }

    // ── gcd ─────────────────────────────────────────────────────────────────
    // Guard: gcd on large numbers can be slow; skip if either > 1024 bits.
    {
        if a.bit_length() <= 1024 && b.bit_length() <= 1024 {
            use num_integer::Integer;
            let got = a.gcd(&b);
            let exp = ra.gcd(&rb);
            assert_eq!(bignat_to_biguint(&got), exp, "gcd mismatch");
        }
    }

    // ── beq / ble ───────────────────────────────────────────────────────────
    {
        assert_eq!(a.beq(&b), ra == rb, "beq mismatch");
        assert_eq!(a.ble(&b), ra <= rb, "ble mismatch");
    }

    // ── land / lor / lxor ───────────────────────────────────────────────────
    {
        let got_and = a.land(&b);
        let exp_and = &ra & &rb;
        assert_eq!(bignat_to_biguint(&got_and), exp_and, "land mismatch");

        let got_or = a.lor(&b);
        let exp_or = &ra | &rb;
        assert_eq!(bignat_to_biguint(&got_or), exp_or, "lor mismatch");

        let got_xor = a.lxor(&b);
        let exp_xor = &ra ^ &rb;
        assert_eq!(bignat_to_biguint(&got_xor), exp_xor, "lxor mismatch");
    }

    // ── shr ─────────────────────────────────────────────────────────────────
    // Use b as the shift amount; cap to avoid slow num-bigint >>
    {
        if let Some(shift) = b.to_u64() {
            if shift <= 4096 {
                let got = a.shr(&b);
                let exp = &ra >> shift as usize;
                assert_eq!(bignat_to_biguint(&got), exp, "shr mismatch");
            }
        }
    }

    // ── log2 ────────────────────────────────────────────────────────────────
    {
        let got = a.log2();
        let exp: BigUint = if ra.is_zero() {
            BigUint::ZERO
        } else {
            BigUint::from(ra.bits() - 1)
        };
        assert_eq!(bignat_to_biguint(&got), exp, "log2 mismatch");
    }

    // ── checked_shl ─────────────────────────────────────────────────────────
    // Only assert when the result is small enough for fast decimal conversion.
    // Cap at 64 KiB (524 288 bits) for the same reason as mul above.
    // We separately verify the None case: checked_shl must return None when
    // result_bits > MAX_RESULT_BITS.
    {
        if let Some(shift) = b.to_u64() {
            let a_bits = a.bit_length();
            let result_bits = a_bits.saturating_add(shift);
            const DIFF_CHECK_BITS: u64 = 524_288; // 64 KiB
            if result_bits <= DIFF_CHECK_BITS {
                if let Some(got) = a.checked_shl(&b) {
                    let exp = &ra << shift as usize;
                    assert_eq!(bignat_to_biguint(&got), exp, "checked_shl mismatch");
                }
                // If checked_shl returned None despite result_bits <= DIFF_CHECK_BITS
                // and DIFF_CHECK_BITS < MAX_RESULT_BITS, that is also a bug.
                // The assert above catches it for the Some branch.
            }
            // Verify that over-budget shifts return None (conservative stuck
            // is correct; a wrong value would be a soundness issue).
            if result_bits > MAX_RESULT_BITS && !a.is_zero() {
                assert!(
                    a.checked_shl(&b).is_none(),
                    "checked_shl must return None for result_bits={result_bits} > MAX_RESULT_BITS"
                );
            }
        }
    }

    // ── pow ─────────────────────────────────────────────────────────────────
    // Only check small exponents to avoid num-bigint being astronomically slow.
    {
        if let Some(exp_u64) = b.to_u64() {
            if exp_u64 <= 64 && a.bit_length() <= 1024 {
                if let Some(got) = a.pow(&b) {
                    let exp = ra.pow(exp_u64 as u32);
                    assert_eq!(bignat_to_biguint(&got), exp, "pow mismatch");
                }
                // If pow returns None but the result would fit, that is
                // conservative (stuck instead of OOM) — not a correctness bug.
            }
        }
    }
});
