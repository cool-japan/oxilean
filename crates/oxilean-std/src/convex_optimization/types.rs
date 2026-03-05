//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::functions::{ConvexFunction, ProxableFunction};

/// Gradient descent optimizer.
#[derive(Debug, Clone)]
pub struct GradientDescent {
    /// Fixed learning rate.
    pub learning_rate: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance (on gradient norm).
    pub tol: f64,
}
impl GradientDescent {
    /// Create a new gradient descent optimizer.
    pub fn new(lr: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            learning_rate: lr,
            max_iter,
            tol,
        }
    }
    /// Backtracking line search satisfying the Armijo condition.
    ///
    /// Returns a step size α such that f(x - α ∇f(x)) ≤ f(x) - c₁ α ‖∇f(x)‖².
    pub fn backtracking_line_search<F: ConvexFunction>(f: &F, x: &[f64], grad: &[f64]) -> f64 {
        let c1 = 1e-4_f64;
        let rho = 0.5_f64;
        let mut alpha = 1.0_f64;
        let fx = f.eval(x);
        let grad_norm_sq: f64 = grad.iter().map(|g| g * g).sum();
        for _ in 0..50 {
            let x_new: Vec<f64> = x.iter().zip(grad).map(|(xi, gi)| xi - alpha * gi).collect();
            if f.eval(&x_new) <= fx - c1 * alpha * grad_norm_sq {
                break;
            }
            alpha *= rho;
        }
        alpha
    }
    /// Minimize `f` starting from `x0`.
    ///
    /// Returns `(x_star, f(x_star), iterations)`.
    pub fn minimize<F: ConvexFunction>(&self, f: &F, x0: &[f64]) -> (Vec<f64>, f64, usize) {
        let mut x = x0.to_vec();
        let mut iters = 0_usize;
        for k in 0..self.max_iter {
            let grad = f.gradient(&x);
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tol {
                iters = k;
                break;
            }
            let alpha = Self::backtracking_line_search(f, &x, &grad);
            for (xi, gi) in x.iter_mut().zip(&grad) {
                *xi -= alpha * gi;
            }
            iters = k + 1;
        }
        let fval = f.eval(&x);
        (x, fval, iters)
    }
}
/// Mirror descent optimizer using a Bregman divergence generating function.
///
/// Minimizes f(x) over a convex domain using the update:
///   x^{t+1} = argmin_{x in X} { eta * <grad f(x^t), x> + D_h(x, x^t) }
///
/// With negative entropy as h, this gives the Multiplicative Weights / Hedge algorithm.
/// With squared Euclidean norm as h, this reduces to standard gradient descent.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MirrorDescentSolver {
    /// Learning rate eta.
    pub eta: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
    /// Whether to use negative entropy (simplex domain) or Euclidean norm (R^n).
    pub use_entropy: bool,
}
impl MirrorDescentSolver {
    /// Create a new mirror descent solver.
    pub fn new(eta: f64, max_iter: usize, tol: f64, use_entropy: bool) -> Self {
        Self {
            eta,
            max_iter,
            tol,
            use_entropy,
        }
    }
    /// Project onto the probability simplex: x_i >= 0, sum x_i = 1.
    pub fn project_simplex(v: &[f64]) -> Vec<f64> {
        let _n = v.len();
        let mut u: Vec<f64> = v.to_vec();
        u.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        let mut cssv = 0.0_f64;
        let mut rho = 0_usize;
        for (j, &uj) in u.iter().enumerate() {
            cssv += uj;
            if uj - (cssv - 1.0) / (j as f64 + 1.0) > 0.0 {
                rho = j;
            }
        }
        let cssv_rho: f64 = u[..=rho].iter().sum();
        let theta = (cssv_rho - 1.0) / (rho as f64 + 1.0);
        v.iter().map(|vi| (vi - theta).max(0.0)).collect()
    }
    /// Mirror descent step with negative entropy (Multiplicative Weights):
    ///   x^{t+1}_i = x^t_i * exp(-eta * grad_i) / Z
    fn entropy_step(x: &[f64], grad: &[f64], eta: f64) -> Vec<f64> {
        let log_x_new: Vec<f64> = x
            .iter()
            .zip(grad)
            .map(|(xi, gi)| xi.ln() - eta * gi)
            .collect();
        let max_log = log_x_new.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_vals: Vec<f64> = log_x_new.iter().map(|v| (v - max_log).exp()).collect();
        let z: f64 = exp_vals.iter().sum();
        exp_vals.iter().map(|v| v / z).collect()
    }
    /// Run mirror descent to minimize `f`. Returns `(x_star, f(x_star), iterations)`.
    pub fn minimize<F: ConvexFunction>(&self, f: &F, x0: &[f64]) -> (Vec<f64>, f64, usize) {
        let mut x = if self.use_entropy {
            Self::project_simplex(x0)
        } else {
            x0.to_vec()
        };
        let mut best_x = x.clone();
        let mut best_f = f.eval(&x);
        let mut iters = self.max_iter;
        for k in 0..self.max_iter {
            let grad = f.gradient(&x);
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tol {
                iters = k;
                break;
            }
            let x_new = if self.use_entropy {
                Self::entropy_step(&x, &grad, self.eta)
            } else {
                x.iter()
                    .zip(&grad)
                    .map(|(xi, gi)| xi - self.eta * gi)
                    .collect()
            };
            let fx_new = f.eval(&x_new);
            if fx_new < best_f {
                best_f = fx_new;
                best_x = x_new.clone();
            }
            x = x_new;
        }
        (best_x, best_f, iters)
    }
    /// Compute the Bregman divergence D_h(x, y) for h = negative entropy:
    ///   D_h(x,y) = sum_i [ x_i * log(x_i / y_i) - x_i + y_i ]  (KL divergence).
    pub fn bregman_kl(x: &[f64], y: &[f64]) -> f64 {
        x.iter()
            .zip(y)
            .map(|(xi, yi)| {
                if *xi <= 0.0 {
                    return 0.0;
                }
                xi * (xi / yi).ln() - xi + yi
            })
            .sum()
    }
}
/// Verifier for the Restricted Isometry Property (RIP) of a matrix.
///
/// Checks the RIP-s condition: for all s-sparse vectors x,
///   (1 - delta) ||x||^2 <= ||Ax||^2 <= (1 + delta) ||x||^2.
///
/// Uses a greedy approximation by testing random sparse vectors.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RipVerifier {
    /// Sparsity level s to check.
    pub sparsity: usize,
    /// Number of random tests to perform.
    pub num_trials: usize,
}
impl RipVerifier {
    /// Create a new RIP verifier.
    pub fn new(sparsity: usize, num_trials: usize) -> Self {
        Self {
            sparsity,
            num_trials,
        }
    }
    /// Compute Ax for matrix A (m x n) and vector x (n).
    fn mat_vec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
        a.iter()
            .map(|row| row.iter().zip(x).map(|(aij, xi)| aij * xi).sum::<f64>())
            .collect()
    }
    /// Estimate the RIP constant delta_s for matrix A by testing sparse vectors.
    ///
    /// Returns `(delta_lower, delta_upper)`: the tightest bounds found over trials.
    /// A small delta_upper < 1 indicates A likely satisfies RIP-s.
    pub fn estimate_rip_constant(&self, a: &[Vec<f64>]) -> (f64, f64) {
        if a.is_empty() {
            return (0.0, 0.0);
        }
        let n = a[0].len();
        let s = self.sparsity.min(n);
        let mut delta_lower = 0.0_f64;
        let mut delta_upper = 0.0_f64;
        for trial in 0..self.num_trials {
            let mut x = vec![0.0_f64; n];
            for k in 0..s {
                let idx = (trial * s + k) % n;
                x[idx] = if k % 2 == 0 { 1.0 } else { -1.0 };
            }
            let x_norm_sq: f64 = x.iter().map(|xi| xi * xi).sum();
            if x_norm_sq < 1e-12 {
                continue;
            }
            let ax = Self::mat_vec(a, &x);
            let ax_norm_sq: f64 = ax.iter().map(|axi| axi * axi).sum();
            let ratio = ax_norm_sq / x_norm_sq;
            let dev = (ratio - 1.0).abs();
            if dev > delta_upper {
                delta_upper = dev;
            }
            let lower_dev = 1.0 - ratio;
            if lower_dev > delta_lower {
                delta_lower = lower_dev;
            }
        }
        (delta_lower, delta_upper)
    }
    /// Check whether matrix A satisfies RIP-s with parameter delta.
    ///
    /// Returns true if the estimated RIP constant is less than delta.
    pub fn satisfies_rip(&self, a: &[Vec<f64>], delta: f64) -> bool {
        let (_, upper) = self.estimate_rip_constant(a);
        upper < delta
    }
    /// Soft thresholding operator for basis pursuit denoising / LASSO.
    ///
    /// Returns the element-wise soft threshold: sign(x_i) * max(|x_i| - lambda, 0).
    pub fn soft_threshold(x: &[f64], lambda: f64) -> Vec<f64> {
        x.iter()
            .map(|xi| xi.signum() * (xi.abs() - lambda).max(0.0))
            .collect()
    }
}
/// Cutting-plane solver (Kelley's method) for convex nonsmooth minimization.
///
/// Maintains a piecewise-linear lower model built from subgradient cuts and
/// solves QP subproblems approximately using gradient descent on the model.
#[derive(Debug, Clone)]
pub struct CuttingPlaneSolver {
    /// Maximum number of cutting-plane iterations.
    pub max_iter: usize,
    /// Convergence tolerance (on the optimality gap).
    pub tol: f64,
    /// Trust-region radius for the QP subproblem.
    pub trust_radius: f64,
}
impl CuttingPlaneSolver {
    /// Create a new cutting-plane solver.
    pub fn new(max_iter: usize, tol: f64, trust_radius: f64) -> Self {
        Self {
            max_iter,
            tol,
            trust_radius,
        }
    }
    /// Minimize `f` starting from `x0` using Kelley's cutting-plane method.
    ///
    /// Returns `(x_star, f(x_star), iterations)`.
    pub fn minimize<F: ConvexFunction>(&self, f: &F, x0: &[f64]) -> (Vec<f64>, f64, usize) {
        let n = x0.len();
        let mut x = x0.to_vec();
        let mut cuts: Vec<(Vec<f64>, f64, Vec<f64>)> = Vec::new();
        let mut best_x = x.clone();
        let mut best_f = f.eval(&x);
        let mut iters = self.max_iter;
        for k in 0..self.max_iter {
            let fk = f.eval(&x);
            let gk = f.gradient(&x);
            cuts.push((x.clone(), fk, gk.clone()));
            if fk < best_f {
                best_f = fk;
                best_x = x.clone();
            }
            let mut z = x.clone();
            let step_model = 0.01_f64 * self.trust_radius;
            for _ in 0..200 {
                let model_vals: Vec<f64> = cuts
                    .iter()
                    .map(|(xj, fj, gj)| {
                        fj + gj
                            .iter()
                            .zip(&z)
                            .zip(xj.iter())
                            .map(|((gi, zi), xji)| gi * (zi - xji))
                            .sum::<f64>()
                    })
                    .collect();
                let active = model_vals
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                let grad_model = &cuts[active].2;
                let mut z_new: Vec<f64> = z
                    .iter()
                    .zip(grad_model)
                    .map(|(zi, gi)| zi - step_model * gi)
                    .collect();
                let dist_sq: f64 = z_new.iter().zip(&x).map(|(zi, xi)| (zi - xi).powi(2)).sum();
                if dist_sq > self.trust_radius * self.trust_radius {
                    let scale = self.trust_radius / dist_sq.sqrt();
                    for i in 0..n {
                        z_new[i] = x[i] + scale * (z_new[i] - x[i]);
                    }
                }
                z = z_new;
            }
            let model_at_z: f64 = cuts
                .iter()
                .map(|(xj, fj, gj)| {
                    fj + gj
                        .iter()
                        .zip(&z)
                        .zip(xj.iter())
                        .map(|((gi, zi), xji)| gi * (zi - xji))
                        .sum::<f64>()
                })
                .fold(f64::NEG_INFINITY, f64::max);
            let gap = best_f - model_at_z;
            if gap < self.tol {
                iters = k + 1;
                break;
            }
            x = z;
        }
        (best_x, best_f, iters)
    }
}
/// Proximal gradient method (ISTA/FISTA) for composite optimization.
///
/// Minimizes f(x) + g(x) where f is L-smooth (known Lipschitz constant)
/// and g has a cheap prox operator. Supports both ISTA (beta=0) and
/// FISTA (beta>0, Nesterov momentum).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProximalGradientSolver {
    /// Lipschitz constant L of gradient of f.
    pub lipschitz: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance on successive iterate change.
    pub tol: f64,
    /// Use FISTA acceleration (true) or plain ISTA (false).
    pub accelerated: bool,
}
impl ProximalGradientSolver {
    /// Create a proximal gradient solver.
    pub fn new(lipschitz: f64, max_iter: usize, tol: f64, accelerated: bool) -> Self {
        Self {
            lipschitz,
            max_iter,
            tol,
            accelerated,
        }
    }
    /// Minimize `smooth + regularizer`. Returns `(x_star, iters)`.
    pub fn minimize<F, G>(&self, smooth: &F, regularizer: &G, x0: &[f64]) -> (Vec<f64>, usize)
    where
        F: ConvexFunction,
        G: ProxableFunction,
    {
        let n = x0.len();
        let step = 1.0 / self.lipschitz;
        let mut x = x0.to_vec();
        let mut y = x.clone();
        let mut t = 1.0_f64;
        let mut iters = self.max_iter;
        for k in 1..=self.max_iter {
            let grad = smooth.gradient(&y);
            let v: Vec<f64> = y.iter().zip(&grad).map(|(yi, gi)| yi - step * gi).collect();
            let x_new = regularizer.prox(&v, step);
            let diff_norm: f64 = x_new
                .iter()
                .zip(&x)
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            if self.accelerated {
                let t_new = (1.0 + (1.0 + 4.0 * t * t).sqrt()) / 2.0;
                let beta = (t - 1.0) / t_new;
                let mut y_new = vec![0.0_f64; n];
                for i in 0..n {
                    y_new[i] = x_new[i] + beta * (x_new[i] - x[i]);
                }
                t = t_new;
                y = y_new;
            } else {
                y = x_new.clone();
            }
            x = x_new;
            if diff_norm < self.tol {
                iters = k;
                break;
            }
        }
        (x, iters)
    }
    /// Estimate the Lipschitz constant via power iteration on the Hessian approximation.
    ///
    /// Returns an upper bound on L by computing `max ||grad f(x + eps * v) - grad f(x)||/eps`.
    pub fn estimate_lipschitz<F: ConvexFunction>(f: &F, x: &[f64], num_trials: usize) -> f64 {
        let eps = 1e-5_f64;
        let n = x.len();
        let grad0 = f.gradient(x);
        let mut max_l = 0.0_f64;
        for i in 0..num_trials.min(n) {
            let mut x_pert = x.to_vec();
            x_pert[i] += eps;
            let grad_pert = f.gradient(&x_pert);
            let diff_norm: f64 = grad_pert
                .iter()
                .zip(&grad0)
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            max_l = max_l.max(diff_norm / eps);
        }
        max_l.max(1e-8)
    }
}
/// FISTA: Fast Iterative Shrinkage-Thresholding Algorithm.
///
/// Minimizes f(x) + g(x) where f is smooth (L-Lipschitz gradient) and
/// g has a cheap proximal operator. Achieves O(1/k²) convergence.
#[derive(Debug, Clone)]
pub struct FISTASolver {
    /// Lipschitz constant L of ∇f.
    pub lipschitz: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance on successive iterates.
    pub tol: f64,
}
impl FISTASolver {
    /// Create a new FISTA solver.
    pub fn new(lipschitz: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            lipschitz,
            max_iter,
            tol,
        }
    }
    /// Run FISTA to minimize `smooth` + `regularizer`.
    ///
    /// Returns `(x_star, iterations)`.
    pub fn minimize<F, G>(&self, smooth: &F, regularizer: &G, x0: &[f64]) -> (Vec<f64>, usize)
    where
        F: ConvexFunction,
        G: ProxableFunction,
    {
        let n = x0.len();
        let step = 1.0 / self.lipschitz;
        let mut x = x0.to_vec();
        let mut y = x.clone();
        let mut t = 1.0_f64;
        let mut iters = self.max_iter;
        for k in 1..=self.max_iter {
            let grad = smooth.gradient(&y);
            let v: Vec<f64> = y.iter().zip(&grad).map(|(yi, gi)| yi - step * gi).collect();
            let x_new = regularizer.prox(&v, step);
            let t_new = (1.0 + (1.0 + 4.0 * t * t).sqrt()) / 2.0;
            let beta = (t - 1.0) / t_new;
            let mut diff_norm = 0.0_f64;
            let mut y_new = vec![0.0_f64; n];
            for i in 0..n {
                y_new[i] = x_new[i] + beta * (x_new[i] - x[i]);
                diff_norm += (x_new[i] - x[i]).powi(2);
            }
            x = x_new;
            y = y_new;
            t = t_new;
            if diff_norm.sqrt() < self.tol {
                iters = k;
                break;
            }
        }
        (x, iters)
    }
}
/// L1 norm penalty: f(x) = λ · ‖x‖₁.
#[derive(Debug, Clone)]
pub struct L1NormFunction {
    /// Regularization weight λ ≥ 0.
    pub lambda: f64,
}
impl L1NormFunction {
    /// Create a new L1 penalty with weight `lambda`.
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }
}
/// Sinkhorn algorithm for entropic-regularized optimal transport.
///
/// Computes the approximate optimal transport plan between discrete measures
/// mu (source) and nu (target) with cost matrix C, using entropy regularization:
///   min_{gamma >= 0, gamma 1 = mu, gamma^T 1 = nu} <C, gamma> - eps * H(gamma)
///
/// The Sinkhorn iterations alternate between row and column normalization.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SinkhornSolver {
    /// Entropy regularization parameter epsilon > 0.
    pub epsilon: f64,
    /// Maximum number of Sinkhorn iterations.
    pub max_iter: usize,
    /// Convergence tolerance on marginal constraint violation.
    pub tol: f64,
}
impl SinkhornSolver {
    /// Create a new Sinkhorn solver.
    pub fn new(epsilon: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            epsilon,
            max_iter,
            tol,
        }
    }
    /// Compute the log-domain Gibbs kernel: K_{ij} = exp(-C_{ij} / epsilon).
    fn gibbs_kernel(cost: &[Vec<f64>], epsilon: f64) -> Vec<Vec<f64>> {
        cost.iter()
            .map(|row| row.iter().map(|&c| (-c / epsilon).exp()).collect())
            .collect()
    }
    /// Solve the entropic OT problem.
    ///
    /// `mu`: source distribution (sums to 1, length m).
    /// `nu`: target distribution (sums to 1, length n).
    /// `cost`: m x n cost matrix.
    ///
    /// Returns `(transport_plan, wasserstein_cost)`.
    pub fn solve(&self, mu: &[f64], nu: &[f64], cost: &[Vec<f64>]) -> (Vec<Vec<f64>>, f64) {
        let m = mu.len();
        let n = nu.len();
        let k = Self::gibbs_kernel(cost, self.epsilon);
        let mut u = vec![1.0_f64; m];
        let mut v = vec![1.0_f64; n];
        for _ in 0..self.max_iter {
            let kv: Vec<f64> = (0..m)
                .map(|i| k[i].iter().zip(&v).map(|(kij, vj)| kij * vj).sum::<f64>())
                .collect();
            let u_new: Vec<f64> = mu
                .iter()
                .zip(&kv)
                .map(|(mi, kvi)| mi / kvi.max(1e-300))
                .collect();
            let kt_u: Vec<f64> = (0..n)
                .map(|j| k.iter().zip(&u_new).map(|(ki, ui)| ki[j] * ui).sum::<f64>())
                .collect();
            let v_new: Vec<f64> = nu
                .iter()
                .zip(&kt_u)
                .map(|(nj, ktuj)| nj / ktuj.max(1e-300))
                .collect();
            let err: f64 = u_new
                .iter()
                .zip(&u)
                .map(|(a, b)| (a - b).abs())
                .sum::<f64>()
                + v_new
                    .iter()
                    .zip(&v)
                    .map(|(a, b)| (a - b).abs())
                    .sum::<f64>();
            u = u_new;
            v = v_new;
            if err < self.tol {
                break;
            }
        }
        let mut gamma = vec![vec![0.0_f64; n]; m];
        let mut w_cost = 0.0_f64;
        for i in 0..m {
            for j in 0..n {
                gamma[i][j] = u[i] * k[i][j] * v[j];
                w_cost += gamma[i][j] * cost[i][j];
            }
        }
        (gamma, w_cost)
    }
    /// Compute the squared 2-Wasserstein distance approximation between two 1D measures.
    ///
    /// `points_mu`: support points of mu (sorted).
    /// `weights_mu`: weights of mu (sums to 1).
    /// `points_nu`: support points of nu (sorted).
    /// `weights_nu`: weights of nu (sums to 1).
    pub fn wasserstein2_1d(
        points_mu: &[f64],
        weights_mu: &[f64],
        points_nu: &[f64],
        weights_nu: &[f64],
    ) -> f64 {
        let m = points_mu.len();
        let n = points_nu.len();
        let cost: Vec<Vec<f64>> = (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| (points_mu[i] - points_nu[j]).powi(2))
                    .collect()
            })
            .collect();
        let solver = Self::new(0.01, 200, 1e-8);
        let (_, w2) = solver.solve(weights_mu, weights_nu, &cost);
        w2
    }
}
/// Geometric program solver via convex transformation.
///
/// A GP in standard form: minimize p_0(x) subject to p_i(x) ≤ 1, i=1..m,
/// where each p_i is a posynomial. Under the change of variables x = exp(y),
/// the GP becomes convex (log-sum-exp minimization).
///
/// This solver applies gradient descent on the log-domain objective.
#[derive(Debug, Clone)]
pub struct GeometricProgramSolver {
    /// Maximum iterations for the inner gradient descent.
    pub max_iter: usize,
    /// Step size for gradient descent in log-domain.
    pub step_size: f64,
    /// Convergence tolerance.
    pub tol: f64,
}
impl GeometricProgramSolver {
    /// Create a new GP solver.
    pub fn new(max_iter: usize, step_size: f64, tol: f64) -> Self {
        Self {
            max_iter,
            step_size,
            tol,
        }
    }
    /// Evaluate a monomial c · ∏ x_i^{a_i} at log-domain point y (x = exp(y)).
    /// Returns log(c) + a^T y.
    pub fn eval_log_monomial(log_c: f64, exponents: &[f64], y: &[f64]) -> f64 {
        let dot: f64 = exponents.iter().zip(y).map(|(ai, yi)| ai * yi).sum();
        log_c + dot
    }
    /// Evaluate log of a posynomial: log(∑_k exp(log_c_k + a_k^T y)).
    pub fn log_sum_exp_posynomial(monomials: &[(f64, Vec<f64>)], y: &[f64]) -> f64 {
        let vals: Vec<f64> = monomials
            .iter()
            .map(|(lc, exp)| Self::eval_log_monomial(*lc, exp, y))
            .collect();
        let max_val = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        if max_val.is_infinite() {
            return max_val;
        }
        max_val + vals.iter().map(|v| (v - max_val).exp()).sum::<f64>().ln()
    }
    /// Solve the GP: minimize objective posynomial subject to constraint posynomials ≤ 1.
    ///
    /// `objective`: list of (log_coefficient, exponent_vector) pairs for objective posynomial.
    /// `constraints`: list of posynomials, each a list of (log_c, exponents) pairs.
    ///
    /// Returns the optimal y = log(x) and the optimal objective value.
    pub fn solve(
        &self,
        objective: &[(f64, Vec<f64>)],
        constraints: &[Vec<(f64, Vec<f64>)>],
        y0: &[f64],
    ) -> (Vec<f64>, f64) {
        let n = y0.len();
        let mut y = y0.to_vec();
        for _ in 0..self.max_iter {
            let obj_vals: Vec<f64> = objective
                .iter()
                .map(|(lc, exp)| Self::eval_log_monomial(*lc, exp, &y))
                .collect();
            let max_v = obj_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let weights: Vec<f64> = obj_vals.iter().map(|v| (v - max_v).exp()).collect();
            let w_sum: f64 = weights.iter().sum();
            let mut grad = vec![0.0_f64; n];
            for (k, (_, exp)) in objective.iter().enumerate() {
                let wk = weights[k] / w_sum;
                for i in 0..n {
                    grad[i] += wk * exp[i];
                }
            }
            for constraint in constraints {
                let c_lse = Self::log_sum_exp_posynomial(constraint, &y);
                if c_lse > 0.0 {
                    let rho = 10.0_f64;
                    let c_vals: Vec<f64> = constraint
                        .iter()
                        .map(|(lc, exp)| Self::eval_log_monomial(*lc, exp, &y))
                        .collect();
                    let c_max = c_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let c_weights: Vec<f64> = c_vals.iter().map(|v| (v - c_max).exp()).collect();
                    let c_wsum: f64 = c_weights.iter().sum();
                    for (k, (_, exp)) in constraint.iter().enumerate() {
                        let wk = c_weights[k] / c_wsum;
                        for i in 0..n {
                            grad[i] += rho * wk * exp[i];
                        }
                    }
                }
            }
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tol {
                break;
            }
            for i in 0..n {
                y[i] -= self.step_size * grad[i];
            }
        }
        let obj_val = Self::log_sum_exp_posynomial(objective, &y).exp();
        (y, obj_val)
    }
}
/// Follow-The-Regularized-Leader (FTRL) online convex optimizer.
///
/// At each round t, plays x_t = argmin_{x in X} { sum_{s<t} f_s(x) + R(x) / eta }
/// where R(x) is a strongly convex regularizer. With R(x) = (1/2)||x||^2 this
/// recovers Online Gradient Descent. Achieves O(sqrt(T)) regret.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OnlineLearner {
    /// Learning rate eta > 0.
    pub eta: f64,
    /// Dimension of the decision space.
    pub dim: usize,
    /// Accumulated gradient sum (cumulative subgradients).
    pub cumulative_grad: Vec<f64>,
    /// Round counter.
    pub round: usize,
    /// Total regret accumulated (sum of f_t(x_t) - f_t(x*) approximation).
    pub cumulative_loss: f64,
}
impl OnlineLearner {
    /// Create a new FTRL online learner.
    pub fn new(eta: f64, dim: usize) -> Self {
        Self {
            eta,
            dim,
            cumulative_grad: vec![0.0_f64; dim],
            round: 0,
            cumulative_loss: 0.0,
        }
    }
    /// Get the current decision x_t (FTRL update with L2 regularizer).
    ///
    /// FTRL with L2: x_t = -eta * sum_{s<t} g_s.
    pub fn current_decision(&self) -> Vec<f64> {
        self.cumulative_grad.iter().map(|g| -self.eta * g).collect()
    }
    /// Receive a subgradient `g_t` for the current round's loss.
    ///
    /// Updates internal state and returns the loss suffered at x_t
    /// (approximated as g_t^T x_t, a linear approximation).
    pub fn update(&mut self, grad: &[f64]) -> f64 {
        let x_t = self.current_decision();
        let loss_t: f64 = grad.iter().zip(&x_t).map(|(gi, xi)| gi * xi).sum();
        for (cg, g) in self.cumulative_grad.iter_mut().zip(grad) {
            *cg += g;
        }
        self.cumulative_loss += loss_t;
        self.round += 1;
        loss_t
    }
    /// Compute the regret bound O(||x*|| * G * sqrt(T)) for gradient bound G.
    pub fn regret_bound(&self, optimal_norm: f64, grad_bound: f64) -> f64 {
        optimal_norm * grad_bound * (self.round as f64).sqrt() / self.eta
            + 0.5 * self.eta * grad_bound * grad_bound * self.round as f64
    }
    /// Reset the learner to initial state.
    pub fn reset(&mut self) {
        self.cumulative_grad = vec![0.0_f64; self.dim];
        self.round = 0;
        self.cumulative_loss = 0.0;
    }
}
/// Bundle method for nonsmooth convex optimization.
///
/// Maintains a bundle of subgradients and uses a stability center with
/// a proximal term to regularize the QP subproblem. Supports serious steps
/// (descent) and null steps (bundle update only).
#[derive(Debug, Clone)]
pub struct BundleMethodSolver {
    /// Proximal parameter μ > 0 (controls step aggressiveness).
    pub mu: f64,
    /// Descent tolerance m_L ∈ (0, 1) for serious step acceptance.
    pub m_l: f64,
    /// Maximum bundle size (older cuts are dropped when exceeded).
    pub max_bundle_size: usize,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
impl BundleMethodSolver {
    /// Create a new bundle method solver.
    pub fn new(mu: f64, m_l: f64, max_bundle_size: usize, max_iter: usize, tol: f64) -> Self {
        Self {
            mu,
            m_l,
            max_bundle_size,
            max_iter,
            tol,
        }
    }
    /// Solve min f(x) starting from x0. Returns (x_star, f(x_star), iterations).
    pub fn minimize<F: ConvexFunction>(&self, f: &F, x0: &[f64]) -> (Vec<f64>, f64, usize) {
        let n = x0.len();
        let mut xhat = x0.to_vec();
        let mut fhat = f.eval(&xhat);
        let mut bundle: Vec<(Vec<f64>, f64)> = Vec::new();
        let g0 = f.gradient(&xhat);
        bundle.push((g0, 0.0));
        let mut iters = self.max_iter;
        for k in 0..self.max_iter {
            let mut d = vec![0.0_f64; n];
            let step_d = 1.0 / (self.mu + 1.0);
            for _ in 0..500 {
                let active_val = bundle
                    .iter()
                    .map(|(gj, alphaj)| {
                        let dot: f64 = gj.iter().zip(&d).map(|(gi, di)| gi * di).sum();
                        -alphaj + dot
                    })
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                let (gj, _alphaj) = &bundle[active_val];
                let grad_d: Vec<f64> = gj
                    .iter()
                    .zip(&d)
                    .map(|(gi, di)| gi + self.mu * di)
                    .collect();
                let grad_norm: f64 = grad_d.iter().map(|g| g * g).sum::<f64>().sqrt();
                if grad_norm < 1e-10 {
                    break;
                }
                for i in 0..n {
                    d[i] -= step_d * grad_d[i];
                }
            }
            let x_cand: Vec<f64> = xhat.iter().zip(&d).map(|(xi, di)| xi + di).collect();
            let f_cand = f.eval(&x_cand);
            let g_cand = f.gradient(&x_cand);
            let delta = fhat - f_cand;
            let model_pred: f64 = bundle
                .iter()
                .map(|(gj, alphaj)| {
                    let dot: f64 = gj.iter().zip(&d).map(|(gi, di)| gi * di).sum();
                    -alphaj + dot
                })
                .fold(f64::NEG_INFINITY, f64::max);
            let model_decrease = -model_pred;
            if model_decrease.abs() < self.tol {
                iters = k + 1;
                break;
            }
            let alpha_new: f64 = fhat
                - f_cand
                - g_cand
                    .iter()
                    .zip(&d)
                    .map(|(gi, di)| gi * (-di))
                    .sum::<f64>();
            if model_decrease > 1e-12 && delta >= self.m_l * model_decrease {
                xhat = x_cand.clone();
                fhat = f_cand;
                bundle.clear();
                bundle.push((g_cand, 0.0_f64.max(alpha_new)));
            } else {
                bundle.push((g_cand, 0.0_f64.max(alpha_new)));
                if bundle.len() > self.max_bundle_size {
                    bundle.remove(0);
                }
            }
        }
        (xhat, fhat, iters)
    }
}
/// Projected gradient descent with box constraints lb ≤ x ≤ ub.
#[derive(Debug, Clone)]
pub struct ProjectedGradient {
    /// Fixed learning rate.
    pub learning_rate: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
    /// Lower bound per coordinate.
    pub lb: Vec<f64>,
    /// Upper bound per coordinate.
    pub ub: Vec<f64>,
}
impl ProjectedGradient {
    /// Create a new projected gradient optimizer.
    pub fn new(lr: f64, max_iter: usize, tol: f64, lb: Vec<f64>, ub: Vec<f64>) -> Self {
        Self {
            learning_rate: lr,
            max_iter,
            tol,
            lb,
            ub,
        }
    }
    /// Project `x` onto the box [lb, ub].
    pub fn project(&self, x: &[f64]) -> Vec<f64> {
        x.iter()
            .enumerate()
            .map(|(i, &xi)| xi.clamp(self.lb[i], self.ub[i]))
            .collect()
    }
    /// Minimize `f` starting from `x0` with box-constraint projection.
    ///
    /// Returns `(x_star, f(x_star))`.
    pub fn minimize<F: ConvexFunction>(&self, f: &F, x0: &[f64]) -> (Vec<f64>, f64) {
        let mut x = self.project(x0);
        for _ in 0..self.max_iter {
            let grad = f.gradient(&x);
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tol {
                break;
            }
            let x_new: Vec<f64> = x
                .iter()
                .zip(&grad)
                .map(|(xi, gi)| xi - self.learning_rate * gi)
                .collect();
            x = self.project(&x_new);
        }
        let fval = f.eval(&x);
        (x, fval)
    }
}
/// Quadratic objective: f(x) = 0.5 x^T Q x + c^T x + d.
#[derive(Debug, Clone)]
pub struct QuadraticFunction {
    /// Positive semidefinite coefficient matrix Q (n × n).
    pub coeffs: Vec<Vec<f64>>,
    /// Linear coefficient vector c (length n).
    pub linear: Vec<f64>,
    /// Constant term d.
    pub constant: f64,
}
impl QuadraticFunction {
    /// Create a new quadratic function with matrix `Q`, linear part `c`, constant `d`.
    pub fn new(q: Vec<Vec<f64>>, c: Vec<f64>, d: f64) -> Self {
        Self {
            coeffs: q,
            linear: c,
            constant: d,
        }
    }
}
/// Alternating Direction Method of Multipliers (consensus form).
#[derive(Debug, Clone)]
pub struct ADMM {
    /// Penalty parameter ρ > 0.
    pub rho: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
impl ADMM {
    /// Create an ADMM solver with penalty `rho`.
    pub fn new(rho: f64) -> Self {
        Self {
            rho,
            max_iter: 1000,
            tol: 1e-6,
        }
    }
    /// Solve the LASSO problem: minimize 0.5‖Ax − b‖² + λ‖x‖₁.
    ///
    /// This is a stub that returns the zero vector (placeholder).
    pub fn solve_lasso(&self, a: &[Vec<f64>], b: &[f64], lambda: f64) -> Vec<f64> {
        let _ = (a, b, lambda, self.rho, self.max_iter, self.tol);
        let n = a.first().map(|row| row.len()).unwrap_or(0);
        vec![0.0; n]
    }
}
/// SDP relaxation of a quadratic program.
///
/// Lifts QP: min x^T Q x + c^T x, s.t. Ax ≤ b, x ∈ {0,1}^n
/// to an SDP over the matrix variable X = x x^T, X ⪰ 0, diag(X) = x.
/// This is a structural stub; `solve` returns a placeholder bound.
#[derive(Debug, Clone)]
pub struct SDPRelaxation {
    /// Objective matrix Q (n × n, PSD).
    pub q: Vec<Vec<f64>>,
    /// Linear objective c (length n).
    pub c: Vec<f64>,
    /// Constraint matrix A (m × n).
    pub a_mat: Vec<Vec<f64>>,
    /// RHS vector b (length m).
    pub b_vec: Vec<f64>,
}
impl SDPRelaxation {
    /// Create an SDP relaxation instance.
    pub fn new(q: Vec<Vec<f64>>, c: Vec<f64>, a_mat: Vec<Vec<f64>>, b_vec: Vec<f64>) -> Self {
        Self { q, c, a_mat, b_vec }
    }
    /// Return the dimension n.
    pub fn dim(&self) -> usize {
        self.c.len()
    }
    /// Compute the SDP lower bound via the trace relaxation:
    /// bound ≤ min_{X ⪰ 0} tr(Q X) + c^T x.
    ///
    /// This stub evaluates the objective at x = 0 (trivial feasible point).
    pub fn solve(&self) -> f64 {
        let _ = (&self.q, &self.c, &self.a_mat, &self.b_vec);
        0.0
    }
    /// Check whether a given matrix (flattened row-major) is PSD using
    /// Sylvester's criterion (all leading principal minors ≥ 0).
    pub fn is_psd(mat: &[Vec<f64>]) -> bool {
        let n = mat.len();
        for size in 1..=n {
            let mut sub: Vec<Vec<f64>> = (0..size).map(|i| mat[i][..size].to_vec()).collect();
            let mut det = 1.0_f64;
            for col in 0..size {
                let pivot_row = (col..size).find(|&r| sub[r][col].abs() > 1e-12);
                let pr = match pivot_row {
                    Some(r) => r,
                    None => return false,
                };
                if pr != col {
                    sub.swap(col, pr);
                    det = -det;
                }
                det *= sub[col][col];
                let pv = sub[col][col];
                for r in (col + 1)..size {
                    let factor = sub[r][col] / pv;
                    for c in col..size {
                        let val = sub[col][c];
                        sub[r][c] -= factor * val;
                    }
                }
            }
            if det < -1e-9 {
                return false;
            }
        }
        true
    }
}
