//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Schnorr identification protocol parameters.
///
/// Proves knowledge of discrete logarithm x such that g^x = y (mod p).
///
/// # WARNING
/// This is an educational implementation with tiny parameters.
/// Real Schnorr uses 256-bit+ groups.
#[derive(Debug, Clone)]
pub struct SchnorrParams {
    /// Prime modulus p
    pub p: u64,
    /// Prime order q of the group (q | p-1)
    pub q: u64,
    /// Generator g of order q
    pub g: u64,
}
impl SchnorrParams {
    /// Prover commits with randomness r; returns (commitment, r).
    ///
    /// # WARNING
    /// Educational only. Use a cryptographic RNG in production.
    pub fn commit(&self, r: u64) -> u64 {
        mod_exp(self.g, r % self.q, self.p)
    }
    /// Prover computes response z = (r + e * x) mod q.
    pub fn respond(&self, r: u64, challenge: u64, secret_x: u64) -> u64 {
        let r = r % self.q;
        let e_x = (challenge as u128 * secret_x as u128 % self.q as u128) as u64;
        (r + e_x) % self.q
    }
    /// Verifier checks: g^z = a · y^e (mod p) where y = g^x.
    pub fn verify(&self, transcript: &SchnorrTranscript, public_y: u64) -> bool {
        let lhs = mod_exp(self.g, transcript.response, self.p);
        let ye = mod_exp(public_y, transcript.challenge, self.p);
        let rhs = (transcript.commitment as u128 * ye as u128 % self.p as u128) as u64;
        lhs == rhs
    }
    /// Complete Schnorr proof-of-knowledge for secret x with randomness r and challenge e.
    pub fn prove(&self, secret_x: u64, r: u64, challenge: u64) -> SchnorrTranscript {
        let commitment = self.commit(r);
        let response = self.respond(r, challenge, secret_x);
        SchnorrTranscript {
            commitment,
            challenge,
            response,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MPCSecurityModel {
    SemiHonest,
    Malicious,
    Covert,
}
/// Shamir (t, n) secret sharing over a prime field F_p.
///
/// # WARNING
/// Educational implementation. Uses small prime fields, unsuitable for production.
#[derive(Debug, Clone)]
pub struct ShamirSS {
    /// Field prime p
    pub p: u64,
    /// Threshold t: minimum shares needed to reconstruct
    pub t: usize,
    /// Total shares n
    pub n: usize,
}
impl ShamirSS {
    /// Split secret `s` into n shares using a random degree-(t-1) polynomial.
    ///
    /// Coefficients a\[0\]=s, a[1..t] come from `coeffs` (length t-1).
    ///
    /// # WARNING
    /// In production, coefficients must be uniformly random elements of F_p.
    pub fn split(&self, secret: u64, coeffs: &[u64]) -> Vec<(u64, u64)> {
        assert_eq!(
            coeffs.len(),
            self.t - 1,
            "Need exactly t-1 random coefficients"
        );
        (1..=(self.n as u64))
            .map(|i| {
                let mut val: u128 = secret as u128;
                let mut ipow: u128 = i as u128;
                for &c in coeffs {
                    val = (val + c as u128 * ipow) % self.p as u128;
                    ipow = ipow * i as u128 % self.p as u128;
                }
                (i, val as u64)
            })
            .collect()
    }
    /// Reconstruct secret from any t shares using Lagrange interpolation mod p.
    ///
    /// `shares`: slice of (x_i, y_i) pairs.
    pub fn reconstruct(&self, shares: &[(u64, u64)]) -> u64 {
        assert!(shares.len() >= self.t, "Need at least t shares");
        let shares = &shares[..self.t];
        let p = self.p;
        let mut secret: i128 = 0;
        for (j, &(xj, yj)) in shares.iter().enumerate() {
            let mut num: i128 = 1;
            let mut den: i128 = 1;
            for (k, &(xk, _)) in shares.iter().enumerate() {
                if k == j {
                    continue;
                }
                num = num * (-(xk as i128)) % p as i128;
                den = den * (xj as i128 - xk as i128) % p as i128;
            }
            let den_inv =
                mod_inv(((den % p as i128 + p as i128) % p as i128) as u64, p).unwrap_or(0) as i128;
            let lagrange = num * den_inv % p as i128;
            secret = (secret + yj as i128 * lagrange) % p as i128;
        }
        ((secret % p as i128 + p as i128) % p as i128) as u64
    }
}
/// Toy 1-of-2 Oblivious Transfer parameters.
///
/// Based on simplified Naor-Pinkas OT using DH assumptions.
///
/// # WARNING
/// This is a simplified, non-secure educational sketch. Real OT requires
/// careful implementation with secure group operations and hash functions.
#[derive(Debug, Clone)]
pub struct ToyOT {
    /// DH group prime p
    pub p: u64,
    /// Generator g
    pub g: u64,
}
impl ToyOT {
    /// Sender setup: pick random a, publish c = g^a mod p.
    pub fn sender_setup(&self, a: u64) -> u64 {
        mod_exp(self.g, a, self.p)
    }
    /// Receiver message for choice bit b ∈ {0, 1}: picks k, sends pk_b = g^k,
    /// sets pk_{1-b} = c / g^k (implicitly). Returns (pk0, pk1) for bit b.
    pub fn receiver_choose(&self, c: u64, b: u8, k: u64) -> (u64, u64) {
        let gk = mod_exp(self.g, k, self.p);
        let gk_inv = mod_inv(gk, self.p).unwrap_or(1);
        let other = (c as u128 * gk_inv as u128 % self.p as u128) as u64;
        if b == 0 {
            (gk, other)
        } else {
            (other, gk)
        }
    }
    /// Receiver derives the shared key for their chosen bit b.
    pub fn receiver_key(&self, c: u64, b: u8, k: u64) -> u64 {
        let _ = b;
        mod_exp(c, k, self.p)
    }
}
/// Oblivious transfer (OT) protocol.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ObliviousTransfer {
    pub variant: OTVariant,
    pub security_parameter: usize,
}
#[allow(dead_code)]
impl ObliviousTransfer {
    pub fn new(variant: OTVariant, sec: usize) -> Self {
        ObliviousTransfer {
            variant,
            security_parameter: sec,
        }
    }
    pub fn is_fundamental(&self) -> bool {
        matches!(self.variant, OTVariant::OneOutOfTwo)
    }
    pub fn n_messages(&self) -> usize {
        match &self.variant {
            OTVariant::OneOutOfTwo => 2,
            OTVariant::OneOutOfN(n) => *n,
            OTVariant::RandomOT => 2,
        }
    }
}
/// Extended Pedersen commitment with batch verification.
///
/// Supports committing to a vector of values and homomorphic operations.
///
/// # WARNING
/// Educational only. Uses toy parameters.
#[derive(Debug, Clone)]
pub struct PedersenCommitment {
    /// Prime field modulus
    pub p: u64,
    /// Group order (prime q | p-1)
    pub q: u64,
    /// Generator g
    pub g: u64,
    /// Independent generator h (log_g(h) unknown)
    pub h: u64,
}
impl PedersenCommitment {
    /// Commit: C = g^m * h^r mod p
    pub fn commit(&self, m: u64, r: u64) -> u64 {
        let gm = mod_exp(self.g, m % self.q, self.p);
        let hr = mod_exp(self.h, r % self.q, self.p);
        (gm as u128 * hr as u128 % self.p as u128) as u64
    }
    /// Verify opening: check C == g^m * h^r mod p
    pub fn verify(&self, c: u64, m: u64, r: u64) -> bool {
        self.commit(m, r) == c
    }
    /// Homomorphic add: Commit(m1,r1) * Commit(m2,r2) = Commit(m1+m2, r1+r2)
    pub fn add(&self, c1: u64, c2: u64) -> u64 {
        (c1 as u128 * c2 as u128 % self.p as u128) as u64
    }
    /// Batch commit: commit to a vector of (value, randomness) pairs.
    /// Returns a vector of commitments.
    pub fn batch_commit(&self, pairs: &[(u64, u64)]) -> Vec<u64> {
        pairs.iter().map(|&(m, r)| self.commit(m, r)).collect()
    }
}
/// Zero-knowledge proof system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZKProofSystem {
    pub name: String,
    pub is_interactive: bool,
    pub soundness_error: f64,
    pub completeness_error: f64,
    pub proof_size_bytes: Option<usize>,
}
#[allow(dead_code)]
impl ZKProofSystem {
    pub fn new(name: &str, interactive: bool) -> Self {
        ZKProofSystem {
            name: name.to_string(),
            is_interactive: interactive,
            soundness_error: 0.5,
            completeness_error: 0.0,
            proof_size_bytes: None,
        }
    }
    pub fn schnorr() -> Self {
        let mut s = ZKProofSystem::new("Schnorr", true);
        s.soundness_error = 1.0 / 2.0_f64.powi(128);
        s.proof_size_bytes = Some(64);
        s
    }
    pub fn groth16() -> Self {
        let mut s = ZKProofSystem::new("Groth16", false);
        s.soundness_error = 1.0 / 2.0_f64.powi(128);
        s.proof_size_bytes = Some(128);
        s
    }
    pub fn plonk() -> Self {
        let mut s = ZKProofSystem::new("PLONK", false);
        s.soundness_error = 1.0 / 2.0_f64.powi(128);
        s.proof_size_bytes = Some(640);
        s
    }
    pub fn is_non_interactive(&self) -> bool {
        !self.is_interactive
    }
    pub fn is_succinct(&self) -> bool {
        matches!(self.proof_size_bytes, Some(n) if n < 1000)
    }
}
/// Extended Shamir secret sharing with explicit reconstruction from arbitrary shares.
///
/// # WARNING
/// Educational only. Small prime field.
#[derive(Debug, Clone)]
pub struct ShamirSecretSharingExtended {
    /// Prime field modulus
    pub p: u64,
    /// Threshold t
    pub t: usize,
    /// Number of shares n
    pub n: usize,
}
impl ShamirSecretSharingExtended {
    /// Share the secret using the provided polynomial coefficients a[1..t].
    /// Returns n shares as (x_i, y_i) pairs.
    pub fn share(&self, secret: u64, coeffs: &[u64]) -> Vec<(u64, u64)> {
        assert_eq!(coeffs.len(), self.t - 1, "Need t-1 coefficients");
        (1..=self.n as u64)
            .map(|i| {
                let mut val: u128 = secret as u128 % self.p as u128;
                let mut ipow: u128 = i as u128;
                for &c in coeffs {
                    val = (val + c as u128 % self.p as u128 * ipow) % self.p as u128;
                    ipow = ipow * i as u128 % self.p as u128;
                }
                (i, val as u64)
            })
            .collect()
    }
    /// Reconstruct from any t shares using Lagrange interpolation mod p.
    pub fn reconstruct(&self, shares: &[(u64, u64)]) -> u64 {
        assert!(shares.len() >= self.t);
        let shares = &shares[..self.t];
        let p = self.p;
        let mut acc: i128 = 0;
        for (j, &(xj, yj)) in shares.iter().enumerate() {
            let mut num: i128 = 1;
            let mut den: i128 = 1;
            for (k, &(xk, _)) in shares.iter().enumerate() {
                if k == j {
                    continue;
                }
                num = num * (p as i128 - xk as i128) % p as i128;
                den = den * ((xj as i128 - xk as i128).rem_euclid(p as i128)) % p as i128;
            }
            let den_pos = den.rem_euclid(p as i128) as u64;
            let inv = mod_inv(den_pos, p).unwrap_or(0) as i128;
            let lagrange = num % p as i128 * inv % p as i128;
            acc = (acc + yj as i128 * lagrange % p as i128).rem_euclid(p as i128);
        }
        acc as u64
    }
}
/// A simple garbled AND/OR gate simulator.
///
/// Labels for 0 and 1 on each wire are represented as u64 tokens.
/// The garbled table encrypts the output label for each input pair.
///
/// # WARNING
/// Educational sketch only. Real garbling uses AES-based encryption.
#[derive(Debug, Clone)]
pub struct GarbledGate {
    /// Garbled table: indexed \[a_bit\]\[b_bit\] → output label
    pub table: [[u64; 2]; 2],
    /// Labels for wire A: \[label_0, label_1\]
    pub labels_a: [u64; 2],
    /// Labels for wire B: \[label_0, label_1\]
    pub labels_b: [u64; 2],
    /// Labels for output wire: \[label_0, label_1\]
    pub labels_out: [u64; 2],
}
impl GarbledGate {
    /// Build a garbled AND gate.
    /// labels_a, labels_b, labels_out: \[label_for_0, label_for_1\] on each wire.
    pub fn garble_and(labels_a: [u64; 2], labels_b: [u64; 2], labels_out: [u64; 2]) -> Self {
        let mut table = [[0u64; 2]; 2];
        for a in 0usize..2 {
            for b in 0usize..2 {
                let out_bit = a & b;
                table[a][b] = toy_encrypt(labels_a[a], labels_b[b], labels_out[out_bit]);
            }
        }
        GarbledGate {
            table,
            labels_a,
            labels_b,
            labels_out,
        }
    }
    /// Build a garbled OR gate.
    pub fn garble_or(labels_a: [u64; 2], labels_b: [u64; 2], labels_out: [u64; 2]) -> Self {
        let mut table = [[0u64; 2]; 2];
        for a in 0usize..2 {
            for b in 0usize..2 {
                let out_bit = a | b;
                table[a][b] = toy_encrypt(labels_a[a], labels_b[b], labels_out[out_bit]);
            }
        }
        GarbledGate {
            table,
            labels_a,
            labels_b,
            labels_out,
        }
    }
    /// Evaluate the garbled gate: given input labels, recover the output label.
    pub fn evaluate(&self, label_a: u64, label_b: u64) -> Option<u64> {
        for a in 0usize..2 {
            for b in 0usize..2 {
                if self.labels_a[a] == label_a && self.labels_b[b] == label_b {
                    let out = toy_encrypt(label_a, label_b, self.table[a][b]);
                    return Some(out);
                }
            }
        }
        None
    }
    /// Check if the recovered output label corresponds to output value 1.
    pub fn is_output_one(&self, output_label: u64) -> bool {
        output_label == self.labels_out[1]
    }
}
/// Pedersen commitment parameters: group order q, generators g and h.
///
/// Commit(m, r) = g^m * h^r mod p.
/// - Perfectly hiding
/// - Computationally binding (under DL assumption)
///
/// # WARNING
/// Educational toy with tiny parameters.
#[derive(Debug, Clone)]
pub struct PedersenParams {
    /// Prime modulus p
    pub p: u64,
    /// Group order q (prime, q | p-1)
    pub q: u64,
    /// First generator g
    pub g: u64,
    /// Second independent generator h (log_g(h) unknown to committer)
    pub h: u64,
}
impl PedersenParams {
    /// Create a commitment to value m with randomness r.
    pub fn commit(&self, m: u64, r: u64) -> u64 {
        let gm = mod_exp(self.g, m % self.q, self.p);
        let hr = mod_exp(self.h, r % self.q, self.p);
        (gm as u128 * hr as u128 % self.p as u128) as u64
    }
    /// Verify that commitment c opens to (m, r).
    pub fn verify(&self, c: u64, m: u64, r: u64) -> bool {
        self.commit(m, r) == c
    }
    /// Homomorphic addition: Commit(m1+m2, r1+r2) = Commit(m1,r1) * Commit(m2,r2) mod p.
    pub fn add_commitments(&self, c1: u64, c2: u64) -> u64 {
        (c1 as u128 * c2 as u128 % self.p as u128) as u64
    }
}
/// A toy 2-party XOR secret-sharing based MPC for boolean functions.
///
/// Each party holds a share; shares XOR to the actual value.
/// Only XOR gates can be computed locally; AND requires interaction.
///
/// # WARNING
/// Educational GMW-style sketch. Not secure or complete.
#[derive(Debug, Clone)]
pub struct MpcShare {
    /// Party index (0 or 1)
    pub party: u8,
    /// Boolean share of some wire value
    pub share: bool,
}
impl MpcShare {
    /// XOR gate: locally XOR the two shares.
    pub fn xor_gate(a: &MpcShare, b: &MpcShare) -> MpcShare {
        assert_eq!(a.party, b.party);
        MpcShare {
            party: a.party,
            share: a.share ^ b.share,
        }
    }
    /// Reconstruct wire value from two parties' shares.
    pub fn reconstruct(s0: &MpcShare, s1: &MpcShare) -> bool {
        s0.share ^ s1.share
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OTVariant {
    OneOutOfTwo,
    OneOutOfN(usize),
    RandomOT,
}
/// Commitment scheme (hiding and binding).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommitmentScheme {
    pub name: String,
    pub is_perfectly_hiding: bool,
    pub is_computationally_binding: bool,
    pub is_homomorphic: bool,
}
#[allow(dead_code)]
impl CommitmentScheme {
    pub fn new(name: &str) -> Self {
        CommitmentScheme {
            name: name.to_string(),
            is_perfectly_hiding: false,
            is_computationally_binding: false,
            is_homomorphic: false,
        }
    }
    pub fn pedersen() -> Self {
        CommitmentScheme {
            name: "Pedersen".to_string(),
            is_perfectly_hiding: true,
            is_computationally_binding: true,
            is_homomorphic: true,
        }
    }
    pub fn sha256_hash() -> Self {
        CommitmentScheme {
            name: "SHA256-hash".to_string(),
            is_perfectly_hiding: false,
            is_computationally_binding: true,
            is_homomorphic: false,
        }
    }
    pub fn satisfies_binding_hiding_tradeoff(&self) -> bool {
        !self.is_perfectly_hiding || !self.is_computationally_binding
    }
}
/// Multiparty computation (MPC) protocol.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MPCProtocol {
    pub name: String,
    pub n_parties: usize,
    pub threshold_corruption: usize,
    pub security_model: MPCSecurityModel,
}
#[allow(dead_code)]
impl MPCProtocol {
    pub fn new(name: &str, n: usize, t: usize, model: MPCSecurityModel) -> Self {
        MPCProtocol {
            name: name.to_string(),
            n_parties: n,
            threshold_corruption: t,
            security_model: model,
        }
    }
    pub fn bgw(n: usize, t: usize) -> Self {
        MPCProtocol::new("BGW", n, t, MPCSecurityModel::Malicious)
    }
    pub fn is_secure_against_majority_corruption(&self) -> bool {
        self.threshold_corruption * 2 < self.n_parties
    }
    pub fn is_optimal_corruption_threshold(&self) -> bool {
        match self.security_model {
            MPCSecurityModel::Malicious => self.threshold_corruption * 3 < self.n_parties,
            MPCSecurityModel::SemiHonest => self.threshold_corruption * 2 < self.n_parties,
            MPCSecurityModel::Covert => self.threshold_corruption * 2 < self.n_parties,
        }
    }
}
/// Secret sharing scheme (Shamir's).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SecretSharing {
    pub threshold: usize,
    pub n_shares: usize,
    pub field_size_bits: usize,
}
#[allow(dead_code)]
impl SecretSharing {
    pub fn new(t: usize, n: usize, field_bits: usize) -> Self {
        assert!(t <= n);
        SecretSharing {
            threshold: t,
            n_shares: n,
            field_size_bits: field_bits,
        }
    }
    pub fn shamir_2_of_3() -> Self {
        SecretSharing::new(2, 3, 256)
    }
    pub fn is_perfect(&self) -> bool {
        true
    }
    pub fn min_shares_needed(&self) -> usize {
        self.threshold
    }
    pub fn share_size_bits(&self) -> usize {
        self.field_size_bits
    }
}
/// A Schnorr proof transcript (commitment, challenge, response).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchnorrTranscript {
    /// Commitment: a = g^r mod p
    pub commitment: u64,
    /// Challenge: e (random element in Z_q)
    pub challenge: u64,
    /// Response: z = r + e*x mod q
    pub response: u64,
}
/// Toy Paillier encryption over Z_n^2 with homomorphic addition.
///
/// For small n = p*q; the real scheme requires RSA-sized moduli (2048+ bits).
///
/// # WARNING
/// Educational only. The small parameters make this completely insecure.
#[derive(Debug, Clone)]
pub struct PaillierHomomorphic {
    /// n = p*q (RSA modulus; tiny here for education)
    pub n: u64,
    /// n^2 = n*n
    pub n_sq: u128,
    /// Public generator g (usually g = n+1)
    pub g: u64,
    /// λ = lcm(p-1, q-1) (private key component)
    pub lambda: u64,
    /// μ = (L(g^λ mod n^2))^{-1} mod n where L(x) = (x-1)/n
    pub mu: u64,
}
impl PaillierHomomorphic {
    /// Encrypt plaintext m ∈ [0, n) with randomness r (gcd(r,n)=1).
    /// Enc(m, r) = g^m * r^n mod n^2
    pub fn encrypt(&self, m: u64, r: u64) -> u128 {
        let n_sq = self.n_sq;
        let gm = {
            let base = (self.g as u128).pow(1) % n_sq;
            let mut result: u128 = 1;
            let mut exp = m;
            let mut b = self.g as u128 % n_sq;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * b % n_sq;
                }
                exp >>= 1;
                b = b * b % n_sq;
            }
            let _ = base;
            result
        };
        let rn = {
            let mut result: u128 = 1;
            let mut exp = self.n;
            let mut b = r as u128 % n_sq;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * b % n_sq;
                }
                exp >>= 1;
                b = b * b % n_sq;
            }
            result
        };
        gm * rn % n_sq
    }
    /// Homomorphic addition: Enc(m1) * Enc(m2) mod n^2 = Enc(m1+m2).
    pub fn add_ciphertexts(&self, c1: u128, c2: u128) -> u128 {
        c1 * c2 % self.n_sq
    }
    /// L function: L(x) = (x - 1) / n
    fn l_func(&self, x: u128) -> u64 {
        ((x - 1) / self.n as u128) as u64
    }
    /// Decrypt ciphertext c: m = L(c^λ mod n^2) * μ mod n
    pub fn decrypt(&self, c: u128) -> u64 {
        let n_sq = self.n_sq;
        let mut result: u128 = 1;
        let mut exp = self.lambda;
        let mut b = c % n_sq;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * b % n_sq;
            }
            exp >>= 1;
            b = b * b % n_sq;
        }
        let lval = self.l_func(result);
        (lval as u128 * self.mu as u128 % self.n as u128) as u64
    }
}
/// Simplified Chaum blind signature protocol over Z_p*.
///
/// Protocol:
/// 1. Signer has key (d, e, n): d=private, e=public, n=modulus (RSA-like but tiny)
/// 2. User blinds message m: c = r^e * m mod n (r is blinding factor)
/// 3. Signer signs blinded message: s' = c^d mod n
/// 4. User unblinds: s = s' * r^{-1} mod n
/// 5. Verify: s^e = m mod n
///
/// # WARNING
/// Educational only. Real blind RSA requires SHA-based full-domain hash + PKCS1v2.1.
#[derive(Debug, Clone)]
pub struct BlindSignatureScheme {
    /// RSA-like modulus n = p*q (tiny, insecure)
    pub n: u64,
    /// Public exponent e
    pub e: u64,
    /// Private exponent d (e*d ≡ 1 mod λ(n))
    pub d: u64,
}
impl BlindSignatureScheme {
    /// User blinds message m with factor r: returns blinded = r^e * m mod n.
    pub fn blind(&self, m: u64, r: u64) -> u64 {
        let re = mod_exp(r, self.e, self.n);
        (re as u128 * m as u128 % self.n as u128) as u64
    }
    /// Signer signs blinded message: s_prime = blinded^d mod n.
    pub fn sign_blinded(&self, blinded: u64) -> u64 {
        mod_exp(blinded, self.d, self.n)
    }
    /// User unblinds: s = s_prime * r^{-1} mod n.
    pub fn unblind(&self, s_prime: u64, r: u64) -> u64 {
        let r_inv = mod_inv(r, self.n).unwrap_or(1);
        (s_prime as u128 * r_inv as u128 % self.n as u128) as u64
    }
    /// Verify signature: check s^e ≡ m (mod n).
    pub fn verify(&self, m: u64, s: u64) -> bool {
        mod_exp(s, self.e, self.n) == m % self.n
    }
}
