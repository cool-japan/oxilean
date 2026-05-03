//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// Growth function Π_H(m) for a hypothesis class.
pub struct GrowthFunction {
    /// VC dimension of the hypothesis class.
    pub vc_dim: usize,
}
impl GrowthFunction {
    /// Create a new GrowthFunction.
    pub fn new(vc_dim: usize) -> Self {
        Self { vc_dim }
    }
    /// Evaluate the Sauer-Shelah upper bound for Π_H(m).
    pub fn evaluate(&self, m: usize) -> usize {
        VCDimension::new(self.vc_dim).sauer_shelah_bound(m)
    }
}
/// Cross-validation scheme.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CrossValidation {
    pub n_folds: usize,
    pub n_samples: usize,
    pub shuffle: bool,
    pub stratified: bool,
}
#[allow(dead_code)]
impl CrossValidation {
    pub fn new(k: usize, n: usize) -> Self {
        CrossValidation {
            n_folds: k,
            n_samples: n,
            shuffle: true,
            stratified: false,
        }
    }
    pub fn k_fold_5(n: usize) -> Self {
        CrossValidation::new(5, n)
    }
    pub fn loocv(n: usize) -> Self {
        CrossValidation::new(n, n)
    }
    pub fn fold_size(&self) -> usize {
        (self.n_samples + self.n_folds - 1) / self.n_folds
    }
    pub fn train_size(&self) -> usize {
        self.n_samples - self.fold_size()
    }
    pub fn n_train_test_splits(&self) -> usize {
        self.n_folds
    }
}
/// Early stopping regularization — implicit regularization by iteration count.
pub struct EarlyStoppingReg {
    /// Maximum number of gradient descent iterations.
    pub max_iters: usize,
    /// Step size.
    pub step_size: f64,
}
impl EarlyStoppingReg {
    /// Create a new EarlyStoppingReg.
    pub fn new(max_iters: usize, step_size: f64) -> Self {
        Self {
            max_iters,
            step_size,
        }
    }
    /// Effective regularization parameter ≈ 1/(step_size * max_iters).
    pub fn effective_lambda(&self) -> f64 {
        1.0 / (self.step_size * self.max_iters as f64)
    }
}
/// AdaBoost: adaptive boosting with exponential loss.
pub struct AdaBoost {
    /// Number of boosting rounds T.
    pub rounds: usize,
    /// Weights α_t for each weak learner.
    pub alphas: Vec<f64>,
    /// Per-round weak learner accuracies.
    pub weak_accuracies: Vec<f64>,
}
impl AdaBoost {
    /// Create a new AdaBoost instance.
    pub fn new(rounds: usize) -> Self {
        Self {
            rounds,
            alphas: Vec::new(),
            weak_accuracies: Vec::new(),
        }
    }
    /// Compute alpha_t = 0.5 * ln((1 - ε_t) / ε_t) for a weak learner with error ε_t.
    pub fn compute_alpha(weak_error: f64) -> f64 {
        0.5 * ((1.0 - weak_error) / weak_error).ln()
    }
    /// Training error bound after T rounds: ≤ exp(-2 Σ γ_t²) where γ_t = 0.5 - ε_t.
    pub fn training_error_bound(gammas: &[f64]) -> f64 {
        let sum_gamma_sq: f64 = gammas.iter().map(|g| g * g).sum();
        (-2.0 * sum_gamma_sq).exp()
    }
    /// Record a round's weak learner accuracy.
    pub fn add_round(&mut self, weak_accuracy: f64) {
        let weak_error = 1.0 - weak_accuracy;
        let alpha = Self::compute_alpha(weak_error);
        self.alphas.push(alpha);
        self.weak_accuracies.push(weak_accuracy);
    }
}
/// Online Gradient Descent with regret bound O(√T).
pub struct OnlineGradientDescent {
    /// Current parameter vector w_t.
    pub weights: Vec<f64>,
    /// Learning rate η.
    pub eta: f64,
    /// Constraint set radius D (‖w‖ ≤ D).
    pub d: f64,
    /// Gradient norm bound G (‖∇_t‖ ≤ G).
    pub g: f64,
    /// Round count.
    pub t: usize,
}
impl OnlineGradientDescent {
    /// Create a new OGD instance.
    pub fn new(dim: usize, eta: f64, d: f64, g: f64) -> Self {
        Self {
            weights: vec![0.0; dim],
            eta,
            d,
            g,
            t: 0,
        }
    }
    /// Update: w_{t+1} = project(w_t - η ∇_t) onto ‖w‖ ≤ D.
    pub fn update(&mut self, gradient: &[f64]) {
        for (wi, &gi) in self.weights.iter_mut().zip(gradient.iter()) {
            *wi -= self.eta * gi;
        }
        let norm: f64 = self.weights.iter().map(|wi| wi * wi).sum::<f64>().sqrt();
        if norm > self.d {
            let scale = self.d / norm;
            for wi in self.weights.iter_mut() {
                *wi *= scale;
            }
        }
        self.t += 1;
    }
    /// Regret bound after T rounds: R_T ≤ D * G * √(2T).
    pub fn regret_bound(&self) -> f64 {
        self.d * self.g * (2.0 * self.t as f64).sqrt()
    }
}
/// Support Vector Machine classifier.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SVMClassifier {
    pub kernel_type: SVMKernel,
    pub c_regularization: f64,
    pub n_support_vectors: usize,
}
#[allow(dead_code)]
impl SVMClassifier {
    pub fn new(kernel: SVMKernel, c: f64) -> Self {
        SVMClassifier {
            kernel_type: kernel,
            c_regularization: c,
            n_support_vectors: 0,
        }
    }
    pub fn linear(c: f64) -> Self {
        SVMClassifier::new(SVMKernel::Linear, c)
    }
    pub fn rbf(gamma: f64, c: f64) -> Self {
        SVMClassifier::new(SVMKernel::RBF(gamma), c)
    }
    pub fn kernel_value(&self, x: &[f64], xp: &[f64]) -> f64 {
        match &self.kernel_type {
            SVMKernel::Linear => dot(x, xp),
            SVMKernel::Polynomial(d) => (dot(x, xp) + 1.0).powi(*d as i32),
            SVMKernel::RBF(gamma) => {
                let sq_dist = x
                    .iter()
                    .zip(xp.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>();
                (-gamma * sq_dist).exp()
            }
            SVMKernel::Sigmoid => (dot(x, xp)).tanh(),
        }
    }
    pub fn sparsity_ratio(&self, n_training: usize) -> f64 {
        if n_training == 0 {
            return 0.0;
        }
        self.n_support_vectors as f64 / n_training as f64
    }
}
/// Fisher information I(θ) = E\[(∂/∂θ log p(x;θ))²\].
pub struct FisherInformation {
    /// Log-density function log p(x; θ) as a closure index (stored as parameter).
    pub theta: f64,
}
impl FisherInformation {
    /// Create a new FisherInformation at parameter θ.
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
    /// Numerical estimate of Fisher information via finite-difference score.
    ///
    /// Given samples x_i and log_density function, I(θ) ≈ (1/n) Σ (∂/∂θ log p(x_i;θ))².
    pub fn estimate(log_density: impl Fn(f64, f64) -> f64, theta: f64, samples: &[f64]) -> f64 {
        let h = 1e-5;
        let n = samples.len() as f64;
        samples
            .iter()
            .map(|&x| {
                let score = (log_density(x, theta + h) - log_density(x, theta - h)) / (2.0 * h);
                score * score
            })
            .sum::<f64>()
            / n
    }
    /// Cramér-Rao lower bound: Var(θ̂) ≥ 1/I(θ).
    pub fn cramer_rao_bound(&self, fisher_val: f64) -> f64 {
        if fisher_val > 0.0 {
            1.0 / fisher_val
        } else {
            f64::INFINITY
        }
    }
}
/// Rademacher complexity estimate for a finite hypothesis class.
///
/// For a finite class H of size |H| over n samples:
/// R_n(H) ≤ √(2 ln|H| / n).
pub struct RademacherComplexity {
    /// Number of samples n.
    pub n: usize,
    /// Number of hypotheses in the class |H|.
    pub class_size: usize,
}
impl RademacherComplexity {
    /// Create a new RademacherComplexity bound.
    pub fn new(n: usize, class_size: usize) -> Self {
        Self { n, class_size }
    }
    /// Upper bound: √(2 ln|H| / n).
    pub fn upper_bound(&self) -> f64 {
        (2.0 * (self.class_size as f64).ln() / self.n as f64).sqrt()
    }
    /// Generalization bound: L_D(h) ≤ L_S(h) + 2 R_n(H) + √(log(2/δ)/(2n)).
    pub fn generalization_bound(&self, empirical_loss: f64, delta: f64) -> f64 {
        let rn = self.upper_bound();
        let confidence_term = ((2.0 / delta).ln() / (2.0 * self.n as f64)).sqrt();
        empirical_loss + 2.0 * rn + confidence_term
    }
}
/// Uniform convergence checker.
pub struct UniformConvergence {
    /// ε: uniform convergence tolerance.
    pub eps: f64,
    /// δ: failure probability.
    pub delta: f64,
}
impl UniformConvergence {
    /// Create a new UniformConvergence instance.
    pub fn new(eps: f64, delta: f64) -> Self {
        Self { eps, delta }
    }
    /// Required samples for ε-uniform convergence for a class of size |H|.
    pub fn required_samples(&self, class_size: usize) -> usize {
        let log_h = (class_size as f64).ln();
        let log_delta = (1.0 / self.delta).ln();
        ((2.0 * (log_h + log_delta)) / (self.eps * self.eps)).ceil() as usize
    }
}
/// Bias-variance tradeoff decomposition: MSE = Bias² + Variance + Noise.
pub struct BiasVarianceTradeoff {
    /// Squared bias of the estimator.
    pub bias_squared: f64,
    /// Variance of the estimator.
    pub variance: f64,
    /// Irreducible noise level σ².
    pub noise: f64,
}
impl BiasVarianceTradeoff {
    /// Create a new BiasVarianceTradeoff.
    pub fn new(bias_squared: f64, variance: f64, noise: f64) -> Self {
        Self {
            bias_squared,
            variance,
            noise,
        }
    }
    /// Total expected MSE = Bias² + Var + σ².
    pub fn total_mse(&self) -> f64 {
        self.bias_squared + self.variance + self.noise
    }
}
/// Mutual information I(X;Y) = H(X) + H(Y) - H(X,Y).
pub struct MutualInformation;
impl MutualInformation {
    /// Compute I(X;Y) from a joint distribution table.
    ///
    /// `joint\[i\]\[j\]` = P(X=i, Y=j).
    pub fn compute(joint: &[Vec<f64>]) -> f64 {
        if joint.is_empty() {
            return 0.0;
        }
        let _n_rows = joint.len();
        let n_cols = joint[0].len();
        let px: Vec<f64> = joint.iter().map(|row| row.iter().sum::<f64>()).collect();
        let py: Vec<f64> = (0..n_cols)
            .map(|j| {
                joint
                    .iter()
                    .map(|row| row.get(j).copied().unwrap_or(0.0))
                    .sum()
            })
            .collect();
        let h_x: f64 = px.iter().filter(|&&p| p > 0.0).map(|&p| -p * p.ln()).sum();
        let h_y: f64 = py.iter().filter(|&&p| p > 0.0).map(|&p| -p * p.ln()).sum();
        let h_xy: f64 = joint
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.ln())
            .sum();
        (h_x + h_y - h_xy).max(0.0)
    }
    /// Data processing inequality: I(X;Z) ≤ I(X;Y) for Markov chain X → Y → Z.
    ///
    /// Verifies that for the given joint tables, I(X;Z) ≤ I(X;Y).
    pub fn data_processing_inequality(joint_xy: &[Vec<f64>], joint_xz: &[Vec<f64>]) -> bool {
        let i_xy = Self::compute(joint_xy);
        let i_xz = Self::compute(joint_xz);
        i_xz <= i_xy + 1e-9
    }
}
/// Ensemble method: gradient boosting.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GradientBoosting {
    pub n_estimators: usize,
    pub learning_rate: f64,
    pub max_depth: usize,
    pub loss: GBLoss,
}
#[allow(dead_code)]
impl GradientBoosting {
    pub fn new(n: usize, lr: f64, depth: usize, loss: GBLoss) -> Self {
        GradientBoosting {
            n_estimators: n,
            learning_rate: lr,
            max_depth: depth,
            loss,
        }
    }
    pub fn xgboost_style(n: usize) -> Self {
        GradientBoosting::new(n, 0.1, 6, GBLoss::MSE)
    }
    pub fn effective_shrinkage(&self) -> f64 {
        self.learning_rate
    }
    pub fn n_leaves_upper_bound(&self) -> usize {
        2usize.pow(self.max_depth as u32)
    }
    pub fn is_regularized(&self) -> bool {
        self.learning_rate < 1.0
    }
}
/// Kernel (Gram) matrix K_{ij} = k(x_i, x_j).
pub struct KernelMatrix {
    /// The matrix entries.
    pub entries: Vec<Vec<f64>>,
    /// Number of data points n.
    pub n: usize,
}
impl KernelMatrix {
    /// Compute the kernel matrix for a dataset and kernel function.
    pub fn compute(kernel: &KernelFunction, data: &[Vec<f64>]) -> Self {
        let n = data.len();
        let entries: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| kernel.evaluate(&data[i], &data[j]))
                    .collect()
            })
            .collect();
        Self { entries, n }
    }
    /// Trace of the kernel matrix (sum of diagonal entries).
    pub fn trace(&self) -> f64 {
        (0..self.n).map(|i| self.entries[i][i]).sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SVMKernel {
    Linear,
    Polynomial(u32),
    RBF(f64),
    Sigmoid,
}
/// KL divergence D_KL(P‖Q) = Σ P(x) log(P(x)/Q(x)).
pub struct KLDivergence;
impl KLDivergence {
    /// Compute D_KL(p ‖ q) in nats.  Returns ∞ if q(x) = 0 when p(x) > 0.
    pub fn compute(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .filter(|(&pi, _)| pi > 0.0)
            .map(|(&pi, &qi)| {
                if qi == 0.0 {
                    f64::INFINITY
                } else {
                    pi * (pi / qi).ln()
                }
            })
            .sum()
    }
    /// Non-negativity check: D_KL(p‖q) ≥ 0.
    pub fn is_nonneg(p: &[f64], q: &[f64]) -> bool {
        Self::compute(p, q) >= -1e-9
    }
}
/// VC dimension and growth function calculations.
pub struct VCDimension {
    /// The claimed VC dimension.
    pub dim: usize,
}
impl VCDimension {
    /// Create a new VCDimension.
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
    /// Sauer-Shelah bound: Π_H(m) ≤ Σ_{i=0}^{d} C(m,i).
    pub fn sauer_shelah_bound(&self, m: usize) -> usize {
        let d = self.dim;
        let mut bound = 0usize;
        let mut binom = 1usize;
        for i in 0..=d.min(m) {
            if i > 0 {
                binom = binom
                    .saturating_mul(m - i + 1)
                    .checked_div(i)
                    .unwrap_or(binom);
            }
            bound = bound.saturating_add(binom);
        }
        bound
    }
    /// Check if d-dimensional threshold classifier can shatter m points.
    ///
    /// For the canonical 1D threshold classifier H = {h_θ : θ ∈ ℝ}, VC dim = 1.
    /// This checks whether the bound is consistent with shattering.
    pub fn can_shatter(&self, m: usize) -> bool {
        m <= self.dim
    }
    /// Fundamental theorem of PAC learning: finite VC dim ↔ PAC learnability.
    pub fn fundamental_theorem_pac(&self) -> bool {
        self.dim < usize::MAX
    }
}
/// Available kernel types.
pub enum KernelType {
    /// Linear kernel: k(x,y) = x·y.
    Linear,
    /// Polynomial kernel: k(x,y) = (x·y + c)^d.
    Polynomial { degree: u32, offset: f64 },
    /// RBF/Gaussian kernel: k(x,y) = exp(-‖x-y‖²/(2σ²)).
    Rbf { sigma: f64 },
    /// Laplace kernel: k(x,y) = exp(-‖x-y‖/σ).
    Laplace { sigma: f64 },
}
/// Regret bound summary.
pub struct RegretBound {
    /// Number of rounds T.
    pub t: usize,
    /// Domain diameter D.
    pub d: f64,
    /// Gradient norm bound G.
    pub g: f64,
}
impl RegretBound {
    /// Create a new RegretBound.
    pub fn new(t: usize, d: f64, g: f64) -> Self {
        Self { t, d, g }
    }
    /// OGD regret bound: D * G * √(2T).
    pub fn ogd_bound(&self) -> f64 {
        self.d * self.g * (2.0 * self.t as f64).sqrt()
    }
}
/// Sample complexity for PAC learning (Blumer et al. / Vapnik-Chervonenkis bound).
///
/// Returns m = ceil((d * ln(d/eps + 1) + ln(2/delta)) / eps) samples.
/// Here `vc_dim` is the VC dimension d of the hypothesis class.
pub struct SampleComplexity {
    /// Accuracy parameter ε ∈ (0,1).
    pub eps: f64,
    /// Confidence parameter δ ∈ (0,1).
    pub delta: f64,
    /// VC dimension d of the hypothesis class.
    pub vc_dim: usize,
}
impl SampleComplexity {
    /// Create a new SampleComplexity instance.
    pub fn new(eps: f64, delta: f64, vc_dim: usize) -> Self {
        Self { eps, delta, vc_dim }
    }
    /// Compute the sample complexity upper bound.
    pub fn compute(&self) -> usize {
        let d = self.vc_dim as f64;
        let numerator = d * (d / self.eps + 1.0).ln() + (2.0 / self.delta).ln();
        (numerator / self.eps).ceil() as usize
    }
}
/// Kernel SVM trainer using a simplified SMO algorithm.
///
/// Implements the Sequential Minimal Optimization (SMO) core update step.
pub struct KernelSVMTrainer {
    /// Number of training points.
    pub n: usize,
    /// Dual variables α_i ∈ \[0, C\].
    pub alphas: Vec<f64>,
    /// Labels y_i ∈ {-1, +1}.
    pub labels: Vec<f64>,
    /// Bias term b.
    pub bias: f64,
    /// Regularization parameter C (upper bound on α_i).
    pub c: f64,
}
impl KernelSVMTrainer {
    /// Create a new KernelSVMTrainer with zero alphas.
    pub fn new(n: usize, labels: Vec<f64>, c: f64) -> Self {
        Self {
            n,
            alphas: vec![0.0; n],
            labels,
            bias: 0.0,
            c,
        }
    }
    /// Compute the SVM decision function f(x) = Σ α_i y_i k(x_i, x) + b.
    pub fn decision(&self, kernel_row: &[f64]) -> f64 {
        self.alphas
            .iter()
            .zip(self.labels.iter())
            .zip(kernel_row.iter())
            .map(|((a, y), k)| a * y * k)
            .sum::<f64>()
            + self.bias
    }
    /// One SMO update step on a pair (i, j) given kernel matrix K.
    ///
    /// Returns true if a meaningful update was made.
    pub fn smo_step(&mut self, i: usize, j: usize, k: &[Vec<f64>]) -> bool {
        if i == j {
            return false;
        }
        let yi = self.labels[i];
        let yj = self.labels[j];
        let fi = self.decision(&k[i]);
        let fj = self.decision(&k[j]);
        let ei = fi - yi;
        let ej = fj - yj;
        let eta = k[i][i] + k[j][j] - 2.0 * k[i][j];
        if eta <= 0.0 {
            return false;
        }
        let alpha_j_old = self.alphas[j];
        let alpha_i_old = self.alphas[i];
        let (l, h) = if (yi - yj).abs() < 1e-9 {
            let sum = alpha_i_old + alpha_j_old;
            ((sum - self.c).max(0.0), sum.min(self.c))
        } else {
            let diff = alpha_j_old - alpha_i_old;
            ((-diff).max(0.0), (self.c - diff).min(self.c))
        };
        let alpha_j_new = (alpha_j_old + yj * (ei - ej) / eta).clamp(l, h);
        if (alpha_j_new - alpha_j_old).abs() < 1e-5 {
            return false;
        }
        let alpha_i_new = alpha_i_old + yi * yj * (alpha_j_old - alpha_j_new);
        let b1 = self.bias
            - ei
            - yi * (alpha_i_new - alpha_i_old) * k[i][i]
            - yj * (alpha_j_new - alpha_j_old) * k[i][j];
        let b2 = self.bias
            - ej
            - yi * (alpha_i_new - alpha_i_old) * k[i][j]
            - yj * (alpha_j_new - alpha_j_old) * k[j][j];
        self.bias = (b1 + b2) / 2.0;
        self.alphas[i] = alpha_i_new;
        self.alphas[j] = alpha_j_new;
        true
    }
    /// SVM generalization bound: ≤ R²/(γ² n) with margin γ.
    pub fn generalization_bound(radius: f64, margin: f64, n: usize) -> f64 {
        (radius * radius) / (margin * margin * n as f64)
    }
}
/// PAC learner: wraps accuracy/confidence parameters.
pub struct PACLearner {
    /// ε: maximum tolerated generalization error.
    pub eps: f64,
    /// δ: failure probability (confidence 1−δ).
    pub delta: f64,
}
impl PACLearner {
    /// Create a new PAC learner.
    pub fn new(eps: f64, delta: f64) -> Self {
        Self { eps, delta }
    }
    /// Required sample size for a hypothesis class of VC dimension d.
    pub fn required_samples(&self, vc_dim: usize) -> usize {
        SampleComplexity::new(self.eps, self.delta, vc_dim).compute()
    }
}
/// Evidence Lower Bound (ELBO) for variational inference.
///
/// ℒ(q) = E_q\[log p(x,z)\] - E_q\[log q(z)\] = log p(x) - D_KL(q(z) ‖ p(z|x))
pub struct ELBO {
    /// D_KL(q‖p) component.
    pub kl_term: f64,
    /// E_q\[log p(x,z)\] reconstruction term.
    pub reconstruction_term: f64,
}
impl ELBO {
    /// Create a new ELBO from its components.
    pub fn new(reconstruction_term: f64, kl_term: f64) -> Self {
        Self {
            kl_term,
            reconstruction_term,
        }
    }
    /// Compute ℒ(q) = reconstruction_term - kl_term.
    pub fn value(&self) -> f64 {
        self.reconstruction_term - self.kl_term
    }
    /// Compute ELBO from discrete distributions q(z) and joint p(x,z).
    ///
    /// ℒ = Σ_z q(z) log(p(x,z)/q(z))
    pub fn compute(q: &[f64], p_joint: &[f64]) -> f64 {
        q.iter()
            .zip(p_joint.iter())
            .filter(|(&qi, _)| qi > 0.0)
            .map(|(&qi, &pi)| {
                if pi == 0.0 {
                    f64::NEG_INFINITY
                } else {
                    qi * (pi / qi).ln()
                }
            })
            .sum()
    }
}
/// Tikhonov (ridge) regularization: min_h L(h) + λ‖h‖².
pub struct TikhonovReg {
    /// Regularization parameter λ.
    pub lambda: f64,
}
impl TikhonovReg {
    /// Create a new TikhonovReg.
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }
    /// Ridge regression closed-form solution: w = (X^T X + λI)^{-1} X^T y.
    /// Here X is n×d (row major), y is length-n.  Returns weight vector w.
    pub fn solve(&self, x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
        let d = if x.is_empty() { 0 } else { x[0].len() };
        let n = x.len();
        let mut xtx = vec![vec![0.0f64; d]; d];
        for i in 0..d {
            xtx[i][i] = self.lambda;
        }
        for row in x {
            for i in 0..d {
                for j in 0..d {
                    xtx[i][j] += row[i] * row[j];
                }
            }
        }
        let mut xty = vec![0.0f64; d];
        for (row, &yi) in x.iter().zip(y.iter()) {
            for j in 0..d {
                xty[j] += row[j] * yi;
            }
        }
        gauss_solve(&xtx, &xty, d, n)
    }
    /// Regularization penalty: λ‖w‖².
    pub fn penalty(&self, w: &[f64]) -> f64 {
        self.lambda * w.iter().map(|wi| wi * wi).sum::<f64>()
    }
}
/// Backdoor adjustment formula for causal inference.
///
/// Computes P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) * P(Z=z)
/// given a set of confounder strata z.
pub struct CausalBackdoor {
    /// Conditional probabilities P(Y=1 | X=x, Z=z) for each stratum z.
    pub cond_probs: Vec<f64>,
    /// Marginal probabilities P(Z=z) for each stratum z.
    pub stratum_probs: Vec<f64>,
}
impl CausalBackdoor {
    /// Create a new CausalBackdoor instance.
    pub fn new(cond_probs: Vec<f64>, stratum_probs: Vec<f64>) -> Self {
        Self {
            cond_probs,
            stratum_probs,
        }
    }
    /// Compute P(Y=1 | do(X=x)) via backdoor adjustment.
    ///
    /// Returns Σ_z P(Y=1 | X=x, Z=z) * P(Z=z).
    pub fn adjust(&self) -> f64 {
        self.cond_probs
            .iter()
            .zip(self.stratum_probs.iter())
            .map(|(p, q)| p * q)
            .sum()
    }
    /// Compute the confounding bias: |observational P(Y|X=x) - interventional P(Y|do(X=x))|.
    pub fn confounding_bias(&self, observational: f64) -> f64 {
        (observational - self.adjust()).abs()
    }
    /// Verify the backdoor adjustment probabilities sum to ≤ 1 (sanity check).
    pub fn is_valid(&self) -> bool {
        let sum: f64 = self.stratum_probs.iter().sum();
        (sum - 1.0).abs() < 1e-6
    }
}
/// Double Rademacher (two-sided) bound.
pub struct DoubleRademacher {
    /// Rademacher complexity instance.
    pub rademacher: RademacherComplexity,
}
impl DoubleRademacher {
    /// Create a new DoubleRademacher instance.
    pub fn new(n: usize, class_size: usize) -> Self {
        Self {
            rademacher: RademacherComplexity::new(n, class_size),
        }
    }
    /// Two-sided bound: |L_D(h) - L_S(h)| ≤ 2 R_n(H) w.h.p.
    pub fn two_sided_bound(&self) -> f64 {
        2.0 * self.rademacher.upper_bound()
    }
}
/// Online Perceptron classifier.
pub struct Perceptron {
    /// Weight vector w ∈ ℝ^d.
    pub weights: Vec<f64>,
    /// Bias term b.
    pub bias: f64,
    /// Number of mistakes made so far.
    pub mistakes: usize,
}
impl Perceptron {
    /// Create a new zero-initialized Perceptron.
    pub fn new(dim: usize) -> Self {
        Self {
            weights: vec![0.0; dim],
            bias: 0.0,
            mistakes: 0,
        }
    }
    /// Predict the label for input x: sign(w·x + b).
    pub fn predict(&self, x: &[f64]) -> f64 {
        let score = dot(&self.weights, x) + self.bias;
        if score >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }
    /// Online update: if prediction wrong, w ← w + y·x, b ← b + y.
    pub fn update(&mut self, x: &[f64], label: f64) -> bool {
        let pred = self.predict(x);
        if (pred * label) <= 0.0 {
            for (wi, &xi) in self.weights.iter_mut().zip(x.iter()) {
                *wi += label * xi;
            }
            self.bias += label;
            self.mistakes += 1;
            true
        } else {
            false
        }
    }
    /// Perceptron mistake bound: M ≤ (R/γ)² where R = radius, γ = margin.
    pub fn mistake_bound(radius: f64, margin: f64) -> usize {
        ((radius / margin).powi(2)).ceil() as usize
    }
}
/// Gaussian complexity (Gaussian analog of Rademacher).
pub struct GaussianComplexity {
    /// Number of samples n.
    pub n: usize,
    /// Number of hypotheses in the class.
    pub class_size: usize,
}
impl GaussianComplexity {
    /// Create a new GaussianComplexity instance.
    pub fn new(n: usize, class_size: usize) -> Self {
        Self { n, class_size }
    }
    /// Upper bound for Gaussian complexity: √(2 ln|H| / n) (same as Rademacher by comparison).
    pub fn upper_bound(&self) -> f64 {
        (2.0 * (self.class_size as f64).ln() / self.n as f64).sqrt()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GBLoss {
    MSE,
    MAE,
    LogLoss,
    Huber(f64),
}
/// Lasso (ℓ₁) regularization: min_h L(h) + λ‖h‖₁.
pub struct LassoReg {
    /// Regularization parameter λ.
    pub lambda: f64,
}
impl LassoReg {
    /// Create a new LassoReg.
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }
    /// Soft-thresholding operator: sign(w) * max(|w| - λ, 0) per coordinate.
    pub fn soft_threshold(&self, w: &[f64]) -> Vec<f64> {
        w.iter()
            .map(|&wi| {
                let abs_wi = wi.abs();
                if abs_wi <= self.lambda {
                    0.0
                } else {
                    wi.signum() * (abs_wi - self.lambda)
                }
            })
            .collect()
    }
    /// Regularization penalty: λ‖w‖₁.
    pub fn penalty(&self, w: &[f64]) -> f64 {
        self.lambda * w.iter().map(|wi| wi.abs()).sum::<f64>()
    }
}
/// UCB1 (Upper Confidence Bound) algorithm for multi-armed bandits.
///
/// Achieves cumulative regret O(√(n T ln T)) where n = number of arms.
pub struct UCBBandit {
    /// Number of arms.
    pub n: usize,
    /// Number of times each arm has been pulled.
    pub counts: Vec<usize>,
    /// Empirical mean reward for each arm.
    pub means: Vec<f64>,
    /// Total rounds elapsed.
    pub t: usize,
}
impl UCBBandit {
    /// Create a new UCB1 bandit instance.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            counts: vec![0; n],
            means: vec![0.0; n],
            t: 0,
        }
    }
    /// Select the arm with the highest UCB index.
    ///
    /// UCB index for arm i: μ_i + √(2 ln t / n_i).
    /// Arms with count 0 are always selected first (infinite UCB).
    pub fn select(&self) -> usize {
        if let Some(i) = self.counts.iter().position(|&c| c == 0) {
            return i;
        }
        let ln_t = (self.t as f64).ln();
        (0..self.n)
            .max_by(|&i, &j| {
                let ucb_i = self.means[i] + (2.0 * ln_t / self.counts[i] as f64).sqrt();
                let ucb_j = self.means[j] + (2.0 * ln_t / self.counts[j] as f64).sqrt();
                ucb_i
                    .partial_cmp(&ucb_j)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0)
    }
    /// Update the chosen arm with observed reward.
    pub fn update(&mut self, arm: usize, reward: f64) {
        self.counts[arm] += 1;
        let n = self.counts[arm] as f64;
        self.means[arm] += (reward - self.means[arm]) / n;
        self.t += 1;
    }
    /// UCB1 regret bound: O(√(n T ln T)).
    pub fn regret_bound_upper(&self) -> f64 {
        let t = self.t as f64;
        let n = self.n as f64;
        (n * t * t.ln()).sqrt()
    }
}
/// PAC-Bayes generalization bound computation.
///
/// McAllester's bound: L_D(Q) ≤ L_S(Q) + √((KL(Q‖P) + ln(n/δ)) / (2n)).
pub struct PACBayesGeneralization {
    /// KL divergence KL(Q‖P) in nats.
    pub kl_qp: f64,
    /// Number of training samples n.
    pub n: usize,
    /// Confidence parameter δ.
    pub delta: f64,
}
impl PACBayesGeneralization {
    /// Create a new PAC-Bayes bound instance.
    pub fn new(kl_qp: f64, n: usize, delta: f64) -> Self {
        Self { kl_qp, n, delta }
    }
    /// McAllester bound: empirical_loss + √((KL + ln(n/δ)) / (2n)).
    pub fn mcallester_bound(&self, empirical_loss: f64) -> f64 {
        let penalty =
            ((self.kl_qp + (self.n as f64 / self.delta).ln()) / (2.0 * self.n as f64)).sqrt();
        empirical_loss + penalty
    }
    /// Catoni's tighter bound (using solve for λ-parameterized form).
    /// Approximation: L_D(Q) ≤ (1/(1-λ/2)) * (L_S(Q) + KL/(2λn)).
    pub fn catoni_bound(&self, empirical_loss: f64, lambda: f64) -> f64 {
        let scale = 1.0 / (1.0 - lambda / 2.0);
        let penalty = self.kl_qp / (2.0 * lambda * self.n as f64);
        scale * (empirical_loss + penalty)
    }
    /// Optimal λ for Catoni bound (minimizing RHS).
    pub fn optimal_lambda(&self, empirical_loss: f64) -> f64 {
        let ratio = self.n as f64 * empirical_loss / (self.kl_qp.max(1e-9));
        1.0 / (1.0 + ratio).sqrt()
    }
}
/// Exponential Weights Algorithm (Hedge / EWA) for online learning.
///
/// Maintains a distribution over n experts and uses multiplicative updates.
/// Guarantees regret R_T ≤ ln(n)/η + η T/2.
pub struct ExponentialWeightsAlgorithm {
    /// Number of experts.
    pub n: usize,
    /// Learning rate η.
    pub eta: f64,
    /// Current weights (unnormalized).
    pub weights: Vec<f64>,
    /// Total rounds elapsed.
    pub rounds: usize,
}
impl ExponentialWeightsAlgorithm {
    /// Create a new EWA with uniform initial weights.
    pub fn new(n: usize, eta: f64) -> Self {
        Self {
            n,
            eta,
            weights: vec![1.0; n],
            rounds: 0,
        }
    }
    /// Return the current probability distribution over experts.
    pub fn distribution(&self) -> Vec<f64> {
        let total: f64 = self.weights.iter().sum();
        self.weights.iter().map(|w| w / total).collect()
    }
    /// Multiplicative update: w_i ← w_i * exp(-η * loss_i).
    pub fn update(&mut self, losses: &[f64]) {
        for (w, &l) in self.weights.iter_mut().zip(losses.iter()) {
            *w *= (-self.eta * l).exp();
        }
        self.rounds += 1;
    }
    /// EWA regret bound: R_T ≤ ln(n)/η + η * T / 2.
    pub fn regret_bound(&self) -> f64 {
        let t = self.rounds as f64;
        (self.n as f64).ln() / self.eta + self.eta * t / 2.0
    }
    /// Optimal learning rate for T rounds: η* = √(2 ln n / T).
    pub fn optimal_eta(n: usize, t: usize) -> f64 {
        (2.0 * (n as f64).ln() / t as f64).sqrt()
    }
}
/// Feature map: maps inputs to a (truncated) explicit feature space.
pub struct FeatureMap {
    /// Dimensionality of the feature space.
    pub feature_dim: usize,
}
impl FeatureMap {
    /// Create a new feature map.
    pub fn new(feature_dim: usize) -> Self {
        Self { feature_dim }
    }
    /// Compute the inner product ⟨φ(x), φ(y)⟩ in feature space.
    /// For the linear kernel, φ(x) = x, so this is just the dot product.
    pub fn inner_product(&self, x: &[f64], y: &[f64]) -> f64 {
        dot(x, y)
    }
}
/// Kernel SVM dual representation.
pub struct KernelSVM {
    /// Dual variables α_i.
    pub alphas: Vec<f64>,
    /// Labels y_i ∈ {-1, +1}.
    pub labels: Vec<f64>,
    /// Bias term b.
    pub bias: f64,
    /// Regularization parameter C.
    pub c: f64,
}
impl KernelSVM {
    /// Create a new kernel SVM (initialized to zero weights).
    pub fn new(n: usize, c: f64) -> Self {
        Self {
            alphas: vec![0.0; n],
            labels: vec![1.0; n],
            bias: 0.0,
            c,
        }
    }
    /// Decision function: f(x) = Σ α_i y_i k(x_i, x) + b.
    pub fn predict(&self, kernel_vals: &[f64]) -> f64 {
        let sum: f64 = self
            .alphas
            .iter()
            .zip(self.labels.iter())
            .zip(kernel_vals.iter())
            .map(|((a, y), k)| a * y * k)
            .sum();
        sum + self.bias
    }
}
/// Gaussian process regression model.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussianProcess {
    pub mean: f64,
    pub length_scale: f64,
    pub signal_variance: f64,
    pub noise_variance: f64,
}
#[allow(dead_code)]
impl GaussianProcess {
    pub fn new(mean: f64, length_scale: f64, signal_var: f64, noise_var: f64) -> Self {
        GaussianProcess {
            mean,
            length_scale,
            signal_variance: signal_var,
            noise_variance: noise_var,
        }
    }
    pub fn default_rbf() -> Self {
        GaussianProcess::new(0.0, 1.0, 1.0, 0.01)
    }
    /// RBF (squared exponential) kernel: k(x, x') = σ^2 exp(-|x-x'|^2 / (2l^2)).
    pub fn rbf_kernel(&self, x: f64, xp: f64) -> f64 {
        let d = x - xp;
        self.signal_variance * (-d * d / (2.0 * self.length_scale.powi(2))).exp()
    }
    /// Predictive variance at a new point (simplified: just signal variance).
    pub fn predictive_variance(&self, x: f64, train_x: &[f64]) -> f64 {
        let k_star_star = self.rbf_kernel(x, x);
        let k_noise: Vec<f64> = train_x.iter().map(|&xi| self.rbf_kernel(x, xi)).collect();
        let contrib: f64 = k_noise.iter().map(|&k| k * k).sum::<f64>()
            / (self.signal_variance + self.noise_variance).max(1e-10);
        (k_star_star - contrib).max(self.noise_variance)
    }
    pub fn log_marginal_likelihood_approx(&self, n: usize) -> f64 {
        -(n as f64) / 2.0 * (2.0 * std::f64::consts::PI * self.signal_variance).ln()
    }
}
/// A kernel function k: ℝ^d × ℝ^d → ℝ.
pub struct KernelFunction {
    /// Kernel type identifier.
    pub kernel_type: KernelType,
}
impl KernelFunction {
    /// Create a new kernel function.
    pub fn new(kernel_type: KernelType) -> Self {
        Self { kernel_type }
    }
    /// Evaluate the kernel k(x, y) where x, y are vectors in ℝ^d.
    pub fn evaluate(&self, x: &[f64], y: &[f64]) -> f64 {
        match &self.kernel_type {
            KernelType::Linear => dot(x, y),
            KernelType::Polynomial { degree, offset } => (dot(x, y) + offset).powi(*degree as i32),
            KernelType::Rbf { sigma } => {
                let diff_sq: f64 = x.iter().zip(y).map(|(a, b)| (a - b).powi(2)).sum();
                (-diff_sq / (2.0 * sigma * sigma)).exp()
            }
            KernelType::Laplace { sigma } => {
                let diff_norm: f64 = x.iter().zip(y).map(|(a, b)| (a - b).abs()).sum();
                (-diff_norm / sigma).exp()
            }
        }
    }
    /// Check if the kernel matrix for a set of points is positive semi-definite
    /// (via Cholesky: all pivots ≥ -ε for numerical tolerance).
    pub fn is_positive_definite(&self, points: &[Vec<f64>]) -> bool {
        let n = points.len();
        let mut k: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| self.evaluate(&points[i], &points[j]))
                    .collect()
            })
            .collect();
        for i in 0..n {
            for j in 0..i {
                let mut sum = k[i][j];
                for l in 0..j {
                    sum -= k[i][l] * k[j][l];
                }
                k[i][j] = sum / k[j][j];
            }
            let mut diag = k[i][i];
            for l in 0..i {
                diag -= k[i][l] * k[i][l];
            }
            if diag < -1e-9 {
                return false;
            }
            k[i][i] = diag.max(0.0).sqrt();
        }
        true
    }
}
