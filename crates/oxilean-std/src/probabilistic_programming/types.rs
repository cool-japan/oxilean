//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::*;

/// Gaussian Process regression for probabilistic numerics.
///
/// Maintains a posterior GP over a function given noisy observations,
/// supporting inference and quadrature.
#[allow(dead_code)]
pub struct GaussianProcessRegressor {
    /// Observed inputs.
    pub x_train: Vec<f64>,
    /// Observed (noisy) outputs.
    pub y_train: Vec<f64>,
    /// Length scale of the RBF kernel.
    pub length_scale: f64,
    /// Signal variance.
    pub signal_var: f64,
    /// Noise variance.
    pub noise_var: f64,
}
#[allow(dead_code)]
impl GaussianProcessRegressor {
    pub fn new(length_scale: f64, signal_var: f64, noise_var: f64) -> Self {
        Self {
            x_train: Vec::new(),
            y_train: Vec::new(),
            length_scale,
            signal_var,
            noise_var,
        }
    }
    /// RBF (squared exponential) kernel.
    fn kernel(&self, xi: f64, xj: f64) -> f64 {
        let d = xi - xj;
        self.signal_var * (-d * d / (2.0 * self.length_scale * self.length_scale)).exp()
    }
    /// Add a new observation.
    pub fn observe(&mut self, x: f64, y: f64) {
        self.x_train.push(x);
        self.y_train.push(y);
    }
    /// Compute the kernel matrix K(X, X) + noise * I.
    fn kernel_matrix_noisy(&self) -> Vec<Vec<f64>> {
        let n = self.x_train.len();
        let mut k = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                k[i][j] = self.kernel(self.x_train[i], self.x_train[j]);
                if i == j {
                    k[i][j] += self.noise_var;
                }
            }
        }
        k
    }
    /// Predict GP posterior mean and variance at a test point via naive inversion
    /// (Cholesky not implemented here; uses direct formula for small datasets).
    pub fn predict(&self, x_star: f64) -> (f64, f64) {
        let n = self.x_train.len();
        if n == 0 {
            return (0.0, self.kernel(x_star, x_star));
        }
        let k_star: Vec<f64> = self
            .x_train
            .iter()
            .map(|&xi| self.kernel(x_star, xi))
            .collect();
        let k_mat = self.kernel_matrix_noisy();
        let alpha = self.solve_linear(&k_mat, &self.y_train);
        let mu: f64 = k_star
            .iter()
            .zip(alpha.iter())
            .map(|(&ki, &ai)| ki * ai)
            .sum();
        let k_inv_k_star = self.solve_linear(&k_mat, &k_star);
        let kss = self.kernel(x_star, x_star) + self.noise_var;
        let var = kss
            - k_star
                .iter()
                .zip(k_inv_k_star.iter())
                .map(|(&ki, &vi)| ki * vi)
                .sum::<f64>();
        (mu, var.max(0.0))
    }
    /// Gaussian elimination for square system Ax = b.
    fn solve_linear(&self, a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let n = b.len();
        let mut mat: Vec<Vec<f64>> = a.to_vec();
        let mut rhs: Vec<f64> = b.to_vec();
        for col in 0..n {
            let pivot = (col..n)
                .max_by(|&i, &j| {
                    mat[i][col]
                        .abs()
                        .partial_cmp(&mat[j][col].abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(col);
            mat.swap(col, pivot);
            rhs.swap(col, pivot);
            let p = mat[col][col];
            if p.abs() < 1e-15 {
                continue;
            }
            for row in (col + 1)..n {
                let factor = mat[row][col] / p;
                for k in col..n {
                    let v = mat[col][k];
                    mat[row][k] -= factor * v;
                }
                rhs[row] -= factor * rhs[col];
            }
        }
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = rhs[i];
            for j in (i + 1)..n {
                sum -= mat[i][j] * x[j];
            }
            x[i] = if mat[i][i].abs() > 1e-15 {
                sum / mat[i][i]
            } else {
                0.0
            };
        }
        x
    }
    /// Bayesian quadrature: estimate ∫ f(x) p(x) dx where p = N(mu, sigma^2).
    pub fn bayesian_quadrature(&self, prior_mean: f64, prior_std: f64) -> f64 {
        let n = self.x_train.len();
        if n == 0 {
            return 0.0;
        }
        let z: Vec<f64> = self
            .x_train
            .iter()
            .map(|&xi| {
                let tau2 = self.length_scale * self.length_scale;
                let s2 = prior_std * prior_std;
                let eff_var = tau2 + s2;
                let d = prior_mean - xi;
                self.signal_var * (tau2 / eff_var).sqrt() * (-0.5 * d * d / eff_var).exp()
            })
            .collect();
        let alpha = self.solve_linear(&self.kernel_matrix_noisy(), &self.y_train);
        z.iter().zip(alpha.iter()).map(|(&zi, &ai)| zi * ai).sum()
    }
}
/// Mean-field VI: optimise ELBO with gradient ascent.
/// Assumes the variational family is a product of Gaussians: q(z) = Π_i N(μ_i, σ_i²).
pub struct MeanFieldVI {
    /// Variational means.
    pub mu: Vec<f64>,
    /// Variational log-standard-deviations (unconstrained).
    pub log_sigma: Vec<f64>,
    pub lr: f64,
    pub n_mc: usize,
    rng: Rng,
}
impl MeanFieldVI {
    pub fn new(d: usize, lr: f64, n_mc: usize, seed: u64) -> Self {
        Self {
            mu: vec![0.0; d],
            log_sigma: vec![0.0; d],
            lr,
            n_mc,
            rng: Rng::new(seed),
        }
    }
    /// Compute the ELBO estimate (single MC sample).
    pub fn elbo_estimate<F>(&mut self, log_joint: &F) -> f64
    where
        F: Fn(&[f64]) -> f64,
    {
        let d = self.mu.len();
        let sigma: Vec<f64> = self.log_sigma.iter().map(|&ls| ls.exp()).collect();
        let mut elbo = 0.0;
        for _ in 0..self.n_mc {
            let eps: Vec<f64> = (0..d).map(|_| self.rng.normal()).collect();
            let z: Vec<f64> = self
                .mu
                .iter()
                .zip(sigma.iter())
                .zip(eps.iter())
                .map(|((&m, &s), &e)| m + s * e)
                .collect();
            let log_p = log_joint(&z);
            let entropy: f64 = sigma.iter().map(|&s| s.ln()).sum::<f64>();
            elbo += log_p + entropy;
        }
        elbo / self.n_mc as f64
    }
    /// Single step of ELBO gradient ascent using reparameterisation.
    pub fn step<F>(&mut self, log_joint: F)
    where
        F: Fn(&[f64]) -> f64,
    {
        let d = self.mu.len();
        let delta = 1e-4;
        let mut grad_mu = vec![0.0f64; d];
        let mut grad_ls = vec![0.0f64; d];
        for _ in 0..self.n_mc {
            let eps: Vec<f64> = (0..d).map(|_| self.rng.normal()).collect();
            let sigma: Vec<f64> = self.log_sigma.iter().map(|&ls| ls.exp()).collect();
            let z: Vec<f64> = self
                .mu
                .iter()
                .zip(sigma.iter())
                .zip(eps.iter())
                .map(|((&m, &s), &e)| m + s * e)
                .collect();
            let lp = log_joint(&z);
            for i in 0..d {
                let mut z_plus = z.clone();
                z_plus[i] += delta;
                let mut z_minus = z.clone();
                z_minus[i] -= delta;
                grad_mu[i] += (log_joint(&z_plus) - log_joint(&z_minus)) / (2.0 * delta);
                grad_ls[i] += eps[i] * sigma[i] * (log_joint(&z_plus) - log_joint(&z_minus))
                    / (2.0 * delta)
                    + 1.0;
                let _ = lp;
            }
        }
        for i in 0..d {
            self.mu[i] += self.lr * grad_mu[i] / self.n_mc as f64;
            self.log_sigma[i] += self.lr * grad_ls[i] / self.n_mc as f64;
        }
    }
    /// Run VI for `n_steps` iterations, returning ELBO history.
    pub fn fit<F>(&mut self, log_joint: F, n_steps: usize) -> Vec<f64>
    where
        F: Fn(&[f64]) -> f64,
    {
        let mut history = Vec::with_capacity(n_steps);
        for _ in 0..n_steps {
            self.step(&log_joint);
            history.push(self.elbo_estimate(&log_joint));
        }
        history
    }
}
/// HMC sampler for a differentiable target distribution.
pub struct Hmc {
    pub step_size: f64,
    pub n_leapfrog: usize,
    rng: Rng,
}
impl Hmc {
    pub fn new(step_size: f64, n_leapfrog: usize, seed: u64) -> Self {
        Self {
            step_size,
            n_leapfrog,
            rng: Rng::new(seed),
        }
    }
    /// Leapfrog integrator for one trajectory.
    fn leapfrog<G>(&self, q0: &[f64], p0: &[f64], grad_log_target: &G) -> (Vec<f64>, Vec<f64>)
    where
        G: Fn(&[f64]) -> Vec<f64>,
    {
        let d = q0.len();
        let mut q = q0.to_vec();
        let mut p = p0.to_vec();
        let g = grad_log_target(&q);
        for i in 0..d {
            p[i] += 0.5 * self.step_size * g[i];
        }
        for _ in 0..self.n_leapfrog {
            for i in 0..d {
                q[i] += self.step_size * p[i];
            }
            let g = grad_log_target(&q);
            for i in 0..d {
                p[i] += self.step_size * g[i];
            }
        }
        let g = grad_log_target(&q);
        for i in 0..d {
            p[i] -= 0.5 * self.step_size * g[i];
        }
        (q, p)
    }
    /// Draw `n_samples` samples using HMC.
    /// `log_target`: log π(q); `grad_log_target`: ∇ log π(q).
    pub fn sample<F, G>(
        &mut self,
        init: Vec<f64>,
        n_samples: usize,
        log_target: F,
        grad_log_target: G,
    ) -> Vec<Vec<f64>>
    where
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
    {
        let d = init.len();
        let mut current_q = init;
        let mut samples = Vec::with_capacity(n_samples);
        for _ in 0..n_samples {
            let p0: Vec<f64> = (0..d).map(|_| self.rng.normal()).collect();
            let (proposed_q, proposed_p) = self.leapfrog(&current_q, &p0, &grad_log_target);
            let h_current =
                -log_target(&current_q) + 0.5 * p0.iter().map(|&pi| pi * pi).sum::<f64>();
            let h_proposed =
                -log_target(&proposed_q) + 0.5 * proposed_p.iter().map(|&pi| pi * pi).sum::<f64>();
            let accept_prob = (-h_proposed + h_current).exp().min(1.0);
            if self.rng.uniform() < accept_prob {
                current_q = proposed_q;
            }
            samples.push(current_q.clone());
        }
        samples
    }
}
/// Variational inference with ELBO optimisation.
pub struct VariationalInference {
    pub variational_family: String,
    pub elbo: f64,
}
impl VariationalInference {
    pub fn new(variational_family: impl Into<String>, elbo: f64) -> Self {
        Self {
            variational_family: variational_family.into(),
            elbo,
        }
    }
    /// Describe the ELBO optimisation procedure.
    pub fn optimize_elbo(&self) -> String {
        format!(
            "Optimise ELBO over family '{}'. \
             Current ELBO = {:.4}. \
             Use gradient ascent: ∇_φ ELBO = E_q[∇_φ log q_φ(z) · (log p(x,z) - log q_φ(z))] \
             (score function) or reparameterisation gradient.",
            self.variational_family, self.elbo
        )
    }
    /// Bound on the KL divergence KL(q||p).
    pub fn kl_divergence_bound(&self) -> String {
        format!(
            "KL(q||p) ≤ log p(x) - ELBO = -ELBO + log p(x). \
             Since ELBO = {:.4}, KL ≤ log p(x) - ({:.4}) = log p(x) + {:.4}.",
            self.elbo, self.elbo, -self.elbo
        )
    }
}
/// Sequential Monte Carlo with adaptive tempering (SMC-PT).
///
/// Uses adaptive temperature selection to maintain a target ESS ratio,
/// making it suitable for multimodal targets.
#[allow(dead_code)]
pub struct SmcAdaptiveTempering {
    pub n_particles: usize,
    /// Target ESS ratio (fraction of n_particles).
    pub ess_target: f64,
    rng: Rng,
}
#[allow(dead_code)]
impl SmcAdaptiveTempering {
    pub fn new(n_particles: usize, ess_target: f64, seed: u64) -> Self {
        Self {
            n_particles,
            ess_target,
            rng: Rng::new(seed),
        }
    }
    /// Binary search for the next temperature increment Δβ such that
    /// ESS(w_{t+Δβ}) ≈ ess_target * n_particles.
    fn find_next_beta(
        &self,
        particles: &[f64],
        log_weights: &[f64],
        log_target: &impl Fn(f64) -> f64,
        beta: f64,
    ) -> f64 {
        let target_ess = self.ess_target * self.n_particles as f64;
        let mut lo = 0.0_f64;
        let mut hi = 1.0 - beta;
        for _ in 0..50 {
            let delta = (lo + hi) / 2.0;
            let new_lw: Vec<f64> = particles
                .iter()
                .zip(log_weights.iter())
                .map(|(&x, &lw)| lw + delta * log_target(x))
                .collect();
            let ess = ImportanceSampler::effective_sample_size(&new_lw);
            if ess > target_ess {
                lo = delta;
            } else {
                hi = delta;
            }
        }
        beta + (lo + hi) / 2.0
    }
    /// Run adaptive SMC tempering from p_0(x) ∝ exp(0) to p_T(x) ∝ exp(log_target(x)).
    /// Returns final particle positions and log normalising constant estimate.
    pub fn run<LT>(
        &mut self,
        init_sample: impl Fn(&mut Rng) -> f64,
        log_target: LT,
    ) -> (Vec<f64>, f64)
    where
        LT: Fn(f64) -> f64,
    {
        let n = self.n_particles;
        let mut particles: Vec<f64> = (0..n).map(|_| init_sample(&mut self.rng)).collect();
        let mut log_weights = vec![0.0_f64; n];
        let mut log_z = 0.0;
        let mut beta = 0.0;
        while beta < 1.0 {
            let next_beta = self.find_next_beta(&particles, &log_weights, &log_target, beta);
            let delta = next_beta - beta;
            for (lw, &x) in log_weights.iter_mut().zip(particles.iter()) {
                *lw += delta * log_target(x);
            }
            let max_lw = log_weights
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            let sum_exp: f64 = log_weights.iter().map(|&lw| (lw - max_lw).exp()).sum();
            log_z += max_lw + (sum_exp / n as f64).ln();
            let w: Vec<f64> = {
                let s: f64 = log_weights.iter().map(|&lw| (lw - max_lw).exp()).sum();
                log_weights
                    .iter()
                    .map(|&lw| (lw - max_lw).exp() / s)
                    .collect()
            };
            let u0 = self.rng.uniform() / n as f64;
            let mut cumsum = 0.0;
            let mut j = 0;
            let mut resampled = Vec::with_capacity(n);
            for i in 0..n {
                let u = u0 + i as f64 / n as f64;
                while j + 1 < n && cumsum + w[j] < u {
                    cumsum += w[j];
                    j += 1;
                }
                resampled.push(particles[j]);
            }
            particles = resampled;
            log_weights = vec![0.0; n];
            for x in particles.iter_mut() {
                let proposal = *x + self.rng.normal() * 0.5;
                let log_p_curr = next_beta * log_target(*x);
                let log_p_prop = next_beta * log_target(proposal);
                if self.rng.uniform() < (log_p_prop - log_p_curr).exp().min(1.0) {
                    *x = proposal;
                }
            }
            beta = next_beta;
            if beta >= 1.0 {
                break;
            }
        }
        (particles, log_z)
    }
    /// Compute effective sample size of the final particle set.
    pub fn final_ess(log_weights: &[f64]) -> f64 {
        ImportanceSampler::effective_sample_size(log_weights)
    }
}
/// Score function (REINFORCE) gradient estimator for ∇_φ E_{z~q_φ}[f(z)].
pub struct ScoreFunctionEstimator {
    pub n_samples: usize,
    rng: Rng,
}
impl ScoreFunctionEstimator {
    pub fn new(n_samples: usize, seed: u64) -> Self {
        Self {
            n_samples,
            rng: Rng::new(seed),
        }
    }
    /// Estimate the gradient using the log-derivative trick.
    /// `sample_fn`: samples z_i from q_φ; `log_q_grad`: ∇_φ log q_φ(z);
    /// `f`: reward/loss function.
    pub fn estimate_gradient<S, Lq, F>(&mut self, sample_fn: S, log_q_grad: Lq, f: F) -> Vec<f64>
    where
        S: Fn(&mut Rng) -> Vec<f64>,
        Lq: Fn(&[f64]) -> Vec<f64>,
        F: Fn(&[f64]) -> f64,
    {
        let n = self.n_samples;
        let mut grad_sum: Option<Vec<f64>> = None;
        for _ in 0..n {
            let z = sample_fn(&mut self.rng);
            let fz = f(&z);
            let score = log_q_grad(&z);
            let contrib: Vec<f64> = score.iter().map(|&s| fz * s).collect();
            match &mut grad_sum {
                None => {
                    grad_sum = Some(contrib);
                }
                Some(ref mut g) => {
                    for (gi, ci) in g.iter_mut().zip(contrib.iter()) {
                        *gi += ci;
                    }
                }
            }
        }
        let grad = grad_sum.unwrap_or_default();
        grad.iter().map(|&g| g / n as f64).collect()
    }
}
/// Stochastic variational inference gradient estimators.
pub struct StochasticVI {
    pub elbo_gradient: Vec<f64>,
    pub num_samples: usize,
}
impl StochasticVI {
    pub fn new(elbo_gradient: Vec<f64>, num_samples: usize) -> Self {
        Self {
            elbo_gradient,
            num_samples,
        }
    }
    /// Describe the reparameterisation trick gradient estimator.
    pub fn reparameterization_trick(&self) -> String {
        format!(
            "Reparameterisation trick with {} sample(s): \
             z = g(φ, ε), ε ~ p(ε). \
             ∇_φ ELBO ≈ (1/N) Σ_i ∇_φ [log p(x,g(φ,εᵢ)) - log q_φ(g(φ,εᵢ))]. \
             Low variance; requires differentiable sampler.",
            self.num_samples
        )
    }
    /// Describe the score function (REINFORCE) gradient estimator.
    pub fn score_function_estimator(&self) -> String {
        format!(
            "Score function estimator (REINFORCE) with {} sample(s): \
             ∇_φ ELBO ≈ (1/N) Σ_i ∇_φ log q_φ(zᵢ) · (log p(x,zᵢ) - log q_φ(zᵢ)). \
             Gradient norm: {:.4}. High variance; use control variates.",
            self.num_samples,
            self.elbo_gradient
                .iter()
                .map(|&g| g * g)
                .sum::<f64>()
                .sqrt()
        )
    }
}
/// Self-normalised importance sampling estimator.
pub struct ImportanceSampler {
    pub n_samples: usize,
    rng: Rng,
}
impl ImportanceSampler {
    pub fn new(n_samples: usize, seed: u64) -> Self {
        Self {
            n_samples,
            rng: Rng::new(seed),
        }
    }
    /// Estimate E_p[f(x)] using proposal q.
    /// `proposal_sample`: draws from q; `log_weight`: log(p/q) at a sample.
    pub fn estimate<F, G, H>(&mut self, f: F, proposal_sample: G, log_weight: H) -> f64
    where
        F: Fn(f64) -> f64,
        G: Fn(&mut Rng) -> f64,
        H: Fn(f64) -> f64,
    {
        let mut weights = Vec::with_capacity(self.n_samples);
        let mut fvals = Vec::with_capacity(self.n_samples);
        for _ in 0..self.n_samples {
            let x = proposal_sample(&mut self.rng);
            let lw = log_weight(x);
            weights.push(lw);
            fvals.push(f(x));
        }
        let max_lw = weights.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let w_exp: Vec<f64> = weights.iter().map(|&lw| (lw - max_lw).exp()).collect();
        let sum_w: f64 = w_exp.iter().sum();
        w_exp
            .iter()
            .zip(fvals.iter())
            .map(|(&w, &fv)| w * fv)
            .sum::<f64>()
            / sum_w
    }
    /// Effective sample size (ESS) for given log-weights.
    pub fn effective_sample_size(log_weights: &[f64]) -> f64 {
        let max_lw = log_weights
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let w: Vec<f64> = log_weights.iter().map(|&lw| (lw - max_lw).exp()).collect();
        let sum_w: f64 = w.iter().sum();
        let sum_w2: f64 = w.iter().map(|&wi| wi * wi).sum();
        if sum_w2 < 1e-15 {
            0.0
        } else {
            sum_w * sum_w / sum_w2
        }
    }
}
/// Sequential importance sampling / particle filter.
pub struct BootstrapFilter {
    pub num_particles: usize,
    pub ess_threshold: f64,
}
impl BootstrapFilter {
    pub fn new(num_particles: usize, ess_threshold: f64) -> Self {
        Self {
            num_particles,
            ess_threshold,
        }
    }
    /// Run one step of sequential importance sampling.
    /// Returns particle weights after resampling.
    pub fn sequential_importance_sample(&self, log_weights: &[f64]) -> Vec<f64> {
        let max_lw = log_weights
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let w: Vec<f64> = log_weights.iter().map(|&lw| (lw - max_lw).exp()).collect();
        let sum_w: f64 = w.iter().sum();
        w.iter().map(|&wi| wi / sum_w).collect()
    }
    /// Compute effective sample size and decide whether to resample.
    pub fn resample(&self, weights: &[f64]) -> bool {
        let sum_w2: f64 = weights.iter().map(|&w| w * w).sum();
        let ess = if sum_w2 > 0.0 { 1.0 / sum_w2 } else { 0.0 };
        ess < self.ess_threshold * self.num_particles as f64
    }
}
/// Stein Variational Gradient Descent (SVGD) for approximate Bayesian inference.
///
/// Transports a set of particles to approximate the target distribution
/// using the kernelised Stein operator.
#[allow(dead_code)]
pub struct SteinVariationalGD {
    /// Bandwidth parameter for the RBF kernel.
    pub bandwidth: f64,
    /// Step size for the gradient descent.
    pub step_size: f64,
    rng: Rng,
}
#[allow(dead_code)]
impl SteinVariationalGD {
    pub fn new(bandwidth: f64, step_size: f64, seed: u64) -> Self {
        Self {
            bandwidth,
            step_size,
            rng: Rng::new(seed),
        }
    }
    /// RBF kernel k(x, y) = exp(-||x-y||^2 / h).
    fn rbf_kernel(&self, x: f64, y: f64) -> f64 {
        let diff = x - y;
        (-diff * diff / self.bandwidth).exp()
    }
    /// Gradient of the RBF kernel w.r.t. the first argument.
    fn rbf_kernel_grad(&self, x: f64, y: f64) -> f64 {
        let diff = x - y;
        -2.0 * diff / self.bandwidth * self.rbf_kernel(x, y)
    }
    /// Compute the SVGD update direction for each particle.
    /// `score`: the score function ∇ log target(x) at a particle.
    pub fn update_direction<S>(&self, particles: &[f64], score: S) -> Vec<f64>
    where
        S: Fn(f64) -> f64,
    {
        let n = particles.len() as f64;
        particles
            .iter()
            .map(|&xi| {
                let phi: f64 = particles
                    .iter()
                    .map(|&xj| self.rbf_kernel(xj, xi) * score(xj) + self.rbf_kernel_grad(xj, xi))
                    .sum::<f64>()
                    / n;
                phi
            })
            .collect()
    }
    /// Run SVGD for `n_iter` iterations.
    /// Returns the final particle positions.
    pub fn run<S>(&self, mut particles: Vec<f64>, n_iter: usize, score: S) -> Vec<f64>
    where
        S: Fn(f64) -> f64,
    {
        for _ in 0..n_iter {
            let directions = self.update_direction(&particles, &score);
            for (p, d) in particles.iter_mut().zip(directions.iter()) {
                *p += self.step_size * d;
            }
        }
        particles
    }
    /// Compute the kernel Stein discrepancy estimate (U-statistic).
    pub fn kernel_stein_discrepancy<S>(&self, particles: &[f64], score: S) -> f64
    where
        S: Fn(f64) -> f64,
    {
        let n = particles.len();
        if n < 2 {
            return 0.0;
        }
        let mut sum = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let xi = particles[i];
                    let xj = particles[j];
                    let kij = self.rbf_kernel(xi, xj);
                    let dxi_kij = self.rbf_kernel_grad(xi, xj);
                    let dxj_kij = self.rbf_kernel_grad(xj, xi);
                    sum += score(xi) * score(xj) * kij + score(xi) * dxj_kij + score(xj) * dxi_kij;
                }
            }
        }
        sum / ((n * (n - 1)) as f64)
    }
}
/// Reparameterised gradient estimator for ∇_φ E_{z~q_φ}[f(z)].
/// Uses the reparameterisation z = g(ε, φ) where ε ~ p(ε) is noise-free.
pub struct ReparamEstimator {
    pub n_samples: usize,
    rng: Rng,
}
impl ReparamEstimator {
    pub fn new(n_samples: usize, seed: u64) -> Self {
        Self {
            n_samples,
            rng: Rng::new(seed),
        }
    }
    /// Estimate gradient via finite differences on the reparameterised objective.
    /// `transform`: z = g(ε, φ) — the reparameterisation;
    /// `f`: the objective; `phi`: current variational parameters; `eps_sample`: sample ε.
    #[allow(clippy::too_many_arguments)]
    pub fn estimate_gradient<T, F, Es>(
        &mut self,
        transform: T,
        f: F,
        phi: &[f64],
        eps_sample: Es,
        delta: f64,
    ) -> Vec<f64>
    where
        T: Fn(&[f64], &[f64]) -> Vec<f64>,
        F: Fn(&[f64]) -> f64,
        Es: Fn(&mut Rng) -> Vec<f64>,
    {
        let d = phi.len();
        let mut grad = vec![0.0f64; d];
        for _ in 0..self.n_samples {
            let eps = eps_sample(&mut self.rng);
            for i in 0..d {
                let mut phi_plus = phi.to_vec();
                phi_plus[i] += delta;
                let mut phi_minus = phi.to_vec();
                phi_minus[i] -= delta;
                let z_plus = transform(&eps, &phi_plus);
                let z_minus = transform(&eps, &phi_minus);
                grad[i] += (f(&z_plus) - f(&z_minus)) / (2.0 * delta);
            }
        }
        grad.iter().map(|&g| g / self.n_samples as f64).collect()
    }
}
/// A probabilistic program with discrete statements and random choices.
pub struct ProbProgram {
    pub statements: Vec<String>,
    pub random_choices: Vec<String>,
}
impl ProbProgram {
    pub fn new(statements: Vec<String>, random_choices: Vec<String>) -> Self {
        Self {
            statements,
            random_choices,
        }
    }
    /// Describe the posterior query for this program.
    pub fn posterior_query(&self) -> String {
        format!(
            "Posterior query over {} random choice(s) given {} statement(s). \
             Compute P(latents | observations) using Bayes' rule.",
            self.random_choices.len(),
            self.statements.len()
        )
    }
    /// Compute the marginal distribution over the given variable.
    pub fn marginal(&self, variable: &str) -> String {
        format!(
            "Marginal of '{}': integrate out all other latent variables. \
             Equivalent to projecting the joint posterior onto '{}'.",
            variable, variable
        )
    }
}
/// A bootstrap particle filter for state-space models.
pub struct ParticleFilter {
    pub n_particles: usize,
    rng: Rng,
}
impl ParticleFilter {
    pub fn new(n_particles: usize, seed: u64) -> Self {
        Self {
            n_particles,
            rng: Rng::new(seed),
        }
    }
    /// Run the bootstrap PF on a sequence of observations.
    ///
    /// - `transition`: x_t | x_{t-1} sampling
    /// - `obs_weight`: log P(y_t | x_t)
    /// - `init_sample`: initial state sample
    ///
    /// Returns the sequence of particle clouds (one per time step).
    pub fn filter<Tr, Ow, Is>(
        &mut self,
        observations: &[f64],
        init_sample: Is,
        transition: Tr,
        obs_weight: Ow,
    ) -> Vec<Vec<f64>>
    where
        Tr: Fn(f64, &mut Rng) -> f64,
        Ow: Fn(f64, f64) -> f64,
        Is: Fn(&mut Rng) -> f64,
    {
        let n = self.n_particles;
        let mut particles: Vec<f64> = (0..n).map(|_| init_sample(&mut self.rng)).collect();
        let mut all_clouds = Vec::with_capacity(observations.len());
        for &y in observations {
            let proposed: Vec<f64> = particles
                .iter()
                .map(|&x| transition(x, &mut self.rng))
                .collect();
            let log_weights: Vec<f64> = proposed.iter().map(|&x| obs_weight(x, y)).collect();
            let max_lw = log_weights
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            let weights: Vec<f64> = log_weights.iter().map(|&lw| (lw - max_lw).exp()).collect();
            let sum_w: f64 = weights.iter().sum();
            let norm_w: Vec<f64> = weights.iter().map(|&w| w / sum_w.max(1e-15)).collect();
            particles = self.systematic_resample(&proposed, &norm_w);
            all_clouds.push(particles.clone());
        }
        all_clouds
    }
    fn systematic_resample(&mut self, particles: &[f64], weights: &[f64]) -> Vec<f64> {
        let n = particles.len();
        let mut cumsum = vec![0.0f64; n + 1];
        for (i, &w) in weights.iter().enumerate() {
            cumsum[i + 1] = cumsum[i] + w;
        }
        let u0 = self.rng.uniform() / n as f64;
        let mut resampled = Vec::with_capacity(n);
        let mut j = 0usize;
        for i in 0..n {
            let u = u0 + i as f64 / n as f64;
            while j + 1 < n && cumsum[j + 1] < u {
                j += 1;
            }
            resampled.push(particles[j]);
        }
        resampled
    }
    /// Estimate the filtered mean at each time step.
    pub fn filter_mean<Tr, Ow, Is>(
        &mut self,
        observations: &[f64],
        init_sample: Is,
        transition: Tr,
        obs_weight: Ow,
    ) -> Vec<f64>
    where
        Tr: Fn(f64, &mut Rng) -> f64,
        Ow: Fn(f64, f64) -> f64,
        Is: Fn(&mut Rng) -> f64,
    {
        self.filter(observations, init_sample, transition, obs_weight)
            .iter()
            .map(|cloud| cloud.iter().sum::<f64>() / cloud.len() as f64)
            .collect()
    }
}
/// Normalizing flow: composition of bijections for density estimation.
pub struct NormalizingFlow {
    pub bijections: Vec<String>,
    pub base_dist: String,
}
impl NormalizingFlow {
    pub fn new(bijections: Vec<String>, base_dist: impl Into<String>) -> Self {
        Self {
            bijections,
            base_dist: base_dist.into(),
        }
    }
    /// Describe the change-of-variables formula.
    pub fn change_of_variables(&self) -> String {
        format!(
            "Change of variables: p_X(x) = p_Z(f⁻¹(x)) · |det J_f⁻¹(x)|. \
             Flow has {} bijection(s): [{}]. Base: '{}'.",
            self.bijections.len(),
            self.bijections.join(" ∘ "),
            self.base_dist
        )
    }
    /// Compute the log-likelihood under the flow.
    pub fn log_likelihood(&self, x: f64) -> f64 {
        let z = x;
        let log_p_z = -0.5 * z * z - 0.5 * (2.0 * std::f64::consts::PI).ln();
        let log_det_jac = 0.0_f64;
        log_p_z + log_det_jac
    }
}
/// Annealed Importance Sampling (AIS) for estimating normalising constants.
///
/// Interpolates between a tractable base distribution and the target via
/// a sequence of intermediate distributions, using MCMC transitions.
#[allow(dead_code)]
pub struct AnnealedIS {
    /// Number of intermediate distributions.
    pub n_temps: usize,
    /// Number of samples.
    pub n_samples: usize,
    rng: Rng,
}
#[allow(dead_code)]
impl AnnealedIS {
    pub fn new(n_temps: usize, n_samples: usize, seed: u64) -> Self {
        Self {
            n_temps,
            n_samples,
            rng: Rng::new(seed),
        }
    }
    /// Compute log weight for one AIS trajectory.
    /// `log_p_base`: log of base (easy) distribution;
    /// `log_p_target`: log of target (hard) distribution;
    /// `sample_base`: draw from base distribution.
    pub fn log_weight<Lb, Lt, Sb>(
        &mut self,
        log_p_base: Lb,
        log_p_target: Lt,
        sample_base: Sb,
    ) -> f64
    where
        Lb: Fn(f64) -> f64,
        Lt: Fn(f64) -> f64,
        Sb: Fn(&mut Rng) -> f64,
    {
        let mut x = sample_base(&mut self.rng);
        let mut log_w = 0.0;
        let n = self.n_temps;
        for t in 1..=n {
            let beta_prev = (t - 1) as f64 / n as f64;
            let beta = t as f64 / n as f64;
            let log_prev = beta_prev * log_p_target(x) + (1.0 - beta_prev) * log_p_base(x);
            let log_curr = beta * log_p_target(x) + (1.0 - beta) * log_p_base(x);
            log_w += log_curr - log_prev;
            let proposal = x + self.rng.normal() * 0.5;
            let log_p_curr = beta * log_p_target(x) + (1.0 - beta) * log_p_base(x);
            let log_p_prop = beta * log_p_target(proposal) + (1.0 - beta) * log_p_base(proposal);
            if self.rng.uniform() < (log_p_prop - log_p_curr).exp().min(1.0) {
                x = proposal;
            }
        }
        log_w
    }
    /// Estimate log Z (log normalising constant) via AIS.
    pub fn estimate_log_z<Lb, Lt, Sb>(
        &mut self,
        log_p_base: Lb,
        log_p_target: Lt,
        sample_base: Sb,
    ) -> f64
    where
        Lb: Fn(f64) -> f64,
        Lt: Fn(f64) -> f64,
        Sb: Fn(&mut Rng) -> f64,
    {
        let n = self.n_samples;
        let mut log_weights = Vec::with_capacity(n);
        for _ in 0..n {
            let lw = self.log_weight(&log_p_base, &log_p_target, &sample_base);
            log_weights.push(lw);
        }
        let max_lw = log_weights
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let sum_exp: f64 = log_weights.iter().map(|&lw| (lw - max_lw).exp()).sum();
        max_lw + (sum_exp / n as f64).ln()
    }
}
/// MCMC sampling algorithms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MCMCSampler {
    MetropolisHastings,
    HMC,
    NUTS,
    Gibbs,
}
impl MCMCSampler {
    /// Describe the proposal distribution used by this sampler.
    pub fn proposal_distribution(&self) -> &'static str {
        match self {
            MCMCSampler::MetropolisHastings => {
                "Symmetric Gaussian random walk q(x'|x) = N(x, σ²I)."
            }
            MCMCSampler::HMC => "Leapfrog integrator on augmented (q,p) space; p ~ N(0,M).",
            MCMCSampler::NUTS => "No-U-Turn criterion: doubles trajectory until U-turn detected.",
            MCMCSampler::Gibbs => "Full conditional p(x_i | x_{-i}) — coordinate-wise proposals.",
        }
    }
    /// Describe the acceptance ratio used by this sampler.
    pub fn acceptance_ratio(&self) -> &'static str {
        match self {
            MCMCSampler::MetropolisHastings => {
                "α = min(1, p(x') q(x|x') / (p(x) q(x'|x))). Symmetric q → α = min(1, p(x')/p(x))."
            }
            MCMCSampler::HMC => "α = min(1, exp(H(q,p) - H(q',p'))). H = -log p(q) + ½ pᵀM⁻¹p.",
            MCMCSampler::NUTS => {
                "Slice sampler on valid tree leaves; no MH correction needed (NUTS is exact)."
            }
            MCMCSampler::Gibbs => "α = 1 always (Gibbs proposals are exact conditionals).",
        }
    }
}
/// Deep generative model variants.
#[derive(Debug, Clone)]
pub enum DeepGenerativeModel {
    /// Variational Autoencoder (Kingma & Welling 2014).
    VAE,
    /// Generative Adversarial Network (Goodfellow et al. 2014).
    GAN,
    /// Normalizing Flow model.
    NormalizingFlow(String),
    /// Diffusion model (DDPM).
    DiffusionModel,
}
impl DeepGenerativeModel {
    /// Return the latent dimension (conceptual; returns 0 for GANs as it is amortised).
    pub fn latent_dim(&self) -> usize {
        match self {
            DeepGenerativeModel::VAE => 64,
            DeepGenerativeModel::GAN => 0,
            DeepGenerativeModel::NormalizingFlow(_) => 128,
            DeepGenerativeModel::DiffusionModel => 256,
        }
    }
    /// Return whether this model provides an explicit density estimate.
    pub fn is_explicit_density(&self) -> bool {
        match self {
            DeepGenerativeModel::VAE => false,
            DeepGenerativeModel::GAN => false,
            DeepGenerativeModel::NormalizingFlow(_) => true,
            DeepGenerativeModel::DiffusionModel => false,
        }
    }
}
/// The `sample` primitive: draw from a named distribution.
#[derive(Debug, Clone)]
pub enum Distribution {
    Normal { mean: f64, std: f64 },
    Uniform { lo: f64, hi: f64 },
    Bernoulli { p: f64 },
    Categorical { probs: Vec<f64> },
    Exponential { rate: f64 },
}
impl Distribution {
    /// Draw one sample from the distribution.
    pub fn sample(&self, rng: &mut Rng) -> f64 {
        match self {
            Distribution::Normal { mean, std } => rng.normal_mv(*mean, *std),
            Distribution::Uniform { lo, hi } => lo + (hi - lo) * rng.uniform(),
            Distribution::Bernoulli { p } => {
                if rng.uniform() < *p {
                    1.0
                } else {
                    0.0
                }
            }
            Distribution::Categorical { probs } => {
                let u = rng.uniform();
                let mut cumsum = 0.0;
                for (i, &p) in probs.iter().enumerate() {
                    cumsum += p;
                    if u < cumsum {
                        return i as f64;
                    }
                }
                (probs.len() - 1) as f64
            }
            Distribution::Exponential { rate } => -rng.uniform().max(1e-15).ln() / rate,
        }
    }
    /// Log-density at `x`.
    pub fn log_density(&self, x: f64) -> f64 {
        match self {
            Distribution::Normal { mean, std } => {
                let z = (x - mean) / std;
                -0.5 * z * z - std.ln() - 0.5 * (2.0 * std::f64::consts::PI).ln()
            }
            Distribution::Uniform { lo, hi } => {
                if x >= *lo && x <= *hi {
                    -(hi - lo).ln()
                } else {
                    f64::NEG_INFINITY
                }
            }
            Distribution::Bernoulli { p } => {
                if x == 1.0 {
                    p.ln()
                } else {
                    (1.0 - p).ln()
                }
            }
            Distribution::Exponential { rate } => {
                if x >= 0.0 {
                    rate.ln() - rate * x
                } else {
                    f64::NEG_INFINITY
                }
            }
            Distribution::Categorical { probs } => {
                let i = x as usize;
                if i < probs.len() {
                    probs[i].max(1e-300).ln()
                } else {
                    f64::NEG_INFINITY
                }
            }
        }
    }
}
/// Simple probability distributions with analytical properties.
/// Uses deterministic pseudo-sampling (no external RNG dependency).
#[derive(Debug, Clone)]
pub enum Dist {
    /// Bernoulli(p): success probability p.
    Bernoulli(f64),
    /// Gaussian(mean, std): normal distribution.
    Gaussian(f64, f64),
    /// Categorical(probs): discrete distribution over k categories.
    Categorical(Vec<f64>),
    /// Dirichlet(alphas): Dirichlet distribution over a simplex.
    Dirichlet(Vec<f64>),
    /// Beta(alpha, beta): beta distribution.
    Beta(f64, f64),
}
impl Dist {
    /// Deterministic pseudo-sample: returns the mean (mode for Categorical).
    pub fn sample(&self) -> f64 {
        match self {
            Dist::Bernoulli(p) => *p,
            Dist::Gaussian(mu, _sigma) => *mu,
            Dist::Categorical(probs) => probs
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i as f64)
                .unwrap_or(0.0),
            Dist::Dirichlet(alphas) => {
                let sum: f64 = alphas.iter().sum();
                if sum > 0.0 {
                    alphas[0] / sum
                } else {
                    0.0
                }
            }
            Dist::Beta(a, b) => {
                if *a > 1.0 && *b > 1.0 {
                    (a - 1.0) / (a + b - 2.0)
                } else {
                    a / (a + b)
                }
            }
        }
    }
    /// Compute the mean of the distribution.
    pub fn mean(&self) -> f64 {
        match self {
            Dist::Bernoulli(p) => *p,
            Dist::Gaussian(mu, _) => *mu,
            Dist::Categorical(probs) => probs.iter().enumerate().map(|(i, &p)| i as f64 * p).sum(),
            Dist::Dirichlet(alphas) => {
                let sum: f64 = alphas.iter().sum();
                if sum > 0.0 {
                    alphas[0] / sum
                } else {
                    0.0
                }
            }
            Dist::Beta(a, b) => a / (a + b),
        }
    }
    /// Compute the variance of the distribution.
    pub fn variance(&self) -> f64 {
        match self {
            Dist::Bernoulli(p) => p * (1.0 - p),
            Dist::Gaussian(_, sigma) => sigma * sigma,
            Dist::Categorical(probs) => {
                let mu = self.mean();
                probs
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| p * (i as f64 - mu).powi(2))
                    .sum()
            }
            Dist::Dirichlet(alphas) => {
                let sum: f64 = alphas.iter().sum();
                let a0 = alphas[0];
                if sum > 0.0 {
                    (a0 * (sum - a0)) / (sum * sum * (sum + 1.0))
                } else {
                    0.0
                }
            }
            Dist::Beta(a, b) => {
                let ab = a + b;
                a * b / (ab * ab * (ab + 1.0))
            }
        }
    }
    /// Compute the entropy of the distribution (in nats).
    pub fn entropy(&self) -> f64 {
        match self {
            Dist::Bernoulli(p) => {
                let q = 1.0 - p;
                let h = |x: f64| if x > 0.0 { -x * x.ln() } else { 0.0 };
                h(*p) + h(q)
            }
            Dist::Gaussian(_, sigma) => {
                0.5 * (2.0 * std::f64::consts::PI * std::f64::consts::E * sigma * sigma).ln()
            }
            Dist::Categorical(probs) => probs
                .iter()
                .map(|&p| if p > 0.0 { -p * p.ln() } else { 0.0 })
                .sum(),
            Dist::Dirichlet(alphas) => {
                let sum: f64 = alphas.iter().sum();
                let k = alphas.len() as f64;
                let digamma = |x: f64| x.ln() - 1.0 / (2.0 * x);
                let ln_b: f64 = alphas.iter().map(|&a| lgamma(a)).sum::<f64>() - lgamma(sum);
                ln_b + (sum - k) * digamma(sum)
                    - alphas.iter().map(|&a| (a - 1.0) * digamma(a)).sum::<f64>()
            }
            Dist::Beta(a, b) => {
                let ab = a + b;
                let digamma = |x: f64| x.ln() - 1.0 / (2.0 * x);
                ln_beta(*a, *b) - (a - 1.0) * digamma(*a) - (b - 1.0) * digamma(*b)
                    + (ab - 2.0) * digamma(ab)
            }
        }
    }
}
/// Church-style probabilistic program semantics.
pub struct ChurchStyle {
    pub program: String,
}
impl ChurchStyle {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
        }
    }
    /// Describe the denotational semantics (measure transformer).
    pub fn denotational_semantics(&self) -> String {
        format!(
            "Church denotational semantics for '{}': \
             ⟦P⟧ : Meas(Env) → Meas(Val) is the sub-probability kernel induced by P. \
             `sample d` draws from kernel d; `observe c` conditions on event c \
             (weight by likelihood); `query` marginalises.",
            self.program
        )
    }
    /// Compute the log trace likelihood for the program execution.
    pub fn trace_likelihood(&self) -> f64 {
        0.0
    }
}
/// A lightweight deterministic PRNG for reproducible sampling.
pub struct Rng {
    state: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
    /// Standard normal via Box-Muller.
    pub fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-15);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
    /// Normal with given mean and std.
    pub fn normal_mv(&mut self, mean: f64, std: f64) -> f64 {
        mean + std * self.normal()
    }
}
/// Posterior predictive distribution for a Bayesian model.
pub struct PosteriorPredictive {
    pub model: String,
    pub data: Vec<f64>,
}
impl PosteriorPredictive {
    pub fn new(model: impl Into<String>, data: Vec<f64>) -> Self {
        Self {
            model: model.into(),
            data,
        }
    }
    /// Describe the posterior predictive distribution.
    pub fn predictive_distribution(&self) -> String {
        format!(
            "Posterior predictive for '{}' given {} data points: \
             p(x_new | x_obs) = ∫ p(x_new | θ) p(θ | x_obs) dθ. \
             Approximated via Monte Carlo: average likelihood over posterior samples.",
            self.model,
            self.data.len()
        )
    }
    /// Compute a credible interval for the posterior predictive (mean ± 2 std).
    pub fn credible_interval(&self) -> (f64, f64) {
        if self.data.is_empty() {
            return (0.0, 0.0);
        }
        let n = self.data.len() as f64;
        let mean = self.data.iter().sum::<f64>() / n;
        let var = self.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let std = var.sqrt();
        (mean - 2.0 * std, mean + 2.0 * std)
    }
}
