//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Toy Diffie-Hellman key exchange simulation.
///
/// Encapsulates a full simulated key exchange between Alice and Bob.
///
/// # WARNING
/// Uses tiny parameters with no real security.
pub struct DiffieHellmanSim {
    /// Shared prime p
    pub p: u64,
    /// Generator g
    pub g: u64,
    /// Alice's private key
    pub alice_priv: u64,
    /// Bob's private key
    pub bob_priv: u64,
}
impl DiffieHellmanSim {
    /// Create a simulation with given parameters.
    pub fn new(p: u64, g: u64, alice_priv: u64, bob_priv: u64) -> Self {
        DiffieHellmanSim {
            p,
            g,
            alice_priv,
            bob_priv,
        }
    }
    /// Alice's public key: g^a mod p.
    pub fn alice_public(&self) -> u64 {
        mod_exp(self.g, self.alice_priv, self.p)
    }
    /// Bob's public key: g^b mod p.
    pub fn bob_public(&self) -> u64 {
        mod_exp(self.g, self.bob_priv, self.p)
    }
    /// Shared secret computed by Alice: (g^b)^a mod p.
    pub fn alice_shared_secret(&self) -> u64 {
        mod_exp(self.bob_public(), self.alice_priv, self.p)
    }
    /// Shared secret computed by Bob: (g^a)^b mod p.
    pub fn bob_shared_secret(&self) -> u64 {
        mod_exp(self.alice_public(), self.bob_priv, self.p)
    }
    /// Verify that Alice and Bob derive the same shared secret.
    pub fn secrets_match(&self) -> bool {
        self.alice_shared_secret() == self.bob_shared_secret()
    }
}
/// Modular arithmetic helpers with support for common field operations.
///
/// # WARNING
/// Educational implementation only.
pub struct ModularArithmetic {
    /// The modulus
    pub modulus: u64,
}
impl ModularArithmetic {
    /// Create a new modular arithmetic context with the given modulus.
    pub fn new(modulus: u64) -> Self {
        ModularArithmetic { modulus }
    }
    /// Add two values mod p.
    pub fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.modulus
    }
    /// Subtract two values mod p.
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        (a + self.modulus - b % self.modulus) % self.modulus
    }
    /// Multiply two values mod p.
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.modulus as u128) as u64
    }
    /// Fast exponentiation a^e mod p using repeated squaring.
    pub fn pow(&self, a: u64, e: u64) -> u64 {
        mod_exp(a, e, self.modulus)
    }
    /// Modular inverse a^{-1} mod p (requires gcd(a, p) = 1).
    pub fn inv(&self, a: u64) -> Option<u64> {
        mod_inverse(a, self.modulus)
    }
    /// Legendre symbol (a | p): 1 if a is a quadratic residue mod prime p,
    /// -1 if not, 0 if p | a. Uses Euler's criterion: a^{(p-1)/2} mod p.
    ///
    /// # WARNING
    /// Assumes modulus is an odd prime; undefined otherwise.
    pub fn legendre(&self, a: u64) -> i64 {
        if a % self.modulus == 0 {
            return 0;
        }
        let exp = (self.modulus - 1) / 2;
        let result = self.pow(a % self.modulus, exp);
        if result == 1 {
            1
        } else {
            -1
        }
    }
}
/// A simple hash chain using a polynomial rolling hash as a stand-in for
/// a cryptographic hash function.
///
/// h_0 = initial_value, h_{i+1} = hash(h_i || data_i)
///
/// # WARNING
/// The polynomial hash used here is NOT a cryptographic hash function and
/// provides no real security. This is a structural illustration only.
#[derive(Debug, Clone)]
pub struct HashChain {
    /// The chain of hash values: chain[0] is genesis, chain[i+1] = hash(chain[i], data[i])
    pub chain: Vec<u64>,
}
impl HashChain {
    /// Create a new hash chain with a genesis value.
    pub fn new(genesis: u64) -> Self {
        HashChain {
            chain: vec![genesis],
        }
    }
    /// Internal link hash: combines previous hash with new data.
    fn link_hash(prev: u64, data: u64) -> u64 {
        let mut buf = [0u8; 16];
        buf[..8].copy_from_slice(&prev.to_le_bytes());
        buf[8..].copy_from_slice(&data.to_le_bytes());
        simple_hash(&buf)
    }
    /// Append a new block with the given data, extending the chain.
    pub fn append(&mut self, data: u64) {
        let prev = *self
            .chain
            .last()
            .expect("chain is non-empty: initialized with genesis value");
        self.chain.push(Self::link_hash(prev, data));
    }
    /// Return the current tip (latest hash).
    pub fn tip(&self) -> u64 {
        *self
            .chain
            .last()
            .expect("chain is non-empty: initialized with genesis value")
    }
    /// Verify the chain is internally consistent.
    /// Returns `true` if all links are valid, assuming the provided data sequence.
    pub fn verify(&self, data: &[u64]) -> bool {
        if data.len() + 1 != self.chain.len() {
            return false;
        }
        for (i, &d) in data.iter().enumerate() {
            let expected = Self::link_hash(self.chain[i], d);
            if self.chain[i + 1] != expected {
                return false;
            }
        }
        true
    }
}
/// Toy Diffie-Hellman parameters (prime p, generator g).
///
/// # WARNING
/// Educational only. Real DH requires carefully chosen safe primes and
/// generators. This implementation is NOT secure.
pub struct ToyDiffieHellman {
    /// A prime modulus p
    pub p: u64,
    /// A generator g of the multiplicative group mod p
    pub g: u64,
}
impl ToyDiffieHellman {
    /// Compute the public key `g^private mod p`.
    ///
    /// # WARNING
    /// Educational only.
    pub fn public_key(&self, private: u64) -> u64 {
        mod_exp(self.g, private, self.p)
    }
    /// Compute the shared secret `their_public^my_private mod p`.
    ///
    /// Both parties derive the same value: (g^a)^b = (g^b)^a mod p.
    ///
    /// # WARNING
    /// Educational only. Does not defend against MITM attacks without
    /// authentication.
    pub fn shared_secret(&self, their_public: u64, my_private: u64) -> u64 {
        mod_exp(their_public, my_private, self.p)
    }
}
/// Toy Schnorr signature scheme over a finite cyclic group Z_p^*.
///
/// # WARNING
/// Educational only. Parameters are not secure.
#[allow(dead_code)]
pub struct ToySchnorr {
    /// Prime modulus p
    pub p: u64,
    /// Group order q (should be a prime divisor of p-1)
    pub q: u64,
    /// Generator g of the subgroup of order q
    pub g: u64,
}
#[allow(dead_code)]
impl ToySchnorr {
    /// Create a Schnorr context.
    pub fn new(p: u64, q: u64, g: u64) -> Self {
        ToySchnorr { p, q, g }
    }
    /// Compute the public key: g^x mod p.
    pub fn public_key(&self, x: u64) -> u64 {
        mod_exp(self.g, x, self.p)
    }
    /// Sign a message hash `h` with private key `x` and nonce `k`.
    ///
    /// Returns (r, s) where r = g^k mod p mod q, s = (k - x*r) mod q.
    ///
    /// # WARNING
    /// Never reuse the nonce k. In practice, k must be uniformly random.
    pub fn sign(&self, x: u64, k: u64, h: u64) -> (u64, u64) {
        let r = mod_exp(self.g, k, self.p) % self.q;
        let s = (k.wrapping_add(x.wrapping_mul(r).wrapping_add(h))) % self.q;
        (r, s)
    }
    /// Verify a Schnorr signature (r, s) on hash h with public key pk.
    ///
    /// Checks: g^s * pk^h ≡ R and R mod q == r.
    pub fn verify(&self, pk: u64, h: u64, r: u64, s: u64) -> bool {
        let gs = mod_exp(self.g, s, self.p);
        let pkh = mod_exp(pk, h, self.p);
        let lhs = ((gs as u128 * pkh as u128) % self.p as u128) as u64;
        lhs % self.q == r
    }
}
/// Toy RSA key pair (n, e, d).
///
/// # WARNING
/// This is a **toy implementation for education only**. It uses tiny parameters
/// and is completely insecure. NEVER use for real data.
pub struct ToyRsa {
    /// RSA modulus n = p * q
    pub n: u64,
    /// Public exponent e (typically 65537 in real RSA)
    pub e: u64,
    /// Private exponent d (e^{-1} mod λ(n))
    pub d: u64,
}
impl ToyRsa {
    /// Generate a toy RSA key pair from two small primes `p` and `q`.
    ///
    /// Computes n = p*q, λ(n) = lcm(p-1, q-1), chooses e=65537 (or 3 as fallback),
    /// and derives d = e^{-1} mod λ(n).
    ///
    /// Returns `None` if the primes are unsuitable (e.g., gcd(e, λ(n)) ≠ 1).
    ///
    /// # WARNING
    /// Educational only. Real RSA requires 2048-bit+ primes.
    pub fn generate(p: u64, q: u64) -> Option<Self> {
        let n = p.checked_mul(q)?;
        let pm1 = p - 1;
        let qm1 = q - 1;
        let g = {
            let (gcd, _, _) = extended_gcd(pm1 as i64, qm1 as i64);
            gcd.unsigned_abs()
        };
        let lambda_n = pm1 / g * qm1;
        let candidates = [65537u64, 17, 3];
        for &e in &candidates {
            if e >= lambda_n {
                continue;
            }
            if let Some(d) = mod_inverse(e, lambda_n) {
                return Some(ToyRsa { n, e, d });
            }
        }
        None
    }
    /// Encrypt plaintext `m` as `m^e mod n`.
    ///
    /// # WARNING
    /// Toy implementation. Does not pad, not semantically secure.
    pub fn encrypt(&self, m: u64) -> u64 {
        mod_exp(m, self.e, self.n)
    }
    /// Decrypt ciphertext `c` as `c^d mod n`.
    ///
    /// # WARNING
    /// Toy implementation only.
    pub fn decrypt(&self, c: u64) -> u64 {
        mod_exp(c, self.d, self.n)
    }
}
/// Polynomial commitment scheme toy (Kate-style, educational model).
///
/// In real KZG commitments, the commitment is an elliptic curve point.
/// Here we use a simplified polynomial evaluation model.
///
/// # WARNING
/// This is a structural illustration only. It provides NO cryptographic security.
#[allow(dead_code)]
pub struct ToyPolyCommit {
    /// Evaluation point (the "trusted setup" scalar τ)
    pub tau: u64,
    /// Field modulus
    pub q: u64,
}
#[allow(dead_code)]
impl ToyPolyCommit {
    /// Create a toy polynomial commitment context.
    pub fn new(tau: u64, q: u64) -> Self {
        ToyPolyCommit { tau, q }
    }
    /// Commit to a polynomial given as coefficient vector (lowest degree first).
    ///
    /// Commitment = p(τ) mod q (educational stand-in for g^{p(τ)} on an EC).
    pub fn commit(&self, coeffs: &[u64]) -> u64 {
        let q = self.q as u128;
        let tau = self.tau as u128;
        let mut power = 1u128;
        let mut result = 0u128;
        for &c in coeffs {
            result = (result + c as u128 * power) % q;
            power = power * tau % q;
        }
        result as u64
    }
    /// Evaluate the polynomial at a given point z.
    pub fn evaluate(&self, coeffs: &[u64], z: u64) -> u64 {
        let q = self.q as u128;
        let z = z as u128;
        let mut power = 1u128;
        let mut result = 0u128;
        for &c in coeffs {
            result = (result + c as u128 * power) % q;
            power = power * z % q;
        }
        result as u64
    }
    /// Verify an opening: checks that the committed value matches
    /// the claimed evaluation p(z) = v by comparing with stored commitment.
    ///
    /// In real KZG this requires a pairing check.
    pub fn verify_opening(&self, commitment: u64, z: u64, v: u64, coeffs: &[u64]) -> bool {
        let actual = self.evaluate(coeffs, z);
        let claimed_commit = self.commit(coeffs);
        actual == v && claimed_commit == commitment
    }
}
/// Lattice-based encryption toy model.
///
/// Implements a toy version of LWE encryption for educational illustration.
/// Uses very small parameters (NOT secure).
///
/// # WARNING
/// This is an educational toy. Parameters are not cryptographically secure.
#[allow(dead_code)]
pub struct ToyLwe {
    /// Dimension n (key length)
    pub n: usize,
    /// Modulus q
    pub q: u64,
    /// Error bound (max absolute value of noise)
    pub error_bound: u64,
}
#[allow(dead_code)]
impl ToyLwe {
    /// Create a new LWE context with given parameters.
    pub fn new(n: usize, q: u64, error_bound: u64) -> Self {
        ToyLwe { n, q, error_bound }
    }
    /// Compute (a · s) mod q, the inner product of a and s.
    pub fn inner_product(&self, a: &[u64], s: &[u64]) -> u64 {
        assert_eq!(a.len(), s.len());
        let q = self.q as u128;
        a.iter()
            .zip(s.iter())
            .fold(0u128, |acc, (&ai, &si)| (acc + ai as u128 * si as u128) % q) as u64
    }
    /// Encrypt a bit (0 or 1) using the LWE public key (A, b = As + e).
    ///
    /// Returns (u, v) where u = A^T r, v = b^T r + bit * floor(q/2).
    /// For simplicity in this toy version, just uses the scalar version.
    pub fn encrypt_bit(&self, secret: &[u64], bit: u8, noise: u64, a: &[u64]) -> (Vec<u64>, u64) {
        let b = (self.inner_product(a, secret) + noise % self.q) % self.q;
        let v = (b + (bit as u64) * (self.q / 2)) % self.q;
        (a.to_vec(), v)
    }
    /// Decrypt: compute v - a · s mod q, return 0 or 1.
    pub fn decrypt_bit(&self, secret: &[u64], a: &[u64], v: u64) -> u8 {
        let dot = self.inner_product(a, secret);
        let diff = (v + self.q - dot) % self.q;
        if diff < self.q / 4 || diff > 3 * self.q / 4 {
            0
        } else {
            1
        }
    }
}
/// Shamir's Secret Sharing over a finite field Z_p.
///
/// Splits a secret s into n shares such that any k shares can reconstruct s
/// via Lagrange interpolation, but any k-1 shares reveal nothing about s.
///
/// # WARNING
/// Educational implementation. The polynomial coefficients are NOT generated
/// with cryptographically secure randomness. Do NOT use for real secrets.
pub struct ShamirSecretShare {
    /// Prime modulus p (field F_p)
    pub p: u64,
    /// Threshold k: minimum shares needed for reconstruction
    pub k: usize,
    /// Total shares n
    pub n: usize,
}
impl ShamirSecretShare {
    /// Create a new Shamir secret sharing instance.
    pub fn new(p: u64, k: usize, n: usize) -> Self {
        ShamirSecretShare { p, k, n }
    }
    /// Evaluate the polynomial f(x) = coeffs[0] + coeffs[1]*x + ... + coeffs[k-1]*x^{k-1} mod p.
    fn eval_poly(&self, coeffs: &[u64], x: u64) -> u64 {
        let mut result = 0u64;
        for &c in coeffs.iter().rev() {
            result = (((result as u128 * x as u128) % self.p as u128 + c as u128) % self.p as u128)
                as u64;
        }
        result
    }
    /// Split a secret `s` into n shares using a deterministic polynomial
    /// with coefficients derived from `seed` (for reproducibility in tests).
    ///
    /// Returns a vector of (x, y) pairs where x = 1..=n and y = f(x).
    ///
    /// # WARNING
    /// The seed-based coefficient generation is NOT secure. Real Shamir's scheme
    /// requires cryptographically random coefficients.
    pub fn share(&self, secret: u64, seed: u64) -> Vec<(u64, u64)> {
        let mut coeffs = vec![secret % self.p];
        for i in 1..self.k {
            let c = (seed
                .wrapping_mul(0x9e3779b97f4a7c15)
                .wrapping_add(i as u64 * 0x6c62272e07bb0142))
                % self.p;
            coeffs.push(c);
        }
        (1..=self.n as u64)
            .map(|x| (x, self.eval_poly(&coeffs, x)))
            .collect()
    }
    /// Reconstruct the secret from any k shares using Lagrange interpolation over F_p.
    ///
    /// Each share is an (x, y) pair. Computes f(0) = Σ y_i * L_i(0) mod p.
    pub fn reconstruct(&self, shares: &[(u64, u64)]) -> Option<u64> {
        if shares.len() < self.k {
            return None;
        }
        let shares = &shares[..self.k];
        let p = self.p;
        let mut secret = 0u64;
        for (i, &(xi, yi)) in shares.iter().enumerate() {
            let mut num: u128 = 1;
            let mut den: u128 = 1;
            for (j, &(xj, _)) in shares.iter().enumerate() {
                if i != j {
                    num = num * ((p - xj % p) as u128) % p as u128;
                    let diff = ((xi as i128 - xj as i128).rem_euclid(p as i128)) as u64;
                    den = den * diff as u128 % p as u128;
                }
            }
            let den_inv = mod_inverse(den as u64, p)?;
            let li = (num as u64 * den_inv % p * yi % p) % p;
            secret = (secret + li) % p;
        }
        Some(secret)
    }
}
/// Toy RSA key generation from scratch: finds two random-ish primes near
/// the given starting point using Miller-Rabin, then builds the key pair.
///
/// # WARNING
/// This is purely educational. Key sizes are dangerously small.
pub struct RsaKeyGen;
impl RsaKeyGen {
    /// Find the next (probably) prime after `start` using Miller-Rabin.
    pub fn next_prime(start: u64) -> u64 {
        let witnesses = [2u64, 3, 5, 7, 11, 13];
        let mut candidate = if start % 2 == 0 { start + 1 } else { start };
        loop {
            if miller_rabin(candidate, &witnesses) {
                return candidate;
            }
            candidate += 2;
        }
    }
    /// Generate a toy RSA key pair by finding two primes near `seed`.
    ///
    /// Returns `(ToyRsa, p, q)` for inspection.
    ///
    /// # WARNING
    /// For illustration only. Do not use for any security purpose.
    pub fn generate_from_seed(seed: u64) -> Option<(ToyRsa, u64, u64)> {
        let p = Self::next_prime(seed);
        let q = Self::next_prime(p + 2);
        let rsa = ToyRsa::generate(p, q)?;
        Some((rsa, p, q))
    }
}
/// Polynomial-ring arithmetic over Z_q[x]/(x^n + 1).
///
/// Supports the ring operations needed for Ring-LWE and NTRU-based schemes.
///
/// # WARNING
/// Educational implementation. Not optimized or secure.
#[allow(dead_code)]
pub struct RingZq {
    /// Ring dimension n (must be a power of 2 for NTT)
    pub n: usize,
    /// Modulus q
    pub q: u64,
}
#[allow(dead_code)]
impl RingZq {
    /// Create a new ring Z_q[x]/(x^n + 1).
    pub fn new(n: usize, q: u64) -> Self {
        RingZq { n, q }
    }
    /// Reduce polynomial coefficients modulo q.
    pub fn reduce(&self, poly: &[u64]) -> Vec<u64> {
        poly.iter().map(|&c| c % self.q).collect()
    }
    /// Add two polynomials in the ring.
    pub fn add(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        a.iter()
            .zip(b.iter())
            .map(|(&ai, &bi)| (ai + bi) % self.q)
            .collect()
    }
    /// Subtract two polynomials in the ring.
    pub fn sub(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        a.iter()
            .zip(b.iter())
            .map(|(&ai, &bi)| (ai + self.q - bi % self.q) % self.q)
            .collect()
    }
    /// Multiply two polynomials in Z_q[x]/(x^n + 1) using schoolbook multiplication.
    ///
    /// O(n^2) — for educational purposes. Real implementations use NTT.
    pub fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.n);
        assert_eq!(b.len(), self.n);
        let mut result = vec![0i128; self.n];
        let q = self.q as i128;
        for i in 0..self.n {
            for j in 0..self.n {
                let idx = i + j;
                let coeff = a[i] as i128 * b[j] as i128 % q;
                if idx < self.n {
                    result[idx] = (result[idx] + coeff) % q;
                } else {
                    result[idx - self.n] = (result[idx - self.n] - coeff + q) % q;
                }
            }
        }
        result.iter().map(|&c| c as u64).collect()
    }
    /// L-infinity norm: max absolute deviation from 0 or q (viewing coefficients
    /// as centered in (-q/2, q/2]).
    pub fn linf_norm(&self, poly: &[u64]) -> u64 {
        let half_q = self.q / 2;
        poly.iter()
            .map(|&c| {
                let c = c % self.q;
                if c <= half_q {
                    c
                } else {
                    self.q - c
                }
            })
            .max()
            .unwrap_or(0)
    }
}
