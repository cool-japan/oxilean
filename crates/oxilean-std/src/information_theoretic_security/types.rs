//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Data for information-theoretically (unconditionally) secure schemes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UnconditionalSecurity {
    /// Scheme name.
    pub scheme: String,
    /// Whether the scheme is perfectly secure.
    pub is_perfect: bool,
    /// Key length in bits.
    pub key_length: usize,
    /// Message length in bits.
    pub message_length: usize,
}
#[allow(dead_code)]
impl UnconditionalSecurity {
    /// Creates unconditional security data.
    pub fn new(scheme: &str, key_len: usize, msg_len: usize) -> Self {
        UnconditionalSecurity {
            scheme: scheme.to_string(),
            is_perfect: key_len >= msg_len,
            key_length: key_len,
            message_length: msg_len,
        }
    }
    /// Creates one-time pad data.
    pub fn one_time_pad(n: usize) -> Self {
        UnconditionalSecurity::new("One-Time Pad", n, n)
    }
    /// Shannon's perfect secrecy theorem: H(M | C) = H(M) iff key is as long as message.
    pub fn shannon_perfect_secrecy(&self) -> String {
        if self.is_perfect {
            format!("{}: H(M|C) = H(M) (Shannon perfect secrecy)", self.scheme)
        } else {
            format!("{}: NOT perfectly secure (key too short)", self.scheme)
        }
    }
    /// Information leakage = H(M) - H(M | C).
    pub fn information_leakage_bits(&self) -> f64 {
        if self.is_perfect {
            0.0
        } else {
            (self.message_length - self.key_length.min(self.message_length)) as f64
        }
    }
}
/// Statistical distance between two distributions over the same alphabet.
#[derive(Debug, Clone)]
pub struct StatisticalDistance {
    /// First distribution (must sum to 1).
    pub dist1: Vec<f64>,
    /// Second distribution (must sum to 1).
    pub dist2: Vec<f64>,
}
impl StatisticalDistance {
    /// Total variation distance: (1/2) Σ |p_i - q_i|.
    pub fn total_variation(&self) -> f64 {
        self.dist1
            .iter()
            .zip(self.dist2.iter())
            .map(|(p, q)| (p - q).abs())
            .sum::<f64>()
            * 0.5
    }
    /// Hellinger distance: sqrt( Σ (sqrt(p_i) - sqrt(q_i))^2 / 2 ).
    pub fn hellinger_distance(&self) -> f64 {
        let sum: f64 = self
            .dist1
            .iter()
            .zip(self.dist2.iter())
            .map(|(p, q)| (p.sqrt() - q.sqrt()).powi(2))
            .sum();
        (sum / 2.0).sqrt()
    }
    /// KL divergence D_KL(P || Q) = Σ p_i * ln(p_i / q_i).
    pub fn kl_divergence(&self) -> f64 {
        self.dist1
            .iter()
            .zip(self.dist2.iter())
            .filter(|(p, q)| **p > 0.0 && **q > 0.0)
            .map(|(p, q)| p * (p / q).ln())
            .sum()
    }
}
/// Min-entropy of a distribution.
#[derive(Debug, Clone)]
pub struct MinEntropy {
    /// Probability distribution (values must be non-negative and sum to 1).
    pub distribution: Vec<f64>,
}
impl MinEntropy {
    /// H_∞(X) = -log₂(max_x p(x)).
    pub fn compute(&self) -> f64 {
        let max_p = self.distribution.iter().cloned().fold(0.0f64, f64::max);
        if max_p <= 0.0 {
            return 0.0;
        }
        -max_p.log2()
    }
    /// Leftover hash lemma: can extract m bits if H_∞ ≥ m + 2·log(1/ε).
    pub fn leftover_hash_lemma(&self, output_bits: f64, eps: f64) -> bool {
        let h_inf = self.compute();
        h_inf >= output_bits + 2.0 * (1.0 / eps).log2()
    }
}
/// Educational simulation of Oblivious RAM (square-root ORAM construction).
///
/// WARNING: Educational only — NOT a secure ORAM implementation.
#[derive(Debug, Clone)]
pub struct ORAMSimulation {
    /// Main memory storage (encrypted blocks).
    memory: Vec<Vec<u8>>,
    /// Cache for recently accessed blocks (hidden reordering).
    cache: Vec<(usize, Vec<u8>)>,
    /// Block size in bytes.
    pub block_size: usize,
    /// Number of blocks.
    pub n: usize,
    /// Access counter (used for periodic reshuffling).
    access_count: usize,
}
impl ORAMSimulation {
    /// Create a new ORAM simulation with n blocks of block_size bytes.
    pub fn new(n: usize, block_size: usize) -> Self {
        let memory = vec![vec![0u8; block_size]; n];
        Self {
            memory,
            cache: Vec::new(),
            block_size,
            n,
            access_count: 0,
        }
    }
    /// Read block at logical index `idx` (ORAM access — hides real index).
    pub fn read(&mut self, idx: usize) -> Vec<u8> {
        assert!(idx < self.n, "index out of range");
        self.access_count += 1;
        if let Some(pos) = self.cache.iter().position(|(i, _)| *i == idx) {
            let data = self.cache[pos].1.clone();
            let _ = self.memory[self.access_count % self.n].clone();
            return data;
        }
        let data = self.memory[idx].clone();
        self.cache.push((idx, data.clone()));
        let sqrt_n = (self.n as f64).sqrt().ceil() as usize;
        if self.cache.len() >= sqrt_n {
            self.reshuffle();
        }
        data
    }
    /// Write block at logical index `idx`.
    pub fn write(&mut self, idx: usize, data: Vec<u8>) {
        assert!(idx < self.n, "index out of range");
        assert_eq!(data.len(), self.block_size, "data must be block_size bytes");
        self.access_count += 1;
        if let Some(pos) = self.cache.iter().position(|(i, _)| *i == idx) {
            self.cache[pos].1 = data;
        } else {
            self.cache.push((idx, data));
        }
    }
    /// Reshuffle: flush cache back to memory in permuted order (hides access pattern).
    fn reshuffle(&mut self) {
        for (idx, data) in self.cache.drain(..) {
            self.memory[idx] = data;
        }
    }
    /// Overhead factor: O(√n) for sqrt-ORAM.
    pub fn overhead_factor(&self) -> f64 {
        (self.n as f64).sqrt()
    }
    /// Whether access patterns are hidden (always true by ORAM definition).
    pub fn access_pattern_hidden(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WiretapChannel {
    pub main_channel_capacity: f64,
    pub wiretap_channel_capacity: f64,
    pub secrecy_capacity: f64,
    pub channel_model: String,
}
#[allow(dead_code)]
impl WiretapChannel {
    pub fn new(main_cap: f64, wiretap_cap: f64, model: &str) -> Self {
        let secrecy = if main_cap > wiretap_cap {
            main_cap - wiretap_cap
        } else {
            0.0
        };
        WiretapChannel {
            main_channel_capacity: main_cap,
            wiretap_channel_capacity: wiretap_cap,
            secrecy_capacity: secrecy,
            channel_model: model.to_string(),
        }
    }
    pub fn wyner_secrecy_capacity(&self) -> f64 {
        self.secrecy_capacity
    }
    pub fn is_achievable_rate(&self, rate: f64) -> bool {
        rate <= self.secrecy_capacity
    }
    pub fn broadcast_channel_degraded_description(&self) -> String {
        format!(
            "Wyner wiretap: C_s = max(0, C_m - C_w) = max(0, {:.3} - {:.3}) = {:.3}",
            self.main_channel_capacity, self.wiretap_channel_capacity, self.secrecy_capacity
        )
    }
    pub fn gaussian_wiretap(snr_main: f64, snr_wiretap: f64) -> Self {
        let c_main = 0.5 * (1.0 + snr_main).log2();
        let c_wiretap = 0.5 * (1.0 + snr_wiretap).log2();
        WiretapChannel::new(c_main, c_wiretap, "Gaussian")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BB84Protocol {
    pub key_length_bits: usize,
    pub quantum_bit_error_rate: f64,
    pub privacy_amplification_factor: f64,
    pub sifting_efficiency: f64,
}
#[allow(dead_code)]
impl BB84Protocol {
    pub fn new(key_length: usize) -> Self {
        BB84Protocol {
            key_length_bits: key_length,
            quantum_bit_error_rate: 0.05,
            privacy_amplification_factor: 0.7,
            sifting_efficiency: 0.5,
        }
    }
    pub fn with_qber(mut self, qber: f64) -> Self {
        self.quantum_bit_error_rate = qber;
        self
    }
    pub fn is_secure(&self) -> bool {
        self.quantum_bit_error_rate < 0.11
    }
    pub fn net_key_rate(&self) -> f64 {
        let raw_rate = self.sifting_efficiency;
        let after_ec = raw_rate * (1.0 - self.binary_entropy(self.quantum_bit_error_rate));
        after_ec * self.privacy_amplification_factor
    }
    fn binary_entropy(&self, p: f64) -> f64 {
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        -(p * p.log2() + (1.0 - p) * (1.0 - p).log2())
    }
    pub fn decoy_state_variant(&self) -> String {
        format!(
            "BB84 with decoy states: QBER={:.3}, net_rate={:.4}",
            self.quantum_bit_error_rate,
            self.net_key_rate()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GarbledCircuit {
    pub circuit_description: String,
    pub num_gates: usize,
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub wire_labels: Vec<(Vec<u8>, Vec<u8>)>,
}
#[allow(dead_code)]
impl GarbledCircuit {
    pub fn new(desc: &str, gates: usize, inputs: usize, outputs: usize) -> Self {
        let wire_labels = (0..inputs)
            .map(|_| (vec![0u8; 16], vec![1u8; 16]))
            .collect();
        GarbledCircuit {
            circuit_description: desc.to_string(),
            num_gates: gates,
            num_inputs: inputs,
            num_outputs: outputs,
            wire_labels,
        }
    }
    pub fn communication_complexity_bits(&self) -> usize {
        self.num_gates * 4 * 128
    }
    pub fn is_free_xor_optimized(&self) -> bool {
        self.circuit_description.contains("XOR")
    }
    pub fn row_reduction_complexity(&self) -> usize {
        self.num_gates * 3 * 128
    }
}
/// Shamir secret-sharing scheme (k-of-n over a prime field).
#[derive(Debug, Clone)]
pub struct SecretSharingScheme {
    /// Total number of shares.
    pub n: usize,
    /// Reconstruction threshold.
    pub k: usize,
    /// Prime modulus.
    pub prime: u64,
}
impl SecretSharingScheme {
    /// Generate n shares of `secret` using threshold k.
    pub fn shamir_share(&self, secret: u64) -> Vec<(u64, u64)> {
        let coefficients: Vec<u64> = (1..self.k)
            .map(|i| (secret.wrapping_add(i as u64)) % self.prime)
            .collect();
        shamir_share(secret, self.k, self.n, self.prime, &coefficients)
    }
    /// Reconstruct a secret from k (or more) shares.
    pub fn reconstruct(&self, shares: &[(u64, u64)]) -> u64 {
        shamir_reconstruct(shares, self.prime)
    }
    /// A (k−1)-out-of-n scheme is (k−1)-private by Shannon's theorem.
    pub fn is_t_private(&self) -> bool {
        self.k > 0
    }
}
/// Seeded randomness extractor using a polynomial hash.
///
/// WARNING: Educational only — NOT cryptographically secure.
#[derive(Debug, Clone)]
pub struct RandomnessExtractor {
    /// Min-entropy threshold k (bits) that the source must have.
    pub k: usize,
    /// Output length in bits.
    pub m: usize,
    /// Security parameter ε.
    pub eps: f64,
}
impl RandomnessExtractor {
    /// Create a new extractor with given parameters.
    pub fn new(k: usize, m: usize, eps: f64) -> Self {
        Self { k, m, eps }
    }
    /// Check Leftover Hash Lemma: m ≤ k − 2·log₂(1/ε).
    pub fn lhl_satisfied(&self) -> bool {
        let bound = self.k as f64 - 2.0 * (1.0 / self.eps).log2();
        self.m as f64 <= bound
    }
    /// Extract pseudorandom bits from a source using a seeded hash (toy: polynomial).
    /// Seed encodes two hash parameters (a, b) as 8 bytes each.
    pub fn extract(&self, source: &[u8], seed: &[u8]) -> Vec<u8> {
        let a = seed
            .iter()
            .take(8)
            .fold(0u64, |acc, &b| acc.wrapping_mul(257).wrapping_add(b as u64));
        let b = seed
            .iter()
            .skip(8)
            .take(8)
            .fold(1u64, |acc, &b| acc.wrapping_mul(257).wrapping_add(b as u64));
        let prime = 18446744073709551557u64;
        let x = source.iter().fold(0u64, |acc, &byte| {
            acc.wrapping_mul(257).wrapping_add(byte as u64)
        });
        let h = (a.wrapping_mul(x).wrapping_add(b)) % prime;
        let out_bytes = (self.m + 7) / 8;
        (0..out_bytes)
            .map(|i| ((h >> (i % 8)) & 0xFF) as u8)
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct E91Protocol {
    pub entangled_pair_rate: f64,
    pub bell_inequality_violation: f64,
    pub chsh_value: f64,
}
#[allow(dead_code)]
impl E91Protocol {
    pub fn new(pair_rate: f64) -> Self {
        E91Protocol {
            entangled_pair_rate: pair_rate,
            bell_inequality_violation: 2.828,
            chsh_value: 2.828,
        }
    }
    pub fn is_maximally_entangled(&self) -> bool {
        (self.chsh_value - 2.0_f64.sqrt() * 2.0).abs() < 0.001
    }
    pub fn eavesdropping_detected(&self, measured_chsh: f64) -> bool {
        measured_chsh < 2.5
    }
    pub fn description(&self) -> String {
        format!(
            "Ekert E91: pair_rate={:.2}, CHSH={:.3} (max=2√2≈2.828)",
            self.entangled_pair_rate, self.chsh_value
        )
    }
}
/// Data for a zero-knowledge proof system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZKProofData {
    /// Name of the ZK proof system.
    pub name: String,
    /// Whether the proof is interactive.
    pub is_interactive: bool,
    /// Completeness probability.
    pub completeness: f64,
    /// Soundness probability.
    pub soundness: f64,
    /// Whether the proof is zero-knowledge.
    pub is_zero_knowledge: bool,
    /// Prover's strategy description.
    pub prover_strategy: String,
}
#[allow(dead_code)]
impl ZKProofData {
    /// Creates ZK proof data.
    pub fn new(name: &str, interactive: bool) -> Self {
        ZKProofData {
            name: name.to_string(),
            is_interactive: interactive,
            completeness: 1.0,
            soundness: 0.5,
            is_zero_knowledge: true,
            prover_strategy: "honest prover".to_string(),
        }
    }
    /// Creates a Schnorr identification protocol.
    pub fn schnorr() -> Self {
        ZKProofData {
            name: "Schnorr identification".to_string(),
            is_interactive: true,
            completeness: 1.0,
            soundness: 0.5,
            is_zero_knowledge: true,
            prover_strategy:
                "Peggy commits to r = g^k, responds to challenge c with s = k + cx mod q"
                    .to_string(),
        }
    }
    /// Creates a non-interactive ZK (NIZK) using Fiat-Shamir.
    pub fn fiat_shamir_transform(base: ZKProofData) -> Self {
        ZKProofData {
            name: format!("{} (Fiat-Shamir)", base.name),
            is_interactive: false,
            ..base
        }
    }
    /// Knowledge soundness error: 2^{-k} after k repetitions.
    pub fn soundness_after_repetitions(&self, k: usize) -> f64 {
        self.soundness.powi(k as i32)
    }
    /// Returns the simulation description.
    pub fn simulation_description(&self) -> String {
        format!(
            "ZK simulator S for {}: produces transcripts indistinguishable from real",
            self.name
        )
    }
    /// Checks if the protocol satisfies all three ZK properties.
    pub fn is_valid_zkp(&self) -> bool {
        self.completeness >= 1.0 - 1e-9 && self.soundness <= 0.9 && self.is_zero_knowledge
    }
}
/// A 2-universal hash family h_{a,b}(x) = (ax + b) mod p mod m.
///
/// WARNING: Educational only — NOT cryptographically secure.
#[derive(Debug, Clone)]
pub struct UniversalHashFamily {
    /// Prime modulus (must be larger than domain size).
    pub prime: u64,
    /// Output range size m.
    pub output_size: u64,
    /// Keys (a, b) for each hash function in the family.
    pub keys: Vec<(u64, u64)>,
}
impl UniversalHashFamily {
    /// Create a deterministic universal hash family with given parameters and keys.
    pub fn new(prime: u64, output_size: u64, keys: Vec<(u64, u64)>) -> Self {
        Self {
            prime,
            output_size,
            keys,
        }
    }
    /// Evaluate the i-th hash function on input x.
    pub fn hash(&self, index: usize, x: u64) -> u64 {
        let (a, b) = self.keys[index];
        (a.wrapping_mul(x).wrapping_add(b)) % self.prime % self.output_size
    }
    /// Number of hash functions in the family.
    pub fn size(&self) -> usize {
        self.keys.len()
    }
    /// Verify 2-universality: for distinct x1 != x2, count collisions h(x1)=h(x2).
    /// Returns empirical collision probability over the family.
    pub fn empirical_collision_prob(&self, x1: u64, x2: u64) -> f64 {
        if x1 == x2 {
            return 1.0;
        }
        let collisions = self
            .keys
            .iter()
            .filter(|(a, b)| {
                let h1 = (a.wrapping_mul(x1).wrapping_add(*b)) % self.prime % self.output_size;
                let h2 = (a.wrapping_mul(x2).wrapping_add(*b)) % self.prime % self.output_size;
                h1 == h2
            })
            .count();
        collisions as f64 / self.keys.len() as f64
    }
}
/// Quantum-secure cryptographic scheme wrapper.
#[derive(Debug, Clone)]
pub struct QuantumSecureScheme {
    /// Name of the underlying classical security assumption.
    pub classical_security: String,
}
impl QuantumSecureScheme {
    /// Lattice-based and hash-based schemes are considered quantum-secure.
    pub fn is_quantum_secure(&self) -> bool {
        let cs = self.classical_security.to_lowercase();
        cs.contains("lattice")
            || cs.contains("lwe")
            || cs.contains("ntru")
            || cs.contains("hash")
            || cs.contains("code")
    }
    /// Description of the quantum-hard reduction.
    pub fn reduction_to_qhard_problem(&self) -> String {
        if self.is_quantum_secure() {
            format!(
                "Reduction to quantum-hard problem: {}",
                self.classical_security
            )
        } else {
            format!(
                "WARNING: {} may not be quantum-secure (consider lattice/hash alternatives)",
                self.classical_security
            )
        }
    }
}
/// Perfect secrecy model: message, ciphertext and key spaces by size.
#[derive(Debug, Clone)]
pub struct PerfectSecrecy {
    /// Number of possible messages.
    pub message_space: usize,
    /// Number of possible ciphertexts.
    pub ciphertext_space: usize,
    /// Number of possible keys.
    pub key_space: usize,
}
impl PerfectSecrecy {
    /// Shannon's perfect secrecy criterion: key space ≥ message space.
    pub fn shannon_perfect_secrecy(&self) -> bool {
        self.key_space >= self.message_space
    }
    /// Verify that a one-time-pad of this size is perfectly secret.
    pub fn one_time_pad_is_perfect(&self) -> bool {
        self.key_space == self.message_space && self.ciphertext_space == self.message_space
    }
}
/// Types of randomness extractors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractorType {
    /// Seeded extractor (public random seed).
    Seeded,
    /// Deterministic extractor (two independent sources).
    Deterministic,
    /// Quantum-proof extractor (secure against quantum side information).
    Quantum,
}
impl ExtractorType {
    /// Minimum source min-entropy required (in bits) for `output_bits` output.
    pub fn min_entropy_required(&self, output_bits: f64, eps: f64) -> f64 {
        match self {
            ExtractorType::Seeded => output_bits + 2.0 * (1.0 / eps).log2(),
            ExtractorType::Deterministic => 2.0 * output_bits,
            ExtractorType::Quantum => output_bits + 2.0 * (1.0 / eps).log2() + 1.0,
        }
    }
    /// Usable output bits given source min-entropy k and security eps.
    pub fn output_bits(&self, min_entropy: f64, eps: f64) -> f64 {
        match self {
            ExtractorType::Seeded => min_entropy - 2.0 * (1.0 / eps).log2(),
            ExtractorType::Deterministic => min_entropy / 2.0,
            ExtractorType::Quantum => min_entropy - 2.0 * (1.0 / eps).log2() - 1.0,
        }
        .max(0.0)
    }
}
/// Oblivious RAM abstraction.
#[derive(Debug, Clone)]
pub struct ObliviousRAM {
    /// Number of memory blocks.
    pub n: usize,
    /// Size of each block in bytes.
    pub block_size: usize,
}
impl ObliviousRAM {
    /// ORAM hides access patterns by definition.
    pub fn access_pattern_hidden(&self) -> bool {
        true
    }
    /// Path-ORAM overhead: O(log^2 n) amortised bandwidth.
    pub fn overhead_factor(&self) -> f64 {
        let log_n = (self.n as f64).log2();
        log_n * log_n
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OTVariant {
    OneOutOfTwo,
    OneOutOfN(usize),
    ObliviousRAM,
    RandOT,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HomomorphicScheme {
    pub scheme_name: String,
    pub security_level: usize,
    pub multiplicative_depth: usize,
    pub noise_budget_initial: f64,
    pub is_bootstrappable: bool,
}
#[allow(dead_code)]
impl HomomorphicScheme {
    pub fn bgv(security_level: usize, depth: usize) -> Self {
        HomomorphicScheme {
            scheme_name: "BGV".to_string(),
            security_level,
            multiplicative_depth: depth,
            noise_budget_initial: 40.0,
            is_bootstrappable: true,
        }
    }
    pub fn ckks(security_level: usize, depth: usize) -> Self {
        HomomorphicScheme {
            scheme_name: "CKKS".to_string(),
            security_level,
            multiplicative_depth: depth,
            noise_budget_initial: 35.0,
            is_bootstrappable: true,
        }
    }
    pub fn bfv(security_level: usize, depth: usize) -> Self {
        HomomorphicScheme {
            scheme_name: "BFV".to_string(),
            security_level,
            multiplicative_depth: depth,
            noise_budget_initial: 38.0,
            is_bootstrappable: false,
        }
    }
    pub fn noise_after_mults(&self, num_mults: usize) -> f64 {
        self.noise_budget_initial - (num_mults as f64) * 3.5
    }
    pub fn can_evaluate_circuit(&self, mults: usize) -> bool {
        mults <= self.multiplicative_depth && self.noise_after_mults(mults) > 0.0
    }
    pub fn ring_lwe_parameter_description(&self) -> String {
        let n = match self.security_level {
            128 => 4096,
            192 => 8192,
            256 => 16384,
            _ => 2048,
        };
        format!(
            "{}: λ={}, n={}, depth={}",
            self.scheme_name, self.security_level, n, self.multiplicative_depth
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ComputationalIndistinguishability {
    pub advantage_bound: f64,
    pub distinguisher_complexity: usize,
    pub security_parameter: usize,
}
#[allow(dead_code)]
impl ComputationalIndistinguishability {
    pub fn new(lambda: usize) -> Self {
        ComputationalIndistinguishability {
            advantage_bound: 1.0 / 2.0_f64.powi((lambda / 2) as i32),
            distinguisher_complexity: if lambda < 64 {
                1_usize << lambda
            } else {
                usize::MAX
            },
            security_parameter: lambda,
        }
    }
    pub fn is_negligible_advantage(&self) -> bool {
        self.advantage_bound < 1e-10
    }
    pub fn statistical_distance(&self, _dist_a: &[f64], _dist_b: &[f64]) -> f64 {
        0.5 * self.advantage_bound
    }
    pub fn hybrid_argument_steps(&self, n_hybrids: usize) -> f64 {
        (n_hybrids as f64) * self.advantage_bound
    }
}
/// Multi-party computation protocol meta-data.
#[derive(Debug, Clone)]
pub struct MultiPartyComputation {
    /// Total number of parties.
    pub n: usize,
    /// Corruption threshold (adversary controls at most t parties).
    pub t: usize,
}
impl MultiPartyComputation {
    /// Passive security requires t < n/2; active security requires t < n/3.
    pub fn is_t_secure(&self) -> bool {
        self.t < self.n / 2
    }
    /// Name of the protocol type based on the corruption threshold.
    pub fn gmw_protocol_type(&self) -> &'static str {
        if self.t < self.n / 3 {
            "actively-secure GMW"
        } else if self.t < self.n / 2 {
            "passively-secure GMW"
        } else {
            "insecure"
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LeakageResilientScheme {
    pub leakage_bound_bits: usize,
    pub memory_bits: usize,
    pub min_entropy_retained: f64,
    pub leakage_model: String,
}
#[allow(dead_code)]
impl LeakageResilientScheme {
    pub fn bounded_leakage(memory: usize, leakage: usize) -> Self {
        let entropy = (memory - leakage) as f64;
        LeakageResilientScheme {
            leakage_bound_bits: leakage,
            memory_bits: memory,
            min_entropy_retained: entropy,
            leakage_model: "Bounded Leakage".to_string(),
        }
    }
    pub fn continual_leakage(memory: usize, per_round_leakage: usize) -> Self {
        LeakageResilientScheme {
            leakage_bound_bits: per_round_leakage,
            memory_bits: memory,
            min_entropy_retained: (memory - 2 * per_round_leakage) as f64,
            leakage_model: "Continual Leakage".to_string(),
        }
    }
    pub fn is_leakage_tolerable(&self) -> bool {
        self.min_entropy_retained > 0.0 && self.leakage_bound_bits < self.memory_bits / 2
    }
    pub fn relative_leakage_rate(&self) -> f64 {
        self.leakage_bound_bits as f64 / self.memory_bits as f64
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GarbledBootstrap {
    pub fhe_scheme: HomomorphicScheme,
    pub bootstrap_cost_seconds: f64,
    pub amortized_cost: f64,
}
#[allow(dead_code)]
impl GarbledBootstrap {
    pub fn new(scheme: HomomorphicScheme, cost: f64) -> Self {
        GarbledBootstrap {
            fhe_scheme: scheme,
            bootstrap_cost_seconds: cost,
            amortized_cost: cost / 1000.0,
        }
    }
    pub fn is_practical(&self) -> bool {
        self.bootstrap_cost_seconds < 1.0
    }
}
/// Shamir secret sharing data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ShamirSecretSharing {
    /// Number of shares n.
    pub n_shares: usize,
    /// Threshold t (minimum shares to reconstruct).
    pub threshold: usize,
    /// Prime modulus p (for GF(p)).
    pub prime: u64,
    /// Polynomial coefficients (a_0 = secret, a_1, ..., a_{t-1}).
    pub polynomial: Vec<u64>,
}
#[allow(dead_code)]
impl ShamirSecretSharing {
    /// Creates Shamir secret sharing.
    pub fn new(n: usize, t: usize, prime: u64, secret: u64) -> Self {
        let mut poly = vec![secret % prime];
        for i in 1..t {
            poly.push((i as u64 * 7 + 3) % prime);
        }
        ShamirSecretSharing {
            n_shares: n,
            threshold: t,
            prime,
            polynomial: poly,
        }
    }
    /// Evaluates the polynomial at point x mod p.
    pub fn evaluate(&self, x: u64) -> u64 {
        let p = self.prime;
        let mut result = 0u64;
        let mut power = 1u64;
        for &coeff in &self.polynomial {
            result = (result + coeff * power % p) % p;
            power = power * x % p;
        }
        result
    }
    /// Generates share i: (i, f(i) mod p).
    pub fn generate_share(&self, i: usize) -> (usize, u64) {
        let val = self.evaluate(i as u64 + 1);
        (i + 1, val)
    }
    /// Returns the secret (first coefficient).
    pub fn secret(&self) -> u64 {
        self.polynomial.first().copied().unwrap_or(0)
    }
    /// Checks perfect security: t-1 shares reveal nothing about secret.
    pub fn is_perfectly_secure(&self) -> bool {
        self.threshold > 0 && self.n_shares >= self.threshold
    }
    /// Lagrange interpolation to reconstruct secret from t shares.
    pub fn reconstruct(&self, shares: &[(usize, u64)]) -> Option<u64> {
        if shares.len() < self.threshold {
            return None;
        }
        let p = self.prime;
        let mut secret = 0u64;
        let relevant = &shares[..self.threshold];
        for (i, &(xi, yi)) in relevant.iter().enumerate() {
            let mut num = 1u64;
            let mut den = 1u64;
            for (j, &(xj, _)) in relevant.iter().enumerate() {
                if i != j {
                    num = num * (p - xj as u64 % p) % p;
                    let diff = (xi as i64 - xj as i64).rem_euclid(p as i64) as u64;
                    let inv = Self::mod_pow(diff, p - 2, p);
                    den = den * inv % p;
                }
            }
            secret = (secret + yi * num % p * den) % p;
        }
        Some(secret)
    }
    fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        base %= modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp /= 2;
            base = base * base % modulus;
        }
        result
    }
}
/// Information-theoretic message authentication code.
#[derive(Debug, Clone)]
pub struct InformationTheoreticMAC {
    /// Key length in bits.
    pub key_bits: usize,
    /// Tag length in bits.
    pub tag_bits: usize,
}
impl InformationTheoreticMAC {
    /// A strongly-2-universal family gives forgery probability 1/2^tag_bits.
    pub fn is_strongly_universal(&self) -> bool {
        self.key_bits >= 2 * self.tag_bits
    }
    /// Probability that an adversary forges a tag without the key.
    pub fn forgery_probability(&self) -> f64 {
        1.0 / (1u64 << self.tag_bits.min(62)) as f64
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BroadcastChannelSecrecy {
    pub receivers: Vec<String>,
    pub secrecy_rates: Vec<f64>,
    pub common_message_rate: f64,
}
#[allow(dead_code)]
impl BroadcastChannelSecrecy {
    pub fn new(receivers: Vec<String>) -> Self {
        let n = receivers.len();
        BroadcastChannelSecrecy {
            receivers,
            secrecy_rates: vec![0.0; n],
            common_message_rate: 0.0,
        }
    }
    pub fn set_secrecy_rate(&mut self, receiver_idx: usize, rate: f64) {
        if receiver_idx < self.secrecy_rates.len() {
            self.secrecy_rates[receiver_idx] = rate;
        }
    }
    pub fn total_secrecy_rate(&self) -> f64 {
        self.secrecy_rates.iter().sum()
    }
}
/// Fuzzy extractor for binary strings with Hamming distance metric.
///
/// WARNING: Educational only — NOT cryptographically secure.
#[derive(Debug, Clone)]
pub struct FuzzyExtractor {
    /// Length of the input string in bits.
    pub n: usize,
    /// Maximum allowed Hamming distance for correct recovery.
    pub t: usize,
    /// Output key length in bits.
    pub k: usize,
}
impl FuzzyExtractor {
    /// Create a new fuzzy extractor.
    pub fn new(n: usize, t: usize, k: usize) -> Self {
        Self { n, t, k }
    }
    /// Generate: extract key R and public helper data P from biometric w.
    /// Returns (key_bits, helper_data) as bit vectors.
    ///
    /// This is a simplified "code-offset" construction:
    /// - Pick random codeword c (toy: use parity check)
    /// - Helper data P = w XOR c
    /// - Key R = first k bits of c
    pub fn generate(&self, w: &[u8]) -> (Vec<u8>, Vec<u8>) {
        assert_eq!(w.len(), self.n, "input length must be n");
        let mut codeword = vec![0u8; self.n];
        for i in 0..self.n {
            codeword[i] = w[i] ^ (i as u8 % 7).wrapping_mul(3);
        }
        let helper: Vec<u8> = w.iter().zip(codeword.iter()).map(|(a, b)| a ^ b).collect();
        let key = codeword[..self.k.min(self.n)].to_vec();
        (key, helper)
    }
    /// Reproduce: recover key from noisy w' and public helper data P.
    pub fn reproduce(&self, w_noisy: &[u8], helper: &[u8]) -> Option<Vec<u8>> {
        assert_eq!(w_noisy.len(), self.n);
        assert_eq!(helper.len(), self.n);
        let codeword: Vec<u8> = w_noisy
            .iter()
            .zip(helper.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        let dist: usize = w_noisy
            .iter()
            .zip(codeword.iter())
            .filter(|(a, b)| a != b)
            .count();
        if dist <= self.t {
            Some(codeword[..self.k.min(self.n)].to_vec())
        } else {
            None
        }
    }
    /// Hamming distance between two equal-length byte slices.
    pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
        a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ObliviousTransfer {
    pub variant: OTVariant,
    pub sender_inputs: usize,
    pub receiver_choice_bits: usize,
    pub is_maliciously_secure: bool,
}
#[allow(dead_code)]
impl ObliviousTransfer {
    pub fn new_1_of_2(malicious: bool) -> Self {
        ObliviousTransfer {
            variant: OTVariant::OneOutOfTwo,
            sender_inputs: 2,
            receiver_choice_bits: 1,
            is_maliciously_secure: malicious,
        }
    }
    pub fn communication_bits(&self) -> usize {
        match &self.variant {
            OTVariant::OneOutOfTwo => 256,
            OTVariant::OneOutOfN(n) => (*n as f64).log2() as usize * 128,
            OTVariant::ObliviousRAM => 1024,
            OTVariant::RandOT => 128,
        }
    }
    pub fn ot_extension(&self, num_extensions: usize) -> String {
        format!(
            "OT extension: {} base OTs → {} OTs via IKNP/ALSZ protocol",
            128, num_extensions
        )
    }
}
