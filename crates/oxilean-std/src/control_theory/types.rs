//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// PID controller state with integral anti-windup clamping.
///
/// Implements the standard proportional-integral-derivative control law:
/// u(t) = kp * e(t) + ki * ∫e(t)dt + kd * de/dt
///
/// Anti-windup clamps the integral term to `[-integral_limit, integral_limit]`.
pub struct PidController {
    /// Proportional gain.
    pub kp: f64,
    /// Integral gain.
    pub ki: f64,
    /// Derivative gain.
    pub kd: f64,
    /// Target setpoint.
    pub setpoint: f64,
    /// Anti-windup integral clamp (absolute value).
    pub integral_limit: f64,
    /// Output saturation limit (absolute value, 0 means no limit).
    pub output_limit: f64,
    integral: f64,
    prev_error: Option<f64>,
}
impl PidController {
    /// Create a new PID controller with given gains and setpoint.
    /// Anti-windup integral limit defaults to `f64::MAX` (no clamping).
    pub fn new(kp: f64, ki: f64, kd: f64, setpoint: f64) -> Self {
        PidController {
            kp,
            ki,
            kd,
            setpoint,
            integral_limit: f64::MAX,
            output_limit: 0.0,
            integral: 0.0,
            prev_error: None,
        }
    }
    /// Create a PID controller with explicit anti-windup and output saturation.
    pub fn with_limits(
        kp: f64,
        ki: f64,
        kd: f64,
        setpoint: f64,
        integral_limit: f64,
        output_limit: f64,
    ) -> Self {
        PidController {
            kp,
            ki,
            kd,
            setpoint,
            integral_limit,
            output_limit,
            integral: 0.0,
            prev_error: None,
        }
    }
    /// Compute the control output given a measurement and time step `dt`.
    pub fn update(&mut self, measurement: f64, dt: f64) -> f64 {
        let error = self.setpoint - measurement;
        self.integral =
            (self.integral + error * dt).clamp(-self.integral_limit, self.integral_limit);
        let derivative = match self.prev_error {
            Some(prev) => (error - prev) / dt,
            None => 0.0,
        };
        self.prev_error = Some(error);
        let raw = self.kp * error + self.ki * self.integral + self.kd * derivative;
        if self.output_limit > 0.0 {
            raw.clamp(-self.output_limit, self.output_limit)
        } else {
            raw
        }
    }
    /// Reset integral accumulator and derivative memory.
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = None;
    }
    /// Change the setpoint.
    pub fn set_setpoint(&mut self, sp: f64) {
        self.setpoint = sp;
    }
    /// Return the current integral accumulator value.
    pub fn integral_state(&self) -> f64 {
        self.integral
    }
}
/// Full state-space model in continuous time:
///   dx/dt = A x + B u
///   y     = C x + D u
pub struct StateSpaceModel {
    /// n×n state matrix A.
    pub a: Vec<Vec<f64>>,
    /// n×m input matrix B.
    pub b: Vec<Vec<f64>>,
    /// p×n output matrix C.
    pub c: Vec<Vec<f64>>,
    /// p×m feedthrough matrix D.
    pub d: Vec<Vec<f64>>,
    /// Number of state variables n.
    pub n_states: usize,
    /// Number of inputs m.
    pub n_inputs: usize,
    /// Number of outputs p.
    pub n_outputs: usize,
}
impl StateSpaceModel {
    /// Construct from matrices A, B, C, D.
    pub fn new(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>, c: Vec<Vec<f64>>, d: Vec<Vec<f64>>) -> Self {
        let n_states = a.len();
        let n_inputs = if b.is_empty() { 0 } else { b[0].len() };
        let n_outputs = c.len();
        StateSpaceModel {
            a,
            b,
            c,
            d,
            n_states,
            n_inputs,
            n_outputs,
        }
    }
    /// Construct with D = 0 (no feedthrough).
    pub fn no_feedthrough(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>, c: Vec<Vec<f64>>) -> Self {
        let p = c.len();
        let m = if b.is_empty() { 0 } else { b[0].len() };
        let d = vec![vec![0.0; m]; p];
        Self::new(a, b, c, d)
    }
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }
    /// Euler integration step: x_{k+1} = x_k + dt * (A x_k + B u_k).
    pub fn euler_step(&self, state: &[f64], input: &[f64], dt: f64) -> Vec<f64> {
        let ax = Self::mat_vec(&self.a, state);
        let bu = Self::mat_vec(&self.b, input);
        let deriv = Self::vec_add(&ax, &bu);
        state
            .iter()
            .zip(deriv.iter())
            .map(|(&x, &d)| x + dt * d)
            .collect()
    }
    /// Compute output: y = C x + D u.
    pub fn output(&self, state: &[f64], input: &[f64]) -> Vec<f64> {
        let cx = Self::mat_vec(&self.c, state);
        let du = Self::mat_vec(&self.d, input);
        Self::vec_add(&cx, &du)
    }
    /// Simulate for a sequence of inputs, returning all states (including initial).
    pub fn simulate(&self, initial: Vec<f64>, inputs: &[Vec<f64>], dt: f64) -> Vec<Vec<f64>> {
        let mut states = vec![initial];
        for inp in inputs {
            let next = self.euler_step(
                states
                    .last()
                    .expect("states is non-empty: initialized with one element"),
                inp,
                dt,
            );
            states.push(next);
        }
        states
    }
    /// Crude stability check: all diagonal elements of A negative.
    pub fn is_stable_diagonal(&self) -> bool {
        (0..self.n_states)
            .all(|i| self.a.get(i).and_then(|r| r.get(i)).copied().unwrap_or(0.0) < 0.0)
    }
    /// Trace of A.
    pub fn trace_a(&self) -> f64 {
        (0..self.n_states)
            .map(|i| self.a.get(i).and_then(|r| r.get(i)).copied().unwrap_or(0.0))
            .sum()
    }
}
/// LQR solver via iterative solution of the discrete-time algebraic Riccati equation.
///
/// Minimises J = Σ (x^T Q x + u^T R u) subject to x_{k+1} = A x_k + B u_k.
pub struct LqrSolver {
    /// State weight matrix Q (n×n, positive semi-definite).
    pub q: Vec<Vec<f64>>,
    /// Input weight matrix R (m×m, positive definite).
    pub r: Vec<Vec<f64>>,
    /// Maximum iterations for Riccati iteration.
    pub max_iter: usize,
    /// Convergence tolerance for Riccati iteration.
    pub tol: f64,
}
impl LqrSolver {
    /// Create an LQR solver with given Q and R weight matrices.
    pub fn new(q: Vec<Vec<f64>>, r: Vec<Vec<f64>>) -> Self {
        LqrSolver {
            q,
            r,
            max_iter: 1000,
            tol: 1e-10,
        }
    }
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows = a.len();
        let cols = if b.is_empty() { 0 } else { b[0].len() };
        let inner = b.len();
        let mut result = vec![vec![0.0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
    fn mat_add(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        a.iter()
            .zip(b.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x + y).collect())
            .collect()
    }
    fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        let mut t = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j][i] = m[i][j];
            }
        }
        t
    }
    fn mat_inv(m: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
        let n = m.len();
        let mut aug: Vec<Vec<f64>> = m
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut r = row.clone();
                let mut id = vec![0.0; n];
                id[i] = 1.0;
                r.extend(id);
                r
            })
            .collect();
        for col in 0..n {
            let pivot = (col..n).max_by(|&a, &b| {
                aug[a][col]
                    .abs()
                    .partial_cmp(&aug[b][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;
            aug.swap(col, pivot);
            let diag = aug[col][col];
            if diag.abs() < 1e-14 {
                return None;
            }
            for j in 0..2 * n {
                aug[col][j] /= diag;
            }
            for i in 0..n {
                if i != col {
                    let factor = aug[i][col];
                    for j in 0..2 * n {
                        let v = aug[col][j];
                        aug[i][j] -= factor * v;
                    }
                }
            }
        }
        Some(aug.iter().map(|row| row[n..].to_vec()).collect())
    }
    fn frobenius_norm(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
        a.iter()
            .zip(b.iter())
            .flat_map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| (x - y).powi(2)))
            .sum::<f64>()
            .sqrt()
    }
    /// Solve the discrete-time algebraic Riccati equation (DARE) iteratively.
    ///
    /// Returns the solution matrix P and the optimal gain K = (R + B^T P B)^{-1} B^T P A,
    /// or `None` if the iteration does not converge.
    pub fn solve(&self, a: &[Vec<f64>], b: &[Vec<f64>]) -> Option<(Vec<Vec<f64>>, Vec<Vec<f64>>)> {
        let n = a.len();
        let mut p = self.q.clone();
        let at = Self::transpose(a);
        let bt = Self::transpose(b);
        for _ in 0..self.max_iter {
            let pb = Self::mat_mul(&p, b);
            let bt_pb = Self::mat_mul(&bt, &pb);
            let s = Self::mat_add(&self.r, &bt_pb);
            let s_inv = Self::mat_inv(&s)?;
            let bt_pa = Self::mat_mul(&bt, &Self::mat_mul(&p, a));
            let s_inv_bt_pa = Self::mat_mul(&s_inv, &bt_pa);
            let pb_s_inv = Self::mat_mul(&pb, &s_inv_bt_pa);
            let corr = Self::mat_mul(&at, &pb_s_inv);
            let at_pa = Self::mat_mul(&at, &Self::mat_mul(&p, a));
            let mut p_new = vec![vec![0.0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    p_new[i][j] = self.q[i][j] + at_pa[i][j] - corr[i][j];
                }
            }
            if Self::frobenius_norm(&p_new, &p) < self.tol {
                let k = s_inv_bt_pa;
                return Some((p_new, k));
            }
            p = p_new;
        }
        None
    }
    /// Compute LQR quadratic cost for a trajectory.
    pub fn trajectory_cost(&self, states: &[Vec<f64>], inputs: &[Vec<f64>]) -> f64 {
        let state_cost: f64 = states
            .iter()
            .map(|x| {
                let qx = Self::mat_vec(&self.q, x);
                x.iter().zip(qx.iter()).map(|(xi, qi)| xi * qi).sum::<f64>()
            })
            .sum();
        let input_cost: f64 = inputs
            .iter()
            .map(|u| {
                let ru = Self::mat_vec(&self.r, u);
                u.iter().zip(ru.iter()).map(|(ui, ri)| ui * ri).sum::<f64>()
            })
            .sum();
        state_cost + input_cost
    }
}
/// Continuous-time transfer function G(s) = num(s) / den(s).
///
/// Numerator and denominator polynomials stored in descending degree order:
/// `[a_n, a_{n-1}, …, a_0]` represents a_n s^n + … + a_0.
#[allow(dead_code)]
pub struct TransferFunction {
    /// Numerator polynomial coefficients (descending degree).
    pub num: Vec<f64>,
    /// Denominator polynomial coefficients (descending degree).
    pub den: Vec<f64>,
}
impl TransferFunction {
    /// Create a new transfer function from numerator and denominator.
    pub fn new(num: Vec<f64>, den: Vec<f64>) -> Self {
        TransferFunction { num, den }
    }
    /// Create a first-order low-pass filter 1/(τs + 1).
    pub fn first_order_lp(tau: f64) -> Self {
        TransferFunction {
            num: vec![1.0],
            den: vec![tau, 1.0],
        }
    }
    /// Create a second-order system ω_n² / (s² + 2ζω_n s + ω_n²).
    pub fn second_order(omega_n: f64, zeta: f64) -> Self {
        let wn2 = omega_n * omega_n;
        TransferFunction {
            num: vec![wn2],
            den: vec![1.0, 2.0 * zeta * omega_n, wn2],
        }
    }
    /// Evaluate polynomial at s = jω (real, imaginary parts).
    fn eval_poly(coeffs: &[f64], omega: f64) -> (f64, f64) {
        let n = coeffs.len();
        let mut re = 0.0f64;
        let mut im = 0.0f64;
        for (k, &c) in coeffs.iter().enumerate() {
            let power = (n - 1 - k) as u32;
            match power % 4 {
                0 => re += c * omega.powi(power as i32),
                1 => im += c * omega.powi(power as i32),
                2 => re -= c * omega.powi(power as i32),
                3 => im -= c * omega.powi(power as i32),
                _ => {}
            }
        }
        (re, im)
    }
    /// Evaluate G(jω): returns (real, imaginary) parts.
    pub fn eval_jw(&self, omega: f64) -> (f64, f64) {
        let (nr, ni) = Self::eval_poly(&self.num, omega);
        let (dr, di) = Self::eval_poly(&self.den, omega);
        let denom = dr * dr + di * di;
        if denom < 1e-30 {
            return (f64::INFINITY, f64::INFINITY);
        }
        let re = (nr * dr + ni * di) / denom;
        let im = (ni * dr - nr * di) / denom;
        (re, im)
    }
    /// Magnitude |G(jω)|.
    pub fn magnitude(&self, omega: f64) -> f64 {
        let (re, im) = self.eval_jw(omega);
        (re * re + im * im).sqrt()
    }
    /// Phase angle arg(G(jω)) in radians.
    pub fn phase(&self, omega: f64) -> f64 {
        let (re, im) = self.eval_jw(omega);
        im.atan2(re)
    }
    /// Gain in decibels: 20 log₁₀ |G(jω)|.
    pub fn gain_db(&self, omega: f64) -> f64 {
        20.0 * self.magnitude(omega).log10()
    }
    /// DC gain G(0) (magnitude at ω = 0).
    pub fn dc_gain(&self) -> f64 {
        self.magnitude(0.0)
    }
    /// Order of the transfer function (degree of denominator).
    pub fn order(&self) -> usize {
        self.den.len().saturating_sub(1)
    }
}
/// Simple model reference adaptive controller (MRAC).
///
/// Implements the MIT rule for parameter adaptation:
///   θ̇ = -γ · e · ∂y/∂θ
/// where e is the tracking error and γ is the adaptation gain.
#[allow(dead_code)]
pub struct MracController {
    /// Adaptive gain γ > 0.
    pub gamma: f64,
    /// Reference model pole (reference system time constant τ_m).
    pub tau_m: f64,
    /// Current adaptive parameter estimate θ.
    pub theta: f64,
    /// Current reference model state x_m.
    pub x_m: f64,
}
impl MracController {
    /// Create a new MRAC controller.
    pub fn new(gamma: f64, tau_m: f64, theta_init: f64) -> Self {
        MracController {
            gamma,
            tau_m,
            theta: theta_init,
            x_m: 0.0,
        }
    }
    /// Update reference model: dx_m/dt = -x_m/τ_m + r/τ_m, Euler step.
    pub fn update_reference_model(&mut self, r: f64, dt: f64) {
        let dx_m = (-self.x_m + r) / self.tau_m;
        self.x_m += dt * dx_m;
    }
    /// Adapt the parameter θ using the MIT rule.
    /// `e` = tracking error (y - x_m), `sensitivity` = ∂y/∂θ.
    pub fn adapt(&mut self, e: f64, sensitivity: f64, dt: f64) {
        self.theta -= self.gamma * e * sensitivity * dt;
    }
    /// Compute the control signal: u = θ · r (simplified adaptive law).
    pub fn control(&self, r: f64) -> f64 {
        self.theta * r
    }
    /// Tracking error: y - x_m.
    pub fn tracking_error(&self, y: f64) -> f64 {
        y - self.x_m
    }
    /// Reset reference model state.
    pub fn reset(&mut self) {
        self.x_m = 0.0;
        self.theta = 1.0;
    }
}
/// Result of a frequency sweep over a range of frequencies.
#[allow(dead_code)]
pub struct BodeData {
    /// Angular frequencies ω (rad/s).
    pub frequencies: Vec<f64>,
    /// Gain values |G(jω)| at each frequency.
    pub magnitudes: Vec<f64>,
    /// Phase angles arg(G(jω)) in radians at each frequency.
    pub phases: Vec<f64>,
    /// Gain in dB (20 log₁₀ of magnitudes).
    pub gains_db: Vec<f64>,
}
impl BodeData {
    /// Compute Bode data for a transfer function over a logarithmic frequency sweep.
    ///
    /// `n_points` frequencies are spaced logarithmically between `omega_min` and `omega_max`.
    pub fn compute(tf: &TransferFunction, omega_min: f64, omega_max: f64, n_points: usize) -> Self {
        let mut frequencies = Vec::with_capacity(n_points);
        let mut magnitudes = Vec::with_capacity(n_points);
        let mut phases = Vec::with_capacity(n_points);
        let mut gains_db = Vec::with_capacity(n_points);
        for i in 0..n_points {
            let t = i as f64 / (n_points.max(2) - 1) as f64;
            let omega = omega_min * (omega_max / omega_min).powf(t);
            let mag = tf.magnitude(omega);
            let phase = tf.phase(omega);
            frequencies.push(omega);
            magnitudes.push(mag);
            phases.push(phase);
            gains_db.push(20.0 * mag.log10());
        }
        BodeData {
            frequencies,
            magnitudes,
            phases,
            gains_db,
        }
    }
    /// Find the gain crossover frequency (first ω where gain ≤ 1.0).
    pub fn gain_crossover_freq(&self) -> Option<f64> {
        for (i, &mag) in self.magnitudes.iter().enumerate() {
            if mag <= 1.0 {
                return Some(self.frequencies[i]);
            }
        }
        None
    }
    /// Phase margin in degrees at the gain crossover frequency.
    pub fn phase_margin_deg(&self) -> Option<f64> {
        let idx = self.magnitudes.iter().position(|&m| m <= 1.0)?;
        let phase_rad = self.phases[idx];
        Some(180.0 + phase_rad.to_degrees())
    }
    /// Bandwidth: frequency at which gain drops below 1/√2 ≈ 0.707 (−3 dB).
    pub fn bandwidth(&self) -> Option<f64> {
        let dc = *self.magnitudes.first()?;
        let threshold = dc / 2.0_f64.sqrt();
        self.magnitudes
            .iter()
            .zip(self.frequencies.iter())
            .find(|(&m, _)| m < threshold)
            .map(|(_, &f)| f)
    }
}
/// Linear time-invariant system in state-space form.
///
/// Dynamics: dx/dt = A x + B u
/// Output:   y     = C x + D u   (D is implicit zero here)
pub struct LtiSystem {
    /// n×n state matrix A.
    pub a: Vec<Vec<f64>>,
    /// n×m input matrix B.
    pub b: Vec<Vec<f64>>,
    /// p×n output matrix C.
    pub c: Vec<Vec<f64>>,
    /// Number of state variables.
    pub n_states: usize,
    /// Number of inputs.
    pub n_inputs: usize,
    /// Number of outputs.
    pub n_outputs: usize,
}
impl LtiSystem {
    /// Create a new LTI system from matrices A, B, C.
    pub fn new(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>, c: Vec<Vec<f64>>) -> Self {
        let n_states = a.len();
        let n_inputs = if b.is_empty() { 0 } else { b[0].len() };
        let n_outputs = c.len();
        LtiSystem {
            a,
            b,
            c,
            n_states,
            n_inputs,
            n_outputs,
        }
    }
    /// Matrix-vector multiplication: returns M * v.
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    /// Compute one Euler integration step: x_{k+1} = x_k + dt * (A x_k + B u_k).
    pub fn euler_step(&self, state: &[f64], input: &[f64], dt: f64) -> Vec<f64> {
        let ax = Self::mat_vec(&self.a, state);
        let bu = Self::mat_vec(&self.b, input);
        state
            .iter()
            .zip(ax.iter().zip(bu.iter()))
            .map(|(&x, (&ax_i, &bu_i))| x + dt * (ax_i + bu_i))
            .collect()
    }
    /// Compute the output y = C x (D assumed zero).
    pub fn output(&self, state: &[f64], _input: &[f64]) -> Vec<f64> {
        Self::mat_vec(&self.c, state)
    }
    /// Simulate the system from an initial state with a sequence of inputs.
    ///
    /// Returns the sequence of states (including the initial state).
    pub fn simulate(&self, initial: Vec<f64>, inputs: &[Vec<f64>], dt: f64) -> Vec<Vec<f64>> {
        let mut states = vec![initial];
        for inp in inputs {
            let next = self.euler_step(
                states
                    .last()
                    .expect("states is non-empty: initialized with one element"),
                inp,
                dt,
            );
            states.push(next);
        }
        states
    }
    /// Crude stability check: returns true if all diagonal elements of A are negative.
    ///
    /// This is only a sufficient condition for stability of diagonal systems.
    pub fn is_stable_diagonal(&self) -> bool {
        (0..self.n_states).all(|i| {
            self.a
                .get(i)
                .and_then(|row| row.get(i))
                .copied()
                .unwrap_or(0.0)
                < 0.0
        })
    }
    /// Compute the trace of the A matrix (sum of diagonal elements).
    pub fn trace_a(&self) -> f64 {
        (0..self.n_states)
            .map(|i| {
                self.a
                    .get(i)
                    .and_then(|row| row.get(i))
                    .copied()
                    .unwrap_or(0.0)
            })
            .sum()
    }
}
/// Multi-dimensional Kalman filter state.
///
/// State model:  x_{k+1} = F x_k + B u_k + w_k  (w ~ N(0, Q))
/// Measurement:  z_k     = H x_k + v_k           (v ~ N(0, R))
pub struct KalmanFilterState {
    /// State estimate vector (n).
    pub x: Vec<f64>,
    /// Error covariance matrix (n×n).
    pub p: Vec<Vec<f64>>,
    /// Process noise covariance matrix Q (n×n).
    pub q: Vec<Vec<f64>>,
    /// Measurement noise covariance matrix R (m×m).
    pub r: Vec<Vec<f64>>,
    /// State transition matrix F (n×n).
    pub f: Vec<Vec<f64>>,
    /// Observation matrix H (m×n).
    pub h: Vec<Vec<f64>>,
    n: usize,
    m: usize,
}
impl KalmanFilterState {
    /// Create a new multi-dimensional Kalman filter.
    pub fn new(
        x0: Vec<f64>,
        p0: Vec<Vec<f64>>,
        f: Vec<Vec<f64>>,
        h: Vec<Vec<f64>>,
        q: Vec<Vec<f64>>,
        r: Vec<Vec<f64>>,
    ) -> Self {
        let n = x0.len();
        let m = h.len();
        KalmanFilterState {
            x: x0,
            p: p0,
            q,
            r,
            f,
            h,
            n,
            m,
        }
    }
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows = a.len();
        let cols = if b.is_empty() { 0 } else { b[0].len() };
        let inner = b.len();
        let mut result = vec![vec![0.0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
    fn mat_add(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        a.iter()
            .zip(b.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x + y).collect())
            .collect()
    }
    fn mat_sub(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        a.iter()
            .zip(b.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x - y).collect())
            .collect()
    }
    fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        let mut t = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j][i] = m[i][j];
            }
        }
        t
    }
    /// Invert a small matrix using Gaussian elimination.
    fn mat_inv(m: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
        let n = m.len();
        let mut aug: Vec<Vec<f64>> = m
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut r = row.clone();
                let mut id = vec![0.0; n];
                id[i] = 1.0;
                r.extend(id);
                r
            })
            .collect();
        for col in 0..n {
            let pivot = (col..n).max_by(|&a, &b| {
                aug[a][col]
                    .abs()
                    .partial_cmp(&aug[b][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;
            aug.swap(col, pivot);
            let diag = aug[col][col];
            if diag.abs() < 1e-14 {
                return None;
            }
            for j in 0..2 * n {
                aug[col][j] /= diag;
            }
            for i in 0..n {
                if i != col {
                    let factor = aug[i][col];
                    for j in 0..2 * n {
                        let v = aug[col][j];
                        aug[i][j] -= factor * v;
                    }
                }
            }
        }
        Some(aug.iter().map(|row| row[n..].to_vec()).collect())
    }
    fn identity(n: usize) -> Vec<Vec<f64>> {
        let mut id = vec![vec![0.0; n]; n];
        for i in 0..n {
            id[i][i] = 1.0;
        }
        id
    }
    /// Kalman predict step: x = F x, P = F P F^T + Q.
    pub fn predict(&mut self) {
        self.x = Self::mat_vec(&self.f, &self.x);
        let fp = Self::mat_mul(&self.f, &self.p);
        let ft = Self::transpose(&self.f);
        let fpft = Self::mat_mul(&fp, &ft);
        self.p = Self::mat_add(&fpft, &self.q);
    }
    /// Kalman update step given measurement z.
    /// Returns the innovation (z - H x) for diagnostics.
    pub fn update(&mut self, z: &[f64]) -> Vec<f64> {
        let hx = Self::mat_vec(&self.h, &self.x);
        let innovation: Vec<f64> = z.iter().zip(hx.iter()).map(|(zi, hxi)| zi - hxi).collect();
        let ht = Self::transpose(&self.h);
        let ph_t = Self::mat_mul(&self.p, &ht);
        let h_p = Self::mat_mul(&self.h, &self.p);
        let s = Self::mat_add(&Self::mat_mul(&h_p, &ht), &self.r);
        if let Some(s_inv) = Self::mat_inv(&s) {
            let k = Self::mat_mul(&ph_t, &s_inv);
            let k_innov = Self::mat_vec(&k, &innovation);
            for (xi, ki) in self.x.iter_mut().zip(k_innov.iter()) {
                *xi += ki;
            }
            let kh = Self::mat_mul(&k, &self.h);
            let i_kh = Self::mat_sub(&Self::identity(self.n), &kh);
            self.p = Self::mat_mul(&i_kh, &self.p);
        }
        innovation
    }
    /// Return current state estimate.
    pub fn state(&self) -> &[f64] {
        &self.x
    }
    /// Return dimension n (state size) and m (measurement size).
    pub fn dims(&self) -> (usize, usize) {
        (self.n, self.m)
    }
}
/// Sliding mode controller for a scalar system.
///
/// Switching surface: s(x) = c^T x (linear)
/// Control law: u = -k * sign(s(x)) + u_eq (equivalent control)
#[allow(dead_code)]
pub struct SlidingModeController {
    /// Sliding surface coefficients c (n-vector).
    pub c: Vec<f64>,
    /// Switching gain k.
    pub k: f64,
    /// Boundary layer thickness (0 = hard switching, >0 = smooth approximation).
    pub boundary: f64,
}
impl SlidingModeController {
    /// Create a new sliding mode controller.
    pub fn new(c: Vec<f64>, k: f64, boundary: f64) -> Self {
        SlidingModeController { c, k, boundary }
    }
    /// Evaluate the sliding variable s = c^T x.
    pub fn sliding_variable(&self, state: &[f64]) -> f64 {
        self.c
            .iter()
            .zip(state.iter())
            .map(|(ci, xi)| ci * xi)
            .sum()
    }
    /// Compute the discontinuous switching term.
    /// Uses a saturation function if boundary > 0 (smooth approximation).
    pub fn switching_term(&self, s: f64) -> f64 {
        if self.boundary > 1e-12 {
            (s / self.boundary).clamp(-1.0, 1.0)
        } else {
            if s > 0.0 {
                1.0
            } else if s < 0.0 {
                -1.0
            } else {
                0.0
            }
        }
    }
    /// Compute control action for state x (no equivalent control).
    pub fn control(&self, state: &[f64]) -> f64 {
        let s = self.sliding_variable(state);
        -self.k * self.switching_term(s)
    }
    /// Control with an equivalent control feedforward term u_eq.
    pub fn control_with_eq(&self, state: &[f64], u_eq: f64) -> f64 {
        self.control(state) + u_eq
    }
    /// Check if state is inside the boundary layer |s| < boundary.
    pub fn in_boundary_layer(&self, state: &[f64]) -> bool {
        let s = self.sliding_variable(state);
        self.boundary > 0.0 && s.abs() < self.boundary
    }
    /// Distance to sliding surface |s(x)|.
    pub fn distance_to_surface(&self, state: &[f64]) -> f64 {
        self.sliding_variable(state).abs()
    }
}
/// Model Predictive Controller with a finite horizon and simple box constraints.
///
/// At each step, it minimises the horizon cost using a greedy LQR-like gain,
/// subject to box constraints `[-u_max, u_max]` on each input dimension.
pub struct MpcController {
    /// System A matrix (n×n).
    pub a: Vec<Vec<f64>>,
    /// System B matrix (n×m).
    pub b: Vec<Vec<f64>>,
    /// State weight Q (n×n).
    pub q: Vec<Vec<f64>>,
    /// Input weight R (m×m).
    pub r: Vec<Vec<f64>>,
    /// Prediction/control horizon (steps).
    pub horizon: usize,
    /// Input constraint: each u_i ∈ [-u_max\[i\], u_max\[i\]]. Empty = no constraint.
    pub u_max: Vec<f64>,
}
impl MpcController {
    /// Create a new MPC controller.
    pub fn new(
        a: Vec<Vec<f64>>,
        b: Vec<Vec<f64>>,
        q: Vec<Vec<f64>>,
        r: Vec<Vec<f64>>,
        horizon: usize,
        u_max: Vec<f64>,
    ) -> Self {
        MpcController {
            a,
            b,
            q,
            r,
            horizon,
            u_max,
        }
    }
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows = a.len();
        let cols = if b.is_empty() { 0 } else { b[0].len() };
        let inner = b.len();
        let mut result = vec![vec![0.0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
    fn mat_add(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        a.iter()
            .zip(b.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x + y).collect())
            .collect()
    }
    fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        let mut t = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                t[j][i] = m[i][j];
            }
        }
        t
    }
    fn mat_inv(m: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
        let n = m.len();
        let mut aug: Vec<Vec<f64>> = m
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut r = row.clone();
                let mut id = vec![0.0; n];
                id[i] = 1.0;
                r.extend(id);
                r
            })
            .collect();
        for col in 0..n {
            let pivot = (col..n).max_by(|&a, &b| {
                aug[a][col]
                    .abs()
                    .partial_cmp(&aug[b][col].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;
            aug.swap(col, pivot);
            let diag = aug[col][col];
            if diag.abs() < 1e-14 {
                return None;
            }
            for j in 0..2 * n {
                aug[col][j] /= diag;
            }
            for i in 0..n {
                if i != col {
                    let factor = aug[i][col];
                    for j in 0..2 * n {
                        let v = aug[col][j];
                        aug[i][j] -= factor * v;
                    }
                }
            }
        }
        Some(aug.iter().map(|row| row[n..].to_vec()).collect())
    }
    /// Compute the control action at the current state using a greedy one-step look-ahead.
    ///
    /// Solves min_{u} x^T Q x + u^T R u  subject to ||u||_∞ ≤ u_max,
    /// where the next state is predicted as A x + B u.
    ///
    /// Returns `None` if R is singular.
    pub fn control(&self, state: &[f64]) -> Option<Vec<f64>> {
        let bt = Self::transpose(&self.b);
        let bt_q = Self::mat_mul(&bt, &self.q);
        let bt_q_a = Self::mat_mul(&bt_q, &self.a);
        let bt_q_b = Self::mat_mul(&bt_q, &self.b);
        let lhs = Self::mat_add(&self.r, &bt_q_b);
        let lhs_inv = Self::mat_inv(&lhs)?;
        let rhs = Self::mat_vec(&bt_q_a, state);
        let mut u: Vec<f64> = Self::mat_vec(&lhs_inv, &rhs).iter().map(|&v| -v).collect();
        if !self.u_max.is_empty() {
            for (ui, &umax) in u.iter_mut().zip(self.u_max.iter()) {
                *ui = ui.clamp(-umax, umax);
            }
        }
        Some(u)
    }
    /// Simulate MPC closed-loop for `steps` steps from initial state.
    ///
    /// Returns `(states, inputs)` where states has length `steps+1`.
    pub fn simulate(
        &self,
        initial: Vec<f64>,
        steps: usize,
        dt: f64,
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut states = vec![initial];
        let mut inputs = Vec::new();
        for _ in 0..steps {
            let x = states
                .last()
                .expect("states is non-empty: initialized with one element");
            let u = self
                .control(x)
                .unwrap_or_else(|| vec![0.0; self.b[0].len()]);
            let ax = Self::mat_vec(&self.a, x);
            let bu = Self::mat_vec(&self.b, &u);
            let next: Vec<f64> = ax
                .iter()
                .zip(bu.iter())
                .zip(x.iter())
                .map(|((&axi, &bui), &xi)| xi + dt * (axi + bui))
                .collect();
            inputs.push(u);
            states.push(next);
        }
        (states, inputs)
    }
    /// Return the horizon length.
    pub fn horizon(&self) -> usize {
        self.horizon
    }
}
/// Discrete-time linear system: x_{k+1} = A x_k + B u_k, y_k = C x_k + D u_k.
#[allow(dead_code)]
pub struct DiscreteStateSpace {
    /// n×n state transition matrix A.
    pub a: Vec<Vec<f64>>,
    /// n×m input matrix B.
    pub b: Vec<Vec<f64>>,
    /// p×n output matrix C.
    pub c: Vec<Vec<f64>>,
    /// p×m feedthrough matrix D.
    pub d: Vec<Vec<f64>>,
    /// Sample period T_s (seconds).
    pub ts: f64,
    /// Number of states.
    pub n_states: usize,
    /// Number of inputs.
    pub n_inputs: usize,
    /// Number of outputs.
    pub n_outputs: usize,
}
impl DiscreteStateSpace {
    /// Create a discrete-time state-space model.
    pub fn new(
        a: Vec<Vec<f64>>,
        b: Vec<Vec<f64>>,
        c: Vec<Vec<f64>>,
        d: Vec<Vec<f64>>,
        ts: f64,
    ) -> Self {
        let n_states = a.len();
        let n_inputs = if b.is_empty() { 0 } else { b[0].len() };
        let n_outputs = c.len();
        DiscreteStateSpace {
            a,
            b,
            c,
            d,
            ts,
            n_states,
            n_inputs,
            n_outputs,
        }
    }
    fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
        m.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, x)| a * x).sum())
            .collect()
    }
    fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }
    /// One step: x_{k+1} = A x_k + B u_k.
    pub fn step(&self, state: &[f64], input: &[f64]) -> Vec<f64> {
        let ax = Self::mat_vec(&self.a, state);
        let bu = Self::mat_vec(&self.b, input);
        Self::vec_add(&ax, &bu)
    }
    /// Output: y_k = C x_k + D u_k.
    pub fn output(&self, state: &[f64], input: &[f64]) -> Vec<f64> {
        let cx = Self::mat_vec(&self.c, state);
        let du = Self::mat_vec(&self.d, input);
        Self::vec_add(&cx, &du)
    }
    /// Simulate for `steps` steps, returning all states.
    pub fn simulate(&self, initial: Vec<f64>, inputs: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut states = vec![initial];
        for inp in inputs {
            let next = self.step(
                states
                    .last()
                    .expect("states is non-empty: initialized with one element"),
                inp,
            );
            states.push(next);
        }
        states
    }
    /// Spectral radius ρ(A) = max |eigenvalue| approximated by power iteration.
    /// Returns None if state dimension is 0.
    pub fn spectral_radius_approx(&self, n_iter: usize) -> Option<f64> {
        if self.n_states == 0 {
            return None;
        }
        let mut v: Vec<f64> = vec![1.0; self.n_states];
        let mut norm = 1.0f64;
        for _ in 0..n_iter {
            let av = Self::mat_vec(&self.a, &v);
            norm = av.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-30 {
                break;
            }
            v = av.iter().map(|x| x / norm).collect();
        }
        Some(norm)
    }
    /// Check discrete stability: spectral radius < 1 (approximate).
    pub fn is_stable(&self, n_iter: usize) -> bool {
        self.spectral_radius_approx(n_iter)
            .map_or(true, |r| r < 1.0)
    }
}
/// Simple 1-dimensional Kalman filter.
///
/// State model:  x_{k+1} = F * x_k + u_k  (process noise ~ N(0, Q))
/// Measurement:  z_k     = x_k             (measurement noise ~ N(0, R))
pub struct KalmanFilter1D {
    /// Current state estimate.
    pub x: f64,
    /// Estimation error covariance.
    pub p: f64,
    /// Process noise covariance.
    pub q: f64,
    /// Measurement noise covariance.
    pub r: f64,
}
impl KalmanFilter1D {
    /// Create a new Kalman filter with initial state `x0`, covariance `p0`,
    /// process noise `q`, and measurement noise `r`.
    pub fn new(x0: f64, p0: f64, q: f64, r: f64) -> Self {
        KalmanFilter1D { x: x0, p: p0, q, r }
    }
    /// Prediction step: x = F * x + u, p = F * p * F + q.
    pub fn predict(&mut self, f: f64, u: f64) {
        self.x = f * self.x + u;
        self.p = f * self.p * f + self.q;
    }
    /// Update step: incorporate measurement z.
    ///
    /// Kalman gain K = p / (p + r), then:
    ///   x = x + K * (z - x)
    ///   p = (1 - K) * p
    pub fn update(&mut self, z: f64) {
        let k = self.p / (self.p + self.r);
        self.x += k * (z - self.x);
        self.p *= 1.0 - k;
    }
    /// Return the current state estimate.
    pub fn estimate(&self) -> f64 {
        self.x
    }
    /// Return the current estimation uncertainty (covariance).
    pub fn uncertainty(&self) -> f64 {
        self.p
    }
}
