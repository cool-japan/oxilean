//! # BigNat — arbitrary-precision natural numbers for the kernel TCB
//!
//! Hand-written, zero-dependency, `unsafe`-free bignum backing
//! [`Literal::Nat`](crate::Literal::Nat). This module is part of the trusted
//! computing base: it is deliberately small, self-contained, and audited by
//! eye. All operations implement **exactly** the Lean 4 kernel semantics for
//! `Nat` literal reduction:
//!
//! | Operation | Lean semantics implemented here |
//! |---|---|
//! | `add` | ordinary addition |
//! | `sub` | truncated subtraction: `a - b = 0` when `b >= a` |
//! | `mul` | ordinary multiplication (schoolbook + Karatsuba) |
//! | `div` | Euclidean floor division, `x / 0 = 0` |
//! | `rem` | Euclidean remainder, `x % 0 = x` |
//! | `pow` | `x ^ 0 = 1` (incl. `0 ^ 0 = 1`); guarded, see below |
//! | `gcd` | Euclid; `gcd 0 b = b`, `gcd a 0 = a` |
//! | `beq` / `ble` | boolean equality / less-or-equal |
//! | `land` / `lor` / `lxor` | limbwise bitwise operations |
//! | `checked_shl` | `a <<< n = a * 2^n`; guarded, see below |
//! | `shr` | `a >>> n = a / 2^n` |
//! | `log2` | floor log2; `log2 0 = 0`, `log2 1 = 0` |
//!
//! ## Resource guard
//!
//! `pow` and `checked_shl` can materialize astronomically large numbers. When
//! the result would exceed [`MAX_RESULT_BITS`] they return `None`, and the
//! kernel leaves the term **stuck** instead of producing a wrong value. Every
//! other operation is bounded by the size of its inputs and cannot explode.
//!
//! ## Representation
//!
//! Little-endian base-2^64 limbs with the invariant that the most significant
//! limb (if any) is non-zero. Zero is the empty limb vector, so structural
//! equality (`==`, `Hash`) coincides with numeric equality.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// Limbs below this count multiply via schoolbook; at or above, Karatsuba.
const KARATSUBA_THRESHOLD: usize = 32;

/// Upper bound (in bits) on results materialized by `pow` / `checked_shl`.
///
/// 2^24 bits = 2 MiB per number (~5 million decimal digits). Exceeding this
/// bound makes the operation return `None` so reduction stays stuck rather
/// than exhausting memory or wrapping.
pub const MAX_RESULT_BITS: u64 = 1 << 24;

/// Largest power of ten that fits in a `u64` (10^19) — decimal chunk radix.
const DECIMAL_CHUNK_RADIX: u64 = 10_000_000_000_000_000_000;
/// Number of decimal digits per chunk (see [`DECIMAL_CHUNK_RADIX`]).
const DECIMAL_CHUNK_DIGITS: usize = 19;

/// Arbitrary-precision natural number (little-endian base-2^64 limbs).
///
/// Invariant: `limbs` has no trailing zero limbs; zero is the empty vector.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct BigNat {
    limbs: Vec<u64>,
}

impl BigNat {
    /// The natural number 0.
    pub const fn zero() -> Self {
        BigNat { limbs: Vec::new() }
    }

    /// The natural number 1.
    pub fn one() -> Self {
        BigNat { limbs: vec![1] }
    }

    /// Build from little-endian limbs, normalizing trailing zeros.
    pub fn from_limbs(mut limbs: Vec<u64>) -> Self {
        while limbs.last() == Some(&0) {
            limbs.pop();
        }
        BigNat { limbs }
    }

    /// The little-endian limb slice (no trailing zeros).
    pub fn as_limbs(&self) -> &[u64] {
        &self.limbs
    }

    /// Number of limbs (0 for the value 0).
    pub fn limb_count(&self) -> usize {
        self.limbs.len()
    }

    /// Whether this is 0.
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// Whether this is 1.
    pub fn is_one(&self) -> bool {
        self.limbs.len() == 1 && self.limbs[0] == 1
    }

    /// Convert to `u64` if the value fits.
    pub fn to_u64(&self) -> Option<u64> {
        match self.limbs.len() {
            0 => Some(0),
            1 => Some(self.limbs[0]),
            _ => None,
        }
    }

    /// Convert to `u32` if the value fits.
    pub fn to_u32(&self) -> Option<u32> {
        self.to_u64().and_then(|n| u32::try_from(n).ok())
    }

    /// Number of bits in the binary representation (`0` has bit length 0).
    pub fn bit_length(&self) -> u64 {
        match self.limbs.last() {
            None => 0,
            Some(&top) => {
                let full = (self.limbs.len() as u64 - 1) * 64;
                full + (64 - u64::from(top.leading_zeros()))
            }
        }
    }

    /// Successor: `n + 1`.
    pub fn succ(&self) -> Self {
        self.add_u64(1)
    }

    /// Predecessor with Lean semantics: `pred 0 = 0`.
    pub fn pred(&self) -> Self {
        if self.is_zero() {
            BigNat::zero()
        } else {
            self.sub_u64(1)
        }
    }

    /// Addition.
    pub fn add(&self, rhs: &Self) -> Self {
        BigNat::from_limbs(add_limbs(&self.limbs, &rhs.limbs))
    }

    /// Add a small value.
    pub fn add_u64(&self, rhs: u64) -> Self {
        self.add(&BigNat::from(rhs))
    }

    /// Truncated subtraction with Lean semantics: `a - b = 0` when `b >= a`.
    pub fn sub(&self, rhs: &Self) -> Self {
        if self <= rhs {
            BigNat::zero()
        } else {
            BigNat::from_limbs(sub_limbs(&self.limbs, &rhs.limbs))
        }
    }

    /// Subtract a small value (truncated at 0).
    pub fn sub_u64(&self, rhs: u64) -> Self {
        self.sub(&BigNat::from(rhs))
    }

    /// Multiplication (schoolbook below `KARATSUBA_THRESHOLD` limbs,
    /// Karatsuba at or above).
    pub fn mul(&self, rhs: &Self) -> Self {
        BigNat::from_limbs(mul_limbs(&self.limbs, &rhs.limbs))
    }

    /// Multiply by a small value.
    pub fn mul_u64(&self, rhs: u64) -> Self {
        if rhs == 0 || self.is_zero() {
            return BigNat::zero();
        }
        let mut out = Vec::with_capacity(self.limbs.len() + 1);
        let mut carry: u128 = 0;
        for &limb in &self.limbs {
            let t = u128::from(limb) * u128::from(rhs) + carry;
            out.push(t as u64);
            carry = t >> 64;
        }
        if carry != 0 {
            out.push(carry as u64);
        }
        BigNat::from_limbs(out)
    }

    /// Euclidean division with Lean semantics: `x / 0 = 0`.
    pub fn div(&self, rhs: &Self) -> Self {
        self.div_rem(rhs).0
    }

    /// Euclidean remainder with Lean semantics: `x % 0 = x`.
    pub fn rem(&self, rhs: &Self) -> Self {
        self.div_rem(rhs).1
    }

    /// Euclidean quotient and remainder with Lean semantics:
    /// `x / 0 = 0` and `x % 0 = x`.
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        if rhs.is_zero() {
            // Lean: x / 0 = 0, x % 0 = x.
            return (BigNat::zero(), self.clone());
        }
        if self < rhs {
            return (BigNat::zero(), self.clone());
        }
        if rhs.limbs.len() == 1 {
            let (q, r) = div_rem_limbs_by_u64(&self.limbs, rhs.limbs[0]);
            return (BigNat::from_limbs(q), BigNat::from(r));
        }
        let (q, r) = div_rem_knuth(&self.limbs, &rhs.limbs);
        (BigNat::from_limbs(q), BigNat::from_limbs(r))
    }

    /// Divide by a small non-zero value, returning quotient and remainder.
    /// With Lean semantics for zero: `(0, self)` when `rhs == 0`.
    pub fn div_rem_u64(&self, rhs: u64) -> (Self, u64) {
        if rhs == 0 {
            return (BigNat::zero(), 0);
        }
        let (q, r) = div_rem_limbs_by_u64(&self.limbs, rhs);
        (BigNat::from_limbs(q), r)
    }

    /// Exponentiation with Lean semantics: `x ^ 0 = 1` (including `0 ^ 0 = 1`).
    ///
    /// The exponent is **not** truncated. Returns `None` when the result
    /// would exceed [`MAX_RESULT_BITS`] bits, so the kernel leaves the term
    /// stuck instead of materializing an astronomical number (or a wrong
    /// one). For bases 0 and 1 the result is always small, so any exponent
    /// is accepted.
    pub fn pow(&self, exp: &Self) -> Option<Self> {
        if exp.is_zero() {
            return Some(BigNat::one());
        }
        if self.is_zero() {
            return Some(BigNat::zero());
        }
        if self.is_one() {
            return Some(BigNat::one());
        }
        // base >= 2: bound the result size before computing anything.
        // result bit length <= exp * bit_length(base); an exponent that does
        // not fit u64 is certainly over the bound.
        let e = exp.to_u64()?;
        let base_bits = self.bit_length();
        if u128::from(e) * u128::from(base_bits) > u128::from(MAX_RESULT_BITS) {
            return None;
        }
        // Square-and-multiply, least-significant exponent bit first.
        let mut result = BigNat::one();
        let mut base = self.clone();
        let mut e = e;
        loop {
            if e & 1 == 1 {
                result = result.mul(&base);
            }
            e >>= 1;
            if e == 0 {
                break;
            }
            base = base.mul(&base);
        }
        Some(result)
    }

    /// Greatest common divisor (Euclid). `gcd 0 b = b`, `gcd a 0 = a`.
    pub fn gcd(&self, rhs: &Self) -> Self {
        let mut a = self.clone();
        let mut b = rhs.clone();
        while !b.is_zero() {
            let r = a.rem(&b);
            a = b;
            b = r;
        }
        a
    }

    /// Boolean equality (`Nat.beq`).
    pub fn beq(&self, rhs: &Self) -> bool {
        self == rhs
    }

    /// Boolean less-or-equal (`Nat.ble`).
    pub fn ble(&self, rhs: &Self) -> bool {
        self <= rhs
    }

    /// Bitwise AND (`Nat.land`).
    pub fn land(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().min(rhs.limbs.len());
        let out: Vec<u64> = (0..n).map(|i| self.limbs[i] & rhs.limbs[i]).collect();
        BigNat::from_limbs(out)
    }

    /// Bitwise OR (`Nat.lor`).
    pub fn lor(&self, rhs: &Self) -> Self {
        let (long, short) = if self.limbs.len() >= rhs.limbs.len() {
            (&self.limbs, &rhs.limbs)
        } else {
            (&rhs.limbs, &self.limbs)
        };
        let mut out = long.clone();
        for (o, s) in out.iter_mut().zip(short.iter()) {
            *o |= *s;
        }
        BigNat::from_limbs(out)
    }

    /// Bitwise XOR (`Nat.xor`).
    pub fn lxor(&self, rhs: &Self) -> Self {
        let (long, short) = if self.limbs.len() >= rhs.limbs.len() {
            (&self.limbs, &rhs.limbs)
        } else {
            (&rhs.limbs, &self.limbs)
        };
        let mut out = long.clone();
        for (o, s) in out.iter_mut().zip(short.iter()) {
            *o ^= *s;
        }
        BigNat::from_limbs(out)
    }

    /// Left shift with Lean semantics: `a <<< n = a * 2^n`. High bits are
    /// **never** dropped. Returns `None` when the result would exceed
    /// [`MAX_RESULT_BITS`] bits (the kernel then leaves the term stuck).
    pub fn checked_shl(&self, n: &Self) -> Option<Self> {
        if self.is_zero() {
            return Some(BigNat::zero());
        }
        // A non-zero value shifted by an amount that does not fit u64 is
        // certainly over the bound.
        let n = n.to_u64()?;
        if u128::from(self.bit_length()) + u128::from(n) > u128::from(MAX_RESULT_BITS) {
            return None;
        }
        Some(self.shl_bits(n))
    }

    /// Right shift with Lean semantics: `a >>> n = a / 2^n`.
    pub fn shr(&self, n: &Self) -> Self {
        match n.to_u64() {
            // Shift amounts beyond u64 exceed any representable bit length.
            None => BigNat::zero(),
            Some(n) => self.shr_bits(n),
        }
    }

    /// Floor base-2 logarithm with Lean semantics:
    /// `log2 0 = 0`, `log2 1 = 0`, `log2 2 = 1`, ...
    pub fn log2(&self) -> Self {
        if self.is_zero() {
            BigNat::zero()
        } else {
            BigNat::from(self.bit_length() - 1)
        }
    }

    /// Shift left by `bits` (unguarded; used internally after bound checks).
    fn shl_bits(&self, bits: u64) -> Self {
        if self.is_zero() || bits == 0 {
            return self.clone();
        }
        let limb_shift = (bits / 64) as usize;
        let bit_shift = (bits % 64) as u32;
        let mut out = vec![0u64; limb_shift];
        if bit_shift == 0 {
            out.extend_from_slice(&self.limbs);
        } else {
            let mut carry = 0u64;
            for &limb in &self.limbs {
                out.push((limb << bit_shift) | carry);
                carry = limb >> (64 - bit_shift);
            }
            if carry != 0 {
                out.push(carry);
            }
        }
        BigNat::from_limbs(out)
    }

    /// Shift right by `bits`.
    fn shr_bits(&self, bits: u64) -> Self {
        if bits >= self.bit_length() {
            return BigNat::zero();
        }
        let limb_shift = (bits / 64) as usize;
        let bit_shift = (bits % 64) as u32;
        let src = &self.limbs[limb_shift..];
        if bit_shift == 0 {
            return BigNat::from_limbs(src.to_vec());
        }
        let mut out = Vec::with_capacity(src.len());
        for i in 0..src.len() {
            let lo = src[i] >> bit_shift;
            let hi = if i + 1 < src.len() {
                src[i + 1] << (64 - bit_shift)
            } else {
                0
            };
            out.push(lo | hi);
        }
        BigNat::from_limbs(out)
    }

    /// Parse a decimal string (digits only, e.g. lean4export `natVal`
    /// payloads). Leading zeros are accepted; an empty string or any
    /// non-digit character yields `None`.
    pub fn from_decimal_str(s: &str) -> Option<Self> {
        if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let bytes = s.as_bytes();
        let mut acc = BigNat::zero();
        let mut pos = 0;
        // First (possibly short) chunk, then full 19-digit chunks.
        let first_len = {
            let rem = bytes.len() % DECIMAL_CHUNK_DIGITS;
            if rem == 0 {
                DECIMAL_CHUNK_DIGITS
            } else {
                rem
            }
        };
        while pos < bytes.len() {
            let len = if pos == 0 {
                first_len
            } else {
                DECIMAL_CHUNK_DIGITS
            };
            let mut chunk: u64 = 0;
            for &b in &bytes[pos..pos + len] {
                chunk = chunk * 10 + u64::from(b - b'0');
            }
            let radix = if len == DECIMAL_CHUNK_DIGITS {
                DECIMAL_CHUNK_RADIX
            } else {
                10u64.pow(len as u32)
            };
            acc = acc.mul_u64(radix).add_u64(chunk);
            pos += len;
        }
        Some(acc)
    }
}

// ── Limb-level algorithms ────────────────────────────────────────────────────

/// Add two little-endian limb slices.
fn add_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(long.len() + 1);
    let mut carry = 0u64;
    for i in 0..long.len() {
        let s = u128::from(long[i])
            + u128::from(if i < short.len() { short[i] } else { 0 })
            + u128::from(carry);
        out.push(s as u64);
        carry = (s >> 64) as u64;
    }
    if carry != 0 {
        out.push(carry);
    }
    out
}

/// Subtract limb slices; requires `a >= b` numerically (internal invariant).
fn sub_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    debug_assert!(cmp_limbs(a, b) != Ordering::Less);
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0u64;
    for i in 0..a.len() {
        let bi = if i < b.len() { b[i] } else { 0 };
        let (d1, b1) = a[i].overflowing_sub(bi);
        let (d2, b2) = d1.overflowing_sub(borrow);
        out.push(d2);
        borrow = u64::from(b1) + u64::from(b2);
    }
    debug_assert_eq!(borrow, 0);
    out
}

/// Compare normalized little-endian limb slices numerically.
fn cmp_limbs(a: &[u64], b: &[u64]) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for i in (0..a.len()).rev() {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => {}
            ord => return ord,
        }
    }
    Ordering::Equal
}

/// Multiply limb slices, dispatching between schoolbook and Karatsuba.
fn mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    if a.len().min(b.len()) < KARATSUBA_THRESHOLD {
        mul_schoolbook(a, b)
    } else {
        mul_karatsuba(a, b)
    }
}

/// Schoolbook O(n*m) multiplication.
fn mul_schoolbook(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64; a.len() + b.len()];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        let ai = u128::from(ai);
        let mut carry: u128 = 0;
        for (j, &bj) in b.iter().enumerate() {
            let t = ai * u128::from(bj) + u128::from(out[i + j]) + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        // The target slot has never been written for this row (see the
        // schoolbook invariant), so the carry fits without further overflow.
        out[i + b.len()] = carry as u64;
    }
    out
}

/// Karatsuba multiplication: splits at half the longer operand.
///
/// With `a = a0 + a1*B^m` and `b = b0 + b1*B^m`:
/// `a*b = z0 + z1*B^m + z2*B^2m` where `z0 = a0*b0`, `z2 = a1*b1`, and
/// `z1 = (a0+a1)*(b0+b1) - z0 - z2` (non-negative by construction).
fn mul_karatsuba(a: &[u64], b: &[u64]) -> Vec<u64> {
    let m = a.len().max(b.len()) / 2;
    let (a0, a1) = a.split_at(a.len().min(m));
    let (b0, b1) = b.split_at(b.len().min(m));
    let z0 = mul_limbs(&normalized(a0), &normalized(b0));
    let z2 = mul_limbs(&normalized(a1), &normalized(b1));
    let a01 = add_limbs(&normalized(a0), &normalized(a1));
    let b01 = add_limbs(&normalized(b0), &normalized(b1));
    let z11 = mul_limbs(&normalized(&a01), &normalized(&b01));
    // z1 = z11 - z0 - z2 (exact, non-negative).
    let z1 = sub_limbs(&sub_limbs(&pad_to(&z11, z0.len().max(z2.len())), &z0), &z2);
    // Normalize before recombining so limb offsets stay within bounds
    // (each value provably fits: z0 < B^2m, z1*B^m and z2*B^2m <= a*b).
    let mut out = vec![0u64; a.len() + b.len() + 1];
    add_into(&mut out, &normalized(&z0), 0);
    add_into(&mut out, &normalized(&z1), m);
    add_into(&mut out, &normalized(&z2), 2 * m);
    out
}

/// Strip trailing zero limbs from a slice (returns an owned normalized Vec).
fn normalized(a: &[u64]) -> Vec<u64> {
    let mut v = a.to_vec();
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

/// Pad a limb vector with high zeros to at least `len` limbs.
fn pad_to(a: &[u64], len: usize) -> Vec<u64> {
    let mut v = a.to_vec();
    while v.len() < len {
        v.push(0);
    }
    v
}

/// Add `x` into `acc` starting at limb offset `shift` (in place; `acc` must
/// be long enough to absorb the carry — guaranteed by the Karatsuba caller).
fn add_into(acc: &mut [u64], x: &[u64], shift: usize) {
    let mut carry = 0u64;
    let mut i = 0;
    while i < x.len() || carry != 0 {
        let idx = shift + i;
        debug_assert!(idx < acc.len());
        let xi = if i < x.len() { x[i] } else { 0 };
        let s = u128::from(acc[idx]) + u128::from(xi) + u128::from(carry);
        acc[idx] = s as u64;
        carry = (s >> 64) as u64;
        i += 1;
    }
}

/// Divide a limb slice by a single non-zero limb.
fn div_rem_limbs_by_u64(a: &[u64], d: u64) -> (Vec<u64>, u64) {
    debug_assert!(d != 0);
    let d128 = u128::from(d);
    let mut q = vec![0u64; a.len()];
    let mut rem: u128 = 0;
    for i in (0..a.len()).rev() {
        let cur = (rem << 64) | u128::from(a[i]);
        q[i] = (cur / d128) as u64;
        rem = cur % d128;
    }
    (q, rem as u64)
}

/// Knuth Algorithm D long division (base 2^64) for a multi-limb divisor.
///
/// Preconditions: `v.len() >= 2`, `v` normalized (top limb non-zero),
/// `u >= v` numerically. Returns `(quotient, remainder)` limb vectors
/// (not necessarily normalized).
fn div_rem_knuth(u_in: &[u64], v_in: &[u64]) -> (Vec<u64>, Vec<u64>) {
    let n = v_in.len();
    debug_assert!(n >= 2);
    // D1: normalize so the divisor's top bit is set.
    let shift = v_in[n - 1].leading_zeros();
    let v = shl_slice(v_in, shift);
    let mut u = shl_slice(u_in, shift);
    debug_assert_eq!(v.len(), n);
    // Ensure u has an extra high limb for the algorithm.
    u.push(0);
    let m = u.len() - 1 - n;
    let mut q = vec![0u64; m + 1];
    let b: u128 = 1 << 64;
    // D2..D7: main loop over quotient digits, most significant first.
    for j in (0..=m).rev() {
        // D3: estimate qhat from the top two limbs of the current window.
        let top = (u128::from(u[j + n]) << 64) | u128::from(u[j + n - 1]);
        let mut qhat = top / u128::from(v[n - 1]);
        let mut rhat = top % u128::from(v[n - 1]);
        loop {
            if qhat >= b || qhat * u128::from(v[n - 2]) > (rhat << 64) | u128::from(u[j + n - 2]) {
                qhat -= 1;
                rhat += u128::from(v[n - 1]);
                if rhat < b {
                    continue;
                }
            }
            break;
        }
        // D4: multiply and subtract qhat * v from the window u[j..=j+n].
        let mut mul_carry: u128 = 0;
        let mut borrow: u64 = 0;
        for i in 0..n {
            let p = qhat * u128::from(v[i]) + mul_carry;
            mul_carry = p >> 64;
            let (d1, b1) = u[j + i].overflowing_sub(p as u64);
            let (d2, b2) = d1.overflowing_sub(borrow);
            u[j + i] = d2;
            borrow = u64::from(b1) + u64::from(b2);
        }
        let (d1, b1) = u[j + n].overflowing_sub(mul_carry as u64);
        let (d2, b2) = d1.overflowing_sub(borrow);
        u[j + n] = d2;
        // D5/D6: if the subtraction went negative, add v back once.
        if b1 || b2 {
            qhat -= 1;
            let mut carry = 0u64;
            for i in 0..n {
                let s = u128::from(u[j + i]) + u128::from(v[i]) + u128::from(carry);
                u[j + i] = s as u64;
                carry = (s >> 64) as u64;
            }
            // The final carry cancels the borrow (mod 2^64).
            u[j + n] = u[j + n].wrapping_add(carry);
        }
        q[j] = qhat as u64;
    }
    // D8: denormalize the remainder.
    let rem = shr_slice(&u[..n], shift);
    (q, rem)
}

/// Shift a limb slice left by `shift < 64` bits (may grow by one limb).
fn shl_slice(a: &[u64], shift: u32) -> Vec<u64> {
    if shift == 0 {
        return a.to_vec();
    }
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = 0u64;
    for &limb in a {
        out.push((limb << shift) | carry);
        carry = limb >> (64 - shift);
    }
    if carry != 0 {
        out.push(carry);
    }
    out
}

/// Shift a limb slice right by `shift < 64` bits.
fn shr_slice(a: &[u64], shift: u32) -> Vec<u64> {
    if shift == 0 {
        return a.to_vec();
    }
    let mut out = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let lo = a[i] >> shift;
        let hi = if i + 1 < a.len() {
            a[i + 1] << (64 - shift)
        } else {
            0
        };
        out.push(lo | hi);
    }
    out
}

// ── Trait implementations ────────────────────────────────────────────────────

impl Ord for BigNat {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_limbs(&self.limbs, &other.limbs)
    }
}

impl PartialOrd for BigNat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<u64> for BigNat {
    fn eq(&self, other: &u64) -> bool {
        self.to_u64() == Some(*other)
    }
}

impl PartialOrd<u64> for BigNat {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        Some(match self.to_u64() {
            Some(n) => n.cmp(other),
            None => Ordering::Greater,
        })
    }
}

impl From<u64> for BigNat {
    fn from(n: u64) -> Self {
        if n == 0 {
            BigNat::zero()
        } else {
            BigNat { limbs: vec![n] }
        }
    }
}

impl From<u32> for BigNat {
    fn from(n: u32) -> Self {
        BigNat::from(u64::from(n))
    }
}

impl From<u128> for BigNat {
    fn from(n: u128) -> Self {
        BigNat::from_limbs(vec![n as u64, (n >> 64) as u64])
    }
}

impl From<usize> for BigNat {
    fn from(n: usize) -> Self {
        BigNat::from(n as u64)
    }
}

impl fmt::Display for BigNat {
    /// Decimal representation (round-trips with [`BigNat::from_decimal_str`]).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }
        // Collect base-10^19 chunks, least significant first.
        let mut chunks: Vec<u64> = Vec::new();
        let mut cur = Cow::Borrowed(self);
        while !cur.is_zero() {
            let (q, r) = cur.div_rem_u64(DECIMAL_CHUNK_RADIX);
            chunks.push(r);
            cur = Cow::Owned(q);
        }
        let mut s = String::new();
        for (i, chunk) in chunks.iter().rev().enumerate() {
            if i == 0 {
                s.push_str(&chunk.to_string());
            } else {
                s.push_str(&format!("{:0width$}", chunk, width = DECIMAL_CHUNK_DIGITS));
            }
        }
        f.write_str(&s)
    }
}

impl fmt::Debug for BigNat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BigNat({})", self)
    }
}

/// Error returned when parsing a [`BigNat`] from a malformed string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParseBigNatError;

impl fmt::Display for ParseBigNatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid decimal natural number")
    }
}

impl std::error::Error for ParseBigNatError {}

impl FromStr for BigNat {
    type Err = ParseBigNatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigNat::from_decimal_str(s).ok_or(ParseBigNatError)
    }
}

#[cfg(test)]
mod tests;
