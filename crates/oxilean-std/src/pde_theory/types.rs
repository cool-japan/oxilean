//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Finite difference solver for the 1D wave equation u_tt = c² u_xx
/// with homogeneous Dirichlet boundary conditions.
#[derive(Debug, Clone)]
pub struct WaveSolver1D {
    /// Wave speed c.
    pub c: f64,
    /// Spatial mesh.
    pub mesh: Mesh1D,
    /// Solution at previous time step.
    pub u_prev: Vec<f64>,
    /// Solution at current time step.
    pub u_curr: Vec<f64>,
    /// Current time.
    pub t: f64,
}
impl WaveSolver1D {
    /// Initialize with initial displacement u₀ and velocity u₁.
    pub fn new(
        c: f64,
        mesh: Mesh1D,
        u0: impl Fn(f64) -> f64,
        u1: impl Fn(f64) -> f64,
        dt: f64,
    ) -> Self {
        let nodes_interior: Vec<f64> = mesh.nodes[1..mesh.n].to_vec();
        let u_curr: Vec<f64> = nodes_interior.iter().map(|&x| u0(x)).collect();
        let h = mesh.mesh_size();
        let r2 = (c * dt / h).powi(2);
        let n = u_curr.len();
        let u_prev: Vec<f64> = (0..n)
            .map(|i| {
                let ul = if i == 0 { 0.0 } else { u_curr[i - 1] };
                let ur = if i == n - 1 { 0.0 } else { u_curr[i + 1] };
                u_curr[i] - dt * u1(nodes_interior[i]) + 0.5 * r2 * (ul - 2.0 * u_curr[i] + ur)
            })
            .collect();
        WaveSolver1D {
            c,
            mesh,
            u_prev,
            u_curr,
            t: 0.0,
        }
    }
    /// Advance one leap-frog step with time-step dt.
    ///
    /// Stability requires c*dt/h ≤ 1 (CFL condition).
    pub fn step(&mut self, dt: f64) {
        let h = self.mesh.mesh_size();
        let r2 = (self.c * dt / h).powi(2);
        let n = self.u_curr.len();
        let mut u_next = vec![0.0; n];
        for i in 0..n {
            let ul = if i == 0 { 0.0 } else { self.u_curr[i - 1] };
            let ur = if i == n - 1 { 0.0 } else { self.u_curr[i + 1] };
            u_next[i] =
                2.0 * self.u_curr[i] - self.u_prev[i] + r2 * (ul - 2.0 * self.u_curr[i] + ur);
        }
        self.u_prev = self.u_curr.clone();
        self.u_curr = u_next;
        self.t += dt;
    }
    /// Maximum absolute value of current solution.
    pub fn max_abs(&self) -> f64 {
        self.u_curr.iter().map(|u| u.abs()).fold(0.0_f64, f64::max)
    }
}
/// Computes the effective (homogenized) coefficient A_hom for the 1D equation
/// -d/dy (A(y) d/dy χ) = d/dy A(y) on the periodic cell [0, 1], giving
/// A_hom = (∫_0^1 1/A(y) dy)^{-1}.
#[derive(Debug, Clone)]
pub struct HomogenizationApprox {
    /// Number of quadrature points for the harmonic mean.
    pub n_quad: usize,
}
impl HomogenizationApprox {
    /// Create with the given number of quadrature points.
    pub fn new(n_quad: usize) -> Self {
        HomogenizationApprox { n_quad }
    }
    /// Compute the homogenized coefficient for a periodic coefficient A(y) given
    /// by a closure, using the harmonic mean formula A_hom = (∫_0^1 1/A(y) dy)^{-1}.
    pub fn compute(&self, a: impl Fn(f64) -> f64) -> f64 {
        let h = 1.0 / self.n_quad as f64;
        let integral: f64 = (0..self.n_quad)
            .map(|i| {
                let y = (i as f64 + 0.5) * h;
                h / a(y)
            })
            .sum();
        1.0 / integral
    }
    /// For a two-phase laminate with coefficient a1 in [0, theta] and a2 in (theta, 1),
    /// the exact harmonic mean is A_hom = 1 / (theta/a1 + (1-theta)/a2).
    pub fn two_phase_exact(a1: f64, a2: f64, theta: f64) -> f64 {
        1.0 / (theta / a1 + (1.0 - theta) / a2)
    }
}
/// Checks numerically whether a discrete Strichartz-type estimate holds for
/// the linear Schrödinger propagator e^{itΔ} on a periodic grid.
#[derive(Debug, Clone)]
pub struct DispersiveEstimateChecker {
    /// Spatial grid size.
    pub n: usize,
    /// Time step for verification.
    pub dt: f64,
}
impl DispersiveEstimateChecker {
    /// Create a new checker.
    pub fn new(n: usize, dt: f64) -> Self {
        DispersiveEstimateChecker { n, dt }
    }
    /// Apply the discrete Schrödinger propagator e^{i dt Δ} (on a periodic grid).
    /// Returns the real part of the propagated function.
    pub fn propagate_re(&self, u_re: &[f64], u_im: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let n = u_re.len();
        let pi2 = 2.0 * std::f64::consts::PI;
        let mut hat_re = vec![0.0_f64; n];
        let mut hat_im = vec![0.0_f64; n];
        for k in 0..n {
            for j in 0..n {
                let angle = pi2 * k as f64 * j as f64 / n as f64;
                hat_re[k] += u_re[j] * angle.cos() + u_im[j] * angle.sin();
                hat_im[k] += -u_re[j] * angle.sin() + u_im[j] * angle.cos();
            }
        }
        for k in 0..n {
            let xi = if k <= n / 2 {
                k as f64
            } else {
                k as f64 - n as f64
            };
            let phase = self.dt * xi * xi;
            let (sin_p, cos_p) = phase.sin_cos();
            let (hr, hi) = (hat_re[k], hat_im[k]);
            hat_re[k] = hr * cos_p - hi * sin_p;
            hat_im[k] = hr * sin_p + hi * cos_p;
        }
        let mut out_re = vec![0.0_f64; n];
        let mut out_im = vec![0.0_f64; n];
        for j in 0..n {
            for k in 0..n {
                let angle = pi2 * k as f64 * j as f64 / n as f64;
                out_re[j] += hat_re[k] * angle.cos() - hat_im[k] * angle.sin();
                out_im[j] += hat_re[k] * angle.sin() + hat_im[k] * angle.cos();
            }
            out_re[j] /= n as f64;
            out_im[j] /= n as f64;
        }
        (out_re, out_im)
    }
    /// Check L² conservation: ‖e^{itΔ} u₀‖_{L²} = ‖u₀‖_{L²} (up to tolerance).
    pub fn check_l2_conservation(&self, u_re: &[f64], u_im: &[f64], tol: f64) -> bool {
        let n = u_re.len();
        let norm0 = (u_re
            .iter()
            .zip(u_im.iter())
            .map(|(r, i)| r * r + i * i)
            .sum::<f64>()
            / n as f64)
            .sqrt();
        let (pr, pi_vec) = self.propagate_re(u_re, u_im);
        let norm1 = (pr
            .iter()
            .zip(pi_vec.iter())
            .map(|(r, i)| r * r + i * i)
            .sum::<f64>()
            / n as f64)
            .sqrt();
        (norm0 - norm1).abs() < tol
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DispersiveEquationType {
    SchrodingerEquation,
    WaveEquation,
    KleinGordon(f64),
}
/// Riemann problem data for Burgers' equation: u_L left state, u_R right state.
#[derive(Debug, Clone)]
pub struct BurgersRiemann {
    /// Left state u_L.
    pub u_left: f64,
    /// Right state u_R.
    pub u_right: f64,
}
impl BurgersRiemann {
    /// Create a new Riemann problem.
    pub fn new(u_left: f64, u_right: f64) -> Self {
        BurgersRiemann { u_left, u_right }
    }
    /// Evaluate the entropy solution at position x and time t.
    ///
    /// - If u_L > u_R: shock with speed s = (u_L + u_R)/2 (Rankine-Hugoniot).
    /// - If u_L < u_R: rarefaction wave u = x/t for u_L ≤ x/t ≤ u_R.
    pub fn eval(&self, x: f64, t: f64) -> f64 {
        if t <= 0.0 {
            return if x < 0.0 { self.u_left } else { self.u_right };
        }
        let xi = x / t;
        if self.u_left >= self.u_right {
            let s = (self.u_left + self.u_right) / 2.0;
            if xi < s {
                self.u_left
            } else {
                self.u_right
            }
        } else {
            if xi <= self.u_left {
                self.u_left
            } else if xi >= self.u_right {
                self.u_right
            } else {
                xi
            }
        }
    }
    /// Rankine-Hugoniot shock speed (for the shock case u_L > u_R).
    pub fn shock_speed(&self) -> f64 {
        (self.u_left + self.u_right) / 2.0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RectifiableCurrents {
    pub dimension: usize,
    pub mass: f64,
    pub boundary_mass: f64,
    pub is_integral: bool,
}
#[allow(dead_code)]
impl RectifiableCurrents {
    pub fn new(dim: usize, mass: f64) -> Self {
        RectifiableCurrents {
            dimension: dim,
            mass,
            boundary_mass: 0.0,
            is_integral: true,
        }
    }
    pub fn federer_fleming_closure_theorem(&self) -> bool {
        self.is_integral && self.mass < f64::INFINITY
    }
    pub fn isoperimetric_inequality(&self) -> String {
        format!(
            "Isoperimetric: M(T) = {:.3} ≤ C * M(∂T)^{}/{} = C * {:.3}",
            self.mass,
            self.dimension,
            self.dimension - 1,
            self.boundary_mass
                .powf(self.dimension as f64 / (self.dimension - 1) as f64)
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NonlinearSchrodinger {
    pub nonlinearity_power: f64,
    pub spatial_dimension: usize,
    pub mass_critical: bool,
    pub energy_critical: bool,
}
#[allow(dead_code)]
impl NonlinearSchrodinger {
    pub fn new(p: f64, dim: usize) -> Self {
        let mass_crit_p = 4.0 / dim as f64;
        let energy_crit_p = 4.0 / (dim as f64 - 2.0).max(1.0);
        NonlinearSchrodinger {
            nonlinearity_power: p,
            spatial_dimension: dim,
            mass_critical: (p - mass_crit_p).abs() < 0.01,
            energy_critical: (p - energy_crit_p).abs() < 0.01,
        }
    }
    pub fn global_well_posedness_h1(&self) -> bool {
        self.nonlinearity_power < 4.0 / (self.spatial_dimension as f64 - 2.0).max(1.0)
    }
    pub fn scattering_theory_description(&self) -> String {
        if self.energy_critical {
            format!(
                "NLS (p={:.2}, d={}): energy-critical, scattering by Kenig-Merle concentration-compactness",
                self.nonlinearity_power, self.spatial_dimension
            )
        } else {
            format!(
                "NLS (p={:.2}, d={}): subcritical, scattering by Strichartz + Morawetz",
                self.nonlinearity_power, self.spatial_dimension
            )
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinimalSurface {
    pub ambient_dimension: usize,
    pub surface_dimension: usize,
    pub is_area_minimizing: bool,
    pub stability_index: i64,
    pub genus: usize,
}
#[allow(dead_code)]
impl MinimalSurface {
    pub fn plane(dim: usize) -> Self {
        MinimalSurface {
            ambient_dimension: dim,
            surface_dimension: dim - 1,
            is_area_minimizing: true,
            stability_index: 0,
            genus: 0,
        }
    }
    pub fn catenoid() -> Self {
        MinimalSurface {
            ambient_dimension: 3,
            surface_dimension: 2,
            is_area_minimizing: false,
            stability_index: 1,
            genus: 0,
        }
    }
    pub fn helicoid() -> Self {
        MinimalSurface {
            ambient_dimension: 3,
            surface_dimension: 2,
            is_area_minimizing: false,
            stability_index: 0,
            genus: 0,
        }
    }
    pub fn simons_gap_theorem(&self) -> String {
        if self.ambient_dimension <= 7 && self.stability_index == 0 {
            format!(
                "Simons gap: stable minimal {}-surface in R^{} must be a hyperplane",
                self.surface_dimension, self.ambient_dimension
            )
        } else {
            "Simons gap not applicable".to_string()
        }
    }
    pub fn mean_curvature(&self) -> f64 {
        0.0
    }
}
/// P1 finite element stiffness matrix on a 1D uniform mesh.
///
/// For the bilinear form a(u, v) = ∫ u'v' dx on H¹₀([a,b]),
/// the element stiffness contributions give the tridiagonal system (1/h)(2,-1,-1,2,...).
#[derive(Debug, Clone)]
pub struct StiffnessMatrix1D {
    /// Interior stiffness matrix (num_interior × num_interior tridiagonal).
    pub data: Vec<Vec<f64>>,
    /// Interior dimension.
    pub dim: usize,
}
impl StiffnessMatrix1D {
    /// Assemble the global stiffness matrix for P1 elements on a uniform mesh.
    pub fn assemble(mesh: &Mesh1D) -> Self {
        let n = mesh.num_interior();
        let h = mesh.mesh_size();
        let mut data = vec![vec![0.0; n]; n];
        for i in 0..n {
            data[i][i] = 2.0 / h;
            if i + 1 < n {
                data[i][i + 1] = -1.0 / h;
                data[i + 1][i] = -1.0 / h;
            }
        }
        StiffnessMatrix1D { data, dim: n }
    }
    /// Apply the matrix to a vector v (matrix-vector product).
    pub fn apply(&self, v: &[f64]) -> Vec<f64> {
        assert_eq!(v.len(), self.dim, "dimension mismatch");
        (0..self.dim)
            .map(|i| self.data[i].iter().zip(v.iter()).map(|(a, b)| a * b).sum())
            .collect()
    }
    /// Solve A x = b via Gaussian elimination (for the tridiagonal system).
    pub fn solve_tridiagonal(&self, b: &[f64]) -> Vec<f64> {
        let n = self.dim;
        assert_eq!(b.len(), n);
        if n == 0 {
            return vec![];
        }
        let mut lower = vec![0.0; n];
        let mut diag = vec![0.0; n];
        let mut upper = vec![0.0; n];
        let mut rhs = b.to_vec();
        for i in 0..n {
            diag[i] = self.data[i][i];
            if i + 1 < n {
                upper[i] = self.data[i][i + 1];
                lower[i + 1] = self.data[i + 1][i];
            }
        }
        for i in 1..n {
            let m = lower[i] / diag[i - 1];
            diag[i] -= m * upper[i - 1];
            rhs[i] -= m * rhs[i - 1];
        }
        let mut x = vec![0.0; n];
        x[n - 1] = rhs[n - 1] / diag[n - 1];
        for i in (0..n - 1).rev() {
            x[i] = (rhs[i] - upper[i] * x[i + 1]) / diag[i];
        }
        x
    }
}
/// Implicit (backward) Euler solver for the parabolic equation u_t = α u_xx + f
/// with homogeneous Dirichlet boundary conditions.  Unconditionally stable.
#[derive(Debug, Clone)]
pub struct ParabolicSolver {
    /// Diffusion coefficient α.
    pub alpha: f64,
    /// Spatial mesh.
    pub mesh: Mesh1D,
    /// Current solution at interior nodes.
    pub u: Vec<f64>,
    /// Current time.
    pub t: f64,
}
impl ParabolicSolver {
    /// Initialize from a mesh and initial condition.
    pub fn new(alpha: f64, mesh: Mesh1D, initial: impl Fn(f64) -> f64) -> Self {
        let u = mesh.nodes[1..mesh.n].iter().map(|&x| initial(x)).collect();
        ParabolicSolver {
            alpha,
            mesh,
            u,
            t: 0.0,
        }
    }
    /// Advance one implicit Euler step with time-step dt.
    /// Solves (I - dt α A) u^{n+1} = u^n where A is the 1D Laplacian.
    pub fn step(&mut self, dt: f64) {
        let h = self.mesh.mesh_size();
        let n = self.u.len();
        let r = self.alpha * dt / (h * h);
        let diag_val = 1.0 + 2.0 * r;
        let off_val = -r;
        let mut diag = vec![diag_val; n];
        let upper = vec![off_val; n];
        let lower = vec![off_val; n];
        let mut rhs = self.u.clone();
        for i in 1..n {
            let m = lower[i] / diag[i - 1];
            diag[i] -= m * upper[i - 1];
            rhs[i] -= m * rhs[i - 1];
        }
        let mut x = vec![0.0; n];
        x[n - 1] = rhs[n - 1] / diag[n - 1];
        for i in (0..n - 1).rev() {
            x[i] = (rhs[i] - upper[i] * x[i + 1]) / diag[i];
        }
        self.u = x;
        self.t += dt;
    }
    /// Advance to time t_end using the given dt.
    pub fn advance_to(&mut self, t_end: f64, dt: f64) {
        while self.t < t_end - 1e-14 {
            let actual_dt = dt.min(t_end - self.t);
            self.step(actual_dt);
        }
    }
    /// L² norm of the current solution.
    pub fn l2_norm(&self) -> f64 {
        let h = self.mesh.mesh_size();
        (self.u.iter().map(|u| u * u).sum::<f64>() * h).sqrt()
    }
}
/// Simulation of a pseudodifferential operator with symbol m(ξ) = (1 + |ξ|²)^{s/2}
/// (Bessel potential), applied via discrete Fourier transform on a periodic grid.
#[derive(Debug, Clone)]
pub struct PseudodiffOperatorSim {
    /// Sobolev order s of the Bessel potential (1 + |ξ|²)^{s/2}.
    pub order: f64,
    /// Number of grid points (should be a power of 2 for FFT efficiency).
    pub n: usize,
}
impl PseudodiffOperatorSim {
    /// Create a new pseudodifferential operator simulator.
    pub fn new(order: f64, n: usize) -> Self {
        PseudodiffOperatorSim { order, n }
    }
    /// Apply the Bessel potential (1 - Δ)^{s/2} to a function u given by its
    /// values on a uniform periodic grid.  Uses a naive DFT (O(n²)) for simplicity.
    pub fn apply(&self, u: &[f64]) -> Vec<f64> {
        let n = u.len();
        if n == 0 {
            return vec![];
        }
        let mut re = vec![0.0_f64; n];
        let mut im = vec![0.0_f64; n];
        let pi2 = 2.0 * std::f64::consts::PI;
        for k in 0..n {
            for j in 0..n {
                let angle = pi2 * k as f64 * j as f64 / n as f64;
                re[k] += u[j] * angle.cos();
                im[k] += u[j] * angle.sin();
            }
        }
        for k in 0..n {
            let xi = if k <= n / 2 {
                k as f64
            } else {
                k as f64 - n as f64
            };
            let symbol = (1.0 + xi * xi).powf(self.order / 2.0);
            re[k] *= symbol;
            im[k] *= symbol;
        }
        let mut out = vec![0.0_f64; n];
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                let angle = pi2 * k as f64 * j as f64 / n as f64;
                sum += re[k] * angle.cos() - im[k] * (-angle).sin();
            }
            out[j] = sum / n as f64;
        }
        out
    }
    /// Estimate the operator norm on L² by the maximum of |symbol| on the grid.
    pub fn l2_operator_norm_bound(&self, n: usize) -> f64 {
        (0..n)
            .map(|k| {
                let xi = if k <= n / 2 {
                    k as f64
                } else {
                    k as f64 - n as f64
                };
                (1.0 + xi * xi).powf(self.order / 2.0)
            })
            .fold(0.0_f64, f64::max)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StochasticHeatEquation {
    pub diffusion_coefficient: f64,
    pub noise_strength: f64,
    pub spatial_dimension: usize,
    pub time_horizon: f64,
    pub is_additive_noise: bool,
}
#[allow(dead_code)]
impl StochasticHeatEquation {
    pub fn new(diffusion: f64, noise: f64, dim: usize) -> Self {
        StochasticHeatEquation {
            diffusion_coefficient: diffusion,
            noise_strength: noise,
            spatial_dimension: dim,
            time_horizon: 1.0,
            is_additive_noise: true,
        }
    }
    pub fn mild_solution_regularity(&self) -> String {
        let reg = if self.spatial_dimension == 1 {
            "C([0,T]; L2) ∩ Hölder(1/4 - ε)"
        } else {
            "C([0,T]; H^{-ε})"
        };
        format!("SPDE solution regularity: {}", reg)
    }
    pub fn is_well_posed_l2(&self) -> bool {
        self.is_additive_noise || self.spatial_dimension <= 2
    }
    pub fn energy_at_time(&self, _t: f64, initial_energy: f64) -> f64 {
        let lambda = self.diffusion_coefficient;
        let sigma_sq = self.noise_strength * self.noise_strength;
        initial_energy + sigma_sq / (2.0 * lambda)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeGiorgiNashMoser {
    pub ellipticity_lambda: f64,
    pub ellipticity_lambda_upper: f64,
    pub dimension: usize,
    pub holder_exponent: f64,
}
#[allow(dead_code)]
impl DeGiorgiNashMoser {
    pub fn new(lambda_lo: f64, lambda_hi: f64, dim: usize) -> Self {
        let ratio = lambda_hi / lambda_lo;
        let alpha = 1.0 / (1.0 + ratio.ln().max(0.1));
        DeGiorgiNashMoser {
            ellipticity_lambda: lambda_lo,
            ellipticity_lambda_upper: lambda_hi,
            dimension: dim,
            holder_exponent: alpha,
        }
    }
    pub fn harnack_inequality_constant(&self) -> f64 {
        let ratio = self.ellipticity_lambda_upper / self.ellipticity_lambda;
        ratio.powf(self.dimension as f64 / 2.0)
    }
    pub fn holder_regularity_statement(&self) -> String {
        format!(
            "De Giorgi-Nash: weak solutions of div(A∇u)=0 are C^{{{:.3}}} (α={:.3})",
            self.holder_exponent, self.holder_exponent
        )
    }
    pub fn ellipticity_ratio(&self) -> f64 {
        self.ellipticity_lambda_upper / self.ellipticity_lambda
    }
}
/// Finite difference solver for the 1D heat equation u_t = α u_xx
/// with homogeneous Dirichlet boundary conditions.
#[derive(Debug, Clone)]
pub struct HeatSolver1D {
    /// Thermal diffusivity α.
    pub alpha: f64,
    /// Spatial mesh.
    pub mesh: Mesh1D,
    /// Current solution at interior nodes.
    pub u: Vec<f64>,
    /// Current time.
    pub t: f64,
}
impl HeatSolver1D {
    /// Initialize with initial condition evaluated at interior mesh nodes.
    pub fn new(alpha: f64, mesh: Mesh1D, initial: impl Fn(f64) -> f64) -> Self {
        let u = mesh.nodes[1..mesh.n].iter().map(|&x| initial(x)).collect();
        HeatSolver1D {
            alpha,
            mesh,
            u,
            t: 0.0,
        }
    }
    /// Advance one explicit Euler step with time-step dt.
    ///
    /// Stability requires dt ≤ h²/(2α) (CFL condition).
    pub fn step(&mut self, dt: f64) {
        let h = self.mesh.mesh_size();
        let r = self.alpha * dt / (h * h);
        let n = self.u.len();
        let mut u_new = vec![0.0; n];
        for i in 0..n {
            let u_left = if i == 0 { 0.0 } else { self.u[i - 1] };
            let u_right = if i == n - 1 { 0.0 } else { self.u[i + 1] };
            u_new[i] = self.u[i] + r * (u_left - 2.0 * self.u[i] + u_right);
        }
        self.u = u_new;
        self.t += dt;
    }
    /// Advance until time T using time-step dt.
    pub fn advance_to(&mut self, t_end: f64, dt: f64) {
        while self.t < t_end - 1e-14 {
            let actual_dt = dt.min(t_end - self.t);
            self.step(actual_dt);
        }
    }
    /// The L2 norm of the current solution (trapezoidal rule).
    pub fn l2_norm(&self) -> f64 {
        let h = self.mesh.mesh_size();
        (self.u.iter().map(|u| u * u).sum::<f64>() * h).sqrt()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KPZEquation {
    pub nonlinearity_strength: f64,
    pub noise_strength: f64,
    pub renormalization_constant: f64,
}
#[allow(dead_code)]
impl KPZEquation {
    pub fn new(lambda: f64, sigma: f64) -> Self {
        KPZEquation {
            nonlinearity_strength: lambda,
            noise_strength: sigma,
            renormalization_constant: 0.0,
        }
    }
    pub fn hopf_cole_transform(&self) -> String {
        format!(
            "Hopf-Cole: Z = exp({:.3}h) satisfies SHE: dZ = ΔZ dt + {:.3}Z dW",
            self.nonlinearity_strength,
            self.nonlinearity_strength * self.noise_strength
        )
    }
    pub fn kpz_exponents(&self) -> (f64, f64, f64) {
        (0.5, 1.0 / 3.0, 1.5)
    }
    pub fn is_kpz_universality_class(&self) -> bool {
        self.nonlinearity_strength.abs() > 0.0
    }
}
/// A uniform 1D mesh on [a, b] with N subintervals.
#[derive(Debug, Clone)]
pub struct Mesh1D {
    /// Left endpoint of the interval.
    pub a: f64,
    /// Right endpoint of the interval.
    pub b: f64,
    /// Number of subintervals.
    pub n: usize,
    /// Mesh nodes (n+1 points including endpoints).
    pub nodes: Vec<f64>,
}
impl Mesh1D {
    /// Create a uniform mesh on [a, b] with n subintervals.
    pub fn uniform(a: f64, b: f64, n: usize) -> Self {
        let h = (b - a) / n as f64;
        let nodes = (0..=n).map(|i| a + i as f64 * h).collect();
        Mesh1D { a, b, n, nodes }
    }
    /// Mesh size h = (b - a) / N.
    pub fn mesh_size(&self) -> f64 {
        (self.b - self.a) / self.n as f64
    }
    /// Number of interior nodes (excluding boundary endpoints).
    pub fn num_interior(&self) -> usize {
        self.n.saturating_sub(1)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StrichartzData {
    pub equation_type: DispersiveEquationType,
    pub spatial_dimension: usize,
    pub admissible_pairs: Vec<(f64, f64)>,
}
#[allow(dead_code)]
impl StrichartzData {
    pub fn schrodinger(dim: usize) -> Self {
        let pairs = vec![(2.0, 2.0 * dim as f64 / (dim as f64 - 2.0).max(1.0))];
        StrichartzData {
            equation_type: DispersiveEquationType::SchrodingerEquation,
            spatial_dimension: dim,
            admissible_pairs: pairs,
        }
    }
    pub fn wave(dim: usize) -> Self {
        StrichartzData {
            equation_type: DispersiveEquationType::WaveEquation,
            spatial_dimension: dim,
            admissible_pairs: vec![(2.0, 2.0)],
        }
    }
    pub fn strichartz_estimate_description(&self) -> String {
        match &self.equation_type {
            DispersiveEquationType::SchrodingerEquation => {
                format!(
                    "Schrödinger Strichartz (d={}): ||e^{{it∆}}u_0||_{{L^q_t L^r_x}} ≤ C||u_0||_{{L^2}}",
                    self.spatial_dimension
                )
            }
            DispersiveEquationType::WaveEquation => {
                format!(
                    "Wave Strichartz (d={}): ||u||_{{L^q_t L^r_x}} ≤ C(||u_0||_{{H^1}} + ||u_1||_{{L^2}})",
                    self.spatial_dimension
                )
            }
            DispersiveEquationType::KleinGordon(m) => {
                format!("Klein-Gordon (m={:.2}) Strichartz estimates", m)
            }
        }
    }
    pub fn is_energy_critical(&self) -> bool {
        match &self.equation_type {
            DispersiveEquationType::SchrodingerEquation => self.spatial_dimension >= 3,
            DispersiveEquationType::WaveEquation => self.spatial_dimension >= 4,
            _ => false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SchauderEstimate {
    pub operator_order: usize,
    pub holder_source: f64,
    pub holder_solution: f64,
    pub domain_diameter: f64,
}
#[allow(dead_code)]
impl SchauderEstimate {
    pub fn second_order(alpha: f64) -> Self {
        SchauderEstimate {
            operator_order: 2,
            holder_source: alpha,
            holder_solution: 2.0 + alpha,
            domain_diameter: 1.0,
        }
    }
    pub fn estimate_constant(&self) -> f64 {
        (1.0 / (1.0 - self.holder_source)).powi(2)
    }
    pub fn interior_estimate(&self) -> String {
        format!(
            "Interior Schauder: ||u||_{{C^{{2,{:.2}}}}} ≤ C ||Lu||_{{C^{{0,{:.2}}}}}",
            self.holder_source, self.holder_source
        )
    }
}
/// Discrete curve evolving under mean curvature flow on the plane.
/// The curve is represented as a sequence of (x, y) points (closed polygon).
#[derive(Debug, Clone)]
pub struct CurvatureFlowSim {
    /// x-coordinates of curve vertices.
    pub x: Vec<f64>,
    /// y-coordinates of curve vertices.
    pub y: Vec<f64>,
    /// Current time.
    pub t: f64,
}
impl CurvatureFlowSim {
    /// Initialize with a given set of (x, y) vertices forming a closed curve.
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        CurvatureFlowSim { x, y, t: 0.0 }
    }
    /// Create an initial circle with given center and radius, discretized into n points.
    pub fn circle(cx: f64, cy: f64, r: f64, n: usize) -> Self {
        let pi2 = 2.0 * std::f64::consts::PI;
        let x = (0..n)
            .map(|i| cx + r * (pi2 * i as f64 / n as f64).cos())
            .collect();
        let y = (0..n)
            .map(|i| cy + r * (pi2 * i as f64 / n as f64).sin())
            .collect();
        CurvatureFlowSim { x, y, t: 0.0 }
    }
    /// Compute the discrete curvature vector at each vertex (second-order finite differences).
    fn curvature_vectors(&self) -> (Vec<f64>, Vec<f64>) {
        let n = self.x.len();
        let mut kx = vec![0.0; n];
        let mut ky = vec![0.0; n];
        for i in 0..n {
            let prev = (i + n - 1) % n;
            let next = (i + 1) % n;
            kx[i] = self.x[prev] - 2.0 * self.x[i] + self.x[next];
            ky[i] = self.y[prev] - 2.0 * self.y[i] + self.y[next];
        }
        (kx, ky)
    }
    /// Advance one explicit step under mean curvature flow x_t = κ n̂.
    pub fn step(&mut self, dt: f64) {
        let (kx, ky) = self.curvature_vectors();
        for i in 0..self.x.len() {
            self.x[i] += dt * kx[i];
            self.y[i] += dt * ky[i];
        }
        self.t += dt;
    }
    /// Compute the approximate enclosed area via the shoelace formula.
    pub fn area(&self) -> f64 {
        let n = self.x.len();
        let mut sum = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            sum += self.x[i] * self.y[j] - self.x[j] * self.y[i];
        }
        0.5 * sum.abs()
    }
    /// Advance to time t_end using the given dt.
    pub fn advance_to(&mut self, t_end: f64, dt: f64) {
        while self.t < t_end - 1e-14 {
            let actual_dt = dt.min(t_end - self.t);
            self.step(actual_dt);
        }
    }
}
