//! Harnesses over `oxilean-kernel`'s public `BigNat` API.
//!
//! Nothing here is copied from `oxilean-kernel`: this module is the *external*
//! specification of what the kernel's bignum arithmetic is supposed to do.
//! Every entry point carries a doc comment stating the property and the
//! **measured** L1 verdict for each obligation it raises; `EXPECTED.toml` at
//! the package root mirrors the same table machine-readably.
//!
//! # Two constraints that shape every harness below
//!
//! 1. **Rule W.** The encoder refuses to emit any SMT term wider than 64 bits,
//!    because the pinned solver mis-encodes `bvsub`/`bvneg` above that width
//!    and can return a *false* `unsat`. `BigNat`'s carry-propagating
//!    algorithms (`add_limbs`, `mul_*`, `div_rem_*`) all compute in `u128`, so
//!    every public operator that reaches one of them is `unsupported(width)`.
//!    That is a soundness rule, not an omission, and one harness
//!    ([`add_reaches_the_u128_carry_harness`]) exists to state it precisely.
//! 2. **`Vec` construction.** A one-element `vec![x]` lowers through
//!    `Box::new_uninit` and is refused by the encoder
//!    (`unsupported-type: union std::mem::MaybeUninit has no logical layout`).
//!    `BigNat::one()` and every `From<u64>`/`From<u32>`/`From<usize>` impl are
//!    written that way upstream, so no harness here constructs a `BigNat`
//!    through them: values come from `BigNat::from_limbs` (which takes an
//!    already-built `Vec`) and from `BigNat::zero()` (`Vec::new()`). Where a
//!    single-limb value is needed, it is built with `Vec::new()` + `push`.
//!    This is a harness-writing choice that changes no property.
//!
//! `from_limbs` **normalises**: it pops trailing zero limbs, so
//! `from_limbs(vec![0, 0])` is zero and the resulting limb count can be
//! smaller than the input vector's length. Several properties below are about
//! exactly that.

// A `#[harness]` body exists only under `formal` or under
// `all(test, oxiformal_runtime_checks)` (see `oxiformal_macros::harness`). In
// the plain build these items would otherwise be unused.
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(unused_imports)
)]
use oxilean_kernel::bignat::BigNat;
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(unused_imports)
)]
use std::cmp::Ordering;

use oxiformal::prelude::*;

/// Longest limb vector any harness here builds. Three limbs is 192 bits,
/// which is past every interesting boundary (empty, one limb, a carry out of
/// the top limb) while keeping the `unwind = 8` bound of
/// `[package.metadata.formal]` an exact unrolling rather than an
/// approximation.
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(dead_code)
)]
const MAX_LIMBS: usize = 3;

/// Two limbs — 128 bits — is the working width for the harnesses that build
/// *two* symbolic numbers, so that the pair still unrolls inside `unwind = 8`.
#[cfg_attr(
    not(any(formal, all(test, oxiformal_runtime_checks))),
    allow(dead_code)
)]
const PAIR_LIMBS: usize = 2;

/// Property: `from_limbs` establishes the representation invariant the whole
/// module rests on — no trailing zero limb, zero is the empty vector — for
/// every input, and never grows the vector.
///
/// This is the invariant that makes structural equality (`==`, `Hash`, and
/// hence the kernel's literal cache) coincide with numeric equality, which is
/// the reason `beq` can be `self == rhs` at all.
///
/// **Measured L1 verdict (2026-09-08): every obligation `unknown`.** The
/// three `assert` sites are `unknown` (two `solver-model-rejected`, one
/// `bounded`), and the `unwinding-assertion` of the normalisation loop itself
/// (`../src/bignat/mod.rs:78:15`) is `unknown (solver-model-rejected)` — OxiZ
/// 0.3.3 answers with a model that fails cargo-formal's mandatory model check
/// (upstream U-Z10). The `bounded` row is downstream of that same failure, not
/// of the bound: re-measured at `--unwind 16`, only the number in the message
/// changes.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn from_limbs_normalizes_trailing_zeros_harness() {
    let limbs: Vec<u64> = any_vec(MAX_LIMBS);
    let value = BigNat::from_limbs(limbs);
    assert(value.limb_count() <= MAX_LIMBS);
    assert(value.as_limbs().last() != Some(&0));
    assert(value.is_zero() == (value.limb_count() == 0));
}

/// Property: `to_u64` is `Some` exactly for the values that fit in one limb,
/// and the normalisation invariant makes "fits in zero limbs" and "is zero"
/// the same statement — so a one-limb `BigNat` can never carry the value `0`.
///
/// **Measured L1 verdict (2026-09-08): every obligation `unknown`.** All three
/// `assert` sites are `solver-model-rejected`; the two incidental
/// `bounds-check`s on `self.limbs[0]` (`../src/bignat/mod.rs:108`) and the
/// `unwinding-assertion` are `unknown` for the same root cause.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn to_u64_matches_the_limb_count_harness() {
    let limbs: Vec<u64> = any_vec(MAX_LIMBS);
    let value = BigNat::from_limbs(limbs);
    match value.to_u64() {
        Some(small) => {
            assert(value.limb_count() <= 1);
            assert((small == 0) == value.is_zero());
        }
        None => assert(value.limb_count() >= 2),
    }
}

/// Property: `to_u32` is `to_u64` composed with a fallible narrowing, written
/// upstream as `self.to_u64().and_then(|n| u32::try_from(n).ok())` — so this
/// harness exercises the encoder's closure handling and its `TryFrom`-between-
/// integers rule as well as the arithmetic.
///
/// **Measured L1 verdict (2026-09-08): `unsupported(no-body)`** —
/// "`oxilean_kernel::bignat::BigNat::to_u32::{closure-0}` is not in the module
/// set" (`../src/bignat/mod.rs:115:9`). Phase 2b's dependency-body lowering
/// reaches the functions of a path dependency, but not the **closures defined
/// inside** one, and `to_u32` is nothing but such a closure. No harness-side
/// spelling avoids it: the closure is in the code under test. This row is the
/// concrete ask for lowering external closure bodies.
///
/// Runtime-checks build: green, unmarked — `unsupported` is a statement about
/// the encoder, not about the property.
#[harness]
fn to_u32_agrees_with_to_u64_harness() {
    let limbs: Vec<u64> = any_vec(PAIR_LIMBS);
    let value = BigNat::from_limbs(limbs);
    match (value.to_u64(), value.to_u32()) {
        (Some(wide), Some(narrow)) => assert(u64::from(narrow) == wide),
        (Some(wide), None) => assert(wide > u64::from(u32::MAX)),
        (None, narrow) => assert(narrow.is_none()),
    }
}

/// Property: the bit length is zero exactly for zero, never exceeds 64 bits
/// per limb, and is strictly greater than `64 * (limb_count - 1)` for every
/// non-zero value — which is true *only because* the top limb is non-zero,
/// i.e. because of the normalisation invariant. A `bit_length` that could
/// return `0` for a non-zero number would make the kernel's `log2` reduction
/// wrong, and `log2` is `bit_length() - 1`.
///
/// **Measured L1 verdict (2026-09-08): every obligation `unknown`.** The three
/// `assert`s and the `arith-overflow-2` row are `solver-model-rejected`; the
/// six `arith-overflow` sites split three `solver-model-rejected` and three
/// `bounded`. Nothing here is an encoding gap — the harness encodes completely
/// and the solver is what does not decide it.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn bit_length_bounds_harness() {
    let limbs: Vec<u64> = any_vec(PAIR_LIMBS);
    let value = BigNat::from_limbs(limbs);
    let bits = value.bit_length();
    assert(bits <= 64 * (PAIR_LIMBS as u64));
    assert((bits == 0) == value.is_zero());
    // Guarded by `is_zero` so the `- 1` is only reached for a value with at
    // least one limb: the encoder sees the subtraction under that path
    // condition, which is what makes its `arith-overflow` obligation
    // discharge.
    assert(value.is_zero() || bits > 64 * (value.limb_count() as u64 - 1));
}

/// Property: `beq` (Lean's `Nat.beq`) is exactly limb-vector equality, and is
/// reflexive. Upstream writes it as `self == rhs` over the derived
/// `PartialEq`, so this states that the derived structural equality on the
/// *normalised* representation is the intended numeric equality.
///
/// **Measured L1 verdict (2026-09-08): every obligation `unknown`.** Both
/// `assert` sites are `bounded` and the `unwinding-assertion` is
/// `solver-model-rejected`.
///
/// This harness settles an open question of the Phase 2b inventory: the
/// derived `PartialEq for BigNat` **is** lowered from the dependency and
/// inlined, so `self == rhs` inside `beq` encodes. The harness is not
/// `unsupported(unsupported-callee)`.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn beq_agrees_with_limb_equality_harness() {
    let a = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let b = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    assert(a.beq(&b) == (a.as_limbs() == b.as_limbs()));
    assert(a.beq(&a));
}

/// Property: `<BigNat as Ord>::cmp` is the length-first, then
/// most-significant-limb-first lexicographic order on the normalised limbs.
/// This is what makes `BigNat`'s `Ord` agree with numeric order, and
/// therefore what `ble` — and any Lean proof that decides an inequality on
/// two `Nat` literals — rests on.
///
/// The spec is spelled out for at most two limbs: equal lengths compare on
/// the top limb, then on the bottom one.
///
/// **Measured L1 verdict (2026-09-08): `unsupported(aliasing)`** — "a
/// reference to a symbolically indexed element", at `cmp_limbs`'s
/// `a[i].cmp(&b[i])` (`../src/bignat/mod.rs:493:15`).
///
/// Read that refusal precisely, because it is *not* the one the Phase 2b
/// inventory predicted. The encoder's ordering fallback to a user
/// `PartialOrd`/`Ord` impl works here: `<BigNat as Ord>::cmp` is resolved to
/// the hand-written impl and inlined, and so is `cmp_limbs`. The refusal is
/// three frames deeper, on the `&b[i]` that `Ord::cmp`'s receiver autoref
/// builds over a symbolically indexed slot. So the ask this row produces is
/// "a reference to `seq[i]` for a symbolic `i`", not "an ordering fallback".
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn cmp_matches_lexicographic_order_harness() {
    let a = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let b = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let ordering = a.cmp(&b);
    let la = a.as_limbs();
    let lb = b.as_limbs();
    let expected = if la.len() != lb.len() {
        la.len().cmp(&lb.len())
    } else if la.is_empty() {
        Ordering::Equal
    } else if la[la.len() - 1] != lb[lb.len() - 1] {
        la[la.len() - 1].cmp(&lb[lb.len() - 1])
    } else {
        la[0].cmp(&lb[0])
    };
    assert(ordering == expected);
}

/// Property: `sub` implements Lean's **truncated** subtraction — `a - b = 0`
/// whenever `b >= a` — and `ble` is the decision procedure that selects that
/// branch. Both go through `self <= rhs` on `&BigNat`, i.e. through the
/// hand-written `Ord`/`PartialOrd` impls rather than through any `core`
/// comparison on a scalar.
///
/// Three statements: `ble` agrees with `cmp`; the difference is zero exactly
/// on the `ble` side; and it is non-zero on the other, which is the part that
/// makes "truncated" a real claim rather than "always returns zero".
///
/// **Measured L1 verdict (2026-09-08): `unsupported(unsupported-rvalue)`** —
/// "a reference has no field", at `cmp_limbs`'s `a.len()`
/// (`../src/bignat/mod.rs:489:8`).
///
/// This is a *different* refusal from the one
/// [`cmp_matches_lexicographic_order_harness`] gets, and the difference is the
/// finding. A direct `a.cmp(&b)` reaches `cmp_limbs` with slice arguments and
/// gets as far as line 493; `ble`'s `self <= rhs` on `&BigNat` goes through
/// `<&A as PartialOrd<&A>>::le` and arrives at the same user impl one
/// indirection deeper, so the very first `a.len()` is projected on a reference
/// and refused.
///
/// That the two refusals are entry-path-dependent, and not one merely masking
/// the other, was **measured**: with `let ordering = a.cmp(&b);` hoisted above
/// the `a.ble(&b)` below, this same harness comes back
/// `unsupported(aliasing)` at `:493:15` instead. The ordering here is
/// therefore deliberate — `ble` first, so that the package records *both*
/// distinct refusals (this one and
/// [`cmp_matches_lexicographic_order_harness`]'s) rather than the same one
/// twice. The verdict is `unsupported` either way; only the reason moves.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn sub_is_truncating_and_ble_agrees_harness() {
    let a = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let b = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let le = a.ble(&b);
    assert(le == (a.cmp(&b) != Ordering::Greater));
    let difference = a.sub(&b);
    if le {
        assert(difference.is_zero());
    } else {
        assert(!difference.is_zero());
    }
}

/// Property: the sum of two values of at most one limb has at most two limbs
/// and is at least as long as either operand.
///
/// This harness exists to state a refusal precisely. `BigNat::add` calls
/// `add_limbs`, whose carry is
/// `u128::from(long[i]) + u128::from(short[i]) + u128::from(carry)`, and
/// **Rule W** forbids emitting an SMT term wider than 64 bits because the
/// pinned solver mis-encodes wide `bvsub`/`bvneg` and can return a false
/// `unsat` — the one wrong answer no model check can catch. So the whole
/// harness is refused, and the same refusal covers `add_u64`, `succ`,
/// `mul`, `mul_u64`, `div`, `rem`, `pow` and `checked_shl`.
///
/// The statement itself is true; it becomes checkable when either the pinned
/// solver's wide bit-vector encoder is fixed and the conformance suite passes
/// at 128 bits, or the encoder learns to lower a `u128` add-with-carry into
/// two 64-bit terms.
///
/// **Measured L1 verdict (2026-09-08): `unsupported(width)`** —
/// "`<u128 as From<u64>>::from`: the result is 128 bits, over the 64-bit limit
/// of design rule W" (`../src/bignat/mod.rs:459:17`).
///
/// Runtime-checks build: green, unmarked — `unsupported` says nothing about
/// whether the property holds, and the plain-build test below exhibits the
/// carry it describes.
#[harness]
fn add_reaches_the_u128_carry_harness() {
    let a = BigNat::from_limbs(any_vec(1));
    let b = BigNat::from_limbs(any_vec(1));
    let sum = a.add(&b);
    assert(sum.limb_count() <= 2);
    assert(sum.limb_count() >= a.limb_count());
    assert(sum.limb_count() >= b.limb_count());
}

/// Property: `shr` is division by a power of two — shifting by `0` is the
/// identity, and shifting by anything never grows the limb count. In Lean
/// semantics `a >>> n = a / 2^n`, and the kernel's `log2` reduction depends
/// on the result staying normalised.
///
/// The shift amount is a `BigNat` built with `Vec::new()` + `push` rather
/// than `BigNat::from(k)`, because the `From<u64>` impl is a one-element
/// `vec![n]` the encoder refuses; `from_limbs` normalises, so `k == 0`
/// collapses to the empty limb vector, which is the same value. It is
/// *constructed* in `0..=127` by masking a `u8` rather than assumed in range,
/// so the randomized build keeps every draw instead of rejecting all but one
/// in 2^57.
///
/// **Measured L1 verdict (2026-09-08): 9 `timeout` and the rest `unknown`,
/// with the package's only two `proved` obligations.** Both proved rows are
/// `unwinding-assertion`s — one for this harness's own `any_vec` draw loop and
/// one inside `shr_bits`'s limb loop (`../src/bignat/mod.rs:405:13`). The
/// worst-of-sites rule nonetheless records that property as `timeout` in
/// `EXPECTED.toml`, because two other `unwinding-assertion` sites time out.
///
/// The cost is concentrated in `shr_bits`: `let limb_shift = (bits / 64)` and
/// `let bit_shift = (bits % 64)` (`../src/bignat/mod.rs:391`, `:392`) are
/// emitted as full 64-bit bit-blasted division and remainder even though the
/// divisor is the constant 64, and they sit under `bit_length`'s
/// `leading_zeros` and an eight-fold unrolled limb loop. Strength-reducing
/// division by a power-of-two constant to a shift and a mask is the concrete
/// encoder ask this harness produces. One of the nine timeouts is borderline
/// at the 30 000 ms budget and comes back `unknown` on a warm re-run; it
/// shares its `EXPECTED.toml` row with a site that times out either way, so
/// no recorded row moves.
///
/// An earlier, stronger version of this harness asserted the exact identity
/// `bit_length(a >>> k) == bit_length(a) - k` for `k < bit_length(a)`. It was
/// measured too: 37 `timeout` obligations and a 2.5-minute solve, deciding
/// nothing. It was reduced to the property above, which is the one the Phase
/// 2b harness inventory specifies.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn shr_is_a_power_of_two_division_harness() {
    let limbs: Vec<u64> = any_vec(PAIR_LIMBS);
    let value = BigNat::from_limbs(limbs);
    let zero = BigNat::zero();
    assert(value.shr(&zero).beq(&value));

    let raw: u8 = any();
    let mut shift_limbs: Vec<u64> = Vec::new();
    shift_limbs.push(u64::from(raw & 0x7f));
    let shifted = value.shr(&BigNat::from_limbs(shift_limbs));
    assert(shifted.limb_count() <= value.limb_count());
}

/// Property: the three bitwise operations respect the lattice bounds on limb
/// counts — `land` cannot be longer than the shorter operand, `lor` is
/// exactly as long as the longer one (its top limb is non-zero because the
/// longer operand's is), and `lxor` cannot exceed it.
///
/// This harness exists to state a refusal. `land` is
/// `(0..n).map(|i| ..).collect()` and `lor`/`lxor` are
/// `out.iter_mut().zip(short.iter())`; `map`, `collect` and `zip` are not in
/// the encoder's iteration model, so the harness aborts at whichever adapter
/// it reaches first and the whole thing is `unsupported(iterator)`. It is the
/// concrete ask for those three adapters, and the property is true.
///
/// **Measured L1 verdict (2026-09-08): `unsupported(iterator)`** —
/// "`std::iter::Iterator::map`: builds or advances an iterator outside the
/// bounded model", at `land`'s `(0..n).map(..).collect()`
/// (`../src/bignat/mod.rs:296:29`). The harness aborts there, so that is the
/// recorded reason; `lor` and `lxor`'s `out.iter_mut().zip(short.iter())`
/// (`:307`, `:321`) are the same gap one call later and would be reported
/// instead if `map`/`collect` were covered.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn bitwise_ops_respect_the_limb_count_lattice_harness() {
    let a = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let b = BigNat::from_limbs(any_vec(PAIR_LIMBS));
    let shorter = a.limb_count().min(b.limb_count());
    let longer = a.limb_count().max(b.limb_count());
    let and = a.land(&b);
    let or = a.lor(&b);
    let xor = a.lxor(&b);
    assert(and.limb_count() <= shorter);
    assert(or.limb_count() == longer);
    assert(xor.limb_count() <= longer);
}

/// Property: the four cheap accessors agree with each other and with the
/// normalisation invariant — `as_limbs().len()` is `limb_count()`, `is_zero`
/// is "no limbs", `is_one` implies a single limb holding `1`, and no value is
/// both zero and one.
///
/// The cheapest statement in the package, and therefore the clearest read on
/// what the solver can do with this shape at all.
///
/// **Measured L1 verdict (2026-09-08): every obligation `unknown`.** All five
/// `assert` sites and all three incidental `bounds-check`s are `bounded`, and
/// the `unwinding-assertion` of `from_limbs`'s normalisation loop is
/// `solver-model-rejected`. This is the harness that was re-measured at
/// `--unwind 16` to establish that the `bounded` rows across this whole
/// package are a consequence of that un-discharged unwinding assertion and not
/// of the bound: at 16 the verdicts are identical.
///
/// Runtime-checks build: green, unmarked.
#[harness]
fn as_limbs_and_the_predicates_agree_harness() {
    let limbs: Vec<u64> = any_vec(MAX_LIMBS);
    let value = BigNat::from_limbs(limbs);
    assert(value.as_limbs().len() == value.limb_count());
    assert(value.is_zero() == value.as_limbs().is_empty());
    assert(!(value.is_zero() && value.is_one()));
    if value.is_one() {
        assert(value.limb_count() == 1);
        assert(value.as_limbs()[0] == 1);
    }
}

#[cfg(all(test, not(oxiformal_runtime_checks), not(formal)))]
mod plain_tests {
    use super::*;

    #[test]
    fn from_limbs_strips_every_trailing_zero() {
        let value = BigNat::from_limbs(vec![7, 0, 0]);
        assert_eq!(value.as_limbs(), &[7]);
        assert_eq!(value.limb_count(), 1);
        assert!(!value.is_zero());

        let zero = BigNat::from_limbs(vec![0, 0]);
        assert!(zero.is_zero());
        assert_eq!(zero.limb_count(), 0);
        assert_eq!(zero.bit_length(), 0);
        assert_eq!(zero.to_u64(), Some(0));
    }

    #[test]
    fn to_u64_is_some_exactly_below_two_limbs() {
        assert_eq!(BigNat::from_limbs(vec![9]).to_u64(), Some(9));
        assert_eq!(BigNat::from_limbs(vec![0, 1]).to_u64(), None);
    }

    #[test]
    fn to_u32_narrows_only_when_the_value_fits() {
        assert_eq!(BigNat::from_limbs(vec![7]).to_u32(), Some(7));
        assert_eq!(
            BigNat::from_limbs(vec![u64::from(u32::MAX) + 1]).to_u32(),
            None
        );
        assert_eq!(BigNat::from_limbs(vec![0, 1]).to_u32(), None);
    }

    #[test]
    fn bit_length_counts_from_the_top_limb() {
        assert_eq!(BigNat::from_limbs(vec![1]).bit_length(), 1);
        assert_eq!(BigNat::from_limbs(vec![u64::MAX]).bit_length(), 64);
        assert_eq!(BigNat::from_limbs(vec![0, 1]).bit_length(), 65);
    }

    #[test]
    fn beq_is_limb_equality_after_normalisation() {
        let seven = BigNat::from_limbs(vec![7]);
        let seven_padded = BigNat::from_limbs(vec![7, 0]);
        assert!(seven.beq(&seven_padded));
        assert!(!seven.beq(&BigNat::from_limbs(vec![7, 1])));
    }

    #[test]
    fn cmp_orders_by_length_then_by_the_top_limb() {
        assert_eq!(
            BigNat::from_limbs(vec![0, 2]).cmp(&BigNat::from_limbs(vec![u64::MAX])),
            Ordering::Greater
        );
        assert_eq!(
            BigNat::from_limbs(vec![1, 2]).cmp(&BigNat::from_limbs(vec![3, 2])),
            Ordering::Less
        );
        assert_eq!(
            BigNat::from_limbs(vec![1, 2]).cmp(&BigNat::from_limbs(vec![1, 2])),
            Ordering::Equal
        );
    }

    #[test]
    fn sub_truncates_at_zero_and_ble_selects_the_branch() {
        let four = BigNat::from_limbs(vec![4]);
        let nine = BigNat::from_limbs(vec![9]);
        assert!(four.ble(&nine));
        assert!(four.sub(&nine).is_zero());
        assert!(!nine.ble(&four));
        let difference = nine.sub(&four);
        assert!(!difference.is_zero());
        assert_eq!(difference.to_u64(), Some(5));
        // Equal operands take the `ble` branch too: `a - a = 0`.
        assert!(nine.ble(&nine));
        assert!(nine.sub(&nine).is_zero());
    }

    #[test]
    fn add_carries_out_of_the_top_limb() {
        let max = BigNat::from_limbs(vec![u64::MAX]);
        let one = BigNat::from_limbs(vec![1]);
        let sum = max.add(&one);
        assert_eq!(sum.limb_count(), 2);
        assert_eq!(sum.as_limbs(), &[0, 1]);
        assert_eq!(max.add(&BigNat::zero()).as_limbs(), &[u64::MAX]);
    }

    #[test]
    fn shr_is_an_exact_power_of_two_division() {
        let value = BigNat::from_limbs(vec![0, 1]); // 2^64
        let zero = BigNat::zero();
        assert!(value.shr(&zero).beq(&value));
        assert_eq!(value.bit_length(), 65);
        // Shifting by 64 leaves a single set bit.
        assert_eq!(value.shr(&BigNat::from_limbs(vec![64])).to_u64(), Some(1));
        assert_eq!(value.shr(&BigNat::from_limbs(vec![64])).bit_length(), 1);
        // At or above the bit length the result is zero.
        assert!(value.shr(&BigNat::from_limbs(vec![65])).is_zero());
        assert!(value.shr(&BigNat::from_limbs(vec![127])).is_zero());
        // A sub-limb shift crosses the limb boundary (the funnel shift).
        assert_eq!(
            BigNat::from_limbs(vec![0, 1])
                .shr(&BigNat::from_limbs(vec![1]))
                .as_limbs(),
            &[1u64 << 63]
        );
    }

    #[test]
    fn bitwise_ops_respect_the_limb_count_lattice() {
        let a = BigNat::from_limbs(vec![0b1100, 1]);
        let b = BigNat::from_limbs(vec![0b1010]);
        assert_eq!(a.land(&b).as_limbs(), &[0b1000]);
        assert_eq!(a.lor(&b).as_limbs(), &[0b1110, 1]);
        assert_eq!(a.lxor(&b).as_limbs(), &[0b0110, 1]);
        assert_eq!(a.lor(&b).limb_count(), 2);
        assert!(a.land(&b).limb_count() <= b.limb_count());
        // `land` can normalise away every limb it touches.
        let disjoint = BigNat::from_limbs(vec![0b0011]);
        assert!(BigNat::from_limbs(vec![0b1100]).land(&disjoint).is_zero());
    }

    #[test]
    fn the_predicates_agree_with_the_limb_vector() {
        let one = BigNat::from_limbs(vec![1]);
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert_eq!(one.as_limbs().len(), one.limb_count());
        let zero = BigNat::zero();
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(zero.as_limbs().is_empty());
    }
}
