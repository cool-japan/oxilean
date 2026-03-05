//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Linear block code (generic).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LinearCodeExt {
    pub n: usize,
    pub k: usize,
    pub d: usize,
    pub is_systematic: bool,
}
#[allow(dead_code)]
impl LinearCodeExt {
    pub fn new(n: usize, k: usize, d: usize) -> Self {
        LinearCodeExt {
            n,
            k,
            d,
            is_systematic: false,
        }
    }
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    pub fn redundancy(&self) -> usize {
        self.n - self.k
    }
    pub fn satisfies_singleton_bound(&self) -> bool {
        self.d <= self.n - self.k + 1
    }
    pub fn satisfies_hamming_bound(&self) -> bool {
        let t = (self.d - 1) / 2;
        let sphere_size: usize = (0..=t).map(|i| binomial(self.n, i)).sum();
        (1usize << self.k) * sphere_size <= (1usize << self.n)
    }
    pub fn is_perfect(&self) -> bool {
        let t = (self.d - 1) / 2;
        let sphere_size: usize = (0..=t).map(|i| binomial(self.n, i)).sum();
        (1usize << self.k) * sphere_size == (1usize << self.n)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FountainCodeType {
    LT,
    Raptor,
    RaptorQ,
}
/// A compressed sensing measurement matrix Φ ∈ R^{m × n} with entries drawn
/// i.i.d. from N(0, 1/m).
///
/// Used to verify the restricted isometry property (RIP) numerically via
/// column coherence and Gram matrix analysis.
#[derive(Debug, Clone)]
pub struct CSMeasurementMatrix {
    /// Number of measurements (rows).
    pub m: usize,
    /// Ambient dimension (columns).
    pub n: usize,
    /// Matrix entries stored row-major (length m * n).
    pub data: Vec<f64>,
}
impl CSMeasurementMatrix {
    /// Create a measurement matrix with given `data` (row-major, length m*n).
    ///
    /// Entries should be scaled by 1/√m for standard RIP analysis.
    pub fn from_data(m: usize, n: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), m * n, "data length must be m * n");
        CSMeasurementMatrix { m, n, data }
    }
    /// Create an all-zeros measurement matrix of shape m × n.
    pub fn zeros(m: usize, n: usize) -> Self {
        CSMeasurementMatrix {
            m,
            n,
            data: vec![0.0; m * n],
        }
    }
    /// Compute the mutual coherence μ = max_{i≠j} |⟨φ_i, φ_j⟩| / (‖φ_i‖ ‖φ_j‖).
    ///
    /// Lower coherence implies better RIP constants (Babel-function approach).
    pub fn mutual_coherence(&self) -> f64 {
        let col_norms: Vec<f64> = (0..self.n)
            .map(|j| {
                let norm_sq: f64 = (0..self.m).map(|i| self.data[i * self.n + j].powi(2)).sum();
                norm_sq.sqrt()
            })
            .collect();
        let mut max_coh = 0.0f64;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if col_norms[i] < 1e-12 || col_norms[j] < 1e-12 {
                    continue;
                }
                let dot: f64 = (0..self.m)
                    .map(|r| self.data[r * self.n + i] * self.data[r * self.n + j])
                    .sum();
                let coh = (dot / (col_norms[i] * col_norms[j])).abs();
                if coh > max_coh {
                    max_coh = coh;
                }
            }
        }
        max_coh
    }
    /// Apply the matrix to a vector x: y = Φ x.
    pub fn apply(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.n, "apply: x must have length n");
        (0..self.m)
            .map(|i| (0..self.n).map(|j| self.data[i * self.n + j] * x[j]).sum())
            .collect()
    }
}
/// A Reed-Solomon code RS(n, k) simulated over a prime field GF(p).
///
/// Encoding: evaluate a degree-(k-1) polynomial at `n` distinct evaluation points.
/// The field is simulated as arithmetic mod `p` (a prime).
#[derive(Debug, Clone)]
pub struct ReedSolomonCode {
    /// Block length (number of evaluation points).
    pub n: usize,
    /// Dimension (degree + 1 of message polynomial).
    pub k: usize,
    /// Field characteristic (prime p; working in GF(p)).
    pub p: u64,
    /// Evaluation points α_0, …, α_{n-1} in GF(p).
    pub eval_points: Vec<u64>,
}
impl ReedSolomonCode {
    /// Create a Reed-Solomon code RS(n, k) over GF(p) with evaluation points 1, 2, …, n.
    ///
    /// Requires n < p and k ≤ n.
    pub fn new(n: usize, k: usize, p: u64) -> Self {
        assert!(n < p as usize, "n must be less than p");
        assert!(k <= n, "k must be at most n");
        let eval_points: Vec<u64> = (1..=(n as u64)).collect();
        ReedSolomonCode {
            n,
            k,
            p,
            eval_points,
        }
    }
    /// Encode a message polynomial `msg[0] + msg[1]*x + … + msg[k-1]*x^{k-1}`
    /// by evaluating at the `n` field points.
    ///
    /// Returns a codeword of length `n`.
    pub fn encode(&self, message: &[u64]) -> Vec<u64> {
        assert_eq!(
            message.len(),
            self.k,
            "encode: message must have k coefficients"
        );
        self.eval_points
            .iter()
            .map(|&alpha| {
                let mut val = 0u64;
                for &coeff in message.iter().rev() {
                    val = (val * alpha + coeff) % self.p;
                }
                val
            })
            .collect()
    }
    /// Minimum distance of the RS code: d_min = n - k + 1.
    pub fn min_distance(&self) -> usize {
        self.n - self.k + 1
    }
    /// Alias for `min_distance` for compatibility.
    pub fn distance(&self) -> usize {
        self.min_distance()
    }
    /// Number of errors the code can correct: t = ⌊(d_min - 1) / 2⌋.
    pub fn error_correction_capability(&self) -> usize {
        (self.min_distance() - 1) / 2
    }
    /// Alias for `error_correction_capability` for compatibility.
    pub fn error_correction_capacity(&self) -> usize {
        self.error_correction_capability()
    }
    /// Number of erasures the code can correct: n - k.
    pub fn erasure_correction_capacity(&self) -> usize {
        self.n - self.k
    }
    /// Code rate: k / n.
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    /// Whether this RS code meets the Singleton bound (all RS codes do).
    pub fn is_mds(&self) -> bool {
        true
    }
}
/// A binary convolutional encoder with `k_in` input bits per step,
/// `n_out` output bits per step, and constraint length `K`.
///
/// State is `K - 1` bits. Generator polynomials define the taps.
#[derive(Debug, Clone)]
pub struct ConvolutionalEncoder {
    /// Number of input bits per clock cycle.
    pub k_in: usize,
    /// Number of output bits per clock cycle.
    pub n_out: usize,
    /// Constraint length (number of shift-register stages + 1).
    pub constraint_length: usize,
    /// Generator polynomials (one per output bit), each as a bit mask.
    ///
    /// `generators[j]` is a bitmask over the `constraint_length` shift-register stages
    /// for output bit `j`. Bit `i` of the mask means register stage `i` feeds into output `j`.
    pub generators: Vec<u32>,
    /// Current shift register state (`constraint_length - 1` bits).
    state: u32,
}
impl ConvolutionalEncoder {
    /// Create a new convolutional encoder.
    ///
    /// `generators` must have length `n_out`. Each generator is a bitmask of
    /// `constraint_length` bits.
    pub fn new(k_in: usize, n_out: usize, constraint_length: usize, generators: Vec<u32>) -> Self {
        assert_eq!(
            generators.len(),
            n_out,
            "generators must have n_out entries"
        );
        ConvolutionalEncoder {
            k_in,
            n_out,
            constraint_length,
            generators,
            state: 0,
        }
    }
    /// Reset encoder state to zero.
    pub fn reset(&mut self) {
        self.state = 0;
    }
    /// Encode a single input bit, returning `n_out` output bits.
    pub fn encode_bit(&mut self, input: bool) -> Vec<bool> {
        let new_state = if input {
            (self.state >> 1) | (1u32 << (self.constraint_length - 2))
        } else {
            self.state >> 1
        };
        let outputs: Vec<bool> = self
            .generators
            .iter()
            .map(|&g| {
                let full_reg = if input {
                    (1u32 << (self.constraint_length - 1)) | self.state
                } else {
                    self.state
                };
                (full_reg & g).count_ones() % 2 == 1
            })
            .collect();
        self.state = new_state;
        outputs
    }
    /// Encode a sequence of input bits.
    ///
    /// Returns a flat vector of output bits (length = `input.len() * n_out`).
    pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
        input.iter().flat_map(|&bit| self.encode_bit(bit)).collect()
    }
    /// Flush the encoder by feeding `constraint_length - 1` zero bits (tail bits),
    /// returning the corresponding output bits.
    pub fn flush(&mut self) -> Vec<bool> {
        let tail = vec![false; self.constraint_length - 1];
        self.encode(&tail)
    }
}
/// GF(2^m) represented with a primitive polynomial.
///
/// Elements are stored as u32 with the natural polynomial basis.
/// Only supports m ≤ 16 for practical use.
#[derive(Debug, Clone)]
pub struct GF2m {
    /// Extension degree m.
    pub m: u32,
    /// Number of elements: 2^m.
    pub size: u32,
    /// Primitive polynomial (as integer, e.g., x^3+x+1 = 0b1011 = 11).
    pub prim_poly: u32,
    /// Log table: `log_table[i]` = discrete log of element i (0 = undefined).
    log_table: Vec<i32>,
    /// Exp table: `exp_table[i]` = α^i.
    exp_table: Vec<u32>,
}
impl GF2m {
    /// Create GF(2^m) using the given primitive polynomial.
    pub fn new(m: u32, prim_poly: u32) -> Self {
        let size = 1u32 << m;
        let mut log_table = vec![-1i32; size as usize];
        let mut exp_table = vec![0u32; (2 * size - 1) as usize];
        let mut x = 1u32;
        for i in 0..(size - 1) {
            exp_table[i as usize] = x;
            log_table[x as usize] = i as i32;
            x <<= 1;
            if x >= size {
                x ^= prim_poly;
            }
        }
        for i in (size - 1)..(2 * size - 1) {
            exp_table[i as usize] = exp_table[(i - (size - 1)) as usize];
        }
        GF2m {
            m,
            size,
            prim_poly,
            log_table,
            exp_table,
        }
    }
    /// Add two field elements (XOR).
    pub fn add(&self, a: u32, b: u32) -> u32 {
        a ^ b
    }
    /// Multiply two field elements.
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            return 0;
        }
        let log_a = self.log_table[a as usize];
        let log_b = self.log_table[b as usize];
        self.exp_table[((log_a + log_b) as u32 % (self.size - 1)) as usize]
    }
    /// Exponentiate: α^i.
    pub fn pow(&self, i: u32) -> u32 {
        if self.size <= 1 {
            return 0;
        }
        self.exp_table[(i % (self.size - 1)) as usize]
    }
    /// Inverse of a nonzero element.
    pub fn inv(&self, a: u32) -> u32 {
        assert_ne!(a, 0, "GF2m::inv: zero has no inverse");
        let log_a = self.log_table[a as usize];
        let order = (self.size - 1) as i32;
        self.exp_table[((order - log_a).rem_euclid(order)) as usize]
    }
}
/// Tensor (product) code C1 ⊗ C2 from two linear codes.
///
/// A codeword of the product code is an `n1 × n2` binary matrix where
/// each row is a codeword of C1 and each column is a codeword of C2.
#[derive(Debug, Clone, Copy)]
pub struct ProductCode {
    /// Parameters of the row code C1.
    pub c1: LinearCode,
    /// Parameters of the column code C2.
    pub c2: LinearCode,
}
impl ProductCode {
    /// Construct the product code C1 ⊗ C2.
    pub fn new(c1: LinearCode, c2: LinearCode) -> Self {
        ProductCode { c1, c2 }
    }
    /// Block length of the product code: n1 * n2.
    pub fn block_length(&self) -> usize {
        self.c1.n * self.c2.n
    }
    /// Dimension of the product code: k1 * k2.
    pub fn dimension(&self) -> usize {
        self.c1.k * self.c2.k
    }
    /// Minimum distance of the product code: d1 * d2.
    pub fn min_distance(&self) -> usize {
        self.c1.d_min * self.c2.d_min
    }
    /// Code rate: (k1 * k2) / (n1 * n2).
    pub fn rate(&self) -> f64 {
        self.dimension() as f64 / self.block_length() as f64
    }
}
/// Polar code construction for the binary erasure channel (BEC).
///
/// Given block length `N = 2^n` and erasure probability `epsilon`, computes
/// the synthetic channel erasure probabilities after polarization, then selects
/// the `k` best (least-noisy) channels as information bit positions.
#[derive(Debug, Clone)]
pub struct PolarCodeBEC {
    /// Log2 of block length (n such that N = 2^n).
    pub n: u32,
    /// Erasure probability of the underlying BEC.
    pub epsilon: f64,
    /// Sorted indices of the k best synthetic channels (information set).
    pub info_bits: Vec<usize>,
    /// Erasure probabilities of all N synthetic channels.
    pub erasure_probs: Vec<f64>,
}
impl PolarCodeBEC {
    /// Construct the polar code for a BEC(epsilon) with block length N = 2^n.
    ///
    /// Selects the `k` synthetic channels with the smallest erasure probability
    /// as information bit positions (rest are frozen to 0).
    pub fn new(n: u32, k: usize, epsilon: f64) -> Self {
        let big_n = 1usize << n;
        assert!(k <= big_n, "k must be ≤ N");
        let mut probs = vec![epsilon; 1];
        for _ in 0..n {
            let mut next = Vec::with_capacity(probs.len() * 2);
            for &z in &probs {
                next.push(2.0 * z - z * z);
                next.push(z * z);
            }
            probs = next;
        }
        assert_eq!(probs.len(), big_n);
        let mut indexed: Vec<(usize, f64)> = probs.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let info_bits: Vec<usize> = indexed[..k].iter().map(|&(i, _)| i).collect();
        PolarCodeBEC {
            n,
            epsilon,
            info_bits,
            erasure_probs: probs,
        }
    }
    /// Block length N = 2^n.
    pub fn block_length(&self) -> usize {
        1usize << self.n
    }
    /// Number of information bits k.
    pub fn dimension(&self) -> usize {
        self.info_bits.len()
    }
    /// Code rate k / N.
    pub fn rate(&self) -> f64 {
        self.dimension() as f64 / self.block_length() as f64
    }
    /// Fraction of polarized channels with erasure probability < threshold.
    pub fn fraction_good(&self, threshold: f64) -> f64 {
        let good = self
            .erasure_probs
            .iter()
            .filter(|&&z| z < threshold)
            .count();
        good as f64 / self.block_length() as f64
    }
}
/// Capacity estimator for BICM with Gray-coded M-QAM.
///
/// BICM treats each coded bit as independently modulated over a separate
/// binary-input channel. The BICM capacity is the sum of individual bit
/// channel capacities.
#[derive(Debug, Clone, Copy)]
pub struct BICMCapacityEstimator {
    /// Modulation order M (number of constellation points; must be a power of 2).
    pub modulation_order: usize,
    /// Number of bits per symbol: b = log2(M).
    pub bits_per_symbol: usize,
}
impl BICMCapacityEstimator {
    /// Create a BICM capacity estimator for M-QAM / M-PSK.
    ///
    /// `modulation_order` must be a power of 2 (e.g., 4 for QPSK, 16, 64, …).
    pub fn new(modulation_order: usize) -> Self {
        assert!(
            modulation_order.is_power_of_two() && modulation_order >= 2,
            "modulation_order must be a power of 2 ≥ 2"
        );
        let bits_per_symbol = modulation_order.trailing_zeros() as usize;
        BICMCapacityEstimator {
            modulation_order,
            bits_per_symbol,
        }
    }
    /// Spectral efficiency upper bound: b bits/channel use (uncoded AWGN Shannon limit
    /// for the modulation).
    pub fn spectral_efficiency_upper_bound(&self) -> f64 {
        self.bits_per_symbol as f64
    }
    /// Approximate BICM capacity at a given SNR (dB) using the Gaussian integral
    /// approximation for high-order QAM.
    ///
    /// For BPSK/QPSK this uses the exact binary channel formula.
    /// For higher orders, returns a simplified lower bound based on Q-function error floor.
    pub fn approximate_capacity_db(&self, snr_db: f64) -> f64 {
        let snr_lin = 10.0f64.powf(snr_db / 10.0);
        if self.bits_per_symbol == 1 {
            let q_arg = (2.0 * snr_lin).sqrt();
            let q_val = 0.5 * erfc_approx(q_arg / 2.0_f64.sqrt());
            1.0 - ChannelCapacity::binary_entropy(q_val)
        } else {
            let snr_per_bit = snr_lin / self.bits_per_symbol as f64;
            let awgn = ChannelCapacity::awgn_capacity(snr_per_bit);
            (awgn * self.bits_per_symbol as f64).min(self.bits_per_symbol as f64)
        }
    }
}
/// An (n, k, d) linear code specified by its parameters.
///
/// - `n`: code length (block length)
/// - `k`: dimension (number of information bits)
/// - `d_min`: minimum Hamming distance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearCode {
    /// Block length.
    pub n: usize,
    /// Dimension (message length).
    pub k: usize,
    /// Minimum distance.
    pub d_min: usize,
}
impl LinearCode {
    /// Create an (n, k, d) linear code.
    pub fn new(n: usize, k: usize, d: usize) -> Self {
        LinearCode { n, k, d_min: d }
    }
    /// Code rate: k / n.
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    /// Redundancy: n - k (number of parity-check bits).
    pub fn redundancy(&self) -> usize {
        self.n - self.k
    }
    /// Number of errors the code can correct: ⌊(d - 1) / 2⌋.
    pub fn corrects_errors(&self) -> usize {
        (self.d_min.saturating_sub(1)) / 2
    }
    /// Number of errors the code can detect: d - 1.
    pub fn detects_errors(&self) -> usize {
        self.d_min.saturating_sub(1)
    }
    /// Returns `true` if d = n - k + 1 (Singleton bound tight, MDS code).
    pub fn meets_singleton_bound(&self) -> bool {
        self.d_min == self.n - self.k + 1
    }
    /// Returns `true` if the code meets the Hamming (sphere-packing) bound with equality
    /// (i.e., it is a perfect code).
    ///
    /// The Hamming bound states: `|C| ≤ 2^n / V(n, t)` where `t = ⌊(d-1)/2⌋` and
    /// `V(n, t) = Σ_{i=0}^{t} C(n, i)`.
    pub fn meets_hamming_bound(&self) -> bool {
        let t = self.corrects_errors();
        let volume = hamming_ball_volume(self.n, t);
        let codewords = 1usize << self.k;
        let capacity = (1usize << self.n) / volume;
        codewords == capacity
    }
    /// Returns true if the code satisfies the Singleton bound: d <= n - k + 1.
    pub fn satisfies_singleton_bound(&self) -> bool {
        self.d_min <= self.n - self.k + 1
    }
    /// Returns true if the code satisfies the Hamming bound: |C| * V(n,t) <= 2^n.
    pub fn satisfies_hamming_bound(&self) -> bool {
        let t = self.corrects_errors();
        let volume = hamming_ball_volume(self.n, t);
        let codewords = 1usize << self.k;
        codewords * volume <= (1usize << self.n)
    }
    /// Whether this code is perfect (meets the Hamming bound with equality).
    pub fn is_perfect(&self) -> bool {
        self.meets_hamming_bound()
    }
}
/// The binary Hamming (7, 4, 3) code with parameter r = 3.
///
/// Generator matrix (standard form):
/// ```text
/// G = [I_4 | P^T]  where P is the parity part
/// ```
/// Parity-check matrix:
/// ```text
/// H = [P | I_3]
/// ```
#[derive(Debug, Clone)]
pub struct HammingCode74 {
    pub inner: LinearCodeMatrix,
}
impl HammingCode74 {
    /// Construct the standard binary Hamming (7,4,3) code.
    pub fn new() -> Self {
        let h0 = BinaryVector::from_bits(vec![false, false, false, true, true, true, true]);
        let h1 = BinaryVector::from_bits(vec![false, true, true, false, false, true, true]);
        let h2 = BinaryVector::from_bits(vec![true, false, true, false, true, false, true]);
        let g0 = BinaryVector::from_bits(vec![true, false, false, false, false, true, true]);
        let g1 = BinaryVector::from_bits(vec![false, true, false, false, true, false, true]);
        let g2 = BinaryVector::from_bits(vec![false, false, true, false, true, true, false]);
        let g3 = BinaryVector::from_bits(vec![false, false, false, true, true, true, true]);
        HammingCode74 {
            inner: LinearCodeMatrix::new(7, 4, vec![g0, g1, g2, g3], vec![h0, h1, h2]),
        }
    }
    /// Encode a 4-bit message into a 7-bit codeword.
    pub fn encode(&self, message: &BinaryVector) -> BinaryVector {
        self.inner.encode(message)
    }
    /// Compute the 3-bit syndrome of a received 7-bit word.
    pub fn syndrome(&self, received: &BinaryVector) -> BinaryVector {
        self.inner.syndrome(received)
    }
    /// Single-error correct a received 7-bit word.
    ///
    /// The syndrome `(s0, s1, s2)` is matched against each column of H to identify
    /// the error position (0-indexed). Returns the corrected codeword (unchanged if
    /// syndrome is zero).
    pub fn correct(&self, received: &BinaryVector) -> BinaryVector {
        let syn = self.syndrome(received);
        if syn.hamming_weight() == 0 {
            return received.clone();
        }
        let error_pos = self.inner.parity_check.iter().enumerate().fold(
            None::<usize>,
            |found, (row_idx, h_row)| {
                let _ = (row_idx, h_row);
                found
            },
        );
        let n = received.bits.len();
        let mut corrected = received.clone();
        for j in 0..n {
            let col_syn: Vec<bool> = self
                .inner
                .parity_check
                .iter()
                .map(|h_row| h_row.bits[j])
                .collect();
            if col_syn == syn.bits {
                corrected.bits[j] ^= true;
                break;
            }
        }
        let _ = error_pos;
        corrected
    }
}
/// Reed-Solomon code parameters.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RSCodeExt {
    pub n: usize,
    pub k: usize,
    pub q: usize,
}
#[allow(dead_code)]
impl RSCodeExt {
    pub fn new(n: usize, k: usize, q: usize) -> Self {
        assert!(k <= n && n < q);
        RSCodeExt { n, k, q }
    }
    pub fn distance(&self) -> usize {
        self.n - self.k + 1
    }
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    pub fn is_mds(&self) -> bool {
        true
    }
    pub fn error_correction_capacity(&self) -> usize {
        (self.distance() - 1) / 2
    }
    pub fn erasure_correction_capacity(&self) -> usize {
        self.distance() - 1
    }
}
/// Turbo code encoder parameters.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TurboCode {
    pub constituent_rate: (usize, usize),
    pub interleaver_size: usize,
    pub n_iterations: usize,
}
#[allow(dead_code)]
impl TurboCode {
    pub fn new(rate_n: usize, rate_k: usize, interleaver: usize, iters: usize) -> Self {
        TurboCode {
            constituent_rate: (rate_n, rate_k),
            interleaver_size: interleaver,
            n_iterations: iters,
        }
    }
    pub fn standard_3gpp(block_size: usize) -> Self {
        TurboCode::new(3, 1, block_size, 8)
    }
    pub fn overall_rate(&self) -> f64 {
        let (n, k) = self.constituent_rate;
        k as f64 / (2 * n + 1) as f64
    }
    pub fn approaches_shannon_capacity(&self) -> bool {
        self.n_iterations >= 10
    }
}
/// A vector over GF(2) (binary field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryVector {
    /// The bits of the vector.
    pub bits: Vec<bool>,
}
impl BinaryVector {
    /// Create an all-zero binary vector of length `n`.
    pub fn new(n: usize) -> Self {
        BinaryVector {
            bits: vec![false; n],
        }
    }
    /// Create a binary vector from a given bit sequence.
    pub fn from_bits(bits: Vec<bool>) -> Self {
        BinaryVector { bits }
    }
    /// Hamming weight: number of 1-bits.
    pub fn hamming_weight(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }
    /// Hamming distance to another vector.
    ///
    /// Panics if vectors have different lengths.
    pub fn hamming_distance(&self, other: &BinaryVector) -> usize {
        self.xor(other).hamming_weight()
    }
    /// Bitwise XOR (component-wise addition mod 2).
    ///
    /// Panics if vectors have different lengths.
    pub fn xor(&self, other: &BinaryVector) -> BinaryVector {
        assert_eq!(
            self.bits.len(),
            other.bits.len(),
            "BinaryVector::xor: length mismatch"
        );
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(&a, &b)| a ^ b)
            .collect();
        BinaryVector { bits }
    }
    /// Inner product mod 2.
    ///
    /// Panics if vectors have different lengths.
    pub fn dot(&self, other: &BinaryVector) -> bool {
        assert_eq!(
            self.bits.len(),
            other.bits.len(),
            "BinaryVector::dot: length mismatch"
        );
        self.bits
            .iter()
            .zip(&other.bits)
            .filter(|(&a, &b)| a && b)
            .count()
            % 2
            == 1
    }
}
/// Polar code (Arıkan's construction).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolarCode {
    pub n: usize,
    pub k: usize,
}
#[allow(dead_code)]
impl PolarCode {
    pub fn new(n: usize, k: usize) -> Self {
        assert!(n.is_power_of_two() && k <= n);
        PolarCode { n, k }
    }
    pub fn rate(&self) -> f64 {
        self.k as f64 / self.n as f64
    }
    pub fn is_capacity_achieving(&self) -> bool {
        true
    }
    pub fn successive_cancellation_complexity(&self) -> usize {
        self.n * (self.n as f64).log2() as usize
    }
    pub fn distance_lower_bound(&self) -> usize {
        (self.n / self.k).max(2)
    }
}
/// A burst error detector based on a cyclic code (Fire code simulation).
///
/// Detects burst errors of length ≤ `burst_len` in a block of `n` bits.
/// In practice this is a CRC-style shift-register check.
#[derive(Debug, Clone)]
pub struct BurstErrorDetector {
    /// Maximum detectable burst length.
    pub burst_len: usize,
    /// Block length.
    pub n: usize,
    /// Generator polynomial (bit vector, degree = burst_len).
    pub generator: Vec<bool>,
}
impl BurstErrorDetector {
    /// Create a burst error detector with the given generator polynomial.
    ///
    /// `generator` should be a bit vector of length `burst_len + 1`
    /// representing the generator polynomial (LSB = constant term).
    pub fn new(n: usize, burst_len: usize, generator: Vec<bool>) -> Self {
        assert_eq!(
            generator.len(),
            burst_len + 1,
            "generator must have burst_len + 1 bits"
        );
        BurstErrorDetector {
            burst_len,
            n,
            generator,
        }
    }
    /// Compute the remainder of `codeword` divided by the generator polynomial (CRC).
    ///
    /// Returns the syndrome (remainder). Zero means no detected burst error.
    pub fn compute_syndrome(&self, codeword: &BinaryVector) -> BinaryVector {
        assert_eq!(
            codeword.bits.len(),
            self.n,
            "compute_syndrome: codeword length mismatch"
        );
        let mut remainder = vec![false; self.burst_len];
        for &bit in &codeword.bits {
            let feedback = bit ^ remainder[0];
            for i in 0..self.burst_len - 1 {
                remainder[i] = remainder[i + 1] ^ (feedback && self.generator[i + 1]);
            }
            remainder[self.burst_len - 1] = feedback && self.generator[self.burst_len];
        }
        BinaryVector::from_bits(remainder)
    }
    /// Returns `true` if no burst error of length ≤ `burst_len` is detected.
    pub fn is_valid(&self, codeword: &BinaryVector) -> bool {
        self.compute_syndrome(codeword).hamming_weight() == 0
    }
}
/// A binary linear code represented by explicit generator and parity-check matrices.
///
/// The generator matrix `G` has shape `k × n` (stored as k row-vectors of length n).
/// The parity-check matrix `H` has shape `(n-k) × n`.
#[derive(Debug, Clone)]
pub struct LinearCodeMatrix {
    /// Block length n.
    pub n: usize,
    /// Dimension k.
    pub k: usize,
    /// Generator matrix G: `k` rows of length `n`.
    pub generator: Vec<BinaryVector>,
    /// Parity-check matrix H: `(n-k)` rows of length `n`.
    pub parity_check: Vec<BinaryVector>,
}
impl LinearCodeMatrix {
    /// Create a `LinearCodeMatrix` from explicit `generator` and `parity_check` matrices.
    ///
    /// Panics if dimensions are inconsistent.
    pub fn new(
        n: usize,
        k: usize,
        generator: Vec<BinaryVector>,
        parity_check: Vec<BinaryVector>,
    ) -> Self {
        assert_eq!(generator.len(), k, "generator must have k rows");
        assert_eq!(parity_check.len(), n - k, "parity_check must have n-k rows");
        for row in &generator {
            assert_eq!(row.bits.len(), n, "generator row length must be n");
        }
        for row in &parity_check {
            assert_eq!(row.bits.len(), n, "parity_check row length must be n");
        }
        LinearCodeMatrix {
            n,
            k,
            generator,
            parity_check,
        }
    }
    /// Encode a message `m` (length k) using the generator matrix G.
    ///
    /// Returns the codeword `c = m * G` (binary matrix-vector product).
    pub fn encode(&self, message: &BinaryVector) -> BinaryVector {
        assert_eq!(
            message.bits.len(),
            self.k,
            "encode: message length must equal k"
        );
        let mut codeword = BinaryVector::new(self.n);
        for (i, &bit) in message.bits.iter().enumerate() {
            if bit {
                codeword = codeword.xor(&self.generator[i]);
            }
        }
        codeword
    }
    /// Compute the syndrome `s = H * r^T` of a received word `r`.
    ///
    /// Returns a `BinaryVector` of length `n - k`. Zero syndrome means no detected error.
    pub fn syndrome(&self, received: &BinaryVector) -> BinaryVector {
        assert_eq!(
            received.bits.len(),
            self.n,
            "syndrome: received length must equal n"
        );
        let s: Vec<bool> = self
            .parity_check
            .iter()
            .map(|h_row| h_row.dot(received))
            .collect();
        BinaryVector::from_bits(s)
    }
    /// Returns `true` if the received word is a valid codeword (zero syndrome).
    pub fn is_codeword(&self, received: &BinaryVector) -> bool {
        self.syndrome(received).hamming_weight() == 0
    }
}
/// Reed-Muller code RM(r, m) parameters.
///
/// - Block length: `n = 2^m`
/// - Dimension: `k = Σ_{i=0}^{r} C(m, i)`
/// - Minimum distance: `d = 2^{m-r}`
#[derive(Debug, Clone, Copy)]
pub struct ReedMullerCode {
    /// Order of the Reed-Muller code (0 ≤ r ≤ m).
    pub r: u32,
    /// Number of variables (log2 of block length).
    pub m: u32,
}
impl ReedMullerCode {
    /// Create the Reed-Muller code RM(r, m).
    ///
    /// Requires 0 ≤ r ≤ m.
    pub fn new(r: u32, m: u32) -> Self {
        assert!(r <= m, "ReedMullerCode: r must be ≤ m");
        ReedMullerCode { r, m }
    }
    /// Block length n = 2^m.
    pub fn block_length(&self) -> usize {
        1usize << self.m
    }
    /// Minimum distance d = 2^{m-r}.
    pub fn min_distance(&self) -> usize {
        1usize << (self.m - self.r)
    }
    /// Dimension k = Σ_{i=0}^{r} C(m, i).
    pub fn dimension(&self) -> usize {
        let mut k = 0usize;
        let mut binom = 1usize;
        for i in 0..=(self.r as usize) {
            k += binom;
            if i < self.r as usize {
                binom = binom * (self.m as usize - i) / (i + 1);
            }
        }
        k
    }
    /// Code rate k / n.
    pub fn rate(&self) -> f64 {
        self.dimension() as f64 / self.block_length() as f64
    }
    /// Returns `true` if this is the first-order RM code (r = 1),
    /// which is equivalent to a simplex code's dual and corrects t = (d-1)/2 errors.
    pub fn is_first_order(&self) -> bool {
        self.r == 1
    }
    /// Error-correction capability t = ⌊(d - 1) / 2⌋.
    pub fn error_correction_capability(&self) -> usize {
        (self.min_distance() - 1) / 2
    }
}
/// Fountain code (Luby transform / raptor).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FountainCode {
    pub k_input: usize,
    pub overhead: f64,
    pub code_type: FountainCodeType,
}
#[allow(dead_code)]
impl FountainCode {
    pub fn new(k: usize, overhead: f64, ct: FountainCodeType) -> Self {
        FountainCode {
            k_input: k,
            overhead,
            code_type: ct,
        }
    }
    pub fn lt_code(k: usize) -> Self {
        FountainCode::new(k, 0.05, FountainCodeType::LT)
    }
    pub fn raptor_code(k: usize) -> Self {
        FountainCode::new(k, 0.01, FountainCodeType::Raptor)
    }
    pub fn n_symbols_to_decode(&self) -> usize {
        (self.k_input as f64 * (1.0 + self.overhead)).ceil() as usize
    }
    pub fn is_rateless(&self) -> bool {
        true
    }
    pub fn decoding_complexity(&self) -> usize {
        match self.code_type {
            FountainCodeType::LT => self.k_input * (self.k_input as f64).ln() as usize,
            FountainCodeType::Raptor | FountainCodeType::RaptorQ => self.k_input,
        }
    }
}
/// A binary Hamming code with parameter `r`.
///
/// The code has parameters `(n, k, d) = (2^r - 1, 2^r - r - 1, 3)`.
#[derive(Debug, Clone, Copy)]
pub struct HammingCode {
    /// The Hamming code parameter (r ≥ 2).
    pub r: u32,
}
impl HammingCode {
    /// Create the Hamming code `Ham(r, 2)`.
    pub fn new(r: u32) -> Self {
        HammingCode { r }
    }
    /// Convert to `LinearCode` with the standard Hamming parameters.
    pub fn to_linear_code(&self) -> LinearCode {
        let n = (1u64 << self.r) as usize - 1;
        let k = n - self.r as usize;
        LinearCode::new(n, k, 3)
    }
    /// Syndrome decode a received word (stub).
    ///
    /// Returns the corrected codeword. For a single-error-correcting code,
    /// the syndrome identifies the error position.
    pub fn syndrome_decode(&self, received: &BinaryVector) -> BinaryVector {
        received.clone()
    }
}
/// Channel capacity formulas for standard channels.
pub struct ChannelCapacity;
impl ChannelCapacity {
    /// Binary symmetric channel (BSC) capacity: `C = 1 - H_b(p)`.
    ///
    /// `p` is the crossover (bit-flip) probability, `0 ≤ p ≤ 1`.
    pub fn bsc_capacity(p: f64) -> f64 {
        1.0 - Self::binary_entropy(p)
    }
    /// Binary erasure channel (BEC) capacity: `C = 1 - ε`.
    ///
    /// `epsilon` is the erasure probability, `0 ≤ ε ≤ 1`.
    pub fn bec_capacity(epsilon: f64) -> f64 {
        1.0 - epsilon
    }
    /// AWGN channel Shannon capacity: `C = ½ log₂(1 + SNR)`.
    ///
    /// `snr` is the signal-to-noise ratio (linear scale, not dB).
    pub fn awgn_capacity(snr: f64) -> f64 {
        0.5 * (1.0 + snr).log2()
    }
    /// Binary entropy function: `H(p) = -p log₂ p - (1-p) log₂(1-p)`.
    ///
    /// Returns 0 for `p = 0` or `p = 1`, and 1 for `p = 0.5`.
    pub fn binary_entropy(p: f64) -> f64 {
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
    }
    /// q-ary entropy function for a q-ary symmetric channel.
    ///
    /// `H_q(p) = -p * log_q(p/(q-1)) - (1-p) * log_q(1-p)`
    pub fn q_ary_entropy(p: f64, q: usize) -> f64 {
        if p <= 0.0 || p >= 1.0 || q < 2 {
            return 0.0;
        }
        let q_f = q as f64;
        let log_q = (q_f).log2();
        let h = -p * (p / (q_f - 1.0)).log2() / log_q - (1.0 - p) * (1.0 - p).log2() / log_q;
        h.max(0.0)
    }
}
