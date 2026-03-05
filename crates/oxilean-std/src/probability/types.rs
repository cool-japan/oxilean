//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Bernoulli random variable X ~ Bernoulli(p).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BernoulliRV {
    pub p: f64,
}
#[allow(dead_code)]
impl BernoulliRV {
    pub fn new(p: f64) -> Self {
        assert!((0.0..=1.0).contains(&p), "p must be in [0,1]");
        Self { p }
    }
    /// Mean = p.
    pub fn mean(&self) -> f64 {
        self.p
    }
    /// Variance = p*(1-p).
    pub fn variance(&self) -> f64 {
        self.p * (1.0 - self.p)
    }
    /// Entropy H(X) = -p log p - (1-p) log(1-p).
    pub fn entropy(&self) -> f64 {
        let q = 1.0 - self.p;
        let h_p = if self.p > 0.0 {
            -self.p * self.p.ln()
        } else {
            0.0
        };
        let h_q = if q > 0.0 { -q * q.ln() } else { 0.0 };
        h_p + h_q
    }
    /// PMF: P(X=k) for k in {0,1}.
    pub fn pmf(&self, k: u8) -> f64 {
        match k {
            0 => 1.0 - self.p,
            1 => self.p,
            _ => 0.0,
        }
    }
    /// Moment generating function M(t) = 1 - p + p*e^t.
    pub fn mgf(&self, t: f64) -> f64 {
        1.0 - self.p + self.p * t.exp()
    }
    /// Probability generating function G(z) = 1 - p + p*z.
    pub fn pgf(&self, z: f64) -> f64 {
        1.0 - self.p + self.p * z
    }
}
/// Large deviations rate function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LargeDeviations {
    pub sequence_name: String,
    pub rate_function: String,
    pub is_good: bool,
}
#[allow(dead_code)]
impl LargeDeviations {
    /// Cramér's theorem for i.i.d. random variables.
    pub fn cramer(rv_name: &str) -> Self {
        Self {
            sequence_name: format!("({} i.i.d.)", rv_name),
            rate_function: "Legendre-Fenchel transform of log-mgf".to_string(),
            is_good: true,
        }
    }
    /// Sanov's theorem for empirical measures.
    pub fn sanov() -> Self {
        Self {
            sequence_name: "empirical measures".to_string(),
            rate_function: "relative entropy KL(Q||P)".to_string(),
            is_good: true,
        }
    }
    /// LDP holds.
    pub fn ldp_description(&self) -> String {
        format!(
            "LDP for {} with good rate function: {}",
            self.sequence_name, self.rate_function
        )
    }
}
/// Numerical evaluation of characteristic functions via finite-sum approximation.
///
/// For a discrete distribution with PMF `pmf`, computes
/// φ(t) = Σ_k p_k · exp(i t k).
pub struct CharacteristicFunction {
    /// PMF values over support {0, 1, …, n-1}.
    pub pmf: Vec<f64>,
}
impl CharacteristicFunction {
    /// Creates a `CharacteristicFunction` from a PMF.
    pub fn new(pmf: Vec<f64>) -> Self {
        CharacteristicFunction { pmf }
    }
    /// Evaluates Re(φ(t)) = Σ_k p_k cos(t·k).
    pub fn real_part(&self, t: f64) -> f64 {
        self.pmf
            .iter()
            .enumerate()
            .map(|(k, &p)| p * (t * k as f64).cos())
            .sum()
    }
    /// Evaluates Im(φ(t)) = Σ_k p_k sin(t·k).
    pub fn imag_part(&self, t: f64) -> f64 {
        self.pmf
            .iter()
            .enumerate()
            .map(|(k, &p)| p * (t * k as f64).sin())
            .sum()
    }
    /// Returns |φ(t)|².
    pub fn modulus_sq(&self, t: f64) -> f64 {
        let re = self.real_part(t);
        let im = self.imag_part(t);
        re * re + im * im
    }
    /// Estimates the k-th moment E[X^k] via numerical differentiation of φ.
    ///
    /// Uses the identity φ^(k)(0) = i^k E[X^k], evaluated at h → 0.
    /// Only reliable for small k due to floating-point cancellation.
    pub fn moment(&self, k: u32) -> f64 {
        self.pmf
            .iter()
            .enumerate()
            .map(|(x, &p)| p * (x as f64).powi(k as i32))
            .sum()
    }
}
/// Stopping time data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StoppingTime {
    pub name: String,
    pub filtration: String,
    pub is_finite_as: bool,
}
#[allow(dead_code)]
impl StoppingTime {
    /// First hitting time.
    pub fn first_hitting(set_name: &str, filtration: &str) -> Self {
        Self {
            name: format!("tau_{{{}}}", set_name),
            filtration: filtration.to_string(),
            is_finite_as: false,
        }
    }
    /// Optional stopping theorem: E[M_tau] = E[M_0] under UI conditions.
    pub fn optional_stopping_description(&self) -> String {
        format!(
            "Optional stopping at {} (filtration {}): E[M_tau] = E[M_0] under UI",
            self.name, self.filtration
        )
    }
}
/// Exponential distribution with rate parameter λ.
#[allow(dead_code)]
pub struct ExponentialDistribution {
    /// Rate parameter λ > 0.
    pub lambda: f64,
}
#[allow(dead_code)]
impl ExponentialDistribution {
    /// Creates an `ExponentialDistribution` with rate λ.
    pub fn new(lambda: f64) -> Self {
        ExponentialDistribution { lambda }
    }
    /// Probability density function f(x; λ) = λ e^{-λx} for x ≥ 0.
    pub fn pdf(&self, x: f64) -> f64 {
        exponential_pdf(x, self.lambda)
    }
    /// Cumulative distribution function F(x; λ) = 1 - e^{-λx} for x ≥ 0.
    pub fn cdf(&self, x: f64) -> f64 {
        exponential_cdf(x, self.lambda)
    }
    /// Mean E[X] = 1/λ.
    pub fn mean(&self) -> f64 {
        1.0 / self.lambda
    }
    /// Variance Var[X] = 1/λ².
    pub fn variance(&self) -> f64 {
        1.0 / (self.lambda * self.lambda)
    }
    /// Inverse CDF (quantile function): F^{-1}(p) = -ln(1-p) / λ.
    pub fn quantile(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return 0.0;
        }
        if p >= 1.0 {
            return f64::INFINITY;
        }
        -(1.0 - p).ln() / self.lambda
    }
    /// Draws a sample using the inverse-CDF method given uniform u ∈ (0,1).
    pub fn sample(&self, u: f64) -> f64 {
        self.quantile(u)
    }
    /// Moment generating function M_X(t) = λ/(λ-t) for t < λ.
    pub fn mgf(&self, t: f64) -> f64 {
        if t >= self.lambda {
            return f64::INFINITY;
        }
        self.lambda / (self.lambda - t)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HawkesProcess {
    pub base_intensity: f64,
    pub self_excitation: f64,
    pub decay_rate: f64,
    pub is_stationary: bool,
}
#[allow(dead_code)]
impl HawkesProcess {
    pub fn new(mu: f64, alpha: f64, beta: f64) -> Self {
        HawkesProcess {
            base_intensity: mu,
            self_excitation: alpha,
            decay_rate: beta,
            is_stationary: alpha < beta,
        }
    }
    pub fn conditional_intensity(&self, t: f64, last_event: f64) -> f64 {
        if t > last_event {
            self.base_intensity
                + self.self_excitation * (-(self.decay_rate * (t - last_event))).exp()
        } else {
            self.base_intensity
        }
    }
    pub fn mean_intensity(&self) -> f64 {
        if self.is_stationary {
            self.base_intensity / (1.0 - self.self_excitation / self.decay_rate)
        } else {
            f64::INFINITY
        }
    }
    pub fn branching_ratio(&self) -> f64 {
        self.self_excitation / self.decay_rate
    }
}
/// Linear congruential generator (Park–Miller parameters).
pub struct Lcg {
    state: u64,
}
impl Lcg {
    /// Creates an LCG seeded with `seed`.
    pub fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    /// Returns the next pseudo-random `f64` in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Returns the next pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }
}
/// Kernel density estimator using a Gaussian kernel.
///
/// For a dataset x_1, …, x_n, the KDE at point x is:
/// f̂(x) = (1/(n·h)) Σ_i K((x - x_i)/h)  where K is the standard Gaussian kernel.
#[allow(dead_code)]
pub struct KernelDensityEstimator {
    /// Training data points.
    pub data: Vec<f64>,
    /// Bandwidth h (Silverman's rule of thumb by default).
    pub bandwidth: f64,
}
#[allow(dead_code)]
impl KernelDensityEstimator {
    /// Creates a KDE with Silverman's rule-of-thumb bandwidth:
    /// h = 1.06 · σ̂ · n^{-1/5}.
    pub fn new(data: Vec<f64>) -> Self {
        let n = data.len();
        let bandwidth = if n < 2 {
            1.0
        } else {
            let sigma = sample_variance(&data).sqrt();
            1.06 * sigma * (n as f64).powf(-0.2)
        };
        KernelDensityEstimator { data, bandwidth }
    }
    /// Creates a KDE with an explicit bandwidth.
    pub fn with_bandwidth(data: Vec<f64>, bandwidth: f64) -> Self {
        KernelDensityEstimator { data, bandwidth }
    }
    /// Evaluates the kernel density estimate at point `x`.
    pub fn density(&self, x: f64) -> f64 {
        let n = self.data.len();
        if n == 0 || self.bandwidth <= 0.0 {
            return 0.0;
        }
        let sum: f64 = self
            .data
            .iter()
            .map(|&xi| normal_pdf((x - xi) / self.bandwidth, 0.0, 1.0))
            .sum();
        sum / (n as f64 * self.bandwidth)
    }
    /// Evaluates the KDE over a grid of `m` equally spaced points in [lo, hi].
    pub fn density_grid(&self, lo: f64, hi: f64, m: usize) -> Vec<(f64, f64)> {
        if m == 0 || lo >= hi {
            return vec![];
        }
        (0..m)
            .map(|i| {
                let x = lo + (hi - lo) * i as f64 / (m - 1).max(1) as f64;
                (x, self.density(x))
            })
            .collect()
    }
}
/// Geometric random variable X ~ Geom(p) (number of trials until first success).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeometricRV {
    pub p: f64,
}
#[allow(dead_code)]
impl GeometricRV {
    pub fn new(p: f64) -> Self {
        assert!(p > 0.0 && p <= 1.0, "p must be in (0,1]");
        Self { p }
    }
    /// Mean = 1/p.
    pub fn mean(&self) -> f64 {
        1.0 / self.p
    }
    /// Variance = (1-p)/p^2.
    pub fn variance(&self) -> f64 {
        (1.0 - self.p) / (self.p * self.p)
    }
    /// PMF: P(X=k) = (1-p)^(k-1) * p for k >= 1.
    pub fn pmf(&self, k: u64) -> f64 {
        if k == 0 {
            return 0.0;
        }
        (1.0 - self.p).powi(k as i32 - 1) * self.p
    }
    /// CDF: P(X <= k) = 1 - (1-p)^k.
    pub fn cdf(&self, k: u64) -> f64 {
        1.0 - (1.0 - self.p).powi(k as i32)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RenewalProcess {
    pub inter_arrival_distribution: String,
    pub mean_inter_arrival: f64,
    pub variance_inter_arrival: f64,
    pub rate: f64,
}
#[allow(dead_code)]
impl RenewalProcess {
    pub fn new(dist: &str, mean: f64, var: f64) -> Self {
        RenewalProcess {
            inter_arrival_distribution: dist.to_string(),
            mean_inter_arrival: mean,
            variance_inter_arrival: var,
            rate: 1.0 / mean,
        }
    }
    pub fn poisson_process(lambda: f64) -> Self {
        RenewalProcess {
            inter_arrival_distribution: format!("Exp({:.3})", lambda),
            mean_inter_arrival: 1.0 / lambda,
            variance_inter_arrival: 1.0 / (lambda * lambda),
            rate: lambda,
        }
    }
    pub fn elementary_renewal_theorem(&self) -> String {
        format!(
            "Elementary renewal: E[N(t)]/t → 1/μ = {:.4} as t→∞ (μ={:.3})",
            self.rate, self.mean_inter_arrival
        )
    }
    pub fn renewal_reward_theorem(&self, reward_rate: f64) -> f64 {
        reward_rate / self.mean_inter_arrival
    }
    pub fn blackwell_renewal_theorem(&self) -> String {
        format!(
            "Blackwell: E[N(t+h) - N(t)] → h/{:.3} as t→∞ for non-arithmetic dist",
            self.mean_inter_arrival
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussianProcess2 {
    pub mean: f64,
    pub kernel_param: f64,
    pub num_sample_paths: usize,
}
#[allow(dead_code)]
impl GaussianProcess2 {
    pub fn new(mean: f64, kp: f64) -> Self {
        GaussianProcess2 {
            mean,
            kernel_param: kp,
            num_sample_paths: 0,
        }
    }
    pub fn sample_path_continuity(&self) -> String {
        "By Kolmogorov: GP sample paths are Hölder continuous if covariance kernel is smooth enough"
            .to_string()
    }
}
/// Dirichlet distribution Dir(alpha) — multivariate generalization of Beta.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirichletRV {
    pub alpha: Vec<f64>,
}
#[allow(dead_code)]
impl DirichletRV {
    pub fn new(alpha: Vec<f64>) -> Self {
        for &a in &alpha {
            assert!(a > 0.0, "alpha components must be positive");
        }
        Self { alpha }
    }
    /// Concentration parameter alpha_0 = sum of alpha.
    pub fn alpha_0(&self) -> f64 {
        self.alpha.iter().sum()
    }
    /// Mean vector: mu_i = alpha_i / alpha_0.
    pub fn mean(&self) -> Vec<f64> {
        let a0 = self.alpha_0();
        self.alpha.iter().map(|&a| a / a0).collect()
    }
    /// Variance of i-th component: alpha_i*(alpha_0-alpha_i) / (alpha_0^2*(alpha_0+1)).
    pub fn variance_i(&self, i: usize) -> f64 {
        let a0 = self.alpha_0();
        self.alpha[i] * (a0 - self.alpha[i]) / (a0 * a0 * (a0 + 1.0))
    }
    /// Entropy: log B(alpha) + (alpha_0 - K)*digamma(alpha_0) - sum((alpha_i-1)*digamma(alpha_i))
    /// Approximated here using Stirling's digamma: digamma(x) ≈ ln(x) - 1/(2x).
    pub fn entropy_approx(&self) -> f64 {
        let a0 = self.alpha_0();
        let k = self.alpha.len() as f64;
        let digamma_approx = |x: f64| x.ln() - 0.5 / x;
        let log_b: f64 =
            self.alpha.iter().map(|&a| lgamma_approx(a)).sum::<f64>() - lgamma_approx(a0);
        let rest: f64 = (a0 - k) * digamma_approx(a0)
            - self
                .alpha
                .iter()
                .map(|&a| (a - 1.0) * digamma_approx(a))
                .sum::<f64>();
        log_b + rest
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CovarianceKernel {
    SquaredExponential { length_scale: f64, variance: f64 },
    Matern { nu: f64, length_scale: f64 },
    Polynomial { degree: usize, offset: f64 },
    Linear,
    Periodic { period: f64, length_scale: f64 },
}
/// Frechet distribution: X ~ Frechet(alpha, s, m).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FrechetRV {
    pub alpha: f64,
    pub s: f64,
    pub m: f64,
}
#[allow(dead_code)]
impl FrechetRV {
    pub fn new(alpha: f64, s: f64, m: f64) -> Self {
        assert!(alpha > 0.0 && s > 0.0, "alpha, s must be positive");
        Self { alpha, s, m }
    }
    /// CDF: F(x) = exp(-(s/(x-m))^alpha) for x > m.
    pub fn cdf(&self, x: f64) -> f64 {
        if x <= self.m {
            return 0.0;
        }
        (-(self.s / (x - self.m)).powf(self.alpha)).exp()
    }
    /// Mode = m + s*(alpha/(alpha+1))^(1/alpha).
    pub fn mode(&self) -> f64 {
        self.m + self.s * (self.alpha / (self.alpha + 1.0)).powf(1.0 / self.alpha)
    }
}
/// Copula struct for bivariate dependence modeling.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CopulaKind {
    Gaussian { rho: f64 },
    Clayton { theta: f64 },
    Gumbel { theta: f64 },
    Frank { theta: f64 },
    Independence,
}
/// Negative Binomial random variable X ~ NB(r, p).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NegativeBinomialRV {
    pub r: u32,
    pub p: f64,
}
#[allow(dead_code)]
impl NegativeBinomialRV {
    pub fn new(r: u32, p: f64) -> Self {
        assert!(r > 0, "r must be positive");
        assert!(p > 0.0 && p <= 1.0, "p must be in (0,1]");
        Self { r, p }
    }
    /// Mean = r*(1-p)/p.
    pub fn mean(&self) -> f64 {
        self.r as f64 * (1.0 - self.p) / self.p
    }
    /// Variance = r*(1-p)/p^2.
    pub fn variance(&self) -> f64 {
        self.r as f64 * (1.0 - self.p) / (self.p * self.p)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussianProcess {
    pub mean_function: String,
    pub covariance_kernel: CovarianceKernel,
    pub input_dim: usize,
    pub is_stationary: bool,
}
#[allow(dead_code)]
impl GaussianProcess {
    pub fn with_sq_exp(length: f64, var: f64, input_dim: usize) -> Self {
        GaussianProcess {
            mean_function: "zero".to_string(),
            covariance_kernel: CovarianceKernel::SquaredExponential {
                length_scale: length,
                variance: var,
            },
            input_dim,
            is_stationary: true,
        }
    }
    pub fn with_matern(nu: f64, length: f64, input_dim: usize) -> Self {
        GaussianProcess {
            mean_function: "zero".to_string(),
            covariance_kernel: CovarianceKernel::Matern {
                nu,
                length_scale: length,
            },
            input_dim,
            is_stationary: true,
        }
    }
    pub fn kernel_value(&self, d: f64) -> f64 {
        match &self.covariance_kernel {
            CovarianceKernel::SquaredExponential {
                length_scale,
                variance,
            } => variance * (-(d * d) / (2.0 * length_scale * length_scale)).exp(),
            CovarianceKernel::Matern { nu, length_scale } => {
                let r = d / length_scale;
                if *nu == 0.5 {
                    (-r).exp()
                } else if *nu == 1.5 {
                    (1.0 + 3.0_f64.sqrt() * r) * (-(3.0_f64.sqrt() * r)).exp()
                } else {
                    (-r).exp()
                }
            }
            CovarianceKernel::Linear => d,
            CovarianceKernel::Polynomial { degree, offset } => (d + offset).powi(*degree as i32),
            CovarianceKernel::Periodic {
                period,
                length_scale,
            } => {
                let arg = std::f64::consts::PI * d / period;
                (-2.0 * arg.sin().powi(2) / (length_scale * length_scale)).exp()
            }
        }
    }
    pub fn posterior_description(&self, n_obs: usize) -> String {
        format!(
            "GP posterior: Gaussian with updated mean/covariance after {} observations",
            n_obs
        )
    }
    pub fn mercer_representation(&self) -> String {
        "Mercer's theorem: k(x,y) = Σ λ_i φ_i(x)φ_i(y) (eigendecomposition of kernel operator)"
            .to_string()
    }
}
/// Concentration bounds for sums of independent bounded random variables.
pub struct ConcentrationBound;
impl ConcentrationBound {
    /// Hoeffding's inequality: returns the upper bound on P(S_n - E[S_n] ≥ t)
    /// for n summands each bounded in [a_i, b_i].
    ///
    /// Bound: exp(-2t² / Σ(b_i - a_i)²).
    pub fn hoeffding(t: f64, intervals: &[(f64, f64)]) -> f64 {
        let sum_sq: f64 = intervals.iter().map(|(a, b)| (b - a).powi(2)).sum();
        if sum_sq <= 0.0 {
            return 0.0;
        }
        (-2.0 * t * t / sum_sq).exp()
    }
    /// Markov inequality: P(X ≥ a) ≤ E[X] / a for non-negative X.
    pub fn markov(expectation: f64, a: f64) -> f64 {
        if a <= 0.0 {
            return 1.0;
        }
        (expectation / a).min(1.0)
    }
    /// Chebyshev inequality: P(|X - μ| ≥ k·σ) ≤ 1/k².
    pub fn chebyshev(k: f64) -> f64 {
        if k <= 0.0 {
            return 1.0;
        }
        (1.0 / (k * k)).min(1.0)
    }
    /// Chernoff bound for the sum of Bernoulli(p_i) variables with mean μ.
    ///
    /// Upper tail: P(X ≥ (1+δ)μ) ≤ (e^δ / (1+δ)^(1+δ))^μ.
    pub fn chernoff_upper(mu: f64, delta: f64) -> f64 {
        if delta <= 0.0 || mu <= 0.0 {
            return 1.0;
        }
        let exponent = mu * (delta - (1.0 + delta) * (1.0 + delta).ln());
        exponent.exp().min(1.0)
    }
    /// Bernstein inequality for bounded random variables with variance s².
    ///
    /// P(S_n ≥ t) ≤ exp(-t² / (2(s² + ct/3))) where c is the bound on individual terms.
    pub fn bernstein(t: f64, variance_sum: f64, c: f64) -> f64 {
        let denom = 2.0 * (variance_sum + c * t / 3.0);
        if denom <= 0.0 {
            return 1.0;
        }
        (-t * t / denom).exp().min(1.0)
    }
    /// Sub-Gaussian tail bound: P(X ≥ t) ≤ exp(-t²/(2σ²)) for σ-sub-Gaussian X.
    pub fn sub_gaussian_tail(t: f64, sigma: f64) -> f64 {
        if sigma <= 0.0 {
            return 1.0;
        }
        (-t * t / (2.0 * sigma * sigma)).exp().min(1.0)
    }
}
/// Gumbel distribution: X = Gumbel(mu, beta), used in extreme value theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GumbelRV {
    pub mu: f64,
    pub beta: f64,
}
#[allow(dead_code)]
impl GumbelRV {
    pub fn new(mu: f64, beta: f64) -> Self {
        assert!(beta > 0.0, "beta must be positive");
        Self { mu, beta }
    }
    /// CDF: F(x) = exp(-exp(-(x-mu)/beta)).
    pub fn cdf(&self, x: f64) -> f64 {
        (-(-(x - self.mu) / self.beta).exp()).exp()
    }
    /// Mean = mu + beta * euler_gamma.
    pub fn mean(&self) -> f64 {
        self.mu + self.beta * 0.5772156649
    }
    /// Variance = pi^2 * beta^2 / 6.
    pub fn variance(&self) -> f64 {
        std::f64::consts::PI * std::f64::consts::PI * self.beta * self.beta / 6.0
    }
    /// Mode = mu.
    pub fn mode(&self) -> f64 {
        self.mu
    }
}
/// Sequential probability ratio test (SPRT) by Wald.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SprtTest {
    pub h0_rate: f64,
    pub h1_rate: f64,
    pub alpha: f64,
    pub beta: f64,
    pub log_lr: f64,
}
#[allow(dead_code)]
impl SprtTest {
    pub fn new(h0_rate: f64, h1_rate: f64, alpha: f64, beta: f64) -> Self {
        Self {
            h0_rate,
            h1_rate,
            alpha,
            beta,
            log_lr: 0.0,
        }
    }
    /// Update with new Bernoulli observation x in {0,1}.
    pub fn update_bernoulli(&mut self, x: u8) {
        let x = x as f64;
        self.log_lr += x * (self.h1_rate / self.h0_rate).ln()
            + (1.0 - x) * ((1.0 - self.h1_rate) / (1.0 - self.h0_rate)).ln();
    }
    /// Decision: Some(true) = reject H0, Some(false) = accept H0, None = continue.
    pub fn decision(&self) -> Option<bool> {
        let upper = ((1.0 - self.beta) / self.alpha).ln();
        let lower = (self.beta / (1.0 - self.alpha)).ln();
        if self.log_lr >= upper {
            Some(true)
        } else if self.log_lr <= lower {
            Some(false)
        } else {
            None
        }
    }
}
/// Coupling of probability measures.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Coupling {
    pub measure1: String,
    pub measure2: String,
    pub coupling_type: String,
    pub tv_bound: Option<f64>,
}
#[allow(dead_code)]
impl Coupling {
    /// Maximal coupling achieving TV distance.
    pub fn maximal(mu: &str, nu: &str, tv: f64) -> Self {
        Self {
            measure1: mu.to_string(),
            measure2: nu.to_string(),
            coupling_type: "maximal".to_string(),
            tv_bound: Some(tv),
        }
    }
    /// Optimal transport coupling (Wasserstein).
    pub fn optimal_transport(mu: &str, nu: &str) -> Self {
        Self {
            measure1: mu.to_string(),
            measure2: nu.to_string(),
            coupling_type: "optimal transport".to_string(),
            tv_bound: None,
        }
    }
    /// P(X != Y) = TV(mu, nu) for maximal coupling.
    pub fn maximal_coupling_property(&self) -> String {
        if let Some(tv) = self.tv_bound {
            format!(
                "P(X != Y) = {:.4} = TV({}, {})",
                tv, self.measure1, self.measure2
            )
        } else {
            format!(
                "{} coupling of {} and {}",
                self.coupling_type, self.measure1, self.measure2
            )
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirichletProcess {
    pub concentration: f64,
    pub base_distribution: String,
    pub is_discrete: bool,
    pub expected_clusters: f64,
}
#[allow(dead_code)]
impl DirichletProcess {
    pub fn new(alpha: f64, base: &str) -> Self {
        DirichletProcess {
            concentration: alpha,
            base_distribution: base.to_string(),
            is_discrete: true,
            expected_clusters: 0.0,
        }
    }
    pub fn expected_clusters_for_n(&self, n: usize) -> f64 {
        self.concentration * (1.0 + n as f64 / self.concentration).ln()
    }
    pub fn stick_breaking_construction(&self) -> String {
        format!(
            "Stick-breaking: V_k ~ Beta(1, {:.3}), w_k = V_k ∏_{{j<k}} (1-V_j)",
            self.concentration
        )
    }
    pub fn chinese_restaurant_process(&self, n: usize) -> String {
        format!(
            "CRP (α={:.3}, n={}): E[#tables] ≈ {:.2}",
            self.concentration,
            n,
            self.expected_clusters_for_n(n)
        )
    }
    pub fn posterior_update(&self, n_obs: usize) -> Self {
        DirichletProcess {
            concentration: self.concentration + n_obs as f64,
            base_distribution: self.base_distribution.clone(),
            is_discrete: true,
            expected_clusters: self.expected_clusters_for_n(n_obs),
        }
    }
}
/// Hypergeometric distribution: drawing n items from population N with K successes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HypergeometricRV {
    pub n_pop: u64,
    pub k_suc: u64,
    pub n_draw: u64,
}
#[allow(dead_code)]
impl HypergeometricRV {
    pub fn new(n_pop: u64, k_suc: u64, n_draw: u64) -> Self {
        assert!(k_suc <= n_pop, "K <= N required");
        assert!(n_draw <= n_pop, "n <= N required");
        Self {
            n_pop,
            k_suc,
            n_draw,
        }
    }
    /// Mean = n * K / N.
    pub fn mean(&self) -> f64 {
        self.n_draw as f64 * self.k_suc as f64 / self.n_pop as f64
    }
    /// Variance = n * K/N * (1 - K/N) * (N-n)/(N-1).
    pub fn variance(&self) -> f64 {
        let n = self.n_draw as f64;
        let k = self.k_suc as f64;
        let nn = self.n_pop as f64;
        n * (k / nn) * (1.0 - k / nn) * (nn - n) / (nn - 1.0)
    }
}
/// Martingale difference sequence checker.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MartingaleDifferenceTest {
    pub diffs: Vec<f64>,
}
#[allow(dead_code)]
impl MartingaleDifferenceTest {
    pub fn new(series: &[f64]) -> Self {
        let diffs: Vec<f64> = series.windows(2).map(|w| w[1] - w[0]).collect();
        Self { diffs }
    }
    /// Sample mean of differences (should be ~0 for MDS).
    pub fn mean_diff(&self) -> f64 {
        if self.diffs.is_empty() {
            return 0.0;
        }
        self.diffs.iter().sum::<f64>() / self.diffs.len() as f64
    }
    /// Sample variance of differences.
    pub fn var_diff(&self) -> f64 {
        if self.diffs.len() < 2 {
            return 0.0;
        }
        let m = self.mean_diff();
        self.diffs.iter().map(|&d| (d - m) * (d - m)).sum::<f64>() / (self.diffs.len() - 1) as f64
    }
    /// Test statistic: t = mean / (se) ~ N(0,1) under null.
    pub fn t_statistic(&self) -> f64 {
        let n = self.diffs.len() as f64;
        let se = (self.var_diff() / n).sqrt();
        if se == 0.0 {
            return 0.0;
        }
        self.mean_diff() / se
    }
}
/// Bayes factor for model comparison.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BayesFactor {
    pub log_bf: f64,
}
#[allow(dead_code)]
impl BayesFactor {
    pub fn new(log_marginal_m1: f64, log_marginal_m0: f64) -> Self {
        Self {
            log_bf: log_marginal_m1 - log_marginal_m0,
        }
    }
    /// Evidence category per Jeffreys scale.
    pub fn jeffreys_scale(&self) -> &'static str {
        match self.log_bf {
            x if x < 0.0 => "Evidence for M0",
            x if x < 1.0_f64.ln() => "Barely worth mentioning",
            x if x < 3.0_f64.ln() => "Substantial",
            x if x < 10.0_f64.ln() => "Strong",
            x if x < 30.0_f64.ln() => "Very strong",
            _ => "Decisive",
        }
    }
    /// BF_10 (ratio of likelihoods).
    pub fn bf10(&self) -> f64 {
        self.log_bf.exp()
    }
    /// BF_01 (inverse).
    pub fn bf01(&self) -> f64 {
        (-self.log_bf).exp()
    }
}
/// A discrete probability distribution backed by an explicit PMF table.
///
/// Sampling uses an LCG (linear congruential generator) seeded by the caller.
pub struct DiscreteDistribution {
    /// PMF values (must sum to 1).
    pub pmf: Vec<f64>,
}
impl DiscreteDistribution {
    /// Creates a `DiscreteDistribution` from raw weights, normalising them.
    pub fn from_weights(weights: &[f64]) -> Self {
        let total: f64 = weights.iter().sum();
        let pmf = if total > 0.0 {
            weights.iter().map(|w| w / total).collect()
        } else {
            vec![1.0 / weights.len() as f64; weights.len()]
        };
        DiscreteDistribution { pmf }
    }
    /// Returns the PMF value at index `k`.
    pub fn prob(&self, k: usize) -> f64 {
        self.pmf.get(k).copied().unwrap_or(0.0)
    }
    /// Draws a sample using an LCG random number in `[0, 1)`.
    ///
    /// Pass successive LCG outputs as `u` to simulate multiple draws.
    pub fn sample(&self, u: f64) -> usize {
        let mut cumulative = 0.0;
        for (i, &p) in self.pmf.iter().enumerate() {
            cumulative += p;
            if u < cumulative {
                return i;
            }
        }
        self.pmf.len().saturating_sub(1)
    }
    /// Computes the mean (E[X]).
    pub fn mean(&self) -> f64 {
        self.pmf
            .iter()
            .enumerate()
            .map(|(i, &p)| i as f64 * p)
            .sum()
    }
    /// Computes the variance (Var[X]).
    pub fn variance(&self) -> f64 {
        let mu = self.mean();
        self.pmf
            .iter()
            .enumerate()
            .map(|(i, &p)| p * (i as f64 - mu).powi(2))
            .sum()
    }
    /// Computes the Shannon entropy H = -Σ p log p (nats).
    pub fn shannon_entropy(&self) -> f64 {
        self.pmf
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.ln())
            .sum()
    }
}
/// Poisson process simulator and summary statistics.
///
/// A Poisson process N(t) with rate λ: inter-arrival times are Exp(λ).
#[allow(dead_code)]
pub struct PoissonProcess {
    /// Arrival rate λ > 0.
    pub lambda: f64,
}
#[allow(dead_code)]
impl PoissonProcess {
    /// Creates a `PoissonProcess` with rate λ.
    pub fn new(lambda: f64) -> Self {
        PoissonProcess { lambda }
    }
    /// Expected number of arrivals in [0, t]: E[N(t)] = λ t.
    pub fn expected_count(&self, t: f64) -> f64 {
        self.lambda * t
    }
    /// PMF of N(t): P(N(t) = k) = (λt)^k e^{-λt} / k!
    pub fn count_pmf(&self, t: f64, k: u32) -> f64 {
        poisson_pmf(self.lambda * t, k)
    }
    /// Variance of N(t): Var[N(t)] = λ t.
    pub fn variance_count(&self, t: f64) -> f64 {
        self.lambda * t
    }
    /// Generates inter-arrival times up to total time `t_max` using an LCG.
    /// Returns the vector of arrival times within [0, t_max].
    pub fn simulate_arrivals(&self, t_max: f64, lcg: &mut Lcg) -> Vec<f64> {
        let exp_dist = ExponentialDistribution::new(self.lambda);
        let mut arrivals = Vec::new();
        let mut current = 0.0;
        loop {
            let u = lcg.next_f64();
            let u = if u <= 0.0 { 1e-15 } else { u };
            let inter = exp_dist.sample(1.0 - u);
            current += inter;
            if current > t_max {
                break;
            }
            arrivals.push(current);
        }
        arrivals
    }
    /// Compound Poisson process: E[Σ Y_i] = λ t E[Y] for i.i.d. jumps Y.
    pub fn compound_expected(&self, t: f64, jump_mean: f64) -> f64 {
        self.lambda * t * jump_mean
    }
}
/// Beta distribution X ~ Beta(alpha, beta).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BetaRV {
    pub alpha: f64,
    pub beta: f64,
}
#[allow(dead_code)]
impl BetaRV {
    pub fn new(alpha: f64, beta: f64) -> Self {
        assert!(alpha > 0.0 && beta > 0.0, "alpha, beta must be positive");
        Self { alpha, beta }
    }
    /// Mean = alpha / (alpha + beta).
    pub fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }
    /// Variance = alpha*beta / ((alpha+beta)^2 * (alpha+beta+1)).
    pub fn variance(&self) -> f64 {
        let s = self.alpha + self.beta;
        self.alpha * self.beta / (s * s * (s + 1.0))
    }
    /// Mode = (alpha-1)/(alpha+beta-2) for alpha,beta > 1.
    pub fn mode(&self) -> Option<f64> {
        if self.alpha > 1.0 && self.beta > 1.0 {
            Some((self.alpha - 1.0) / (self.alpha + self.beta - 2.0))
        } else {
            None
        }
    }
}
/// Discrete-time, finite-state Markov chain with mixing time estimation.
pub struct MarkovChain {
    /// Number of states.
    pub states: usize,
    /// Row-stochastic transition matrix: `transition[i][j]` = P(i → j).
    pub transition: Vec<Vec<f64>>,
}
impl MarkovChain {
    /// Creates a new `MarkovChain` from a row-stochastic transition matrix.
    pub fn new(transition: Vec<Vec<f64>>) -> Self {
        let states = transition.len();
        MarkovChain { states, transition }
    }
    /// Computes the stationary distribution via power iteration.
    ///
    /// Returns a distribution π such that π P = π.
    pub fn stationary_distribution(&self) -> Vec<f64> {
        let n = self.states;
        if n == 0 {
            return vec![];
        }
        let mut dist = vec![1.0 / n as f64; n];
        for _ in 0..1000 {
            let mut next = vec![0.0f64; n];
            for j in 0..n {
                for i in 0..n {
                    next[j] += dist[i] * self.transition[i][j];
                }
            }
            let total: f64 = next.iter().sum();
            if total > 0.0 {
                for v in next.iter_mut() {
                    *v /= total;
                }
            }
            let diff: f64 = dist
                .iter()
                .zip(next.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();
            dist = next;
            if diff < 1e-10 {
                break;
            }
        }
        dist
    }
    /// Estimates the ε-mixing time: smallest t such that
    /// max_i d_TV(P^t(i, ·), π) ≤ ε.
    pub fn mixing_time(&self, epsilon: f64) -> usize {
        let n = self.states;
        if n == 0 {
            return 0;
        }
        let stationary = self.stationary_distribution();
        let mut current = vec![0.0f64; n];
        current[0] = 1.0;
        for t in 1..=10_000 {
            let mut next = vec![0.0f64; n];
            for j in 0..n {
                for i in 0..n {
                    next[j] += current[i] * self.transition[i][j];
                }
            }
            let tv: f64 = 0.5
                * next
                    .iter()
                    .zip(stationary.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum::<f64>();
            current = next;
            if tv <= epsilon {
                return t;
            }
        }
        10_000
    }
    /// Checks whether the chain is ergodic (all states communicate).
    pub fn is_ergodic(&self) -> bool {
        let n = self.states;
        if n == 0 {
            return true;
        }
        let forward = self.reachable_from(0);
        if forward.iter().any(|&r| !r) {
            return false;
        }
        for start in 0..n {
            let reach = self.reachable_from(start);
            if !reach[0] {
                return false;
            }
        }
        true
    }
    /// BFS reachability: which states are reachable from `start`?
    fn reachable_from(&self, start: usize) -> Vec<bool> {
        let n = self.states;
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        visited[start] = true;
        queue.push_back(start);
        while let Some(cur) = queue.pop_front() {
            for next in 0..n {
                if !visited[next] && self.transition[cur][next] > 0.0 {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        visited
    }
}
/// Running estimate of mean and variance using Welford's online algorithm.
///
/// Processes one sample at a time in O(1) time and O(1) space.
#[allow(dead_code)]
pub struct WelfordEstimator {
    count: u64,
    mean: f64,
    m2: f64,
}
#[allow(dead_code)]
impl WelfordEstimator {
    /// Creates a new empty `WelfordEstimator`.
    pub fn new() -> Self {
        WelfordEstimator {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
    /// Incorporates a new observation `x`.
    pub fn update(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }
    /// Returns the current sample count.
    pub fn count(&self) -> u64 {
        self.count
    }
    /// Returns the current mean estimate.
    pub fn mean(&self) -> f64 {
        self.mean
    }
    /// Returns the current unbiased variance estimate (n ≥ 2 required).
    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            return 0.0;
        }
        self.m2 / (self.count - 1) as f64
    }
    /// Returns the current standard deviation estimate.
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    /// Merges another `WelfordEstimator` into this one (parallel algorithm).
    pub fn merge(&mut self, other: &WelfordEstimator) {
        let combined = self.count + other.count;
        if combined == 0 {
            return;
        }
        let delta = other.mean - self.mean;
        self.m2 = self.m2
            + other.m2
            + delta * delta * self.count as f64 * other.count as f64 / combined as f64;
        self.mean =
            (self.mean * self.count as f64 + other.mean * other.count as f64) / combined as f64;
        self.count = combined;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussianProcessRegression {
    pub gp: GaussianProcess,
    pub noise_variance: f64,
    pub n_training: usize,
    pub prediction_method: String,
}
#[allow(dead_code)]
impl GaussianProcessRegression {
    pub fn new(gp: GaussianProcess, noise: f64) -> Self {
        GaussianProcessRegression {
            gp,
            noise_variance: noise,
            n_training: 0,
            prediction_method: "exact".to_string(),
        }
    }
    pub fn complexity_exact(&self) -> String {
        format!(
            "Exact GPR: O(n³) training, O(n²) per prediction (n={})",
            self.n_training
        )
    }
    pub fn sparse_gp_complexity(&self, m: usize) -> String {
        format!(
            "Sparse GPR: O(nm²) training, O(m²) per prediction (m={} inducing points)",
            m
        )
    }
    pub fn log_marginal_likelihood(&self) -> String {
        "log p(y|X) = -½ y^T(K+σ²I)^{-1}y - ½ log|K+σ²I| - n/2 log(2π)".to_string()
    }
}
/// Normal (Gaussian) distribution with mean μ and standard deviation σ.
pub struct GaussianDistribution {
    /// Mean.
    pub mu: f64,
    /// Standard deviation (must be > 0).
    pub sigma: f64,
}
impl GaussianDistribution {
    /// Creates a `GaussianDistribution`.
    pub fn new(mu: f64, sigma: f64) -> Self {
        GaussianDistribution { mu, sigma }
    }
    /// Probability density function f(x; μ, σ).
    pub fn pdf(&self, x: f64) -> f64 {
        normal_pdf(x, self.mu, self.sigma)
    }
    /// Approximates the CDF Φ((x-μ)/σ) using the Abramowitz & Stegun
    /// rational approximation (maximum error 7.5 × 10⁻⁸).
    pub fn cdf(&self, x: f64) -> f64 {
        let z = (x - self.mu) / self.sigma;
        standard_normal_cdf(z)
    }
    /// Draws a sample using the Box–Muller transform given two uniform inputs.
    pub fn sample_box_muller(&self, u1: f64, u2: f64) -> f64 {
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        self.mu + self.sigma * z
    }
    /// Returns the moment generating function M_X(t) = exp(μt + σ²t²/2).
    pub fn mgf(&self, t: f64) -> f64 {
        (self.mu * t + 0.5 * self.sigma * self.sigma * t * t).exp()
    }
    /// Returns the k-th central moment of the standard normal distribution (μ=0, σ=1).
    /// Even moments: (k-1)!! = 1·3·5·…·(k-1).  Odd moments: 0.
    pub fn standard_moment(k: u32) -> f64 {
        if k % 2 == 1 {
            return 0.0;
        }
        let mut result = 1.0f64;
        let mut i = 1u32;
        while i < k {
            result *= i as f64;
            i += 2;
        }
        result
    }
}
/// Empirical CDF (ECDF) from a finite sample.
///
/// F̂_n(x) = (1/n) |{i : X_i ≤ x}|.
#[allow(dead_code)]
pub struct EmpiricalCdf {
    /// Sorted sample values.
    sorted: Vec<f64>,
}
#[allow(dead_code)]
impl EmpiricalCdf {
    /// Creates an `EmpiricalCdf` from raw (unsorted) data.
    pub fn new(mut data: Vec<f64>) -> Self {
        data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        EmpiricalCdf { sorted: data }
    }
    /// Evaluates F̂_n(x) = P̂(X ≤ x).
    pub fn eval(&self, x: f64) -> f64 {
        let n = self.sorted.len();
        if n == 0 {
            return 0.0;
        }
        let count = self.sorted.partition_point(|&v| v <= x);
        count as f64 / n as f64
    }
    /// Returns the sample size.
    pub fn len(&self) -> usize {
        self.sorted.len()
    }
    /// Returns true if the sample is empty.
    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }
    /// Computes the Kolmogorov–Smirnov statistic D_n = sup_x |F̂_n(x) - F(x)|
    /// against a reference CDF given as a closure.
    pub fn ks_statistic(&self, reference_cdf: impl Fn(f64) -> f64) -> f64 {
        let n = self.sorted.len();
        if n == 0 {
            return 0.0;
        }
        let mut max_diff: f64 = 0.0;
        for (i, &x) in self.sorted.iter().enumerate() {
            let f_hat_minus = i as f64 / n as f64;
            let f_hat_plus = (i + 1) as f64 / n as f64;
            let f_ref = reference_cdf(x);
            let diff = (f_hat_minus - f_ref).abs().max((f_hat_plus - f_ref).abs());
            if diff > max_diff {
                max_diff = diff;
            }
        }
        max_diff
    }
    /// Returns the empirical quantile at level p ∈ [0,1].
    pub fn quantile(&self, p: f64) -> f64 {
        let n = self.sorted.len();
        if n == 0 {
            return f64::NAN;
        }
        let p = p.clamp(0.0, 1.0);
        let idx = ((p * n as f64).ceil() as usize)
            .saturating_sub(1)
            .min(n - 1);
        self.sorted[idx]
    }
}
/// Kalman filter state estimator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KalmanFilter {
    /// State estimate x_hat.
    pub x_hat: Vec<f64>,
    /// Error covariance P.
    pub p: Vec<Vec<f64>>,
    /// State transition matrix F.
    pub f: Vec<Vec<f64>>,
    /// Observation matrix H.
    pub h: Vec<Vec<f64>>,
    /// Process noise covariance Q.
    pub q: Vec<Vec<f64>>,
    /// Measurement noise covariance R.
    pub r: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl KalmanFilter {
    pub fn new_1d(f: f64, h: f64, q: f64, r_val: f64, x0: f64, p0: f64) -> Self {
        Self {
            x_hat: vec![x0],
            p: vec![vec![p0]],
            f: vec![vec![f]],
            h: vec![vec![h]],
            q: vec![vec![q]],
            r: vec![vec![r_val]],
        }
    }
    /// Predict step (1-D specialization).
    pub fn predict_1d(&mut self) {
        self.x_hat[0] = self.f[0][0] * self.x_hat[0];
        self.p[0][0] = self.f[0][0] * self.p[0][0] * self.f[0][0] + self.q[0][0];
    }
    /// Update step (1-D specialization).
    pub fn update_1d(&mut self, z: f64) {
        let h = self.h[0][0];
        let s = h * self.p[0][0] * h + self.r[0][0];
        let k = self.p[0][0] * h / s;
        let y = z - h * self.x_hat[0];
        self.x_hat[0] += k * y;
        self.p[0][0] = (1.0 - k * h) * self.p[0][0];
    }
    /// Filtered state estimate.
    pub fn estimate(&self) -> f64 {
        self.x_hat[0]
    }
    /// Current error variance.
    pub fn error_variance(&self) -> f64 {
        self.p[0][0]
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Copula {
    pub kind: CopulaKind,
}
#[allow(dead_code)]
impl Copula {
    pub fn gaussian(rho: f64) -> Self {
        assert!(rho.abs() < 1.0, "rho must be in (-1,1)");
        Self {
            kind: CopulaKind::Gaussian { rho },
        }
    }
    pub fn clayton(theta: f64) -> Self {
        assert!(theta > 0.0, "Clayton theta must be positive");
        Self {
            kind: CopulaKind::Clayton { theta },
        }
    }
    pub fn gumbel(theta: f64) -> Self {
        assert!(theta >= 1.0, "Gumbel theta must be >= 1");
        Self {
            kind: CopulaKind::Gumbel { theta },
        }
    }
    pub fn frank(theta: f64) -> Self {
        Self {
            kind: CopulaKind::Frank { theta },
        }
    }
    pub fn independence() -> Self {
        Self {
            kind: CopulaKind::Independence,
        }
    }
    /// Evaluate Clayton copula C(u,v) = max(u^(-theta)+v^(-theta)-1, 0)^(-1/theta).
    pub fn evaluate_clayton(&self, u: f64, v: f64) -> f64 {
        if let CopulaKind::Clayton { theta } = self.kind {
            let val = u.powf(-theta) + v.powf(-theta) - 1.0;
            if val <= 0.0 {
                return 0.0;
            }
            val.powf(-1.0 / theta)
        } else {
            u * v
        }
    }
    /// Evaluate Gumbel copula C(u,v) = exp(-[(-ln u)^theta+(-ln v)^theta]^(1/theta)).
    pub fn evaluate_gumbel(&self, u: f64, v: f64) -> f64 {
        if let CopulaKind::Gumbel { theta } = self.kind {
            let a = (-u.ln()).powf(theta);
            let b = (-v.ln()).powf(theta);
            (-(a + b).powf(1.0 / theta)).exp()
        } else {
            u * v
        }
    }
    /// Spearman's rho for Clayton copula: 3*theta/(theta+2) (approximation).
    pub fn spearman_rho_clayton_approx(&self) -> Option<f64> {
        if let CopulaKind::Clayton { theta } = self.kind {
            Some(3.0 * theta / (theta + 2.0))
        } else {
            None
        }
    }
}
/// Ergodic theorem variant.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ErgodicTheoremData {
    pub theorem_name: String,
    pub convergence_type: String,
    pub limit: String,
}
#[allow(dead_code)]
impl ErgodicTheoremData {
    /// Birkhoff's ergodic theorem (pointwise a.e.).
    pub fn birkhoff(measure_preserving: &str) -> Self {
        Self {
            theorem_name: "Birkhoff".to_string(),
            convergence_type: "a.e. and L1".to_string(),
            limit: format!(
                "E[f | Invariant sigma-algebra]({} system)",
                measure_preserving
            ),
        }
    }
    /// von Neumann mean ergodic theorem (L2 convergence).
    pub fn von_neumann() -> Self {
        Self {
            theorem_name: "von Neumann Mean Ergodic".to_string(),
            convergence_type: "L2".to_string(),
            limit: "projection onto invariant subspace".to_string(),
        }
    }
}
/// Characteristic function (Fourier transform of measure).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CharFunctionData {
    pub distribution: String,
    pub formula: String,
    pub is_integrable: bool,
}
#[allow(dead_code)]
impl CharFunctionData {
    /// Characteristic function of a Gaussian.
    pub fn gaussian(mean: f64, variance: f64) -> Self {
        Self {
            distribution: "Normal".to_string(),
            formula: format!("exp(i*{:.2}*t - {:.2}*t^2/2)", mean, variance),
            is_integrable: true,
        }
    }
    /// Characteristic function of Poisson.
    pub fn poisson(lambda: f64) -> Self {
        Self {
            distribution: "Poisson".to_string(),
            formula: format!("exp({:.2}*(e^{{it}} - 1))", lambda),
            is_integrable: false,
        }
    }
    /// Lévy-Cramér continuity theorem: convergence in distribution iff pointwise convergence of char. functions.
    pub fn levy_cramer_applies(&self) -> bool {
        true
    }
}
