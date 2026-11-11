//! Structural hashing for the kernel's term types.
//!
//! [`Name`](crate::Name), [`Level`](crate::Level) and every shared subterm of
//! an [`Expr`](crate::Expr) (a [`Node`](crate::Node)) store a 64-bit hash of
//! their own structure. It is computed once, when the value is built, from
//! the stored hashes of the value's immediate children — so building a value
//! costs O(1) hashing work, and hashing a finished term costs O(1) however
//! large the term is and however much of it is shared.
//!
//! Before this, `Expr`'s `Hash` walked the whole term on every call, as a
//! TREE: a subterm reachable from several parents was hashed once per path.
//! The reducer's and the def-eq checker's caches are keyed by terms, so each
//! lookup paid for the full size of its key, and a term with nested sharing
//! paid exponentially.
//!
//! # What the hash is
//!
//! A pure function of exactly the structure `==` compares: two equal values
//! always have equal hashes, in every process (no per-process seed). That is
//! all the kernel relies on, in two places:
//!
//! * as the value written by `Hash`, where the usual `Hash`/`Eq` contract is
//!   what makes it correct;
//! * as a fast NEGATIVE test for `==` — different hashes prove two values
//!   differ. Equal hashes prove nothing: `==` then compares the structure as
//!   before, so a collision can cost time but can never make two different
//!   terms equal.
//!
//! It is not a cryptographic hash and nothing here treats it as one. It is
//! not keyed either: like the hash in Lean's own kernel, it is a fixed
//! function, so someone who sets out to can construct different terms that
//! share a hash. Such terms make the caches slower for the declaration that
//! contains them and change nothing else — every use above compares the
//! terms themselves before treating them as equal.
//!
//! # Seeds
//!
//! Each constructor starts from its own seed, so a `Lam` and a `Pi` over the
//! same children, or a string name component `"1"` and a numeric component
//! `1`, hash differently.

/// First operand key of [`mix`].
const KEY_STATE: u64 = 0x9E37_79B9_7F4A_7C15;
/// Second operand key of [`mix`]; different from [`KEY_STATE`], which is
/// what makes `mix(a, b)` and `mix(b, a)` differ.
const KEY_VALUE: u64 = 0xD1B5_4A32_D192_ED03;

/// Absorb one 64-bit `value` into a running hash `state`.
///
/// A folded 64×64→128-bit multiply: both operands are whitened with distinct
/// keys, multiplied, and the two halves of the product are XORed together.
/// Every input bit reaches every output bit, and the operation is not
/// commutative, so the order in which fields are absorbed matters —
/// `App(f, a)` and `App(a, f)` hash differently.
#[inline]
#[must_use]
pub(crate) const fn mix(state: u64, value: u64) -> u64 {
    // Two operands below 2^64 cannot overflow 128 bits.
    let product = ((state ^ KEY_STATE) as u128) * ((value ^ KEY_VALUE) as u128);
    ((product >> 64) as u64) ^ (product as u64)
}

/// A well-mixed constant derived from a small tag (the `splitmix64`
/// finalizer), used to give every constructor a distinct seed.
const fn seed(tag: u64) -> u64 {
    let mut z = tag.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// `Name` constructors.
pub(crate) const NAME_ANONYMOUS: u64 = seed(1);
pub(crate) const NAME_STR: u64 = seed(2);
pub(crate) const NAME_NUM: u64 = seed(3);

// `Level` constructors.
pub(crate) const LEVEL_ZERO: u64 = seed(10);
pub(crate) const LEVEL_SUCC: u64 = seed(11);
pub(crate) const LEVEL_MAX: u64 = seed(12);
pub(crate) const LEVEL_IMAX: u64 = seed(13);
pub(crate) const LEVEL_PARAM: u64 = seed(14);
pub(crate) const LEVEL_MVAR: u64 = seed(15);

// `Expr` constructors.
pub(crate) const EXPR_SORT: u64 = seed(20);
pub(crate) const EXPR_BVAR: u64 = seed(21);
pub(crate) const EXPR_FVAR: u64 = seed(22);
pub(crate) const EXPR_CONST: u64 = seed(23);
pub(crate) const EXPR_APP: u64 = seed(24);
pub(crate) const EXPR_LAM: u64 = seed(25);
pub(crate) const EXPR_PI: u64 = seed(26);
pub(crate) const EXPR_LET: u64 = seed(27);
pub(crate) const EXPR_LIT_NAT: u64 = seed(28);
pub(crate) const EXPR_LIT_STR: u64 = seed(29);
pub(crate) const EXPR_PROJ: u64 = seed(30);

/// Hash a byte string, starting from `state`.
///
/// The bytes are absorbed eight at a time (little-endian), a short final
/// group zero-padded, and the length last — so `"a"` and `"a\0"`, which pad to
/// the same final group, still differ.
#[must_use]
pub(crate) fn mix_bytes(state: u64, bytes: &[u8]) -> u64 {
    let mut hash = state;
    for group in bytes.chunks(8) {
        let word = group
            .iter()
            .enumerate()
            .fold(0u64, |acc, (i, &byte)| acc | (u64::from(byte) << (8 * i)));
        hash = mix(hash, word);
    }
    // `usize` is at most 64 bits wide on every supported target.
    mix(hash, bytes.len() as u64)
}

/// Hash a sequence of 64-bit words, starting from `state` (the limbs of a
/// natural-number literal). The length is absorbed last.
#[must_use]
pub(crate) fn mix_words(state: u64, words: &[u64]) -> u64 {
    let hash = words.iter().fold(state, |acc, &word| mix(acc, word));
    mix(hash, words.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_is_order_sensitive() {
        for (a, b) in [(1u64, 2u64), (0, 1), (u64::MAX, 0), (7, 0x1234_5678)] {
            assert_ne!(mix(a, b), mix(b, a), "mix({a}, {b}) is symmetric");
            assert_ne!(
                mix(mix(EXPR_APP, a), b),
                mix(mix(EXPR_APP, b), a),
                "absorbing {a} then {b} equals absorbing {b} then {a}"
            );
        }
    }

    #[test]
    fn mix_separates_small_consecutive_values() {
        // De Bruijn indices and free-variable ids are small consecutive
        // integers: they must not collide, and must not differ in a few low
        // bits only.
        let hashes: Vec<u64> = (0u64..4096).map(|n| mix(EXPR_BVAR, n)).collect();
        let distinct: std::collections::HashSet<u64> = hashes.iter().copied().collect();
        assert_eq!(distinct.len(), hashes.len());
        let high: std::collections::HashSet<u64> = hashes.iter().map(|h| h >> 48).collect();
        assert!(
            high.len() > 3500,
            "top 16 bits take only {} values over 4096 inputs",
            high.len()
        );
        let low: std::collections::HashSet<u64> = hashes.iter().map(|h| h & 0xFFFF).collect();
        assert!(
            low.len() > 3500,
            "bottom 16 bits take only {} values over 4096 inputs",
            low.len()
        );
    }

    #[test]
    fn seeds_are_pairwise_distinct() {
        let seeds = [
            NAME_ANONYMOUS,
            NAME_STR,
            NAME_NUM,
            LEVEL_ZERO,
            LEVEL_SUCC,
            LEVEL_MAX,
            LEVEL_IMAX,
            LEVEL_PARAM,
            LEVEL_MVAR,
            EXPR_SORT,
            EXPR_BVAR,
            EXPR_FVAR,
            EXPR_CONST,
            EXPR_APP,
            EXPR_LAM,
            EXPR_PI,
            EXPR_LET,
            EXPR_LIT_NAT,
            EXPR_LIT_STR,
            EXPR_PROJ,
        ];
        let distinct: std::collections::HashSet<u64> = seeds.iter().copied().collect();
        assert_eq!(distinct.len(), seeds.len());
    }

    #[test]
    fn mix_bytes_distinguishes_padding_and_length() {
        let h = |s: &str| mix_bytes(EXPR_LIT_STR, s.as_bytes());
        assert_ne!(h(""), h("\0"));
        assert_ne!(h("a"), h("a\0"));
        assert_ne!(h("abcdefgh"), h("abcdefgh\0"));
        assert_ne!(h("abcdefgh"), h("abcdefg"));
        assert_ne!(h("ab"), h("ba"));
        assert_eq!(h("Nat.add"), h("Nat.add"));
        // A group boundary inside the string is not a separator.
        assert_ne!(h("abcdefghi"), h("abcdefgh"));
    }

    #[test]
    fn mix_words_distinguishes_length() {
        assert_ne!(mix_words(EXPR_LIT_NAT, &[]), mix_words(EXPR_LIT_NAT, &[0]));
        assert_ne!(
            mix_words(EXPR_LIT_NAT, &[1, 2]),
            mix_words(EXPR_LIT_NAT, &[2, 1])
        );
        assert_eq!(
            mix_words(EXPR_LIT_NAT, &[5, 6, 7]),
            mix_words(EXPR_LIT_NAT, &[5, 6, 7])
        );
    }
}
