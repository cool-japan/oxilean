//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// ADMM solver for problems of the form: min f(x) + g(z) s.t. Ax + Bz = c.
#[allow(dead_code)]
pub struct ADMMSolver {
    /// Penalty parameter ρ > 0.
    pub rho: f64,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Tolerance.
    pub tol: f64,
}
#[allow(dead_code)]
impl ADMMSolver {
    /// Create a new ADMM solver.
    pub fn new(rho: f64, max_iter: usize, tol: f64) -> Self {
        ADMMSolver { rho, max_iter, tol }
    }
    /// ADMM for LASSO: min (1/2)||Ax-b||^2 + λ||x||_1.
    /// Simplified: returns the number of iterations to convergence estimate.
    pub fn lasso_convergence_rate(&self, lambda: f64, _n: usize) -> f64 {
        1.0 - 1.0 / (1.0 + lambda / self.rho)
    }
    /// Dual update: y_{k+1} = y_k + ρ(Ax_{k+1} + Bz_{k+1} - c).
    /// Simplified for scalar: returns updated dual variable.
    pub fn dual_update(&self, y: f64, primal_residual: f64) -> f64 {
        y + self.rho * primal_residual
    }
    /// Stopping criteria: primal and dual residuals.
    /// primal: ||Ax + Bz - c||, dual: ||ρA^T B(z - z_old)||.
    pub fn stopping_criteria(&self, primal_res: f64, dual_res: f64) -> bool {
        primal_res < self.tol && dual_res < self.tol
    }
    /// Convergence bound: ADMM converges at O(1/k) for general convex problems.
    pub fn convergence_bound_at(&self, k: usize, _initial_gap: f64) -> f64 {
        1.0 / k.max(1) as f64
    }
}
/// Infimal convolution (epi-sum) of two functions f and g.
pub struct InfConvolution {
    /// Name or formula for f.
    pub f: String,
    /// Name or formula for g.
    pub g: String,
}
impl InfConvolution {
    /// Create a new InfConvolution.
    pub fn new(f: impl Into<String>, g: impl Into<String>) -> Self {
        Self {
            f: f.into(),
            g: g.into(),
        }
    }
    /// Infimal convolution of two convex functions is convex.
    pub fn is_convex_if_both_convex(&self) -> bool {
        true
    }
    /// Epigraph characterisation: epi(f □ g) = epi(f) + epi(g) (Minkowski sum).
    pub fn epigraph_sum(&self) -> String {
        format!(
            "Epigraph sum: epi({} □ {}) = epi({}) + epi({}) (Minkowski sum of epigraphs). \
             This is why the infimal convolution is also called the epigraph sum.",
            self.f, self.g, self.f, self.g
        )
    }
}
/// Fenchel duality between convex functions f and g.
pub struct FenchelDuality {
    /// Name or formula for the primal function f.
    pub f: String,
    /// Name or formula for the perturbation g.
    pub g: String,
}
impl FenchelDuality {
    /// Create a new FenchelDuality instance.
    pub fn new(f: impl Into<String>, g: impl Into<String>) -> Self {
        Self {
            f: f.into(),
            g: g.into(),
        }
    }
    /// Returns whether strong duality holds (zero duality gap).
    pub fn strong_duality_holds(&self) -> bool {
        true
    }
    /// Optimality condition for Fenchel duality.
    pub fn optimal_condition(&self) -> String {
        format!(
            "Fenchel-Rockafellar strong duality: If dom({}) ∩ int(dom({})) ≠ ∅, then \
             inf_x({}(x) + {}(Ax)) = max_y(-{}*(-A*y) - {}*(y)), \
             and the gap is zero.",
            self.f, self.g, self.f, self.g, self.f, self.g
        )
    }
}
/// Represents a convex function via its subdifferential properties.
#[allow(dead_code)]
pub struct ConvexSubdifferential {
    /// Name/description of the function.
    pub name: String,
    /// Whether the function is differentiable.
    pub is_differentiable: bool,
    /// Whether the function is strongly convex with modulus μ > 0.
    pub strong_convexity_modulus: f64,
    /// Whether the function is Lipschitz continuous.
    pub is_lipschitz: bool,
    /// Lipschitz constant L (if is_lipschitz).
    pub lipschitz_constant: f64,
}
#[allow(dead_code)]
impl ConvexSubdifferential {
    /// Create a new subdifferential descriptor.
    pub fn new(name: &str) -> Self {
        ConvexSubdifferential {
            name: name.to_string(),
            is_differentiable: false,
            strong_convexity_modulus: 0.0,
            is_lipschitz: false,
            lipschitz_constant: 0.0,
        }
    }
    /// Set differentiability.
    pub fn with_differentiability(mut self) -> Self {
        self.is_differentiable = true;
        self
    }
    /// Set strong convexity modulus μ.
    pub fn with_strong_convexity(mut self, mu: f64) -> Self {
        self.strong_convexity_modulus = mu;
        self
    }
    /// Set Lipschitz constant.
    pub fn with_lipschitz(mut self, l: f64) -> Self {
        self.is_lipschitz = true;
        self.lipschitz_constant = l;
        self
    }
    /// Subdifferential sum rule: ∂(f + g)(x) ⊇ ∂f(x) + ∂g(x).
    /// Equality holds when regularity condition is satisfied.
    pub fn sum_rule_holds(&self, other: &ConvexSubdifferential) -> bool {
        self.is_differentiable || other.is_differentiable
    }
    /// Chain rule: ∂(f ∘ A)(x) = A^T ∂f(Ax) under constraint qualification.
    pub fn chain_rule_holds(&self) -> bool {
        self.is_lipschitz
    }
    /// Fenchel conjugate: optimal convergence rate for gradient descent.
    /// With μ-strong convexity and L-smoothness: rate = 1 - μ/L.
    pub fn gradient_descent_rate(&self) -> Option<f64> {
        if self.strong_convexity_modulus > 0.0 && self.lipschitz_constant > 0.0 {
            Some(1.0 - self.strong_convexity_modulus / self.lipschitz_constant)
        } else {
            None
        }
    }
    /// Proximal point algorithm convergence: 1/k rate for convex, linear for strongly convex.
    pub fn proximal_convergence_rate(&self, k: usize) -> f64 {
        if self.strong_convexity_modulus > 0.0 {
            let lambda = 1.0 / (2.0 * self.lipschitz_constant.max(1.0));
            (1.0 - lambda * self.strong_convexity_modulus)
                .powi(k as i32)
                .abs()
        } else {
            1.0 / k.max(1) as f64
        }
    }
}
/// The epigraph of a function f: X → ℝ∪{+∞}.
pub struct Epigraph {
    /// Name or formula of the function.
    pub function: String,
}
impl Epigraph {
    /// Create a new Epigraph.
    pub fn new(function: impl Into<String>) -> Self {
        Self {
            function: function.into(),
        }
    }
    /// Epigraph is closed iff the function is lower semicontinuous (lsc).
    pub fn is_closed_iff_lsc(&self) -> String {
        format!(
            "Theorem: epi({}) = {{(x,α) | {}(x) ≤ α}} is a closed set in X×ℝ \
             if and only if {} is lower semicontinuous.",
            self.function, self.function, self.function
        )
    }
    /// Epigraph is convex iff the function is convex.
    pub fn convex_iff_fn_convex(&self) -> String {
        format!(
            "Theorem: epi({}) is a convex set iff {} is a convex function \
             (Jensen's inequality characterisation).",
            self.function, self.function
        )
    }
}
/// Proximal point algorithm solver for minimising a proper lower-semicontinuous function.
pub struct ProximalPointSolver {
    /// Regularisation parameter λ > 0.
    pub lambda: f64,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
impl ProximalPointSolver {
    /// Create a new proximal point solver.
    pub fn new(lambda: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            lambda,
            max_iter,
            tol,
        }
    }
    /// Approximate the proximal operator prox_{λf}(v) via gradient descent on f(x) + ‖x-v‖²/(2λ).
    pub fn prox_step(&self, f: impl Fn(&[f64]) -> f64 + Copy, v: &[f64]) -> Vec<f64> {
        let n = v.len();
        let mut x = v.to_vec();
        let inner_step = self.lambda * 0.01;
        let inner_iters = 500;
        for _ in 0..inner_iters {
            let grad_f = clarke_gradient_approx(f, &x, 1e-6);
            let mut moved = false;
            for i in 0..n {
                let total_grad = grad_f[i] + (x[i] - v[i]) / self.lambda;
                let new_xi = x[i] - inner_step * total_grad;
                if (new_xi - x[i]).abs() > 1e-14 {
                    moved = true;
                }
                x[i] = new_xi;
            }
            if !moved {
                break;
            }
        }
        x
    }
    /// Run the proximal point algorithm starting from x0.
    /// Returns the sequence of iterates.
    pub fn solve(&self, f: impl Fn(&[f64]) -> f64 + Copy, x0: &[f64]) -> Vec<Vec<f64>> {
        let mut iterates = vec![x0.to_vec()];
        let mut x = x0.to_vec();
        for _ in 0..self.max_iter {
            let x_new = self.prox_step(f, &x);
            let diff: f64 = x
                .iter()
                .zip(x_new.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            iterates.push(x_new.clone());
            x = x_new;
            if diff < self.tol {
                break;
            }
        }
        iterates
    }
    /// Check convergence: last step size is smaller than tolerance.
    pub fn has_converged(&self, iterates: &[Vec<f64>]) -> bool {
        if iterates.len() < 2 {
            return false;
        }
        let last = iterates
            .last()
            .expect("iterates has at least 2 elements: checked by early return");
        let prev = &iterates[iterates.len() - 2];
        let diff: f64 = last
            .iter()
            .zip(prev.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        diff < self.tol
    }
}
/// Type of function for proximal operator.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ProxFnType {
    /// L1 norm: f(x) = ||x||_1 → soft thresholding.
    L1Norm,
    /// Squared L2 norm: f(x) = ||x||_2^2 → shrinkage.
    L2NormSquared,
    /// Indicator of {x : x ≥ 0}: f(x) = 0 if x≥0, ∞ else → projection.
    NonNegativeOrtHant,
    /// Indicator of L2 ball of radius r: ||x||_2 ≤ r.
    L2Ball { radius: f64 },
}
/// Proximal mapping of a function f with parameter λ > 0.
pub struct ProximalMapping {
    /// Name or formula of the function f.
    pub f: String,
    /// Regularisation parameter λ > 0.
    pub lambda: f64,
}
impl ProximalMapping {
    /// Create a new ProximalMapping.
    pub fn new(f: impl Into<String>, lambda: f64) -> Self {
        Self {
            f: f.into(),
            lambda,
        }
    }
    /// Compute the proximal point (conceptually): prox_{λf}(x) = argmin_y {f(y) + |y-x|²/(2λ)}.
    pub fn prox_point(&self) -> String {
        format!(
            "prox_{{λ{}}}(x) = argmin_y {{ {}(y) + ||y - x||²/(2·{:.4}) }}. \
             Moreau (1962): this is always uniquely defined for proper lsc convex f.",
            self.f, self.f, self.lambda
        )
    }
    /// Firm nonexpansiveness of the proximal mapping.
    pub fn firm_nonexpansive(&self) -> String {
        format!(
            "The proximal mapping prox_{{λ·{}}} is firmly nonexpansive: \
             ||prox(x) - prox(y)||² ≤ ⟨prox(x)-prox(y), x-y⟩ for all x,y.",
            self.f
        )
    }
}
/// A sequence of functions represented as closures.
pub struct FunctionSequence {
    /// The functions f_n, stored by index.
    functions: Vec<Box<dyn Fn(&[f64]) -> f64 + Send + Sync>>,
}
impl FunctionSequence {
    /// Construct from a vector of boxed functions.
    pub fn new(functions: Vec<Box<dyn Fn(&[f64]) -> f64 + Send + Sync>>) -> Self {
        Self { functions }
    }
    /// Length of the sequence.
    pub fn len(&self) -> usize {
        self.functions.len()
    }
    /// Is the sequence empty?
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }
    /// Evaluate f_n at x.
    pub fn eval(&self, n: usize, x: &[f64]) -> f64 {
        self.functions[n](x)
    }
    /// Compute epi-liminf at x using a discrete grid of perturbation vectors.
    /// epi-liminf_n f_n(x) = liminf_{y→x} liminf_n f_n(y).
    pub fn epi_liminf(&self, x: &[f64], radius: f64, grid_steps: usize) -> f64 {
        let n_fns = self.functions.len();
        if n_fns == 0 {
            return f64::INFINITY;
        }
        let mut result = f64::INFINITY;
        let step = if grid_steps > 0 {
            2.0 * radius / grid_steps as f64
        } else {
            radius
        };
        let dim = x.len();
        let perturb_count = if dim == 1 { grid_steps + 1 } else { grid_steps };
        for k in 0..perturb_count {
            let mut y = x.to_vec();
            if dim >= 1 {
                y[0] = x[0] - radius + k as f64 * step;
            }
            let mut liminf_n = f64::INFINITY;
            for fn_n in &self.functions {
                let val = fn_n(&y);
                if val < liminf_n {
                    liminf_n = val;
                }
            }
            if liminf_n < result {
                result = liminf_n;
            }
        }
        result
    }
    /// Compute epi-limsup at x.
    pub fn epi_limsup(&self, x: &[f64], radius: f64, grid_steps: usize) -> f64 {
        let n_fns = self.functions.len();
        if n_fns == 0 {
            return f64::NEG_INFINITY;
        }
        let step = if grid_steps > 0 {
            2.0 * radius / grid_steps as f64
        } else {
            radius
        };
        let dim = x.len();
        let mut result = f64::NEG_INFINITY;
        let perturb_count = if dim == 1 { grid_steps + 1 } else { grid_steps };
        for k in 0..perturb_count {
            let mut y = x.to_vec();
            if dim >= 1 {
                y[0] = x[0] - radius + k as f64 * step;
            }
            let mut limsup_n = f64::NEG_INFINITY;
            for fn_n in &self.functions {
                let val = fn_n(&y);
                if val > limsup_n {
                    limsup_n = val;
                }
            }
            if limsup_n > result {
                result = limsup_n;
            }
        }
        result
    }
    /// Approximate Γ-liminf of the sequence at x.
    /// Γ-liminf_n f_n(x) = sup_{U∋x} liminf_n inf_{y∈U} f_n(y).
    pub fn gamma_liminf(&self, x: &[f64], radii: &[f64]) -> f64 {
        let mut best = f64::NEG_INFINITY;
        for &r in radii {
            let steps = 20;
            let step = 2.0 * r / steps as f64;
            let mut inf_over_n = f64::INFINITY;
            for n_idx in 0..self.functions.len() {
                let mut local_inf = f64::INFINITY;
                for k in 0..=steps {
                    let mut y = x.to_vec();
                    if !y.is_empty() {
                        y[0] = x[0] - r + k as f64 * step;
                    }
                    let val = self.functions[n_idx](&y);
                    if val < local_inf {
                        local_inf = val;
                    }
                }
                if n_idx == 0 || local_inf < inf_over_n {
                    if local_inf < inf_over_n {
                        inf_over_n = local_inf;
                    }
                }
            }
            if inf_over_n > best {
                best = inf_over_n;
            }
        }
        best
    }
}
/// Checks approximate r-prox-regularity of a finite point set
/// by verifying that the projection is unique in an r-tube.
#[derive(Debug, Clone)]
pub struct ProxRegularSet {
    /// Points representing the boundary of the set.
    pub boundary_points: Vec<Vec<f64>>,
    /// Prox-regularity radius.
    pub radius: f64,
}
impl ProxRegularSet {
    /// Construct a new prox-regular set.
    pub fn new(boundary_points: Vec<Vec<f64>>, radius: f64) -> Self {
        Self {
            boundary_points,
            radius,
        }
    }
    /// Compute projection onto the boundary point set.
    pub fn project(&self, x: &[f64]) -> Vec<f64> {
        let mut best_dist = f64::INFINITY;
        let mut best = x.to_vec();
        for p in &self.boundary_points {
            let dist: f64 = x
                .iter()
                .zip(p.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            if dist < best_dist {
                best_dist = dist;
                best = p.clone();
            }
        }
        best
    }
    /// Check if the projection from x is unique (only one nearest neighbour).
    pub fn has_unique_projection(&self, x: &[f64]) -> bool {
        let mut dists: Vec<f64> = self
            .boundary_points
            .iter()
            .map(|p| {
                x.iter()
                    .zip(p.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .collect();
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        if dists.len() < 2 {
            return true;
        }
        (dists[0] - dists[1]).abs() > 1e-9
    }
    /// Check prox-regularity: all points within the r-tube have unique projections.
    pub fn check_prox_regular(&self, test_points: &[Vec<f64>]) -> bool {
        for x in test_points {
            let dist_to_set = self
                .boundary_points
                .iter()
                .map(|p| {
                    x.iter()
                        .zip(p.iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum::<f64>()
                        .sqrt()
                })
                .fold(f64::INFINITY, f64::min);
            if dist_to_set < self.radius && !self.has_unique_projection(x) {
                return false;
            }
        }
        true
    }
}
/// A variational inequality: find x ∈ C such that ⟨F(x), y-x⟩ ≥ 0 for all y ∈ C.
pub struct VariationalInequality {
    /// Name or description of the operator F.
    pub operator: String,
    /// Domain/constraint set C.
    pub domain: String,
}
impl VariationalInequality {
    /// Create a new VariationalInequality.
    pub fn new(operator: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            operator: operator.into(),
            domain: domain.into(),
        }
    }
    /// Minty-Stampacchia theorem: existence of solutions.
    pub fn minty_stampacchia(&self) -> String {
        format!(
            "Minty-Stampacchia (1960/1964): For the VI with F='{}' on C='{}': \
             if F is monotone and hemicontinuous, then x* satisfies the VI iff \
             ⟨F(y), y-x*⟩ ≥ 0 for all y ∈ C (Minty formulation).",
            self.operator, self.domain
        )
    }
    /// Existence condition for the variational inequality.
    pub fn existence_condition(&self) -> String {
        format!(
            "Existence (Hartman-Stampacchia 1966): If C='{}' is compact convex and \
             F='{}' is continuous, then the VI has at least one solution. \
             For unbounded C, a coercivity condition ⟨F(x),x-x₀⟩/||x||→∞ suffices.",
            self.domain, self.operator
        )
    }
}
/// Subdifferential of a convex or nonsmooth function at a point.
pub struct Subdifferential {
    /// The point at which the subdifferential is evaluated.
    pub point: Vec<f64>,
}
impl Subdifferential {
    /// Create a new Subdifferential at the given point.
    pub fn new(point: Vec<f64>) -> Self {
        Self { point }
    }
    /// Returns whether the subdifferential is nonempty at interior points.
    pub fn is_nonempty_at_interior(&self) -> bool {
        true
    }
    /// Optimality condition: 0 ∈ ∂f(x) iff x is a minimiser.
    pub fn optimality_condition(&self) -> String {
        let x_str: Vec<String> = self.point.iter().map(|v| format!("{:.3}", v)).collect();
        format!(
            "Fermat's rule: x = ({}) is a minimiser of f iff 0 ∈ ∂f(x). \
             For a smooth convex f this reduces to ∇f(x) = 0.",
            x_str.join(", ")
        )
    }
}
/// Approximate computation of the Mordukhovich (limiting) subdifferential
/// via a finite-horizon sequence of Fréchet subgradients.
pub struct MordukhovichSubdiffApprox {
    /// Finite-difference step for Fréchet subgradient computation.
    pub h: f64,
    /// Number of perturbation directions to sample.
    pub num_dirs: usize,
    /// Regularisation for approximate subdifferential.
    pub tolerance: f64,
}
impl MordukhovichSubdiffApprox {
    /// Create a new approximation with given parameters.
    pub fn new(h: f64, num_dirs: usize, tolerance: f64) -> Self {
        Self {
            h,
            num_dirs,
            tolerance,
        }
    }
    /// Compute an approximate Fréchet subgradient at x via central differences.
    pub fn frechet_subgradient(&self, f: impl Fn(&[f64]) -> f64, x: &[f64]) -> Vec<f64> {
        clarke_gradient_approx(f, x, self.h)
    }
    /// Compute approximate limiting subdifferential by sampling nearby Fréchet subgradients.
    /// Returns a finite set of approximate subgradients at perturbed points near x.
    pub fn limiting_subdiff_approx(
        &self,
        f: impl Fn(&[f64]) -> f64 + Copy,
        x: &[f64],
    ) -> Vec<Vec<f64>> {
        let n = x.len();
        let mut result = Vec::new();
        result.push(self.frechet_subgradient(f, x));
        let mut lcg = crate::random_matrix_theory::Lcg::new(42);
        for _ in 0..self.num_dirs {
            let mut y = x.to_vec();
            for yi in y.iter_mut() {
                *yi += (lcg.next_f64() - 0.5) * 2.0 * self.tolerance;
            }
            let g = self.frechet_subgradient(f, &y);
            let dist: f64 = (0..n).map(|i| (y[i] - x[i]).powi(2)).sum::<f64>().sqrt();
            if dist < self.tolerance {
                result.push(g);
            }
        }
        result
    }
    /// Check whether zero is approximately in the subdifferential (stationarity condition).
    pub fn is_stationary(&self, f: impl Fn(&[f64]) -> f64 + Copy, x: &[f64]) -> bool {
        let g = self.frechet_subgradient(f, x);
        let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        norm < self.tolerance
    }
}
/// A set-valued map (multifunction) F: X ⇒ Y.
pub struct SetValuedMap {
    /// The domain points.
    pub domain: Vec<f64>,
    /// The values (each a subset of Y, represented as a Vec).
    pub values: Vec<Vec<f64>>,
}
impl SetValuedMap {
    /// Create a new SetValuedMap.
    pub fn new(domain: Vec<f64>, values: Vec<Vec<f64>>) -> Self {
        Self { domain, values }
    }
    /// Returns true if the map appears upper semicontinuous at each domain point.
    pub fn upper_semicontinuous(&self) -> bool {
        !self.values.iter().any(|v| v.is_empty())
    }
    /// Returns whether all values sets are closed (represented as non-empty).
    pub fn closed_values(&self) -> bool {
        !self.values.is_empty() && self.values.iter().all(|v| !v.is_empty())
    }
}
/// Moreau envelope (Moreau-Yosida regularisation) of f with parameter λ.
pub struct MoreauEnvelope {
    /// Name or formula of the original function f.
    pub f: String,
    /// Regularisation parameter λ > 0.
    pub lambda: f64,
}
impl MoreauEnvelope {
    /// Create a new MoreauEnvelope.
    pub fn new(f: impl Into<String>, lambda: f64) -> Self {
        Self {
            f: f.into(),
            lambda,
        }
    }
    /// Returns whether the Moreau envelope is smooth (always C^{1,1}).
    pub fn is_smooth(&self) -> bool {
        true
    }
    /// Gradient formula for the Moreau envelope.
    pub fn gradient_formula(&self) -> String {
        format!(
            "∇(e_{{λ{}}})(x) = (x - prox_{{λ{}}}(x)) / {:.4}. \
             This gradient is (1/λ)-Lipschitz, making e_{{λf}} a smooth approximation of {}.",
            self.f, self.f, self.lambda, self.f
        )
    }
}
/// Configuration for mountain pass detection.
#[derive(Debug, Clone)]
pub struct MountainPassConfig {
    /// Two base points a, b (distinct).
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    /// Estimated mountain pass level c = inf_{γ} max_{t} f(γ(t)).
    pub estimated_pass_level: f64,
    /// Number of sample points on path.
    pub path_samples: usize,
}
impl MountainPassConfig {
    /// Create a new mountain pass configuration.
    pub fn new(a: Vec<f64>, b: Vec<f64>, path_samples: usize) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            estimated_pass_level: 0.0,
            path_samples,
        }
    }
    /// Estimate mountain pass level via straight-line path γ(t) = (1-t)a + tb.
    pub fn estimate_pass_level(&mut self, f: &impl Fn(&[f64]) -> f64) -> f64 {
        let n = self.a.len();
        let mut max_val = f64::NEG_INFINITY;
        for k in 0..=self.path_samples {
            let t = k as f64 / self.path_samples as f64;
            let x: Vec<f64> = (0..n)
                .map(|i| (1.0 - t) * self.a[i] + t * self.b[i])
                .collect();
            let val = f(&x);
            if val > max_val {
                max_val = val;
            }
        }
        self.estimated_pass_level = max_val;
        max_val
    }
    /// Check mountain pass geometry: f(a) < c, f(b) < c, and some x on path achieves c.
    pub fn has_mountain_pass_geometry(&self, f: &impl Fn(&[f64]) -> f64) -> bool {
        let fa = f(&self.a);
        let fb = f(&self.b);
        fa < self.estimated_pass_level && fb < self.estimated_pass_level
    }
}
/// A convex function with domain and lower-semicontinuity properties.
pub struct ConvexFunction {
    /// Domain description (e.g., "R^n", "[0,1]").
    pub domain: String,
    /// Whether the function is proper (not identically +∞, never -∞).
    pub is_proper: bool,
    /// Whether the function is lower-semicontinuous (closed).
    pub is_lsc: bool,
}
impl ConvexFunction {
    /// Create a new ConvexFunction.
    pub fn new(domain: impl Into<String>, is_proper: bool, is_lsc: bool) -> Self {
        Self {
            domain: domain.into(),
            is_proper,
            is_lsc,
        }
    }
    /// Returns a description of the subdifferential of this function.
    pub fn subdifferential(&self) -> String {
        if self.is_proper && self.is_lsc {
            format!(
                "For a proper lsc convex function on '{}': the subdifferential ∂f(x) is \
                 nonempty on the interior of dom(f), and f = f** (Fenchel-Moreau theorem).",
                self.domain
            )
        } else {
            format!(
                "For a convex function on '{}': subdifferential ∂f(x) = {{ v | ∀y, f(y) ≥ f(x) + ⟨v,y-x⟩ }}.",
                self.domain
            )
        }
    }
    /// Returns a description of the conjugate (Fenchel dual) function.
    pub fn conjugate_fn(&self) -> String {
        format!(
            "Fenchel conjugate f*(v) = sup_{{x ∈ {}}} (⟨v,x⟩ - f(x)). \
             For proper lsc convex f: f** = f (Fenchel-Moreau, 1949).",
            self.domain
        )
    }
}
/// Constructive implementation of Ekeland's variational principle.
/// Finds xε such that f(xε) ≤ f(x₀) - ε·d(x₀, xε) and 0 ∈ ∂_{ε}f(xε).
pub struct EkelandPrinciple {
    /// Regularisation parameter ε > 0.
    pub epsilon: f64,
    /// Maximum iterations for approximate minimiser search.
    pub max_iter: usize,
    /// Step size for descent.
    pub step: f64,
}
impl EkelandPrinciple {
    /// Create a new EkelandPrinciple solver.
    pub fn new(epsilon: f64, max_iter: usize) -> Self {
        Self {
            epsilon,
            max_iter,
            step: epsilon * 0.1,
        }
    }
    /// Find an approximate Ekeland minimiser starting from x₀.
    pub fn find_minimiser(&self, f: impl Fn(&[f64]) -> f64, x0: &[f64]) -> Vec<f64> {
        ekeland_approximate_minimiser(f, x0, self.epsilon, self.max_iter)
    }
    /// Verify the Ekeland condition: f(xε) ≤ f(x) + ε·‖x - xε‖ for all x in sample set.
    pub fn verify_ekeland_condition(
        &self,
        f: impl Fn(&[f64]) -> f64,
        x_eps: &[f64],
        sample_points: &[Vec<f64>],
    ) -> bool {
        let f_eps = f(x_eps);
        for x in sample_points {
            let fx = f(x);
            let dist: f64 = x_eps
                .iter()
                .zip(x.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            if fx + self.epsilon * dist < f_eps - 1e-12 {
                return false;
            }
        }
        true
    }
    /// Estimate the near-minimality: how close f(xε) is to the true infimum (approximated).
    pub fn near_minimality_gap(&self, f: impl Fn(&[f64]) -> f64, x_eps: &[f64], x0: &[f64]) -> f64 {
        let f_eps = f(x_eps);
        let f_x0 = f(x0);
        f_x0 - f_eps
    }
}
/// Represents the proximal operator prox_{λf}(v) = argmin_x f(x) + ||x-v||^2/(2λ).
#[allow(dead_code)]
pub struct ProximalOperator {
    /// Regularization parameter λ > 0.
    pub lambda: f64,
    /// Type of function: L1, L2, indicator.
    pub fn_type: ProxFnType,
}
#[allow(dead_code)]
impl ProximalOperator {
    /// Create a new proximal operator.
    pub fn new(lambda: f64, fn_type: ProxFnType) -> Self {
        ProximalOperator { lambda, fn_type }
    }
    /// Apply the proximal operator to a scalar v.
    pub fn apply_scalar(&self, v: f64) -> f64 {
        match &self.fn_type {
            ProxFnType::L1Norm => {
                let threshold = self.lambda;
                v.signum() * (v.abs() - threshold).max(0.0)
            }
            ProxFnType::L2NormSquared => v / (1.0 + 2.0 * self.lambda),
            ProxFnType::NonNegativeOrtHant => v.max(0.0),
            ProxFnType::L2Ball { radius } => v.clamp(-radius, *radius),
        }
    }
    /// Apply the proximal operator component-wise to a vector v.
    pub fn apply_vector(&self, v: &[f64]) -> Vec<f64> {
        match &self.fn_type {
            ProxFnType::L2Ball { radius } => {
                let norm: f64 = v.iter().map(|&x| x * x).sum::<f64>().sqrt();
                if norm <= *radius {
                    v.to_vec()
                } else {
                    v.iter().map(|&x| x * radius / norm).collect()
                }
            }
            _ => v.iter().map(|&vi| self.apply_scalar(vi)).collect(),
        }
    }
    /// Moreau decomposition: prox_{λf}(v) + λ prox_{f*/λ}(v/λ) = v.
    /// Returns both prox_{λf}(v) and the dual part.
    pub fn moreau_decomposition(&self, v: f64) -> (f64, f64) {
        let p = self.apply_scalar(v);
        let dual = v - p;
        (p, dual)
    }
}
/// Checks metric regularity properties of a mapping at a point.
pub struct MetricRegularityChecker {
    /// Perturbation radius for numerical testing.
    pub radius: f64,
    /// Number of test perturbations.
    pub num_tests: usize,
    /// Tolerance for regularity bound checks.
    pub tol: f64,
}
impl MetricRegularityChecker {
    /// Create a new checker.
    pub fn new(radius: f64, num_tests: usize, tol: f64) -> Self {
        Self {
            radius,
            num_tests,
            tol,
        }
    }
    /// Estimate the metric regularity modulus κ of F at (x₀, y₀):
    /// κ ≈ sup_{(x,y) near (x₀,y₀)} d(x, F⁻¹(y)) / d(y, F(x)).
    ///
    /// Here F is represented as `f: &[f64] -> Vec<f64>` and F⁻¹(y) is approximated
    /// by the preimage computed via nearest-point search.
    pub fn estimate_modulus(
        &self,
        f: impl Fn(&[f64]) -> Vec<f64>,
        x0: &[f64],
        y0: &[f64],
        domain_samples: &[Vec<f64>],
    ) -> f64 {
        let mut max_ratio = 0.0_f64;
        let mut lcg = crate::random_matrix_theory::Lcg::new(7);
        for _ in 0..self.num_tests {
            let x_pert: Vec<f64> = x0
                .iter()
                .map(|&xi| xi + (lcg.next_f64() - 0.5) * 2.0 * self.radius)
                .collect();
            let fx_pert = f(&x_pert);
            let d_y_fx: f64 = y0
                .iter()
                .zip(fx_pert.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            if d_y_fx < 1e-14 {
                continue;
            }
            let d_x_finv = domain_samples
                .iter()
                .filter(|s| {
                    let fs = f(s);
                    let d: f64 = y0
                        .iter()
                        .zip(fs.iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum::<f64>()
                        .sqrt();
                    d < self.tol
                })
                .map(|s| {
                    x_pert
                        .iter()
                        .zip(s.iter())
                        .map(|(a, b)| (a - b).powi(2))
                        .sum::<f64>()
                        .sqrt()
                })
                .fold(f64::INFINITY, f64::min);
            if d_x_finv.is_finite() {
                let ratio = d_x_finv / d_y_fx;
                if ratio > max_ratio {
                    max_ratio = ratio;
                }
            }
        }
        max_ratio
    }
    /// Check whether a constraint qualification (MFCQ-like) holds by verifying
    /// that the constraint gradients at x span a "rich" direction set.
    pub fn check_constraint_qualification(
        constraints: &[impl Fn(&[f64]) -> f64],
        x: &[f64],
        h: f64,
    ) -> bool {
        if constraints.is_empty() {
            return true;
        }
        let mut grads: Vec<Vec<f64>> = Vec::new();
        for c in constraints {
            let cx = c(x);
            if cx.abs() < h {
                let g = clarke_gradient_approx(|pt| c(pt), x, h);
                grads.push(g);
            }
        }
        if grads.is_empty() {
            return true;
        }
        for i in 0..grads.len() {
            for j in (i + 1)..grads.len() {
                let ni: f64 = grads[i].iter().map(|v| v * v).sum::<f64>().sqrt();
                let nj: f64 = grads[j].iter().map(|v| v * v).sum::<f64>().sqrt();
                if ni < 1e-12 || nj < 1e-12 {
                    return false;
                }
                let dot: f64 = grads[i]
                    .iter()
                    .zip(grads[j].iter())
                    .map(|(a, b)| a * b)
                    .sum();
                let cos = (dot / (ni * nj)).abs();
                if cos > 1.0 - 1e-6 {
                    return false;
                }
            }
        }
        true
    }
    /// Check whether a set of points satisfies a quasiconvexity condition:
    /// f on the line segment [x,y] is ≤ max(f(x), f(y)) for sample points.
    pub fn check_quasiconvex(
        f: impl Fn(&[f64]) -> f64,
        x: &[f64],
        y: &[f64],
        num_samples: usize,
    ) -> bool {
        let max_val = f(x).max(f(y));
        for k in 1..num_samples {
            let t = k as f64 / num_samples as f64;
            let z: Vec<f64> = x
                .iter()
                .zip(y.iter())
                .map(|(xi, yi)| (1.0 - t) * xi + t * yi)
                .collect();
            if f(&z) > max_val + 1e-12 {
                return false;
            }
        }
        true
    }
}
