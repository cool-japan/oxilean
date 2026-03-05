//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Recession cone of a convex set.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RecessionCone {
    pub set_name: String,
}
impl RecessionCone {
    #[allow(dead_code)]
    pub fn new(set: &str) -> Self {
        Self {
            set_name: set.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "recess({}) = {{d : x + t*d in {} for all x in C, t >= 0}}",
            self.set_name, self.set_name
        )
    }
    #[allow(dead_code)]
    pub fn compact_iff_trivial_recession(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FenchelDualityPair {
    pub primal: String,
    pub dual: String,
    pub duality_gap: f64,
    pub strong_duality_holds: bool,
}
#[allow(dead_code)]
impl FenchelDualityPair {
    pub fn new(primal: &str, dual: &str, gap: f64) -> Self {
        FenchelDualityPair {
            primal: primal.to_string(),
            dual: dual.to_string(),
            duality_gap: gap,
            strong_duality_holds: gap.abs() < 1e-12,
        }
    }
    pub fn slater_condition_met(&self) -> bool {
        self.strong_duality_holds
    }
    pub fn duality_gap_description(&self) -> String {
        if self.strong_duality_holds {
            format!("Strong duality: val(P) = val(D) = {}", self.primal)
        } else {
            format!("Duality gap = {:.6}", self.duality_gap)
        }
    }
}
/// Subdifferential of a convex function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Subdifferential {
    pub function_name: String,
    pub point: Vec<f64>,
}
impl Subdifferential {
    #[allow(dead_code)]
    pub fn new(f: &str, x: Vec<f64>) -> Self {
        Self {
            function_name: f.to_string(),
            point: x,
        }
    }
    #[allow(dead_code)]
    pub fn subgradient_description(&self) -> String {
        format!(
            "g in partial f({:?}): f(y) >= f(x) + <g, y-x> for all y",
            self.point
        )
    }
    #[allow(dead_code)]
    pub fn is_differentiable_iff_singleton(&self) -> bool {
        true
    }
}
/// Step size schedule for subgradient methods.
#[derive(Debug, Clone, Copy)]
pub enum StepSchedule {
    /// Constant step: η_k = η.
    Constant(f64),
    /// Diminishing step: η_k = η / sqrt(k + 1).
    DiminishingSqrt(f64),
    /// Polyak step (requires lower bound f*): η_k = (f(x_k) - f*) / ‖g_k‖².
    Polyak { f_star: f64 },
}
/// Alternating projection method (von Neumann / Dykstra variant) for finding
/// a point in the intersection of two convex sets.
///
/// The sets are represented by their projection operators.
pub struct AlternatingProjectionSolver {
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance on ‖x_{k+1} - x_k‖.
    pub tol: f64,
}
impl AlternatingProjectionSolver {
    /// Create a new solver.
    pub fn new(max_iter: usize, tol: f64) -> Self {
        Self { max_iter, tol }
    }
    /// Run alternating projections between two sets given by their projection functions.
    /// Returns the sequence of iterates.
    pub fn run(
        &self,
        proj_a: impl Fn(&[f64]) -> Vec<f64>,
        proj_b: impl Fn(&[f64]) -> Vec<f64>,
        x0: &[f64],
    ) -> Vec<Vec<f64>> {
        let mut x = x0.to_vec();
        let mut iterates = vec![x.clone()];
        for _ in 0..self.max_iter {
            let pa = proj_a(&x);
            let pb = proj_b(&pa);
            let delta: f64 = x
                .iter()
                .zip(pb.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            x = pb;
            iterates.push(x.clone());
            if delta < self.tol {
                break;
            }
        }
        iterates
    }
}
/// Convex cone.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConvexCone {
    pub name: String,
    pub dimension: usize,
    pub is_pointed: bool,
    pub is_closed: bool,
}
impl ConvexCone {
    #[allow(dead_code)]
    pub fn nonneg_orthant(n: usize) -> Self {
        Self {
            name: format!("R^{}+", n),
            dimension: n,
            is_pointed: true,
            is_closed: true,
        }
    }
    #[allow(dead_code)]
    pub fn second_order_cone(n: usize) -> Self {
        Self {
            name: format!("SOC({}) = {{(x,t) : ||x|| <= t}}", n),
            dimension: n + 1,
            is_pointed: true,
            is_closed: true,
        }
    }
    #[allow(dead_code)]
    pub fn psd_cone(n: usize) -> Self {
        Self {
            name: format!("PSD({}) = S^{}+", n, n),
            dimension: n * (n + 1) / 2,
            is_pointed: true,
            is_closed: true,
        }
    }
    #[allow(dead_code)]
    pub fn dual_cone_description(&self) -> String {
        format!(
            "C* = {{y : <y, x> >= 0 for all x in {}}} (dual cone)",
            self.name
        )
    }
    #[allow(dead_code)]
    pub fn is_self_dual_nonneg_orthant(&self) -> bool {
        self.name.contains("R^") && self.name.ends_with('+')
    }
}
/// Represents a convex function f : ℝ^n → ℝ with optional gradient.
#[derive(Debug, Clone)]
pub struct ConvexFunction {
    /// Name of the function.
    pub name: String,
    /// Evaluation: f(x).
    eval_fn: fn(&[f64]) -> f64,
    /// Gradient: ∇f(x) (finite differences if not provided).
    grad_fn: Option<fn(&[f64]) -> Vec<f64>>,
    /// Lipschitz constant of gradient (if known).
    pub lipschitz_grad: Option<f64>,
    /// Strong convexity modulus (if known).
    pub strong_convexity: Option<f64>,
}
impl ConvexFunction {
    /// Construct a new convex function.
    pub fn new(
        name: impl Into<String>,
        eval_fn: fn(&[f64]) -> f64,
        grad_fn: Option<fn(&[f64]) -> Vec<f64>>,
    ) -> Self {
        Self {
            name: name.into(),
            eval_fn,
            grad_fn,
            lipschitz_grad: None,
            strong_convexity: None,
        }
    }
    /// Evaluate f at x.
    pub fn eval(&self, x: &[f64]) -> f64 {
        (self.eval_fn)(x)
    }
    /// Compute gradient of f at x (finite differences if no exact gradient provided).
    pub fn gradient(&self, x: &[f64]) -> Vec<f64> {
        if let Some(g) = self.grad_fn {
            return g(x);
        }
        let h = 1e-7;
        let mut grad = vec![0.0; x.len()];
        for i in 0..x.len() {
            let mut xp = x.to_vec();
            xp[i] += h;
            let mut xm = x.to_vec();
            xm[i] -= h;
            grad[i] = ((self.eval_fn)(&xp) - (self.eval_fn)(&xm)) / (2.0 * h);
        }
        grad
    }
    /// Compute the Fenchel conjugate f*(y) = sup_x {⟨y,x⟩ - f(x)} approximately
    /// via grid search on [-R, R]^n with resolution `steps`.
    pub fn fenchel_conjugate_approx(&self, y: &[f64], radius: f64, steps: usize) -> f64 {
        let n = y.len();
        if n == 0 {
            return 0.0;
        }
        let step_size = 2.0 * radius / steps as f64;
        if n == 1 {
            let mut best = f64::NEG_INFINITY;
            for k in 0..=steps {
                let x = -radius + k as f64 * step_size;
                let val = y[0] * x - self.eval(&[x]);
                if val > best {
                    best = val;
                }
            }
            return best;
        }
        let mut best = f64::NEG_INFINITY;
        for _ in 0..(steps * steps) {
            let x: Vec<f64> = (0..n)
                .map(|i| (i as f64 / n as f64) * 2.0 * radius - radius)
                .collect();
            let dot: f64 = y.iter().zip(x.iter()).map(|(a, b)| a * b).sum();
            let val = dot - self.eval(&x);
            if val > best {
                best = val;
            }
        }
        best
    }
    /// Compute epigraph membership: (x, t) ∈ epi(f) iff f(x) ≤ t.
    pub fn in_epigraph(&self, x: &[f64], t: f64) -> bool {
        self.eval(x) <= t
    }
    /// Compute level set membership: x ∈ lev_α(f) iff f(x) ≤ α.
    pub fn in_level_set(&self, x: &[f64], alpha: f64) -> bool {
        self.eval(x) <= alpha
    }
}
/// Represents a hyperplane { x | ⟨a, x⟩ = b }.
#[derive(Debug, Clone)]
pub struct Hyperplane {
    /// Normal vector a.
    pub normal: Vec<f64>,
    /// Offset b.
    pub offset: f64,
}
impl Hyperplane {
    /// Construct a hyperplane with given normal and offset.
    pub fn new(normal: Vec<f64>, offset: f64) -> Self {
        Self { normal, offset }
    }
    /// Signed distance ⟨a, x⟩ - b.
    pub fn signed_distance(&self, x: &[f64]) -> f64 {
        let dot: f64 = self.normal.iter().zip(x.iter()).map(|(a, xi)| a * xi).sum();
        dot - self.offset
    }
    /// Separates two point sets: all points in A above and all points in B below (or equal).
    pub fn separates(&self, a_points: &[Vec<f64>], b_points: &[Vec<f64>]) -> bool {
        a_points.iter().all(|x| self.signed_distance(x) >= -1e-9)
            && b_points.iter().all(|x| self.signed_distance(x) <= 1e-9)
    }
    /// Strictly separates two point sets.
    pub fn strictly_separates(&self, a_points: &[Vec<f64>], b_points: &[Vec<f64>]) -> bool {
        let min_a = a_points
            .iter()
            .map(|x| self.signed_distance(x))
            .fold(f64::INFINITY, f64::min);
        let max_b = b_points
            .iter()
            .map(|x| self.signed_distance(x))
            .fold(f64::NEG_INFINITY, f64::max);
        min_a > max_b + 1e-9
    }
}
/// Infimal convolution of two functions.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InfimalConvolution {
    pub f_name: String,
    pub g_name: String,
}
impl InfimalConvolution {
    #[allow(dead_code)]
    pub fn new(f: &str, g: &str) -> Self {
        Self {
            f_name: f.to_string(),
            g_name: g.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "({} inf-conv {})(x) = inf_y {{ {}(y) + {}(x-y) }}",
            self.f_name, self.g_name, self.f_name, self.g_name
        )
    }
    #[allow(dead_code)]
    pub fn conjugate_of_infconv(&self) -> String {
        format!(
            "({} inf-conv {})* = {}* + {}*",
            self.f_name, self.g_name, self.f_name, self.g_name
        )
    }
}
/// Proximal gradient method for composite minimisation: min f(x) + g(x).
/// f is smooth, g is proximable.
#[derive(Debug, Clone)]
pub struct ProximalGradientSolver {
    /// Step size (1/L where L is Lipschitz constant of ∇f).
    pub step_size: f64,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance on iterates.
    pub tol: f64,
}
impl ProximalGradientSolver {
    /// Create a new solver.
    pub fn new(step_size: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            step_size,
            max_iter,
            tol,
        }
    }
    /// Run proximal gradient: x_{k+1} = prox_{t*g}(x_k - t * ∇f(x_k)).
    pub fn solve(
        &self,
        f_smooth: &ConvexFunction,
        prox_g: impl Fn(&[f64], f64) -> Vec<f64>,
        x0: &[f64],
    ) -> Vec<f64> {
        let mut x = x0.to_vec();
        let n = x.len();
        for _ in 0..self.max_iter {
            let grad = f_smooth.gradient(&x);
            let gradient_step: Vec<f64> = x
                .iter()
                .zip(grad.iter())
                .map(|(xi, gi)| xi - self.step_size * gi)
                .collect();
            let new_x = prox_g(&gradient_step, self.step_size);
            let delta: f64 = x
                .iter()
                .zip(new_x.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            x = new_x;
            let _ = n;
            if delta < self.tol {
                break;
            }
        }
        x
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ADMMData {
    pub penalty_parameter: f64,
    pub num_iterations: usize,
    pub primal_residual: f64,
    pub dual_residual: f64,
    pub convergence_rate: f64,
}
#[allow(dead_code)]
impl ADMMData {
    pub fn new(rho: f64) -> Self {
        ADMMData {
            penalty_parameter: rho,
            num_iterations: 0,
            primal_residual: f64::INFINITY,
            dual_residual: f64::INFINITY,
            convergence_rate: 1.0 / rho,
        }
    }
    pub fn update_residuals(&mut self, primal: f64, dual: f64) {
        self.primal_residual = primal;
        self.dual_residual = dual;
        self.num_iterations += 1;
    }
    pub fn has_converged(&self, tol: f64) -> bool {
        self.primal_residual < tol && self.dual_residual < tol
    }
    pub fn admm_update_description(&self) -> String {
        format!(
            "ADMM (ρ={:.3}): x-update → z-update → y-update (dual ascent)",
            self.penalty_parameter
        )
    }
    pub fn convergence_guarantee(&self) -> String {
        "ADMM: O(1/k) convergence for convex problems; linear convergence for strongly convex"
            .to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvexProblemClass {
    Lp,
    Qp,
    Qcqp,
    Socp,
    Sdp,
    Gp,
    Cp,
}
/// Configuration for proximal operator computation via ADMM / gradient descent.
#[derive(Debug, Clone)]
pub struct ProxConfig {
    /// Regularisation parameter λ > 0.
    pub lambda: f64,
    /// Maximum number of gradient steps.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
    /// Step size for inner minimisation.
    pub step_size: f64,
}
impl ProxConfig {
    /// Default configuration.
    pub fn new(lambda: f64) -> Self {
        Self {
            lambda,
            max_iter: 500,
            tol: 1e-8,
            step_size: 0.01,
        }
    }
}
/// Proximal operator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProximalOperator {
    pub function_name: String,
    pub step_size: f64,
}
impl ProximalOperator {
    #[allow(dead_code)]
    pub fn new(f: &str, gamma: f64) -> Self {
        Self {
            function_name: f.to_string(),
            step_size: gamma,
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "prox_{{gamma {} }}(x) = argmin_y {{ {}(y) + ||y-x||^2/(2*{}) }}",
            self.function_name, self.function_name, self.step_size
        )
    }
    #[allow(dead_code)]
    pub fn for_indicator_function(&self) -> String {
        "prox_{gamma * I_C}(x) = proj_C(x) (projection onto convex set C)".to_string()
    }
    #[allow(dead_code)]
    pub fn moreau_decomposition(&self) -> String {
        format!(
            "Moreau: x = prox_{{{}}}(x) + sigma * prox_{{{}*/sigma}}(x/sigma)",
            self.function_name, self.function_name
        )
    }
}
/// ADMM (Alternating Direction Method of Multipliers) solver.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AdmmSolver {
    pub rho: f64,
    pub max_iter: usize,
    pub tolerance: f64,
}
impl AdmmSolver {
    #[allow(dead_code)]
    pub fn new(rho: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            rho,
            max_iter,
            tolerance: tol,
        }
    }
    #[allow(dead_code)]
    pub fn update_x_description(&self) -> String {
        format!(
            "x-update: x_(k+1) = argmin_x L_rho(x, z_k, u_k) with rho={}",
            self.rho
        )
    }
    #[allow(dead_code)]
    pub fn update_z_description(&self) -> String {
        "z-update: z_{k+1} = prox_{g/rho}(x_{k+1} + u_k)".to_string()
    }
    #[allow(dead_code)]
    pub fn convergence_description(&self) -> String {
        format!(
            "ADMM converges for any rho > 0 when f, g convex; tolerance {}",
            self.tolerance
        )
    }
}
/// Convex optimization problem in standard form.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConvexProgram {
    pub name: String,
    pub num_variables: usize,
    pub num_constraints: usize,
    pub problem_class: ConvexProblemClass,
}
impl ConvexProgram {
    #[allow(dead_code)]
    pub fn new(name: &str, n: usize, m: usize, class: ConvexProblemClass) -> Self {
        Self {
            name: name.to_string(),
            num_variables: n,
            num_constraints: m,
            problem_class: class,
        }
    }
    #[allow(dead_code)]
    pub fn is_lp(&self) -> bool {
        matches!(self.problem_class, ConvexProblemClass::Lp)
    }
    #[allow(dead_code)]
    pub fn interior_point_complexity(&self) -> String {
        match &self.problem_class {
            ConvexProblemClass::Lp => {
                format!("O(n^3.5 log(1/eps)) for LP n={}", self.num_variables)
            }
            ConvexProblemClass::Sdp => {
                format!("O(n^6 log(1/eps)) for SDP n={}", self.num_variables)
            }
            ConvexProblemClass::Socp => {
                format!("O(n^3.5 log(1/eps)) for SOCP n={}", self.num_variables)
            }
            _ => {
                format!(
                    "Poly(n,m) for n={}, m={}",
                    self.num_variables, self.num_constraints
                )
            }
        }
    }
    #[allow(dead_code)]
    pub fn strong_duality_holds(&self) -> bool {
        true
    }
}
/// Evaluates the Fenchel conjugate f* for specific function classes exactly.
pub struct FenchelConjugateEvaluator {
    /// The function class.
    pub class: FunctionClass,
}
impl FenchelConjugateEvaluator {
    /// Create a new evaluator for the given class.
    pub fn new(class: FunctionClass) -> Self {
        Self { class }
    }
    /// Evaluate f*(y) exactly (or fallback to numerical if `class = Numerical`).
    pub fn eval(&self, y: &[f64]) -> f64 {
        match &self.class {
            FunctionClass::SquaredNorm => 0.5 * y.iter().map(|yi| yi * yi).sum::<f64>(),
            FunctionClass::EuclideanNorm => {
                let norm: f64 = y.iter().map(|yi| yi * yi).sum::<f64>().sqrt();
                if norm <= 1.0 + 1e-9 {
                    0.0
                } else {
                    f64::INFINITY
                }
            }
            FunctionClass::NegativeEntropy => y.iter().map(|yi| (yi - 1.0).exp()).sum(),
            FunctionClass::LpNorm { p } => {
                if *p <= 1.0 + 1e-12 {
                    return f64::INFINITY;
                }
                let q = p / (p - 1.0);
                let lq: f64 = y.iter().map(|yi| yi.abs().powf(q)).sum::<f64>();
                lq / q
            }
            FunctionClass::BoxIndicator { lo, hi } => {
                y.iter().map(|yi| (lo * yi).max(hi * yi)).sum()
            }
            FunctionClass::Numerical => f64::NAN,
        }
    }
    /// Check Fenchel-Young inequality: ⟨x, y⟩ ≤ f(x) + f*(y).
    pub fn check_fenchel_young(&self, x: &[f64], y: &[f64], f_val: f64) -> bool {
        let inner: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
        let conj = self.eval(y);
        inner <= f_val + conj + 1e-9
    }
}
/// Identifies the class of a convex function for exact conjugate computation.
#[derive(Debug, Clone)]
pub enum FunctionClass {
    /// f(x) = (1/2) ‖x‖² — conjugate is f* = (1/2) ‖y‖².
    SquaredNorm,
    /// f(x) = ‖x‖ (Euclidean norm) — conjugate is indicator of unit ball.
    EuclideanNorm,
    /// f(x) = sum_i x_i log x_i (negative entropy, domain x > 0) — conjugate is sum_i e^{y_i - 1}.
    NegativeEntropy,
    /// f(x) = (1/p) ‖x‖_p^p — conjugate is (1/q) ‖y‖_q^q (1/p + 1/q = 1).
    LpNorm { p: f64 },
    /// Indicator of a hyperbox [lo, hi]^n — conjugate is sum_i max(lo y_i, hi y_i).
    BoxIndicator { lo: f64, hi: f64 },
    /// Unknown / use numerical approximation.
    Numerical,
}
/// Lagrangian duality.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LagrangianDuality {
    pub primal_name: String,
    pub dual_name: String,
    pub duality_gap: f64,
}
impl LagrangianDuality {
    #[allow(dead_code)]
    pub fn new(primal: &str, dual: &str, gap: f64) -> Self {
        Self {
            primal_name: primal.to_string(),
            dual_name: dual.to_string(),
            duality_gap: gap,
        }
    }
    #[allow(dead_code)]
    pub fn strong_duality(&self) -> bool {
        self.duality_gap < 1e-10
    }
    #[allow(dead_code)]
    pub fn slater_condition_description(&self) -> String {
        "Slater's condition: exists strictly feasible point => strong duality".to_string()
    }
    #[allow(dead_code)]
    pub fn kkt_conditions(&self) -> Vec<String> {
        vec![
            "Primal feasibility: Ax + Bz = c, x >= 0".to_string(),
            "Dual feasibility: lambda >= 0".to_string(),
            "Complementary slackness: lambda_i * h_i(x) = 0".to_string(),
            "Stationarity: grad f + sum lambda_i grad h_i = 0".to_string(),
        ]
    }
}
/// Proximal Point Algorithm (PPA): x_{k+1} = prox_{λ_k f}(x_k).
#[derive(Debug, Clone)]
pub struct ProximalPointAlgorithm {
    /// Proximal parameters λ_k > 0.  If shorter than max_iter, last value is repeated.
    pub lambdas: Vec<f64>,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance on ‖x_{k+1} - x_k‖.
    pub tol: f64,
    /// Inner solver step size for the proximal subproblem.
    pub inner_step: f64,
    /// Inner solver max iterations.
    pub inner_iter: usize,
}
impl ProximalPointAlgorithm {
    /// Construct a PPA with a constant proximal parameter λ.
    pub fn constant(lambda: f64, max_iter: usize, tol: f64) -> Self {
        Self {
            lambdas: vec![lambda],
            max_iter,
            tol,
            inner_step: 0.01,
            inner_iter: 300,
        }
    }
    /// Run the proximal point algorithm.  Returns iterates (all x_k).
    pub fn run(&self, f: &ConvexFunction, x0: &[f64]) -> Vec<Vec<f64>> {
        let mut x = x0.to_vec();
        let mut iterates = vec![x.clone()];
        for k in 0..self.max_iter {
            let lambda = self
                .lambdas
                .get(k)
                .copied()
                .unwrap_or_else(|| *self.lambdas.last().unwrap_or(&1.0));
            let cfg = ProxConfig {
                lambda,
                max_iter: self.inner_iter,
                tol: self.tol * 0.01,
                step_size: self.inner_step,
            };
            let new_x = proximal_operator(f, &x, &cfg);
            let delta: f64 = x
                .iter()
                .zip(new_x.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            x = new_x;
            iterates.push(x.clone());
            if delta < self.tol {
                break;
            }
        }
        iterates
    }
}
/// Sublevel set.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SublevelSet {
    pub function_name: String,
    pub level: f64,
}
impl SublevelSet {
    #[allow(dead_code)]
    pub fn new(f: &str, c: f64) -> Self {
        Self {
            function_name: f.to_string(),
            level: c,
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!("C_c = {{x : {}(x) <= {}}}", self.function_name, self.level)
    }
    #[allow(dead_code)]
    pub fn is_convex_if_f_quasiconvex(&self) -> bool {
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProximalOperatorNew {
    pub function_name: String,
    pub regularization: f64,
    pub has_closed_form: bool,
    pub formula: String,
}
#[allow(dead_code)]
impl ProximalOperatorNew {
    pub fn l1_norm(lambda: f64) -> Self {
        ProximalOperatorNew {
            function_name: "||·||_1".to_string(),
            regularization: lambda,
            has_closed_form: true,
            formula: format!(
                "soft_threshold(x, {:.3}) = sign(x)*max(|x|-{:.3}, 0)",
                lambda, lambda
            ),
        }
    }
    pub fn l2_squared(lambda: f64) -> Self {
        ProximalOperatorNew {
            function_name: "(1/2)||·||²".to_string(),
            regularization: lambda,
            has_closed_form: true,
            formula: format!("x / (1 + {:.3})", lambda),
        }
    }
    pub fn indicator_halfspace(lambda: f64) -> Self {
        ProximalOperatorNew {
            function_name: "δ_{x: a^Tx ≤ b}".to_string(),
            regularization: lambda,
            has_closed_form: true,
            formula: "projection onto halfspace: x - max(0, a^Tx-b)/||a||² * a".to_string(),
        }
    }
    pub fn proximal_point_formula(&self) -> String {
        format!(
            "prox_{{λ{}}}(v) = argmin_x [{{{}}}/λ + (1/2)||x-v||²]",
            self.function_name, self.function_name
        )
    }
    pub fn moreau_decomposition(&self) -> String {
        format!(
            "Moreau: v = prox_{{λ{}}}(v) + λ·prox_{{(1/λ){}*}}(v/λ)",
            self.function_name, self.function_name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExtragradientMethod {
    pub step_size: f64,
    pub operator: String,
    pub iterations: usize,
    pub history: Vec<f64>,
}
#[allow(dead_code)]
impl ExtragradientMethod {
    pub fn new(step: f64, op: &str) -> Self {
        ExtragradientMethod {
            step_size: step,
            operator: op.to_string(),
            iterations: 0,
            history: vec![],
        }
    }
    pub fn korpelevich_step_description(&self) -> String {
        format!(
            "Extragradient (Korpelevich): y_k = P_C(x_k - τF(x_k)), x_{{k+1}} = P_C(x_k - τF(y_k)), τ={:.3}",
            self.step_size
        )
    }
    pub fn convergence_condition(&self, lipschitz_l: f64) -> bool {
        self.step_size < 1.0 / lipschitz_l
    }
    pub fn do_step(&mut self, residual: f64) {
        self.history.push(residual);
        self.iterations += 1;
    }
}
/// Mirror descent algorithm.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MirrorDescent {
    pub step_size: f64,
    pub bregman_divergence: String,
    pub num_iterations: usize,
}
impl MirrorDescent {
    #[allow(dead_code)]
    pub fn with_entropy(eta: f64, iters: usize) -> Self {
        Self {
            step_size: eta,
            bregman_divergence: "KL divergence (entropic MD = Exponentiated Gradient)".to_string(),
            num_iterations: iters,
        }
    }
    #[allow(dead_code)]
    pub fn convergence_rate(&self, diameter: f64, lipschitz: f64) -> f64 {
        lipschitz * diameter / (self.num_iterations as f64).sqrt()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConvexConjugate {
    pub function_name: String,
    pub domain: String,
    pub conjugate_name: String,
    pub conjugate_domain: String,
    pub is_proper: bool,
    pub is_closed: bool,
}
#[allow(dead_code)]
impl ConvexConjugate {
    pub fn new(name: &str, domain: &str, conj_name: &str, conj_domain: &str) -> Self {
        ConvexConjugate {
            function_name: name.to_string(),
            domain: domain.to_string(),
            conjugate_name: conj_name.to_string(),
            conjugate_domain: conj_domain.to_string(),
            is_proper: true,
            is_closed: true,
        }
    }
    pub fn quadratic_conjugate() -> Self {
        ConvexConjugate {
            function_name: "||x||²/2".to_string(),
            domain: "R^n".to_string(),
            conjugate_name: "||y||²/2".to_string(),
            conjugate_domain: "R^n".to_string(),
            is_proper: true,
            is_closed: true,
        }
    }
    pub fn indicator_conjugate(set: &str) -> Self {
        ConvexConjugate {
            function_name: format!("δ_{{{}}}", set),
            domain: set.to_string(),
            conjugate_name: format!("h_{{{}}}", set),
            conjugate_domain: "R^n".to_string(),
            is_proper: true,
            is_closed: true,
        }
    }
    pub fn biconjugate_equals_f(&self) -> bool {
        self.is_proper && self.is_closed
    }
    pub fn fenchel_moreau_theorem(&self) -> String {
        if self.biconjugate_equals_f() {
            format!(
                "Fenchel-Moreau: ({})** = {} (proper, closed, convex)",
                self.function_name, self.function_name
            )
        } else {
            format!(
                "({})** = convex closure of {}",
                self.function_name, self.function_name
            )
        }
    }
    pub fn young_fenchel_inequality(&self) -> String {
        format!(
            "Young-Fenchel: {}(x) + {}(y) ≥ ⟨x,y⟩ with equality at x = ∂{}(y)",
            self.function_name, self.conjugate_name, self.conjugate_name
        )
    }
}
/// Finds a separating hyperplane between two finite polytopes using a simple
/// SVM-inspired hard-margin approach (gradient on the margin).
pub struct SeparatingHyperplaneFinder {
    /// Learning rate for gradient ascent on margin.
    pub lr: f64,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Convergence tolerance.
    pub tol: f64,
}
impl SeparatingHyperplaneFinder {
    /// Create a new finder.
    pub fn new(lr: f64, max_iter: usize, tol: f64) -> Self {
        Self { lr, max_iter, tol }
    }
    /// Find a separating hyperplane between point sets A and B.
    /// Returns `Some(Hyperplane)` if separated, `None` if the sets appear non-separable.
    pub fn find(&self, a_points: &[Vec<f64>], b_points: &[Vec<f64>]) -> Option<Hyperplane> {
        if a_points.is_empty() || b_points.is_empty() {
            return None;
        }
        let initial = compute_separating_hyperplane(a_points, b_points)?;
        let n = initial.normal.len();
        let mut w = initial.normal.clone();
        let mut b = initial.offset;
        for _ in 0..self.max_iter {
            let mut dw = vec![0.0; n];
            let mut db = 0.0_f64;
            let mut total_violation = 0.0_f64;
            for x in a_points {
                let margin: f64 = w.iter().zip(x.iter()).map(|(wi, xi)| wi * xi).sum::<f64>() - b;
                if margin < 1.0 {
                    for i in 0..n {
                        dw[i] += x[i];
                    }
                    db -= 1.0;
                    total_violation += (1.0 - margin).abs();
                }
            }
            for x in b_points {
                let margin: f64 = w.iter().zip(x.iter()).map(|(wi, xi)| wi * xi).sum::<f64>() - b;
                if margin > -1.0 {
                    for i in 0..n {
                        dw[i] -= x[i];
                    }
                    db += 1.0;
                    total_violation += (1.0 + margin).abs();
                }
            }
            if total_violation < self.tol {
                break;
            }
            for i in 0..n {
                w[i] += self.lr * dw[i];
            }
            b += self.lr * db;
        }
        let norm: f64 = w.iter().map(|wi| wi * wi).sum::<f64>().sqrt();
        if norm < 1e-12 {
            return None;
        }
        let unit_w: Vec<f64> = w.iter().map(|wi| wi / norm).collect();
        let unit_b = b / norm;
        let hp = Hyperplane::new(unit_w, unit_b);
        if hp.separates(a_points, b_points) {
            Some(hp)
        } else {
            let fb = compute_separating_hyperplane(a_points, b_points)?;
            if fb.separates(a_points, b_points) {
                Some(fb)
            } else {
                None
            }
        }
    }
}
/// Epigraph of a function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Epigraph {
    pub function_name: String,
}
impl Epigraph {
    #[allow(dead_code)]
    pub fn new(f: &str) -> Self {
        Self {
            function_name: f.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "epi({}) = {{(x, t) : {}(x) <= t}}",
            self.function_name, self.function_name
        )
    }
    #[allow(dead_code)]
    pub fn f_convex_iff_epi_convex(&self) -> bool {
        true
    }
}
/// Legendre-Fenchel conjugate.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FenchelConjugate {
    pub function_name: String,
    pub conjugate_name: String,
}
impl FenchelConjugate {
    #[allow(dead_code)]
    pub fn new(f: &str) -> Self {
        Self {
            function_name: f.to_string(),
            conjugate_name: format!("{}*", f),
        }
    }
    #[allow(dead_code)]
    pub fn definition(&self) -> String {
        format!(
            "{}*(y) = sup_x <x, y> - {}(x)",
            self.function_name, self.function_name
        )
    }
    #[allow(dead_code)]
    pub fn biconjugate_is_convex_hull(&self) -> String {
        format!(
            "{}** = closed convex hull of {}",
            self.function_name, self.function_name
        )
    }
    #[allow(dead_code)]
    pub fn fenchel_inequality(&self) -> String {
        format!(
            "<x, y> <= {}(x) + {}*(y) for all x, y",
            self.function_name, self.function_name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariationalInequality {
    pub operator_name: String,
    pub constraint_set: String,
    pub is_monotone: bool,
    pub is_strongly_monotone: bool,
    pub strong_monotonicity: f64,
}
#[allow(dead_code)]
impl VariationalInequality {
    pub fn new(op: &str, set: &str, monotone: bool) -> Self {
        VariationalInequality {
            operator_name: op.to_string(),
            constraint_set: set.to_string(),
            is_monotone: monotone,
            is_strongly_monotone: false,
            strong_monotonicity: 0.0,
        }
    }
    pub fn strongly_monotone(op: &str, set: &str, alpha: f64) -> Self {
        VariationalInequality {
            operator_name: op.to_string(),
            constraint_set: set.to_string(),
            is_monotone: true,
            is_strongly_monotone: true,
            strong_monotonicity: alpha,
        }
    }
    pub fn vi_formulation(&self) -> String {
        format!(
            "Find x* ∈ {} s.t. ⟨F(x*), x - x*⟩ ≥ 0 ∀x ∈ {} (F = {})",
            self.constraint_set, self.constraint_set, self.operator_name
        )
    }
    pub fn stampacchia_existence(&self) -> String {
        if self.is_monotone {
            "Stampacchia theorem: monotone + coercive → VI has solution".to_string()
        } else {
            "No monotonicity: existence not guaranteed by Stampacchia".to_string()
        }
    }
    pub fn unique_solution_exists(&self) -> bool {
        self.is_strongly_monotone
    }
}
/// Subgradient descent for non-smooth convex minimisation.
#[derive(Debug, Clone)]
pub struct SubgradientMethod {
    /// Step size schedule.
    pub schedule: StepSchedule,
    /// Maximum number of iterations.
    pub max_iter: usize,
    /// Return the best iterate (lowest function value seen).
    pub track_best: bool,
}
impl SubgradientMethod {
    /// Create a new subgradient method with the given schedule.
    pub fn new(schedule: StepSchedule, max_iter: usize) -> Self {
        Self {
            schedule,
            max_iter,
            track_best: true,
        }
    }
    /// Run subgradient descent starting from `x0`.
    /// Returns the best iterate and the sequence of function values.
    pub fn run(&self, f: &ConvexFunction, x0: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let n = x0.len();
        let mut x = x0.to_vec();
        let mut best_x = x.clone();
        let mut best_val = f.eval(&x);
        let mut history = Vec::with_capacity(self.max_iter);
        for k in 0..self.max_iter {
            let val = f.eval(&x);
            history.push(val);
            if val < best_val {
                best_val = val;
                best_x = x.clone();
            }
            let g = f.gradient(&x);
            let g_norm_sq: f64 = g.iter().map(|gi| gi * gi).sum();
            if g_norm_sq < 1e-30 {
                break;
            }
            let eta = match self.schedule {
                StepSchedule::Constant(eta) => eta,
                StepSchedule::DiminishingSqrt(eta) => eta / ((k + 1) as f64).sqrt(),
                StepSchedule::Polyak { f_star } => ((val - f_star).max(0.0)) / g_norm_sq,
            };
            let mut new_x = vec![0.0; n];
            for i in 0..n {
                new_x[i] = x[i] - eta * g[i];
            }
            x = new_x;
        }
        let best = if self.track_best { best_x } else { x };
        (best, history)
    }
}
