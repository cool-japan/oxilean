//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A complex number a + bi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrComplex {
    pub re: f64,
    pub im: f64,
}
impl StrComplex {
    pub fn new(re: f64, im: f64) -> Self {
        StrComplex { re, im }
    }
    pub fn zero() -> Self {
        StrComplex { re: 0.0, im: 0.0 }
    }
    pub fn abs_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(&self) -> f64 {
        self.abs_sq().sqrt()
    }
    pub fn add(&self, other: &StrComplex) -> StrComplex {
        StrComplex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    pub fn mul(&self, other: &StrComplex) -> StrComplex {
        StrComplex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    /// exp(iθ) = cos θ + i sin θ
    pub fn exp_i(theta: f64) -> StrComplex {
        StrComplex {
            re: theta.cos(),
            im: theta.sin(),
        }
    }
}
/// Calabi-Yau Hodge number database and calculator.
///
/// Tracks Hodge numbers h^{p,q} for Calabi-Yau n-folds.
#[derive(Debug, Clone)]
pub struct CalabiyauHodge {
    /// Complex dimension n (CY_n).
    pub complex_dim: u32,
    /// h^{1,1}: number of Kähler deformations.
    pub h11: u32,
    /// h^{2,1}: number of complex structure deformations (for CY3).
    pub h21: u32,
    /// Name of the CY manifold.
    pub name: String,
}
impl CalabiyauHodge {
    /// Create a CY Hodge data set.
    pub fn new(complex_dim: u32, h11: u32, h21: u32, name: impl Into<String>) -> Self {
        CalabiyauHodge {
            complex_dim,
            h11,
            h21,
            name: name.into(),
        }
    }
    /// Quintic threefold in ℙ⁴: (h11, h21) = (1, 101).
    pub fn quintic() -> Self {
        Self::new(3, 1, 101, "Quintic in P^4")
    }
    /// Mirror quintic: (h11, h21) = (101, 1).
    pub fn mirror_quintic() -> Self {
        Self::new(3, 101, 1, "Mirror quintic")
    }
    /// Octic in weighted ℙ(1,1,2,2,2): (h11, h21) = (2, 86).
    pub fn octic_wp() -> Self {
        Self::new(3, 2, 86, "Octic in WP(1,1,2,2,2)")
    }
    /// Euler characteristic χ = 2(h^{1,1} - h^{2,1}) for a CY3.
    pub fn euler_characteristic(&self) -> i64 {
        2 * (self.h11 as i64 - self.h21 as i64)
    }
    /// Total number of moduli: h11 + h21.
    pub fn total_moduli(&self) -> u32 {
        self.h11 + self.h21
    }
    /// Check if this is a mirror of the given CY.
    pub fn is_mirror_of(&self, other: &CalabiyauHodge) -> bool {
        self.h11 == other.h21 && self.h21 == other.h11 && self.complex_dim == other.complex_dim
    }
    /// Number of generations in a heterotic compactification: |χ|/2.
    pub fn num_generations(&self) -> u64 {
        self.euler_characteristic().unsigned_abs() / 2
    }
    /// Check Noether-Lefschetz: h^{2,1} ≥ 1 (non-trivial complex structure moduli).
    pub fn has_complex_structure_moduli(&self) -> bool {
        self.h21 >= 1
    }
}
/// AdS/CFT dictionary for bulk↔boundary field correspondence.
///
/// Encodes the map between bulk fields in AdS_{d+1} and boundary operators in CFT_d.
#[derive(Debug, Clone)]
pub struct AdSCFTDictionary {
    /// Dimension d of the boundary CFT.
    pub cft_dimension: u32,
    /// AdS radius of curvature L.
    pub ads_radius: f64,
    /// Newton's constant G_N (in units where ℏ = c = 1).
    pub newton_constant: f64,
    /// Number of colors N (for AdS5/CFT4: N = number of D3 branes).
    pub num_colors: u32,
    /// String coupling g_s.
    pub string_coupling: f64,
}
impl AdSCFTDictionary {
    /// Create an AdS/CFT dictionary instance.
    pub fn new(
        cft_dimension: u32,
        ads_radius: f64,
        newton_constant: f64,
        num_colors: u32,
        string_coupling: f64,
    ) -> Self {
        AdSCFTDictionary {
            cft_dimension,
            ads_radius,
            newton_constant,
            num_colors,
            string_coupling,
        }
    }
    /// Standard AdS5/CFT4 (Maldacena's original duality).
    pub fn ads5_cft4(num_d3_branes: u32) -> Self {
        let n = num_d3_branes as f64;
        let g_s = 1.0 / n.sqrt();
        let l = (4.0 * std::f64::consts::PI * g_s * n).sqrt().sqrt();
        AdSCFTDictionary {
            cft_dimension: 4,
            ads_radius: l,
            newton_constant: 1.0 / (n * n),
            num_colors: num_d3_branes,
            string_coupling: g_s,
        }
    }
    /// Conformal dimension from bulk mass: Δ = d/2 + √((d/2)² + m²L²).
    pub fn conformal_dimension(&self, mass_sq: f64) -> f64 {
        let d = self.cft_dimension as f64;
        let half_d = d / 2.0;
        half_d + (half_d * half_d + mass_sq * self.ads_radius * self.ads_radius).sqrt()
    }
    /// Breitenlohner-Freedman bound: m² ≥ -(d/2)² / L² (stability in AdS).
    pub fn bf_bound(&self) -> f64 {
        let d = self.cft_dimension as f64;
        -(d / 2.0).powi(2) / self.ads_radius.powi(2)
    }
    /// Central charge of the boundary CFT (schematic): c ~ N².
    pub fn central_charge(&self) -> f64 {
        (self.num_colors as f64).powi(2)
    }
    /// 't Hooft coupling λ = g_{YM}² N = 4π g_s N.
    pub fn t_hooft_coupling(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.string_coupling * self.num_colors as f64
    }
    /// Ryu-Takayanagi entanglement entropy: S_EE = Area/(4G_N).
    pub fn ryu_takayanagi_entropy(&self, minimal_surface_area: f64) -> f64 {
        minimal_surface_area / (4.0 * self.newton_constant)
    }
    /// Check if we are in the supergravity regime (λ >> 1, N >> 1).
    pub fn is_supergravity_regime(&self) -> bool {
        let lambda = self.t_hooft_coupling();
        lambda > 10.0 && self.num_colors > 10
    }
}
/// Extended Virasoro algebra tracker.
///
/// Tracks central charge, mode range, and level-N degeneracy.
#[derive(Debug, Clone)]
pub struct VirasoroAlgebraExt {
    /// Central charge c.
    pub central_charge: f64,
    /// Ground state conformal weight h.
    pub ground_weight: f64,
    /// Maximum oscillator level to track.
    pub max_level: u32,
}
impl VirasoroAlgebraExt {
    /// Create a Virasoro algebra extension tracker.
    pub fn new(central_charge: f64, ground_weight: f64, max_level: u32) -> Self {
        VirasoroAlgebraExt {
            central_charge,
            ground_weight,
            max_level,
        }
    }
    /// Partition function p(n): number of partitions of n (= degeneracy at level n).
    /// Computed via the recurrence p(n) = Σ_{k≠0} (-1)^{k+1} p(n - k(3k-1)/2).
    pub fn partition_number(&self, n: u32) -> u64 {
        let n = n as usize;
        let mut p = vec![0u64; n + 1];
        p[0] = 1;
        for i in 1..=n {
            let mut k: i64 = 1;
            loop {
                let penta1 = k * (3 * k - 1) / 2;
                let penta2 = k * (3 * k + 1) / 2;
                if penta1 as usize > i {
                    break;
                }
                let sign = if k % 2 == 1 { 1i64 } else { -1i64 };
                let idx1 = i - penta1 as usize;
                p[i] = (p[i] as i64 + sign * p[idx1] as i64) as u64;
                if penta2 as usize <= i {
                    let idx2 = i - penta2 as usize;
                    p[i] = (p[i] as i64 + sign * p[idx2] as i64) as u64;
                }
                k += 1;
            }
        }
        p[n]
    }
    /// Conformal weight of a level-n descendant: h + n.
    pub fn descendant_weight(&self, level: u32) -> f64 {
        self.ground_weight + level as f64
    }
    /// Character χ_h(q) = q^{h - c/24} Σ_n p(n) q^n (truncated to max_level).
    pub fn character_truncated(&self, q: f64) -> f64 {
        let prefactor = q.powf(self.ground_weight - self.central_charge / 24.0);
        let mut sum = 0.0;
        for n in 0..=self.max_level {
            sum += self.partition_number(n) as f64 * q.powi(n as i32);
        }
        prefactor * sum
    }
    /// Central term in the commutator \[L_m, L_{-m}\] = 2m L_0 + c/12 m(m²-1).
    pub fn central_term(&self, m: i64) -> f64 {
        self.central_charge / 12.0 * (m as f64) * ((m * m - 1) as f64)
    }
    /// Check the Kac determinant sign at level n (positive iff no null states below).
    /// Returns true if h > 0 and h ≥ h_{r,s} for all r,s with rs ≤ n (simplified check).
    pub fn kac_determinant_positive(&self, level: u32) -> bool {
        self.ground_weight > 0.0 && self.ground_weight >= level as f64 * 0.1
    }
}
/// String action with tension and worldsheet dimension.
#[derive(Debug, Clone)]
pub struct StringAction {
    /// String tension T = 1/(2πα').
    pub tension: f64,
    /// Worldsheet dimension (usually 2).
    pub worldsheet_dim: u32,
}
impl StringAction {
    /// Create a string action.
    pub fn new(tension: f64, worldsheet_dim: u32) -> Self {
        StringAction {
            tension,
            worldsheet_dim,
        }
    }
    /// Nambu-Goto action: S_NG = −T ∫ dA, where dA is the worldsheet area element.
    /// Returns the action value for a worldsheet of given area.
    pub fn nambu_goto_action(&self, area: f64) -> f64 {
        -self.tension * area
    }
    /// Polyakov action: S_P = −T/2 ∫ d²σ √h h^{ab} ∂_a X · ∂_b X.
    /// Returns action value for given metric determinant and kinetic term.
    pub fn polyakov_action(&self, metric_det: f64, kinetic_term: f64) -> f64 {
        -self.tension / 2.0 * metric_det.sqrt() * kinetic_term
    }
    /// Equations of motion: returns whether the worldsheet is conformally flat.
    pub fn equations_of_motion(&self) -> bool {
        self.worldsheet_dim == 2
    }
}
/// A 2D worldsheet represented as a flat rectangle \[0, 2π\] × \[0, T\].
#[derive(Debug, Clone)]
pub struct RectangularWorldsheet {
    /// Spatial extent (0 to sigma_length).
    pub sigma_length: f64,
    /// Temporal extent (0 to tau_length).
    pub tau_length: f64,
    /// Number of spatial grid points.
    pub n_sigma: usize,
    /// Number of temporal grid points.
    pub n_tau: usize,
}
impl RectangularWorldsheet {
    pub fn new(sigma_length: f64, tau_length: f64, n_sigma: usize, n_tau: usize) -> Self {
        RectangularWorldsheet {
            sigma_length,
            tau_length,
            n_sigma,
            n_tau,
        }
    }
    pub fn d_sigma(&self) -> f64 {
        self.sigma_length / self.n_sigma as f64
    }
    pub fn d_tau(&self) -> f64 {
        self.tau_length / self.n_tau as f64
    }
    pub fn area(&self) -> f64 {
        self.sigma_length * self.tau_length
    }
}
/// String theory compactification on an internal manifold.
#[derive(Debug, Clone)]
pub struct Compactification {
    /// Number of internal (compactified) dimensions.
    pub internal_dim: u32,
    /// Name of the compactification manifold.
    pub manifold: String,
}
impl Compactification {
    /// Create a compactification.
    pub fn new(internal_dim: u32, manifold: impl Into<String>) -> Self {
        Compactification {
            internal_dim,
            manifold: manifold.into(),
        }
    }
    /// Check if this is a Calabi-Yau compactification (dim must be even, >= 4).
    pub fn calabi_yau(&self) -> bool {
        self.internal_dim >= 4
            && self.internal_dim % 2 == 0
            && (self.manifold.contains("CY") || self.manifold.contains("Calabi"))
    }
    /// Rough estimate of number of generations from Euler characteristic χ.
    /// N_gen ≈ |χ|/2 (for CY3 compactification).
    pub fn num_generations_estimate(&self, euler_characteristic: i64) -> u64 {
        (euler_characteristic.unsigned_abs()) / 2
    }
    /// Moduli stabilisation: returns true if moduli are (formally) stabilised.
    /// (Simplified: returns true if internal_dim >= 6.)
    pub fn stabilize_moduli(&self) -> bool {
        self.internal_dim >= 6
    }
}
/// BPS (Bogomolny-Prasad-Sommerfeld) state.
///
/// A BPS state satisfies M = |Z| where Z is the central charge.
#[derive(Debug, Clone)]
pub struct BPS {
    /// Central charge magnitude (BPS mass bound).
    pub charge: f64,
    /// Actual mass of the state.
    pub mass: f64,
}
impl BPS {
    /// Create a BPS state.
    pub fn new(charge: f64, mass: f64) -> Self {
        BPS { charge, mass }
    }
    /// BPS bound is saturated when M = |Z| (mass equals charge).
    pub fn bps_bound_saturated(&self) -> bool {
        (self.mass - self.charge.abs()).abs() < 1e-12
    }
    /// BPS states are stable (protected by SUSY).
    pub fn is_stable(&self) -> bool {
        self.bps_bound_saturated() && self.charge.abs() > 0.0
    }
}
/// Types of string theory dualities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DualityType {
    /// S-duality: maps strong to weak coupling.
    S,
    /// T-duality: exchanges winding and momentum, relates R and α'/R.
    T,
    /// Mirror symmetry: exchanges Hodge numbers h^{1,1} and h^{2,1}.
    Mirror,
    /// M-theory duality: 11D origin of type IIA.
    Mtheory,
}
impl DualityType {
    /// Which theories are related by this duality.
    pub fn relates_theories(&self) -> (&'static str, &'static str) {
        match self {
            DualityType::S => ("Type I", "Heterotic SO(32)"),
            DualityType::T => ("Type IIA", "Type IIB"),
            DualityType::Mirror => ("CY_X", "CY_Y (mirror)"),
            DualityType::Mtheory => ("Type IIA (strong)", "M-theory"),
        }
    }
    /// Whether this duality maps coupling to inverse coupling (g → 1/g).
    pub fn maps_coupling_to_coupling(&self) -> bool {
        matches!(self, DualityType::S | DualityType::Mtheory)
    }
}
/// String scattering amplitude at genus g with n insertions.
#[derive(Debug, Clone)]
pub struct StringAmplitude {
    /// Genus of the worldsheet (0 = sphere/tree level, 1 = torus/1-loop, ...).
    pub genus: u32,
    /// Number of vertex operator insertions.
    pub num_insertions: u32,
}
impl StringAmplitude {
    /// Create a string amplitude.
    pub fn new(genus: u32, num_insertions: u32) -> Self {
        StringAmplitude {
            genus,
            num_insertions,
        }
    }
    /// Tree-level (genus 0) amplitude value (returns true if genus == 0).
    pub fn tree_level(&self) -> bool {
        self.genus == 0
    }
    /// Number of loop corrections (= genus).
    pub fn loop_corrections(&self) -> u32 {
        self.genus
    }
    /// S-matrix element (schematic): returns coupling power α'^genus.
    pub fn s_matrix(&self, alpha_prime: f64) -> f64 {
        alpha_prime.powi(self.genus as i32)
    }
}
/// Veneziano amplitude calculator.
///
/// Computes the bosonic open string 4-point amplitude.
#[derive(Debug, Clone)]
pub struct VenezianoAmplitudeCalc {
    /// Regge slope α'.
    pub alpha_prime: f64,
}
impl VenezianoAmplitudeCalc {
    /// Create a Veneziano amplitude calculator.
    pub fn new(alpha_prime: f64) -> Self {
        VenezianoAmplitudeCalc { alpha_prime }
    }
    /// Regge trajectory: α(s) = 1 + α' s / 2.
    pub fn regge_trajectory(&self, s: f64) -> f64 {
        1.0 + self.alpha_prime * s / 2.0
    }
    /// Veneziano amplitude via the Beta function approximation:
    /// A(s,t) ≈ B(-α(s), -α(t)) = Γ(-α(s)) Γ(-α(t)) / Γ(-α(s)-α(t)).
    /// Here we use the simplified form for positive s,t away from poles.
    pub fn amplitude_beta(&self, s: f64, t: f64) -> f64 {
        let as_ = self.regge_trajectory(s);
        let at = self.regge_trajectory(t);
        if as_ >= 0.0 || at >= 0.0 {
            return f64::INFINITY;
        }
        let lg_as = lgamma_approx(-as_);
        let lg_at = lgamma_approx(-at);
        let lg_sum = lgamma_approx(-as_ - at);
        (lg_as + lg_at - lg_sum).exp()
    }
    /// High-energy (s → ∞, fixed t) behavior: A ~ s^{α(t)} (power-law softening).
    pub fn high_energy_behavior(&self, s: f64, t: f64) -> f64 {
        let at = self.regge_trajectory(t);
        s.powf(at)
    }
    /// Low-energy (α's << 1) expansion: A(s,t) ≈ 1/(−s) + 1/(−t) + O(α').
    /// Recovers the point-particle field theory limit.
    pub fn low_energy_amplitude(&self, s: f64, t: f64) -> f64 {
        if s.abs() < 1e-12 || t.abs() < 1e-12 {
            return f64::INFINITY;
        }
        1.0 / (-s) + 1.0 / (-t)
    }
}
/// A 1D open string X^μ(σ) at fixed τ, in flat Minkowski spacetime.
/// Represented as a set of spacetime coordinates at each lattice point.
#[derive(Debug, Clone)]
pub struct StringConfiguration {
    /// Spacetime dimension.
    pub dim: usize,
    /// Number of lattice points.
    pub n_points: usize,
    /// Lattice spacing.
    pub d_sigma: f64,
    /// Field values X^μ_i, stored as \[n_points × dim\] row-major.
    pub coords: Vec<f64>,
}
impl StringConfiguration {
    /// Create a straight string along the x^1 axis.
    pub fn straight(dim: usize, n_points: usize, d_sigma: f64) -> Self {
        let mut coords = vec![0.0; n_points * dim];
        for i in 0..n_points {
            coords[i * dim + 1] = i as f64 * d_sigma;
        }
        StringConfiguration {
            dim,
            n_points,
            d_sigma,
            coords,
        }
    }
    /// Get X^μ at site i.
    pub fn get(&self, i: usize, mu: usize) -> f64 {
        self.coords[i * self.dim + mu]
    }
    /// Compute the Nambu-Goto action (in static gauge, 2D): S ∝ -T ∫ dσ |∂_σ X|.
    /// Here we use the flat-worldsheet approximation.
    pub fn nambu_goto_action(&self, tension: f64) -> f64 {
        let mut action = 0.0;
        for i in 0..(self.n_points - 1) {
            let mut grad_sq = 0.0;
            for mu in 1..self.dim {
                let dx = self.get(i + 1, mu) - self.get(i, mu);
                grad_sq += dx * dx / (self.d_sigma * self.d_sigma);
            }
            action += grad_sq.sqrt() * self.d_sigma;
        }
        -tension * action
    }
}
/// Superstring theory types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Superstring {
    /// Type I superstring theory.
    TypeI,
    /// Type IIA superstring theory.
    TypeIIA,
    /// Type IIB superstring theory.
    TypeIIB,
    /// Heterotic SO(32) superstring theory.
    HeteroticSO32,
    /// Heterotic E8×E8 superstring theory.
    HeteroticE8E8,
}
impl Superstring {
    /// Critical spacetime dimension for each superstring theory (all = 10).
    pub fn critical_dimension(&self) -> u32 {
        10
    }
    /// Whether the theory is supersymmetric.
    pub fn is_supersymmetric(&self) -> bool {
        true
    }
    /// Brief description of the moduli space.
    pub fn moduli_space(&self) -> &'static str {
        match self {
            Superstring::TypeI => "SO(32) gauge group, 1 SUSY in 10D",
            Superstring::TypeIIA => "Non-chiral, U(1) gauge group",
            Superstring::TypeIIB => "Chiral, SL(2,Z) duality group",
            Superstring::HeteroticSO32 => "SO(32) heterotic, rank-16 gauge group",
            Superstring::HeteroticE8E8 => "E8×E8 heterotic, rank-16 gauge group",
        }
    }
}
/// Some known Calabi-Yau threefold Hodge numbers (h^{1,1}, h^{2,1}).
pub struct KnownCY3 {
    pub name: &'static str,
    pub h11: u64,
    pub h21: u64,
}
impl KnownCY3 {
    pub fn quintic() -> Self {
        KnownCY3 {
            name: "Quintic in P^4",
            h11: 1,
            h21: 101,
        }
    }
    pub fn mirror_quintic() -> Self {
        KnownCY3 {
            name: "Mirror quintic",
            h11: 101,
            h21: 1,
        }
    }
    pub fn euler_characteristic(&self) -> i64 {
        cy3_euler_characteristic(self.h11 as i64, self.h21 as i64)
    }
}
/// Topological string theory (A-model and/or B-model).
#[derive(Debug, Clone)]
pub struct TopologicalString {
    /// Whether A-model is active.
    pub a_model: bool,
    /// Whether B-model is active.
    pub b_model: bool,
}
impl TopologicalString {
    /// Create a topological string theory model.
    pub fn new(a_model: bool, b_model: bool) -> Self {
        TopologicalString { a_model, b_model }
    }
    /// Mirror symmetry exchanges A-model and B-model.
    /// Returns true if both models are present (mirror pair).
    pub fn mirror_symmetry(&self) -> bool {
        self.a_model && self.b_model
    }
    /// Gromov-Witten invariants: computed by the A-model.
    /// Returns true if A-model is active.
    pub fn gromov_witten(&self) -> bool {
        self.a_model
    }
    /// Kodaira-Spencer theory: governs B-model deformations of complex structure.
    /// Returns true if B-model is active.
    pub fn kodaira_spencer(&self) -> bool {
        self.b_model
    }
}
/// AdS/CFT correspondence parameters.
///
/// Relates an AdS_{d+1} bulk gravity theory to a CFT_d on the boundary.
#[derive(Debug, Clone)]
pub struct AdSCFT {
    /// AdS radius of curvature L.
    pub ads_radius: f64,
    /// Central charge c of the boundary CFT.
    pub cft_central_charge: f64,
}
impl AdSCFT {
    /// Create an AdS/CFT model.
    pub fn new(ads_radius: f64, cft_central_charge: f64) -> Self {
        AdSCFT {
            ads_radius,
            cft_central_charge,
        }
    }
    /// Holographic dictionary: maps bulk fields to boundary operators.
    /// Returns the conformal dimension Δ for a bulk scalar of mass m.
    /// Δ = d/2 + sqrt((d/2)² + m²·L²) for AdS_{d+1}/CFT_d.
    pub fn holographic_dictionary(&self, mass_sq: f64, d: f64) -> f64 {
        let half_d = d / 2.0;
        half_d + (half_d * half_d + mass_sq * self.ads_radius.powi(2)).sqrt()
    }
    /// Bulk-boundary correspondence: Bekenstein-Hawking entropy.
    /// S_BH = A/(4·G_N) ≈ c·L²/(12·G_N) (schematic).
    pub fn bulk_boundary_correspondence(&self) -> f64 {
        self.cft_central_charge * self.ads_radius.powi(2) / 12.0
    }
}
/// M-theory: the 11-dimensional theory underlying all superstring theories.
#[derive(Debug, Clone)]
pub struct MTheory {
    /// Spacetime dimension (should be 11).
    pub dimension: u32,
}
impl MTheory {
    /// Create an M-theory model.
    pub fn new(dimension: u32) -> Self {
        MTheory { dimension }
    }
    /// M2 and M5 branes: the fundamental BPS objects of M-theory.
    /// Returns (M2_tension, M5_tension) in units of M_Planck = 1.
    pub fn m2_m5_branes(&self) -> (f64, f64) {
        let m2 = 1.0 / (2.0 * std::f64::consts::PI).powi(2);
        let m5 = 1.0 / (2.0 * std::f64::consts::PI).powi(5);
        (m2, m5)
    }
    /// 11-dimensional supergravity: the low-energy limit of M-theory.
    /// Returns true if dimension == 11.
    pub fn eleven_dim_supergravity(&self) -> bool {
        self.dimension == 11
    }
    /// F-theory: 12-dimensional formulation (adds 2 extra dimensions).
    /// Returns the F-theory dimension.
    pub fn f_theory(&self) -> u32 {
        self.dimension + 1
    }
}
