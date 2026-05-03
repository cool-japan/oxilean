//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::functions::{EPSILON_0, MU_0};

/// A simplified (flat) spacetime for geodesic computations.
///
/// In flat (Minkowski) spacetime all Christoffel symbols vanish, so geodesics
/// are straight lines and the curvature is identically zero.
#[derive(Debug, Clone)]
pub struct GeodesicEquation {
    /// Metric components g_{μν} (stored as a square matrix).
    pub metric_components: Vec<Vec<f64>>,
}
impl GeodesicEquation {
    /// Create a flat (Minkowski-like) metric in `dim` dimensions.
    pub fn new(dim: usize) -> Self {
        let mut g = vec![vec![0.0; dim]; dim];
        for i in 0..dim {
            g[i][i] = 1.0;
        }
        Self {
            metric_components: g,
        }
    }
    /// Return Γ^i_{jk}: zero for a flat metric (stub).
    pub fn christoffel_symbol(&self, _i: usize, _j: usize, _k: usize) -> f64 {
        0.0
    }
    /// Return a scalar curvature measure (Ricci scalar); zero for flat metric.
    pub fn geodesic_deviation(&self) -> f64 {
        0.0
    }
}
/// Numerical integration method for Hamiltonian systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegratorMethod {
    /// Forward Euler (not symplectic — for reference).
    Euler,
    /// Leapfrog / Störmer-Verlet (symplectic, second-order).
    Leapfrog,
    /// Runge-Kutta 4 (not symplectic — for reference).
    RK4,
    /// Symplectic Euler (first-order symplectic).
    SymplecticEuler,
}
/// A simple cochain complex representing a truncated BRST complex.
///
/// Stores cohomology-degree cochains as vectors of real numbers.
/// The BRST differential Q increases ghost number by 1.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BRSTComplex {
    /// Cochains at each ghost number 0, 1, 2, ...
    pub cochains: Vec<Vec<f64>>,
    /// The differential matrix Q\[k\] : C^k → C^{k+1}.
    pub differentials: Vec<Vec<Vec<f64>>>,
}
#[allow(dead_code)]
impl BRSTComplex {
    /// Create a zero BRST complex with given dimensions at each degree.
    pub fn new(dims: Vec<usize>) -> Self {
        let n = dims.len();
        let cochains = dims.iter().map(|&d| vec![0.0; d]).collect();
        let differentials = (0..n.saturating_sub(1))
            .map(|k| vec![vec![0.0; dims[k]]; dims[k + 1]])
            .collect();
        Self {
            cochains,
            differentials,
        }
    }
    /// Apply the differential Q at degree k: (Qv)\[i\] = Σ_j Q\[k\]\[i\]\[j\] * v\[j\].
    pub fn apply_differential(&self, k: usize, v: &[f64]) -> Vec<f64> {
        if k + 1 >= self.differentials.len() + 1 {
            return vec![];
        }
        let q = &self.differentials[k];
        q.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum())
            .collect()
    }
    /// Check nilpotency Q² = 0 at degree k (||Q_{k+1} Q_k||_F < ε).
    pub fn check_nilpotency(&self, k: usize, eps: f64) -> bool {
        if k + 1 >= self.differentials.len() {
            return true;
        }
        let qk = &self.differentials[k];
        let qk1 = &self.differentials[k + 1];
        let rows1 = qk1.len();
        let cols1 = if rows1 > 0 {
            qk1[0].len()
        } else {
            return true;
        };
        let cols0 = if !qk.is_empty() {
            qk[0].len()
        } else {
            return true;
        };
        for i in 0..rows1 {
            for j in 0..cols0 {
                let val: f64 = (0..cols1)
                    .map(|l| {
                        qk1[i].get(l).copied().unwrap_or(0.0)
                            * qk.get(l).and_then(|r| r.get(j)).copied().unwrap_or(0.0)
                    })
                    .sum();
                if val.abs() > eps {
                    return false;
                }
            }
        }
        true
    }
    /// Compute the Euler characteristic χ = Σ (-1)^k dim C^k.
    pub fn euler_characteristic(&self) -> i64 {
        self.cochains
            .iter()
            .enumerate()
            .map(|(k, c)| {
                if k % 2 == 0 {
                    c.len() as i64
                } else {
                    -(c.len() as i64)
                }
            })
            .sum()
    }
    /// Return the dimensions of the cochain spaces.
    pub fn dimensions(&self) -> Vec<usize> {
        self.cochains.iter().map(|c| c.len()).collect()
    }
}
/// Exact one-soliton solution of the KdV equation.
///
/// The KdV equation is: ∂u/∂t − 6u ∂u/∂x + ∂³u/∂x³ = 0.
/// (Using the convention with the −6u ux term.)
///
/// The one-soliton solution is: u(x, t) = −κ²/2 · sech²(κ/2 · (x − κ²t − x₀)).
#[derive(Debug, Clone)]
pub struct KdVSoliton {
    /// Wave-number parameter κ > 0.  Speed = κ².
    pub kappa: f64,
    /// Initial position x₀.
    pub x0: f64,
}
impl KdVSoliton {
    /// Create a soliton with wave-number `kappa` centred at `x0`.
    pub fn new(kappa: f64, x0: f64) -> Self {
        assert!(kappa > 0.0, "kappa must be positive");
        Self { kappa, x0 }
    }
    /// Evaluate u(x, t).
    pub fn eval(&self, x: f64, t: f64) -> f64 {
        let xi = 0.5 * self.kappa * (x - self.kappa * self.kappa * t - self.x0);
        let sech = 1.0 / xi.cosh();
        -0.5 * self.kappa * self.kappa * sech * sech
    }
    /// Soliton speed c = κ².
    pub fn speed(&self) -> f64 {
        self.kappa * self.kappa
    }
    /// Peak amplitude A = −κ²/2.
    pub fn amplitude(&self) -> f64 {
        -0.5 * self.kappa * self.kappa
    }
    /// Evaluate the soliton at multiple (x, t) pairs.
    pub fn eval_grid(&self, xs: &[f64], t: f64) -> Vec<f64> {
        xs.iter().map(|&x| self.eval(x, t)).collect()
    }
}
/// A complete Hamiltonian system with state (q, p) and a user-supplied
/// Hamiltonian gradient function.
///
/// Uses symplectic Euler integration by default for long-time stability.
#[derive(Debug, Clone)]
pub struct HamiltonianSystem {
    /// Generalised coordinates q.
    pub q: Vec<f64>,
    /// Conjugate momenta p.
    pub p: Vec<f64>,
    /// Time step.
    pub dt: f64,
    /// Current simulation time.
    pub time: f64,
}
impl HamiltonianSystem {
    /// Create a new system with given initial conditions and time step.
    pub fn new(q: Vec<f64>, p: Vec<f64>, dt: f64) -> Self {
        assert_eq!(q.len(), p.len(), "q and p must have the same dimension");
        Self {
            q,
            p,
            dt,
            time: 0.0,
        }
    }
    /// Advance one time step using symplectic Euler integration.
    ///
    /// `grad_h(q, p)` must return `(∂H/∂p, ∂H/∂q)`.
    pub fn step(&mut self, grad_h: &dyn Fn(&[f64], &[f64]) -> (Vec<f64>, Vec<f64>)) {
        let integrator = SymplecticIntegrator {
            dt: self.dt,
            method: IntegratorMethod::SymplecticEuler,
        };
        let (q_new, p_new) = integrator.step(&self.q, &self.p, grad_h);
        self.q = q_new;
        self.p = p_new;
        self.time += self.dt;
    }
    /// Run `n` steps and return the trajectory as `(q_history, p_history)`.
    pub fn run(
        &mut self,
        n: usize,
        grad_h: &dyn Fn(&[f64], &[f64]) -> (Vec<f64>, Vec<f64>),
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let mut qs = Vec::with_capacity(n);
        let mut ps = Vec::with_capacity(n);
        for _ in 0..n {
            self.step(grad_h);
            qs.push(self.q.clone());
            ps.push(self.p.clone());
        }
        (qs, ps)
    }
    /// Evaluate a scalar Hamiltonian H(q,p) = Σ p_i²/(2m_i) + V(q).
    /// Provided for convenience; `grad_h` must still be supplied externally.
    pub fn kinetic_energy(&self, masses: &[f64]) -> f64 {
        self.p
            .iter()
            .zip(masses.iter())
            .map(|(&pi, &mi)| pi * pi / (2.0 * mi))
            .sum()
    }
}
/// A classical point particle in n-dimensional space.
#[derive(Debug, Clone)]
pub struct ClassicalParticle {
    /// Mass of the particle (kg).
    pub mass: f64,
    /// Position vector (m).
    pub position: Vec<f64>,
    /// Velocity vector (m/s).
    pub velocity: Vec<f64>,
}
impl ClassicalParticle {
    /// Create a new particle at the origin with zero velocity.
    pub fn new(mass: f64, dim: usize) -> Self {
        Self {
            mass,
            position: vec![0.0; dim],
            velocity: vec![0.0; dim],
        }
    }
    /// Kinetic energy: K = ½ m v².
    pub fn kinetic_energy(&self) -> f64 {
        let v_sq: f64 = self.velocity.iter().map(|&vi| vi * vi).sum();
        0.5 * self.mass * v_sq
    }
    /// Set the position vector.
    pub fn set_position(&mut self, pos: Vec<f64>) {
        self.position = pos;
    }
    /// Set the velocity vector.
    pub fn set_velocity(&mut self, vel: Vec<f64>) {
        self.velocity = vel;
    }
    /// Linear momentum: p = m v.
    pub fn momentum(&self) -> Vec<f64> {
        self.velocity.iter().map(|&vi| self.mass * vi).collect()
    }
}
/// A simple Path Integral Monte Carlo estimator for 1D quantum mechanics.
///
/// Uses a discrete Euclidean path (imaginary time) to estimate the ground
/// state energy via the virial estimator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PathIntegralMonteCarlo {
    /// Number of time slices.
    pub n_slices: usize,
    /// Imaginary time step τ = β/N.
    pub tau: f64,
    /// Particle mass.
    pub mass: f64,
    /// Current path (positions at each time slice).
    pub path: Vec<f64>,
}
#[allow(dead_code)]
impl PathIntegralMonteCarlo {
    /// Create a new PIMC estimator with all positions set to zero.
    pub fn new(n_slices: usize, beta: f64, mass: f64) -> Self {
        Self {
            n_slices,
            tau: beta / (n_slices as f64),
            mass,
            path: vec![0.0; n_slices],
        }
    }
    /// Euclidean (kinetic) action for the current path.
    /// S_E = Σ_k m/(2τ) * (x_{k+1} − x_k)²
    pub fn kinetic_action(&self) -> f64 {
        let prefactor = self.mass / (2.0 * self.tau);
        let n = self.n_slices;
        (0..n)
            .map(|k| {
                let diff = self.path[(k + 1) % n] - self.path[k];
                prefactor * diff * diff
            })
            .sum()
    }
    /// Harmonic potential action contribution.
    /// S_V = τ * Σ_k ½ ω² x_k²
    pub fn potential_action_harmonic(&self, omega: f64) -> f64 {
        self.tau
            * self
                .path
                .iter()
                .map(|&x| 0.5 * omega * omega * x * x)
                .sum::<f64>()
    }
    /// Total Euclidean action (kinetic + harmonic potential).
    pub fn total_action_harmonic(&self, omega: f64) -> f64 {
        self.kinetic_action() + self.potential_action_harmonic(omega)
    }
    /// Estimate the ground state energy via the virial theorem (harmonic case).
    /// E_0 ≈ ½ω² ⟨x²⟩ + ½/(2m τ²) corrections (simplified).
    pub fn virial_energy_estimate(&self, omega: f64) -> f64 {
        let x_sq: f64 = self.path.iter().map(|&x| x * x).sum::<f64>() / (self.n_slices as f64);
        0.5 * omega * omega * x_sq
    }
    /// Set the path to a given configuration.
    pub fn set_path(&mut self, path: Vec<f64>) {
        assert_eq!(path.len(), self.n_slices);
        self.path = path;
    }
    /// Compute the winding number (only meaningful for a ring).
    pub fn winding_number(&self) -> f64 {
        let first = self.path[0];
        let last = self.path[self.n_slices - 1];
        (last - first) / (2.0 * std::f64::consts::PI)
    }
}
/// 1D Schrödinger equation propagator on a uniform spatial grid.
///
/// Evolves a complex wave function ψ(x,t) under H = −(ħ²/2m) d²/dx² + V(x)
/// using the split-operator method: `exp(−iHΔt/ħ) ≈ exp(−iVΔt/2ħ) exp(−iTΔt/ħ) exp(−iVΔt/2ħ)`.
///
/// Here we use a simplified real-valued approximation (ignoring the imaginary
/// time phase) to stay dependency-free.
#[derive(Debug, Clone)]
pub struct SchrodingerPropagator {
    /// Spatial grid size (number of points).
    pub n_grid: usize,
    /// Spatial step Δx (m).
    pub dx: f64,
    /// Particle mass (kg, in units where ħ = 1).
    pub mass: f64,
    /// Time step Δt.
    pub dt: f64,
    /// Potential energy at each grid point V(x_i).
    pub potential: Vec<f64>,
}
impl SchrodingerPropagator {
    /// Create a propagator on `n_grid` points with spacing `dx`.
    pub fn new(n_grid: usize, dx: f64, mass: f64, dt: f64, potential: Vec<f64>) -> Self {
        assert_eq!(
            potential.len(),
            n_grid,
            "potential length must equal n_grid"
        );
        Self {
            n_grid,
            dx,
            mass,
            dt,
            potential,
        }
    }
    /// Apply one kinetic-energy half-step via the finite-difference Laplacian.
    ///
    /// Returns the real part of `exp(−i T Δt) ψ` using a first-order
    /// Euler approximation: `ψ' = ψ − i(Δt/2m Δx²) Δ_disc ψ`.
    ///
    /// Stores (re, im) as two parallel `Vec<f64>`.
    pub fn kinetic_half_step(&self, psi_re: &[f64], psi_im: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let coeff = self.dt / (2.0 * self.mass * self.dx * self.dx);
        let n = self.n_grid;
        let mut re_out = vec![0.0; n];
        let mut im_out = vec![0.0; n];
        for i in 0..n {
            let left = if i == 0 { 0.0 } else { 1.0 };
            let right = if i + 1 == n { 0.0 } else { 1.0 };
            let lap_re = left * psi_re[i.saturating_sub(1)] - 2.0 * psi_re[i]
                + right * psi_re[(i + 1).min(n - 1)];
            let lap_im = left * psi_im[i.saturating_sub(1)] - 2.0 * psi_im[i]
                + right * psi_im[(i + 1).min(n - 1)];
            re_out[i] = psi_re[i] - coeff * lap_im;
            im_out[i] = psi_im[i] + coeff * lap_re;
        }
        (re_out, im_out)
    }
    /// Apply one potential-energy phase step: `exp(−iV(x)Δt/2) ψ`.
    pub fn potential_half_step(&self, psi_re: &[f64], psi_im: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut re_out = vec![0.0; self.n_grid];
        let mut im_out = vec![0.0; self.n_grid];
        for i in 0..self.n_grid {
            let phase = -self.potential[i] * self.dt / 2.0;
            let (s, c) = phase.sin_cos();
            re_out[i] = psi_re[i] * c - psi_im[i] * s;
            im_out[i] = psi_re[i] * s + psi_im[i] * c;
        }
        (re_out, im_out)
    }
    /// Full split-operator step: V/2 → T → V/2.
    pub fn step(&self, psi_re: &[f64], psi_im: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let (r1, i1) = self.potential_half_step(psi_re, psi_im);
        let (r2, i2) = self.kinetic_half_step(&r1, &i1);
        self.potential_half_step(&r2, &i2)
    }
    /// Compute the norm ‖ψ‖² = Σ (|ψ_i|²) dx.
    pub fn norm_squared(&self, psi_re: &[f64], psi_im: &[f64]) -> f64 {
        psi_re
            .iter()
            .zip(psi_im.iter())
            .map(|(&r, &i)| (r * r + i * i) * self.dx)
            .sum()
    }
}
/// A vector in a bosonic Fock space, represented by occupation numbers.
///
/// The state |n_0, n_1, ..., n_{k-1}⟩ specifies how many particles occupy
/// each single-particle mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FockSpaceVec {
    /// Occupation numbers for each mode.
    pub occupations: Vec<u64>,
}
impl FockSpaceVec {
    /// Create a vacuum state |0,0,...,0⟩ with `n_modes` modes.
    pub fn vacuum(n_modes: usize) -> Self {
        Self {
            occupations: vec![0; n_modes],
        }
    }
    /// Total particle number N = Σ n_k.
    pub fn total_number(&self) -> u64 {
        self.occupations.iter().sum()
    }
    /// Apply creation operator a†_k: increase mode k by 1.
    /// Returns `None` if `k` is out of bounds.
    /// The norm factor √(n_k + 1) is returned as the second element.
    pub fn apply_creation(&self, k: usize) -> Option<(Self, f64)> {
        if k >= self.occupations.len() {
            return None;
        }
        let mut new_occ = self.occupations.clone();
        let n_k = new_occ[k];
        new_occ[k] += 1;
        let norm = ((n_k + 1) as f64).sqrt();
        Some((
            Self {
                occupations: new_occ,
            },
            norm,
        ))
    }
    /// Apply annihilation operator a_k: decrease mode k by 1.
    /// Returns `None` if `k` is out of bounds or mode is already empty.
    /// The norm factor √(n_k) is returned as the second element.
    pub fn apply_annihilation(&self, k: usize) -> Option<(Self, f64)> {
        if k >= self.occupations.len() || self.occupations[k] == 0 {
            return None;
        }
        let mut new_occ = self.occupations.clone();
        let n_k = new_occ[k];
        new_occ[k] -= 1;
        let norm = (n_k as f64).sqrt();
        Some((
            Self {
                occupations: new_occ,
            },
            norm,
        ))
    }
    /// Number operator expectation value for mode k.
    pub fn number_op(&self, k: usize) -> u64 {
        self.occupations.get(k).copied().unwrap_or(0)
    }
    /// Inner product ⟨self | other⟩: 1 if occupation vectors are identical, else 0.
    pub fn inner_product(&self, other: &FockSpaceVec) -> f64 {
        if self.occupations == other.occupations {
            1.0
        } else {
            0.0
        }
    }
}
/// A classical electromagnetic field at a single point.
#[derive(Debug, Clone)]
pub struct ElectromagneticField {
    /// Electric field vector (V/m).
    pub e: Vec<f64>,
    /// Magnetic field vector (T).
    pub b: Vec<f64>,
}
impl ElectromagneticField {
    /// Create a zero electromagnetic field.
    pub fn new() -> Self {
        Self {
            e: vec![0.0; 3],
            b: vec![0.0; 3],
        }
    }
    /// Electromagnetic energy density u = (ε₀|E|² + |B|²/μ₀) / 2.
    pub fn energy_density(&self) -> f64 {
        let e_sq: f64 = self.e.iter().map(|&ei| ei * ei).sum();
        let b_sq: f64 = self.b.iter().map(|&bi| bi * bi).sum();
        (EPSILON_0 * e_sq + b_sq / MU_0) / 2.0
    }
    /// Poynting vector S = E × B / μ₀.
    pub fn poynting_vector(&self) -> Vec<f64> {
        let cross = cross3(&self.e, &self.b);
        cross.into_iter().map(|c| c / MU_0).collect()
    }
    /// Lorentz force on a charge q moving with velocity v: F = q(E + v × B).
    pub fn lorentz_force(&self, q: f64, v: &[f64]) -> Vec<f64> {
        let vxb = cross3(v, &self.b);
        (0..3).map(|i| q * (self.e[i] + vxb[i])).collect()
    }
}
/// Data encoding a mirror symmetry pair of Calabi-Yau manifolds.
///
/// Tracks the Hodge numbers h^{p,q} of both manifolds and verifies the
/// mirror exchange h^{p,q}(X) = h^{n-p,q}(Y).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MirrorSymmetryData {
    /// Complex dimension n.
    pub complex_dim: usize,
    /// Hodge numbers h^{p,q} of the manifold X (stored as a matrix).
    pub hodge_x: Vec<Vec<u64>>,
    /// Hodge numbers h^{p,q} of the mirror Y.
    pub hodge_y: Vec<Vec<u64>>,
}
#[allow(dead_code)]
impl MirrorSymmetryData {
    /// Create a mirror symmetry data structure with given dimension.
    pub fn new(complex_dim: usize) -> Self {
        let n = complex_dim + 1;
        Self {
            complex_dim,
            hodge_x: vec![vec![0; n]; n],
            hodge_y: vec![vec![0; n]; n],
        }
    }
    /// Set h^{p,q}(X).
    pub fn set_hodge_x(&mut self, p: usize, q: usize, val: u64) {
        self.hodge_x[p][q] = val;
    }
    /// Set h^{p,q}(Y).
    pub fn set_hodge_y(&mut self, p: usize, q: usize, val: u64) {
        self.hodge_y[p][q] = val;
    }
    /// Verify mirror symmetry: h^{p,q}(X) = h^{n-p,q}(Y) for all p, q.
    pub fn verify_mirror(&self) -> bool {
        let n = self.complex_dim;
        for p in 0..=n {
            for q in 0..=n {
                if n < p {
                    continue;
                }
                let mirror_p = n - p;
                if self.hodge_x[p][q] != self.hodge_y[mirror_p][q] {
                    return false;
                }
            }
        }
        true
    }
    /// Euler characteristic χ = Σ_{p,q} (-1)^{p+q} h^{p,q}.
    pub fn euler_characteristic_x(&self) -> i64 {
        self.hodge_x
            .iter()
            .enumerate()
            .flat_map(|(p, row)| {
                row.iter().enumerate().map(move |(q, &h)| {
                    if (p + q) % 2 == 0 {
                        h as i64
                    } else {
                        -(h as i64)
                    }
                })
            })
            .sum()
    }
    /// Hodge number h^{1,1} for X (Kähler moduli count).
    pub fn h11_x(&self) -> u64 {
        if self.complex_dim >= 1 {
            self.hodge_x[1][1]
        } else {
            0
        }
    }
    /// Hodge number h^{1,1} for Y (mirror = h^{n-1,1}(X)).
    pub fn h11_y(&self) -> u64 {
        if self.complex_dim >= 1 {
            self.hodge_y[1][1]
        } else {
            0
        }
    }
}
/// A numerical integrator for Hamiltonian mechanics.
#[derive(Debug, Clone)]
pub struct SymplecticIntegrator {
    /// Time step (s).
    pub dt: f64,
    /// Integration method.
    pub method: IntegratorMethod,
}
impl SymplecticIntegrator {
    /// Advance the state `(q, p)` by one time step.
    ///
    /// `grad_H` returns `(∂H/∂p, ∂H/∂q)` — the Hamiltonian gradients used
    /// to compute Hamilton's equations q̇ = ∂H/∂p, ṗ = −∂H/∂q.
    pub fn step(
        &self,
        q: &[f64],
        p: &[f64],
        grad_h: &dyn Fn(&[f64], &[f64]) -> (Vec<f64>, Vec<f64>),
    ) -> (Vec<f64>, Vec<f64>) {
        let dt = self.dt;
        match self.method {
            IntegratorMethod::Euler => {
                let (dh_dp, dh_dq) = grad_h(q, p);
                let q_new: Vec<f64> = q
                    .iter()
                    .zip(&dh_dp)
                    .map(|(&qi, &di)| qi + dt * di)
                    .collect();
                let p_new: Vec<f64> = p
                    .iter()
                    .zip(&dh_dq)
                    .map(|(&pi, &di)| pi - dt * di)
                    .collect();
                (q_new, p_new)
            }
            IntegratorMethod::SymplecticEuler => {
                let (_, dh_dq) = grad_h(q, p);
                let p_new: Vec<f64> = p
                    .iter()
                    .zip(&dh_dq)
                    .map(|(&pi, &di)| pi - dt * di)
                    .collect();
                let (dh_dp_new, _) = grad_h(q, &p_new);
                let q_new: Vec<f64> = q
                    .iter()
                    .zip(&dh_dp_new)
                    .map(|(&qi, &di)| qi + dt * di)
                    .collect();
                (q_new, p_new)
            }
            IntegratorMethod::Leapfrog => {
                let (_, dh_dq) = grad_h(q, p);
                let p_half: Vec<f64> = p
                    .iter()
                    .zip(&dh_dq)
                    .map(|(&pi, &di)| pi - 0.5 * dt * di)
                    .collect();
                let (dh_dp_half, _) = grad_h(q, &p_half);
                let q_new: Vec<f64> = q
                    .iter()
                    .zip(&dh_dp_half)
                    .map(|(&qi, &di)| qi + dt * di)
                    .collect();
                let (_, dh_dq_new) = grad_h(&q_new, &p_half);
                let p_new: Vec<f64> = p_half
                    .iter()
                    .zip(&dh_dq_new)
                    .map(|(&pi, &di)| pi - 0.5 * dt * di)
                    .collect();
                (q_new, p_new)
            }
            IntegratorMethod::RK4 => {
                let f = |q: &[f64], p: &[f64]| -> (Vec<f64>, Vec<f64>) {
                    let (dh_dp, dh_dq) = grad_h(q, p);
                    let dq: Vec<f64> = dh_dp;
                    let dp: Vec<f64> = dh_dq.iter().map(|&d| -d).collect();
                    (dq, dp)
                };
                let (k1q, k1p) = f(q, p);
                let q1: Vec<f64> = q
                    .iter()
                    .zip(&k1q)
                    .map(|(&qi, &ki)| qi + 0.5 * dt * ki)
                    .collect();
                let p1: Vec<f64> = p
                    .iter()
                    .zip(&k1p)
                    .map(|(&pi, &ki)| pi + 0.5 * dt * ki)
                    .collect();
                let (k2q, k2p) = f(&q1, &p1);
                let q2: Vec<f64> = q
                    .iter()
                    .zip(&k2q)
                    .map(|(&qi, &ki)| qi + 0.5 * dt * ki)
                    .collect();
                let p2: Vec<f64> = p
                    .iter()
                    .zip(&k2p)
                    .map(|(&pi, &ki)| pi + 0.5 * dt * ki)
                    .collect();
                let (k3q, k3p) = f(&q2, &p2);
                let q3: Vec<f64> = q.iter().zip(&k3q).map(|(&qi, &ki)| qi + dt * ki).collect();
                let p3: Vec<f64> = p.iter().zip(&k3p).map(|(&pi, &ki)| pi + dt * ki).collect();
                let (k4q, k4p) = f(&q3, &p3);
                let q_new: Vec<f64> = q
                    .iter()
                    .enumerate()
                    .map(|(i, &qi)| qi + dt / 6.0 * (k1q[i] + 2.0 * k2q[i] + 2.0 * k3q[i] + k4q[i]))
                    .collect();
                let p_new: Vec<f64> = p
                    .iter()
                    .enumerate()
                    .map(|(i, &pi)| pi + dt / 6.0 * (k1p[i] + 2.0 * k2p[i] + 2.0 * k3p[i] + k4p[i]))
                    .collect();
                (q_new, p_new)
            }
        }
    }
}
/// A U(1) lattice gauge field on a 2D periodic lattice.
///
/// Each link (i, j, mu) stores a phase angle θ ∈ \[−π, π\], representing
/// the gauge connection U_{i,mu} = e^{iθ_{i,mu}}.
///
/// The Wilson plaquette action is S = β Σ_{plaq} (1 − cos(θ_plaq)).
#[derive(Debug, Clone)]
pub struct GaugeField {
    /// Lattice size in each spatial direction.
    pub size: usize,
    /// Link angles: `links[mu]\[i * size + j\]` for direction mu, site (i,j).
    pub links: Vec<Vec<f64>>,
    /// Inverse coupling β = 1/g².
    pub beta: f64,
}
impl GaugeField {
    /// Create a cold-start gauge field (all links set to zero angle).
    pub fn new_cold(size: usize, beta: f64) -> Self {
        let n = size * size;
        Self {
            size,
            links: vec![vec![0.0; n], vec![0.0; n]],
            beta,
        }
    }
    /// Site index for lattice coordinate (i, j) with periodic boundary.
    fn site(&self, i: usize, j: usize) -> usize {
        (i % self.size) * self.size + (j % self.size)
    }
    /// Compute the plaquette angle sum θ_01(i,j) = θ_0(i,j) + θ_1(i+1,j)
    ///                                               − θ_0(i,j+1) − θ_1(i,j).
    pub fn plaquette_angle(&self, i: usize, j: usize) -> f64 {
        let s00 = self.site(i, j);
        let s10 = self.site(i + 1, j);
        let s01 = self.site(i, j + 1);
        self.links[0][s00] + self.links[1][s10] - self.links[0][s01] - self.links[1][s00]
    }
    /// Wilson plaquette action: S = β Σ (1 − cos θ_plaq).
    pub fn action(&self) -> f64 {
        let n = self.size;
        let mut s = 0.0;
        for i in 0..n {
            for j in 0..n {
                s += 1.0 - self.plaquette_angle(i, j).cos();
            }
        }
        self.beta * s
    }
    /// Polyakov loop in the μ=0 direction at column j: L(j) = Π_i U_{i,0}(j).
    pub fn polyakov_loop(&self, j: usize) -> f64 {
        let angle: f64 = (0..self.size).map(|i| self.links[0][self.site(i, j)]).sum();
        angle.cos()
    }
}
/// The supersymmetric harmonic oscillator spectrum.
///
/// The SUSY oscillator has bosonic and fermionic modes.
/// States are labeled by (n_b, n_f) where n_b ∈ ℕ and n_f ∈ {0, 1}.
/// Energy = ω * n_b (zero-point energies cancel).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SupersymmetricOscillator {
    /// Angular frequency ω.
    pub omega: f64,
}
#[allow(dead_code)]
impl SupersymmetricOscillator {
    /// Create a SUSY oscillator with given frequency.
    pub fn new(omega: f64) -> Self {
        assert!(omega > 0.0, "omega must be positive");
        Self { omega }
    }
    /// Energy of state (n_b, n_f): E = ω * n_b.
    pub fn energy(&self, n_b: u64, _n_f: u64) -> f64 {
        self.omega * (n_b as f64)
    }
    /// Number of states at energy level E = ω * n (two states for n > 0, one for n = 0).
    pub fn degeneracy(&self, n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            2
        }
    }
    /// Witten index: Tr((-1)^F e^{-βH}) = 1 (SUSY unbroken).
    pub fn witten_index(&self) -> i64 {
        1
    }
    /// Partition function Z(β) = Σ_n e^{-βEn} * degeneracy(n), truncated.
    pub fn partition_function(&self, beta: f64, n_max: u64) -> f64 {
        (0..=n_max)
            .map(|n| {
                let e = self.energy(n, 0);
                (self.degeneracy(n) as f64) * (-beta * e).exp()
            })
            .sum()
    }
    /// Supersymmetric ground state energy (exactly zero).
    pub fn ground_state_energy(&self) -> f64 {
        0.0
    }
}
/// A tracker for Donaldson-Witten invariants of a smooth 4-manifold.
///
/// Stores the instanton moduli space dimensions and mock invariant values
/// for different instantiation numbers k.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DonaldsonWitten {
    /// Name of the 4-manifold.
    pub manifold_name: String,
    /// Signature σ of the manifold.
    pub signature: i64,
    /// Euler characteristic χ.
    pub euler_char: i64,
    /// Stored Donaldson invariants: (instanton_number, invariant_value).
    pub invariants: Vec<(u64, f64)>,
}
#[allow(dead_code)]
impl DonaldsonWitten {
    /// Create a new invariant tracker.
    pub fn new(name: impl Into<String>, signature: i64, euler_char: i64) -> Self {
        Self {
            manifold_name: name.into(),
            signature,
            euler_char,
            invariants: Vec::new(),
        }
    }
    /// Virtual dimension of the instanton moduli space M_k(G=SU(2)):
    /// dim M_k = 8k − 3(1 + b₁ − b₂⁺).
    pub fn virtual_dimension(&self, k: u64) -> i64 {
        let b2_plus = (self.euler_char + self.signature) / 2;
        let b1 = 0i64;
        8 * (k as i64) - 3 * (1 + b1 - b2_plus)
    }
    /// Add a Donaldson invariant for instanton number k.
    pub fn add_invariant(&mut self, k: u64, value: f64) {
        self.invariants.push((k, value));
    }
    /// Look up the invariant for instanton number k.
    pub fn get_invariant(&self, k: u64) -> Option<f64> {
        self.invariants
            .iter()
            .find(|&&(kk, _)| kk == k)
            .map(|&(_, v)| v)
    }
    /// b₂⁺ = (χ + σ) / 2.
    pub fn b2_plus(&self) -> i64 {
        (self.euler_char + self.signature) / 2
    }
    /// b₂⁻ = (χ − σ) / 2.
    pub fn b2_minus(&self) -> i64 {
        (self.euler_char - self.signature) / 2
    }
}
