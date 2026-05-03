//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// An OPE coefficient table for a 2D CFT.
/// Stores C_{ij}^k (structure constants of the operator algebra).
#[derive(Debug, Clone)]
pub struct OpeTable {
    /// Operator labels (by name).
    pub operators: Vec<String>,
    /// Conformal dimensions Δ_i.
    pub dimensions: Vec<f64>,
    /// OPE coefficients C_{ij}^k stored as (i, j, k, value).
    pub coefficients: Vec<(usize, usize, usize, f64)>,
}
impl OpeTable {
    pub fn new() -> Self {
        OpeTable {
            operators: Vec::new(),
            dimensions: Vec::new(),
            coefficients: Vec::new(),
        }
    }
    /// Add an operator with given conformal dimension.
    pub fn add_operator(&mut self, name: &str, delta: f64) -> usize {
        let idx = self.operators.len();
        self.operators.push(name.to_string());
        self.dimensions.push(delta);
        idx
    }
    /// Set OPE coefficient C_{i,j}^k.
    pub fn set_coefficient(&mut self, i: usize, j: usize, k: usize, c: f64) {
        self.coefficients.push((i, j, k, c));
    }
    /// Retrieve OPE coefficient C_{i,j}^k (returns 0 if not set).
    pub fn get_coefficient(&self, i: usize, j: usize, k: usize) -> f64 {
        for &(ci, cj, ck, cv) in &self.coefficients {
            if ci == i && cj == j && ck == k {
                return cv;
            }
        }
        0.0
    }
    /// Check the bootstrap crossing equation for a 4-point function (simplified).
    /// Returns |Σ_k C_{12}^k C_{34}^k - Σ_k C_{13}^k C_{24}^k|.
    pub fn crossing_residual(&self, o1: usize, o2: usize, o3: usize, o4: usize) -> f64 {
        let n = self.operators.len();
        let mut s_channel = 0.0;
        let mut t_channel = 0.0;
        for k in 0..n {
            s_channel += self.get_coefficient(o1, o2, k) * self.get_coefficient(o3, o4, k);
            t_channel += self.get_coefficient(o1, o3, k) * self.get_coefficient(o2, o4, k);
        }
        (s_channel - t_channel).abs()
    }
}
/// Data for BRST cohomology in gauge theories.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BRSTData {
    /// BRST charge Q (nilpotent: Q^2 = 0).
    pub brst_charge_name: String,
    /// Physical states (BRST cohomology classes).
    pub physical_states: Vec<String>,
    /// Ghost number of each state.
    pub ghost_numbers: Vec<i32>,
}
#[allow(dead_code)]
impl BRSTData {
    /// Creates BRST data.
    pub fn new() -> Self {
        BRSTData {
            brst_charge_name: "Q_BRST".to_string(),
            physical_states: Vec::new(),
            ghost_numbers: Vec::new(),
        }
    }
    /// Adds a physical state with ghost number.
    pub fn add_state(&mut self, state: &str, ghost_number: i32) {
        self.physical_states.push(state.to_string());
        self.ghost_numbers.push(ghost_number);
    }
    /// Checks nilpotency: Q^2 = 0 (always true by construction).
    pub fn is_nilpotent(&self) -> bool {
        true
    }
    /// Returns physical states with ghost number 0 (the true physical states).
    pub fn physical_ghost_zero(&self) -> Vec<&str> {
        self.physical_states
            .iter()
            .zip(self.ghost_numbers.iter())
            .filter(|(_, &g)| g == 0)
            .map(|(s, _)| s.as_str())
            .collect()
    }
    /// BRST cohomology description.
    pub fn cohomology_description(&self) -> String {
        format!(
            "H^0({}) = {} physical states",
            self.brst_charge_name,
            self.physical_ghost_zero().len()
        )
    }
}
/// A simple model of BRST cohomology over a graded vector space.
#[derive(Debug, Clone)]
pub struct BrstComplex {
    /// Cochains at each ghost number grading.
    pub cochains: Vec<Vec<f64>>,
    /// The differential s: cochain\[k\] → cochain[k+1].
    /// Stored as a sequence of matrices.
    pub differentials: Vec<Vec<Vec<f64>>>,
}
impl BrstComplex {
    /// Create a trivial BRST complex with given dimensions.
    pub fn new(dimensions: Vec<usize>) -> Self {
        let n = dimensions.len();
        let cochains = dimensions.iter().map(|&d| vec![0.0; d]).collect();
        let differentials = (0..n.saturating_sub(1))
            .map(|k| {
                let rows = dimensions[k + 1];
                let cols = dimensions[k];
                vec![vec![0.0; cols]; rows]
            })
            .collect();
        BrstComplex {
            cochains,
            differentials,
        }
    }
    /// Apply the BRST differential s to a cochain at grading k.
    pub fn apply_differential(&self, k: usize, v: &[f64]) -> Vec<f64> {
        if k >= self.differentials.len() {
            return vec![];
        }
        let d = &self.differentials[k];
        d.iter()
            .map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum())
            .collect()
    }
    /// Check s² = 0: applying the differential twice gives 0.
    pub fn check_nilpotency(&self, k: usize, v: &[f64]) -> bool {
        if k + 1 >= self.differentials.len() {
            return true;
        }
        let sv = self.apply_differential(k, v);
        let ssv = self.apply_differential(k + 1, &sv);
        ssv.iter().all(|x| x.abs() < 1e-12)
    }
}
/// A Feynman diagram represented combinatorially.
#[derive(Debug, Clone)]
pub struct FeynmanDiagram {
    /// Number of external legs.
    pub n_external: usize,
    /// Number of internal vertices.
    pub n_vertices: usize,
    /// Number of internal propagator lines.
    pub n_propagators: usize,
    /// Coupling constant order (power of λ for φ⁴ theory).
    pub coupling_order: usize,
}
impl FeynmanDiagram {
    pub fn new(n_external: usize, n_vertices: usize, n_propagators: usize) -> Self {
        FeynmanDiagram {
            n_external,
            n_vertices,
            n_propagators,
            coupling_order: n_vertices,
        }
    }
    /// Loop number: L = I - V + 1 (connected diagram formula).
    pub fn loop_number(&self) -> i64 {
        self.n_propagators as i64 - self.n_vertices as i64 + 1
    }
    /// Superficial degree of divergence in d=4 for φ⁴ theory:
    /// D = 4L - 2I = 4 - E  (E = external legs, for renormalizable φ⁴)
    pub fn superficial_divergence(&self, spacetime_dim: u32) -> i64 {
        let l = self.loop_number();
        let loops = if l > 0 { l } else { 0 };
        (spacetime_dim as i64) * loops - 2 * (self.n_propagators as i64)
    }
    /// Is this diagram UV divergent in d=4?
    pub fn is_uv_divergent(&self) -> bool {
        self.superficial_divergence(4) >= 0
    }
    /// Compute the symmetry factor for a simple φ⁴ diagram at 1-loop.
    /// For the "figure-8" vacuum diagram: S = 8.
    /// For the 1-loop 2-point function: S = 2.
    pub fn symmetry_factor_estimate(&self) -> u64 {
        match (self.n_external, self.loop_number()) {
            (0, 2) => 8,
            (2, 1) => 2,
            (4, 1) => 1,
            _ => 1,
        }
    }
}
/// Computes the Virasoro algebra commutator \[L_m, L_n\].
/// \[L_m, L_n\] = (m-n) L_{m+n} + (c/12) m(m²-1) δ_{m+n,0}
#[derive(Debug, Clone, Copy)]
pub struct VirasoroCommutator {
    /// Central charge c.
    pub central_charge: f64,
}
impl VirasoroCommutator {
    pub fn new(central_charge: f64) -> Self {
        VirasoroCommutator { central_charge }
    }
    /// Compute \[L_m, L_n\] expressed as: (coeff_of_L, index_of_L, anomaly_term).
    /// Returns (coefficient of L_{m+n}, anomaly = c/12 m(m²-1) δ_{m+n,0}).
    pub fn commutator(&self, m: i64, n: i64) -> (f64, i64, f64) {
        let coeff = (m - n) as f64;
        let l_index = m + n;
        let anomaly = if m + n == 0 {
            self.central_charge / 12.0 * (m as f64) * ((m * m - 1) as f64)
        } else {
            0.0
        };
        (coeff, l_index, anomaly)
    }
    /// Check the Jacobi identity for L_m, L_n, L_p:
    /// [L_m,\[L_n,L_p\]] + [L_n,\[L_p,L_m\]] + [L_p,\[L_m,L_n\]] = 0.
    /// Here we check only the coefficient of L_{m+n+p} vanishes.
    pub fn check_jacobi(&self, m: i64, n: i64, p: i64) -> bool {
        let a = (m - n) * (m + n - p);
        let b = (n - p) * (n + p - m);
        let c = (p - m) * (p + m - n);
        a + b + c == 0
    }
    /// Witt algebra limit (c=0): \[L_m, L_n\] = (m-n) L_{m+n}.
    pub fn witt_commutator(m: i64, n: i64) -> (f64, i64) {
        ((m - n) as f64, m + n)
    }
}
/// Evaluator for simple scalar φ⁴ Feynman rules in momentum space.
/// Each propagator contributes i/(p²-m²) and each vertex contributes -iλ.
#[derive(Debug, Clone)]
pub struct FeynmanDiagramEvaluator {
    /// Coupling constant λ.
    pub lambda: f64,
    /// Scalar mass m.
    pub mass: f64,
}
impl FeynmanDiagramEvaluator {
    pub fn new(lambda: f64, mass: f64) -> Self {
        FeynmanDiagramEvaluator { lambda, mass }
    }
    /// Evaluate a single propagator i/(p²-m²+iε).
    /// Returns (re, im) of the Feynman propagator.
    pub fn propagator(&self, p_sq: f64, epsilon: f64) -> QftComplex {
        let denom_re = p_sq - self.mass * self.mass;
        let denom_im = epsilon;
        let denom_sq = denom_re * denom_re + denom_im * denom_im;
        QftComplex::new(denom_im / denom_sq, denom_re / denom_sq)
    }
    /// Evaluate a φ⁴ vertex: returns -iλ.
    pub fn vertex(&self) -> QftComplex {
        QftComplex::new(0.0, -self.lambda)
    }
    /// Evaluate a tree-level 2→2 scattering amplitude (all momenta on-shell).
    /// M = -iλ (leading order, one vertex, three channels).
    pub fn tree_amplitude_2to2(&self) -> QftComplex {
        self.vertex()
    }
    /// Estimate 1-loop correction to 2→2 amplitude via a bubble diagram.
    /// Approximated as (-iλ)² · I_bubble where I_bubble ~ i/(16π²) ln(Λ²/m²).
    pub fn one_loop_bubble_approx(&self, lambda_uv: f64) -> QftComplex {
        use std::f64::consts::PI;
        let log_factor = (lambda_uv * lambda_uv / (self.mass * self.mass)).ln();
        let i_bubble_re = 0.0;
        let i_bubble_im = log_factor / (16.0 * PI * PI);
        let i_bubble = QftComplex::new(i_bubble_re, i_bubble_im);
        let v = self.vertex();
        v.mul(&v).mul(&i_bubble)
    }
}
/// A complex number a + bi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QftComplex {
    pub re: f64,
    pub im: f64,
}
impl QftComplex {
    pub fn new(re: f64, im: f64) -> Self {
        QftComplex { re, im }
    }
    pub fn zero() -> Self {
        QftComplex { re: 0.0, im: 0.0 }
    }
    pub fn one() -> Self {
        QftComplex { re: 1.0, im: 0.0 }
    }
    pub fn i() -> Self {
        QftComplex { re: 0.0, im: 1.0 }
    }
    pub fn abs_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(&self) -> f64 {
        self.abs_sq().sqrt()
    }
    pub fn conj(&self) -> Self {
        QftComplex {
            re: self.re,
            im: -self.im,
        }
    }
    pub fn add(&self, other: &QftComplex) -> QftComplex {
        QftComplex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    pub fn mul(&self, other: &QftComplex) -> QftComplex {
        QftComplex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    pub fn scale(&self, s: f64) -> QftComplex {
        QftComplex {
            re: self.re * s,
            im: self.im * s,
        }
    }
    /// exp(iθ) = cos θ + i sin θ
    pub fn exp_i(theta: f64) -> QftComplex {
        QftComplex {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
}
/// Computes 2D Ising model spin-spin correlation function on a square lattice.
/// Uses the transfer matrix approach for the 1D chain approximation.
#[derive(Debug, Clone)]
pub struct CorrelationFunctionLattice {
    /// Inverse temperature β = J/(k_B T).
    pub beta: f64,
    /// Lattice size (N×N).
    pub size: usize,
    /// Spin configuration (+1 or -1 per site).
    pub spins: Vec<Vec<i32>>,
}
impl CorrelationFunctionLattice {
    /// Create a ferromagnetic initial configuration (all spins up).
    pub fn new_ferromagnetic(size: usize, beta: f64) -> Self {
        CorrelationFunctionLattice {
            beta,
            size,
            spins: vec![vec![1i32; size]; size],
        }
    }
    /// Compute the energy E = -J Σ_{ij} s_i s_j (with J=1).
    pub fn energy(&self) -> f64 {
        let n = self.size;
        let mut e = 0.0;
        for i in 0..n {
            for j in 0..n {
                let right = self.spins[i][(j + 1) % n] as f64;
                let down = self.spins[(i + 1) % n][j] as f64;
                e -= (self.spins[i][j] as f64) * (right + down);
            }
        }
        e
    }
    /// Magnetization per site: m = (1/N²) Σ s_i.
    pub fn magnetization(&self) -> f64 {
        let total: i32 = self.spins.iter().flat_map(|row| row.iter()).sum();
        total as f64 / (self.size * self.size) as f64
    }
    /// Metropolis single-spin flip update.
    /// Uses a simple linear congruential generator for reproducibility.
    pub fn metropolis_step(&mut self, rng_state: &mut u64) {
        let n = self.size;
        *rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let i = ((*rng_state >> 33) as usize) % n;
        *rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = ((*rng_state >> 33) as usize) % n;
        let s = self.spins[i][j];
        let neighbors = self.spins[(i + n - 1) % n][j]
            + self.spins[(i + 1) % n][j]
            + self.spins[i][(j + n - 1) % n]
            + self.spins[i][(j + 1) % n];
        let delta_e = 2 * s * neighbors;
        *rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let rand_float = (*rng_state >> 11) as f64 / (1u64 << 53) as f64;
        if delta_e <= 0 || rand_float < (-self.beta * delta_e as f64).exp() {
            self.spins[i][j] = -s;
        }
    }
    /// Two-point spin correlation ⟨s(0,0) s(r,0)⟩ (sampled from current config).
    pub fn two_point_correlation(&self, r: usize) -> f64 {
        let n = self.size;
        let mut corr = 0.0;
        for i in 0..n {
            for j in 0..n {
                corr += (self.spins[i][j] as f64) * (self.spins[i][(j + r) % n] as f64);
            }
        }
        corr / (n * n) as f64
    }
    /// Analytical Ising correlation length: ξ = -1/ln(tanh(β)).
    pub fn analytical_correlation_length(&self) -> f64 {
        let t = self.beta.tanh();
        if t <= 0.0 {
            f64::INFINITY
        } else {
            -1.0 / t.ln()
        }
    }
}
/// Data for a 2D conformal field theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConformalFieldTheory2D {
    /// Central charge c.
    pub central_charge: f64,
    /// Primary operators (name, dimension h, spin s).
    pub primary_operators: Vec<(String, f64, f64)>,
    /// Whether the theory is rational (RCFT).
    pub is_rational: bool,
}
#[allow(dead_code)]
impl ConformalFieldTheory2D {
    /// Creates a 2D CFT.
    pub fn new(c: f64) -> Self {
        ConformalFieldTheory2D {
            central_charge: c,
            primary_operators: Vec::new(),
            is_rational: false,
        }
    }
    /// Creates the free boson CFT (c=1).
    pub fn free_boson() -> Self {
        ConformalFieldTheory2D::new(1.0)
    }
    /// Creates the Ising model CFT (c=1/2).
    pub fn ising() -> Self {
        let mut cft = ConformalFieldTheory2D::new(0.5);
        cft.is_rational = true;
        cft.add_primary("1", 0.0, 0.0);
        cft.add_primary("σ", 0.0625, 0.0);
        cft.add_primary("ε", 0.5, 0.0);
        cft
    }
    /// Adds a primary operator.
    pub fn add_primary(&mut self, name: &str, h: f64, s: f64) {
        self.primary_operators.push((name.to_string(), h, s));
    }
    /// OPE coefficient c_{12}^3 ≈ 1 (placeholder).
    pub fn ope_coefficient_approx(&self, _i: usize, _j: usize, _k: usize) -> f64 {
        1.0
    }
    /// Virasoro character of primary with dimension h.
    /// χ_h(q) = q^{h - c/24} / η(q) (simplified: return exponent only).
    pub fn character_exponent(&self, h: f64) -> f64 {
        h - self.central_charge / 24.0
    }
    /// Modular S-matrix element for RCFT (Verlinde formula input).
    pub fn verlinde_data(&self) -> String {
        format!(
            "RCFT with c={}, {} primaries (Verlinde formula applies: {})",
            self.central_charge,
            self.primary_operators.len(),
            self.is_rational
        )
    }
}
/// Data for scattering amplitude computations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScatteringAmplitude {
    /// Process description.
    pub process: String,
    /// Number of external particles.
    pub n_particles: usize,
    /// Mandelstam variables (s, t, u).
    pub mandelstam: Option<(f64, f64, f64)>,
    /// Tree-level amplitude (as a real approximation).
    pub tree_amplitude: f64,
}
#[allow(dead_code)]
impl ScatteringAmplitude {
    /// Creates scattering amplitude data.
    pub fn new(process: &str, n: usize) -> Self {
        ScatteringAmplitude {
            process: process.to_string(),
            n_particles: n,
            mandelstam: None,
            tree_amplitude: 0.0,
        }
    }
    /// Sets Mandelstam variables.
    pub fn with_mandelstam(mut self, s: f64, t: f64, u: f64) -> Self {
        self.mandelstam = Some((s, t, u));
        self
    }
    /// Sets tree-level amplitude.
    pub fn with_tree_amplitude(mut self, a: f64) -> Self {
        self.tree_amplitude = a;
        self
    }
    /// Checks crossing symmetry: s + t + u = sum of masses^2 (= 0 for massless).
    pub fn crossing_satisfied(&self, mass_sq_sum: f64, tol: f64) -> bool {
        if let Some((s, t, u)) = self.mandelstam {
            (s + t + u - mass_sq_sum).abs() < tol
        } else {
            false
        }
    }
    /// Returns the Parke-Taylor amplitude description for n-gluon scattering.
    pub fn parke_taylor_description(&self) -> String {
        if self.n_particles >= 4 {
            format!(
                "Parke-Taylor: A_n = <ij>^4 / (<12><23>...<n1>) for {} gluons",
                self.n_particles
            )
        } else {
            "Parke-Taylor not applicable for < 4 particles".to_string()
        }
    }
}
/// A discretized Klein-Gordon field on a 1D lattice.
#[derive(Debug, Clone)]
pub struct KleinGordonField {
    /// Number of lattice sites.
    pub n_sites: usize,
    /// Lattice spacing.
    pub dx: f64,
    /// Mass parameter m².
    pub mass_sq: f64,
    /// Field values φ_i at each site.
    pub phi: Vec<f64>,
    /// Conjugate momenta π_i = ∂_t φ_i at each site.
    pub pi: Vec<f64>,
}
impl KleinGordonField {
    /// Create a zero field configuration.
    pub fn new(n_sites: usize, dx: f64, mass_sq: f64) -> Self {
        KleinGordonField {
            n_sites,
            dx,
            mass_sq,
            phi: vec![0.0; n_sites],
            pi: vec![0.0; n_sites],
        }
    }
    /// Compute the Hamiltonian H = Σ_i \[½ π_i² + ½((φ_{i+1}-φ_i)/dx)² + ½m²φ_i²\].
    pub fn hamiltonian(&self) -> f64 {
        let mut h = 0.0;
        for i in 0..self.n_sites {
            let next = (i + 1) % self.n_sites;
            let pi_sq = self.pi[i] * self.pi[i];
            let grad = (self.phi[next] - self.phi[i]) / self.dx;
            let pot = self.mass_sq * self.phi[i] * self.phi[i];
            h += 0.5 * (pi_sq + grad * grad + pot) * self.dx;
        }
        h
    }
    /// Compute the equations-of-motion residual: □φ + m²φ at site i.
    /// Uses finite differences: residual = (φ_{i-1} - 2φ_i + φ_{i+1})/dx² - m²φ_i
    pub fn eom_residual(&self, i: usize) -> f64 {
        let prev = if i == 0 { self.n_sites - 1 } else { i - 1 };
        let next = (i + 1) % self.n_sites;
        let laplacian = (self.phi[prev] - 2.0 * self.phi[i] + self.phi[next]) / (self.dx * self.dx);
        laplacian - self.mass_sq * self.phi[i]
    }
    /// Symplectic (leapfrog) time evolution by one step dt.
    pub fn step(&mut self, dt: f64) {
        for i in 0..self.n_sites {
            let prev = if i == 0 { self.n_sites - 1 } else { i - 1 };
            let next = (i + 1) % self.n_sites;
            let laplacian =
                (self.phi[prev] - 2.0 * self.phi[i] + self.phi[next]) / (self.dx * self.dx);
            let force = laplacian - self.mass_sq * self.phi[i];
            self.pi[i] += 0.5 * dt * force;
        }
        for i in 0..self.n_sites {
            self.phi[i] += dt * self.pi[i];
        }
        for i in 0..self.n_sites {
            let prev = if i == 0 { self.n_sites - 1 } else { i - 1 };
            let next = (i + 1) % self.n_sites;
            let laplacian =
                (self.phi[prev] - 2.0 * self.phi[i] + self.phi[next]) / (self.dx * self.dx);
            let force = laplacian - self.mass_sq * self.phi[i];
            self.pi[i] += 0.5 * dt * force;
        }
    }
}
/// A finite Fock space with a fixed number of modes.
#[derive(Debug, Clone)]
pub struct FiniteFockSpace {
    /// Occupation numbers for each mode.
    pub occupations: Vec<u64>,
    /// Whether this is a bosonic (false) or fermionic (true) Fock space.
    pub fermionic: bool,
}
impl FiniteFockSpace {
    /// Create a vacuum state with `n_modes` modes.
    pub fn vacuum(n_modes: usize, fermionic: bool) -> Self {
        FiniteFockSpace {
            occupations: vec![0; n_modes],
            fermionic,
        }
    }
    /// Total particle number N = Σ n_k.
    pub fn total_number(&self) -> u64 {
        self.occupations.iter().sum()
    }
    /// Apply creation operator a†_k to this state (bosonic).
    /// Returns None for fermionic states with occupation ≥ 1.
    pub fn create(&self, mode: usize) -> Option<(Self, f64)> {
        if mode >= self.occupations.len() {
            return None;
        }
        let n = self.occupations[mode];
        if self.fermionic && n >= 1 {
            return None;
        }
        let mut new_occ = self.occupations.clone();
        new_occ[mode] += 1;
        let norm = ((n + 1) as f64).sqrt();
        Some((
            FiniteFockSpace {
                occupations: new_occ,
                fermionic: self.fermionic,
            },
            norm,
        ))
    }
    /// Apply annihilation operator a_k to this state.
    /// Returns None if occupation is 0.
    pub fn annihilate(&self, mode: usize) -> Option<(Self, f64)> {
        if mode >= self.occupations.len() {
            return None;
        }
        let n = self.occupations[mode];
        if n == 0 {
            return None;
        }
        let mut new_occ = self.occupations.clone();
        new_occ[mode] -= 1;
        let norm = (n as f64).sqrt();
        Some((
            FiniteFockSpace {
                occupations: new_occ,
                fermionic: self.fermionic,
            },
            norm,
        ))
    }
    /// Compute ⟨N⟩ = total occupation number.
    pub fn number_expectation(&self) -> f64 {
        self.total_number() as f64
    }
}
/// One-loop RG beta function computer for various theories.
#[derive(Debug, Clone)]
pub struct BetaFunctionRG {
    /// Number of colors N_c (for non-abelian gauge theories).
    pub n_colors: u32,
    /// Number of quark/fermion flavors N_f.
    pub n_flavors: u32,
    /// Number of scalar fields N_s.
    pub n_scalars: u32,
}
impl BetaFunctionRG {
    pub fn new(n_colors: u32, n_flavors: u32, n_scalars: u32) -> Self {
        BetaFunctionRG {
            n_colors,
            n_flavors,
            n_scalars,
        }
    }
    /// One-loop beta function coefficient b₀ for SU(N_c) gauge theory.
    /// β(g) = -b₀ g³/(16π²) + ...
    /// b₀ = (11/3) N_c - (2/3) N_f - (1/6) N_s
    pub fn b0_coefficient(&self) -> f64 {
        let nc = self.n_colors as f64;
        let nf = self.n_flavors as f64;
        let ns = self.n_scalars as f64;
        (11.0 / 3.0) * nc - (2.0 / 3.0) * nf - (1.0 / 6.0) * ns
    }
    /// Is this theory asymptotically free? (b₀ > 0 means AF.)
    pub fn is_asymptotically_free(&self) -> bool {
        self.b0_coefficient() > 0.0
    }
    /// One-loop beta function: β(g) = -b₀ g³/(16π²).
    pub fn beta_function(&self, g: f64) -> f64 {
        use std::f64::consts::PI;
        -self.b0_coefficient() * g * g * g / (16.0 * PI * PI)
    }
    /// Running coupling at scale μ given g(μ₀) = g₀.
    /// g²(μ) = g₀² / (1 + b₀ g₀²/(8π²) ln(μ/μ₀))
    pub fn running_coupling(&self, g0: f64, mu: f64, mu0: f64) -> f64 {
        use std::f64::consts::PI;
        let log_ratio = (mu / mu0).ln();
        let g0_sq = g0 * g0;
        let denom = 1.0 + self.b0_coefficient() * g0_sq / (8.0 * PI * PI) * log_ratio;
        if denom <= 0.0 {
            f64::INFINITY
        } else {
            (g0_sq / denom).sqrt()
        }
    }
    /// One-loop anomalous dimension for the scalar field in φ⁴ theory.
    /// γ(g) = g²/(32π²)
    pub fn anomalous_dimension_phi4(g: f64) -> f64 {
        use std::f64::consts::PI;
        g * g / (32.0 * PI * PI)
    }
}
/// Data for a Euclidean path integral in QFT.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PathIntegralData {
    /// Action functional name.
    pub action_name: String,
    /// Coupling constants.
    pub couplings: Vec<(String, f64)>,
    /// Spacetime dimension.
    pub spacetime_dim: usize,
    /// Whether the theory is renormalizable.
    pub is_renormalizable: bool,
}
#[allow(dead_code)]
impl PathIntegralData {
    /// Creates path integral data.
    pub fn new(action: &str, dim: usize) -> Self {
        PathIntegralData {
            action_name: action.to_string(),
            couplings: Vec::new(),
            spacetime_dim: dim,
            is_renormalizable: true,
        }
    }
    /// Adds a coupling constant.
    pub fn add_coupling(&mut self, name: &str, val: f64) {
        self.couplings.push((name.to_string(), val));
    }
    /// Partition function Z = ∫ Dφ e^{-S\[φ\]} (formal description).
    pub fn partition_function_description(&self) -> String {
        format!("Z = ∫ Dφ exp(-S_{{{}}}[φ])", self.action_name)
    }
    /// Returns the superficial degree of divergence for a graph with L loops.
    /// D = d*L - 2*I where d = spacetime dimension, I = internal lines.
    pub fn superficial_divergence(&self, loops: usize, internal_lines: usize) -> i64 {
        self.spacetime_dim as i64 * loops as i64 - 2 * internal_lines as i64
    }
    /// Checks power-counting renormalizability: all vertex operators have dim <= d.
    pub fn power_counting_renormalizable(&self, max_coupling_dim: f64) -> bool {
        max_coupling_dim >= 0.0
    }
    /// Returns the Wilsonian effective action description.
    pub fn wilsonian_description(&self) -> String {
        format!("S_{{eff}}[φ; Λ] = S[φ] + loop corrections at cutoff Λ")
    }
}
/// Data for a Yang-Mills gauge theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct YangMillsTheory {
    /// Gauge group description.
    pub gauge_group: String,
    /// Rank of the gauge group.
    pub rank: usize,
    /// Number of colors N (for U(N) or SU(N)).
    pub n_colors: usize,
    /// Coupling constant g.
    pub coupling: f64,
    /// Whether the theory is asymptotically free.
    pub asymptotically_free: bool,
    /// Number of massless quark flavors.
    pub n_flavors: usize,
}
#[allow(dead_code)]
impl YangMillsTheory {
    /// Creates a Yang-Mills theory.
    pub fn new(group: &str, n: usize, coupling: f64) -> Self {
        YangMillsTheory {
            gauge_group: group.to_string(),
            rank: n - 1,
            n_colors: n,
            coupling,
            asymptotically_free: true,
            n_flavors: 0,
        }
    }
    /// Sets number of quark flavors.
    pub fn with_flavors(mut self, n_f: usize) -> Self {
        self.n_flavors = n_f;
        let beta0 = 11.0 * self.n_colors as f64 / 3.0 - 2.0 * n_f as f64 / 3.0;
        self.asymptotically_free = beta0 > 0.0;
        self
    }
    /// One-loop beta function coefficient β_0 = 11N/3 - 2N_f/3.
    pub fn beta0(&self) -> f64 {
        11.0 * self.n_colors as f64 / 3.0 - 2.0 * self.n_flavors as f64 / 3.0
    }
    /// Running coupling at scale μ: g(μ)^2 ≈ g(μ_0)^2 / (1 + β_0 g^2 / (8π^2) * log(μ/μ_0)).
    pub fn running_coupling_squared(&self, mu: f64, mu0: f64) -> f64 {
        let g0_sq = self.coupling * self.coupling;
        let beta0 = self.beta0();
        let log_ratio = (mu / mu0).abs().ln();
        let denom = 1.0 + beta0 * g0_sq / (8.0 * std::f64::consts::PI.powi(2)) * log_ratio;
        if denom <= 0.0 {
            f64::INFINITY
        } else {
            g0_sq / denom
        }
    }
    /// Confinement scale Λ_QCD (where coupling becomes strong).
    pub fn qcd_scale(&self, mu0: f64) -> f64 {
        let g0_sq = self.coupling * self.coupling;
        let beta0 = self.beta0();
        let exponent = -8.0 * std::f64::consts::PI.powi(2) / (beta0 * g0_sq);
        mu0 * exponent.exp()
    }
    /// Dual Coxeter number for SU(N): h^∨ = N.
    pub fn dual_coxeter_number(&self) -> usize {
        self.n_colors
    }
}
