//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Identifies BSC or BEC channel type.
pub enum ChannelKind {
    /// Binary Symmetric Channel with flip probability p.
    Bsc(f64),
    /// Binary Erasure Channel with erasure probability epsilon.
    Bec(f64),
}
/// Slepian-Wolf distributed source coding rate region.
///
/// For correlated sources (X,Y), the achievable rate region is:
///   R1 ≥ H(X|Y),  R2 ≥ H(Y|X),  R1+R2 ≥ H(X,Y)
///
/// This struct computes the corner points of the Slepian-Wolf rate region
/// from a joint distribution table.
pub struct SlepianWolfCoder {
    /// Joint distribution table: joint[i][j] = P(X=i, Y=j).
    pub joint: Vec<Vec<f64>>,
}
impl SlepianWolfCoder {
    /// Create a new Slepian-Wolf coder from a joint distribution.
    pub fn new(joint: Vec<Vec<f64>>) -> Self {
        SlepianWolfCoder { joint }
    }
    /// Compute H(X|Y): conditional entropy of X given Y.
    pub fn h_x_given_y(&self) -> f64 {
        conditional_entropy(&self.joint)
    }
    /// Compute H(Y|X): conditional entropy of Y given X.
    ///
    /// Uses the transposed joint table.
    pub fn h_y_given_x(&self) -> f64 {
        if self.joint.is_empty() {
            return 0.0;
        }
        let cols = self.joint[0].len();
        let rows = self.joint.len();
        let transposed: Vec<Vec<f64>> = (0..cols)
            .map(|j| {
                (0..rows)
                    .map(|i| self.joint[i].get(j).copied().unwrap_or(0.0))
                    .collect()
            })
            .collect();
        conditional_entropy(&transposed)
    }
    /// Compute H(X,Y): joint entropy.
    pub fn h_xy(&self) -> f64 {
        joint_entropy(&self.joint)
    }
    /// Return the three corner-point rates of the Slepian-Wolf rate region:
    ///   (H(X|Y), H(X,Y) - H(X|Y)),
    ///   (H(X,Y) - H(Y|X), H(Y|X)),
    ///
    /// These are the two corner points of the pentagon boundary.
    pub fn corner_rates(&self) -> ((f64, f64), (f64, f64)) {
        let r1_min = self.h_x_given_y();
        let r2_min = self.h_y_given_x();
        let h_xy = self.h_xy();
        let corner1 = (r1_min, h_xy - r1_min);
        let corner2 = (h_xy - r2_min, r2_min);
        (corner1, corner2)
    }
    /// Check whether a rate pair (r1, r2) is achievable in the Slepian-Wolf region.
    pub fn is_achievable(&self, r1: f64, r2: f64) -> bool {
        let h_x_y = self.h_x_given_y();
        let h_y_x = self.h_y_given_x();
        let h_xy = self.h_xy();
        r1 >= h_x_y && r2 >= h_y_x && (r1 + r2) >= h_xy
    }
}
/// Wiretap channel secrecy capacity computation.
///
/// For the degraded wiretap channel (X → Y → Z), the secrecy capacity is:
///   C_s = max_{p(x)} [I(X;Y) - I(X;Z)]
///
/// This implementation evaluates secrecy rate for a fixed input distribution
/// over both the legitimate channel W_Y and the eavesdropper channel W_Z.
pub struct WiretapChannel {
    /// Legitimate channel transition matrix: wy[x][y] = P(Y=y|X=x).
    pub wy: Vec<Vec<f64>>,
    /// Eavesdropper channel transition matrix: wz[x][z] = P(Z=z|X=x).
    pub wz: Vec<Vec<f64>>,
}
impl WiretapChannel {
    /// Create a wiretap channel with given legitimate and eavesdropper channels.
    pub fn new(wy: Vec<Vec<f64>>, wz: Vec<Vec<f64>>) -> Self {
        WiretapChannel { wy, wz }
    }
    /// Compute I(X;Y) for a given input distribution q.
    fn mutual_info_channel(channel: &[Vec<f64>], q: &[f64]) -> f64 {
        let x_size = channel.len();
        if x_size == 0 {
            return 0.0;
        }
        let y_size = channel[0].len();
        let mut ry = vec![0.0f64; y_size];
        for x in 0..x_size {
            for y in 0..y_size {
                ry[y] += q[x] * channel[x][y];
            }
        }
        let mut mi = 0.0f64;
        for x in 0..x_size {
            for y in 0..y_size {
                let w = channel[x][y];
                if w > 0.0 && ry[y] > 0.0 {
                    mi += q[x] * w * (w / ry[y]).log2();
                }
            }
        }
        mi
    }
    /// Compute the secrecy rate I(X;Y) - I(X;Z) for a given input distribution.
    pub fn secrecy_rate(&self, q: &[f64]) -> f64 {
        let i_xy = Self::mutual_info_channel(&self.wy, q);
        let i_xz = Self::mutual_info_channel(&self.wz, q);
        (i_xy - i_xz).max(0.0)
    }
    /// Estimate secrecy capacity via a simple grid search over input distributions.
    ///
    /// For binary input (|X|=2), sweeps p from 0 to 1 and returns the maximum
    /// secrecy rate found. For larger alphabets, uses uniform distribution as
    /// an approximation.
    pub fn estimate_secrecy_capacity(&self) -> f64 {
        let x_size = self.wy.len();
        if x_size == 0 {
            return 0.0;
        }
        if x_size == 2 {
            let steps = 1000;
            (0..=steps)
                .map(|k| {
                    let p = k as f64 / steps as f64;
                    let q = vec![p, 1.0 - p];
                    self.secrecy_rate(&q)
                })
                .fold(f64::NEG_INFINITY, f64::max)
        } else {
            let q = vec![1.0 / x_size as f64; x_size];
            self.secrecy_rate(&q)
        }
    }
}
/// Rate-distortion theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RateDistortion {
    pub source_entropy_bits: f64,
    pub distortion_threshold: f64,
    pub distortion_type: DistortionMeasure,
}
#[allow(dead_code)]
impl RateDistortion {
    pub fn new(h: f64, d: f64, dm: DistortionMeasure) -> Self {
        RateDistortion {
            source_entropy_bits: h,
            distortion_threshold: d,
            distortion_type: dm,
        }
    }
    /// Rate-distortion function for binary source with Hamming distortion.
    /// R(D) = H_b(p) - H_b(D) for 0 ≤ D ≤ p ≤ 1/2.
    pub fn binary_rd_function(p: f64, d: f64) -> f64 {
        if d >= p {
            return 0.0;
        }
        h_b(p) - h_b(d)
    }
    pub fn minimum_rate(&self) -> f64 {
        (self.source_entropy_bits - h_b(self.distortion_threshold)).max(0.0)
    }
}
/// Simulate a Binary Symmetric Channel (BSC) or Binary Erasure Channel (BEC).
pub struct ChannelSimulator {
    /// Channel type and flip/erasure probability.
    pub kind: ChannelKind,
}
impl ChannelSimulator {
    /// Create a BSC with flip probability `p`.
    pub fn new_bsc(p: f64) -> Self {
        ChannelSimulator {
            kind: ChannelKind::Bsc(p),
        }
    }
    /// Create a BEC with erasure probability `epsilon`.
    pub fn new_bec(epsilon: f64) -> Self {
        ChannelSimulator {
            kind: ChannelKind::Bec(epsilon),
        }
    }
    /// Return the Shannon capacity of this channel.
    pub fn capacity(&self) -> f64 {
        match self.kind {
            ChannelKind::Bsc(p) => bsc_capacity(p),
            ChannelKind::Bec(eps) => bec_capacity(eps),
        }
    }
    /// Compute the empirical bit-error rate for BSC given transition probability.
    ///
    /// For BSC(p), the BER is simply p.
    pub fn ber(&self) -> f64 {
        match self.kind {
            ChannelKind::Bsc(p) => p,
            ChannelKind::Bec(eps) => eps,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DistortionMeasure {
    Hamming,
    SquaredError,
    AbsoluteError,
    WeightedHamming(f64),
}
/// Huffman code: maps symbol indices to binary codewords.
pub struct HuffmanCode {
    /// Codeword for each symbol (as a Vec<bool>, false=0, true=1).
    pub codewords: Vec<Vec<bool>>,
    /// Average bits per symbol under the source distribution.
    pub avg_bits: f64,
}
impl HuffmanCode {
    /// Build a Huffman code from symbol probabilities.
    pub fn build(probs: &[f64]) -> Self {
        let n = probs.len();
        if n == 0 {
            return HuffmanCode {
                codewords: vec![],
                avg_bits: 0.0,
            };
        }
        if n == 1 {
            return HuffmanCode {
                codewords: vec![vec![false]],
                avg_bits: probs[0],
            };
        }
        let mut heap: Vec<(f64, Vec<usize>)> = probs
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, vec![i]))
            .collect();
        let mut code_bits: Vec<Vec<bool>> = vec![vec![]; n];
        while heap.len() > 1 {
            heap.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let (p1, idx1) = heap.remove(0);
            let (p2, idx2) = heap.remove(0);
            for i in &idx1 {
                code_bits[*i].insert(0, false);
            }
            for i in &idx2 {
                code_bits[*i].insert(0, true);
            }
            let mut merged = idx1;
            merged.extend(idx2);
            heap.push((p1 + p2, merged));
        }
        let avg_bits = probs
            .iter()
            .zip(code_bits.iter())
            .map(|(&p, bits)| p * bits.len() as f64)
            .sum();
        HuffmanCode {
            codewords: code_bits,
            avg_bits,
        }
    }
    /// Encode a sequence of symbol indices to a flat bit string.
    pub fn encode(&self, symbols: &[usize]) -> Vec<bool> {
        symbols
            .iter()
            .flat_map(|&s| self.codewords[s].iter().copied())
            .collect()
    }
    /// Decode a flat bit string back to symbol indices.
    ///
    /// Returns `None` if the bit string does not correspond to valid codewords.
    pub fn decode(&self, bits: &[bool]) -> Option<Vec<usize>> {
        let mut result = Vec::new();
        let mut pos = 0;
        while pos < bits.len() {
            let mut found = false;
            for (sym, cw) in self.codewords.iter().enumerate() {
                if bits.len() >= pos + cw.len() && &bits[pos..pos + cw.len()] == cw.as_slice() {
                    result.push(sym);
                    pos += cw.len();
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
        }
        Some(result)
    }
}
/// Rényi entropy computation of order α with various utilities.
///
/// H_α(X) = (1/(1-α)) * log2(Σ p(x)^α)
pub struct RenyiEntropyComputer {
    /// Order parameter α (must be non-negative, α ≠ 1).
    pub alpha: f64,
}
impl RenyiEntropyComputer {
    /// Create a Rényi entropy computer for order alpha.
    ///
    /// Panics if alpha is negative.
    pub fn new(alpha: f64) -> Self {
        assert!(alpha >= 0.0, "Rényi order must be non-negative");
        RenyiEntropyComputer { alpha }
    }
    /// Compute H_α(X) for the given probability distribution.
    pub fn compute(&self, probs: &[f64]) -> f64 {
        renyi_entropy(self.alpha, probs)
    }
    /// Compute the Rényi divergence D_α(P || Q) = (1/(α-1)) log Σ p^α / q^{α-1}.
    ///
    /// Returns infinity if Q has zero mass where P does not.
    pub fn renyi_divergence(&self, p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(p.len(), q.len(), "P and Q must have same length");
        if (self.alpha - 1.0).abs() < 1e-10 {
            return kl_divergence(p, q);
        }
        let sum: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi <= 0.0 {
                    0.0
                } else if qi <= 0.0 {
                    f64::INFINITY
                } else {
                    pi.powf(self.alpha) / qi.powf(self.alpha - 1.0)
                }
            })
            .sum();
        if sum.is_infinite() {
            return f64::INFINITY;
        }
        sum.log2() / (self.alpha - 1.0)
    }
    /// Compute the collision entropy H_2(X) = -log2(Σ p^2) regardless of stored alpha.
    pub fn collision_entropy(probs: &[f64]) -> f64 {
        renyi_entropy(2.0, probs)
    }
    /// Compute Hartley entropy H_0(X) = log2(|support|).
    pub fn hartley_entropy(probs: &[f64]) -> f64 {
        let support_size = probs.iter().filter(|&&p| p > 0.0).count();
        if support_size == 0 {
            0.0
        } else {
            (support_size as f64).log2()
        }
    }
}
/// Kolmogorov complexity (approximated via compression ratio).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KolmogorovComplexity {
    pub source_bits: usize,
    pub compressed_bits_approx: usize,
}
#[allow(dead_code)]
impl KolmogorovComplexity {
    pub fn new(source: usize, compressed: usize) -> Self {
        KolmogorovComplexity {
            source_bits: source,
            compressed_bits_approx: compressed,
        }
    }
    pub fn compression_ratio(&self) -> f64 {
        if self.source_bits == 0 {
            return 1.0;
        }
        self.compressed_bits_approx as f64 / self.source_bits as f64
    }
    pub fn is_random(&self) -> bool {
        self.compression_ratio() > 0.95
    }
    pub fn redundancy(&self) -> f64 {
        (1.0 - self.compression_ratio()).max(0.0)
    }
}
/// Arithmetic coder for a fixed symbol alphabet with known probabilities.
pub struct ArithmeticCoder {
    /// Symbol probabilities (must sum to 1).
    pub probs: Vec<f64>,
    /// Cumulative probabilities: cum_probs[i] = Σ_{j<i} probs[j].
    pub cum_probs: Vec<f64>,
}
impl ArithmeticCoder {
    /// Create a new arithmetic coder.
    ///
    /// Panics if `probs` is empty.
    pub fn new(probs: Vec<f64>) -> Self {
        assert!(!probs.is_empty(), "probs must be non-empty");
        let mut cum = vec![0.0f64; probs.len() + 1];
        for (i, &p) in probs.iter().enumerate() {
            cum[i + 1] = cum[i] + p;
        }
        let cum_probs = cum[..probs.len()].to_vec();
        ArithmeticCoder { probs, cum_probs }
    }
    /// Return the interval (low, high) assigned to a symbol in arithmetic coding.
    ///
    /// Interval for symbol `s` is [cum_probs[s], cum_probs[s] + probs[s]).
    pub fn encode_symbol(&self, symbol: usize) -> (f64, f64) {
        let low = self.cum_probs[symbol];
        let high = low + self.probs[symbol];
        (low, high)
    }
    /// Approximate the number of bits needed to encode a sequence.
    ///
    /// Returns -Σ log2(p(x_i)) (the Shannon information content of the sequence).
    pub fn entropy_approx_bits(&self, sequence: &[usize]) -> f64 {
        sequence
            .iter()
            .map(|&sym| {
                let p = self.probs[sym];
                if p <= 0.0 {
                    f64::INFINITY
                } else {
                    -p.log2()
                }
            })
            .sum()
    }
}
/// Tsallis entropy of order q: S_q(X) = (1/(q-1)) * (1 - Σ p(x)^q).
///
/// - For q → 1, Tsallis entropy converges to Shannon entropy.
/// - Used in nonextensive statistical mechanics.
pub struct TsallisEntropyComputer {
    /// Entropic index q (must be positive).
    pub q: f64,
}
impl TsallisEntropyComputer {
    /// Create a Tsallis entropy computer for index q.
    ///
    /// Panics if q ≤ 0.
    pub fn new(q: f64) -> Self {
        assert!(q > 0.0, "Tsallis index q must be positive");
        TsallisEntropyComputer { q }
    }
    /// Compute S_q(X) for the given probability distribution.
    pub fn compute(&self, probs: &[f64]) -> f64 {
        if probs.is_empty() {
            return 0.0;
        }
        if (self.q - 1.0).abs() < 1e-10 {
            return entropy(probs);
        }
        let sum_pq: f64 = probs
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| p.powf(self.q))
            .sum();
        (1.0 - sum_pq) / (self.q - 1.0)
    }
    /// Compute the Tsallis relative entropy (q-divergence) D_q(P||Q).
    ///   D_q(P||Q) = (1/(q-1)) * (1 - Σ p(x)^q / q(x)^{q-1})
    pub fn tsallis_divergence(&self, p: &[f64], q_dist: &[f64]) -> f64 {
        assert_eq!(
            p.len(),
            q_dist.len(),
            "distributions must have equal length"
        );
        if (self.q - 1.0).abs() < 1e-10 {
            return kl_divergence(p, q_dist);
        }
        let sum: f64 = p
            .iter()
            .zip(q_dist.iter())
            .map(|(&pi, &qi)| {
                if pi <= 0.0 {
                    0.0
                } else if qi <= 0.0 {
                    f64::INFINITY
                } else {
                    pi.powf(self.q) / qi.powf(self.q - 1.0)
                }
            })
            .sum();
        if sum.is_infinite() {
            return f64::INFINITY;
        }
        (1.0 - sum) / (self.q - 1.0)
    }
}
/// Extended KL divergence utilities.
pub struct KLDivergenceCalc;
impl KLDivergenceCalc {
    /// Compute D(p || q) in bits.
    pub fn compute(p: &[f64], q: &[f64]) -> f64 {
        kl_divergence(p, q)
    }
    /// Compute the symmetrized KL divergence (Jensen-Shannon divergence numerator).
    ///
    /// Returns D(p||q) + D(q||p).
    pub fn symmetrized(p: &[f64], q: &[f64]) -> f64 {
        kl_divergence(p, q) + kl_divergence(q, p)
    }
    /// Jensen-Shannon divergence JS(p,q) = (D(p||m) + D(q||m)) / 2 where m = (p+q)/2.
    pub fn jensen_shannon(p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(
            p.len(),
            q.len(),
            "Jensen-Shannon: p and q must have same length"
        );
        let m: Vec<f64> = p
            .iter()
            .zip(q.iter())
            .map(|(&a, &b)| (a + b) / 2.0)
            .collect();
        (kl_divergence(p, &m) + kl_divergence(q, &m)) / 2.0
    }
}
/// Information-theoretic security (unconditional security).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InfoTheoreticSecurity {
    pub secret_bits: usize,
    pub key_bits: usize,
    pub ciphertext_bits: usize,
}
#[allow(dead_code)]
impl InfoTheoreticSecurity {
    pub fn new(s: usize, k: usize, c: usize) -> Self {
        InfoTheoreticSecurity {
            secret_bits: s,
            key_bits: k,
            ciphertext_bits: c,
        }
    }
    pub fn is_perfectly_secret(&self) -> bool {
        self.key_bits >= self.secret_bits
    }
    pub fn is_one_time_pad(&self) -> bool {
        self.key_bits == self.secret_bits && self.ciphertext_bits == self.secret_bits
    }
    pub fn leakage_bits(&self) -> f64 {
        if self.is_perfectly_secret() {
            0.0
        } else {
            (self.secret_bits - self.key_bits) as f64
        }
    }
}
/// Chernoff information between two probability distributions.
///
/// C(P, Q) = -log min_{0 ≤ t ≤ 1} Σ_x P(x)^t * Q(x)^{1-t}
///
/// This is the optimal error exponent for the Bayesian binary hypothesis
/// testing problem between P and Q.
pub struct ChernoffInformationCalc;
impl ChernoffInformationCalc {
    /// Compute the Bhattacharyya coefficient at parameter t:
    ///   B(t) = Σ_x P(x)^t * Q(x)^{1-t}
    pub fn bhattacharyya_at_t(p: &[f64], q: &[f64], t: f64) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi <= 0.0 || qi <= 0.0 {
                    0.0
                } else {
                    pi.powf(t) * qi.powf(1.0 - t)
                }
            })
            .sum()
    }
    /// Compute the Chernoff information C(P, Q) via line search over t ∈ [0,1].
    ///
    /// Uses a ternary search to minimize B(t), then returns -log2(min B(t)).
    pub fn compute(p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(p.len(), q.len(), "P and Q must have the same support size");
        let mut lo = 0.0f64;
        let mut hi = 1.0f64;
        for _ in 0..200 {
            let m1 = lo + (hi - lo) / 3.0;
            let m2 = hi - (hi - lo) / 3.0;
            let b1 = Self::bhattacharyya_at_t(p, q, m1);
            let b2 = Self::bhattacharyya_at_t(p, q, m2);
            if b1 < b2 {
                hi = m2;
            } else {
                lo = m1;
            }
        }
        let t_opt = (lo + hi) / 2.0;
        let min_b = Self::bhattacharyya_at_t(p, q, t_opt);
        if min_b <= 0.0 {
            return f64::INFINITY;
        }
        -min_b.log2()
    }
    /// Compute the Bhattacharyya distance: -log2 B(0.5).
    pub fn bhattacharyya_distance(p: &[f64], q: &[f64]) -> f64 {
        let b = Self::bhattacharyya_at_t(p, q, 0.5);
        if b <= 0.0 {
            f64::INFINITY
        } else {
            -b.log2()
        }
    }
}
/// Estimate entropy from observed symbol frequencies.
///
/// Uses the empirical probability distribution derived from counts.
pub struct EntropyEstimator {
    /// Raw frequency counts per symbol.
    pub counts: Vec<u64>,
}
impl EntropyEstimator {
    /// Create estimator from raw frequency counts.
    pub fn new(counts: Vec<u64>) -> Self {
        EntropyEstimator { counts }
    }
    /// Compute the empirical probability distribution.
    pub fn probabilities(&self) -> Vec<f64> {
        let total: u64 = self.counts.iter().sum();
        if total == 0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts
            .iter()
            .map(|&c| c as f64 / total as f64)
            .collect()
    }
    /// Estimate Shannon entropy in bits.
    pub fn estimate_entropy(&self) -> f64 {
        entropy(&self.probabilities())
    }
    /// Estimate Rényi entropy of order alpha.
    pub fn estimate_renyi(&self, alpha: f64) -> f64 {
        let probs = self.probabilities();
        renyi_entropy(alpha, &probs)
    }
    /// Estimate min-entropy H_∞ = -log2(max p).
    pub fn estimate_min_entropy(&self) -> f64 {
        let probs = self.probabilities();
        min_entropy(&probs)
    }
}
/// Blahut-Arimoto algorithm for computing discrete channel capacity.
///
/// Iterates between two fixed-point equations until convergence:
/// - Update output distribution: r(y) = Σ_x q(x) W(y|x)
/// - Update input distribution: q(x) ∝ exp(Σ_y W(y|x) log(W(y|x)/r(y)))
pub struct BlahutArimoto {
    /// Channel transition matrix W[x][y] = P(Y=y | X=x).
    pub channel: Vec<Vec<f64>>,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance (in bits).
    pub tolerance: f64,
}
impl BlahutArimoto {
    /// Create a new Blahut-Arimoto instance.
    pub fn new(channel: Vec<Vec<f64>>, max_iter: usize, tolerance: f64) -> Self {
        BlahutArimoto {
            channel,
            max_iter,
            tolerance,
        }
    }
    /// Run the algorithm and return `(capacity_bits, optimal_input_distribution)`.
    pub fn run(&self) -> (f64, Vec<f64>) {
        let x_size = self.channel.len();
        if x_size == 0 {
            return (0.0, vec![]);
        }
        let y_size = self.channel[0].len();
        if y_size == 0 {
            return (0.0, vec![0.0; x_size]);
        }
        let mut q = vec![1.0 / x_size as f64; x_size];
        for _ in 0..self.max_iter {
            let mut r = vec![0.0f64; y_size];
            for x in 0..x_size {
                for y in 0..y_size {
                    r[y] += q[x] * self.channel[x][y];
                }
            }
            let mut log_c = vec![0.0f64; x_size];
            for x in 0..x_size {
                for y in 0..y_size {
                    let w = self.channel[x][y];
                    if w > 0.0 && r[y] > 0.0 {
                        log_c[x] += w * (w / r[y]).log2();
                    }
                }
            }
            let max_lc = log_c.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let mut q_new: Vec<f64> = log_c.iter().map(|&lc| (lc - max_lc).exp()).collect();
            let sum_q: f64 = q_new.iter().sum();
            for qi in q_new.iter_mut() {
                *qi /= sum_q;
            }
            let delta: f64 = q.iter().zip(q_new.iter()).map(|(a, b)| (a - b).abs()).sum();
            q = q_new;
            if delta < self.tolerance {
                break;
            }
        }
        let capacity = self.compute_mutual_information(&q);
        (capacity, q)
    }
    fn compute_mutual_information(&self, q: &[f64]) -> f64 {
        let x_size = self.channel.len();
        let y_size = if x_size > 0 { self.channel[0].len() } else { 0 };
        let mut r = vec![0.0f64; y_size];
        for x in 0..x_size {
            for y in 0..y_size {
                r[y] += q[x] * self.channel[x][y];
            }
        }
        let mut mi = 0.0f64;
        for x in 0..x_size {
            for y in 0..y_size {
                let w = self.channel[x][y];
                if w > 0.0 && r[y] > 0.0 {
                    mi += q[x] * w * (w / r[y]).log2();
                }
            }
        }
        mi
    }
}
/// Lempel-Ziv complexity of a binary string.
///
/// Computes c(x^n) = number of distinct phrases in the LZ78 parse of x.
/// The asymptotic rate c(x^n) * log(n) / n → H(X) for ergodic sources.
pub struct LempelZivComplexity;
impl LempelZivComplexity {
    /// Compute the LZ78 complexity of a binary sequence.
    ///
    /// Returns the number of distinct phrases (dictionary entries) in the LZ78 parse.
    pub fn compute(bits: &[bool]) -> usize {
        if bits.is_empty() {
            return 0;
        }
        let mut dictionary: std::collections::HashSet<Vec<bool>> = std::collections::HashSet::new();
        let mut current: Vec<bool> = Vec::new();
        let mut count = 0;
        for &bit in bits {
            current.push(bit);
            if !dictionary.contains(&current) {
                dictionary.insert(current.clone());
                count += 1;
                current.clear();
            }
        }
        if !current.is_empty() {
            count += 1;
        }
        count
    }
    /// Estimate the per-symbol entropy rate using LZ complexity:
    ///   H_lz ≈ c(x^n) * log2(n) / n
    pub fn entropy_rate_estimate(bits: &[bool]) -> f64 {
        let n = bits.len();
        if n == 0 {
            return 0.0;
        }
        let c = Self::compute(bits) as f64;
        c * (n as f64).log2() / n as f64
    }
}
/// Output symbol for a BEC (0, 1, or erased).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BecOutput {
    /// Received bit 0.
    Zero,
    /// Received bit 1.
    One,
    /// Erased (unknown).
    Erased,
}
/// Mutual information I(X;Y) estimator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MutualInformation {
    pub joint_pmf: Vec<Vec<f64>>,
    pub n_x: usize,
    pub n_y: usize,
}
#[allow(dead_code)]
impl MutualInformation {
    pub fn new(joint: Vec<Vec<f64>>) -> Self {
        let nx = joint.len();
        let ny = if nx > 0 { joint[0].len() } else { 0 };
        MutualInformation {
            joint_pmf: joint,
            n_x: nx,
            n_y: ny,
        }
    }
    pub fn marginal_x(&self) -> Vec<f64> {
        (0..self.n_x)
            .map(|i| self.joint_pmf[i].iter().sum::<f64>())
            .collect()
    }
    pub fn marginal_y(&self) -> Vec<f64> {
        (0..self.n_y)
            .map(|j| self.joint_pmf.iter().map(|row| row[j]).sum::<f64>())
            .collect()
    }
    pub fn compute(&self) -> f64 {
        let px = self.marginal_x();
        let py = self.marginal_y();
        let mut mi = 0.0;
        for i in 0..self.n_x {
            for j in 0..self.n_y {
                let pxy = self.joint_pmf[i][j];
                if pxy > 0.0 && px[i] > 0.0 && py[j] > 0.0 {
                    mi += pxy * (pxy / (px[i] * py[j])).log2();
                }
            }
        }
        mi
    }
    /// Normalized MI in [0, 1].
    pub fn normalized(&self) -> f64 {
        let mi = self.compute();
        let hx = shannon_entropy(&self.marginal_x());
        let hy = shannon_entropy(&self.marginal_y());
        let denom = (hx + hy) / 2.0;
        if denom < 1e-12 {
            0.0
        } else {
            mi / denom
        }
    }
}
