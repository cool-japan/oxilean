//! Functions for zero-knowledge proof systems.

use super::types::{
    Commitment, DlogProof, PedersenParams, RangeProofBits, SigmaProtocol, ZkProof, ZkStatement,
    ZkWitness,
};

// ── Modular arithmetic ────────────────────────────────────────────────────────

/// Fast modular exponentiation: `base^exp mod modulus`.
///
/// Uses the square-and-multiply algorithm.  Returns 1 when `modulus == 1`.
pub fn modpow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let mut b = (base as u128) % (modulus as u128);
    let mut e = exp;
    let m = modulus as u128;
    while e > 0 {
        if e & 1 == 1 {
            result = result * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    result as u64
}

/// Compute `(a * b) mod m` without overflow using 128-bit intermediate.
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Reduce a potentially-negative integer value modulo m (always returns [0, m)).
fn reduce_mod(x: i128, m: u64) -> u64 {
    let m_i = m as i128;
    let r = x % m_i;
    if r < 0 {
        (r + m_i) as u64
    } else {
        r as u64
    }
}

// ── Pedersen commitments ──────────────────────────────────────────────────────

/// Toy Pedersen parameters using a small safe prime for testing.
///
/// p = 1019 (prime), g = 2, h = 3 (generators of a large subgroup mod 1019).
pub fn toy_params() -> PedersenParams {
    PedersenParams {
        g: 2,
        h: 3,
        p: 1019,
    }
}

/// Compute the Pedersen commitment `g^x * h^r mod p`.
///
/// The secret value `x` is taken modulo (p-1) to handle negative values.
pub fn pedersen_commit(x: i64, r: u64, params: &PedersenParams) -> Commitment {
    let p = params.p;
    let order = p - 1; // group order for prime p (Fermat)
    let x_mod = reduce_mod(x as i128, order);
    let gx = modpow(params.g, x_mod, p);
    let hr = modpow(params.h, r, p);
    let value = mulmod(gx, hr, p);
    Commitment {
        value,
        randomness: r,
    }
}

/// Verify that a commitment opens correctly: check `g^x * h^r ≡ c.value (mod p)`.
pub fn pedersen_open(c: &Commitment, x: i64, r: u64, params: &PedersenParams) -> bool {
    let expected = pedersen_commit(x, r, params);
    expected.value == c.value && c.randomness == r
}

// ── Discrete-log proofs (Schnorr) ─────────────────────────────────────────────

/// Prove knowledge of `secret` such that `generator^secret ≡ public (mod p)`.
///
/// This is a non-interactive Schnorr proof using a deterministic nonce derived
/// from `seed` via a simple mixing function (not a cryptographic hash — toy only).
///
/// Protocol:
///   1. Nonce `k = mix(seed, secret) mod (p-1)`.
///   2. Commitment `R = g^k mod p`.
///   3. Challenge `c = mix(R, public) mod (p-1)`.
///   4. Response `s = k - c * secret` (integer, may be negative).
pub fn dlog_prove(secret: i64, generator: u64, p: u64, seed: u64) -> DlogProof {
    let order = p - 1;
    // Derive nonce k deterministically.
    let k_raw = toy_mix(seed, secret as u64) % order;
    let k = if k_raw == 0 { 1 } else { k_raw };
    let commitment = modpow(generator, k, p);
    // Public key y = g^secret mod p.
    let secret_mod = reduce_mod(secret as i128, order);
    let public = modpow(generator, secret_mod, p);
    let challenge = toy_mix(commitment, public) % order;
    // s = k - c * secret  (integer arithmetic, unreduced)
    let s: i64 = (k as i64) - (challenge as i64) * secret;
    DlogProof {
        commitment,
        challenge,
        response: s,
    }
}

/// Verify a Schnorr discrete-log proof.
///
/// Checks: `g^s * public^c ≡ R (mod p)`.
///
/// Reduces `s` into the range `[0, p-1)` before exponentiating.
pub fn dlog_verify(public: u64, proof: &DlogProof, generator: u64, p: u64) -> bool {
    let order = p - 1;
    let s_mod = reduce_mod(proof.response as i128, order);
    let c_mod = proof.challenge % order;
    let gs = modpow(generator, s_mod, p);
    let yc = modpow(public, c_mod, p);
    let lhs = mulmod(gs, yc, p);
    lhs == proof.commitment
}

// ── Sigma protocol properties ─────────────────────────────────────────────────

/// Return the three standard properties of the basic (Schnorr) Sigma protocol.
pub fn sigma_protocol_properties() -> SigmaProtocol {
    SigmaProtocol {
        name: "Schnorr".to_string(),
        completeness: true,
        soundness: true,
        zero_knowledge: true,
    }
}

// ── Range proofs ──────────────────────────────────────────────────────────────

/// Prove that `lo ≤ x ≤ hi` using bit decomposition.
///
/// Decomposes `v = x - lo` into bits `b_0, …, b_{k-1}` (k = ceil(log2(hi-lo+1))).
/// For each bit produces a DlogProof of `g^{b_i} mod p` using `params.g` as generator.
///
/// Returns `None` if `x < lo` or `x > hi`.
pub fn prove_range(
    x: u64,
    lo: u64,
    hi: u64,
    params: &PedersenParams,
    seed: u64,
) -> Option<RangeProofBits> {
    if x < lo || x > hi {
        return None;
    }
    let v = x - lo;
    let range_size = hi - lo;
    let num_bits = if range_size == 0 {
        1usize
    } else {
        (u64::BITS - range_size.leading_zeros()) as usize
    };

    let mut bits = Vec::with_capacity(num_bits);
    for i in 0..num_bits {
        let bit = (v >> i) & 1; // 0 or 1
                                // secret = bit (0 or 1), g^0 = 1, g^1 = g mod p
        let proof = dlog_prove(bit as i64, params.g, params.p, toy_mix(seed, i as u64));
        bits.push(proof);
    }
    Some(RangeProofBits {
        bits,
        range: (lo, hi),
    })
}

/// Verify a bit-decomposition range proof.
///
/// For each bit proof checks: `g^s * pub^c ≡ R (mod p)` where `pub` is either
/// `1` (bit=0) or `g` (bit=1).  We verify against both possibilities and accept
/// if either passes — this is the toy "OR proof" approximation.
pub fn verify_range_proof(
    proof: &RangeProofBits,
    lo: u64,
    hi: u64,
    params: &PedersenParams,
) -> bool {
    if proof.range != (lo, hi) {
        return false;
    }
    let p = params.p;
    let g = params.g;
    for bit_proof in &proof.bits {
        // Check if valid for bit=0 (public = g^0 = 1) OR bit=1 (public = g^1 = g).
        let valid_zero = dlog_verify(1, bit_proof, g, p);
        let valid_one = dlog_verify(g, bit_proof, g, p);
        if !valid_zero && !valid_one {
            return false;
        }
    }
    true
}

// ── ZkProof construction helpers ──────────────────────────────────────────────

/// Construct a ZK proof for a witness under a given statement.
///
/// This is a toy multi-secret Sigma protocol: for each secret value `w_i` in
/// the witness we run a Schnorr-like dlog_prove with generator `g` and
/// accumulate commitment/response vectors.
pub fn build_zk_proof(
    statement: &ZkStatement,
    witness: &ZkWitness,
    params: &PedersenParams,
    seed: u64,
) -> ZkProof {
    let g = params.g;
    let p = params.p;
    let mut commitments = Vec::new();
    let mut responses = Vec::new();
    let mut challenge_acc: u64 = 0;

    for (i, &w) in witness.secret_values.iter().enumerate() {
        let proof = dlog_prove(w, g, p, toy_mix(seed, i as u64));
        commitments.push(proof.commitment);
        responses.push(proof.response);
        challenge_acc = toy_mix(challenge_acc, proof.challenge);
    }
    // Add statement description into challenge for binding.
    for ch in statement.description.bytes() {
        challenge_acc = toy_mix(challenge_acc, ch as u64);
    }
    ZkProof {
        challenge: challenge_acc % (p - 1),
        response: responses,
        commitment: commitments,
    }
}

/// Verify a multi-secret ZK proof given public keys and parameters.
///
/// `public_keys\[i\] = g^{w_i} mod p` must be provided by the verifier.
pub fn verify_zk_proof(proof: &ZkProof, public_keys: &[u64], params: &PedersenParams) -> bool {
    if proof.commitment.len() != public_keys.len() || proof.response.len() != public_keys.len() {
        return false;
    }
    let g = params.g;
    let p = params.p;
    let order = p - 1;
    // Recompute per-index challenges from commitments.
    for (i, (&pk, (&r, &c))) in public_keys
        .iter()
        .zip(proof.commitment.iter().zip(proof.response.iter()))
        .enumerate()
    {
        let _ = i;
        let s_mod = reduce_mod(c as i128, order);
        let expected_challenge = toy_mix(r, pk) % order;
        let gs = modpow(g, s_mod, p);
        let yc = modpow(pk, expected_challenge, p);
        let lhs = mulmod(gs, yc, p);
        if lhs != r {
            return false;
        }
    }
    true
}

// ── Utility ───────────────────────────────────────────────────────────────────

/// Simple non-cryptographic mixing function for deterministic test nonces.
///
/// Mixes two u64 values using xorshift-like operations.  NOT secure.
fn toy_mix(a: u64, b: u64) -> u64 {
    let mut x = a.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut y = b.wrapping_add(0x6c62_272e_07bb_0142);
    x = x.wrapping_mul(0x517c_c1b7_2722_0a95);
    y ^= x.rotate_left(17);
    y = y.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    y ^ x
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modpow_simple() {
        assert_eq!(modpow(2, 10, 1000), 24); // 1024 mod 1000
        assert_eq!(modpow(3, 0, 7), 1);
        assert_eq!(modpow(0, 5, 13), 0);
        assert_eq!(modpow(5, 1, 7), 5);
    }

    #[test]
    fn test_modpow_mod_one() {
        assert_eq!(modpow(999, 999, 1), 0);
    }

    #[test]
    fn test_modpow_large() {
        // 2^62 mod 1019 — just check it doesn't overflow
        let result = modpow(2, 62, 1019);
        assert!(result < 1019);
    }

    #[test]
    fn test_toy_params_valid() {
        let p = toy_params();
        assert_eq!(p.p, 1019);
        assert_eq!(p.g, 2);
        assert_eq!(p.h, 3);
    }

    #[test]
    fn test_pedersen_commit_deterministic() {
        let params = toy_params();
        let c1 = pedersen_commit(5, 7, &params);
        let c2 = pedersen_commit(5, 7, &params);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_pedersen_commit_open_valid() {
        let params = toy_params();
        let c = pedersen_commit(42, 13, &params);
        assert!(pedersen_open(&c, 42, 13, &params));
    }

    #[test]
    fn test_pedersen_open_wrong_secret() {
        let params = toy_params();
        let c = pedersen_commit(42, 13, &params);
        assert!(!pedersen_open(&c, 43, 13, &params));
    }

    #[test]
    fn test_pedersen_open_wrong_randomness() {
        let params = toy_params();
        let c = pedersen_commit(42, 13, &params);
        assert!(!pedersen_open(&c, 42, 14, &params));
    }

    #[test]
    fn test_pedersen_commit_zero() {
        let params = toy_params();
        let c = pedersen_commit(0, 0, &params);
        // g^0 * h^0 = 1 * 1 = 1
        assert_eq!(c.value, 1);
    }

    #[test]
    fn test_pedersen_commit_negative_secret() {
        let params = toy_params();
        // Negative secrets should be reduced mod (p-1) and still open.
        let c = pedersen_commit(-5, 3, &params);
        assert!(pedersen_open(&c, -5, 3, &params));
    }

    #[test]
    fn test_pedersen_hiding() {
        let params = toy_params();
        let c1 = pedersen_commit(7, 1, &params);
        let c2 = pedersen_commit(7, 2, &params);
        assert_ne!(
            c1.value, c2.value,
            "Different randomness should give different commitments"
        );
    }

    #[test]
    fn test_dlog_prove_verify_basic() {
        let params = toy_params();
        let p = params.p;
        let g = params.g;
        let secret: i64 = 42;
        let order = (p - 1) as i64;
        let secret_mod = ((secret % order) + order) as u64 % (p - 1);
        let public = modpow(g, secret_mod, p);
        let proof = dlog_prove(secret, g, p, 12345);
        assert!(dlog_verify(public, &proof, g, p));
    }

    #[test]
    fn test_dlog_prove_verify_secret_one() {
        let params = toy_params();
        let p = params.p;
        let g = params.g;
        let public = modpow(g, 1, p); // g^1
        let proof = dlog_prove(1, g, p, 999);
        assert!(dlog_verify(public, &proof, g, p));
    }

    #[test]
    fn test_dlog_prove_verify_secret_zero() {
        let params = toy_params();
        let p = params.p;
        let g = params.g;
        let public = modpow(g, 0, p); // g^0 = 1
        let proof = dlog_prove(0, g, p, 1);
        assert!(dlog_verify(public, &proof, g, p));
    }

    #[test]
    fn test_dlog_verify_wrong_public() {
        let params = toy_params();
        let p = params.p;
        let g = params.g;
        let secret: i64 = 10;
        let proof = dlog_prove(secret, g, p, 42);
        let wrong_public = modpow(g, 11, p); // g^11 ≠ g^10
        assert!(!dlog_verify(wrong_public, &proof, g, p));
    }

    #[test]
    fn test_sigma_protocol_properties() {
        let sp = sigma_protocol_properties();
        assert_eq!(sp.name, "Schnorr");
        assert!(sp.completeness);
        assert!(sp.soundness);
        assert!(sp.zero_knowledge);
    }

    #[test]
    fn test_prove_range_valid_in_range() {
        let params = toy_params();
        let proof = prove_range(5, 0, 15, &params, 777);
        assert!(proof.is_some());
    }

    #[test]
    fn test_prove_range_out_of_range_low() {
        let params = toy_params();
        assert!(prove_range(3, 5, 10, &params, 1).is_none());
    }

    #[test]
    fn test_prove_range_out_of_range_high() {
        let params = toy_params();
        assert!(prove_range(11, 5, 10, &params, 2).is_none());
    }

    #[test]
    fn test_prove_range_boundary_lo() {
        let params = toy_params();
        let proof = prove_range(5, 5, 10, &params, 3);
        assert!(proof.is_some());
    }

    #[test]
    fn test_prove_range_boundary_hi() {
        let params = toy_params();
        let proof = prove_range(10, 5, 10, &params, 4);
        assert!(proof.is_some());
    }

    #[test]
    fn test_verify_range_proof_valid() {
        let params = toy_params();
        let proof = prove_range(7, 0, 15, &params, 100).expect("in range");
        assert!(verify_range_proof(&proof, 0, 15, &params));
    }

    #[test]
    fn test_verify_range_proof_wrong_range() {
        let params = toy_params();
        let proof = prove_range(7, 0, 15, &params, 100).expect("in range");
        // Wrong range pair.
        assert!(!verify_range_proof(&proof, 1, 15, &params));
    }

    #[test]
    fn test_build_zk_proof_structure() {
        let params = toy_params();
        let stmt = ZkStatement {
            description: "know x".to_string(),
            public_params: vec!["g".to_string(), "p".to_string()],
        };
        let witness = ZkWitness {
            secret_values: vec![3, 7],
        };
        let proof = build_zk_proof(&stmt, &witness, &params, 42);
        assert_eq!(proof.commitment.len(), 2);
        assert_eq!(proof.response.len(), 2);
    }

    #[test]
    fn test_verify_zk_proof_valid() {
        let params = toy_params();
        let g = params.g;
        let p = params.p;
        let secrets = vec![5i64, 11i64];
        let public_keys: Vec<u64> = secrets
            .iter()
            .map(|&s| {
                let s_mod = ((s % (p as i64 - 1)) + (p as i64 - 1)) as u64 % (p - 1);
                modpow(g, s_mod, p)
            })
            .collect();
        let stmt = ZkStatement {
            description: "test".to_string(),
            public_params: vec![],
        };
        let witness = ZkWitness {
            secret_values: secrets,
        };
        let proof = build_zk_proof(&stmt, &witness, &params, 99);
        assert!(verify_zk_proof(&proof, &public_keys, &params));
    }

    #[test]
    fn test_verify_zk_proof_empty_witness() {
        let params = toy_params();
        let stmt = ZkStatement {
            description: "empty".to_string(),
            public_params: vec![],
        };
        let witness = ZkWitness {
            secret_values: vec![],
        };
        let proof = build_zk_proof(&stmt, &witness, &params, 1);
        assert!(verify_zk_proof(&proof, &[], &params));
    }

    #[test]
    fn test_zk_statement_fields() {
        let stmt = ZkStatement {
            description: "knows discrete log".to_string(),
            public_params: vec!["g".to_string(), "p".to_string(), "y".to_string()],
        };
        assert_eq!(stmt.public_params.len(), 3);
        assert!(stmt.description.contains("discrete log"));
    }

    #[test]
    fn test_zk_witness_fields() {
        let w = ZkWitness {
            secret_values: vec![1, 2, 3],
        };
        assert_eq!(w.secret_values.len(), 3);
        assert_eq!(w.secret_values[2], 3);
    }
}
