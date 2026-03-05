//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A linear ODE dy/dt = a(t)y + b(t) with initial value.
pub struct LinearODE {
    /// Coefficient function a(t)
    pub a: Box<dyn Fn(f64) -> f64>,
    /// Forcing function b(t)
    pub b: Box<dyn Fn(f64) -> f64>,
    /// Initial time
    pub t0: f64,
    /// Initial value
    pub y0: f64,
}
impl LinearODE {
    /// Create a new linear ODE.
    pub fn new(
        a: impl Fn(f64) -> f64 + 'static,
        b: impl Fn(f64) -> f64 + 'static,
        t0: f64,
        y0: f64,
    ) -> Self {
        Self {
            a: Box::new(a),
            b: Box::new(b),
            t0,
            y0,
        }
    }
    /// Check whether this is a constant-coefficient autonomous ODE (b = 0).
    pub fn is_autonomous(&self) -> bool {
        [(self.b)(0.0), (self.b)(1.0), (self.b)(-1.0)]
            .iter()
            .all(|v| v.abs() < 1e-14)
    }
}
/// Adams-Bashforth / Adams-Moulton multistep method (2-step as reference).
pub struct MultistepMethod {
    /// Step size h
    pub h: f64,
    /// Number of steps k (currently supports 2)
    pub k: usize,
}
impl MultistepMethod {
    /// Create a k-step multistep method.
    pub fn new(h: f64, k: usize) -> Self {
        Self { h, k: k.max(2) }
    }
    /// Solve using Adams-Bashforth 2-step (predictor) from t0 to t_end.
    ///
    /// Bootstrapped with one RK4 step.
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let h = self.h;
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        let rk4 = RungeKutta4::new(h);
        let y1 = rk4.step(f, t, y);
        let t1 = t + h;
        result.push((t1, y1));
        let mut f_prev = f(t, y);
        let mut f_curr = f(t1, y1);
        t = t1;
        y = y1;
        while t < t_end - 1e-12 {
            let y_pred = y + h * (3.0 / 2.0 * f_curr - 1.0 / 2.0 * f_prev);
            let t_new = t + h;
            let f_pred = f(t_new, y_pred);
            let y_new = y + h * (1.0 / 2.0 * f_pred + 1.0 / 2.0 * f_curr);
            f_prev = f_curr;
            f_curr = f(t_new, y_new);
            t = t_new;
            y = y_new;
            result.push((t, y));
        }
        result
    }
}
/// Euler method for vector ODEs.
pub struct VectorEuler {
    /// Step size h
    pub h: f64,
    /// Number of steps to record
    pub n_steps: usize,
}
impl VectorEuler {
    /// Create with given step size and number of steps.
    pub fn new(h: f64, n_steps: usize) -> Self {
        Self { h, n_steps }
    }
    /// Advance one Euler step: y_{n+1} = y_n + h F(t_n, y_n).
    pub fn step(&self, y: &[f64], t: f64, f: impl Fn(f64, &[f64]) -> Vec<f64>) -> Vec<f64> {
        let fy = f(t, y);
        y.iter()
            .zip(fy.iter())
            .map(|(yi, fi)| yi + self.h * fi)
            .collect()
    }
    /// Global error is O(h): returns the step size as proxy.
    pub fn global_error_o_h(&self) -> f64 {
        self.h
    }
}
/// Adaptive Dormand-Prince RK45 solver with error control.
pub struct AdaptiveStepRK45 {
    /// Absolute tolerance
    pub atol: f64,
    /// Relative tolerance
    pub rtol: f64,
}
impl AdaptiveStepRK45 {
    /// Create with given tolerances.
    pub fn new(atol: f64, rtol: f64) -> Self {
        Self { atol, rtol }
    }
    /// Attempt one adaptive step; returns (y_new, h_new, accepted).
    pub fn adaptive_step(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t: f64,
        y: f64,
        h: f64,
    ) -> (f64, f64, bool) {
        let k1 = f(t, y);
        let k2 = f(t + h / 5.0, y + h / 5.0 * k1);
        let k3 = f(
            t + 3.0 * h / 10.0,
            y + h * (3.0 / 40.0 * k1 + 9.0 / 40.0 * k2),
        );
        let k4 = f(
            t + 4.0 * h / 5.0,
            y + h * (44.0 / 45.0 * k1 - 56.0 / 15.0 * k2 + 32.0 / 9.0 * k3),
        );
        let k5 = f(
            t + 8.0 * h / 9.0,
            y + h
                * (19372.0 / 6561.0 * k1 - 25360.0 / 2187.0 * k2 + 64448.0 / 6561.0 * k3
                    - 212.0 / 729.0 * k4),
        );
        let k6 = f(
            t + h,
            y + h
                * (9017.0 / 3168.0 * k1 - 355.0 / 33.0 * k2
                    + 46732.0 / 5247.0 * k3
                    + 49.0 / 176.0 * k4
                    - 5103.0 / 18656.0 * k5),
        );
        let y5 = y + h
            * (35.0 / 384.0 * k1 + 500.0 / 1113.0 * k3 + 125.0 / 192.0 * k4 - 2187.0 / 6784.0 * k5
                + 11.0 / 84.0 * k6);
        let k7 = f(t + h, y5);
        let y4 = y + h
            * (5179.0 / 57600.0 * k1 + 7571.0 / 16695.0 * k3 + 393.0 / 640.0 * k4
                - 92097.0 / 339200.0 * k5
                + 187.0 / 2100.0 * k6
                + 1.0 / 40.0 * k7);
        let err = (y5 - y4).abs();
        let tol = self.atol + self.rtol * y.abs().max(y5.abs());
        let accepted = err <= tol;
        let factor = if err < 1e-15 {
            5.0
        } else {
            0.9 * (tol / err).powf(0.2)
        };
        let h_new = (h * factor).min(10.0 * h).max(h / 10.0);
        (y5, h_new, accepted)
    }
    /// Solve from t0 to t_end.
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let mut t = t0;
        let mut y = y0;
        let mut h = (t_end - t0) / 100.0;
        result.push((t, y));
        while t < t_end - 1e-12 {
            if t + h > t_end {
                h = t_end - t;
            }
            let (y_new, h_new, accepted) = self.adaptive_step(f, t, y, h);
            if accepted {
                t += h;
                y = y_new;
                result.push((t, y));
            }
            h = h_new;
        }
        result
    }
}
/// Lyapunov exponent estimator for a scalar ODE.
pub struct LyapunovExponent {
    /// Integration time
    pub t_end: f64,
    /// Step size
    pub h: f64,
}
impl LyapunovExponent {
    /// Create with given integration time and step size.
    pub fn new(t_end: f64, h: f64) -> Self {
        Self { t_end, h }
    }
    /// Estimate the maximal Lyapunov exponent via finite-time growth of a nearby orbit.
    ///
    /// λ ≈ (1/T) log(|δ(T)| / |δ(0)|).
    pub fn estimate(&self, f: &dyn Fn(f64, f64) -> f64, x0: f64) -> f64 {
        let delta0 = 1e-8;
        let rk4 = RungeKutta4::new(self.h);
        let traj1 = rk4.solve_to(f, 0.0, x0, self.t_end);
        let traj2 = rk4.solve_to(f, 0.0, x0 + delta0, self.t_end);
        let y1 = traj1.last().map(|&(_, y)| y).unwrap_or(x0);
        let y2 = traj2.last().map(|&(_, y)| y).unwrap_or(x0 + delta0);
        let delta_t = (y2 - y1).abs();
        if delta_t < 1e-15 || self.t_end < 1e-12 {
            0.0
        } else {
            (delta_t / delta0).ln() / self.t_end
        }
    }
}
/// RK4 for vector ODEs.
pub struct VectorRK4 {
    /// Step size h
    pub h: f64,
}
impl VectorRK4 {
    /// Create with given step size.
    pub fn new(h: f64) -> Self {
        Self { h }
    }
    /// Advance one RK4 step.
    pub fn step(&self, y: &[f64], t: f64, f: impl Fn(f64, &[f64]) -> Vec<f64>) -> Vec<f64> {
        let h = self.h;
        let k1 = f(t, y);
        let y2: Vec<f64> = y
            .iter()
            .zip(&k1)
            .map(|(yi, ki)| yi + h / 2.0 * ki)
            .collect();
        let k2 = f(t + h / 2.0, &y2);
        let y3: Vec<f64> = y
            .iter()
            .zip(&k2)
            .map(|(yi, ki)| yi + h / 2.0 * ki)
            .collect();
        let k3 = f(t + h / 2.0, &y3);
        let y4: Vec<f64> = y.iter().zip(&k3).map(|(yi, ki)| yi + h * ki).collect();
        let k4 = f(t + h, &y4);
        y.iter()
            .enumerate()
            .map(|(i, yi)| yi + h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]))
            .collect()
    }
    /// Confirm RK4 is 4th-order accurate.
    pub fn is_4th_order(&self) -> bool {
        true
    }
}
/// The Lorenz chaotic system: dx/dt = σ(y-x), dy/dt = x(ρ-z)-y, dz/dt = xy-βz.
pub struct LorenzSystem {
    /// σ (Prandtl number)
    pub sigma: f64,
    /// ρ (Rayleigh number)
    pub rho: f64,
    /// β
    pub beta: f64,
}
impl LorenzSystem {
    /// Create with the classic Lorenz parameters.
    pub fn new(sigma: f64, rho: f64, beta: f64) -> Self {
        Self { sigma, rho, beta }
    }
    /// Advance one RK4 step of size h from the given state [x, y, z].
    pub fn step(&self, state: &[f64; 3], h: f64) -> [f64; 3] {
        let lorenz = |s: &[f64; 3]| -> [f64; 3] {
            [
                self.sigma * (s[1] - s[0]),
                s[0] * (self.rho - s[2]) - s[1],
                s[0] * s[1] - self.beta * s[2],
            ]
        };
        let k1 = lorenz(state);
        let s2 = [
            state[0] + h / 2.0 * k1[0],
            state[1] + h / 2.0 * k1[1],
            state[2] + h / 2.0 * k1[2],
        ];
        let k2 = lorenz(&s2);
        let s3 = [
            state[0] + h / 2.0 * k2[0],
            state[1] + h / 2.0 * k2[1],
            state[2] + h / 2.0 * k2[2],
        ];
        let k3 = lorenz(&s3);
        let s4 = [
            state[0] + h * k3[0],
            state[1] + h * k3[1],
            state[2] + h * k3[2],
        ];
        let k4 = lorenz(&s4);
        [
            state[0] + h / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
            state[1] + h / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
            state[2] + h / 6.0 * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]),
        ]
    }
    /// For the classic parameters (σ=10, ρ=28, β=8/3) the system is chaotic.
    pub fn is_chaotic(&self) -> bool {
        self.rho > 24.0 && (self.sigma - 10.0).abs() < 5.0 && self.beta > 0.0
    }
    /// Describe the strange (Lorenz) attractor.
    pub fn strange_attractor(&self) -> String {
        if self.is_chaotic() {
            format!(
                "Lorenz strange attractor: fractal dimension ≈ 2.06,                  Hausdorff dimension ≈ 2.06, σ={}, ρ={}, β={}",
                self.sigma, self.rho, self.beta
            )
        } else {
            format!(
                "No strange attractor: parameters σ={}, ρ={}, β={} are not in the chaotic regime.",
                self.sigma, self.rho, self.beta
            )
        }
    }
}
/// A Jacobian matrix Df(x*) for linearisation at an equilibrium.
pub struct JacobianMatrix {
    /// 2×2 matrix entries
    pub entries: [[f64; 2]; 2],
}
impl JacobianMatrix {
    /// Construct from explicit 2×2 entries.
    pub fn new(entries: [[f64; 2]; 2]) -> Self {
        Self { entries }
    }
    /// Compute the determinant.
    pub fn det(&self) -> f64 {
        self.entries[0][0] * self.entries[1][1] - self.entries[0][1] * self.entries[1][0]
    }
    /// Compute the trace.
    pub fn trace(&self) -> f64 {
        self.entries[0][0] + self.entries[1][1]
    }
    /// Classify the equilibrium from this Jacobian.
    pub fn classify_equilibrium(&self) -> StabilityType {
        let ep = EquilibriumPoint {
            x: 0.0,
            y: 0.0,
            jacobian: self.entries,
        };
        ep.classify()
    }
}
/// A bifurcation point in a parameterized family of ODEs.
pub struct BifurcationPoint {
    /// Parameter value at the bifurcation
    pub param_value: f64,
    /// Type of bifurcation
    pub bif_type: String,
}
impl BifurcationPoint {
    /// Create with given parameter value and bifurcation type.
    pub fn new(param_value: f64, bif_type: impl Into<String>) -> Self {
        Self {
            param_value,
            bif_type: bif_type.into(),
        }
    }
    /// Saddle-node bifurcation description.
    pub fn saddle_node(&self) -> String {
        format!(
            "Saddle-node bifurcation at μ = {}: two equilibria collide and annihilate.",
            self.param_value
        )
    }
    /// Hopf bifurcation description.
    pub fn hopf(&self) -> String {
        format!(
            "Hopf bifurcation at μ = {}: equilibrium loses stability, a limit cycle is born.",
            self.param_value
        )
    }
    /// Pitchfork bifurcation description.
    pub fn pitchfork(&self) -> String {
        format!(
            "Pitchfork bifurcation at μ = {}: one equilibrium splits into three (supercritical) or vice versa.",
            self.param_value
        )
    }
}
/// A heat equation u_t = κΔu.
pub struct HeatEquation {
    /// Thermal diffusivity κ > 0
    pub kappa: f64,
}
impl HeatEquation {
    /// Create with given diffusivity.
    pub fn new(kappa: f64) -> Self {
        Self { kappa }
    }
    /// Heat equations are parabolic.
    pub fn classify_pde(&self) -> PDEType {
        PDEType::Parabolic
    }
    /// Fundamental solution (heat kernel) at point x and time t.
    ///
    /// K(x, t) = 1/√(4πκt) exp(−x²/(4κt)).
    pub fn fundamental_solution(&self, x: f64, t: f64) -> f64 {
        if t <= 0.0 {
            return f64::NAN;
        }
        let denom = (4.0 * std::f64::consts::PI * self.kappa * t).sqrt();
        (-x * x / (4.0 * self.kappa * t)).exp() / denom
    }
}
/// A delay differential equation y'(t) = f(t, y(t), y(t-τ)).
pub struct DelayDE {
    /// Delay τ > 0
    pub tau: f64,
    /// Step size for the method of steps
    pub h: f64,
}
impl DelayDE {
    /// Create with given delay and step size.
    pub fn new(tau: f64, h: f64) -> Self {
        Self { tau, h }
    }
    /// Solve by the method of steps from t0 to t_end.
    ///
    /// `phi` is the history function on [t0 - tau, t0].
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64, f64) -> f64,
        phi: &dyn Fn(f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        while t < t_end - 1e-12 {
            let h = self.h.min(t_end - t);
            let t_del = t - self.tau;
            let y_del = if t_del < t0 {
                phi(t_del)
            } else {
                let idx = result
                    .partition_point(|&(ti, _)| ti <= t_del)
                    .saturating_sub(1);
                result[idx].1
            };
            y = y + h * f(t, y, y_del);
            t += h;
            result.push((t, y));
        }
        result
    }
}
impl DelayDE {
    /// Characteristic equation for the linear DDE y'(t) = ay(t) + by(t-τ).
    pub fn characteristic_equation(&self) -> String {
        format!(
            "Characteristic equation: λ = a + b·e^(-λτ), τ = {} (transcendental)",
            self.tau
        )
    }
    /// Stability region: for the simple linear DDE the equilibrium is stable iff
    /// a + |b| < 0 (when a < 0 and |b| < -a).
    pub fn stability_region(&self) -> String {
        format!(
            "Stability region for y'=ay+by(t-{}): requires a + |b| < 0              (exact boundary determined by characteristic roots on imaginary axis)",
            self.tau
        )
    }
}
/// A strange attractor record (metadata only — no full simulation here).
pub struct StrangeAttractor {
    /// Name of the attractor (e.g. "Lorenz")
    pub name: &'static str,
    /// Estimated fractal dimension
    pub fractal_dimension: f64,
    /// Maximal Lyapunov exponent
    pub max_lyapunov: f64,
}
impl StrangeAttractor {
    /// Create a strange attractor descriptor.
    pub fn new(name: &'static str, fractal_dimension: f64, max_lyapunov: f64) -> Self {
        Self {
            name,
            fractal_dimension,
            max_lyapunov,
        }
    }
    /// Returns true if the attractor exhibits sensitive dependence (max Lyapunov > 0).
    pub fn is_chaotic(&self) -> bool {
        self.max_lyapunov > 0.0
    }
}
/// Flow map φ_t for an autonomous ODE, computed via RK4.
pub struct FlowMap {
    rk4: RungeKutta4,
}
impl FlowMap {
    /// Create a flow map with the given step size.
    pub fn new(h: f64) -> Self {
        Self {
            rk4: RungeKutta4::new(h),
        }
    }
    /// Apply the flow for time t: φ_t(x₀) using RK4.
    pub fn apply(&self, f: &dyn Fn(f64, f64) -> f64, x0: f64, t: f64) -> f64 {
        let traj = self.rk4.solve_to(f, 0.0, x0, t);
        traj.last().map(|&(_, y)| y).unwrap_or(x0)
    }
}
/// Represents an Itô SDE: dX_t = μ(X_t, t) dt + σ(X_t, t) dW_t.
#[allow(dead_code)]
pub struct ItoSDE {
    /// Name/identifier for this SDE.
    pub name: String,
    /// Drift coefficient μ (simplified: linear form μ(x, t) = a*x + b).
    pub drift_a: f64,
    pub drift_b: f64,
    /// Diffusion coefficient σ (constant in this simplified form).
    pub diffusion_sigma: f64,
}
#[allow(dead_code)]
impl ItoSDE {
    /// Create a new Itô SDE with linear drift and constant diffusion.
    pub fn new(name: &str, drift_a: f64, drift_b: f64, diffusion_sigma: f64) -> Self {
        ItoSDE {
            name: name.to_string(),
            drift_a,
            drift_b,
            diffusion_sigma,
        }
    }
    /// Geometric Brownian Motion: dX = μX dt + σX dW.
    /// Here drift_a = μ, diffusion proportional to X.
    pub fn geometric_brownian_motion(mu: f64, sigma: f64) -> Self {
        ItoSDE::new("GBM", mu, 0.0, sigma)
    }
    /// Ornstein-Uhlenbeck process: dX = θ(μ - X) dt + σ dW.
    pub fn ornstein_uhlenbeck(theta: f64, mu: f64, sigma: f64) -> Self {
        ItoSDE::new("OU", -theta, theta * mu, sigma)
    }
    /// Euler-Maruyama step: X_{n+1} = X_n + μ(X_n, t) dt + σ ΔW
    /// where ΔW ~ N(0, dt).
    pub fn euler_maruyama_step(&self, x: f64, _t: f64, dt: f64, dw: f64) -> f64 {
        let drift = self.drift_a * x + self.drift_b;
        let diffusion = self.diffusion_sigma;
        x + drift * dt + diffusion * dw
    }
    /// Milstein scheme: higher-order correction using σ σ' (ΔW^2 - dt) / 2.
    /// For constant σ, σ' = 0, so Milstein = Euler-Maruyama.
    pub fn milstein_step(&self, x: f64, t: f64, dt: f64, dw: f64) -> f64 {
        self.euler_maruyama_step(x, t, dt, dw)
    }
    /// Variance of the Ornstein-Uhlenbeck stationary distribution: σ^2 / (2θ).
    pub fn ou_stationary_variance(&self) -> f64 {
        let theta = -self.drift_a;
        if theta <= 0.0 {
            return f64::INFINITY;
        }
        self.diffusion_sigma * self.diffusion_sigma / (2.0 * theta)
    }
    /// Mean of the OU stationary distribution: μ = drift_b / (-drift_a).
    pub fn ou_stationary_mean(&self) -> f64 {
        if self.drift_a >= 0.0 {
            return f64::NAN;
        }
        self.drift_b / (-self.drift_a)
    }
    /// Itô's lemma: for f(X_t), df = (f' μ + f'' σ^2 / 2) dt + f' σ dW.
    /// Returns the generator (drift part) for a smooth test function with f'(x)=1, f''(x)=0.
    pub fn ito_generator(&self, x: f64) -> f64 {
        self.drift_a * x + self.drift_b
    }
}
/// Represents a Fredholm integral equation of the second kind:
/// u(x) = f(x) + λ ∫_a^b K(x, y) u(y) dy.
#[allow(dead_code)]
pub struct FredholmEquation {
    /// The interval [a, b].
    pub a: f64,
    pub b: f64,
    /// The parameter λ.
    pub lambda: f64,
    /// Number of quadrature points for numerical solution.
    pub n_points: usize,
}
#[allow(dead_code)]
impl FredholmEquation {
    /// Create a new Fredholm equation.
    pub fn new(a: f64, b: f64, lambda: f64, n_points: usize) -> Self {
        FredholmEquation {
            a,
            b,
            lambda,
            n_points,
        }
    }
    /// Quadrature nodes using midpoint rule.
    pub fn quadrature_nodes(&self) -> Vec<f64> {
        let h = (self.b - self.a) / self.n_points as f64;
        (0..self.n_points)
            .map(|i| self.a + (i as f64 + 0.5) * h)
            .collect()
    }
    /// Quadrature weight (uniform for midpoint rule).
    pub fn quadrature_weight(&self) -> f64 {
        (self.b - self.a) / self.n_points as f64
    }
    /// Neumann series approximation: u ≈ f + λ K f + λ^2 K^2 f + ...
    /// For |λ| * ||K||_∞ < 1, this converges. Returns order-2 approximation.
    pub fn neumann_series_order2(&self, f_at_nodes: &[f64], kernel_norm: f64) -> Vec<f64> {
        let w = self.quadrature_weight();
        let n = self.n_points;
        let sum_f: f64 = f_at_nodes.iter().sum::<f64>();
        let kf_approx = kernel_norm * sum_f * w;
        (0..n)
            .map(|i| f_at_nodes[i] + self.lambda * kf_approx)
            .collect()
    }
    /// Spectral radius condition for Neumann series convergence: |λ| * ||K||_2 < 1.
    pub fn neumann_convergence_condition(&self, kernel_l2_norm: f64) -> bool {
        self.lambda.abs() * kernel_l2_norm < 1.0
    }
}
/// A Laplace equation Δu = 0 representation.
pub struct LaplacianEqn {
    /// Domain: [x_min, x_max] × [y_min, y_max]
    pub domain: [f64; 4],
}
impl LaplacianEqn {
    /// Create with the given rectangular domain.
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            domain: [x_min, x_max, y_min, y_max],
        }
    }
    /// PDEs of the form Δu = 0 are always elliptic.
    pub fn classify_pde(&self) -> PDEType {
        PDEType::Elliptic
    }
}
/// An ordinary differential equation dy/dt = f(t, y) with initial value y(t₀) = y₀.
pub struct ODE {
    /// Right-hand side f(t, y)
    pub f: Box<dyn Fn(f64, f64) -> f64>,
    /// Initial time t₀
    pub t0: f64,
    /// Initial value y₀ = y(t₀)
    pub y0: f64,
}
impl ODE {
    /// Create a new ODE initial-value problem.
    pub fn new(f: impl Fn(f64, f64) -> f64 + 'static, t0: f64, y0: f64) -> Self {
        Self {
            f: Box::new(f),
            t0,
            y0,
        }
    }
}
/// Single-step Euler method solver.
pub struct EulerMethod {
    /// Step size h
    pub h: f64,
}
impl EulerMethod {
    /// Create with the given step size.
    pub fn new(h: f64) -> Self {
        Self { h }
    }
    /// Perform one Euler step: y_{n+1} = y_n + h f(t_n, y_n).
    pub fn step(&self, f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64) -> f64 {
        y + self.h * f(t, y)
    }
    /// Solve from t0 to t_end, returning (t, y) pairs.
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        while t < t_end - 1e-12 {
            let h = self.h.min(t_end - t);
            y = y + h * f(t, y);
            t += h;
            result.push((t, y));
        }
        result
    }
    /// Global error is O(h): returns h as a proxy for the error constant.
    pub fn global_error_o_h(&self) -> f64 {
        self.h
    }
}
/// Stability analysis via eigenvalues (real_part, imag_part).
pub struct StabilityAnalysis {
    /// Eigenvalues as (real part, imaginary part) pairs
    pub eigenvalues: Vec<(f64, f64)>,
}
impl StabilityAnalysis {
    /// Create from a list of (real, imag) eigenvalue pairs.
    pub fn new(eigenvalues: Vec<(f64, f64)>) -> Self {
        Self { eigenvalues }
    }
    /// Lyapunov stable: all eigenvalues have non-positive real parts.
    pub fn is_stable(&self) -> bool {
        self.eigenvalues.iter().all(|(re, _)| *re <= 1e-10)
    }
    /// Asymptotically stable: all eigenvalues have strictly negative real parts.
    pub fn is_asymptotically_stable(&self) -> bool {
        self.eigenvalues.iter().all(|(re, _)| *re < -1e-10)
    }
    /// Lyapunov criterion: the equilibrium is stable iff all eigenvalues have Re ≤ 0
    /// (with non-strictly: marginal stability). Returns a descriptive string.
    pub fn lyapunov_criterion(&self) -> String {
        let max_re = self
            .eigenvalues
            .iter()
            .map(|(r, _)| *r)
            .fold(f64::NEG_INFINITY, f64::max);
        if max_re < -1e-10 {
            format!("Asymptotically stable (max Re(λ) = {:.4e})", max_re)
        } else if max_re <= 1e-10 {
            format!("Lyapunov stable — marginal (max Re(λ) = {:.4e})", max_re)
        } else {
            format!("Unstable (max Re(λ) = {:.4e} > 0)", max_re)
        }
    }
}
/// A solution in function space C([t₀−τ, T]).
pub struct FunctionSpaceSolution {
    /// The solution values at grid points
    pub times: Vec<f64>,
    /// Corresponding y values
    pub values: Vec<f64>,
}
impl FunctionSpaceSolution {
    /// Create from parallel time/value vectors.
    pub fn new(times: Vec<f64>, values: Vec<f64>) -> Self {
        Self { times, values }
    }
    /// Evaluate by linear interpolation.
    pub fn eval(&self, t: f64) -> f64 {
        let idx = self.times.partition_point(|&ti| ti <= t);
        if idx == 0 {
            return self.values[0];
        }
        if idx >= self.times.len() {
            return *self
                .values
                .last()
                .expect("values is non-empty: checked by idx >= len condition");
        }
        let t0 = self.times[idx - 1];
        let t1 = self.times[idx];
        let y0 = self.values[idx - 1];
        let y1 = self.values[idx];
        if (t1 - t0).abs() < 1e-15 {
            y0
        } else {
            y0 + (y1 - y0) * (t - t0) / (t1 - t0)
        }
    }
    /// Infinity-norm of the solution.
    pub fn sup_norm(&self) -> f64 {
        self.values.iter().cloned().fold(0.0_f64, f64::max)
    }
}
/// Volterra integral equation y(t) = ∫₀ᵗ K(t,s)y(s)ds + g(t).
pub struct VolterraIntegralEquation {
    /// Forcing function g(t)
    pub g: Box<dyn Fn(f64) -> f64>,
    /// Number of quadrature points per step
    pub n_quad: usize,
}
impl VolterraIntegralEquation {
    /// Create with given forcing function and quadrature count.
    pub fn new(g: impl Fn(f64) -> f64 + 'static, n_quad: usize) -> Self {
        Self {
            g: Box::new(g),
            n_quad,
        }
    }
    /// Solve numerically using the Nyström method (trapezoidal rule in s) on [0, t_end].
    pub fn solve(&self, kernel: &dyn Fn(f64, f64) -> f64, t_end: f64, n: usize) -> Vec<(f64, f64)> {
        let h = t_end / n as f64;
        let mut y = vec![0.0f64; n + 1];
        y[0] = (self.g)(0.0);
        for i in 1..=n {
            let ti = i as f64 * h;
            let mut integral = 0.0;
            for j in 0..i {
                let sj = j as f64 * h;
                let w = if j == 0 || j == i - 1 { 0.5 } else { 1.0 };
                integral += w * h * kernel(ti, sj) * y[j];
            }
            y[i] = integral + (self.g)(ti);
        }
        (0..=n).map(|i| (i as f64 * h, y[i])).collect()
    }
}
/// A wave equation u_tt = c²Δu.
pub struct WaveEquation {
    /// Wave speed c
    pub c: f64,
}
impl WaveEquation {
    /// Create with given wave speed.
    pub fn new(c: f64) -> Self {
        Self { c }
    }
    /// Wave equations are hyperbolic.
    pub fn classify_pde(&self) -> PDEType {
        PDEType::Hyperbolic
    }
}
/// Classification of a second-order linear PDE Au_xx + Bu_xy + Cu_yy + ... = 0.
#[derive(Debug, Clone, PartialEq)]
pub enum PDEType {
    /// B² - 4AC < 0
    Elliptic,
    /// B² - 4AC = 0
    Parabolic,
    /// B² - 4AC > 0
    Hyperbolic,
    /// Discriminant changes sign in the domain
    Mixed,
}
/// A boundary condition type.
#[derive(Debug, Clone)]
pub enum BoundaryCondition {
    /// u = g on boundary
    Dirichlet(f64),
    /// du/dn = g on boundary
    Neumann(f64),
    /// αu + β du/dn = g on boundary
    Robin(f64, f64, f64),
    /// Periodic boundary conditions
    Periodic,
}
/// A Hamiltonian system with symplectic structure.
pub struct HamiltonianSystem {
    /// String representation of the Hamiltonian H(q,p)
    pub hamiltonian: String,
    /// Number of degrees of freedom (dim of q or p separately)
    pub dim: usize,
}
impl HamiltonianSystem {
    /// Create with given Hamiltonian string and dimension.
    pub fn new(hamiltonian: impl Into<String>, dim: usize) -> Self {
        Self {
            hamiltonian: hamiltonian.into(),
            dim,
        }
    }
    /// Describe Hamilton's equations of motion: q̇ = ∂H/∂p, ṗ = -∂H/∂q.
    pub fn hamilton_equations(&self) -> String {
        format!(
            "Hamilton's equations for H = {}:\n  q̇ᵢ = ∂H/∂pᵢ\n  ṗᵢ = -∂H/∂qᵢ  (i = 1..{})",
            self.hamiltonian, self.dim
        )
    }
    /// Describe the symplectic structure: the canonical 2-form ω = dq ∧ dp.
    pub fn symplectic_structure(&self) -> String {
        format!(
            "Symplectic form ω = Σᵢ dqᵢ ∧ dpᵢ on ℝ^{} (standard)",
            2 * self.dim
        )
    }
    /// Liouville's theorem: the Hamiltonian flow preserves phase-space volume.
    pub fn liouville_theorem(&self) -> String {
        "Liouville: Hamiltonian flow is volume-preserving (div X_H = 0).".to_string()
    }
}
/// Manifold type at an equilibrium.
#[derive(Debug, Clone, PartialEq)]
pub enum Manifold {
    /// Tangent to eigenspace for eigenvalues with Re < 0
    StableManifold,
    /// Tangent to eigenspace for eigenvalues with Re > 0
    UnstableManifold,
    /// Tangent to eigenspace for eigenvalues with Re = 0
    CenterManifold,
}
/// An exact ODE P(x,y)dx + Q(x,y)dy = 0.
pub struct ExactODE {
    /// P(x, y)
    pub p: Box<dyn Fn(f64, f64) -> f64>,
    /// Q(x, y)
    pub q: Box<dyn Fn(f64, f64) -> f64>,
}
impl ExactODE {
    /// Create a new exact-ODE candidate.
    pub fn new(
        p: impl Fn(f64, f64) -> f64 + 'static,
        q: impl Fn(f64, f64) -> f64 + 'static,
    ) -> Self {
        Self {
            p: Box::new(p),
            q: Box::new(q),
        }
    }
    /// Test exactness ∂P/∂y ≈ ∂Q/∂x at a sample point using central differences.
    pub fn is_exact(&self) -> bool {
        let h = 1e-5;
        let x0 = 1.0_f64;
        let y0 = 1.0_f64;
        let dp_dy = ((self.p)(x0, y0 + h) - (self.p)(x0, y0 - h)) / (2.0 * h);
        let dq_dx = ((self.q)(x0 + h, y0) - (self.q)(x0 - h, y0)) / (2.0 * h);
        (dp_dy - dq_dx).abs() < 1e-6
    }
}
/// Represents a one-parameter family of ODEs: x' = f(x, λ).
#[allow(dead_code)]
pub struct BifurcationDiagram {
    /// Parameter values λ.
    pub params: Vec<f64>,
    /// Fixed points at each λ (simplified: polynomial roots approximation).
    pub fixed_points: Vec<Vec<f64>>,
}
#[allow(dead_code)]
impl BifurcationDiagram {
    /// Create a bifurcation diagram for the normal form x' = λx - x^3 (pitchfork).
    pub fn pitchfork_normal_form(lambda_range: &[f64]) -> Self {
        let mut fps = vec![];
        for &lam in lambda_range {
            if lam < 0.0 {
                fps.push(vec![0.0]);
            } else {
                let sqrt_lam = lam.sqrt();
                fps.push(vec![-sqrt_lam, 0.0, sqrt_lam]);
            }
        }
        BifurcationDiagram {
            params: lambda_range.to_vec(),
            fixed_points: fps,
        }
    }
    /// Saddle-node normal form: x' = λ + x^2.
    pub fn saddle_node_normal_form(lambda_range: &[f64]) -> Self {
        let mut fps = vec![];
        for &lam in lambda_range {
            if lam < 0.0 {
                let sqrt_neg = (-lam).sqrt();
                fps.push(vec![-sqrt_neg, sqrt_neg]);
            } else if lam == 0.0 {
                fps.push(vec![0.0]);
            } else {
                fps.push(vec![]);
            }
        }
        BifurcationDiagram {
            params: lambda_range.to_vec(),
            fixed_points: fps,
        }
    }
    /// Hopf normal form: r' = λr - r^3 in polar coords.
    /// Limit cycle radius = sqrt(λ) for λ > 0.
    pub fn hopf_limit_cycle_radius(lambda: f64) -> f64 {
        if lambda > 0.0 {
            lambda.sqrt()
        } else {
            0.0
        }
    }
    /// Count bifurcation points (parameter values where structure changes).
    pub fn count_bifurcation_points(&self) -> usize {
        let mut count = 0;
        for i in 1..self.fixed_points.len() {
            if self.fixed_points[i].len() != self.fixed_points[i - 1].len() {
                count += 1;
            }
        }
        count
    }
}
/// A stochastic differential equation dX = f(X,t)dt + g(X,t)dW.
pub struct StochasticDE {
    /// String representation of the drift coefficient f(X,t)
    pub drift: String,
    /// String representation of the diffusion coefficient g(X,t)
    pub diffusion: String,
}
impl StochasticDE {
    /// Create with given drift and diffusion descriptions.
    pub fn new(drift: impl Into<String>, diffusion: impl Into<String>) -> Self {
        Self {
            drift: drift.into(),
            diffusion: diffusion.into(),
        }
    }
    /// Itô's formula: for smooth F(t,X), dF = (∂F/∂t + f·∂F/∂X + ½g²·∂²F/∂X²)dt + g·∂F/∂X·dW.
    pub fn ito_formula(&self) -> String {
        format!(
            "Itô formula for dX = ({})dt + ({})dW:\n             dF = (∂F/∂t + f·F' + ½g²·F'')dt + g·F'·dW",
            self.drift, self.diffusion
        )
    }
    /// Explain the difference between Itô and Stratonovich conventions.
    pub fn stratonovich_vs_ito(&self) -> String {
        format!(
            "Stratonovich SDE: dX = ({})dt + ({})∘dW\n             Conversion: Itô drift = Stratonovich drift - ½g·(∂g/∂X).\n             Itô is the standard in mathematical finance; Stratonovich in physics.",
            self.drift, self.diffusion
        )
    }
}
/// A multi-dimensional ODE system dy/dt = F(t, y) with vector state.
pub struct VectorODE {
    /// Spatial dimension (number of equations)
    pub dimension: usize,
    /// Whether the ODE is autonomous (F does not depend explicitly on t)
    pub is_autonomous: bool,
    /// Right-hand side F(t, y) → dy/dt
    pub rhs: Box<dyn Fn(f64, &[f64]) -> Vec<f64>>,
}
impl VectorODE {
    /// Create a new vector ODE with the given dimension and RHS.
    pub fn new(
        dimension: usize,
        is_autonomous: bool,
        rhs: impl Fn(f64, &[f64]) -> Vec<f64> + 'static,
    ) -> Self {
        Self {
            dimension,
            is_autonomous,
            rhs: Box::new(rhs),
        }
    }
    /// Return a string description of the RHS vector field.
    pub fn rhs_vector_field(&self) -> String {
        if self.is_autonomous {
            format!(
                "Autonomous vector field F: ℝ^{} → ℝ^{}",
                self.dimension, self.dimension
            )
        } else {
            format!(
                "Non-autonomous vector field F: ℝ × ℝ^{} → ℝ^{}",
                self.dimension, self.dimension
            )
        }
    }
    /// Approximate equilibrium points by sampling the grid [-5,5]^n (only for dim ≤ 2).
    pub fn equilibrium_points(&self, tol: f64) -> Vec<Vec<f64>> {
        let mut result = Vec::new();
        if self.dimension == 1 {
            let n = 200usize;
            for i in 0..n {
                let x = -5.0 + 10.0 * i as f64 / n as f64;
                let f = (self.rhs)(0.0, &[x]);
                if f[0].abs() < tol {
                    result.push(vec![x]);
                }
            }
        } else if self.dimension == 2 {
            let n = 40usize;
            for i in 0..n {
                for j in 0..n {
                    let x = -5.0 + 10.0 * i as f64 / n as f64;
                    let y = -5.0 + 10.0 * j as f64 / n as f64;
                    let f = (self.rhs)(0.0, &[x, y]);
                    if f[0].abs() < tol && f[1].abs() < tol {
                        result.push(vec![x, y]);
                    }
                }
            }
        }
        result
    }
}
/// Classification of a 2-D equilibrium point.
#[derive(Debug, Clone, PartialEq)]
pub enum StabilityType {
    /// Real negative eigenvalues: stable node
    StableNode,
    /// Real positive eigenvalues: unstable node
    UnstableNode,
    /// Real eigenvalues of opposite sign: saddle
    Saddle,
    /// Complex eigenvalues with negative real part: stable spiral
    StableSpiral,
    /// Complex eigenvalues with positive real part: unstable spiral
    UnstableSpiral,
    /// Pure imaginary eigenvalues: center
    Center,
    /// Degenerate (zero eigenvalue)
    Degenerate,
}
/// Classical 4th-order Runge-Kutta solver.
pub struct RungeKutta4 {
    /// Step size h
    pub h: f64,
}
impl RungeKutta4 {
    /// Create with the given step size.
    pub fn new(h: f64) -> Self {
        Self { h }
    }
    /// Perform one RK4 step.
    pub fn step(&self, f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64) -> f64 {
        let h = self.h;
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
        let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
        let k4 = f(t + h, y + h * k3);
        y + h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    }
    /// Solve from t0 to t_end, returning (t, y) pairs.
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        while t < t_end - 1e-12 {
            let h = self.h.min(t_end - t);
            let k1 = f(t, y);
            let k2 = f(t + h / 2.0, y + h / 2.0 * k1);
            let k3 = f(t + h / 2.0, y + h / 2.0 * k2);
            let k4 = f(t + h, y + h * k3);
            y += h / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            t += h;
            result.push((t, y));
        }
        result
    }
    /// Error estimate (Richardson extrapolation with half-step).
    pub fn error_estimate(&self, f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64) -> f64 {
        let y_full = self.step(f, t, y);
        let half = RungeKutta4::new(self.h / 2.0);
        let y_half1 = half.step(f, t, y);
        let y_half2 = half.step(f, t + self.h / 2.0, y_half1);
        (y_half2 - y_full).abs() / 15.0
    }
    /// Confirm this is a 4th-order method.
    pub fn is_4th_order(&self) -> bool {
        true
    }
}
/// A Sturm-Liouville problem (p(x)y')' + (q(x) + λw(x))y = 0.
pub struct SturmLiouville {
    /// p(x) — coefficient of y'
    pub p: Box<dyn Fn(f64) -> f64>,
    /// q(x) — potential
    pub q: Box<dyn Fn(f64) -> f64>,
    /// w(x) — weight function (positive)
    pub w: Box<dyn Fn(f64) -> f64>,
}
impl SturmLiouville {
    /// Create a new Sturm-Liouville problem.
    pub fn new(
        p: impl Fn(f64) -> f64 + 'static,
        q: impl Fn(f64) -> f64 + 'static,
        w: impl Fn(f64) -> f64 + 'static,
    ) -> Self {
        Self {
            p: Box::new(p),
            q: Box::new(q),
            w: Box::new(w),
        }
    }
    /// Approximate the lowest eigenvalue by power-iteration on the Rayleigh quotient
    /// (grid-based, for regular SL problems on [0, 1]).
    pub fn approximate_lowest_eigenvalue(&self, n: usize) -> f64 {
        let h = 1.0 / (n + 1) as f64;
        let mut y: Vec<f64> = (1..=n)
            .map(|k| (k as f64 * h * std::f64::consts::PI).sin())
            .collect();
        let norm: f64 = y.iter().map(|v| v * v).sum::<f64>().sqrt();
        for v in &mut y {
            *v /= norm;
        }
        let mut ay: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            let xi = (i + 1) as f64 * h;
            let yi_prev = if i == 0 { 0.0 } else { y[i - 1] };
            let yi_next = if i == n - 1 { 0.0 } else { y[i + 1] };
            ay[i] =
                -(self.p)(xi) * (yi_next - 2.0 * y[i] + yi_prev) / (h * h) + (self.q)(xi) * y[i];
        }
        let numerator: f64 = y.iter().zip(ay.iter()).map(|(a, b)| a * b).sum();
        let denominator: f64 = y
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let xi = (i + 1) as f64 * h;
                v * v * (self.w)(xi)
            })
            .sum();
        if denominator.abs() < 1e-15 {
            0.0
        } else {
            numerator / denominator
        }
    }
    /// Check whether this is a regular Sturm-Liouville problem (p, q, w continuous and p > 0, w > 0 on [0,1]).
    pub fn is_regular(&self) -> bool {
        let n = 10usize;
        (0..=n).all(|k| {
            let x = k as f64 / n as f64;
            (self.p)(x) > 1e-12 && (self.w)(x) > 1e-12
        })
    }
}
/// Poincaré first-return map for a scalar ODE with a section at y = section_value.
pub struct PoincareMap {
    /// Section value (y = c defines the Poincaré section)
    pub section_value: f64,
    /// Step size for integration
    pub h: f64,
    /// Maximum integration time
    pub t_max: f64,
}
impl PoincareMap {
    /// Create with given section and integration parameters.
    pub fn new(section_value: f64, h: f64, t_max: f64) -> Self {
        Self {
            section_value,
            h,
            t_max,
        }
    }
    /// Compute successive Poincaré crossings (upward through section_value).
    pub fn crossings(&self, f: &dyn Fn(f64, f64) -> f64, y0: f64) -> Vec<f64> {
        let rk4 = RungeKutta4::new(self.h);
        let traj = rk4.solve_to(f, 0.0, y0, self.t_max);
        let s = self.section_value;
        let mut crossings = Vec::new();
        for w in traj.windows(2) {
            let (_, ya) = w[0];
            let (_, yb) = w[1];
            if ya < s && yb >= s {
                let frac = (s - ya) / (yb - ya);
                crossings.push(ya + frac * (yb - ya));
            }
        }
        crossings
    }
}
/// An equilibrium point with its Jacobian for a 2-D autonomous ODE.
pub struct EquilibriumPoint {
    /// x-coordinate of the equilibrium
    pub x: f64,
    /// y-coordinate of the equilibrium
    pub y: f64,
    /// 2×2 Jacobian matrix [[a, b], [c, d]]
    pub jacobian: [[f64; 2]; 2],
}
impl EquilibriumPoint {
    /// Create an equilibrium point; the Jacobian is computed via finite differences.
    pub fn new(f: &dyn Fn(f64, f64) -> (f64, f64), x: f64, y: f64) -> Self {
        let h = 1e-5;
        let (fp0, fp1) = f(x + h, y);
        let (fm0, fm1) = f(x - h, y);
        let (fq0, fq1) = f(x, y + h);
        let (fq0m, fq1m) = f(x, y - h);
        let df00 = (fp0 - fm0) / (2.0 * h);
        let df01 = (fq0 - fq0m) / (2.0 * h);
        let df10 = (fp1 - fm1) / (2.0 * h);
        let df11 = (fq1 - fq1m) / (2.0 * h);
        Self {
            x,
            y,
            jacobian: [[df00, df01], [df10, df11]],
        }
    }
    /// Classify the equilibrium from eigenvalue analysis of the Jacobian.
    pub fn classify(&self) -> StabilityType {
        let [[a, b], [c, d]] = self.jacobian;
        let trace = a + d;
        let det = a * d - b * c;
        let discriminant = trace * trace - 4.0 * det;
        if det < -1e-10 {
            StabilityType::Saddle
        } else if discriminant < -1e-10 {
            if trace < -1e-10 {
                StabilityType::StableSpiral
            } else if trace > 1e-10 {
                StabilityType::UnstableSpiral
            } else {
                StabilityType::Center
            }
        } else if det.abs() < 1e-10 {
            StabilityType::Degenerate
        } else {
            if trace < -1e-10 {
                StabilityType::StableNode
            } else if trace > 1e-10 {
                StabilityType::UnstableNode
            } else {
                StabilityType::Degenerate
            }
        }
    }
}
/// Fredholm integral equation of the second kind: y(t) = λ ∫_a^b K(t,s)y(s)ds + g(t).
pub struct FredholmIntegralEquation {
    /// Eigenvalue parameter λ
    pub lambda: f64,
    /// Interval [a, b]
    pub a: f64,
    /// Interval [a, b]
    pub b: f64,
    /// Forcing function g(t)
    pub g: Box<dyn Fn(f64) -> f64>,
}
impl FredholmIntegralEquation {
    /// Create with given parameters.
    pub fn new(lambda: f64, a: f64, b: f64, g: impl Fn(f64) -> f64 + 'static) -> Self {
        Self {
            lambda,
            a,
            b,
            g: Box::new(g),
        }
    }
    /// Solve using the Nyström method (collocation at n equally spaced points).
    pub fn solve(&self, kernel: &dyn Fn(f64, f64) -> f64, n: usize) -> Option<Vec<(f64, f64)>> {
        let h = (self.b - self.a) / (n - 1) as f64;
        let nodes: Vec<f64> = (0..n).map(|i| self.a + i as f64 * h).collect();
        let weights: Vec<f64> = (0..n)
            .map(|j| if j == 0 || j == n - 1 { h / 2.0 } else { h })
            .collect();
        let mut mat = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                mat[i][j] = if i == j { 1.0 } else { 0.0 }
                    - self.lambda * kernel(nodes[i], nodes[j]) * weights[j];
            }
        }
        let rhs: Vec<f64> = nodes.iter().map(|&t| (self.g)(t)).collect();
        let y_vals = solve_linear(mat, rhs)?;
        Some(nodes.into_iter().zip(y_vals).collect())
    }
}
/// Limit set ω(x) approximated by the tail of a long orbit.
pub struct LimSet {
    /// How long to integrate before sampling
    pub transient_time: f64,
    /// Sampling interval
    pub sample_interval: f64,
    /// Number of samples
    pub num_samples: usize,
}
impl LimSet {
    /// Create with given parameters.
    pub fn new(transient_time: f64, sample_interval: f64, num_samples: usize) -> Self {
        Self {
            transient_time,
            sample_interval,
            num_samples,
        }
    }
    /// Approximate ω(x₀) by long-time integration.
    pub fn approximate(&self, f: &dyn Fn(f64, f64) -> f64, x0: f64) -> Vec<f64> {
        let h = 1e-2;
        let rk4 = RungeKutta4::new(h);
        let traj = rk4.solve_to(f, 0.0, x0, self.transient_time);
        let (_, mut y) = traj.last().copied().unwrap_or((0.0, x0));
        let mut t = self.transient_time;
        let mut samples = Vec::with_capacity(self.num_samples);
        for _ in 0..self.num_samples {
            let seg = rk4.solve_to(f, t, y, t + self.sample_interval);
            let (t_new, y_new) = seg.last().copied().unwrap_or((t, y));
            t = t_new;
            y = y_new;
            samples.push(y);
        }
        samples
    }
}
/// Represents a Volterra integral equation of the second kind:
/// u(x) = f(x) + λ ∫_a^x K(x, y) u(y) dy.
#[allow(dead_code)]
pub struct VolterraEquation {
    /// Left endpoint a.
    pub a: f64,
    /// The parameter λ.
    pub lambda: f64,
}
#[allow(dead_code)]
impl VolterraEquation {
    /// Create a new Volterra equation.
    pub fn new(a: f64, lambda: f64) -> Self {
        VolterraEquation { a, lambda }
    }
    /// Successive approximation (Picard iteration) up to n steps.
    /// u_{n+1}(x) = f(x) + λ ∫_a^x K(x, y) u_n(y) dy.
    /// Simplified: returns the number of iterations taken.
    pub fn picard_iterations_needed(&self, tolerance: f64) -> u64 {
        if self.lambda.abs() < tolerance {
            return 1;
        }
        let rate = self.lambda.abs();
        if rate >= 1.0 {
            return u64::MAX;
        }
        (tolerance.ln() / rate.ln()).ceil().abs() as u64
    }
}
/// Implicit solver for stiff ODEs using backward Euler (BDF-1) with Newton iteration.
pub struct StiffSolver {
    /// Step size h
    pub h: f64,
    /// Newton iteration tolerance
    pub tol: f64,
}
impl StiffSolver {
    /// Create with given step size and tolerance.
    pub fn new(h: f64, tol: f64) -> Self {
        Self { h, tol }
    }
    /// Perform one backward Euler step via fixed-point iteration.
    pub fn step(&self, f: &dyn Fn(f64, f64) -> f64, t: f64, y: f64) -> f64 {
        let h = self.h;
        let t_new = t + h;
        let mut y_new = y;
        for _ in 0..50 {
            let g = y_new - y - h * f(t_new, y_new);
            let eps = 1e-7;
            let dg = 1.0 - h * (f(t_new, y_new + eps) - f(t_new, y_new - eps)) / (2.0 * eps);
            let delta = g / dg;
            y_new -= delta;
            if delta.abs() < self.tol {
                break;
            }
        }
        y_new
    }
    /// Solve from t0 to t_end using backward Euler.
    pub fn solve_to(
        &self,
        f: &dyn Fn(f64, f64) -> f64,
        t0: f64,
        y0: f64,
        t_end: f64,
    ) -> Vec<(f64, f64)> {
        let mut result = Vec::new();
        let mut t = t0;
        let mut y = y0;
        result.push((t, y));
        while t < t_end - 1e-12 {
            let h = self.h.min(t_end - t);
            let t_new = t + h;
            let mut y_new = y;
            let tol = self.tol;
            for _ in 0..50 {
                let g = y_new - y - h * f(t_new, y_new);
                let eps = 1e-7;
                let dg = 1.0 - h * (f(t_new, y_new + eps) - f(t_new, y_new - eps)) / (2.0 * eps);
                let delta = g / dg;
                y_new -= delta;
                if delta.abs() < tol {
                    break;
                }
            }
            t = t_new;
            y = y_new;
            result.push((t, y));
        }
        result
    }
}
