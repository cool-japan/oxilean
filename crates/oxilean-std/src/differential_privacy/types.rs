//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoiseType {
    Laplace,
    Exponential,
    Gumbel,
}
/// Full Laplace mechanism with sensitivity and epsilon.
///
/// Adds Lap(Δf / ε) noise to the true query answer.
pub struct LaplaceMechanism {
    /// Global L1 sensitivity of the query.
    pub sensitivity: f64,
    /// Privacy parameter ε.
    pub epsilon: f64,
    /// Derived scale b = sensitivity / epsilon.
    pub scale: f64,
}
impl LaplaceMechanism {
    /// Create a new Laplace mechanism.
    ///
    /// Achieves ε-pure DP for any query with L1 sensitivity `sensitivity`.
    pub fn new(sensitivity: f64, epsilon: f64) -> Self {
        assert!(sensitivity > 0.0, "sensitivity must be positive");
        assert!(epsilon > 0.0, "epsilon must be positive");
        let scale = sensitivity / epsilon;
        LaplaceMechanism {
            sensitivity,
            epsilon,
            scale,
        }
    }
    /// Apply the mechanism: add Laplace noise to a true answer using
    /// a uniform sample u ∈ (0, 1) (caller provides randomness).
    pub fn apply(&self, true_answer: f64, u: f64) -> f64 {
        assert!(u > 0.0 && u < 1.0, "u must be in (0, 1)");
        let v = u - 0.5;
        let noise = -self.scale * v.signum() * (1.0 - 2.0 * v.abs()).ln();
        true_answer + noise
    }
    /// Privacy loss ε = sensitivity / scale (recovers the configured ε).
    pub fn privacy_loss(&self) -> f64 {
        self.sensitivity / self.scale
    }
}
/// Renyi differential privacy (alpha-RDP).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RenyiDp {
    pub alpha: f64,
    pub epsilon: f64,
}
impl RenyiDp {
    #[allow(dead_code)]
    pub fn new(alpha: f64, epsilon: f64) -> Self {
        assert!(alpha > 1.0, "RDP order alpha must be > 1");
        assert!(epsilon >= 0.0, "epsilon must be >= 0");
        Self { alpha, epsilon }
    }
    #[allow(dead_code)]
    pub fn to_pure_dp(&self) -> (f64, f64) {
        let delta: f64 = 1e-5;
        let eps_prime = self.epsilon + (1.0_f64 / delta).ln() / (self.alpha - 1.0);
        (eps_prime, delta)
    }
    #[allow(dead_code)]
    pub fn compose(&self, other: &RenyiDp) -> Option<RenyiDp> {
        if (self.alpha - other.alpha).abs() < 1e-10 {
            Some(RenyiDp::new(self.alpha, self.epsilon + other.epsilon))
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn gaussian_mechanism_epsilon(alpha: f64, sigma: f64, sensitivity: f64) -> Self {
        let eps = alpha * sensitivity * sensitivity / (2.0 * sigma * sigma);
        Self::new(alpha, eps)
    }
}
/// Privacy accounting via Rényi Differential Privacy (RDP).
pub struct RenyiAccountant {
    /// Accumulated RDP budgets per order α.
    /// Each entry is (alpha, epsilon_rdp).
    pub ledger: Vec<(f64, f64)>,
}
impl RenyiAccountant {
    /// Create an empty RDP accountant.
    pub fn new() -> Self {
        RenyiAccountant { ledger: vec![] }
    }
    /// Record a mechanism with (α, ε_rdp)-RDP guarantee.
    pub fn compose(&mut self, alpha: f64, eps_rdp: f64) {
        if let Some(entry) = self
            .ledger
            .iter_mut()
            .find(|(a, _)| (*a - alpha).abs() < 1e-10)
        {
            entry.1 += eps_rdp;
        } else {
            self.ledger.push((alpha, eps_rdp));
        }
    }
    /// Convert (α, ε_rdp)-RDP to (ε, δ)-DP for a given δ.
    ///
    /// Formula: ε = ε_rdp + log(1/δ) / (α - 1).
    pub fn to_approx_dp(&self, alpha: f64, eps_rdp: f64, delta: f64) -> f64 {
        assert!(alpha > 1.0, "α must be > 1 for RDP to DP conversion");
        assert!(delta > 0.0 && delta < 1.0);
        eps_rdp + (1.0 / delta).ln() / (alpha - 1.0)
    }
    /// Find the best ε across all tracked α values for a given δ.
    pub fn optimal_eps(&self, delta: f64) -> f64 {
        self.ledger
            .iter()
            .filter(|(alpha, _)| *alpha > 1.0)
            .map(|(alpha, eps_rdp)| self.to_approx_dp(*alpha, *eps_rdp, delta))
            .fold(f64::INFINITY, f64::min)
    }
}
/// Exponential mechanism for private selection.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExponentialMechanismExt {
    pub epsilon: f64,
    pub sensitivity: f64,
    pub output_range_name: String,
}
impl ExponentialMechanismExt {
    #[allow(dead_code)]
    pub fn new(eps: f64, sens: f64, range: &str) -> Self {
        Self {
            epsilon: eps,
            sensitivity: sens,
            output_range_name: range.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn sampling_probability_description(&self) -> String {
        format!(
            "Pr[output = r] proportional to exp(epsilon * u(D, r) / (2 * sensitivity)), range={}",
            self.output_range_name
        )
    }
    #[allow(dead_code)]
    pub fn utility_guarantee(&self, opt_utility: f64, range_size: usize) -> f64 {
        self.sensitivity * (range_size as f64).ln() / self.epsilon + opt_utility
    }
    #[allow(dead_code)]
    pub fn is_epsilon_dp(&self) -> bool {
        true
    }
}
/// Laplace distribution sampler using the inverse CDF method.
///
/// Draws a sample from Laplace(0, b) using uniform random bits.
/// In production, use a cryptographically secure RNG.
pub struct LaplaceNoise {
    /// Scale parameter b = Δf / ε.
    pub scale: f64,
}
impl LaplaceNoise {
    /// Create a new Laplace noise generator with the given scale.
    pub fn new(scale: f64) -> Self {
        assert!(scale > 0.0, "Laplace scale must be positive");
        LaplaceNoise { scale }
    }
    /// Sample from Laplace(0, scale) using a provided uniform sample u ∈ (0,1).
    ///
    /// Uses the inverse CDF: X = -scale * sign(u - 0.5) * ln(1 - 2|u - 0.5|).
    pub fn sample_from_uniform(&self, u: f64) -> f64 {
        assert!(u > 0.0 && u < 1.0, "u must be in (0, 1)");
        let v = u - 0.5;
        -self.scale * v.signum() * (1.0 - 2.0 * v.abs()).ln()
    }
    /// Compute the scale parameter for (ε, 0)-DP given L1 sensitivity Δf.
    pub fn scale_for_pure_dp(sensitivity: f64, eps: f64) -> f64 {
        assert!(eps > 0.0, "ε must be positive");
        sensitivity / eps
    }
}
/// Differentially private synthetic data generation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpSyntheticData {
    pub epsilon: f64,
    pub delta: f64,
    pub num_attributes: usize,
    pub method: SyntheticDataMethod,
}
impl DpSyntheticData {
    #[allow(dead_code)]
    pub fn new(eps: f64, delta: f64, attrs: usize, method: SyntheticDataMethod) -> Self {
        Self {
            epsilon: eps,
            delta,
            num_attributes: attrs,
            method,
        }
    }
    #[allow(dead_code)]
    pub fn marginal_error_bound(&self) -> f64 {
        (self.num_attributes as f64).sqrt() / (self.epsilon * 100.0)
    }
}
/// Gaussian noise sampler for (ε, δ)-DP.
pub struct GaussianNoise {
    /// Standard deviation σ.
    pub sigma: f64,
}
impl GaussianNoise {
    /// Create a new Gaussian noise generator.
    pub fn new(sigma: f64) -> Self {
        assert!(sigma > 0.0, "Gaussian sigma must be positive");
        GaussianNoise { sigma }
    }
    /// Compute σ for (ε, δ)-DP given L2 sensitivity Δ₂f.
    ///
    /// Uses the analytic Gaussian mechanism: σ = Δ₂f * sqrt(2 * ln(1.25/δ)) / ε.
    pub fn sigma_for_approx_dp(l2_sensitivity: f64, eps: f64, delta: f64) -> f64 {
        assert!(eps > 0.0 && delta > 0.0 && delta < 1.0);
        l2_sensitivity * (2.0 * (1.25f64 / delta).ln()).sqrt() / eps
    }
    /// Box-Muller transform: given two uniform samples u1, u2 ∈ (0,1),
    /// return a standard normal sample.
    pub fn box_muller(u1: f64, u2: f64) -> f64 {
        assert!(u1 > 0.0 && u2 > 0.0);
        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * std::f64::consts::PI * u2;
        r * theta.cos()
    }
    /// Scale a standard normal sample by σ.
    pub fn scale_sample(&self, z: f64) -> f64 {
        self.sigma * z
    }
}
/// Zero Concentrated Differential Privacy (zCDP).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZcdpBound {
    pub rho: f64,
}
impl ZcdpBound {
    #[allow(dead_code)]
    pub fn new(rho: f64) -> Self {
        assert!(rho >= 0.0);
        Self { rho }
    }
    #[allow(dead_code)]
    pub fn to_approximate_dp(&self, delta: f64) -> f64 {
        self.rho + 2.0 * (self.rho * (1.0 / delta).ln()).sqrt()
    }
    #[allow(dead_code)]
    pub fn gaussian_mechanism_rho(sigma: f64, sensitivity: f64) -> Self {
        Self::new(sensitivity * sensitivity / (2.0 * sigma * sigma))
    }
    #[allow(dead_code)]
    pub fn compose(&self, other: &ZcdpBound) -> ZcdpBound {
        ZcdpBound::new(self.rho + other.rho)
    }
}
/// Differentially private histogram.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpHistogram {
    pub bins: usize,
    pub epsilon: f64,
    pub noise_mechanism: NoiseType,
}
impl DpHistogram {
    #[allow(dead_code)]
    pub fn laplace(bins: usize, eps: f64) -> Self {
        Self {
            bins,
            epsilon: eps,
            noise_mechanism: NoiseType::Laplace,
        }
    }
    #[allow(dead_code)]
    pub fn l1_sensitivity(&self) -> f64 {
        2.0
    }
    #[allow(dead_code)]
    pub fn noise_scale(&self) -> f64 {
        self.l1_sensitivity() / self.epsilon
    }
    #[allow(dead_code)]
    pub fn expected_absolute_error(&self) -> f64 {
        self.noise_scale()
    }
}
/// Full Gaussian mechanism with sensitivity and (ε, δ) target.
///
/// Adds N(0, σ²) noise where σ = Δ₂f * sqrt(2 * ln(1.25/δ)) / ε.
pub struct GaussianMechanism {
    /// Global L2 sensitivity of the query.
    pub l2_sensitivity: f64,
    /// Privacy parameter ε.
    pub epsilon: f64,
    /// Privacy parameter δ.
    pub delta: f64,
    /// Derived standard deviation σ.
    pub sigma: f64,
}
impl GaussianMechanism {
    /// Create a new Gaussian mechanism satisfying (ε, δ)-DP.
    pub fn new(l2_sensitivity: f64, epsilon: f64, delta: f64) -> Self {
        assert!(l2_sensitivity > 0.0, "l2_sensitivity must be positive");
        assert!(epsilon > 0.0, "epsilon must be positive");
        assert!(delta > 0.0 && delta < 1.0, "delta must be in (0, 1)");
        let sigma = l2_sensitivity * (2.0 * (1.25f64 / delta).ln()).sqrt() / epsilon;
        GaussianMechanism {
            l2_sensitivity,
            epsilon,
            delta,
            sigma,
        }
    }
    /// Apply the mechanism using Box-Muller transform with two uniform samples.
    pub fn apply(&self, true_answer: f64, u1: f64, u2: f64) -> f64 {
        let z = GaussianNoise::box_muller(u1, u2);
        true_answer + self.sigma * z
    }
    /// RDP guarantee at order α: ε_rdp = α * Δ₂² / (2σ²).
    pub fn rdp_guarantee(&self, alpha: f64) -> f64 {
        assert!(alpha > 1.0, "α must be > 1");
        alpha * self.l2_sensitivity * self.l2_sensitivity / (2.0 * self.sigma * self.sigma)
    }
}
/// Exponential mechanism for discrete outputs.
///
/// Samples an output proportional to exp(ε * u(D, r) / (2 * Δu)).
pub struct ExponentialMechanism {
    /// Privacy parameter ε.
    pub epsilon: f64,
    /// Sensitivity of the utility function Δu.
    pub utility_sensitivity: f64,
}
impl ExponentialMechanism {
    /// Create a new Exponential mechanism.
    pub fn new(epsilon: f64, utility_sensitivity: f64) -> Self {
        assert!(epsilon > 0.0, "epsilon must be positive");
        assert!(
            utility_sensitivity > 0.0,
            "utility_sensitivity must be positive"
        );
        ExponentialMechanism {
            epsilon,
            utility_sensitivity,
        }
    }
    /// Compute sampling probabilities for each candidate given their utility scores.
    pub fn probabilities(&self, utility_scores: &[f64]) -> Vec<f64> {
        assert!(!utility_scores.is_empty(), "need at least one candidate");
        let scale = self.epsilon / (2.0 * self.utility_sensitivity);
        let weights: Vec<f64> = utility_scores.iter().map(|&u| (scale * u).exp()).collect();
        let total: f64 = weights.iter().sum();
        weights.iter().map(|&w| w / total).collect()
    }
    /// Select a candidate index given its probabilities and a uniform sample u ∈ [0, 1).
    pub fn sample_index(&self, probs: &[f64], u: f64) -> usize {
        assert!(u >= 0.0 && u < 1.0, "u must be in [0, 1)");
        let mut cumulative = 0.0;
        for (i, &p) in probs.iter().enumerate() {
            cumulative += p;
            if u < cumulative {
                return i;
            }
        }
        probs.len() - 1
    }
}
/// Local differential privacy mechanism.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalDpMechanism {
    pub epsilon: f64,
    pub mechanism_type: LocalMechanismType,
    pub domain_size: usize,
}
impl LocalDpMechanism {
    #[allow(dead_code)]
    pub fn randomized_response(eps: f64) -> Self {
        Self {
            epsilon: eps,
            mechanism_type: LocalMechanismType::RandomizedResponse,
            domain_size: 2,
        }
    }
    #[allow(dead_code)]
    pub fn unary_encoding(eps: f64, d: usize) -> Self {
        Self {
            epsilon: eps,
            mechanism_type: LocalMechanismType::UnaryEncoding,
            domain_size: d,
        }
    }
    #[allow(dead_code)]
    pub fn variance_estimate(&self) -> f64 {
        let e = self.epsilon.exp();
        let d = self.domain_size as f64;
        match self.mechanism_type {
            LocalMechanismType::RandomizedResponse => 4.0 * e / ((e - 1.0) * (e - 1.0)),
            LocalMechanismType::UnaryEncoding => (e + 1.0) / (e - 1.0) * (e + 1.0) / (e - 1.0) / d,
            _ => 1.0 / (d * self.epsilon * self.epsilon),
        }
    }
    #[allow(dead_code)]
    pub fn is_locally_private(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntheticDataMethod {
    PrivBayes,
    Mst,
    Aim,
    Gem,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalMechanismType {
    RandomizedResponse,
    UnaryEncoding,
    OptimizedUnaryEncoding,
    HadamardResponse,
    SampledHistogram,
}
/// Privacy ledger tracking cumulative privacy cost.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PrivacyLedger {
    pub entries: Vec<PrivacyEntry>,
}
impl PrivacyLedger {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn add_entry(&mut self, name: &str, eps: f64, delta: f64, comp: CompositionType) {
        self.entries.push(PrivacyEntry {
            mechanism_name: name.to_string(),
            epsilon: eps,
            delta,
            composition: comp,
        });
    }
    #[allow(dead_code)]
    pub fn total_sequential_epsilon(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| e.composition == CompositionType::Sequential)
            .map(|e| e.epsilon)
            .sum()
    }
    #[allow(dead_code)]
    pub fn total_sequential_delta(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| e.composition == CompositionType::Sequential)
            .map(|e| e.delta)
            .sum()
    }
    #[allow(dead_code)]
    pub fn parallel_max_epsilon(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| e.composition == CompositionType::Parallel)
            .map(|e| e.epsilon)
            .fold(0.0_f64, f64::max)
    }
}
/// Report Noisy Max mechanism.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReportNoisyMax {
    pub epsilon: f64,
    pub noise_type: NoiseType,
}
impl ReportNoisyMax {
    #[allow(dead_code)]
    pub fn with_laplace(epsilon: f64) -> Self {
        Self {
            epsilon,
            noise_type: NoiseType::Laplace,
        }
    }
    #[allow(dead_code)]
    pub fn is_pure_dp(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn scale(&self) -> f64 {
        1.0 / self.epsilon
    }
}
/// Privacy budget tracker for sequential (ε, δ)-DP composition.
///
/// Tracks total spent budget across sequential mechanism applications.
pub struct PrivacyBudget {
    /// Total allocated ε budget.
    pub total_epsilon: f64,
    /// Total allocated δ budget.
    pub total_delta: f64,
    /// Accumulated spent ε so far.
    pub spent_epsilon: f64,
    /// Accumulated spent δ so far.
    pub spent_delta: f64,
}
impl PrivacyBudget {
    /// Create a new budget with the given total allowance.
    pub fn new(total_epsilon: f64, total_delta: f64) -> Self {
        assert!(total_epsilon > 0.0, "total_epsilon must be positive");
        assert!(total_delta >= 0.0, "total_delta must be non-negative");
        PrivacyBudget {
            total_epsilon,
            total_delta,
            spent_epsilon: 0.0,
            spent_delta: 0.0,
        }
    }
    /// Attempt to spend (eps, delta) from the budget.
    ///
    /// Returns `Ok(())` if sufficient budget remains, `Err` otherwise.
    pub fn spend(&mut self, eps: f64, delta: f64) -> Result<(), String> {
        let new_eps = self.spent_epsilon + eps;
        let new_delta = self.spent_delta + delta;
        if new_eps > self.total_epsilon + 1e-12 {
            return Err(format!(
                "Epsilon budget exceeded: need {:.4}, have {:.4}",
                new_eps, self.total_epsilon
            ));
        }
        if new_delta > self.total_delta + 1e-12 {
            return Err(format!(
                "Delta budget exceeded: need {:.4}, have {:.4}",
                new_delta, self.total_delta
            ));
        }
        self.spent_epsilon = new_eps;
        self.spent_delta = new_delta;
        Ok(())
    }
    /// Remaining ε budget.
    pub fn remaining_epsilon(&self) -> f64 {
        (self.total_epsilon - self.spent_epsilon).max(0.0)
    }
    /// Remaining δ budget.
    pub fn remaining_delta(&self) -> f64 {
        (self.total_delta - self.spent_delta).max(0.0)
    }
    /// True if the budget has not been exceeded.
    pub fn is_valid(&self) -> bool {
        self.spent_epsilon <= self.total_epsilon + 1e-12
            && self.spent_delta <= self.total_delta + 1e-12
    }
}
/// Differentially private mean estimation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpMeanEstimator {
    pub epsilon: f64,
    pub delta: f64,
    pub range: (f64, f64),
    pub n: usize,
}
impl DpMeanEstimator {
    #[allow(dead_code)]
    pub fn new(eps: f64, delta: f64, lo: f64, hi: f64, n: usize) -> Self {
        Self {
            epsilon: eps,
            delta,
            range: (lo, hi),
            n,
        }
    }
    #[allow(dead_code)]
    pub fn clipped_sensitivity(&self) -> f64 {
        (self.range.1 - self.range.0) / self.n as f64
    }
    #[allow(dead_code)]
    pub fn mse_gaussian_mechanism(&self) -> f64 {
        let sigma =
            self.clipped_sensitivity() * (2.0 * (1.25 / self.delta).ln()).sqrt() / self.epsilon;
        sigma * sigma
    }
}
/// Differentially private stochastic gradient descent (DP-SGD).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpSgd {
    pub learning_rate: f64,
    pub noise_multiplier: f64,
    pub max_grad_norm: f64,
    pub batch_size: usize,
    pub num_steps: usize,
    pub dataset_size: usize,
}
impl DpSgd {
    #[allow(dead_code)]
    pub fn new(
        lr: f64,
        noise_mult: f64,
        max_norm: f64,
        batch: usize,
        steps: usize,
        n: usize,
    ) -> Self {
        Self {
            learning_rate: lr,
            noise_multiplier: noise_mult,
            max_grad_norm: max_norm,
            batch_size: batch,
            num_steps: steps,
            dataset_size: n,
        }
    }
    #[allow(dead_code)]
    pub fn sampling_rate(&self) -> f64 {
        self.batch_size as f64 / self.dataset_size as f64
    }
    #[allow(dead_code)]
    pub fn privacy_spent_rdp_alpha(&self, alpha: f64) -> f64 {
        let q = self.sampling_rate();
        alpha * q * q / (2.0 * self.noise_multiplier * self.noise_multiplier)
            * self.num_steps as f64
    }
    #[allow(dead_code)]
    pub fn gradient_clipping_description(&self) -> String {
        format!("Clip grad to L2 norm <= {}", self.max_grad_norm)
    }
}
/// Privacy amplification by shuffling.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ShuffleAmplification {
    pub local_epsilon: f64,
    pub n: usize,
}
impl ShuffleAmplification {
    #[allow(dead_code)]
    pub fn new(local_eps: f64, n: usize) -> Self {
        Self {
            local_epsilon: local_eps,
            n,
        }
    }
    #[allow(dead_code)]
    pub fn central_epsilon_approx(&self) -> f64 {
        let e_eps = self.local_epsilon.exp();
        e_eps * (((self.n as f64).ln()).sqrt()) / (self.n as f64).sqrt()
    }
    #[allow(dead_code)]
    pub fn is_stronger_than_local_dp(&self) -> bool {
        self.central_epsilon_approx() < self.local_epsilon
    }
}
/// Differentially private median estimation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpMedianEstimator {
    pub epsilon: f64,
    pub domain_size: usize,
}
impl DpMedianEstimator {
    #[allow(dead_code)]
    pub fn new(eps: f64, d: usize) -> Self {
        Self {
            epsilon: eps,
            domain_size: d,
        }
    }
    #[allow(dead_code)]
    pub fn exponential_mechanism_based(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn sensitivity(&self) -> usize {
        1
    }
}
/// Inference attack model against differentially private output.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferenceAttackModel {
    pub adversary_advantage: f64,
    pub privacy_bound: f64,
}
impl InferenceAttackModel {
    #[allow(dead_code)]
    pub fn new(adv: f64, eps: f64) -> Self {
        Self {
            adversary_advantage: adv,
            privacy_bound: eps,
        }
    }
    #[allow(dead_code)]
    pub fn advantage_bounded_by_dp(&self) -> bool {
        let dp_bound = self.privacy_bound.exp() - 1.0;
        self.adversary_advantage <= dp_bound + 1e-10
    }
}
/// A single privacy expense entry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrivacyEntry {
    pub mechanism_name: String,
    pub epsilon: f64,
    pub delta: f64,
    pub composition: CompositionType,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionType {
    Sequential,
    Parallel,
    PostProcessing,
}

/// Simple (epsilon, delta) budget pair for composition functions.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleBudget {
    pub epsilon: f64,
    pub delta: f64,
}

impl SimpleBudget {
    pub fn new(epsilon: f64, delta: f64) -> Self {
        SimpleBudget { epsilon, delta }
    }
    pub fn pure_dp(epsilon: f64) -> Self {
        SimpleBudget {
            epsilon,
            delta: 0.0,
        }
    }
}

/// Mechanism variant used in DpMechanism.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mechanism {
    Laplace,
    Gaussian,
    Exponential,
    RandomizedResponse,
}

/// Sensitivity type for DP mechanisms.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SensitivityType {
    Global,
    Local,
    Smooth,
}

/// A concrete DP mechanism with its parameters.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DpMechanism {
    pub name: String,
    pub mechanism: Mechanism,
    pub sensitivity: f64,
    pub epsilon: f64,
    pub delta: f64,
}

impl DpMechanism {
    #[allow(dead_code)]
    pub fn new(
        name: impl Into<String>,
        mechanism: Mechanism,
        sensitivity: f64,
        epsilon: f64,
        delta: f64,
    ) -> Self {
        DpMechanism {
            name: name.into(),
            mechanism,
            sensitivity,
            epsilon,
            delta,
        }
    }
    #[allow(dead_code)]
    pub fn budget(&self) -> SimpleBudget {
        SimpleBudget::new(self.epsilon, self.delta)
    }
}

/// High-level composition theorem variant.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionTheorem {
    Sequential,
    Parallel,
    Advanced,
}
