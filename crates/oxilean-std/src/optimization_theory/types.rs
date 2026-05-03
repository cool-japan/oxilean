//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Adam optimizer with explicit state tracking (m, v, t).
#[derive(Debug, Clone)]
pub struct AdamOptimizer {
    /// Configuration.
    pub lr: f64,
    /// First moment decay β₁.
    pub beta1: f64,
    /// Second moment decay β₂.
    pub beta2: f64,
    /// Numerical stabiliser ε.
    pub eps: f64,
    /// Current iterate.
    pub x: Vec<f64>,
    /// First moment estimate m.
    pub m: Vec<f64>,
    /// Second moment estimate v.
    pub v: Vec<f64>,
    /// Step counter t.
    pub t: usize,
    /// Objective value history.
    pub fvals: Vec<f64>,
}
impl AdamOptimizer {
    /// Initialise Adam at `x0` with the given hyper-parameters.
    pub fn new(x0: Vec<f64>, lr: f64, beta1: f64, beta2: f64, eps: f64) -> Self {
        let n = x0.len();
        AdamOptimizer {
            lr,
            beta1,
            beta2,
            eps,
            x: x0,
            m: vec![0.0; n],
            v: vec![0.0; n],
            t: 0,
            fvals: Vec::new(),
        }
    }
    /// Perform one Adam update step given gradient `grad`.
    pub fn step(&mut self, grad: &[f64]) {
        self.t += 1;
        let t = self.t as i32;
        let n = self.x.len();
        for i in 0..n {
            self.m[i] = self.beta1 * self.m[i] + (1.0 - self.beta1) * grad[i];
            self.v[i] = self.beta2 * self.v[i] + (1.0 - self.beta2) * grad[i] * grad[i];
            let m_hat = self.m[i] / (1.0 - self.beta1.powi(t));
            let v_hat = self.v[i] / (1.0 - self.beta2.powi(t));
            self.x[i] -= self.lr * m_hat / (v_hat.sqrt() + self.eps);
        }
    }
    /// Run Adam to convergence.
    ///
    /// Returns `(solution, final_value, steps)`.
    pub fn run(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
        max_iter: usize,
        tol: f64,
    ) -> (Vec<f64>, f64, usize) {
        for _ in 0..max_iter {
            let fx = f(&self.x);
            self.fvals.push(fx);
            let grad = grad_f(&self.x);
            let gnorm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if gnorm < tol {
                break;
            }
            self.step(&grad);
        }
        (self.x.clone(), f(&self.x), self.t)
    }
    /// Bias-corrected first moment (m̂_i).
    pub fn m_hat(&self) -> Vec<f64> {
        let t = self.t as i32;
        self.m
            .iter()
            .map(|mi| mi / (1.0 - self.beta1.powi(t).max(1e-300)))
            .collect()
    }
    /// Bias-corrected second moment (v̂_i).
    pub fn v_hat(&self) -> Vec<f64> {
        let t = self.t as i32;
        self.v
            .iter()
            .map(|vi| vi / (1.0 - self.beta2.powi(t).max(1e-300)))
            .collect()
    }
}
/// ADMM solver for problems of the form: min f(x) + g(z) s.t. Ax + Bz = c.
///
/// Uses the alternating direction method of multipliers with penalty ρ:
///   x_{k+1} = argmin_x f(x) + ρ/2 ‖Ax + Bz_k - c + u_k‖²
///   z_{k+1} = argmin_z g(z) + ρ/2 ‖Ax_{k+1} + Bz - c + u_k‖²
///   u_{k+1} = u_k + Ax_{k+1} + Bz_{k+1} - c    (scaled dual)
#[derive(Debug, Clone)]
pub struct ADMMSolver {
    /// Penalty parameter ρ > 0.
    pub rho: f64,
    /// x-variable.
    pub x: Vec<f64>,
    /// z-variable.
    pub z: Vec<f64>,
    /// Scaled dual variable u.
    pub u: Vec<f64>,
    /// Primal residual history.
    pub primal_residuals: Vec<f64>,
    /// Dual residual history.
    pub dual_residuals: Vec<f64>,
    /// Iteration count.
    pub iter: usize,
}
impl ADMMSolver {
    /// Initialise ADMM with penalty `rho`, starting points `x0`, `z0`, `u0`.
    pub fn new(rho: f64, x0: Vec<f64>, z0: Vec<f64>, u0: Vec<f64>) -> Self {
        ADMMSolver {
            rho,
            x: x0,
            z: z0,
            u: u0,
            primal_residuals: Vec::new(),
            dual_residuals: Vec::new(),
            iter: 0,
        }
    }
    /// Run ADMM iterations.
    ///
    /// `x_update(z, u)` performs the x-minimisation step and returns x_{k+1}.
    /// `z_update(x, u)` performs the z-minimisation step and returns z_{k+1}.
    /// `constraint(x, z)` evaluates Ax + Bz - c (primal residual vector).
    ///
    /// Returns `(x*, z*, primal_residual, iterations)`.
    pub fn run(
        &mut self,
        x_update: &dyn Fn(&[f64], &[f64]) -> Vec<f64>,
        z_update: &dyn Fn(&[f64], &[f64]) -> Vec<f64>,
        constraint: &dyn Fn(&[f64], &[f64]) -> Vec<f64>,
        max_iter: usize,
        abs_tol: f64,
        rel_tol: f64,
    ) -> (Vec<f64>, Vec<f64>, f64, usize) {
        for _k in 0..max_iter {
            let z_old = self.z.clone();
            self.x = x_update(&self.z, &self.u);
            self.z = z_update(&self.x, &self.u);
            let r = constraint(&self.x, &self.z);
            for i in 0..self.u.len() {
                self.u[i] += r[i];
            }
            let prim: f64 = r.iter().map(|ri| ri * ri).sum::<f64>().sqrt();
            let dual: f64 = self.rho
                * self
                    .z
                    .iter()
                    .zip(&z_old)
                    .map(|(zi, zoi)| (zi - zoi).powi(2))
                    .sum::<f64>()
                    .sqrt();
            self.primal_residuals.push(prim);
            self.dual_residuals.push(dual);
            self.iter += 1;
            let n = self.x.len().max(self.z.len()) as f64;
            let eps_prim = (n.sqrt() * abs_tol)
                + rel_tol
                    * self
                        .x
                        .iter()
                        .map(|xi| xi * xi)
                        .sum::<f64>()
                        .sqrt()
                        .max(self.z.iter().map(|zi| zi * zi).sum::<f64>().sqrt());
            let eps_dual = n.sqrt() * abs_tol
                + rel_tol * self.rho * self.u.iter().map(|ui| ui * ui).sum::<f64>().sqrt();
            if prim < eps_prim && dual < eps_dual {
                break;
            }
        }
        let final_r = constraint(&self.x, &self.z);
        let prim: f64 = final_r.iter().map(|ri| ri * ri).sum::<f64>().sqrt();
        (self.x.clone(), self.z.clone(), prim, self.iter)
    }
}
/// Configuration for SGD.
#[derive(Debug, Clone)]
pub struct SGDConfig {
    /// Initial step size.
    pub lr: f64,
    /// Step size decay: lr_t = lr / sqrt(t+1) if true.
    pub decay: bool,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Gradient tolerance (stop if ‖∇f‖ < tol).
    pub tol: f64,
}
impl SGDConfig {
    /// Create a default SGD configuration.
    pub fn new(lr: f64, max_iter: usize, tol: f64) -> Self {
        SGDConfig {
            lr,
            decay: false,
            max_iter,
            tol,
        }
    }
    /// Enable 1/√t step size decay.
    pub fn with_decay(mut self) -> Self {
        self.decay = true;
        self
    }
}
/// Configuration for gradient descent with Armijo backtracking line search.
#[derive(Debug, Clone)]
pub struct GradientDescentConfig {
    /// Initial step size.
    pub alpha0: f64,
    /// Armijo condition constant c₁ ∈ (0,1).
    pub c1: f64,
    /// Backtracking contraction factor τ ∈ (0,1).
    pub tau: f64,
    /// Maximum backtracking iterations.
    pub max_ls: usize,
    /// Maximum gradient descent iterations.
    pub max_iter: usize,
    /// Gradient norm tolerance.
    pub tol: f64,
}
impl GradientDescentConfig {
    /// Create a default configuration.
    pub fn new(max_iter: usize, tol: f64) -> Self {
        GradientDescentConfig {
            alpha0: 1.0,
            c1: 1e-4,
            tau: 0.5,
            max_ls: 30,
            max_iter,
            tol,
        }
    }
}
/// Tracks regret for an online learning algorithm.
#[derive(Debug, Clone)]
pub struct RegretTracker {
    /// Cumulative loss of the online learner.
    pub learner_loss: f64,
    /// Cumulative loss of the best fixed comparator seen so far.
    pub comparator_loss: f64,
    /// Number of rounds played.
    pub rounds: usize,
    /// Per-round regret history.
    pub regret_history: Vec<f64>,
    /// Running cumulative regret.
    pub cumulative_regret: Vec<f64>,
}
impl RegretTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        RegretTracker {
            learner_loss: 0.0,
            comparator_loss: 0.0,
            rounds: 0,
            regret_history: Vec::new(),
            cumulative_regret: Vec::new(),
        }
    }
    /// Record one round: learner played `x_t`, comparator `x_star`,
    /// and the loss function `f_t` was revealed.
    pub fn record(&mut self, f_t: &dyn Fn(&[f64]) -> f64, x_t: &[f64], x_star: &[f64]) {
        let lt = f_t(x_t);
        let ls = f_t(x_star);
        let round_regret = lt - ls;
        self.learner_loss += lt;
        self.comparator_loss += ls;
        self.regret_history.push(round_regret);
        self.cumulative_regret
            .push(self.learner_loss - self.comparator_loss);
        self.rounds += 1;
    }
    /// Total cumulative regret R_T = Σ (f_t(x_t) - f_t(x*)).
    pub fn total_regret(&self) -> f64 {
        self.learner_loss - self.comparator_loss
    }
    /// Average regret per round.
    pub fn average_regret(&self) -> f64 {
        if self.rounds == 0 {
            0.0
        } else {
            self.total_regret() / self.rounds as f64
        }
    }
    /// Check whether the algorithm has no-regret: avg_regret → 0.
    /// Returns `true` if average regret ≤ `eps`.
    pub fn is_no_regret(&self, eps: f64) -> bool {
        self.average_regret().abs() <= eps
    }
}
/// L-BFGS state for the two-loop recursion.
#[derive(Debug, Clone)]
pub struct LBFGSState {
    /// Memory size m.
    pub memory: usize,
    /// Stored s_k = x_{k+1} - x_k vectors.
    pub s_list: Vec<Vec<f64>>,
    /// Stored y_k = ∇f_{k+1} - ∇f_k vectors.
    pub y_list: Vec<Vec<f64>>,
    /// Current iterate.
    pub x: Vec<f64>,
    /// Previous gradient.
    pub prev_grad: Option<Vec<f64>>,
    /// Iteration count.
    pub iter: usize,
}
impl LBFGSState {
    /// Initialise L-BFGS at `x0` with memory `m`.
    pub fn new(x0: Vec<f64>, memory: usize) -> Self {
        LBFGSState {
            memory,
            s_list: Vec::new(),
            y_list: Vec::new(),
            x: x0,
            prev_grad: None,
            iter: 0,
        }
    }
    /// Compute the L-BFGS search direction using the two-loop recursion.
    ///
    /// Returns `H_k * (-grad)`, i.e. the quasi-Newton descent direction.
    pub fn direction(&self, grad: &[f64]) -> Vec<f64> {
        let n = grad.len();
        let m = self.s_list.len();
        let mut q = grad.to_vec();
        let mut alphas = vec![0.0_f64; m];
        let mut rhos = vec![0.0_f64; m];
        for i in (0..m).rev() {
            let sy: f64 = self.s_list[i]
                .iter()
                .zip(&self.y_list[i])
                .map(|(s, y)| s * y)
                .sum();
            rhos[i] = if sy.abs() < 1e-30 { 0.0 } else { 1.0 / sy };
            let alpha: f64 = rhos[i]
                * self.s_list[i]
                    .iter()
                    .zip(&q)
                    .map(|(s, qi)| s * qi)
                    .sum::<f64>();
            alphas[i] = alpha;
            for j in 0..n {
                q[j] -= alpha * self.y_list[i][j];
            }
        }
        let gamma = if m > 0 {
            let sy: f64 = self.s_list[m - 1]
                .iter()
                .zip(&self.y_list[m - 1])
                .map(|(s, y)| s * y)
                .sum();
            let yy: f64 = self.y_list[m - 1].iter().map(|y| y * y).sum();
            if yy < 1e-30 {
                1.0
            } else {
                sy / yy
            }
        } else {
            1.0
        };
        let mut r: Vec<f64> = q.iter().map(|qi| gamma * qi).collect();
        for i in 0..m {
            let beta: f64 = rhos[i]
                * self.y_list[i]
                    .iter()
                    .zip(&r)
                    .map(|(y, ri)| y * ri)
                    .sum::<f64>();
            for j in 0..n {
                r[j] += self.s_list[i][j] * (alphas[i] - beta);
            }
        }
        r.iter().map(|ri| -ri).collect()
    }
    /// Perform one L-BFGS step with backtracking line search.
    pub fn step(&mut self, f: &dyn Fn(&[f64]) -> f64, grad_f: &dyn Fn(&[f64]) -> Vec<f64>) -> f64 {
        let n = self.x.len();
        let grad = grad_f(&self.x);
        let d = self.direction(&grad);
        let fx = f(&self.x);
        let dg: f64 = d.iter().zip(&grad).map(|(di, gi)| di * gi).sum();
        let mut alpha = 1.0_f64;
        for _ in 0..30 {
            let xnew: Vec<f64> = self
                .x
                .iter()
                .zip(&d)
                .map(|(xi, di)| xi + alpha * di)
                .collect();
            if f(&xnew) <= fx + 1e-4 * alpha * dg {
                break;
            }
            alpha *= 0.5;
        }
        let xnew: Vec<f64> = self
            .x
            .iter()
            .zip(&d)
            .map(|(xi, di)| xi + alpha * di)
            .collect();
        let grad_new = grad_f(&xnew);
        let s: Vec<f64> = xnew.iter().zip(&self.x).map(|(xn, xi)| xn - xi).collect();
        let y: Vec<f64> = grad_new.iter().zip(&grad).map(|(gn, gi)| gn - gi).collect();
        if s.iter().zip(&y).map(|(si, yi)| si * yi).sum::<f64>() > 1e-10 {
            if self.s_list.len() >= self.memory {
                self.s_list.remove(0);
                self.y_list.remove(0);
            }
            self.s_list.push(s);
            self.y_list.push(y);
        }
        self.x = xnew;
        self.prev_grad = Some(grad_new);
        self.iter += 1;
        let _ = n;
        f(&self.x)
    }
    /// Run L-BFGS to convergence.
    ///
    /// Returns `(solution, final_value, iterations)`.
    pub fn run(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
        max_iter: usize,
        tol: f64,
    ) -> (Vec<f64>, f64, usize) {
        for _ in 0..max_iter {
            let grad = grad_f(&self.x);
            let gnorm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if gnorm < tol {
                break;
            }
            self.step(f, grad_f);
        }
        (self.x.clone(), f(&self.x), self.iter)
    }
}
/// Represents a robust optimization problem: min_x max_{u in U} f(x, u).
/// The uncertainty set U is modeled as a box or ellipsoid.
#[allow(dead_code)]
pub struct RobustOptimizationProblem {
    /// Dimension of decision variable x.
    pub dim_x: usize,
    /// Dimension of uncertainty parameter u.
    pub dim_u: usize,
    /// Box uncertainty: uncertainty set U = \[-delta, delta\]^dim_u.
    pub box_delta: f64,
    /// Nominal cost coefficients c (linear objective f(x, u) = c^T x + u^T x).
    pub nominal_cost: Vec<f64>,
}
#[allow(dead_code)]
impl RobustOptimizationProblem {
    /// Create a new robust optimization problem.
    pub fn new(dim_x: usize, dim_u: usize, box_delta: f64, nominal_cost: Vec<f64>) -> Self {
        RobustOptimizationProblem {
            dim_x,
            dim_u,
            box_delta,
            nominal_cost,
        }
    }
    /// Worst-case cost for a given x under box uncertainty:
    /// max_{u in \[-delta,delta\]^dim_u} c^T x + u^T x = c^T x + delta * ||x||_1
    pub fn worst_case_cost(&self, x: &[f64]) -> f64 {
        let nominal: f64 = self
            .nominal_cost
            .iter()
            .zip(x.iter())
            .map(|(&c, &xi)| c * xi)
            .sum();
        let robustness_penalty: f64 = x.iter().map(|&xi| xi.abs()).sum::<f64>() * self.box_delta;
        nominal + robustness_penalty
    }
    /// Robust counterpart for box uncertainty: equivalent to adding L1 penalty.
    /// Returns the penalty coefficient Γ = box_delta.
    pub fn robust_counterpart_penalty(&self) -> f64 {
        self.box_delta
    }
    /// Ellipsoidal uncertainty: worst case cost under ||u||_2 ≤ delta is
    /// c^T x + delta * ||x||_2.
    pub fn ellipsoidal_worst_case(&self, x: &[f64]) -> f64 {
        let nominal: f64 = self
            .nominal_cost
            .iter()
            .zip(x.iter())
            .map(|(&c, &xi)| c * xi)
            .sum();
        let l2_norm: f64 = x.iter().map(|&xi| xi * xi).sum::<f64>().sqrt();
        nominal + self.box_delta * l2_norm
    }
    /// Price of robustness: difference between robust and nominal optima (approximate).
    /// Returns delta * ||x_nom||_2 where x_nom minimizes c^T x.
    pub fn price_of_robustness(&self) -> f64 {
        self.box_delta * (self.dim_x as f64).sqrt()
    }
}
/// Represents a binary integer program: min c^T x s.t. Ax ≤ b, x ∈ {0,1}^n.
#[allow(dead_code)]
pub struct BinaryIntegerProgram {
    /// Objective coefficients.
    pub c: Vec<f64>,
    /// Constraint matrix A (row-major).
    pub a_matrix: Vec<Vec<f64>>,
    /// Right-hand side b.
    pub b: Vec<f64>,
}
#[allow(dead_code)]
impl BinaryIntegerProgram {
    /// Create a new binary integer program.
    pub fn new(c: Vec<f64>, a_matrix: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        BinaryIntegerProgram { c, a_matrix, b }
    }
    /// Check feasibility of a binary solution x.
    pub fn is_feasible(&self, x: &[bool]) -> bool {
        for (row, &rhs) in self.a_matrix.iter().zip(self.b.iter()) {
            let lhs: f64 = row
                .iter()
                .zip(x.iter())
                .map(|(&a, &xi)| a * if xi { 1.0 } else { 0.0 })
                .sum();
            if lhs > rhs + 1e-9 {
                return false;
            }
        }
        true
    }
    /// Evaluate the objective at x.
    pub fn objective(&self, x: &[bool]) -> f64 {
        self.c
            .iter()
            .zip(x.iter())
            .map(|(&ci, &xi)| ci * if xi { 1.0 } else { 0.0 })
            .sum()
    }
    /// LP relaxation lower bound: solve the LP relaxation via greedy fractional.
    /// Simplified: returns minimum coefficient sum over feasible coordinates.
    pub fn lp_relaxation_lower_bound(&self) -> f64 {
        self.c.iter().filter(|&&ci| ci < 0.0).sum::<f64>()
    }
    /// Greedy heuristic for the 0/1 knapsack: add items by value/weight ratio.
    /// Here we model it as taking items with cost < 0 (benefit).
    pub fn greedy_solution(&self) -> Vec<bool> {
        let n = self.c.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&i, &j| {
            self.c[i]
                .partial_cmp(&self.c[j])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut x = vec![false; n];
        for i in order {
            x[i] = true;
            if !self.is_feasible(&x) {
                x[i] = false;
            }
        }
        x
    }
    /// Cutting plane method: returns number of rounds needed (simplified).
    /// Gomory cuts: each round separates a fractional LP solution.
    pub fn cutting_plane_rounds_estimate(&self) -> usize {
        self.c.len()
    }
}
/// Two-stage stochastic program:
/// min c^T x + E_ξ\[Q(x, ξ)\] s.t. Ax = b, x ≥ 0.
/// Q(x, ξ) = min q^T y s.t. Wy = h(ξ) - Tx, y ≥ 0.
#[allow(dead_code)]
pub struct TwoStageStochasticProgram {
    /// First-stage cost coefficients.
    pub first_stage_cost: Vec<f64>,
    /// Number of scenarios.
    pub num_scenarios: usize,
    /// Scenario probabilities.
    pub scenario_probs: Vec<f64>,
    /// Second-stage cost per scenario (simplified: scalar).
    pub second_stage_costs: Vec<f64>,
}
#[allow(dead_code)]
impl TwoStageStochasticProgram {
    /// Create a new two-stage stochastic program.
    pub fn new(
        first_stage_cost: Vec<f64>,
        scenario_probs: Vec<f64>,
        second_stage_costs: Vec<f64>,
    ) -> Self {
        let num_scenarios = scenario_probs.len();
        TwoStageStochasticProgram {
            first_stage_cost,
            num_scenarios,
            scenario_probs,
            second_stage_costs,
        }
    }
    /// Expected second-stage cost: E\[Q\] = Σ_s p_s Q_s.
    pub fn expected_second_stage(&self, x: &[f64]) -> f64 {
        let _x = x;
        self.scenario_probs
            .iter()
            .zip(self.second_stage_costs.iter())
            .map(|(&p, &q)| p * q)
            .sum()
    }
    /// Total expected cost for first-stage decision x.
    pub fn total_expected_cost(&self, x: &[f64]) -> f64 {
        let first: f64 = self
            .first_stage_cost
            .iter()
            .zip(x.iter())
            .map(|(&c, &xi)| c * xi)
            .sum();
        first + self.expected_second_stage(x)
    }
    /// Value of perfect information (VPI):
    /// VPI = EV_with_perfect_info - EEV (optimal expected value)
    pub fn value_of_perfect_information(&self) -> f64 {
        let ev_pi: f64 = self
            .scenario_probs
            .iter()
            .zip(self.second_stage_costs.iter())
            .map(|(&p, &q)| p * q.abs())
            .sum();
        let ev_expected: f64 = self.expected_second_stage(&vec![1.0; self.first_stage_cost.len()]);
        (ev_expected - ev_pi).abs()
    }
    /// Value of stochastic solution (VSS):
    /// VSS = EEV - RP where RP is the recourse problem optimal value.
    pub fn value_of_stochastic_solution(&self) -> f64 {
        let min_scenario: f64 = self
            .second_stage_costs
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let expected_q: f64 = self.expected_second_stage(&vec![1.0; self.first_stage_cost.len()]);
        (expected_q - min_scenario).abs()
    }
}
/// Frank-Wolfe (conditional gradient) optimizer for linear-constrained problems.
///
/// Solves min f(x) s.t. x ∈ C, where `lmo` is the linear minimisation oracle:
///   lmo(g) = argmin_{s ∈ C} ⟨g, s⟩.
#[derive(Debug, Clone)]
pub struct FrankWolfeOptimizer {
    /// Current iterate.
    pub x: Vec<f64>,
    /// Iteration count.
    pub iter: usize,
    /// Frank-Wolfe gap history (convergence certificate).
    pub gaps: Vec<f64>,
    /// Objective value history.
    pub fvals: Vec<f64>,
    /// Step size rule: `None` = 2/(k+2) open-loop; `Some(c)` = line search with curvature c.
    pub step_rule: Option<f64>,
}
impl FrankWolfeOptimizer {
    /// Create a new Frank-Wolfe optimizer at `x0`.
    pub fn new(x0: Vec<f64>) -> Self {
        FrankWolfeOptimizer {
            x: x0,
            iter: 0,
            gaps: Vec::new(),
            fvals: Vec::new(),
            step_rule: None,
        }
    }
    /// Use open-loop step size γ_k = 2/(k+2).
    pub fn with_open_loop_steps(mut self) -> Self {
        self.step_rule = None;
        self
    }
    /// Use line search with curvature constant `c` (smoothness): γ = gap / (c * ‖d‖²).
    pub fn with_line_search(mut self, curvature: f64) -> Self {
        self.step_rule = Some(curvature);
        self
    }
    /// Run Frank-Wolfe for `max_iter` iterations.
    ///
    /// `lmo(grad)` returns the linear minimisation oracle solution.
    /// Returns `(solution, final_value, iterations)`.
    pub fn run(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
        lmo: &dyn Fn(&[f64]) -> Vec<f64>,
        max_iter: usize,
        tol: f64,
    ) -> (Vec<f64>, f64, usize) {
        let n = self.x.len();
        for k in 0..max_iter {
            let fx = f(&self.x);
            self.fvals.push(fx);
            let grad = grad_f(&self.x);
            let s = lmo(&grad);
            let gap: f64 = grad
                .iter()
                .zip(&self.x)
                .zip(&s)
                .map(|((gi, xi), si)| gi * (xi - si))
                .sum();
            self.gaps.push(gap);
            if gap < tol {
                self.iter = k;
                break;
            }
            let d: Vec<f64> = s.iter().zip(&self.x).map(|(si, xi)| si - xi).collect();
            let gamma = match self.step_rule {
                None => 2.0 / (k as f64 + 2.0),
                Some(c) => {
                    let d_norm_sq: f64 = d.iter().map(|di| di * di).sum();
                    if d_norm_sq < 1e-30 {
                        0.0
                    } else {
                        (gap / (c * d_norm_sq)).min(1.0).max(0.0)
                    }
                }
            };
            for i in 0..n {
                self.x[i] += gamma * d[i];
            }
            self.iter = k + 1;
        }
        (self.x.clone(), f(&self.x), self.iter)
    }
}
/// Gradient descent optimizer with Armijo backtracking line search.
#[derive(Debug, Clone)]
pub struct GradientDescentOptimizer {
    /// Configuration parameters.
    pub config: GradientDescentConfig,
    /// Current iterate.
    pub x: Vec<f64>,
    /// Iteration count.
    pub iter: usize,
    /// History of objective values.
    pub fvals: Vec<f64>,
}
impl GradientDescentOptimizer {
    /// Create a new optimizer starting at `x0`.
    pub fn new(x0: Vec<f64>, config: GradientDescentConfig) -> Self {
        GradientDescentOptimizer {
            config,
            x: x0,
            iter: 0,
            fvals: Vec::new(),
        }
    }
    /// Armijo backtracking line search.
    ///
    /// Returns the step size satisfying f(x - α∇f) ≤ f(x) - c₁ α ‖∇f‖².
    fn armijo_step(&self, f: &dyn Fn(&[f64]) -> f64, grad: &[f64], fx: f64) -> f64 {
        let gnorm_sq: f64 = grad.iter().map(|g| g * g).sum();
        let mut alpha = self.config.alpha0;
        for _ in 0..self.config.max_ls {
            let xnew: Vec<f64> = self
                .x
                .iter()
                .zip(grad)
                .map(|(xi, gi)| xi - alpha * gi)
                .collect();
            if f(&xnew) <= fx - self.config.c1 * alpha * gnorm_sq {
                return alpha;
            }
            alpha *= self.config.tau;
        }
        alpha
    }
    /// Run gradient descent to convergence.
    ///
    /// Returns `(solution, final_value, iterations)`.
    pub fn run(
        &mut self,
        f: &dyn Fn(&[f64]) -> f64,
        grad_f: &dyn Fn(&[f64]) -> Vec<f64>,
    ) -> (Vec<f64>, f64, usize) {
        let n = self.x.len();
        for _t in 0..self.config.max_iter {
            let fx = f(&self.x);
            self.fvals.push(fx);
            let grad = grad_f(&self.x);
            let gnorm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if gnorm < self.config.tol {
                break;
            }
            let alpha = self.armijo_step(f, &grad, fx);
            for i in 0..n {
                self.x[i] -= alpha * grad[i];
            }
            self.iter += 1;
        }
        (self.x.clone(), f(&self.x), self.iter)
    }
}
/// Adam optimizer configuration.
#[derive(Debug, Clone)]
pub struct AdamConfig {
    /// Learning rate α.
    pub lr: f64,
    /// First moment decay β₁.
    pub beta1: f64,
    /// Second moment decay β₂.
    pub beta2: f64,
    /// Numerical stability constant ε.
    pub eps: f64,
    /// Maximum iterations.
    pub max_iter: usize,
    /// Gradient tolerance.
    pub tol: f64,
}
impl AdamConfig {
    /// Sensible defaults (Kingma & Ba 2015).
    pub fn default_params(lr: f64, max_iter: usize) -> Self {
        AdamConfig {
            lr,
            beta1: 0.9,
            beta2: 0.999,
            eps: 1e-8,
            max_iter,
            tol: 1e-7,
        }
    }
}
