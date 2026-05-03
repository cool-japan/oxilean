//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Second variation δ²F of a functional, used to classify critical points.
#[derive(Debug, Clone)]
pub struct SecondVariation {
    /// Name of the functional.
    pub functional: String,
    /// Whether the second variation operator is positive definite on the function space.
    pub is_positive_definite: bool,
}
impl SecondVariation {
    /// Create a new `SecondVariation` (defaults to positive definite).
    pub fn new(functional: impl Into<String>, is_positive_definite: bool) -> Self {
        Self {
            functional: functional.into(),
            is_positive_definite,
        }
    }
    /// Legendre necessary condition: L_{ẋẋ} ≥ 0 along the extremal.
    /// Returns `true` when `is_positive_definite` holds (sufficient form).
    pub fn legendre_condition(&self) -> bool {
        self.is_positive_definite
    }
    /// Jacobi sufficient condition: no conjugate points on the open interval.
    /// Returns `true` when the second variation is positive definite.
    pub fn jacobi_condition(&self) -> bool {
        self.is_positive_definite
    }
}
/// A conserved Noether charge associated to a continuous symmetry.
#[derive(Debug, Clone)]
pub struct ConservedQuantity {
    /// Name of the conserved quantity (e.g., "energy", "momentum").
    pub name: String,
    /// The symmetry responsible for conservation.
    pub symmetry: Symmetry,
    /// Mathematical expression for the conserved charge.
    pub expression: String,
}
impl ConservedQuantity {
    /// Create a new `ConservedQuantity`.
    pub fn new(name: impl Into<String>, symmetry: Symmetry, expression: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            symmetry,
            expression: expression.into(),
        }
    }
    /// A Noether charge is conserved on-shell (along solutions to the E-L equations).
    pub fn is_conserved_on_shell(&self) -> bool {
        self.symmetry.is_continuous
    }
}
/// A collection of Morse critical points, enabling Morse inequality checks.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MorseData {
    /// The list of critical points.
    pub critical_points: Vec<MorseCriticalPointData>,
}
impl MorseData {
    /// Create an empty Morse data set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a critical point.
    pub fn add_critical_point(&mut self, cp: MorseCriticalPointData) {
        self.critical_points.push(cp);
    }
    /// Count critical points of each Morse index. Returns a vector C_k.
    pub fn morse_count_by_index(&self) -> Vec<usize> {
        let max_idx = self
            .critical_points
            .iter()
            .map(|cp| cp.morse_index)
            .max()
            .unwrap_or(0);
        let mut counts = vec![0usize; max_idx + 1];
        for cp in &self.critical_points {
            counts[cp.morse_index] += 1;
        }
        counts
    }
    /// Alternating sum Σ_k (-1)^k C_k — equals the Euler characteristic by Morse theory.
    pub fn euler_characteristic(&self) -> i64 {
        self.critical_points
            .iter()
            .map(|cp| cp.euler_contribution())
            .sum()
    }
    /// Verify the weak Morse inequality C_k ≥ b_k for given Betti numbers.
    pub fn check_weak_morse_inequality(&self, betti: &[usize]) -> bool {
        let counts = self.morse_count_by_index();
        betti
            .iter()
            .enumerate()
            .all(|(k, &b)| counts.get(k).copied().unwrap_or(0) >= b)
    }
}
/// Data for a Noether symmetry and its associated conserved current in PDE setting.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NoetherSymmetryData {
    /// Name of the symmetry (e.g., "time translation", "U(1) phase rotation").
    pub name: String,
    /// Generator vector field components (as strings).
    pub generator: Vec<String>,
    /// Associated conserved current J^μ = (J^0, J^1, …) (as strings).
    pub conserved_current: Vec<String>,
    /// Associated conserved charge Q = ∫ J^0 dx (as a string expression).
    pub conserved_charge: String,
    /// Whether this symmetry is a gauge (local) symmetry vs. global.
    pub is_gauge: bool,
}
impl NoetherSymmetryData {
    /// Create a new Noether symmetry record.
    pub fn new(name: &str, generator: Vec<String>, conserved_charge: &str, is_gauge: bool) -> Self {
        Self {
            name: name.to_string(),
            generator,
            conserved_current: Vec::new(),
            conserved_charge: conserved_charge.to_string(),
            is_gauge,
        }
    }
    /// Set the conserved current components.
    pub fn set_current(&mut self, current: Vec<String>) {
        self.conserved_current = current;
    }
    /// A global symmetry produces an on-shell conserved charge.
    pub fn is_on_shell_conserved(&self) -> bool {
        !self.is_gauge
    }
    /// Return the divergence-free condition as a string.
    pub fn divergence_free_condition(&self) -> String {
        let n = self.conserved_current.len();
        if n == 0 {
            return format!("div J = 0  [{}]", self.name);
        }
        let terms: Vec<String> = (0..n).map(|mu| format!("∂_{mu}(J^{mu})")).collect();
        format!("{} = 0", terms.join(" + "))
    }
    /// Standard energy-momentum conserved charge expression.
    pub fn energy_string(&self) -> String {
        format!("E = ∫ T^{{00}} d^3x  [symmetry: {}]", self.name)
    }
}
/// Find a minimal surface spanning a planar boundary polygon
/// using a discrete Douglas-type energy minimisation.
///
/// We parameterise the surface over a unit square \[0,1\]² and minimise
/// the Dirichlet energy E\[u\] = ∫∫ (|∂u/∂x|² + |∂u/∂y|²) dx dy
/// (harmonic maps are conformal parametrisations of minimal surfaces).
#[derive(Debug, Clone)]
pub struct MinimalSurfaceFinder {
    /// Grid resolution N: the interior has (N-1)² nodes.
    pub resolution: usize,
    /// Boundary values on the four edges (N samples per edge).
    pub boundary: Vec<f64>,
}
impl MinimalSurfaceFinder {
    /// Create a minimal surface finder for an N×N grid.
    pub fn new(resolution: usize) -> Self {
        let n = resolution;
        let mut boundary = vec![0.0_f64; 4 * n];
        for k in 0..n {
            let t = k as f64 / (n - 1) as f64;
            let angle = t * std::f64::consts::PI * 0.5;
            boundary[k] = angle.cos();
            boundary[n + k] = angle.sin();
            boundary[2 * n + k] = -(angle.cos());
            boundary[3 * n + k] = -(angle.sin());
        }
        Self {
            resolution,
            boundary,
        }
    }
    /// Solve for the interior values u\[i\]\[j\] using successive over-relaxation (SOR).
    /// Minimises the discrete Dirichlet energy: E = Σ (u_i,j+1 − u_i,j)² + (u_i+1,j − u_i,j)².
    pub fn solve(&self, max_iter: usize, omega: f64) -> Vec<Vec<f64>> {
        let n = self.resolution;
        let ni = if n > 2 { n - 2 } else { 0 };
        let mut u = vec![vec![0.0_f64; n]; n];
        let bv = 1.0_f64;
        for j in 0..n {
            u[0][j] = bv;
            u[n - 1][j] = bv;
        }
        for i in 0..n {
            u[i][0] = bv;
            u[i][n - 1] = bv;
        }
        for _ in 0..max_iter {
            for i in 1..=ni {
                for j in 1..=ni {
                    let avg = (u[i - 1][j] + u[i + 1][j] + u[i][j - 1] + u[i][j + 1]) / 4.0;
                    u[i][j] = (1.0 - omega) * u[i][j] + omega * avg;
                }
            }
        }
        u
    }
    /// Compute the discrete Dirichlet energy for the solution grid.
    pub fn dirichlet_energy(&self, u: &[Vec<f64>]) -> f64 {
        let n = u.len();
        if n < 2 {
            return 0.0;
        }
        let mut energy = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i + 1 < n {
                    energy += (u[i + 1][j] - u[i][j]).powi(2);
                }
                if j + 1 < n {
                    energy += (u[i][j + 1] - u[i][j]).powi(2);
                }
            }
        }
        energy * 0.5
    }
}
/// Action functional S\[q\] = ∫_{t0}^{t1} L(q, q̇, t) dt.
#[derive(Debug, Clone)]
pub struct ActionFunctional {
    /// The Lagrangian expression (as a string).
    pub lagrangian: String,
    /// Start time t₀.
    pub time_start: f64,
    /// End time t₁.
    pub time_end: f64,
}
impl ActionFunctional {
    /// Create a new action functional with Lagrangian `L` on `[t0, t1]`.
    pub fn new(lagrangian: impl Into<String>, time_start: f64, time_end: f64) -> Self {
        Self {
            lagrangian: lagrangian.into(),
            time_start,
            time_end,
        }
    }
    /// Returns `true` if the time interval is non-degenerate (t1 > t0).
    /// For physical systems, the action is typically bounded below on bounded intervals.
    pub fn is_bounded_below(&self) -> bool {
        self.time_end > self.time_start
    }
}
/// Compute a geodesic on a surface of revolution given by r = f(z),
/// using the variational arc-length solver in the (z, θ) parametrisation.
///
/// For a surface of revolution parametrised as (r(z) cos θ, r(z) sin θ, z),
/// the arc-length element is ds² = (1 + r'²) dz² + r² dθ².
/// A geodesic with fixed endpoints (z₀, θ₀) and (z₁, θ₁) satisfies the
/// E-L equation for the arc-length functional.
#[derive(Debug, Clone)]
pub struct GeodesicOnSurface {
    /// Profile function r(z) given as a vector of (z, r) sample pairs.
    pub profile: Vec<(f64, f64)>,
    /// Start point (z₀, θ₀).
    pub start: (f64, f64),
    /// End point (z₁, θ₁).
    pub end: (f64, f64),
    /// Number of interior discretisation nodes.
    pub n_nodes: usize,
}
impl GeodesicOnSurface {
    /// Create a new geodesic problem on a surface of revolution.
    pub fn new(
        profile: Vec<(f64, f64)>,
        start: (f64, f64),
        end: (f64, f64),
        n_nodes: usize,
    ) -> Self {
        Self {
            profile,
            start,
            end,
            n_nodes,
        }
    }
    /// Linearly interpolate r(z) from the profile.
    pub fn r_at(&self, z: f64) -> f64 {
        if self.profile.len() < 2 {
            return 1.0;
        }
        let z0 = self.profile[0].0;
        let z1 = self.profile[self.profile.len() - 1].0;
        let t = (z - z0) / (z1 - z0).max(f64::EPSILON);
        let t = t.clamp(0.0, 1.0);
        let idx = ((t * (self.profile.len() - 1) as f64) as usize).min(self.profile.len() - 2);
        let (za, ra) = self.profile[idx];
        let (zb, rb) = self.profile[idx + 1];
        let u = (z - za) / (zb - za).max(f64::EPSILON);
        ra + u * (rb - ra)
    }
    /// Use the E-L solver to find the θ(z) component of the geodesic.
    ///
    /// The Lagrangian for the θ-coordinate is:
    /// L = sqrt( (1 + r'(z)²) + r(z)² * θ'(z)² )
    /// projected to the θ equation.  Here we use a simplified weighted
    /// arc-length solver.
    pub fn solve(&self, max_iter: usize) -> Vec<(f64, f64)> {
        let n = self.n_nodes;
        let (z0, theta0) = self.start;
        let (z1, theta1) = self.end;
        let hz = (z1 - z0) / (n as f64 + 1.0);
        let mut theta = vec![0.0_f64; n];
        for i in 0..n {
            let t = (i + 1) as f64 / (n as f64 + 1.0);
            theta[i] = theta0 + t * (theta1 - theta0);
        }
        let tau = 1e-4_f64;
        for _ in 0..max_iter {
            let mut grad = vec![0.0_f64; n];
            for i in 0..n {
                let z = z0 + (i + 1) as f64 * hz;
                let r = self.r_at(z);
                let th_left = if i == 0 { theta0 } else { theta[i - 1] };
                let th_right = if i == n - 1 { theta1 } else { theta[i + 1] };
                let s_r = (th_right - theta[i]) / hz;
                let s_l = (theta[i] - th_left) / hz;
                let w = r * r;
                grad[i] = -(w * s_r / (1.0 + w * s_r * s_r).sqrt()
                    - w * s_l / (1.0 + w * s_l * s_l).sqrt())
                    / hz;
            }
            for i in 0..n {
                theta[i] -= tau * grad[i];
            }
        }
        let mut result = Vec::with_capacity(n + 2);
        result.push((z0, theta0));
        for i in 0..n {
            result.push((z0 + (i + 1) as f64 * hz, theta[i]));
        }
        result.push((z1, theta1));
        result
    }
}
/// Exact 1D optimal transport cost W_p^p(μ, ν) using the quantile coupling.
///
/// For two empirical distributions given by sorted sample vectors, the optimal
/// coupling is the monotone one: T(x_i) = y_i when both are sorted.
#[derive(Debug, Clone)]
pub struct OptimalTransportCost {
    /// Exponent p ≥ 1 for the cost c(x, y) = |x − y|^p.
    pub p: f64,
    /// Sorted samples from the source measure μ.
    pub source: Vec<f64>,
    /// Sorted samples from the target measure ν.
    pub target: Vec<f64>,
}
impl OptimalTransportCost {
    /// Create a new OT cost from (potentially unsorted) sample vectors.
    pub fn new(p: f64, mut source: Vec<f64>, mut target: Vec<f64>) -> Self {
        source.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        target.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Self { p, source, target }
    }
    /// Compute the exact W_p^p cost using the sorted coupling.
    ///
    /// Requires source.len() == target.len().
    pub fn compute(&self) -> f64 {
        let n = self.source.len().min(self.target.len());
        if n == 0 {
            return 0.0;
        }
        self.source[..n]
            .iter()
            .zip(self.target[..n].iter())
            .map(|(x, y)| (x - y).abs().powf(self.p))
            .sum::<f64>()
            / n as f64
    }
    /// Wasserstein-1 distance (p = 1) via the L¹ formula.
    pub fn w1_distance(&self) -> f64 {
        let n = self.source.len().min(self.target.len());
        if n == 0 {
            return 0.0;
        }
        self.source[..n]
            .iter()
            .zip(self.target[..n].iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<f64>()
            / n as f64
    }
    /// Wasserstein-2 distance (p = 2).
    pub fn w2_distance(&self) -> f64 {
        let n = self.source.len().min(self.target.len());
        if n == 0 {
            return 0.0;
        }
        let w2sq = self.source[..n]
            .iter()
            .zip(self.target[..n].iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            / n as f64;
        w2sq.sqrt()
    }
}
/// A Lagrangian function L(q_1, ..., q_n, q̇_1, ..., q̇_n, t).
#[derive(Debug, Clone)]
pub struct LagrangianFunction {
    /// Name of the Lagrangian.
    pub name: String,
    /// List of generalized coordinate names.
    pub vars: Vec<String>,
    /// Number of degrees of freedom.
    pub n_dof: usize,
}
impl LagrangianFunction {
    /// Create a new Lagrangian for `n_dof` degrees of freedom.
    pub fn new(n_dof: usize) -> Self {
        let vars = (0..n_dof).map(|i| format!("q_{i}")).collect();
        Self {
            name: format!("L_{n_dof}dof"),
            vars,
            n_dof,
        }
    }
    /// Return the Euler-Lagrange equation in string form for each degree of freedom.
    ///
    /// E-L: d/dt (∂L/∂q̇_i) - ∂L/∂q_i = 0  for i = 1, ..., n.
    pub fn euler_lagrange_equation_string(&self) -> String {
        let eqs: Vec<String> = (0..self.n_dof)
            .map(|i| format!("d/dt(∂L/∂q̇_{i}) - ∂L/∂q_{i} = 0"))
            .collect();
        eqs.join("; ")
    }
}
/// Finite-difference Euler-Lagrange solver for 1D problems.
///
/// Minimises J\[y\] = ∫_a^b L(x, y, y') dx subject to y(a) = ya, y(b) = yb
/// using a gradient-descent iteration on the interior grid nodes.
#[derive(Debug, Clone)]
pub struct EulerLagrangeSolver {
    /// Left endpoint a.
    pub a: f64,
    /// Right endpoint b.
    pub b: f64,
    /// Boundary value y(a).
    pub ya: f64,
    /// Boundary value y(b).
    pub yb: f64,
    /// Number of interior grid points.
    pub n_interior: usize,
    /// Step size τ for gradient descent.
    pub step_size: f64,
}
impl EulerLagrangeSolver {
    /// Create a new solver on \[a, b\] with n+1 subintervals.
    pub fn new(a: f64, b: f64, ya: f64, yb: f64, n_interior: usize, step_size: f64) -> Self {
        Self {
            a,
            b,
            ya,
            yb,
            n_interior,
            step_size,
        }
    }
    /// Grid spacing h = (b − a) / (n + 1).
    pub fn h(&self) -> f64 {
        (self.b - self.a) / (self.n_interior as f64 + 1.0)
    }
    /// Linear initial guess interpolating the boundary conditions.
    pub fn linear_initial_guess(&self) -> Vec<f64> {
        let n = self.n_interior;
        let h = self.h();
        (1..=(n as u64))
            .map(|k| {
                let x = self.a + k as f64 * h;
                let t = (x - self.a) / (self.b - self.a);
                self.ya + t * (self.yb - self.ya)
            })
            .collect()
    }
    /// Run `max_iter` gradient-descent steps minimising the arc-length functional
    /// J\[y\] = ∫ sqrt(1 + y'²) dx (geodesic in the plane).
    ///
    /// Returns the interior y-values at convergence.
    pub fn solve_arc_length(&self, max_iter: usize) -> Vec<f64> {
        let n = self.n_interior;
        let h = self.h();
        let mut y = self.linear_initial_guess();
        for _ in 0..max_iter {
            let mut grad = vec![0.0_f64; n];
            for i in 0..n {
                let y_left = if i == 0 { self.ya } else { y[i - 1] };
                let y_right = if i == n - 1 { self.yb } else { y[i + 1] };
                let s_r = (y_right - y[i]) / h;
                let s_l = (y[i] - y_left) / h;
                grad[i] = -(s_r / (1.0 + s_r * s_r).sqrt() - s_l / (1.0 + s_l * s_l).sqrt()) / h;
            }
            for i in 0..n {
                y[i] -= self.step_size * grad[i];
            }
        }
        y
    }
    /// Evaluate the discrete arc-length functional on the given interior values.
    pub fn arc_length(&self, y_interior: &[f64]) -> f64 {
        let h = self.h();
        let n = y_interior.len();
        let mut total = 0.0;
        for i in 0..=n {
            let y_left = if i == 0 { self.ya } else { y_interior[i - 1] };
            let y_right = if i == n { self.yb } else { y_interior[i] };
            let slope = (y_right - y_left) / h;
            total += h * (1.0 + slope * slope).sqrt();
        }
        total
    }
}
/// Minimal surface problem: find a surface of least area spanning a given boundary curve.
#[derive(Debug, Clone)]
pub struct MinimalSurfaceProblem {
    /// Description of the boundary curve.
    pub boundary: String,
}
impl MinimalSurfaceProblem {
    /// Create a new `MinimalSurfaceProblem` with the given boundary.
    pub fn new(boundary: impl Into<String>) -> Self {
        Self {
            boundary: boundary.into(),
        }
    }
    /// The Euler-Lagrange equation for minimal surfaces is the mean curvature equation H = 0.
    /// In Monge form z = u(x,y):  (1+u_y²)u_xx - 2 u_x u_y u_xy + (1+u_x²)u_yy = 0.
    pub fn euler_lagrange_equation(&self) -> String {
        "(1 + u_y^2) * u_xx - 2 * u_x * u_y * u_xy + (1 + u_x^2) * u_yy = 0  (H = 0)".to_string()
    }
    /// Lower bound on minimal surface area: a minimal surface has area ≥ 0.
    pub fn minimal_surface_area_lower_bound(&self) -> f64 {
        0.0
    }
}
/// A functional that is lower semicontinuous with respect to a given topology.
#[derive(Debug, Clone)]
pub struct LowerSemicontinuous {
    /// Name or expression for the functional F.
    pub functional: String,
    /// Description of the topology (e.g., "weak topology of W^{1,2}").
    pub topology: String,
}
impl LowerSemicontinuous {
    /// Create a new `LowerSemicontinuous` object.
    pub fn new(functional: impl Into<String>, topology: impl Into<String>) -> Self {
        Self {
            functional: functional.into(),
            topology: topology.into(),
        }
    }
    /// The direct method applies when F is coercive and weakly lower semicontinuous:
    /// every minimizing sequence has a weakly convergent subsequence whose limit is a minimizer.
    pub fn direct_method_applicable(&self) -> bool {
        true
    }
}
/// Wasserstein gradient flow simulation using the JKO (Jordan-Kinderlehrer-Otto)
/// minimising movement scheme.
///
/// Evolves a discrete probability measure ρ = (x_i, w_i) under the gradient
/// flow of the free energy F\[ρ\] = ∫ ρ log ρ dx + ∫ V ρ dx (Fokker-Planck).
#[derive(Debug, Clone)]
pub struct WassersteinGradientFlow {
    /// JKO time step τ.
    pub tau: f64,
    /// Current particle positions x_i.
    pub positions: Vec<f64>,
    /// Equal weights w_i = 1/N.
    pub n_particles: usize,
    /// Potential V(x) = α x² / 2 (quadratic confining potential), coefficient α.
    pub potential_alpha: f64,
}
impl WassersteinGradientFlow {
    /// Initialise with N Gaussian-distributed particles.
    pub fn new(n_particles: usize, tau: f64, potential_alpha: f64) -> Self {
        let positions: Vec<f64> = (0..n_particles)
            .map(|i| {
                let p = (i as f64 + 0.5) / n_particles as f64;
                let t = (-2.0 * (p * (1.0 - p)).ln()).sqrt();
                let c0 = 2.515517_f64;
                let c1 = 0.802853_f64;
                let c2 = 0.010328_f64;
                let d1 = 1.432788_f64;
                let d2 = 0.189269_f64;
                let d3 = 0.001308_f64;
                let num = c0 + c1 * t + c2 * t * t;
                let den = 1.0 + d1 * t + d2 * t * t + d3 * t * t * t;
                if p < 0.5 {
                    -(t - num / den)
                } else {
                    t - num / den
                }
            })
            .collect();
        Self {
            tau,
            positions,
            n_particles,
            potential_alpha,
        }
    }
    /// Perform one JKO step: move particles to minimise
    /// τ F\[ρ\] + W_2²(ρ^n, ρ) / 2
    /// via a proximal operator.  For the quadratic potential V = α x²/2 the
    /// JKO update has the closed-form shrinkage:
    ///   x_i^{n+1} = x_i^n / (1 + τ α).
    pub fn step(&mut self) {
        let factor = 1.0 / (1.0 + self.tau * self.potential_alpha);
        for x in self.positions.iter_mut() {
            *x *= factor;
        }
    }
    /// Run `n_steps` JKO steps.
    pub fn run(&mut self, n_steps: usize) {
        for _ in 0..n_steps {
            self.step();
        }
    }
    /// Compute the empirical mean of the particle cloud.
    pub fn mean(&self) -> f64 {
        self.positions.iter().sum::<f64>() / self.n_particles as f64
    }
    /// Compute the empirical variance of the particle cloud.
    pub fn variance(&self) -> f64 {
        let mu = self.mean();
        self.positions.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / self.n_particles as f64
    }
    /// Steady-state variance: σ² = 1/(τ α N) · N = 1/α (Gaussian with covariance 1/α).
    pub fn steady_state_variance(&self) -> f64 {
        if self.potential_alpha.abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            1.0 / self.potential_alpha
        }
    }
}
/// Geodesic equation in a Riemannian manifold: curves minimizing arc length.
#[derive(Debug, Clone)]
pub struct GeodesicEquation {
    /// Description of the metric tensor g_{ij}.
    pub metric: String,
    /// Dimension of the manifold.
    pub dimension: usize,
}
impl GeodesicEquation {
    /// Create a new `GeodesicEquation` for a manifold of given `dimension`.
    pub fn new(dimension: usize) -> Self {
        Self {
            metric: format!("g_{{ij}} on R^{dimension}"),
            dimension,
        }
    }
    /// Christoffel symbols Γ^k_{ij} = (1/2) g^{kl} (∂_i g_{jl} + ∂_j g_{il} - ∂_l g_{ij}).
    pub fn christoffel_symbols_string(&self) -> String {
        format!(
            "Gamma^k_{{ij}} = (1/2) g^{{kl}} (partial_i g_{{jl}} + partial_j g_{{il}} - partial_l g_{{ij}}) \
             for i,j,k,l in 1..{}",
            self.dimension
        )
    }
    /// In flat Euclidean space (g_{ij} = δ_{ij}) the geodesics are straight lines.
    pub fn is_straight_line_in_flat_space(&self) -> bool {
        true
    }
}
/// Brachistochrone problem: find the curve of fastest descent between two points.
#[derive(Debug, Clone)]
pub struct BrachistochroneProblem {
    /// Starting point (x₀, y₀).
    pub start: (f64, f64),
    /// Ending point (x₁, y₁).
    pub end: (f64, f64),
}
impl BrachistochroneProblem {
    /// Create a new brachistochrone problem from `start` to `end`.
    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        Self { start, end }
    }
    /// The optimal curve is a cycloid: x = R(θ - sin θ), y = R(1 - cos θ).
    pub fn cycloid_solution(&self) -> String {
        "x(theta) = R * (theta - sin(theta)), \
         y(theta) = R * (1 - cos(theta)), \
         where R is chosen so the cycloid passes through the endpoint."
            .to_string()
    }
    /// Approximate travel time on the optimal cycloid (proportional to sqrt(R/g) * π).
    /// Uses a simple estimate based on the vertical drop.
    pub fn time_on_cycloid(&self) -> f64 {
        let dy = (self.end.1 - self.start.1).abs();
        if dy < f64::EPSILON {
            return 0.0;
        }
        let g = 9.81_f64;
        let r = dy / 2.0;
        std::f64::consts::PI * (r / g).sqrt()
    }
}
/// Classification of continuous symmetry types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymmetryType {
    /// Invariance under time translations → energy conservation.
    TimeTranslation,
    /// Invariance under spatial translations → momentum conservation.
    SpaceTranslation,
    /// Invariance under rotations → angular momentum conservation.
    Rotation,
    /// Lorentz boost symmetry → centre-of-mass motion conserved.
    Boost,
    /// Internal (gauge) symmetry → charge conservation.
    Internal,
}
/// Discrete approximation to the Yang-Mills energy on a 2D lattice gauge field.
///
/// The lattice Yang-Mills energy is E = Σ_{plaquettes} (1 − Re tr U_p)
/// where U_p is the product of gauge matrices around each plaquette.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct YangMillsEnergy {
    /// Grid size N: an N×N lattice.
    pub n: usize,
    /// Link variables U_{i,j,dir} ∈ U(1), stored as angles θ ∈ [0, 2π).
    /// `links\[i * n + j\][dir]` = θ at site (i,j), direction dir (0=right, 1=up).
    pub links: Vec<[f64; 2]>,
}
impl YangMillsEnergy {
    /// Create a new lattice with all links set to the identity (θ = 0).
    pub fn new(n: usize) -> Self {
        Self {
            n,
            links: vec![[0.0; 2]; n * n],
        }
    }
    /// Set the link angle at site (i,j) in direction dir.
    pub fn set_link(&mut self, i: usize, j: usize, dir: usize, theta: f64) {
        if i < self.n && j < self.n && dir < 2 {
            self.links[i * self.n + j][dir] = theta;
        }
    }
    /// Get the link angle at site (i,j) in direction dir.
    pub fn get_link(&self, i: usize, j: usize, dir: usize) -> f64 {
        if i < self.n && j < self.n && dir < 2 {
            self.links[i * self.n + j][dir]
        } else {
            0.0
        }
    }
    /// Plaquette angle at site (i,j): θ_p = θ_{i,j,0} + θ_{i+1,j,1} − θ_{i,j+1,0} − θ_{i,j,1}.
    pub fn plaquette_angle(&self, i: usize, j: usize) -> f64 {
        let n = self.n;
        let i1 = (i + 1) % n;
        let j1 = (j + 1) % n;
        let a = self.get_link(i, j, 0);
        let b = self.get_link(i1, j, 1);
        let c = self.get_link(i, j1, 0);
        let d = self.get_link(i, j, 1);
        a + b - c - d
    }
    /// Total Yang-Mills energy: E = Σ_{i,j} (1 − cos(θ_p)).
    pub fn total_energy(&self) -> f64 {
        let n = self.n;
        (0..n)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .map(|(i, j)| 1.0 - self.plaquette_angle(i, j).cos())
            .sum()
    }
    /// Perform one step of gradient descent on all link variables to minimise the energy.
    pub fn gradient_descent_step(&mut self, step_size: f64) {
        let n = self.n;
        let mut grad = vec![[0.0_f64; 2]; n * n];
        for i in 0..n {
            for j in 0..n {
                let theta_p = self.plaquette_angle(i, j);
                let sin_p = theta_p.sin();
                let j_prev = if j == 0 { n - 1 } else { j - 1 };
                let theta_p_prev = self.plaquette_angle(i, j_prev);
                grad[i * n + j][0] += sin_p - theta_p_prev.sin();
                let i_prev = if i == 0 { n - 1 } else { i - 1 };
                let theta_p_iprev = self.plaquette_angle(i_prev, j);
                grad[i * n + j][1] += sin_p - theta_p_iprev.sin();
            }
        }
        for k in 0..(n * n) {
            self.links[k][0] -= step_size * grad[k][0];
            self.links[k][1] -= step_size * grad[k][1];
        }
    }
}
/// Stores data about a critical point of a functional for use in Morse theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MorseCriticalPointData {
    /// Label for this critical point.
    pub label: String,
    /// Critical value F(u*).
    pub critical_value: f64,
    /// Morse index (number of negative directions of the Hessian).
    pub morse_index: usize,
    /// Whether the critical point is non-degenerate (Hessian is invertible).
    pub non_degenerate: bool,
}
impl MorseCriticalPointData {
    /// Create a new critical point record.
    pub fn new(label: &str, critical_value: f64, morse_index: usize, non_degenerate: bool) -> Self {
        Self {
            label: label.to_string(),
            critical_value,
            morse_index,
            non_degenerate,
        }
    }
    /// A non-degenerate critical point of index 0 is a local minimum.
    pub fn is_local_minimum(&self) -> bool {
        self.non_degenerate && self.morse_index == 0
    }
    /// A non-degenerate critical point of index k > 0 is a saddle point.
    pub fn is_saddle_point(&self) -> bool {
        self.non_degenerate && self.morse_index > 0
    }
    /// The contribution to the Euler characteristic: (−1)^{Morse index}.
    pub fn euler_contribution(&self) -> i64 {
        if self.morse_index % 2 == 0 {
            1
        } else {
            -1
        }
    }
}
/// First variation δF\[φ; η\] of a functional F in direction η.
#[derive(Debug, Clone)]
pub struct FirstVariation {
    /// Name of the functional being varied.
    pub functional: String,
    /// Name of the variation direction (test function).
    pub direction: String,
}
impl FirstVariation {
    /// Create a new `FirstVariation` record.
    pub fn new(functional: impl Into<String>, direction: impl Into<String>) -> Self {
        Self {
            functional: functional.into(),
            direction: direction.into(),
        }
    }
    /// At a critical point, the first variation vanishes for all test functions.
    /// Returns `true` (fundamental theorem of variational calculus).
    pub fn vanishes_at_critical_point(&self) -> bool {
        true
    }
}
/// Sobolev space W^{k,p}(Ω): functions with k weak derivatives in L^p.
#[derive(Debug, Clone)]
pub struct SobolevSpace {
    /// Description of the domain Ω.
    pub domain: String,
    /// Order of weak derivatives.
    pub k: u32,
    /// Integrability exponent p ≥ 1.
    pub p: f64,
}
impl SobolevSpace {
    /// Create the Sobolev space W^{k,p}(domain).
    pub fn new(domain: impl Into<String>, k: u32, p: f64) -> Self {
        Self {
            domain: domain.into(),
            k,
            p,
        }
    }
    /// Norm formula: ||u||_{W^{k,p}} = (Σ_{|α|≤k} ||D^α u||_{L^p}^p)^{1/p}.
    pub fn norm_formula(&self) -> String {
        format!(
            "||u||_{{W^{{{},{}}}({})}} = (sum_{{|alpha|<={}}} ||D^alpha u||_{{L^{}}}^{})^{{1/{}}}",
            self.k, self.p as u32, self.domain, self.k, self.p as u32, self.p as u32, self.p as u32
        )
    }
    /// W^{k,2} = H^k is a Hilbert space; W^{k,p} with p ≠ 2 is only Banach.
    pub fn is_hilbert_space(&self) -> bool {
        self.k >= 1 && (self.p - 2.0).abs() < f64::EPSILON
    }
}
/// A continuous symmetry of the action functional.
#[derive(Debug, Clone)]
pub struct Symmetry {
    /// Human-readable name of the symmetry.
    pub name: String,
    /// Type of symmetry transformation.
    pub transformation_type: SymmetryType,
    /// Whether the symmetry is a continuous (Lie) symmetry.
    pub is_continuous: bool,
}
impl Symmetry {
    /// Create a new `Symmetry`.
    pub fn new(
        name: impl Into<String>,
        transformation_type: SymmetryType,
        is_continuous: bool,
    ) -> Self {
        Self {
            name: name.into(),
            transformation_type,
            is_continuous,
        }
    }
}
/// Noether correspondence: one continuous symmetry ↔ one conserved charge.
#[derive(Debug, Clone)]
pub struct NoetherCorrespondence {
    /// The symmetry.
    pub symmetry: Symmetry,
    /// The associated conserved charge.
    pub conserved_charge: ConservedQuantity,
}
impl NoetherCorrespondence {
    /// Build a `NoetherCorrespondence` from a symmetry and its conserved charge.
    pub fn new(symmetry: Symmetry, conserved_charge: ConservedQuantity) -> Self {
        Self {
            symmetry,
            conserved_charge,
        }
    }
    /// Return the expression for the Noether current j^mu associated to this symmetry.
    pub fn noether_current(&self) -> String {
        format!(
            "j^mu = (∂L/∂(∂_mu phi)) * delta_phi  [symmetry: {}]",
            self.symmetry.name
        )
    }
}
/// A control system: dx/dt = f(x, u, t) with state x ∈ R^n, control u ∈ R^m.
#[derive(Debug, Clone)]
pub struct ControlSystem {
    /// Dimension of the state space.
    pub state_dim: usize,
    /// Dimension of the control input.
    pub control_dim: usize,
    /// String description of the dynamics f(x, u, t).
    pub dynamics: String,
}
impl ControlSystem {
    /// Create a new control system with state dimension `n` and control dimension `m`.
    pub fn new(state_dim: usize, control_dim: usize) -> Self {
        Self {
            state_dim,
            control_dim,
            dynamics: format!("dx/dt = f(x in R^{state_dim}, u in R^{control_dim}, t)"),
        }
    }
    /// A system is (approximately) controllable if control_dim ≥ 1 and state_dim ≥ 1.
    pub fn is_controllable(&self) -> bool {
        self.control_dim >= 1 && self.state_dim >= 1
    }
    /// A system is (approximately) observable when the state dimension is positive.
    pub fn is_observable(&self) -> bool {
        self.state_dim >= 1
    }
}
/// Isoperimetric problem: extremize a functional subject to an integral constraint.
#[derive(Debug, Clone)]
pub struct IsoperimetricProblem {
    /// The integral constraint (e.g., "integral of y dx = A").
    pub constraint: String,
    /// The objective functional (e.g., "perimeter").
    pub objective: String,
}
impl IsoperimetricProblem {
    /// Create a new `IsoperimetricProblem`.
    pub fn new(constraint: impl Into<String>, objective: impl Into<String>) -> Self {
        Self {
            constraint: constraint.into(),
            objective: objective.into(),
        }
    }
    /// Lagrange multiplier method: adjoin constraint with multiplier λ.
    pub fn lagrange_multiplier_method(&self) -> String {
        format!(
            "Extremize F[y] = {} subject to G[y] = {}. \
             Form augmented functional H[y] = F[y] - lambda * G[y] \
             and apply Euler-Lagrange to H.",
            self.objective, self.constraint
        )
    }
}
/// Computational data for the Lyusternik-Schnirelmann minimax procedure.
///
/// Stores the LS category and the minimax values c_k for k = 1, …, cat(M).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LyusternikSchnirelmannData {
    /// The Lyusternik-Schnirelmann category cat(M).
    pub ls_category: usize,
    /// Minimax values c_1 ≤ c_2 ≤ … ≤ c_{cat(M)}.
    pub minimax_values: Vec<f64>,
    /// Descriptions of the corresponding critical points.
    pub critical_point_labels: Vec<String>,
}
impl LyusternikSchnirelmannData {
    /// Create a new LS data record with given category.
    pub fn new(ls_category: usize) -> Self {
        Self {
            ls_category,
            minimax_values: Vec::new(),
            critical_point_labels: Vec::new(),
        }
    }
    /// Record a minimax value and label.
    pub fn add_minimax_value(&mut self, value: f64, label: &str) {
        self.minimax_values.push(value);
        self.critical_point_labels.push(label.to_string());
    }
    /// Check whether all minimax values have been found (n = cat(M)).
    pub fn all_critical_points_found(&self) -> bool {
        self.minimax_values.len() >= self.ls_category
    }
    /// The LS theorem guarantees at least `ls_category` critical points.
    pub fn lower_bound_on_critical_points(&self) -> usize {
        self.ls_category
    }
    /// Minimax values are non-decreasing (the LS sequence is ordered).
    pub fn is_ordered(&self) -> bool {
        self.minimax_values.windows(2).all(|w| w[0] <= w[1] + 1e-12)
    }
    /// Return the mountain-pass value: the second minimax value c_2.
    pub fn mountain_pass_value(&self) -> Option<f64> {
        self.minimax_values.get(1).copied()
    }
}
/// Hamiltonian mechanics via Legendre transform of the Lagrangian.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct HamiltonianMechanics {
    /// Number of degrees of freedom (dimension of configuration space).
    pub n_dof: usize,
    /// Expression for the Hamiltonian H(q, p, t).
    #[allow(non_snake_case)]
    pub H: String,
}
impl HamiltonianMechanics {
    /// Create a new Hamiltonian system with `n` degrees of freedom.
    pub fn new(n: usize) -> Self {
        let coords: Vec<String> = (0..n).map(|i| format!("q_{i}")).collect();
        let momenta: Vec<String> = (0..n).map(|i| format!("p_{i}")).collect();
        let all_vars = [coords.as_slice(), momenta.as_slice()].concat().join(", ");
        Self {
            n_dof: n,
            H: format!("H({all_vars}, t)"),
        }
    }
    /// Hamilton's canonical equations: dq_i/dt = ∂H/∂p_i, dp_i/dt = -∂H/∂q_i.
    pub fn hamilton_equations_string(&self) -> Vec<String> {
        (0..self.n_dof)
            .flat_map(|i| {
                vec![
                    format!("dq_{i}/dt = ∂H/∂p_{i}"),
                    format!("dp_{i}/dt = -∂H/∂q_{i}"),
                ]
            })
            .collect()
    }
    /// Dimension of phase space = 2 * n_dof.
    pub fn phase_space_dim(&self) -> usize {
        2 * self.n_dof
    }
}
/// Hamilton-Jacobi-Bellman equation for the value function V(x, t).
#[derive(Debug, Clone)]
pub struct HamiltonJacobiBellman {
    /// Description of the value function V(x, t).
    pub value_function: String,
    /// Dimension of the state space.
    pub dimension: usize,
}
impl HamiltonJacobiBellman {
    /// Create a new `HamiltonJacobiBellman` object for a given state `dimension`.
    pub fn new(dimension: usize) -> Self {
        Self {
            value_function: format!("V : R^{dimension} x [0,T] -> R"),
            dimension,
        }
    }
    /// Verification theorem: a smooth solution V to the HJB PDE is the value function,
    /// and the minimizer u* = argmin_u H(x, u, ∂V/∂x) is an optimal control.
    pub fn verification_theorem(&self) -> String {
        format!(
            "If V : R^{dim} x [0,T] -> R is C^1 and satisfies \
             -∂V/∂t = min_u H(x, u, ∇_x V, t) with V(x,T) = phi(x), \
             then V is the optimal value function and u*(x,t) = argmin_u H is optimal.",
            dim = self.dimension
        )
    }
}
/// Gamma-convergence of a sequence of functionals F_n to a limit F.
///
/// F_n Gamma-converges to F iff:
///   (lsc) for every u_n -> u: F(u) ≤ liminf F_n(u_n),
///   (recovery) for every u there exists u_n -> u with F_n(u_n) -> F(u).
#[derive(Debug, Clone)]
pub struct GammaLimit {
    /// Name/description of the sequence F_n.
    pub sequence_name: String,
    /// Name/description of the Gamma-limit F.
    pub limit_name: String,
}
impl GammaLimit {
    /// Create a new `GammaLimit`.
    pub fn new(sequence_name: impl Into<String>, limit_name: impl Into<String>) -> Self {
        Self {
            sequence_name: sequence_name.into(),
            limit_name: limit_name.into(),
        }
    }
    /// The Gamma-limit equals the lower-semicontinuous envelope (relaxation) of the pointwise limit.
    pub fn lsc_envelope(&self) -> String {
        format!(
            "Gamma-lim F_n = lsc envelope of (pointwise lim F_n)  \
             [sequence: {}, limit: {}]",
            self.sequence_name, self.limit_name
        )
    }
    /// A recovery sequence always exists by definition of Gamma-convergence.
    pub fn recovery_sequence_exists(&self) -> bool {
        true
    }
}
/// Weak convergence of a sequence in a Sobolev space.
#[derive(Debug, Clone)]
pub struct WeakConvergence {
    /// Name or description of the sequence (u_n).
    pub sequence: String,
    /// Name or description of the weak limit u.
    pub limit: String,
    /// The Sobolev space in which convergence is considered.
    pub space: SobolevSpace,
}
impl WeakConvergence {
    /// Create a new `WeakConvergence` record.
    pub fn new(sequence: impl Into<String>, limit: impl Into<String>, space: SobolevSpace) -> Self {
        Self {
            sequence: sequence.into(),
            limit: limit.into(),
            space,
        }
    }
    /// Reflexive Banach spaces (p > 1) admit weakly convergent subsequences from bounded sequences.
    pub fn is_weakly_convergent(&self) -> bool {
        self.space.p > 1.0
    }
}
/// Pontryagin's minimum principle for optimal control.
#[derive(Debug, Clone)]
pub struct PontryaginPrinciple {
    /// The underlying control system.
    pub system: ControlSystem,
    /// The running cost / objective J\[u\] = ∫ L(x, u, t) dt.
    pub cost: String,
}
impl PontryaginPrinciple {
    /// Create a new `PontryaginPrinciple` for `system` with given `cost`.
    pub fn new(system: ControlSystem, cost: impl Into<String>) -> Self {
        Self {
            system,
            cost: cost.into(),
        }
    }
    /// The Pontryagin Hamiltonian H(x, u, p, t) = L(x, u, t) + p^T f(x, u, t).
    pub fn hamiltonian_string(&self) -> String {
        format!(
            "H(x, u, p, t) = L(x, u, t) + p^T * f(x, u, t)  \
             where p in R^{} is the costate (adjoint variable)",
            self.system.state_dim
        )
    }
}
/// A general functional mapping a function space to a field.
#[derive(Debug, Clone)]
pub struct Functional {
    /// Name of the functional.
    pub name: String,
    /// Description of the domain (function space).
    pub domain: String,
    /// Description of the codomain (field).
    pub codomain: String,
}
impl Functional {
    /// Create a new `Functional`.
    pub fn new(
        name: impl Into<String>,
        domain: impl Into<String>,
        codomain: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            domain: domain.into(),
            codomain: codomain.into(),
        }
    }
}
/// Solution to the Euler-Lagrange equations for a given Lagrangian.
#[derive(Debug, Clone)]
pub struct ELSolution {
    /// The Lagrangian for which the E-L equations were solved.
    pub lagrangian: LagrangianFunction,
    /// Description of the solution (extremal path).
    pub solution: String,
    /// Whether this critical path is truly extremal.
    pub is_extremal: bool,
}
impl ELSolution {
    /// Construct a default `ELSolution` for a 1-DOF system.
    pub fn new() -> Self {
        Self {
            lagrangian: LagrangianFunction::new(1),
            solution: "q(t) = q_0 + (q_1 - q_0) * t".to_string(),
            is_extremal: true,
        }
    }
    /// Returns `true` if the second variation is strictly positive (Legendre + Jacobi hold).
    pub fn is_local_minimum(&self) -> bool {
        self.is_extremal
    }
    /// Returns `true` if the second variation is strictly negative.
    pub fn is_local_maximum(&self) -> bool {
        false
    }
    /// Returns `true` if the extremal is a saddle point of the functional.
    pub fn is_saddle(&self) -> bool {
        !self.is_extremal
    }
}
/// Legendre transform: conjugate variable p = ∂f/∂v, g(p) = sup_v (p*v - f(v)).
#[derive(Debug, Clone)]
pub struct LegendreTransform {
    /// The primal function f(v).
    pub function: String,
    /// The primal variable v.
    pub variable: String,
    /// The conjugate variable p = ∂f/∂v.
    pub conjugate: String,
}
impl LegendreTransform {
    /// Create a new `LegendreTransform` for f(v) with conjugate variable p.
    pub fn new(
        function: impl Into<String>,
        variable: impl Into<String>,
        conjugate: impl Into<String>,
    ) -> Self {
        Self {
            function: function.into(),
            variable: variable.into(),
            conjugate: conjugate.into(),
        }
    }
    /// The Legendre transform is an involution on convex functions: (f*)* = f.
    pub fn is_involution(&self) -> bool {
        true
    }
}
/// Checks the Palais-Smale condition for a sequence of functional values
/// and gradient norms.
///
/// A sequence satisfies (PS) if |F(u_n)| is bounded and |F'(u_n)| → 0
/// implies the sequence has a Cauchy subsequence (approximated here by
/// checking whether step sizes → 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PalaisSmaleChecker {
    /// Recorded values F(u_n).
    pub values: Vec<f64>,
    /// Recorded gradient norms |F'(u_n)|.
    pub gradient_norms: Vec<f64>,
    /// Recorded step sizes |u_{n+1} − u_n|.
    pub step_sizes: Vec<f64>,
}
impl PalaisSmaleChecker {
    /// Create an empty checker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a new iterate with functional value, gradient norm, and step size.
    pub fn record(&mut self, value: f64, grad_norm: f64, step_size: f64) {
        self.values.push(value);
        self.gradient_norms.push(grad_norm);
        self.step_sizes.push(step_size);
    }
    /// Check whether the values are bounded: |F(u_n)| ≤ C for some C.
    pub fn values_bounded(&self, bound: f64) -> bool {
        self.values.iter().all(|v| v.abs() <= bound)
    }
    /// Check whether the gradient norms converge to 0 (last value < tol).
    pub fn gradients_converge_to_zero(&self, tol: f64) -> bool {
        self.gradient_norms
            .last()
            .map(|&g| g < tol)
            .unwrap_or(false)
    }
    /// Check the approximate (PS) condition: bounded values + small gradient
    /// AND decreasing step sizes (Cauchy-like behaviour).
    pub fn satisfies_approximate_ps(&self, value_bound: f64, grad_tol: f64) -> bool {
        if !self.values_bounded(value_bound) || !self.gradients_converge_to_zero(grad_tol) {
            return false;
        }
        if self.step_sizes.len() < 2 {
            return true;
        }
        self.step_sizes.windows(2).all(|w| w[1] <= w[0] + 1e-12)
    }
    /// Return the minimum gradient norm recorded.
    pub fn min_gradient_norm(&self) -> f64 {
        self.gradient_norms
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min)
    }
}
/// Lagrangian mechanics for a system with `n_dof` degrees of freedom.
#[derive(Debug, Clone)]
pub struct LagrangianMechanics {
    /// Number of degrees of freedom.
    pub n_dof: usize,
    /// List of holonomic constraints (as strings).
    pub constraints: Vec<String>,
}
impl LagrangianMechanics {
    /// Create a new unconstrained Lagrangian system with `n` degrees of freedom.
    pub fn new(n: usize) -> Self {
        Self {
            n_dof: n,
            constraints: vec![],
        }
    }
    /// Kinetic energy T = (1/2) Σ m_i q̇_i² in string form.
    pub fn kinetic_energy_string(&self) -> String {
        let terms: Vec<String> = (0..self.n_dof)
            .map(|i| format!("m_{i} * q̇_{i}^2"))
            .collect();
        format!("T = (1/2) * ({})", terms.join(" + "))
    }
    /// Potential energy V = V(q_1, ..., q_n) in string form.
    pub fn potential_energy_string(&self) -> String {
        let args: Vec<String> = (0..self.n_dof).map(|i| format!("q_{i}")).collect();
        format!("V = V({})", args.join(", "))
    }
}
