//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// CRYSTALS-Dilithium signature scheme data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DilithiumParams {
    pub security_level: usize,
    pub q: u64,
    pub n: usize,
    pub k: usize,
    pub l: usize,
    pub eta: usize,
}
#[allow(dead_code)]
impl DilithiumParams {
    /// Dilithium2 parameters.
    pub fn dilithium2() -> Self {
        Self {
            security_level: 2,
            q: 8_380_417,
            n: 256,
            k: 4,
            l: 4,
            eta: 2,
        }
    }
    /// Dilithium3 parameters.
    pub fn dilithium3() -> Self {
        Self {
            security_level: 3,
            q: 8_380_417,
            n: 256,
            k: 6,
            l: 5,
            eta: 4,
        }
    }
    /// Public key size in bytes (approximate).
    pub fn public_key_size(&self) -> usize {
        32 + self.k * self.n * 10 / 8
    }
    /// Signature size in bytes (approximate).
    pub fn signature_size(&self) -> usize {
        self.l * self.n * 5 / 8 + 80
    }
}
/// Arithmetic utilities for polynomial rings used in lattice cryptography.
///
/// Implements NTT-free modular polynomial operations for toy demonstrations.
///
/// # WARNING: Educational only. No NTT, no constant-time guarantees.
#[derive(Debug, Clone)]
pub struct ModularLatticeArithmetic {
    /// Polynomial degree n (ring Z_q\[x\]/(x^n + 1)).
    pub n: usize,
    /// Modulus q.
    pub q: i64,
}
impl ModularLatticeArithmetic {
    /// Create a new modular lattice arithmetic context.
    pub fn new(n: usize, q: i64) -> Self {
        Self { n, q }
    }
    /// Reduce a coefficient mod q to the range [0, q).
    pub fn reduce(&self, x: i64) -> i64 {
        x.rem_euclid(self.q)
    }
    /// Center-reduce a coefficient to (-q/2, q/2].
    pub fn center_reduce(&self, x: i64) -> i64 {
        let r = x.rem_euclid(self.q);
        if r > self.q / 2 {
            r - self.q
        } else {
            r
        }
    }
    /// Polynomial addition mod (x^n + 1, q).
    pub fn poly_add(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        a.iter()
            .zip(b)
            .map(|(ai, bi)| self.reduce(ai + bi))
            .collect()
    }
    /// Polynomial subtraction mod (x^n + 1, q).
    pub fn poly_sub(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        a.iter()
            .zip(b)
            .map(|(ai, bi)| self.reduce(ai - bi))
            .collect()
    }
    /// Polynomial multiplication mod (x^n + 1, q) using schoolbook O(n^2).
    pub fn poly_mul(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        let mut out = vec![0i64; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                let k = i + j;
                if k < self.n {
                    out[k] = self.reduce(out[k] + a[i] * b[j]);
                } else {
                    out[k - self.n] = self.reduce(out[k - self.n] - a[i] * b[j]);
                }
            }
        }
        out
    }
    /// Scalar multiplication: multiply polynomial by a scalar mod q.
    pub fn scalar_mul(&self, a: &[i64], scalar: i64) -> Vec<i64> {
        a.iter().map(|&ai| self.reduce(ai * scalar)).collect()
    }
    /// L_inf norm of a polynomial (max |coefficient| in centered representation).
    pub fn inf_norm(&self, a: &[i64]) -> i64 {
        a.iter()
            .map(|&ai| self.center_reduce(ai).abs())
            .max()
            .unwrap_or(0)
    }
    /// L_2 norm squared of a polynomial (sum of squares of coefficients).
    pub fn l2_norm_sq(&self, a: &[i64]) -> i64 {
        a.iter()
            .map(|&ai| {
                let c = self.center_reduce(ai);
                c * c
            })
            .sum()
    }
    /// Check if a polynomial is "small": all centered coefficients in \[-eta, eta\].
    pub fn is_small(&self, a: &[i64], eta: i64) -> bool {
        a.iter().all(|&ai| self.center_reduce(ai).abs() <= eta)
    }
}
/// A Lamport key pair: secret keys and public (hashed) keys.
#[derive(Debug, Clone)]
pub struct LamportKeyPair {
    /// Secret keys: sk\[i\]\[b\] = pre-image for bit i value b
    pub sk: Vec<[u64; 2]>,
    /// Public keys: pk\[i\]\[b\] = hash(sk\[i\]\[b\])
    pub pk: Vec<[u64; 2]>,
}
/// Toy WOTS+ (Winternitz One-Time Signature Plus) implementation.
///
/// Uses Winternitz parameter w=16 and hash output of 64 bits (toy only).
///
/// # WARNING: Educational only. n=64 bits has no real security.
#[derive(Debug, Clone)]
pub struct HashBasedSignature {
    /// Winternitz parameter w (chains of length w-1).
    pub w: usize,
    /// Number of hash chains (key/signature elements).
    pub chains: usize,
}
impl HashBasedSignature {
    /// Create a new WOTS+ instance with the given parameters.
    pub fn new(w: usize, chains: usize) -> Self {
        Self { w, chains }
    }
    /// Default WOTS+ with w=16, chains=32.
    pub fn default_params() -> Self {
        Self::new(16, 32)
    }
    /// Simple toy hash function (FNV-like).
    fn hash_once(&self, x: u64, idx: u64) -> u64 {
        x.wrapping_mul(0x100000001b3)
            .wrapping_add(idx)
            .rotate_left(13)
            .wrapping_add(0xcbf29ce484222325)
    }
    /// Apply the hash function k times (a chain of length k).
    fn chain(&self, mut x: u64, start: usize, steps: usize, idx: u64) -> u64 {
        for i in start..(start + steps) {
            x = self.hash_once(x, idx.wrapping_add(i as u64));
        }
        x
    }
    /// Generate a WOTS+ key pair from a seed.
    pub fn keygen(&self, seed: u64) -> WOTSKeyPair {
        let sk: Vec<u64> = (0..self.chains)
            .map(|i| self.hash_once(seed, i as u64))
            .collect();
        let pk: Vec<u64> = sk
            .iter()
            .enumerate()
            .map(|(i, &s)| self.chain(s, 0, self.w - 1, i as u64))
            .collect();
        WOTSKeyPair { sk, pk }
    }
    /// Sign a message hash m (encoded as chain lengths).
    /// Each element of m must be in \[0, w-1\].
    pub fn sign(&self, kp: &WOTSKeyPair, m: &[usize]) -> Vec<u64> {
        assert_eq!(m.len(), self.chains);
        m.iter()
            .enumerate()
            .map(|(i, &mi)| {
                assert!(mi < self.w, "message element out of range");
                self.chain(kp.sk[i], 0, mi, i as u64)
            })
            .collect()
    }
    /// Verify a WOTS+ signature.
    pub fn verify(&self, pk: &WOTSKeyPair, m: &[usize], sig: &[u64]) -> bool {
        assert_eq!(m.len(), self.chains);
        assert_eq!(sig.len(), self.chains);
        sig.iter().enumerate().all(|(i, &s)| {
            let remaining = self.w - 1 - m[i];
            self.chain(s, m[i], remaining, i as u64) == pk.pk[i]
        })
    }
}
/// Toy Lamport one-time signature scheme with n=8 (8-bit hash for illustration).
///
/// WARNING: n=8 has no security whatsoever. Real Lamport uses n=256+.
#[derive(Debug, Clone)]
pub struct ToyLamport {
    /// Number of hash bits (toy: 8)
    pub n: usize,
}
impl ToyLamport {
    /// Simple toy hash: polynomial hash for illustration.
    fn hash(&self, x: u64) -> u64 {
        const M: u64 = (1 << 31) - 1;
        x.wrapping_mul(0x9e3779b97f4a7c15)
            .wrapping_add(0x6c62272e07bb0142)
            % M
    }
    /// Generate a Lamport key pair from a seed (deterministic for tests).
    pub fn keygen(&self, seed: u64) -> LamportKeyPair {
        let mut sk = Vec::with_capacity(self.n);
        let mut pk = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let s0 = self.hash(seed.wrapping_add(2 * i as u64));
            let s1 = self.hash(seed.wrapping_add(2 * i as u64 + 1));
            sk.push([s0, s1]);
            pk.push([self.hash(s0), self.hash(s1)]);
        }
        LamportKeyPair { sk, pk }
    }
    /// Sign a message (must fit in n bits).
    pub fn sign(&self, kp: &LamportKeyPair, msg_bits: &[u8]) -> Vec<u64> {
        assert_eq!(msg_bits.len(), self.n);
        msg_bits
            .iter()
            .enumerate()
            .map(|(i, &b)| kp.sk[i][b as usize])
            .collect()
    }
    /// Verify a Lamport signature.
    pub fn verify(&self, pk: &LamportKeyPair, msg_bits: &[u8], sig: &[u64]) -> bool {
        assert_eq!(msg_bits.len(), self.n);
        assert_eq!(sig.len(), self.n);
        sig.iter()
            .enumerate()
            .all(|(i, &s)| self.hash(s) == pk.pk[i][msg_bits[i] as usize])
    }
}
/// Toy LWE encryption parameters.
///
/// n-dimensional LWE over Z_q. Secret s ∈ {0,1}^n (binary secret variant).
///
/// # WARNING
/// n=8, q=97 has no security. Real LWE uses n≥512, q≥2^12.
#[derive(Debug, Clone)]
pub struct ToyLWE {
    /// Dimension n
    pub n: usize,
    /// Modulus q
    pub q: i64,
    /// Error bound B: errors are in \[-B, B\]
    pub b: i64,
}
impl ToyLWE {
    /// Encrypt a single bit `m ∈ {0,1}` using public key (A rows, b vector).
    ///
    /// c = (sum of random subset of A rows, sum of b_i + m*floor(q/2))
    ///
    /// `mask`: a boolean vector of length n selecting which rows to sum.
    pub fn encrypt(
        &self,
        a_rows: &[Vec<i64>],
        b_vec: &[i64],
        m: u8,
        mask: &[bool],
    ) -> (Vec<i64>, i64) {
        let c1: Vec<i64> = (0..self.n)
            .map(|j| {
                mask.iter()
                    .enumerate()
                    .filter(|(_, &sel)| sel)
                    .map(|(i, _)| a_rows[i][j])
                    .sum::<i64>()
                    .rem_euclid(self.q)
            })
            .collect();
        let c2: i64 = (mask
            .iter()
            .enumerate()
            .filter(|(_, &sel)| sel)
            .map(|(i, _)| b_vec[i])
            .sum::<i64>()
            + m as i64 * (self.q / 2))
            .rem_euclid(self.q);
        (c1, c2)
    }
    /// Decrypt: compute c2 - ⟨c1, s⟩ mod q, then round to 0 or 1.
    pub fn decrypt(&self, c1: &[i64], c2: i64, s: &[i64]) -> u8 {
        let inner: i64 = c1
            .iter()
            .zip(s)
            .map(|(a, b)| a * b)
            .sum::<i64>()
            .rem_euclid(self.q);
        let mut diff = (c2 - inner).rem_euclid(self.q);
        if diff > self.q / 2 {
            diff -= self.q;
        }
        if diff.abs() > self.q / 4 {
            1
        } else {
            0
        }
    }
}
/// Toy lattice basis reducer implementing a simplified 2D version of LLL.
///
/// Only handles 2-dimensional lattices for illustration.
/// Real BKZ/LLL operates in arbitrary dimensions.
///
/// # WARNING: Educational only.
#[derive(Debug, Clone)]
pub struct LatticeBasisReducer {
    /// LLL delta parameter (0.5 < delta < 1); typically 0.75.
    pub delta: f64,
}
impl LatticeBasisReducer {
    /// Create a new basis reducer with the given LLL delta.
    pub fn new(delta: f64) -> Self {
        assert!(
            delta > 0.5 && delta < 1.0,
            "LLL delta must be in (0.5, 1.0)"
        );
        Self { delta }
    }
    /// Reduce a 2D basis given as [\[b00, b01\], \[b10, b11\]].
    /// Returns the reduced basis (Gauss reduction for dimension 2).
    pub fn reduce_2d(&self, b0: [i64; 2], b1: [i64; 2]) -> ([i64; 2], [i64; 2]) {
        let mut v0 = b0;
        let mut v1 = b1;
        for _ in 0..100 {
            let dot01 = v0[0] * v1[0] + v0[1] * v1[1];
            let dot00 = v0[0] * v0[0] + v0[1] * v0[1];
            if dot00 == 0 {
                break;
            }
            let mu = (dot01 as f64 / dot00 as f64).round() as i64;
            if mu == 0 {
                break;
            }
            v1[0] -= mu * v0[0];
            v1[1] -= mu * v0[1];
            let norm0_sq = v0[0] * v0[0] + v0[1] * v0[1];
            let norm1_sq = v1[0] * v1[0] + v1[1] * v1[1];
            if (norm1_sq as f64) < self.delta * (norm0_sq as f64) {
                std::mem::swap(&mut v0, &mut v1);
            }
        }
        (v0, v1)
    }
    /// Compute the squared length of a 2D vector.
    pub fn norm_sq(v: [i64; 2]) -> i64 {
        v[0] * v[0] + v[1] * v[1]
    }
    /// Check whether a 2D basis is LLL-reduced (simplified).
    pub fn is_lll_reduced_2d(&self, b0: [i64; 2], b1: [i64; 2]) -> bool {
        let dot01 = b0[0] * b1[0] + b0[1] * b1[1];
        let dot00 = b0[0] * b0[0] + b0[1] * b0[1];
        let mu = if dot00 == 0 {
            0.0
        } else {
            dot01 as f64 / dot00 as f64
        };
        let norm0_sq = Self::norm_sq(b0) as f64;
        let norm1_sq = Self::norm_sq(b1) as f64;
        mu.abs() <= 0.5 && norm1_sq >= (self.delta - mu * mu) * norm0_sq
    }
}
/// Falcon parameter variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FalconVariant {
    /// Falcon-512: n=512, NIST security level 1
    Falcon512,
    /// Falcon-1024: n=1024, NIST security level 5
    Falcon1024,
}
impl FalconVariant {
    /// NTRU polynomial degree n.
    pub fn degree(&self) -> usize {
        match self {
            Self::Falcon512 => 512,
            Self::Falcon1024 => 1024,
        }
    }
    /// Signature size in bytes (approximate).
    pub fn signature_bytes(&self) -> usize {
        match self {
            Self::Falcon512 => 666,
            Self::Falcon1024 => 1280,
        }
    }
    /// Modulus q = 12289 for all Falcon variants.
    pub fn modulus(&self) -> u64 {
        12289
    }
}
/// Isogeny-based cryptography (SIKE/CSIDH-type).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IsogenyCrypto {
    pub scheme: String,
    pub prime: String,
    pub security_bits: usize,
    pub is_post_sike: bool,
}
#[allow(dead_code)]
impl IsogenyCrypto {
    /// CSIDH-512 (commutative SIDH).
    pub fn csidh_512() -> Self {
        Self {
            scheme: "CSIDH-512".to_string(),
            prime: "p = 2 * prod(small primes) * l_1 * ... * l_n - 1".to_string(),
            security_bits: 64,
            is_post_sike: true,
        }
    }
    /// Note: SIKE was broken in 2022 by Castryck-Decru; this represents post-SIKE alternatives.
    pub fn note() -> &'static str {
        "SIKE broken in 2022; CSIDH remains under study"
    }
}
/// Dilithium parameter variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DilithiumVariant {
    /// Dilithium2: (k=4, l=4), NIST security level 2
    Dilithium2,
    /// Dilithium3: (k=6, l=5), NIST security level 3
    Dilithium3,
    /// Dilithium5: (k=8, l=7), NIST security level 5
    Dilithium5,
}
impl DilithiumVariant {
    /// Matrix dimension (k, l).
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Self::Dilithium2 => (4, 4),
            Self::Dilithium3 => (6, 5),
            Self::Dilithium5 => (8, 7),
        }
    }
    /// Signature size in bytes.
    pub fn signature_bytes(&self) -> usize {
        match self {
            Self::Dilithium2 => 2420,
            Self::Dilithium3 => 3293,
            Self::Dilithium5 => 4595,
        }
    }
    /// Public key size in bytes.
    pub fn public_key_bytes(&self) -> usize {
        match self {
            Self::Dilithium2 => 1312,
            Self::Dilithium3 => 1952,
            Self::Dilithium5 => 2592,
        }
    }
}
/// A toy Merkle signature tree of depth 2 (4 leaves = 4 one-time keys).
#[derive(Debug, Clone)]
pub struct ToyMerkleTree {
    /// Leaf public keys (hashed with toy hash)
    pub leaves: Vec<u64>,
    /// Internal nodes: inner[0..2] = parents of leaf pairs; inner\[2\] = root
    pub inner: Vec<u64>,
}
impl ToyMerkleTree {
    /// Build a Merkle tree from 4 leaf values.
    pub fn build(leaves: [u64; 4]) -> Self {
        let inner = vec![
            merkle_hash(leaves[0], leaves[1]),
            merkle_hash(leaves[2], leaves[3]),
            merkle_hash(
                merkle_hash(leaves[0], leaves[1]),
                merkle_hash(leaves[2], leaves[3]),
            ),
        ];
        ToyMerkleTree {
            leaves: leaves.to_vec(),
            inner,
        }
    }
    /// Root hash (public key).
    pub fn root(&self) -> u64 {
        self.inner[2]
    }
    /// Authentication path for leaf index i (0..4).
    /// Returns (sibling_leaf, parent_sibling).
    pub fn auth_path(&self, i: usize) -> (u64, u64) {
        match i {
            0 => (self.leaves[1], self.inner[1]),
            1 => (self.leaves[0], self.inner[1]),
            2 => (self.leaves[3], self.inner[0]),
            3 => (self.leaves[2], self.inner[0]),
            _ => panic!("leaf index out of range"),
        }
    }
    /// Verify a leaf against the root using its authentication path.
    pub fn verify_leaf(&self, leaf: u64, i: usize, path: (u64, u64)) -> bool {
        let (sib, parent_sib) = path;
        let parent = if i % 2 == 0 {
            merkle_hash(leaf, sib)
        } else {
            merkle_hash(sib, leaf)
        };
        let root = if i / 2 == 0 {
            merkle_hash(parent, parent_sib)
        } else {
            merkle_hash(parent_sib, parent)
        };
        root == self.root()
    }
}
/// A WOTS+ key pair.
#[derive(Debug, Clone)]
pub struct WOTSKeyPair {
    /// Secret key: one random value per chain.
    pub sk: Vec<u64>,
    /// Public key: hash^{w-1}(sk\[i\]) for each chain i.
    pub pk: Vec<u64>,
}
/// FALCON signature scheme data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FalconParams {
    pub n: usize,
    pub security_level: usize,
    pub signature_size: usize,
    pub public_key_size: usize,
}
#[allow(dead_code)]
impl FalconParams {
    /// Falcon-512.
    pub fn falcon512() -> Self {
        Self {
            n: 512,
            security_level: 1,
            signature_size: 666,
            public_key_size: 897,
        }
    }
    /// Falcon-1024.
    pub fn falcon1024() -> Self {
        Self {
            n: 1024,
            security_level: 5,
            signature_size: 1280,
            public_key_size: 1793,
        }
    }
    /// Falcon is based on NTRU lattices with Gaussian sampling.
    pub fn description(&self) -> String {
        format!(
            "Falcon-{}: security level {}, sig_size={} bytes",
            self.n, self.security_level, self.signature_size
        )
    }
}
/// A polynomial in Z_q\[x\] / (x^n + 1), stored as coefficient vector.
///
/// Used for toy implementations of RLWE / Kyber / Dilithium.
///
/// # WARNING
/// Educational only. Production NTT-based implementations are required
/// for any real use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RingPoly {
    /// Coefficients mod q, length n.
    pub coeffs: Vec<i64>,
    /// Modulus q
    pub q: i64,
}
impl RingPoly {
    /// Create a zero polynomial of degree n over Z_q.
    pub fn zero(n: usize, q: i64) -> Self {
        RingPoly {
            coeffs: vec![0; n],
            q,
        }
    }
    /// Reduce all coefficients mod q into (-q/2, q/2].
    pub fn reduce(&mut self) {
        for c in &mut self.coeffs {
            *c = c.rem_euclid(self.q);
            if *c > self.q / 2 {
                *c -= self.q;
            }
        }
    }
    /// Add two polynomials in the ring (coefficient-wise mod q).
    pub fn add(&self, other: &RingPoly) -> RingPoly {
        assert_eq!(self.coeffs.len(), other.coeffs.len());
        assert_eq!(self.q, other.q);
        let mut res = RingPoly {
            coeffs: self
                .coeffs
                .iter()
                .zip(&other.coeffs)
                .map(|(a, b)| (a + b).rem_euclid(self.q))
                .collect(),
            q: self.q,
        };
        res.reduce();
        res
    }
    /// Subtract two polynomials in the ring.
    pub fn sub(&self, other: &RingPoly) -> RingPoly {
        assert_eq!(self.coeffs.len(), other.coeffs.len());
        assert_eq!(self.q, other.q);
        let mut res = RingPoly {
            coeffs: self
                .coeffs
                .iter()
                .zip(&other.coeffs)
                .map(|(a, b)| (a - b).rem_euclid(self.q))
                .collect(),
            q: self.q,
        };
        res.reduce();
        res
    }
    /// Multiply two polynomials mod (x^n + 1) and q.
    ///
    /// Uses naive O(n^2) schoolbook multiplication. Production code uses NTT.
    pub fn mul(&self, other: &RingPoly) -> RingPoly {
        let n = self.coeffs.len();
        assert_eq!(n, other.coeffs.len());
        assert_eq!(self.q, other.q);
        let mut out = vec![0i64; n];
        for i in 0..n {
            for j in 0..n {
                let k = i + j;
                if k < n {
                    out[k] = (out[k] + self.coeffs[i] * other.coeffs[j]).rem_euclid(self.q);
                } else {
                    out[k - n] = (out[k - n] - self.coeffs[i] * other.coeffs[j]).rem_euclid(self.q);
                }
            }
        }
        let mut res = RingPoly {
            coeffs: out,
            q: self.q,
        };
        res.reduce();
        res
    }
    /// Infinity norm: max |coefficient|.
    pub fn inf_norm(&self) -> i64 {
        self.coeffs.iter().map(|&c| c.abs()).max().unwrap_or(0)
    }
}
/// Kyber security parameter sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KyberVariant {
    /// Kyber-512: k=2, ~128-bit security
    Kyber512,
    /// Kyber-768: k=3, ~192-bit security
    Kyber768,
    /// Kyber-1024: k=4, ~256-bit security
    Kyber1024,
}
impl KyberVariant {
    /// Module dimension k.
    pub fn k(&self) -> usize {
        match self {
            Self::Kyber512 => 2,
            Self::Kyber768 => 3,
            Self::Kyber1024 => 4,
        }
    }
    /// Ciphertext size in bytes.
    pub fn ciphertext_bytes(&self) -> usize {
        match self {
            Self::Kyber512 => 768,
            Self::Kyber768 => 1088,
            Self::Kyber1024 => 1568,
        }
    }
    /// Public key size in bytes.
    pub fn public_key_bytes(&self) -> usize {
        match self {
            Self::Kyber512 => 800,
            Self::Kyber768 => 1184,
            Self::Kyber1024 => 1568,
        }
    }
    /// Classical security bit estimate.
    pub fn security_bits(&self) -> u32 {
        match self {
            Self::Kyber512 => 128,
            Self::Kyber768 => 192,
            Self::Kyber1024 => 256,
        }
    }
}
/// Generator for toy LWE samples (A, b = As + e).
///
/// Generates rows of the LWE matrix and corresponding b values for
/// educational purposes. Uses a simple linear congruential generator
/// in place of a cryptographic RNG.
///
/// # WARNING: Educational only.
#[derive(Debug, Clone)]
pub struct LWESampleGenerator {
    /// Dimension n.
    pub n: usize,
    /// Modulus q.
    pub q: i64,
    /// Error bound b (errors uniform in \[-b, b\]).
    pub b: i64,
    /// PRNG state.
    state: u64,
}
impl LWESampleGenerator {
    /// Create a new LWE sample generator with the given parameters.
    pub fn new(n: usize, q: i64, b: i64, seed: u64) -> Self {
        Self {
            n,
            q,
            b,
            state: seed,
        }
    }
    /// Advance the LCG state and return the next pseudo-random u64.
    fn next_rand(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    /// Sample a uniform element of Z_q.
    fn sample_uniform(&mut self) -> i64 {
        (self.next_rand() as i64).rem_euclid(self.q)
    }
    /// Sample a small error from \[-b, b\].
    pub fn sample_error(&mut self) -> i64 {
        let raw = (self.next_rand() as i64).rem_euclid(2 * self.b + 1);
        raw - self.b
    }
    /// Generate a random LWE matrix row a ∈ Z_q^n.
    pub fn sample_a(&mut self) -> Vec<i64> {
        (0..self.n).map(|_| self.sample_uniform()).collect()
    }
    /// Generate a LWE sample (a, b = ⟨a, s⟩ + e) given the secret s.
    pub fn sample(&mut self, s: &[i64]) -> (Vec<i64>, i64) {
        assert_eq!(s.len(), self.n);
        let a = self.sample_a();
        let inner: i64 = a
            .iter()
            .zip(s)
            .map(|(ai, si)| ai * si)
            .sum::<i64>()
            .rem_euclid(self.q);
        let e = self.sample_error();
        let bval = (inner + e).rem_euclid(self.q);
        (a, bval)
    }
    /// Generate m LWE samples.
    pub fn sample_many(&mut self, s: &[i64], m: usize) -> Vec<(Vec<i64>, i64)> {
        (0..m).map(|_| self.sample(s)).collect()
    }
}
/// Code-based cryptography (McEliece).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct McElieceParams {
    pub n: usize,
    pub k: usize,
    pub t: usize,
    pub public_key_size: usize,
}
#[allow(dead_code)]
impl McElieceParams {
    /// Classic McEliece 348864 parameters.
    pub fn kem_348864() -> Self {
        Self {
            n: 3488,
            k: 2720,
            t: 64,
            public_key_size: 261_120,
        }
    }
    /// Rate of the code.
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    /// Error correction capability.
    pub fn error_capability(&self) -> usize {
        self.t
    }
}
/// SPHINCS+ hash-based signature data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SphincsParams {
    pub variant: String,
    pub n: usize,
    pub h: usize,
    pub d: usize,
    pub sig_size: usize,
}
#[allow(dead_code)]
impl SphincsParams {
    /// SPHINCS+-SHA2-128s.
    pub fn sha2_128s() -> Self {
        Self {
            variant: "SPHINCS+-SHA2-128s".to_string(),
            n: 16,
            h: 63,
            d: 7,
            sig_size: 7_856,
        }
    }
    /// SPHINCS+-SHA2-256f (fast).
    pub fn sha2_256f() -> Self {
        Self {
            variant: "SPHINCS+-SHA2-256f".to_string(),
            n: 32,
            h: 68,
            d: 17,
            sig_size: 49_856,
        }
    }
    /// Stateless — no state to maintain between signatures.
    pub fn is_stateless(&self) -> bool {
        true
    }
}
