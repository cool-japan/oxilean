//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Gaussian Orthogonal Ensemble with Dyson index β.
pub struct GOEEnsemble {
    /// Matrix dimension.
    pub n: usize,
    /// Dyson index (β = 1 for GOE, β = 2 for GUE, β = 4 for GSE).
    pub beta: f64,
}
impl GOEEnsemble {
    /// Create a new GOE ensemble.
    pub fn new(n: usize, beta: f64) -> Self {
        Self { n, beta }
    }
    /// Return the symbolic form of the joint eigenvalue density.
    /// P(λ₁,…,λₙ) ∝ ∏_{i<j} |λᵢ − λⱼ|^β · exp(-β/4 · Σ λᵢ²)
    pub fn joint_eigenvalue_density_formula(&self) -> String {
        let beta_val = self.beta;
        format!(
            "GOE joint density: P(λ₁,…,λ_{n}) ∝ ∏_{{i<j}} |λᵢ−λⱼ|^{beta_val:.1} · exp(-{beta_val:.1}/4·Σλᵢ²)",
            n = self.n, beta_val = beta_val
        )
    }
}
/// Data for free probability theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FreeProbabilityData {
    /// Free cumulants of a distribution.
    pub free_cumulants: Vec<f64>,
    /// Moments of the distribution.
    pub moments: Vec<f64>,
    /// Whether the distribution is R-diagonal.
    pub is_r_diagonal: bool,
}
#[allow(dead_code)]
impl FreeProbabilityData {
    /// Creates free probability data.
    pub fn new() -> Self {
        FreeProbabilityData {
            free_cumulants: Vec::new(),
            moments: Vec::new(),
            is_r_diagonal: false,
        }
    }
    /// Adds a moment m_k.
    pub fn add_moment(&mut self, m: f64) {
        self.moments.push(m);
    }
    /// Adds a free cumulant κ_n.
    pub fn add_free_cumulant(&mut self, k: f64) {
        self.free_cumulants.push(k);
    }
    /// Computes the R-transform R(z) = sum_{n>=1} κ_n z^{n-1} truncated.
    pub fn r_transform_truncated(&self, z: f64) -> f64 {
        self.free_cumulants
            .iter()
            .enumerate()
            .map(|(i, &k)| k * z.powi(i as i32))
            .sum()
    }
    /// Returns the S-transform description for free multiplicative convolution.
    pub fn s_transform_description(&self) -> String {
        "S-transform: relates R-transforms of free random variables".to_string()
    }
    /// Checks if κ_1 = mean and κ_2 = variance (non-commutative analog).
    pub fn first_cumulants_ok(&self, mean: f64, variance: f64, tol: f64) -> bool {
        let ok1 = self
            .free_cumulants
            .first()
            .map(|&k| (k - mean).abs() < tol)
            .unwrap_or(false);
        let ok2 = self
            .free_cumulants
            .get(1)
            .map(|&k| (k - variance).abs() < tol)
            .unwrap_or(false);
        ok1 && ok2
    }
}
/// Data for Marchenko-Pastur distribution (Wishart matrices).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MarchenkoPasturData {
    /// Ratio γ = p/n (dimension / sample size).
    pub gamma: f64,
    /// Variance σ^2.
    pub variance: f64,
}
#[allow(dead_code)]
impl MarchenkoPasturData {
    /// Creates Marchenko-Pastur data.
    pub fn new(gamma: f64, variance: f64) -> Self {
        MarchenkoPasturData { gamma, variance }
    }
    /// Returns support endpoints: \[λ-, λ+\] = σ^2 (1 ± sqrt(γ))^2.
    pub fn support_endpoints(&self) -> (f64, f64) {
        let sqrt_g = self.gamma.sqrt();
        let lo = self.variance * (1.0 - sqrt_g).powi(2);
        let hi = self.variance * (1.0 + sqrt_g).powi(2);
        (lo, hi)
    }
    /// Marchenko-Pastur density at x.
    pub fn density(&self, x: f64) -> f64 {
        let (lo, hi) = self.support_endpoints();
        if x < lo || x > hi {
            return 0.0;
        }
        let num = ((hi - x) * (x - lo)).sqrt();
        let denom = 2.0 * std::f64::consts::PI * self.variance * self.gamma * x;
        if denom.abs() < 1e-14 {
            0.0
        } else {
            num / denom
        }
    }
    /// Returns the mean of the distribution: σ^2.
    pub fn mean(&self) -> f64 {
        self.variance
    }
    /// Returns the variance of the distribution: σ^4 * γ.
    pub fn distribution_variance(&self) -> f64 {
        self.variance * self.variance * self.gamma
    }
}
/// Connections between random matrix eigenvalue statistics and the Riemann zeta function.
pub struct RiemannZetaConnections;
impl RiemannZetaConnections {
    /// Create a new RiemannZetaConnections.
    pub fn new() -> Self {
        Self
    }
    /// Return a description of the Montgomery pair correlation conjecture.
    /// Montgomery (1973): the pair correlation of zeta zeros equals the GUE sine-kernel pair correlation.
    pub fn montgomery_pair_correlation(&self) -> String {
        "Montgomery pair correlation: R₂(x) = 1 - (sin(πx)/(πx))²  [matches GUE sine kernel]"
            .to_string()
    }
}
/// Data for Wigner semicircle law verification.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WignerSemicircleData {
    /// Sample eigenvalues.
    pub eigenvalues: Vec<f64>,
    /// Matrix dimension N.
    pub dimension: usize,
    /// Variance σ^2.
    pub variance: f64,
}
#[allow(dead_code)]
impl WignerSemicircleData {
    /// Creates Wigner data.
    pub fn new(eigenvalues: Vec<f64>, variance: f64) -> Self {
        let dimension = eigenvalues.len();
        WignerSemicircleData {
            eigenvalues,
            dimension,
            variance,
        }
    }
    /// Semicircle density at x: ρ(x) = sqrt(4σ^2 - x^2) / (2π σ^2).
    pub fn semicircle_density(&self, x: f64) -> f64 {
        let r_sq = 4.0 * self.variance;
        let r = r_sq.sqrt();
        if x.abs() > r {
            return 0.0;
        }
        (r_sq - x * x).sqrt() / (std::f64::consts::PI * self.variance)
    }
    /// Estimates the empirical spectral density in interval \[a, b\].
    pub fn empirical_density(&self, a: f64, b: f64) -> f64 {
        let count = self
            .eigenvalues
            .iter()
            .filter(|&&e| e >= a && e <= b)
            .count();
        count as f64 / self.eigenvalues.len() as f64
    }
    /// Returns the endpoints of the support: \[-2σ, 2σ\].
    pub fn support_endpoints(&self) -> (f64, f64) {
        let r = 2.0 * self.variance.sqrt();
        (-r, r)
    }
    /// Checks if eigenvalues are within the bulk (support of semicircle).
    pub fn bulk_fraction(&self) -> f64 {
        let (lo, hi) = self.support_endpoints();
        let count = self
            .eigenvalues
            .iter()
            .filter(|&&e| e >= lo && e <= hi)
            .count();
        count as f64 / self.eigenvalues.len() as f64
    }
}
/// Free convolution of two probability distributions.
pub struct FreeConvolution {
    /// Description of distribution A.
    pub dist_a: String,
    /// Description of distribution B.
    pub dist_b: String,
}
impl FreeConvolution {
    /// Create a free convolution struct.
    pub fn new(dist_a: String, dist_b: String) -> Self {
        Self { dist_a, dist_b }
    }
    /// Return a description of the free cumulants of the free convolution.
    /// Key property: free cumulants of A ⊞ B are the sums of free cumulants of A and B.
    pub fn free_cumulants(&self) -> String {
        format!(
            "Free convolution {} ⊞ {}: κₙ(A⊞B) = κₙ(A) + κₙ(B)  [additive free cumulants]",
            self.dist_a, self.dist_b
        )
    }
}
/// Computes and stores the empirical spectral distribution (ESD) of a collection of eigenvalues.
pub struct EmpiricalSpectralDistribution {
    /// Stored eigenvalues.
    pub eigenvalues: Vec<f64>,
    /// Number of histogram bins.
    pub num_bins: usize,
}
impl EmpiricalSpectralDistribution {
    /// Create a new ESD from a set of eigenvalues.
    pub fn new(eigenvalues: Vec<f64>, num_bins: usize) -> Self {
        Self {
            eigenvalues,
            num_bins,
        }
    }
    /// Compute the histogram: returns (bin_centers, densities).
    pub fn histogram(&self) -> (Vec<f64>, Vec<f64>) {
        empirical_spectral_distribution(&self.eigenvalues, self.num_bins)
    }
    /// Compute L² deviation from the Wigner semicircle density with parameter σ.
    pub fn deviation_from_semicircle(&self, sigma: f64) -> f64 {
        semicircle_deviation(&self.eigenvalues, sigma, self.num_bins)
    }
    /// Compute L² deviation from the Marchenko-Pastur density with parameters γ, σ².
    pub fn deviation_from_marchenko_pastur(&self, gamma: f64, sigma_sq: f64) -> f64 {
        let (centers, empirical) = self.histogram();
        if centers.is_empty() {
            return 0.0;
        }
        centers
            .iter()
            .zip(empirical.iter())
            .map(|(&x, &emp)| {
                let mp = marchenko_pastur_density(x, gamma, sigma_sq);
                (emp - mp).powi(2)
            })
            .sum::<f64>()
            / centers.len() as f64
    }
    /// Number of eigenvalues.
    pub fn len(&self) -> usize {
        self.eigenvalues.len()
    }
    /// Whether the ESD is empty.
    pub fn is_empty(&self) -> bool {
        self.eigenvalues.is_empty()
    }
}
/// A symmetric n×n real matrix stored in row-major order
#[derive(Debug, Clone)]
pub struct SymmetricMatrix {
    pub n: usize,
    pub data: Vec<f64>,
}
impl SymmetricMatrix {
    /// Create an n×n zero matrix
    pub fn zeros(n: usize) -> Self {
        Self {
            n,
            data: vec![0.0; n * n],
        }
    }
    /// Element access
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.n + j]
    }
    /// Element mutation
    pub fn set(&mut self, i: usize, j: usize, v: f64) {
        self.data[i * self.n + j] = v;
    }
    /// Frobenius norm
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    /// Trace
    pub fn trace(&self) -> f64 {
        (0..self.n).map(|i| self.get(i, i)).sum()
    }
    /// Matrix-vector product y = A x
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        (0..self.n)
            .map(|i| (0..self.n).map(|j| self.get(i, j) * x[j]).sum())
            .collect()
    }
}
/// Approximation data for the Tracy-Widom β=2 distribution.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TracyWidomData {
    /// Maximum eigenvalue samples from GUE matrices.
    pub max_eigenvalues: Vec<f64>,
    /// Beta parameter (1=GOE, 2=GUE, 4=GSE).
    pub beta: usize,
}
#[allow(dead_code)]
impl TracyWidomData {
    /// Creates Tracy-Widom data.
    pub fn new(beta: usize) -> Self {
        TracyWidomData {
            max_eigenvalues: Vec::new(),
            beta,
        }
    }
    /// Adds a sample maximum eigenvalue.
    pub fn add_sample(&mut self, val: f64) {
        self.max_eigenvalues.push(val);
    }
    /// Returns the mean of the maximum eigenvalue distribution.
    pub fn sample_mean(&self) -> f64 {
        if self.max_eigenvalues.is_empty() {
            return 0.0;
        }
        self.max_eigenvalues.iter().sum::<f64>() / self.max_eigenvalues.len() as f64
    }
    /// Returns the sample variance.
    pub fn sample_variance(&self) -> f64 {
        if self.max_eigenvalues.len() < 2 {
            return 0.0;
        }
        let mean = self.sample_mean();
        let var: f64 = self
            .max_eigenvalues
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / (self.max_eigenvalues.len() - 1) as f64;
        var
    }
    /// Returns the description of the fluctuation scale.
    pub fn fluctuation_scale(&self, n: usize) -> f64 {
        (n as f64).powf(-2.0 / 3.0)
    }
    /// Returns whether beta is valid.
    pub fn is_valid_beta(&self) -> bool {
        matches!(self.beta, 1 | 2 | 4)
    }
}
/// Deterministic LCG pseudo-random number generator (no external crate needed)
pub struct Lcg {
    state: u64,
}
impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0xdeadbeef_cafebabe,
        }
    }
    /// Next f64 in [0, 1)
    pub fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Box-Muller: sample N(0, 1)
    pub fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-300);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}
/// Dyson Brownian Motion — stochastic flow on eigenvalue processes.
pub struct DysonBrownianMotion {
    /// Number of particles (eigenvalues).
    pub n: usize,
    /// Dyson index β.
    pub beta: f64,
}
impl DysonBrownianMotion {
    /// Create a Dyson Brownian Motion.
    pub fn new(n: usize, beta: f64) -> Self {
        Self { n, beta }
    }
    /// Return a description of the equilibrium measure.
    /// For β = 2 (GUE dynamics) the equilibrium measure is the Wigner semicircle.
    pub fn equilibrium_measure(&self) -> String {
        format!(
            "Dyson BM(n={n}, beta={beta:.1}): equilibrium measure is Wigner semicircle (beta=2) or Selberg-type density",
            n = self.n, beta = self.beta
        )
    }
}
/// Tracy-Widom distribution for the largest eigenvalue fluctuations.
pub struct TracyWidom {
    /// Dyson index β (1 = GOE, 2 = GUE, 4 = GSE).
    pub beta: f64,
}
impl TracyWidom {
    /// Create a Tracy-Widom distribution.
    pub fn new(beta: f64) -> Self {
        Self { beta }
    }
    /// Return a description of the largest eigenvalue distribution.
    pub fn largest_eigenvalue_distribution(&self) -> String {
        format!(
            "Tracy-Widom beta={b:.0}: P(lambda_max <= s) = F_beta(s) defined via Painleve II transcendent",
            b = self.beta
        )
    }
    /// Return the GUE limit (β = 2) formula.
    pub fn gue_limit(&self) -> String {
        "GUE limit: (λ_max - 2√n) · n^(1/6) → TW₂ (Tracy-Widom GUE distribution)".to_string()
    }
}
/// k-point eigenvalue correlation functions.
pub struct EigenvalueCorrelation {
    /// Matrix dimension.
    pub n: usize,
    /// Correlation order.
    pub k: usize,
}
impl EigenvalueCorrelation {
    /// Create an eigenvalue correlation struct.
    pub fn new(n: usize, k: usize) -> Self {
        Self { n, k }
    }
    /// Return a symbolic description of the k-point correlation formula.
    /// For GUE: Rₖ(x₁,…,xₖ) = det\[K_n(xᵢ,xⱼ)\]_{i,j=1}^k  where K_n is the sine kernel.
    pub fn k_point_correlation_formula(&self) -> String {
        format!(
            "R_{k}(x₁,…,x_{k}) = det[K_n(xᵢ,xⱼ)]_{{i,j=1}}^{k}  (determinantal point process, n={n})",
            k = self.k, n = self.n
        )
    }
}
/// Data describing universality class of a random matrix ensemble.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UniversalityData {
    /// Ensemble name.
    pub ensemble: String,
    /// Whether GUE universality holds.
    pub gue_universal: bool,
    /// Local statistics description.
    pub local_stats: String,
    /// Bulk scaling limit.
    pub bulk_scaling: String,
    /// Edge scaling limit.
    pub edge_scaling: String,
}
#[allow(dead_code)]
impl UniversalityData {
    /// Creates universality data.
    pub fn new(ensemble: &str) -> Self {
        UniversalityData {
            ensemble: ensemble.to_string(),
            gue_universal: false,
            local_stats: "unknown".to_string(),
            bulk_scaling: "sine kernel".to_string(),
            edge_scaling: "Airy kernel".to_string(),
        }
    }
    /// Sets GUE universality.
    pub fn with_gue_universality(mut self) -> Self {
        self.gue_universal = true;
        self
    }
    /// Sets local statistics.
    pub fn with_local_stats(mut self, stats: &str) -> Self {
        self.local_stats = stats.to_string();
        self
    }
    /// Returns the correlation kernel description.
    pub fn correlation_kernel(&self) -> String {
        if self.gue_universal {
            format!(
                "Sine kernel (bulk) + Airy kernel (edge) for {}",
                self.ensemble
            )
        } else {
            format!("Non-standard kernel for {}", self.ensemble)
        }
    }
    /// Level spacing distribution (GUE: Wigner surmise p(s) = 32/π^2 s^2 e^{-4s^2/π}).
    pub fn level_spacing_density(&self, s: f64) -> f64 {
        if !self.gue_universal {
            return (-s).exp();
        }
        let c = 32.0 / (std::f64::consts::PI * std::f64::consts::PI);
        c * s * s * (-(4.0 * s * s) / std::f64::consts::PI).exp()
    }
}
/// Gaussian Unitary Ensemble of n×n Hermitian random matrices.
pub struct GUEEnsemble {
    /// Matrix dimension.
    pub n: usize,
}
impl GUEEnsemble {
    /// Create a new GUE ensemble of size n.
    pub fn new(n: usize) -> Self {
        Self { n }
    }
    /// Return the empirical eigenvalue density description.
    /// By the Wigner semicircle law, the empirical spectral distribution converges
    /// to the semicircle distribution with radius 2√n on \[-2√n, 2√n\].
    pub fn empirical_eigenvalue_density(&self) -> String {
        let r = 2.0 * (self.n as f64).sqrt();
        format!(
            "GUE({n}): empirical spectral distribution → Wigner semicircle on [-{r:.4}, {r:.4}]",
            n = self.n
        )
    }
    /// Return a description of the GUE level spacing distribution (Wigner surmise).
    /// P(s) = (32/π²) s² exp(-4s²/π)
    pub fn level_spacing_distribution(&self) -> String {
        "GUE level spacing: P(s) = (32/π²) s² exp(-4s²/π)  [Wigner surmise, β=2]".to_string()
    }
}
/// Marchenko-Pastur law (limiting spectral distribution of Wishart matrices).
pub struct MarcenkoPastur {
    /// Aspect ratio λ = p/n (p features, n samples).
    pub lambda: f64,
    /// Scale parameter σ².
    pub sigma: f64,
}
impl MarcenkoPastur {
    /// Create a Marchenko-Pastur distribution.
    pub fn new(lambda: f64, sigma: f64) -> Self {
        Self { lambda, sigma }
    }
    /// Density at x.
    /// ρ(x) = (1/(2πσ²)) · √((λ₊-x)(x-λ₋)) / (λx) on \[λ₋, λ₊\].
    pub fn density_at(&self, x: f64) -> f64 {
        marchenko_pastur_density(x, self.lambda, self.sigma * self.sigma)
    }
    /// Support of the distribution: \[λ₋, λ₊\] where λ± = σ²(1±√λ)².
    pub fn support(&self) -> (f64, f64) {
        let s = self.sigma * self.sigma;
        let lo = s * (1.0 - self.lambda.sqrt()).powi(2);
        let hi = s * (1.0 + self.lambda.sqrt()).powi(2);
        (lo, hi)
    }
}
/// Samples Gaussian random matrices for GUE, GOE, or GSE ensembles.
pub struct GaussianMatrixSampler {
    /// Matrix dimension n.
    pub n: usize,
    /// Dyson index β determining the ensemble.
    pub beta: DysonBeta,
    /// Internal LCG state for pseudo-randomness.
    pub(super) rng: Lcg,
}
impl GaussianMatrixSampler {
    /// Create a new sampler with given dimension, ensemble, and seed.
    pub fn new(n: usize, beta: DysonBeta, seed: u64) -> Self {
        Self {
            n,
            beta,
            rng: Lcg::new(seed),
        }
    }
    /// Sample a GOE(n) matrix (real symmetric, β=1).
    pub fn sample_goe(&mut self) -> SymmetricMatrix {
        let n = self.n;
        let mut m = SymmetricMatrix::zeros(n);
        let scale_off = (1.0 / n as f64).sqrt();
        let scale_diag = (2.0 / n as f64).sqrt();
        for i in 0..n {
            m.set(i, i, self.rng.next_normal() * scale_diag);
            for j in (i + 1)..n {
                let v = self.rng.next_normal() * scale_off;
                m.set(i, j, v);
                m.set(j, i, v);
            }
        }
        m
    }
    /// Sample a GUE(n) matrix — represented by its real part (diagonal + off-diagonal).
    /// Strictly speaking GUE is complex Hermitian; here we return the real (GOE-like) analogue.
    pub fn sample_gue_real_part(&mut self) -> SymmetricMatrix {
        let n = self.n;
        let mut m = SymmetricMatrix::zeros(n);
        let scale = (1.0 / (2.0 * n as f64)).sqrt();
        for i in 0..n {
            m.set(i, i, self.rng.next_normal() * scale * 2.0_f64.sqrt());
            for j in (i + 1)..n {
                let re = self.rng.next_normal() * scale;
                m.set(i, j, re);
                m.set(j, i, re);
            }
        }
        m
    }
    /// Sample a Wigner matrix with specified entry distribution (Box-Muller normal approximation).
    pub fn sample_wigner(&mut self) -> SymmetricMatrix {
        match self.beta {
            DysonBeta::Beta1 => self.sample_goe(),
            DysonBeta::Beta2 => self.sample_gue_real_part(),
            DysonBeta::Beta4 => {
                let n = self.n;
                let mut m = SymmetricMatrix::zeros(n);
                let scale = (1.0 / (4.0 * n as f64)).sqrt();
                for i in 0..n {
                    m.set(i, i, self.rng.next_normal() * scale * 2.0_f64.sqrt());
                    for j in (i + 1)..n {
                        let v = self.rng.next_normal() * scale;
                        m.set(i, j, v);
                        m.set(j, i, v);
                    }
                }
                m
            }
        }
    }
    /// Sample eigenvalues of a random matrix from this ensemble.
    pub fn sample_eigenvalues(&mut self) -> Vec<f64> {
        let m = self.sample_wigner();
        eigenvalues_symmetric(&m)
    }
}
/// Data for Dyson Brownian Motion (eigenvalue dynamics).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DysonBrownianMotionData {
    /// Current eigenvalue positions.
    pub positions: Vec<f64>,
    /// Beta parameter (inverse temperature).
    pub beta: f64,
    /// Time step.
    pub dt: f64,
}
#[allow(dead_code)]
impl DysonBrownianMotionData {
    /// Creates Dyson Brownian motion data.
    pub fn new(positions: Vec<f64>, beta: f64, dt: f64) -> Self {
        DysonBrownianMotionData {
            positions,
            beta,
            dt,
        }
    }
    /// Returns the drift term for eigenvalue i: β * sum_{j≠i} 1/(x_i - x_j).
    pub fn drift(&self, i: usize) -> f64 {
        let xi = self.positions[i];
        self.positions
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &xj)| {
                let d = xi - xj;
                if d.abs() < 1e-14 {
                    0.0
                } else {
                    self.beta / (2.0 * d)
                }
            })
            .sum()
    }
    /// Returns the repulsion energy sum_{i<j} log|x_i - x_j|.
    pub fn log_repulsion_energy(&self) -> f64 {
        let mut energy = 0.0;
        let n = self.positions.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let d = (self.positions[i] - self.positions[j]).abs();
                if d > 1e-14 {
                    energy += d.ln();
                }
            }
        }
        energy
    }
    /// Estimates equilibrium measure support radius (semicircle law endpoint).
    pub fn equilibrium_radius(&self) -> f64 {
        let n = self.positions.len() as f64;
        2.0 * n.sqrt()
    }
}
/// Wigner semicircle distribution with radius R.
pub struct WignerSemicircle {
    /// Radius of the semicircle support \[-R, R\].
    pub r: f64,
}
impl WignerSemicircle {
    /// Create a Wigner semicircle with given radius.
    pub fn new(r: f64) -> Self {
        Self { r }
    }
    /// Density at point x: ρ(x) = (2/πR²)√(R²−x²) for |x| ≤ R, else 0.
    pub fn density_at(&self, x: f64) -> f64 {
        if x.abs() > self.r {
            0.0
        } else {
            (2.0 / (std::f64::consts::PI * self.r * self.r)) * (self.r * self.r - x * x).sqrt()
        }
    }
    /// Variance of the semicircle distribution: Var = R²/4.
    pub fn variance(&self) -> f64 {
        self.r * self.r / 4.0
    }
    /// k-th moment of the semicircle distribution.
    /// Odd moments vanish; even moments are Catalan numbers times (R/2)^k.
    pub fn moments(&self, k: u32) -> f64 {
        if k % 2 == 1 {
            return 0.0;
        }
        let p = k / 2;
        let catalan = catalan_number(p);
        let half_r_pow = (self.r / 2.0).powi(k as i32);
        catalan * half_r_pow
    }
}
/// A general random matrix (not necessarily symmetric).
pub struct RandomMatrix {
    /// Entries stored row-major.
    pub entries: Vec<Vec<f64>>,
    /// Whether the matrix is declared Hermitian (symmetric for real case).
    pub is_hermitian: bool,
}
impl RandomMatrix {
    /// Create a new random matrix.
    pub fn new(entries: Vec<Vec<f64>>, is_hermitian: bool) -> Self {
        Self {
            entries,
            is_hermitian,
        }
    }
    /// Trace of the matrix (sum of diagonal entries).
    pub fn trace(&self) -> f64 {
        let n = self
            .entries
            .len()
            .min(self.entries.first().map(|r| r.len()).unwrap_or(0));
        (0..n).map(|i| self.entries[i][i]).sum()
    }
    /// Frobenius norm: √(Σᵢⱼ |aᵢⱼ|²).
    pub fn frobenius_norm(&self) -> f64 {
        let s: f64 = self
            .entries
            .iter()
            .flat_map(|row| row.iter())
            .map(|&v| v * v)
            .sum();
        s.sqrt()
    }
}
/// Evaluates the Stieltjes (Cauchy) transform of an empirical spectral measure.
pub struct StieltjesTransformEval {
    /// Eigenvalues forming the empirical measure.
    pub eigenvalues: Vec<f64>,
}
impl StieltjesTransformEval {
    /// Create a new evaluator.
    pub fn new(eigenvalues: Vec<f64>) -> Self {
        Self { eigenvalues }
    }
    /// Evaluate G(z) = (1/n) Σ 1/(z − λᵢ) at z = x + iε.
    /// Returns (Re G(z), Im G(z)).
    pub fn eval(&self, x: f64, epsilon: f64) -> (f64, f64) {
        stieltjes_transform(&self.eigenvalues, x, epsilon)
    }
    /// Approximate spectral density via Stieltjes inversion: ρ(x) ≈ (1/π) |Im G(x + iε)|.
    pub fn density_approx(&self, x: f64, epsilon: f64) -> f64 {
        let (_, im) = self.eval(x, epsilon);
        -im / std::f64::consts::PI
    }
    /// Check whether Im G(x + iε) ≤ 0 for ε > 0 (required for a valid Stieltjes transform).
    pub fn is_valid_stieltjes(&self, x: f64, epsilon: f64) -> bool {
        let (_, im) = self.eval(x, epsilon);
        im <= 1e-14
    }
    /// Compute the R-transform approximation: R(z) ≈ G⁻¹(-1/z) - 1/z.
    /// This is a numerical approximation for small |z|.
    pub fn r_transform_approx(&self, z_real: f64, z_imag: f64) -> (f64, f64) {
        let (g_re, g_im) = self.eval(z_real, z_imag.abs().max(1e-8));
        let w_sq = g_re * g_re + g_im * g_im;
        if w_sq < 1e-28 {
            return (0.0, 0.0);
        }
        let inv_g_re = g_re / w_sq;
        let inv_g_im = -g_im / w_sq;
        (z_real - inv_g_re, z_imag - inv_g_im)
    }
}
/// Dyson index β for each classical ensemble
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DysonBeta {
    /// GOE: real symmetric, β = 1
    Beta1,
    /// GUE: complex Hermitian, β = 2
    Beta2,
    /// GSE: quaternion self-dual, β = 4
    Beta4,
}
impl DysonBeta {
    /// Return the integer value of β
    pub fn value(self) -> u32 {
        match self {
            DysonBeta::Beta1 => 1,
            DysonBeta::Beta2 => 2,
            DysonBeta::Beta4 => 4,
        }
    }
}
/// Computes level-spacing statistics from a set of eigenvalues.
pub struct LevelSpacingStats {
    /// Normalised level spacings s_i = (λ_{i+1} − λᵢ) / ⟨s⟩.
    pub spacings: Vec<f64>,
}
impl LevelSpacingStats {
    /// Create level spacing statistics from raw eigenvalues.
    pub fn from_eigenvalues(eigenvalues: &[f64]) -> Self {
        Self {
            spacings: level_spacings(eigenvalues),
        }
    }
    /// Mean spacing (should be ≈ 1 after normalisation).
    pub fn mean(&self) -> f64 {
        if self.spacings.is_empty() {
            return 0.0;
        }
        self.spacings.iter().sum::<f64>() / self.spacings.len() as f64
    }
    /// Variance of spacings.
    pub fn variance(&self) -> f64 {
        if self.spacings.is_empty() {
            return 0.0;
        }
        let m = self.mean();
        self.spacings.iter().map(|s| (s - m).powi(2)).sum::<f64>() / self.spacings.len() as f64
    }
    /// Ratio statistic rₙ = min(sₙ, sₙ₊₁) / max(sₙ, sₙ₊₁) — avoids unfolding.
    pub fn ratio_statistics(&self) -> Vec<f64> {
        self.spacings
            .windows(2)
            .map(|w| {
                let (a, b) = (w[0].abs(), w[1].abs());
                let (lo, hi) = if a < b { (a, b) } else { (b, a) };
                if hi < 1e-14 {
                    0.0
                } else {
                    lo / hi
                }
            })
            .collect()
    }
    /// GUE Wigner surmise value at each observed spacing.
    pub fn gue_surmise_values(&self) -> Vec<f64> {
        self.spacings
            .iter()
            .map(|&s| gue_level_spacing_distribution(s))
            .collect()
    }
    /// GOE Wigner surmise value at each observed spacing.
    pub fn goe_surmise_values(&self) -> Vec<f64> {
        self.spacings
            .iter()
            .map(|&s| goe_level_spacing_distribution(s))
            .collect()
    }
    /// Compute number variance Σ²(L) from the unfolded spacings.
    pub fn number_variance(&self, l: f64) -> f64 {
        let mut unfolded = vec![0.0f64];
        for &s in &self.spacings {
            unfolded.push(
                unfolded
                    .last()
                    .expect("unfolded is non-empty: initialized with 0.0")
                    + s,
            );
        }
        number_variance(&unfolded, l)
    }
    /// Estimate Dyson β from the mean ratio statistic.
    /// GUE: ⟨r⟩ ≈ 0.5996; GOE: ⟨r⟩ ≈ 0.5307; Poisson: ⟨r⟩ ≈ 0.3863.
    pub fn estimate_beta(&self) -> f64 {
        let ratios = self.ratio_statistics();
        if ratios.is_empty() {
            return 0.0;
        }
        let mean_r = ratios.iter().sum::<f64>() / ratios.len() as f64;
        if mean_r < 0.42 {
            0.0
        } else if mean_r < 0.52 {
            1.0
        } else {
            2.0
        }
    }
}
