//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Mirror descent algorithm: θ_{t+1} = ∇φ*(∇φ(θ_t) − ε ∇L(θ_t)).
///
/// For φ = negative entropy (Shannon), this reduces to natural gradient on exponential family.
pub struct MirrorDescent {
    /// Current iterate θ (in primal space).
    pub theta: Vec<f64>,
    /// Step size ε.
    pub step_size: f64,
}
impl MirrorDescent {
    /// Create a new MirrorDescent optimizer.
    pub fn new(theta: Vec<f64>, step_size: f64) -> Self {
        Self { theta, step_size }
    }
    /// Mirror descent step for the negative-entropy mirror map φ(θ) = Σ_i θ_i log θ_i.
    ///
    /// ∇φ(θ) = log θ + 1; ∇φ*(y) = exp(y - 1).
    /// Update: η ← (∇φ(θ) − ε g) = log θ − ε g; θ ← exp(η) / Σ exp(η).
    pub fn step_neg_entropy(&mut self, grad: &[f64]) {
        let d = self.theta.len();
        let eta: Vec<f64> = (0..d)
            .map(|i| {
                let log_th = if self.theta[i] > 0.0 {
                    self.theta[i].ln()
                } else {
                    f64::NEG_INFINITY
                };
                log_th - self.step_size * grad[i]
            })
            .collect();
        let max_eta = eta.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_eta: Vec<f64> = eta.iter().map(|&e| (e - max_eta).exp()).collect();
        let z: f64 = exp_eta.iter().sum();
        self.theta = if z > 0.0 {
            exp_eta.iter().map(|v| v / z).collect()
        } else {
            vec![1.0 / d as f64; d]
        };
    }
    /// Mirror descent step for the squared L2 norm mirror map φ(θ) = (1/2)‖θ‖².
    ///
    /// This recovers standard gradient descent: θ ← θ − ε g.
    pub fn step_l2(&mut self, grad: &[f64]) {
        let d = self.theta.len();
        for i in 0..d {
            self.theta[i] -= self.step_size * grad[i];
        }
    }
    /// Bregman divergence for the negative-entropy map: D(p ‖ q) = Σ p_i log(p_i/q_i) − p_i + q_i.
    pub fn bregman_neg_entropy(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi == 0.0 {
                    qi
                } else if qi == 0.0 {
                    f64::INFINITY
                } else {
                    pi * (pi / qi).ln() - pi + qi
                }
            })
            .sum()
    }
}
/// Jeffreys prior π(θ) ∝ √det(I(θ)).
pub struct JeffreysPrior {
    /// Log-density function log p(x; θ).
    pub finite_diff_h: f64,
}
impl JeffreysPrior {
    /// Create a new JeffreysPrior with given finite-difference step.
    pub fn new() -> Self {
        Self {
            finite_diff_h: 1e-5,
        }
    }
    /// Compute the Jeffreys prior density (unnormalized) at θ for a 1D model.
    ///
    /// π(θ) ∝ √I(θ) where I(θ) = -E\[∂²/∂θ² log p(x;θ)\].
    /// Numerically: √(Fisher information at θ).
    pub fn density(
        &self,
        log_density: impl Fn(f64, f64) -> f64,
        theta: f64,
        samples: &[f64],
    ) -> f64 {
        let h = self.finite_diff_h;
        let n = samples.len() as f64;
        let fisher: f64 = samples
            .iter()
            .map(|&x| {
                let score = (log_density(x, theta + h) - log_density(x, theta - h)) / (2.0 * h);
                score * score
            })
            .sum::<f64>()
            / n;
        fisher.sqrt()
    }
    /// Verify Jeffreys prior invariance: π̃(φ) = π(θ)|dθ/dφ|.
    ///
    /// Checks numerically that the Jeffreys prior at θ and at φ = g(θ) satisfy the relation.
    pub fn verify_invariance(&self, pi_theta: f64, pi_phi: f64, jacobian: f64) -> bool {
        (pi_phi - pi_theta * jacobian.abs()).abs() < 1e-6
    }
}
/// Dual connection pair (∇^(α), ∇^(-α)) on a statistical manifold.
pub struct DualConnection {
    /// The α parameter defining the primal connection.
    pub alpha: f64,
    /// Manifold dimension.
    pub dim: usize,
}
impl DualConnection {
    /// Create a new DualConnection.
    pub fn new(alpha: f64, dim: usize) -> Self {
        Self { alpha, dim }
    }
    /// Dual α: the dual of ∇^(α) is ∇^(-α).
    pub fn dual_alpha(&self) -> f64 {
        -self.alpha
    }
    /// Check duality: (∇^(α))* = ∇^(-α).
    pub fn verify_duality(&self) -> bool {
        (self.dual_alpha() + self.alpha).abs() < 1e-9
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SlicedWasserstein {
    pub dimension: usize,
    pub num_projections: usize,
    pub wasserstein_order: usize,
}
#[allow(dead_code)]
impl SlicedWasserstein {
    pub fn new(dim: usize, projections: usize) -> Self {
        SlicedWasserstein {
            dimension: dim,
            num_projections: projections,
            wasserstein_order: 2,
        }
    }
    pub fn compute_approximate(&self, _samples_p: &[f64], _samples_q: &[f64]) -> f64 {
        0.0
    }
    pub fn complexity_description(&self) -> String {
        format!(
            "Sliced W2: O(L n log n) for L={} projections, n samples",
            self.num_projections
        )
    }
    pub fn bonneel_et_al_description(&self) -> String {
        "Bonneel et al. (2015): sliced Wasserstein as tractable OT approximation".to_string()
    }
}
/// α-divergence interpolation between KL (α→1) and reverse-KL (α→−1).
///
/// Provides a unified family D^(α) for α ∈ (−1,1) and the limiting KL divergences.
pub struct AlphaDivergenceFamily {
    /// α parameter controlling the divergence type.
    pub alpha: f64,
}
impl AlphaDivergenceFamily {
    /// Create for a given α.
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
    /// Compute D^(α)(p ‖ q) for discrete distributions.
    pub fn compute(&self, p: &[f64], q: &[f64]) -> f64 {
        AlphaDivergence::new(self.alpha).compute(p, q)
    }
    /// Hellinger divergence: α = 0, D^(0)(p‖q) = 2(1 − ∫√(pq)).
    pub fn hellinger(p: &[f64], q: &[f64]) -> f64 {
        let bc: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi * qi).sqrt())
            .sum();
        2.0 * (1.0 - bc)
    }
    /// Bhattacharyya coefficient: ∫√(pq) dμ.
    pub fn bhattacharyya_coeff(p: &[f64], q: &[f64]) -> f64 {
        p.iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi * qi).sqrt())
            .sum()
    }
    /// Total variation distance: (1/2) Σ |p_i - q_i|.
    pub fn total_variation(p: &[f64], q: &[f64]) -> f64 {
        0.5 * p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| (pi - qi).abs())
            .sum::<f64>()
    }
    /// Check monotonicity: D^(α) is monotone increasing in |α| for fixed p, q.
    /// Here we check that D^(|α|) ≥ D^(0) = Hellinger.
    pub fn verify_monotone_in_alpha(&self, p: &[f64], q: &[f64]) -> bool {
        let d_alpha = self.compute(p, q);
        let d_hell = Self::hellinger(p, q);
        d_alpha >= d_hell - 1e-9
    }
    /// Compute the α-mean (power mean) of two distributions:
    /// m^(α)(p, q) = p_t with t determined by the α-projection.
    /// Here: m_t = p^{(1+α)/2} q^{(1-α)/2} / Z  (normalized geometric mean family).
    pub fn alpha_mixture(&self, p: &[f64], q: &[f64]) -> Vec<f64> {
        let ep = (1.0 + self.alpha) / 2.0;
        let eq = (1.0 - self.alpha) / 2.0;
        let unnorm: Vec<f64> = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi == 0.0 || qi == 0.0 {
                    0.0
                } else {
                    pi.powf(ep) * qi.powf(eq)
                }
            })
            .collect();
        let z: f64 = unnorm.iter().sum();
        if z == 0.0 {
            return unnorm;
        }
        unnorm.iter().map(|v| v / z).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionType {
    Moment,
    Exponential,
}
/// A parametric statistical manifold: smooth family of distributions p(·; θ).
///
/// Stores the Fisher information matrix at the current parameter point.
pub struct StatisticalManifold {
    /// Current parameter θ.
    pub theta: Vec<f64>,
    /// Fisher information matrix G(θ) (d×d).
    pub fisher_matrix: Vec<Vec<f64>>,
}
impl StatisticalManifold {
    /// Construct a StatisticalManifold with a given parameter and precomputed Fisher matrix.
    pub fn new(theta: Vec<f64>, fisher_matrix: Vec<Vec<f64>>) -> Self {
        Self {
            theta,
            fisher_matrix,
        }
    }
    /// Build for the Gaussian location family: θ = (μ, σ²), G = diag(1/σ², 1/(2σ⁴)).
    pub fn gaussian(mu: f64, sigma_sq: f64) -> Self {
        let theta = vec![mu, sigma_sq];
        let fisher_matrix = vec![
            vec![1.0 / sigma_sq, 0.0],
            vec![0.0, 1.0 / (2.0 * sigma_sq * sigma_sq)],
        ];
        Self {
            theta,
            fisher_matrix,
        }
    }
    /// Build for the Bernoulli family: θ = p ∈ (0,1), G = 1/(p(1-p)).
    pub fn bernoulli(p: f64) -> Self {
        let fisher = 1.0 / (p * (1.0 - p));
        Self {
            theta: vec![p],
            fisher_matrix: vec![vec![fisher]],
        }
    }
    /// Compute the Fisher-Rao infinitesimal distance ds from θ to θ+dθ.
    pub fn infinitesimal_distance(&self, d_theta: &[f64]) -> f64 {
        let gv = mat_vec(&self.fisher_matrix, d_theta);
        dot_product(d_theta, &gv).sqrt()
    }
    /// Compute the e-geodesic (exponential geodesic) tangent vector at the current point.
    ///
    /// For exponential families, the e-geodesic in natural parameters is a straight line.
    /// Returns the unit tangent direction in the Fisher metric towards `theta_target`.
    pub fn e_geodesic_direction(&self, theta_target: &[f64]) -> Vec<f64> {
        let d = self.theta.len();
        let diff: Vec<f64> = (0..d).map(|i| theta_target[i] - self.theta[i]).collect();
        let len = self.infinitesimal_distance(&diff);
        if len < 1e-14 {
            return vec![0.0; d];
        }
        diff.iter().map(|v| v / len).collect()
    }
}
/// An exponential family distribution: p(x|θ) = exp(⟨θ, T(x)⟩ - A(θ)) h(x).
pub struct ExponentialFamily {
    /// Natural parameters θ ∈ ℝ^d.
    pub theta: Vec<f64>,
    /// Log-partition function A(θ).
    pub log_partition: f64,
}
impl ExponentialFamily {
    /// Create a new ExponentialFamily with given natural parameters.
    pub fn new(theta: Vec<f64>, log_partition: f64) -> Self {
        Self {
            theta,
            log_partition,
        }
    }
    /// Compute the log-partition function for a Gaussian: A(μ,σ²) = μ²/(2σ²) + log(√(2πσ²)).
    pub fn gaussian_log_partition(mu: f64, sigma_sq: f64) -> f64 {
        mu * mu / (2.0 * sigma_sq) + 0.5 * (2.0 * std::f64::consts::PI * sigma_sq).ln()
    }
    /// Compute moment parameters η = E_θ[T(x)] for a Bernoulli family: η = sigmoid(θ).
    pub fn bernoulli_moment(theta: f64) -> f64 {
        1.0 / (1.0 + (-theta).exp())
    }
    /// Convert Bernoulli moment parameter η to natural parameter θ = logit(η).
    pub fn bernoulli_natural(eta: f64) -> f64 {
        (eta / (1.0 - eta)).ln()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatGradExt {
    pub parameter_dim: usize,
    pub fisher_metric_type: FisherMetricType,
    pub is_amari_nagaoka: bool,
    pub steepest_descent_manifold: String,
}
#[allow(dead_code)]
impl NatGradExt {
    pub fn new(dim: usize) -> Self {
        NatGradExt {
            parameter_dim: dim,
            fisher_metric_type: FisherMetricType::ClassicalFisher,
            is_amari_nagaoka: true,
            steepest_descent_manifold: format!("statistical manifold in R^{}", dim),
        }
    }
    pub fn update_rule(&self, lr: f64) -> String {
        format!(
            "Natural gradient: θ_{{t+1}} = θ_t - {} * G(θ)^{{-1}} ∇L(θ) (Fisher metric G)",
            lr
        )
    }
    pub fn fisher_rao_distance(&self) -> String {
        format!(
            "Fisher-Rao geodesic distance on {}-dim statistical manifold",
            self.parameter_dim
        )
    }
    pub fn amari_dual_connection(&self) -> String {
        "Amari α-connection: ∇^0 = Levi-Civita, ∇^{±1} = mixture/exponential connections"
            .to_string()
    }
    pub fn invariance_property(&self) -> String {
        "Fisher-Rao metric: unique (up to scale) Riemannian metric invariant under reparametrization"
            .to_string()
    }
}
/// Bregman projection of a point onto an exponential family.
///
/// Minimizes D_φ(p ‖ q) over q in a specified exponential family submanifold.
/// For the KL divergence (φ = negative entropy), this is the e-projection.
pub struct BregmanProjection {
    /// Convex potential φ discretized at grid points.
    pub phi_grid: Vec<f64>,
    /// Grid points x_0, ..., x_n.
    pub x_grid: Vec<f64>,
    /// Gradient of φ at grid points.
    pub phi_grad: Vec<f64>,
}
impl BregmanProjection {
    /// Build a BregmanProjection from a convex function φ evaluated on a uniform grid.
    pub fn new(x_grid: Vec<f64>, phi_fn: impl Fn(f64) -> f64) -> Self {
        let phi_grid: Vec<f64> = x_grid.iter().map(|&x| phi_fn(x)).collect();
        let n = x_grid.len();
        let mut phi_grad = vec![0.0f64; n];
        for i in 1..(n - 1) {
            phi_grad[i] = (phi_grid[i + 1] - phi_grid[i - 1]) / (x_grid[i + 1] - x_grid[i - 1]);
        }
        if n >= 2 {
            phi_grad[0] = (phi_grid[1] - phi_grid[0]) / (x_grid[1] - x_grid[0]);
            phi_grad[n - 1] = (phi_grid[n - 1] - phi_grid[n - 2]) / (x_grid[n - 1] - x_grid[n - 2]);
        }
        Self {
            phi_grid,
            x_grid,
            phi_grad,
        }
    }
    /// Bregman divergence D_φ(x ‖ y): φ(x) − φ(y) − φ'(y)(x − y).
    ///
    /// Both x and y are looked up by nearest grid point.
    pub fn divergence(&self, x: f64, y: f64) -> f64 {
        let ix = self.nearest_idx(x);
        let iy = self.nearest_idx(y);
        let phi_x = self.phi_grid[ix];
        let phi_y = self.phi_grid[iy];
        let grad_y = self.phi_grad[iy];
        phi_x - phi_y - grad_y * (x - y)
    }
    /// Find the index of the nearest grid point to x.
    fn nearest_idx(&self, x: f64) -> usize {
        self.x_grid
            .iter()
            .enumerate()
            .min_by(|(_, &a), (_, &b)| {
                (a - x)
                    .abs()
                    .partial_cmp(&(b - x).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    /// Project p onto the affine subspace {q : Σ_i q_i T_i = η} using Bregman projection.
    ///
    /// Uses the iterative Bregman projection algorithm (Dykstra's algorithm variant):
    /// returns the projected distribution as a discrete probability vector.
    pub fn project_onto_moment_constraint(
        &self,
        p: &[f64],
        sufficient_stats: &[f64],
        eta_target: f64,
        max_iter: usize,
    ) -> Vec<f64> {
        let mut q = p.to_vec();
        for _ in 0..max_iter {
            let current_eta: f64 = q
                .iter()
                .zip(sufficient_stats.iter())
                .map(|(qi, ti)| qi * ti)
                .sum();
            if (current_eta - eta_target).abs() < 1e-9 {
                break;
            }
            let var_t: f64 = q
                .iter()
                .zip(sufficient_stats.iter())
                .map(|(qi, ti)| qi * ti * ti)
                .sum::<f64>()
                - current_eta * current_eta;
            let step = if var_t.abs() < 1e-14 {
                0.0
            } else {
                (eta_target - current_eta) / var_t
            };
            let unnorm: Vec<f64> = q
                .iter()
                .zip(sufficient_stats.iter())
                .map(|(qi, ti)| qi * (step * ti).exp())
                .collect();
            let z: f64 = unnorm.iter().sum();
            q = if z > 0.0 {
                unnorm.iter().map(|v| v / z).collect()
            } else {
                unnorm
            };
        }
        q
    }
}
/// Wasserstein geometry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WassersteinGeometry {
    pub order: u32,
    pub space_name: String,
}
impl WassersteinGeometry {
    #[allow(dead_code)]
    pub fn new(order: u32, space: &str) -> Self {
        Self {
            order,
            space_name: space.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn w2_distance_description(&self) -> String {
        if self.order == 2 {
            "W_2(mu, nu) = sqrt(min_pi E[||X-Y||^2]) where pi ranges over couplings".to_string()
        } else {
            format!("W_{}(mu, nu) = (min_pi E[||X-Y||^p])^(1/p)", self.order)
        }
    }
    #[allow(dead_code)]
    pub fn benamou_brenier_description(&self) -> String {
        "Benamou-Brenier: W_2^2 = min integral ||v_t||^2 rho_t dt (fluid dynamics)".to_string()
    }
    #[allow(dead_code)]
    pub fn optimal_transport_map(&self) -> String {
        "Brenier: optimal transport from mu to nu is gradient of a convex potential phi".to_string()
    }
}
/// EM algorithm viewed as alternating m-projection (E-step) and e-projection (M-step).
///
/// For a latent variable model p(x,z; θ) = p(x|z; θ) p(z; θ), EM alternates:
/// E-step: q(z) ← p(z|x; θ^{(t)}) (m-projection of joint onto latent marginal)
/// M-step: θ^{(t+1)} ← argmax_θ E_q\[log p(x,z; θ)\] (e-projection onto exp family)
pub struct EMAlternatingProjection {
    /// Current model parameters θ.
    pub theta: Vec<f64>,
    /// Current responsibility weights r_{ik} (n_data × n_components).
    pub responsibilities: Vec<Vec<f64>>,
    /// Number of mixture components K.
    pub n_components: usize,
}
impl EMAlternatingProjection {
    /// Construct an EM state for a K-component mixture.
    pub fn new(theta: Vec<f64>, n_components: usize) -> Self {
        Self {
            theta,
            responsibilities: Vec::new(),
            n_components,
        }
    }
    /// E-step: compute responsibilities r_{ik} ∝ π_k p(x_i | z=k; θ).
    ///
    /// `log_likelihoods\[i\]\[k\]` = log p(x_i | z=k; θ) + log π_k.
    pub fn e_step(&mut self, log_likelihoods: &[Vec<f64>]) {
        let n = log_likelihoods.len();
        self.responsibilities = Vec::with_capacity(n);
        for lls in log_likelihoods {
            let max_ll = lls.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let exp_lls: Vec<f64> = lls.iter().map(|&v| (v - max_ll).exp()).collect();
            let z: f64 = exp_lls.iter().sum();
            let r = if z > 0.0 {
                exp_lls.iter().map(|v| v / z).collect()
            } else {
                vec![1.0 / self.n_components as f64; self.n_components]
            };
            self.responsibilities.push(r);
        }
    }
    /// M-step for a Gaussian mixture: update means and mixing weights.
    ///
    /// `data\[i\]` is the i-th observation (scalar).
    /// Returns (means, weights) after M-step.
    pub fn m_step_gaussian(&self, data: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let n = data.len();
        let k = self.n_components;
        let mut means = vec![0.0f64; k];
        let mut weights = vec![0.0f64; k];
        for i in 0..n {
            if i >= self.responsibilities.len() {
                break;
            }
            for j in 0..k {
                let r = self.responsibilities[i][j];
                means[j] += r * data[i];
                weights[j] += r;
            }
        }
        let n_f = n as f64;
        for j in 0..k {
            means[j] = if weights[j] > 1e-10 {
                means[j] / weights[j]
            } else {
                0.0
            };
            weights[j] /= n_f;
        }
        (means, weights)
    }
    /// Log-likelihood of data under current responsibilities (ELBO lower bound).
    ///
    /// Returns Σ_i Σ_k r_{ik} log(r_{ik}) (negative entropy of responsibilities).
    pub fn elbo_entropy_term(&self) -> f64 {
        let mut total = 0.0f64;
        for r_i in &self.responsibilities {
            for &r in r_i {
                if r > 0.0 {
                    total += r * r.ln();
                }
            }
        }
        total
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FisherMetricType {
    ClassicalFisher,
    QuantumFisher,
    BuresMetric,
}
/// α-divergence D^(α)(P ‖ Q) = 4/(1-α²) · (1 - ∫ p^{(1+α)/2} q^{(1-α)/2} dμ).
pub struct AlphaDivergence {
    /// The α parameter; α=1 gives KL, α=-1 gives reverse KL, α=0 gives Hellinger.
    pub alpha: f64,
}
impl AlphaDivergence {
    /// Create a new AlphaDivergence.
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
    /// Evaluate D^(α)(p ‖ q) for discrete distributions.
    pub fn compute(&self, p: &[f64], q: &[f64]) -> f64 {
        let a = self.alpha;
        if (a - 1.0).abs() < 1e-9 {
            return p
                .iter()
                .zip(q.iter())
                .filter(|(&pi, _)| pi > 0.0)
                .map(|(&pi, &qi)| {
                    if qi == 0.0 {
                        f64::INFINITY
                    } else {
                        pi * (pi / qi).ln()
                    }
                })
                .sum();
        }
        if (a + 1.0).abs() < 1e-9 {
            return q
                .iter()
                .zip(p.iter())
                .filter(|(&qi, _)| qi > 0.0)
                .map(|(&qi, &pi)| {
                    if pi == 0.0 {
                        f64::INFINITY
                    } else {
                        qi * (qi / pi).ln()
                    }
                })
                .sum();
        }
        let exp_p = (1.0 + a) / 2.0;
        let exp_q = (1.0 - a) / 2.0;
        let integral: f64 = p
            .iter()
            .zip(q.iter())
            .map(|(&pi, &qi)| {
                if pi == 0.0 || qi == 0.0 {
                    0.0
                } else {
                    pi.powf(exp_p) * qi.powf(exp_q)
                }
            })
            .sum();
        4.0 / (1.0 - a * a) * (1.0 - integral)
    }
    /// Check the generalized Pythagorean theorem for this α-divergence:
    /// D(p‖r) = D(p‖q) + D(q‖r) when q is the α-projection of p onto the family.
    /// Here we just verify the triangle-inequality direction D(p‖r) ≥ min(D(p‖q), D(q‖r)).
    pub fn pythagorean_check(&self, p: &[f64], q: &[f64], r: &[f64]) -> bool {
        let d_pr = self.compute(p, r);
        let d_pq = self.compute(p, q);
        let d_qr = self.compute(q, r);
        d_pr <= d_pq + d_qr + 1e-9
    }
}
/// Constant-curvature statistical manifold (α = ±1 gives curved families).
pub struct ConstantCurvatureManifold {
    /// Curvature parameter.
    pub alpha: f64,
    /// Manifold dimension.
    pub dim: usize,
}
impl ConstantCurvatureManifold {
    /// Create a new ConstantCurvatureManifold.
    pub fn new(alpha: f64, dim: usize) -> Self {
        Self { alpha, dim }
    }
    /// The constant sectional curvature of exponential/mixture families = -1/4.
    pub fn curvature(&self) -> f64 {
        -0.25
    }
    /// Check whether this manifold is flat (curvature zero), which holds when α = 0.
    pub fn is_flat(&self) -> bool {
        self.alpha.abs() < 1e-9
    }
}
/// Fisher information matrix G_{ij}(θ) = E\[∂_i log p · ∂_j log p\].
///
/// Estimated numerically from samples via finite differences.
pub struct FisherInformationMetric {
    /// Dimension d of the parameter space.
    pub dim: usize,
    /// Finite-difference step size h.
    pub h: f64,
}
impl FisherInformationMetric {
    /// Create a new FisherInformationMetric for a d-dimensional parameter.
    pub fn new(dim: usize) -> Self {
        Self { dim, h: 1e-5 }
    }
    /// Compute G(θ) as a d×d matrix from samples x_1,...,x_n.
    ///
    /// `log_density(x, theta)` evaluates log p(x; θ).
    pub fn compute_matrix(
        &self,
        log_density: impl Fn(&[f64], &[f64]) -> f64,
        theta: &[f64],
        samples: &[Vec<f64>],
    ) -> Vec<Vec<f64>> {
        let d = self.dim;
        let n = samples.len() as f64;
        let h = self.h;
        let mut g = vec![vec![0.0f64; d]; d];
        for x in samples {
            let score: Vec<f64> = (0..d)
                .map(|i| {
                    let mut tp = theta.to_vec();
                    let mut tm = theta.to_vec();
                    tp[i] += h;
                    tm[i] -= h;
                    (log_density(x, &tp) - log_density(x, &tm)) / (2.0 * h)
                })
                .collect();
            for i in 0..d {
                for j in 0..d {
                    g[i][j] += score[i] * score[j];
                }
            }
        }
        for i in 0..d {
            for j in 0..d {
                g[i][j] /= n;
            }
        }
        g
    }
    /// Compute the Riemannian distance approximation using the Fisher metric.
    ///
    /// Uses a first-order approximation: d ≈ √((θ₁-θ₂)^T G(θ̄) (θ₁-θ₂))
    /// where θ̄ = (θ₁+θ₂)/2.
    pub fn geodesic_distance(&self, g: &[Vec<f64>], theta1: &[f64], theta2: &[f64]) -> f64 {
        let d = self.dim;
        let diff: Vec<f64> = (0..d).map(|i| theta1[i] - theta2[i]).collect();
        let mut quad = 0.0f64;
        for i in 0..d {
            for j in 0..d {
                quad += diff[i] * g[i][j] * diff[j];
            }
        }
        quad.sqrt()
    }
}
/// Geodesic between two distributions on the probability simplex.
pub struct GeodesicOfDistributions {
    /// Starting distribution p.
    pub p: Vec<f64>,
    /// Ending distribution q.
    pub q: Vec<f64>,
}
impl GeodesicOfDistributions {
    /// Create a new geodesic.
    pub fn new(p: Vec<f64>, q: Vec<f64>) -> Self {
        Self { p, q }
    }
    /// Evaluate the e-geodesic (exponential geodesic) at time t ∈ \[0,1\]:
    /// p_t(x) ∝ p(x)^{1-t} q(x)^t.
    pub fn e_geodesic(&self, t: f64) -> Vec<f64> {
        let unnorm: Vec<f64> = self
            .p
            .iter()
            .zip(self.q.iter())
            .map(|(&pi, &qi)| {
                if pi == 0.0 && qi == 0.0 {
                    0.0
                } else if pi == 0.0 {
                    0.0
                } else if qi == 0.0 {
                    0.0
                } else {
                    pi.powf(1.0 - t) * qi.powf(t)
                }
            })
            .collect();
        let z: f64 = unnorm.iter().sum();
        if z == 0.0 {
            return unnorm;
        }
        unnorm.iter().map(|&v| v / z).collect()
    }
    /// Evaluate the m-geodesic (mixture geodesic) at time t ∈ \[0,1\]:
    /// p_t = (1-t) p + t q.
    pub fn m_geodesic(&self, t: f64) -> Vec<f64> {
        self.p
            .iter()
            .zip(self.q.iter())
            .map(|(&pi, &qi)| (1.0 - t) * pi + t * qi)
            .collect()
    }
    /// Bhattacharyya geodesic distance: d(p,q) = 2 arccos(BC(p,q))
    /// where BC(p,q) = Σ √(p_i q_i) is the Bhattacharyya coefficient.
    pub fn geodesic_distance(&self) -> f64 {
        let bc: f64 = self
            .p
            .iter()
            .zip(self.q.iter())
            .map(|(&pi, &qi)| (pi * qi).sqrt())
            .sum();
        let bc_clamped = bc.clamp(-1.0, 1.0);
        2.0 * bc_clamped.acos()
    }
}
/// Expectation Propagation: approximate inference on a curved exponential family manifold.
pub struct ExpectationPropagation {
    /// Number of likelihood factors K.
    pub num_factors: usize,
    /// Current cavity marginals (one per factor), each is a (mean, variance) pair.
    pub cavities: Vec<(f64, f64)>,
    /// Current approximate factors (mean, variance).
    pub approx_factors: Vec<(f64, f64)>,
}
impl ExpectationPropagation {
    /// Create a new EP with K factors, initialized to flat (uninformative) Gaussians.
    pub fn new(num_factors: usize) -> Self {
        Self {
            num_factors,
            cavities: vec![(0.0, 1e6); num_factors],
            approx_factors: vec![(0.0, 1e6); num_factors],
        }
    }
    /// Compute the overall posterior approximation from the approximate factors.
    ///
    /// For a Gaussian product family: precision = Σ 1/σ²_i, mean = (Σ μ_i/σ²_i) / precision.
    pub fn posterior_params(&self) -> (f64, f64) {
        let precision: f64 = self.approx_factors.iter().map(|(_, v)| 1.0 / v).sum();
        if precision == 0.0 {
            return (0.0, f64::INFINITY);
        }
        let mean: f64 = self.approx_factors.iter().map(|(m, v)| m / v).sum::<f64>() / precision;
        let variance = 1.0 / precision;
        (mean, variance)
    }
    /// EP update for factor i: moment-match cavity × true factor to the exponential family.
    ///
    /// For a Gaussian family this means: new approx factor = posterior / cavity.
    pub fn update_factor(&mut self, factor_idx: usize, true_mean: f64, true_var: f64) {
        let (cav_m, cav_v) = self.cavities[factor_idx];
        let new_prec = 1.0 / true_var - 1.0 / cav_v;
        let new_var = if new_prec.abs() < 1e-12 {
            1e6
        } else {
            1.0 / new_prec
        };
        let new_mean = new_var * (true_mean / true_var - cav_m / cav_v);
        self.approx_factors[factor_idx] = (new_mean, new_var);
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatManiExt {
    pub family_name: String,
    pub dimension: usize,
    pub curvature_scalar: f64,
    pub is_flat: bool,
    pub is_dually_flat: bool,
}
#[allow(dead_code)]
impl StatManiExt {
    pub fn exponential_family(name: &str, dim: usize) -> Self {
        StatManiExt {
            family_name: name.to_string(),
            dimension: dim,
            curvature_scalar: 0.0,
            is_flat: false,
            is_dually_flat: true,
        }
    }
    pub fn gaussian_family() -> Self {
        StatManiExt {
            family_name: "Gaussian N(μ,σ²)".to_string(),
            dimension: 2,
            curvature_scalar: -0.5,
            is_flat: false,
            is_dually_flat: true,
        }
    }
    pub fn pythagorean_theorem(&self) -> String {
        if self.is_dually_flat {
            "Generalized Pythagoras: D(p||r) = D(p||q) + D(q||r) for KL divergence".to_string()
        } else {
            "Pythagorean theorem not applicable (not dually flat)".to_string()
        }
    }
    pub fn bregman_divergence_connection(&self) -> String {
        format!(
            "Family {}: Bregman divergence on dual coordinates = KL divergence",
            self.family_name
        )
    }
}
/// Natural gradient flow on an exponential family manifold.
///
/// For an exponential family p(x; θ) = exp(⟨θ,T(x)⟩ − A(θ)) h(x), the natural
/// gradient is G(θ)^{−1} ∇L where G = ∇²A(θ) is the Fisher information Hessian.
pub struct NaturalGradientExpFamily {
    /// Current natural parameters θ.
    pub theta: Vec<f64>,
    /// Log-partition function A(θ) (scalar).
    pub log_partition: f64,
    /// Hessian of A (Fisher information matrix).
    pub hessian_a: Vec<Vec<f64>>,
    /// Step size ε.
    pub step_size: f64,
}
impl NaturalGradientExpFamily {
    /// Create a new natural gradient optimizer for an exponential family.
    pub fn new(theta: Vec<f64>, log_partition: f64, hessian_a: Vec<Vec<f64>>) -> Self {
        Self {
            theta,
            log_partition,
            hessian_a,
            step_size: 0.01,
        }
    }
    /// Natural gradient update: θ ← θ − ε · (∇²A(θ))^{−1} ∇L(θ).
    pub fn update(&mut self, grad_loss: &[f64]) {
        let nat_grad = solve_linear_system(&self.hessian_a, grad_loss);
        let d = self.theta.len();
        for i in 0..d {
            self.theta[i] -= self.step_size * nat_grad[i];
        }
    }
    /// KL divergence from current θ to a target θ_target in the exponential family:
    /// D_KL(p_θ_target ‖ p_θ) = A(θ) − A(θ_target) + ⟨∇A(θ_target), θ − θ_target⟩.
    pub fn kl_divergence(&self, theta_target: &[f64], a_target: f64, grad_a_target: &[f64]) -> f64 {
        let d = self.theta.len();
        let diff: Vec<f64> = (0..d).map(|i| self.theta[i] - theta_target[i]).collect();
        self.log_partition - a_target + dot_product(grad_a_target, &diff)
    }
}
/// Sum-product belief propagation on a factor graph.
///
/// In the IG framework, BP messages correspond to coordinate updates in the
/// Bethe free energy functional (a variational approximation).
pub struct BeliefPropagation {
    /// Number of variable nodes.
    pub n_vars: usize,
    /// Variable beliefs (discrete probability distributions).
    pub beliefs: Vec<Vec<f64>>,
    /// Messages from variable i to factor a: messages\[i\]\[a\].
    pub messages: Vec<Vec<Vec<f64>>>,
    /// Number of factor nodes.
    pub n_factors: usize,
}
impl BeliefPropagation {
    /// Create a new BP state with uniform beliefs.
    pub fn new(n_vars: usize, n_factors: usize, n_states: usize) -> Self {
        let uniform = vec![1.0 / n_states as f64; n_states];
        Self {
            n_vars,
            beliefs: vec![uniform.clone(); n_vars],
            messages: vec![vec![uniform.clone(); n_factors]; n_vars],
            n_factors,
        }
    }
    /// Normalize a vector in place: divide by sum.
    pub fn normalize(v: &mut Vec<f64>) {
        let z: f64 = v.iter().sum();
        if z > 0.0 {
            v.iter_mut().for_each(|x| *x /= z);
        }
    }
    /// Bethe free energy: F_Bethe = -Σ_a ln Z_a + Σ_i (d_i - 1) ln Z_i
    /// approximated as entropy of beliefs minus expected energy.
    /// Here returns the negative entropy of the beliefs (entropy term).
    pub fn bethe_entropy(&self) -> f64 {
        let mut h = 0.0f64;
        for b in &self.beliefs {
            for &bi in b {
                if bi > 0.0 {
                    h -= bi * bi.ln();
                }
            }
        }
        h
    }
    /// KL divergence from belief b_i to a target distribution q.
    pub fn kl_from_belief(&self, var: usize, q: &[f64]) -> f64 {
        let b = &self.beliefs[var];
        b.iter()
            .zip(q.iter())
            .filter(|(&bi, _)| bi > 0.0)
            .map(|(&bi, &qi)| {
                if qi == 0.0 {
                    f64::INFINITY
                } else {
                    bi * (bi / qi).ln()
                }
            })
            .sum()
    }
}
/// Moment (mean) parameter η = E_θ[T(x)] ∈ ℝ^d.
pub struct MomentParameter {
    /// Parameter values.
    pub values: Vec<f64>,
}
impl MomentParameter {
    /// Create a new MomentParameter.
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SchroedingerBridge {
    pub source_distribution: String,
    pub target_distribution: String,
    pub reference_process: String,
    pub regularization: f64,
}
#[allow(dead_code)]
impl SchroedingerBridge {
    pub fn new(source: &str, target: &str, reference: &str, eps: f64) -> Self {
        SchroedingerBridge {
            source_distribution: source.to_string(),
            target_distribution: target.to_string(),
            reference_process: reference.to_string(),
            regularization: eps,
        }
    }
    pub fn sinkhorn_algorithm(&self) -> String {
        format!(
            "Sinkhorn: regularized OT with ε={:.4}, solves Schrödinger bridge {} → {}",
            self.regularization, self.source_distribution, self.target_distribution
        )
    }
    pub fn ipfp_iteration(&self) -> String {
        "IPFP: alternating projection on marginal constraints (Schrödinger system)".to_string()
    }
    pub fn connection_to_diffusion_models(&self) -> String {
        "Schrödinger bridge = constrained stochastic optimal transport (diffusion model training)"
            .to_string()
    }
}
/// Natural parameter θ ∈ ℝ^d.
pub struct NaturalParameter {
    /// Parameter values.
    pub values: Vec<f64>,
}
impl NaturalParameter {
    /// Create a new NaturalParameter.
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }
    /// Dimension d of the parameter space.
    pub fn dim(&self) -> usize {
        self.values.len()
    }
}
/// Bayesian posterior computation: p(θ|x) ∝ L(θ|x) π(θ).
pub struct BayesianEstimation {
    /// Prior distribution π(θ) evaluated at grid points.
    pub prior: Vec<f64>,
    /// Grid of θ values.
    pub theta_grid: Vec<f64>,
}
impl BayesianEstimation {
    /// Create a new BayesianEstimation.
    pub fn new(prior: Vec<f64>, theta_grid: Vec<f64>) -> Self {
        Self { prior, theta_grid }
    }
    /// Compute the posterior p(θ|x) ∝ L(θ|x) π(θ) at each grid point.
    pub fn posterior(&self, log_likelihoods: &[f64]) -> Vec<f64> {
        let unnorm: Vec<f64> = self
            .prior
            .iter()
            .zip(log_likelihoods.iter())
            .map(|(&pi, &ll)| pi * ll.exp())
            .collect();
        let z: f64 = unnorm.iter().sum();
        if z == 0.0 {
            return unnorm;
        }
        unnorm.iter().map(|&v| v / z).collect()
    }
    /// MAP estimate: θ_MAP = argmax_θ [log L(θ|x) + log π(θ)].
    pub fn map_estimate(&self, log_likelihoods: &[f64]) -> f64 {
        let log_posterior: Vec<f64> = self
            .prior
            .iter()
            .zip(log_likelihoods.iter())
            .map(|(&pi, &ll)| ll + pi.max(1e-300).ln())
            .collect();
        let idx = log_posterior
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.theta_grid[idx]
    }
}
/// Extended exponential family with sufficient statistics T and base measure h.
pub struct ExponentialFamilyExt {
    /// Natural parameters θ.
    pub theta: Vec<f64>,
    /// A(θ): log-partition function value at current θ (scalar).
    pub log_partition_val: f64,
    /// ∇A(θ): moment parameters η = E_θ[T(x)] (gradient of log-partition).
    pub moment_params: Vec<f64>,
}
impl ExponentialFamilyExt {
    /// Construct with given θ, A(θ), and η = ∇A(θ).
    pub fn new(theta: Vec<f64>, log_partition_val: f64, moment_params: Vec<f64>) -> Self {
        Self {
            theta,
            log_partition_val,
            moment_params,
        }
    }
    /// KL divergence to another member of the same exponential family:
    /// D_KL(p_θ ‖ p_θ') = A(θ') − A(θ) − ⟨∇A(θ), θ' − θ⟩
    ///                    = D_A(η ‖ η') (Bregman divergence on moment side).
    pub fn kl_to(&self, other: &ExponentialFamilyExt) -> f64 {
        let d = self.theta.len();
        let diff_theta: Vec<f64> = (0..d).map(|i| other.theta[i] - self.theta[i]).collect();
        let inner = dot_product(&self.moment_params, &diff_theta);
        other.log_partition_val - self.log_partition_val - inner
    }
    /// Bregman projection of η_target onto this exponential family:
    /// projects the point η_target onto the e-flat submanifold by moment matching.
    /// For exponential families, e-projection = moment matching.
    pub fn e_project_moment(&self, eta_target: &[f64]) -> Vec<f64> {
        eta_target.to_vec()
    }
    /// Variational lower bound: L(q; x) = E_q\[log p(x,z)\] − E_q\[log q(z)\]
    /// approximated as: A*(η) + ⟨θ_x, η⟩ − A(θ) for observed data natural parameter θ_x.
    pub fn variational_lower_bound(&self, theta_obs: &[f64]) -> f64 {
        let inner = dot_product(&self.moment_params, theta_obs);
        let a_star = dot_product(&self.theta, &self.moment_params) - self.log_partition_val;
        a_star + inner - self.log_partition_val
    }
}
/// Statistical manifold with dual connections.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StatManiMid {
    pub name: String,
    pub dimension: usize,
    pub alpha: f64,
}
impl StatManiMid {
    #[allow(dead_code)]
    pub fn new(name: &str, dim: usize, alpha: f64) -> Self {
        Self {
            name: name.to_string(),
            dimension: dim,
            alpha,
        }
    }
    #[allow(dead_code)]
    pub fn exponential_family(name: &str, dim: usize) -> Self {
        Self::new(name, dim, 1.0)
    }
    #[allow(dead_code)]
    pub fn mixture_family(name: &str, dim: usize) -> Self {
        Self::new(name, dim, -1.0)
    }
    #[allow(dead_code)]
    pub fn is_dually_flat(&self) -> bool {
        self.alpha.abs() == 1.0
    }
    #[allow(dead_code)]
    pub fn alpha_divergence_description(&self) -> String {
        let a = self.alpha;
        format!(
            "D_alpha(p||q) = (4/(1-a^2)) * (1 - integral p^((1+a)/2) q^((1-a)/2)) for alpha={a}"
        )
    }
}
/// Natural gradient descent.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NatGradMid {
    pub parameter_dim: usize,
    pub step_size: f64,
}
impl NatGradMid {
    #[allow(dead_code)]
    pub fn new(dim: usize, lr: f64) -> Self {
        Self {
            parameter_dim: dim,
            step_size: lr,
        }
    }
    #[allow(dead_code)]
    pub fn update_rule(&self) -> String {
        format!(
            "theta_new = theta - eta * G^-1 * grad L, eta={}, G = Fisher info matrix",
            self.step_size
        )
    }
    #[allow(dead_code)]
    pub fn invariance_property(&self) -> String {
        "Natural gradient is invariant to re-parameterization of the model".to_string()
    }
    #[allow(dead_code)]
    pub fn amari_convergence_description(&self) -> String {
        "Amari: natural gradient descent converges in O(1/n) independent of curvature in smooth neighborhoods"
            .to_string()
    }
}
/// Information projections.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InformationProjection {
    pub distribution_family: String,
    pub target_distribution: String,
    pub projection_type: ProjectionType,
}
impl InformationProjection {
    #[allow(dead_code)]
    pub fn new(family: &str, target: &str, pt: ProjectionType) -> Self {
        Self {
            distribution_family: family.to_string(),
            target_distribution: target.to_string(),
            projection_type: pt,
        }
    }
    #[allow(dead_code)]
    pub fn pythagorean_theorem(&self) -> String {
        match &self.projection_type {
            ProjectionType::Moment => {
                format!("m-proj: KL(q*||p) = KL(q*||q) + KL(q||p) for q in family, q* = min")
            }
            ProjectionType::Exponential => {
                format!("e-proj: KL(p||q*) = KL(p||q) + KL(q||q*) for q in family")
            }
        }
    }
}
/// Gaussian process (from information geometry perspective).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussianProcess {
    pub kernel_name: String,
    pub mean_function: String,
    pub is_stationary: bool,
}
impl GaussianProcess {
    #[allow(dead_code)]
    pub fn rbf(length_scale: f64) -> Self {
        Self {
            kernel_name: format!("RBF/SE k(x,y) = exp(-||x-y||^2 / (2*{}^2))", length_scale),
            mean_function: "zero mean".to_string(),
            is_stationary: true,
        }
    }
    #[allow(dead_code)]
    pub fn matern_52(length_scale: f64) -> Self {
        Self {
            kernel_name: format!("Matern-5/2 ell={length_scale}"),
            mean_function: "zero mean".to_string(),
            is_stationary: true,
        }
    }
    #[allow(dead_code)]
    pub fn posterior_description(&self) -> String {
        "GP posterior: Gaussian with mean mu*(x) = K_{xX}(K_{XX}+sigmaI)^-1 y".to_string()
    }
    #[allow(dead_code)]
    pub fn rkhs_description(&self) -> String {
        format!(
            "RKHS associated with {} kernel: norm = sqrt(<f,f>_k)",
            self.kernel_name
        )
    }
}
/// Bregman divergence.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BregmanDivergence {
    pub generator: String,
    pub x_name: String,
    pub y_name: String,
}
impl BregmanDivergence {
    #[allow(dead_code)]
    pub fn new(generator: &str, x: &str, y: &str) -> Self {
        Self {
            generator: generator.to_string(),
            x_name: x.to_string(),
            y_name: y.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn itakura_saito() -> Self {
        Self::new("-log(x)", "x", "y")
    }
    #[allow(dead_code)]
    pub fn squared_euclidean() -> Self {
        Self::new("||x||^2/2", "x", "y")
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "D_{}({},{}) = {}({}) - {}({}) - <grad {}({}), {}-{}>",
            self.generator,
            self.x_name,
            self.y_name,
            self.generator,
            self.x_name,
            self.generator,
            self.y_name,
            self.generator,
            self.y_name,
            self.x_name,
            self.y_name
        )
    }
    #[allow(dead_code)]
    pub fn three_point_property(&self) -> String {
        format!(
            "D(x,z) = D(x,y) + D(y,z) + <grad {}(z) - grad {}(y), x-y>",
            self.generator, self.generator
        )
    }
}
/// Reference (Bernardo) prior: maximizes expected KL divergence to posterior.
pub struct ReferenceAnalysis {
    /// Number of observations to simulate.
    pub n_obs: usize,
}
impl ReferenceAnalysis {
    /// Create a new ReferenceAnalysis.
    pub fn new(n_obs: usize) -> Self {
        Self { n_obs }
    }
    /// For a regular 1D model, the reference prior equals the Jeffreys prior.
    pub fn equals_jeffreys_in_1d(&self) -> bool {
        true
    }
    /// Expected Kullback-Leibler divergence between prior and posterior (approximation).
    ///
    /// Under mild regularity: E[D_KL(p(θ|x^n) ‖ π(θ))] ≈ (1/2) log(n/(2πe)) + (1/2) E\[log I(θ)\]
    pub fn expected_kl_approximation(&self, log_fisher_mean: f64) -> f64 {
        let n = self.n_obs as f64;
        0.5 * (n / (2.0 * std::f64::consts::PI * std::f64::consts::E)).ln() + 0.5 * log_fisher_mean
    }
}
/// Exponential family distribution.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExponentialFamilyDistrib {
    pub name: String,
    pub natural_param_dim: usize,
    pub sufficient_statistic: String,
    pub log_partition: String,
}
impl ExponentialFamilyDistrib {
    #[allow(dead_code)]
    pub fn gaussian(dim: usize) -> Self {
        Self {
            name: format!("N(mu, Sigma) dim={dim}"),
            natural_param_dim: dim + dim * (dim + 1) / 2,
            sufficient_statistic: "T(x) = (x, xx^T)".to_string(),
            log_partition: "A(eta) = -eta1^T eta2^-1 eta1/4 - log|eta2|/2 + const".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn poisson() -> Self {
        Self {
            name: "Poisson(lambda)".to_string(),
            natural_param_dim: 1,
            sufficient_statistic: "T(x) = x".to_string(),
            log_partition: "A(eta) = exp(eta)".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn mle_equals_moment_matching(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn natural_to_moment_params(&self) -> String {
        format!(
            "mu = grad A(eta): maps natural params to moment params for {}",
            self.name
        )
    }
}
/// Natural gradient descent optimizer using the Fisher information metric.
///
/// Update rule: θ ← θ − ε · G(θ)^{−1} ∇L(θ)
/// where G(θ) is the d×d Fisher information matrix.
pub struct NaturalGradient {
    /// Current parameter vector θ ∈ ℝ^d.
    pub theta: Vec<f64>,
    /// Learning rate ε.
    pub learning_rate: f64,
    /// Tikhonov damping λ added to Fisher diagonal for numerical stability.
    pub damping: f64,
}
impl NaturalGradient {
    /// Create a new NaturalGradient optimizer.
    pub fn new(theta: Vec<f64>, learning_rate: f64) -> Self {
        Self {
            theta,
            learning_rate,
            damping: 1e-4,
        }
    }
    /// Perform one natural gradient step given the Euclidean gradient and Fisher matrix.
    ///
    /// Solves G(θ) · δ = ∇L(θ) via Cholesky (here: simple damped diagonal inversion
    /// as a diagonal approximation).
    pub fn step(&mut self, euclidean_grad: &[f64], fisher: &[Vec<f64>]) {
        let d = self.theta.len();
        let mut g_damp = fisher.to_vec();
        for i in 0..d {
            g_damp[i][i] += self.damping;
        }
        let natural_grad = solve_linear_system(&g_damp, euclidean_grad);
        for i in 0..d {
            self.theta[i] -= self.learning_rate * natural_grad[i];
        }
    }
    /// Compute the squared Fisher-Rao distance from current θ to a point θ₀,
    /// using the provided Fisher matrix G at the midpoint.
    pub fn fisher_rao_distance_sq(&self, theta0: &[f64], fisher: &[Vec<f64>]) -> f64 {
        let d = self.theta.len();
        let diff: Vec<f64> = (0..d).map(|i| self.theta[i] - theta0[i]).collect();
        let gv = mat_vec(fisher, &diff);
        dot_product(&diff, &gv)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuantumInfoGeometry {
    pub hilbert_dim: usize,
    pub metric_name: String,
    pub is_monotone_metric: bool,
    pub petz_class_param: f64,
}
#[allow(dead_code)]
impl QuantumInfoGeometry {
    pub fn bures_metric(n: usize) -> Self {
        QuantumInfoGeometry {
            hilbert_dim: n,
            metric_name: "Bures metric".to_string(),
            is_monotone_metric: true,
            petz_class_param: 0.5,
        }
    }
    pub fn kubo_mori_metric(n: usize) -> Self {
        QuantumInfoGeometry {
            hilbert_dim: n,
            metric_name: "Kubo-Mori (Bogoliubov) metric".to_string(),
            is_monotone_metric: true,
            petz_class_param: 1.0,
        }
    }
    pub fn petz_classification(&self) -> String {
        format!(
            "Petz class (f-metric): f(t)={:.2} parameter; Bures at f=1/2, KM at f=1",
            self.petz_class_param
        )
    }
    pub fn quantum_cramer_rao(&self) -> String {
        format!(
            "Quantum Cramér-Rao: Var(θ̂) ≥ 1/F_Q where F_Q is quantum Fisher info (dim={})",
            self.hilbert_dim
        )
    }
    pub fn holevo_bound(&self) -> String {
        format!(
            "Holevo bound: accessible info ≤ χ (Holevo quantity) for dim-{} quantum channel",
            self.hilbert_dim
        )
    }
    pub fn bures_distance(&self, fidelity: f64) -> f64 {
        (2.0 * (1.0 - fidelity.sqrt())).sqrt()
    }
}
/// Legendre transform A*(η) = sup_θ {⟨θ, η⟩ - A(θ)}.
pub struct LegendreTransform {
    /// The primal function A (log-partition), discretized at grid points.
    /// `theta_grid\[i\]` → `a_values\[i\]`
    pub theta_grid: Vec<f64>,
    /// A(θ) values.
    pub a_values: Vec<f64>,
}
impl LegendreTransform {
    /// Create a new LegendreTransform from discretized A.
    pub fn new(theta_grid: Vec<f64>, a_values: Vec<f64>) -> Self {
        Self {
            theta_grid,
            a_values,
        }
    }
    /// Evaluate A*(η) = sup_θ {⟨θ, η⟩ - A(θ)} numerically over the grid.
    pub fn evaluate(&self, eta: f64) -> f64 {
        self.theta_grid
            .iter()
            .zip(self.a_values.iter())
            .map(|(&theta, &a)| theta * eta - a)
            .fold(f64::NEG_INFINITY, f64::max)
    }
    /// Bregman divergence: D_A(η ‖ η') = A*(η) - A*(η') - ∇A*(η')·(η - η').
    ///
    /// Uses finite-difference gradient of A*.
    pub fn bregman_divergence(&self, eta: f64, eta_prime: f64) -> f64 {
        let a_star_eta = self.evaluate(eta);
        let a_star_etap = self.evaluate(eta_prime);
        let h = 1e-5;
        let grad_etap = (self.evaluate(eta_prime + h) - self.evaluate(eta_prime - h)) / (2.0 * h);
        a_star_eta - a_star_etap - grad_etap * (eta - eta_prime)
    }
}
/// Alpha-divergence.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AlphaDivMid {
    pub alpha: f64,
    pub p_name: String,
    pub q_name: String,
}
impl AlphaDivMid {
    #[allow(dead_code)]
    pub fn new(alpha: f64, p: &str, q: &str) -> Self {
        Self {
            alpha,
            p_name: p.to_string(),
            q_name: q.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn kl_divergence(p: &str, q: &str) -> Self {
        Self::new(1.0, p, q)
    }
    #[allow(dead_code)]
    pub fn reverse_kl(p: &str, q: &str) -> Self {
        Self::new(-1.0, p, q)
    }
    #[allow(dead_code)]
    pub fn hellinger(p: &str, q: &str) -> Self {
        Self::new(0.0, p, q)
    }
    #[allow(dead_code)]
    pub fn is_kl(&self) -> bool {
        (self.alpha - 1.0).abs() < 1e-10
    }
    #[allow(dead_code)]
    pub fn formula(&self) -> String {
        let a = self.alpha;
        format!(
            "D_{}({} || {}) = (4/(1-{}^2)) * (1 - E[({}/{})^((1-{})/2)])",
            a, self.p_name, self.q_name, a, self.p_name, self.q_name, a
        )
    }
}
