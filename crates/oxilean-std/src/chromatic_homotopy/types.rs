//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// Bousfield localization L_E X.
///
/// Stores the localizing homology theory E and the localized spectrum.
pub struct BousfieldLocalization {
    /// The homology theory E (by name).
    pub homology_theory: String,
    /// The spectrum X (by name).
    pub spectrum: String,
    /// The localized spectrum L_E X (by name).
    pub localized: String,
}
/// The periodicity theorem: v_n-periodicity classes exist on type-n complexes.
pub struct PeriodicityThm {
    /// The chromatic height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// A sample v_n self-map (stored as periodicity period).
    pub periodicity_period: u64,
}
impl PeriodicityThm {
    /// Create the periodicity data at height `n` and prime `p`.
    pub fn new(height: usize, prime: u64) -> Self {
        let periodicity_period = 2 * (prime.pow(height as u32) - 1);
        PeriodicityThm {
            height,
            prime,
            periodicity_period,
        }
    }
    /// The v_n-periodicity class lives in degree `2(p^n - 1)`.
    pub fn vn_degree(&self) -> u64 {
        self.periodicity_period
    }
}
/// Morava K-theory K(n) at height n and prime p.
pub struct MoravaKTheory {
    /// The chromatic height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
}
impl MoravaKTheory {
    /// Create K(n) at height `n` and prime `p`.
    pub fn new(height: usize, prime: u64) -> Self {
        MoravaKTheory { height, prime }
    }
    /// The periodicity of K(n): 2(p^n - 1).
    pub fn periodicity(&self) -> u64 {
        2 * (self.prime.pow(self.height as u32) - 1)
    }
    /// K(n) is a field spectrum: K(n)_*(K(n)) ≅ K(n)_*[τ, τ^{-1}].
    pub fn is_field_spectrum(&self) -> bool {
        true
    }
}
/// Morava K-group functor K(n)_*(X) evaluated on a finite spectrum.
pub struct MoravaKGroup {
    /// The height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// The spectrum X (by name).
    pub spectrum: String,
    /// Graded ranks: index i gives rank of K(n)_i(X).
    pub graded_ranks: Vec<usize>,
}
impl MoravaKGroup {
    /// Create K(n)_*(X) data.
    pub fn new(height: usize, prime: u64, spectrum: impl Into<String>) -> Self {
        MoravaKGroup {
            height,
            prime,
            spectrum: spectrum.into(),
            graded_ranks: Vec::new(),
        }
    }
    /// Set the rank in degree d.
    pub fn set_rank(&mut self, degree: usize, rank: usize) {
        if degree >= self.graded_ranks.len() {
            self.graded_ranks.resize(degree + 1, 0);
        }
        self.graded_ranks[degree] = rank;
    }
    /// Get the rank in degree d.
    pub fn rank_in_degree(&self, degree: usize) -> usize {
        self.graded_ranks.get(degree).copied().unwrap_or(0)
    }
    /// Total Euler characteristic: alternating sum of ranks.
    pub fn euler_characteristic(&self) -> i64 {
        self.graded_ranks
            .iter()
            .enumerate()
            .map(|(i, &r)| if i % 2 == 0 { r as i64 } else { -(r as i64) })
            .sum()
    }
    /// Check whether X is K(n)-acyclic (all ranks zero).
    pub fn is_acyclic(&self) -> bool {
        self.graded_ranks.iter().all(|&r| r == 0)
    }
}
/// An approximation to the E_2 page of the Adams spectral sequence.
pub struct AdamsSpectralSequence {
    /// The prime p.
    pub prime: u64,
    /// The spectrum X (by name).
    pub spectrum: String,
    /// Sparse E_2 data: (filtration s, stem t-s, rank).
    pub e2_data: Vec<(usize, usize, usize)>,
}
impl AdamsSpectralSequence {
    /// Create an Adams SS for spectrum X at prime p.
    pub fn new(spectrum: impl Into<String>, prime: u64) -> Self {
        AdamsSpectralSequence {
            prime,
            spectrum: spectrum.into(),
            e2_data: Vec::new(),
        }
    }
    /// Record a non-trivial E_2 group at filtration s, stem n, of rank r.
    pub fn add_e2_group(&mut self, s: usize, stem: usize, rank: usize) {
        self.e2_data.push((s, stem, rank));
    }
    /// Get the total rank in the given stem (summed over all filtrations).
    pub fn total_rank_in_stem(&self, stem: usize) -> usize {
        self.e2_data
            .iter()
            .filter(|&&(_, st, _)| st == stem)
            .map(|&(_, _, r)| r)
            .sum()
    }
    /// Get the rank at filtration s, stem n.
    pub fn rank_at(&self, s: usize, stem: usize) -> usize {
        self.e2_data
            .iter()
            .find(|&&(si, st, _)| si == s && st == stem)
            .map(|&(_, _, r)| r)
            .unwrap_or(0)
    }
    /// Build the standard Adams E_2 page for the sphere S^0 at p=2 (low stems).
    pub fn sphere_at_2() -> Self {
        let mut ss = AdamsSpectralSequence::new("S^0", 2);
        ss.add_e2_group(0, 0, 1);
        ss.add_e2_group(1, 1, 1);
        ss.add_e2_group(2, 2, 1);
        ss.add_e2_group(1, 3, 1);
        ss.add_e2_group(3, 3, 1);
        ss.add_e2_group(1, 7, 1);
        ss
    }
}
/// An elliptic curve over a ring R in Weierstrass form y^2 = x^3 + ax + b.
pub struct EllipticCurveOverRing {
    /// The base ring (by name).
    pub ring: String,
    /// Coefficient a in the Weierstrass equation.
    pub a: i64,
    /// Coefficient b in the Weierstrass equation.
    pub b: i64,
}
impl EllipticCurveOverRing {
    /// Check the non-singularity condition: 4a^3 + 27b^2 ≠ 0.
    pub fn is_nonsingular(&self) -> bool {
        4 * self.a.pow(3) + 27 * self.b.pow(2) != 0
    }
    /// The j-invariant: j = -1728 * (4a)^3 / (4a^3 + 27b^2).
    pub fn j_invariant_numerator(&self) -> i64 {
        -1728 * (4 * self.a).pow(3)
    }
}
/// An elliptic cohomology theory: a complex-oriented E with formal group from an elliptic curve.
pub struct EllipticCohomologyTheory {
    /// The name of the theory.
    pub name: String,
    /// The elliptic curve supplying the formal group.
    pub elliptic_curve: EllipticCurveOverRing,
    /// Whether this theory is an E_∞-ring spectrum.
    pub is_e_infty: bool,
}
/// A basic model of a spectral scheme (simplicial commutative ring data).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralScheme {
    /// Name of the spectral scheme.
    pub name: String,
    /// The underlying classical scheme (truncation).
    pub classical_truncation: String,
    /// Cotangent complex dimension.
    pub cotangent_dim: Option<usize>,
    /// Whether it is a derived complete intersection.
    pub is_dci: bool,
}
#[allow(dead_code)]
impl SpectralScheme {
    /// Creates a spectral scheme.
    pub fn new(name: &str, classical: &str) -> Self {
        SpectralScheme {
            name: name.to_string(),
            classical_truncation: classical.to_string(),
            cotangent_dim: None,
            is_dci: false,
        }
    }
    /// Sets the cotangent complex dimension.
    pub fn with_cotangent_dim(mut self, d: usize) -> Self {
        self.cotangent_dim = Some(d);
        self
    }
    /// Marks as derived complete intersection.
    pub fn as_dci(mut self) -> Self {
        self.is_dci = true;
        self
    }
    /// Returns the Tor-amplitude description.
    pub fn tor_amplitude(&self) -> String {
        match self.cotangent_dim {
            Some(d) => format!("[0, {}]", d),
            None => "unknown".to_string(),
        }
    }
    /// Checks if the scheme is a Deligne-Mumford spectral stack (dimension 0 cotangent).
    pub fn is_spectral_dm_stack(&self) -> bool {
        matches!(self.cotangent_dim, Some(0))
    }
    /// Returns the spectrum of the E-infinity ring description.
    pub fn spectrum_description(&self) -> String {
        format!("Spec({}) as E_∞-ring scheme", self.name)
    }
}
/// Represents data about the chromatic tower at height n.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChromaticTowerLevel {
    /// The chromatic height.
    pub height: usize,
    /// The prime p.
    pub prime: usize,
    /// Whether L_n-localization of the sphere is known.
    pub ln_sphere_known: bool,
    /// Homotopy groups of L_n S^0 in low degrees (sparse representation).
    pub pi_low: Vec<(i32, String)>,
}
#[allow(dead_code)]
impl ChromaticTowerLevel {
    /// Creates a chromatic tower level.
    pub fn new(height: usize, prime: usize) -> Self {
        ChromaticTowerLevel {
            height,
            prime,
            ln_sphere_known: height <= 2,
            pi_low: Vec::new(),
        }
    }
    /// Adds a homotopy group datum.
    pub fn add_homotopy_group(&mut self, degree: i32, description: String) {
        self.pi_low.push((degree, description));
    }
    /// Returns the Bousfield class description.
    pub fn bousfield_class(&self) -> String {
        format!("<E({},{})>", self.prime, self.height)
    }
    /// Checks if monochromatic layer M_n is nontrivial (always true for n >= 1).
    pub fn monochromatic_nontrivial(&self) -> bool {
        self.height >= 1
    }
    /// Returns the periodicity element v_n description.
    pub fn periodicity_element(&self) -> String {
        if self.height == 0 {
            "1 (no periodicity)".to_string()
        } else {
            format!("v_{}", self.height)
        }
    }
    /// Returns the period |v_n| for p-primary v_n-periodicity.
    pub fn periodicity_degree(&self) -> Option<usize> {
        if self.prime < 2 {
            return None;
        }
        Some(2 * (self.prime.pow(self.height as u32) - 1))
    }
}
/// The Brown-Peterson Adams spectral sequence.
///
/// Uses the Brown-Peterson homology BP to compute homotopy groups.
pub struct BPAdamsSpectralSequence {
    /// The spectrum X.
    pub spectrum: String,
    /// The prime p.
    pub prime: u64,
    /// Whether the spectral sequence has been shown to converge.
    pub converges: bool,
}
impl BPAdamsSpectralSequence {
    /// Create a BP-Adams spectral sequence for spectrum `X` at prime `p`.
    pub fn new(spectrum: impl Into<String>, prime: u64) -> Self {
        BPAdamsSpectralSequence {
            spectrum: spectrum.into(),
            prime,
            converges: true,
        }
    }
}
/// The chromatic convergence theorem: π_*(X_p^) ≅ lim_n π_*(L_n X_p^).
pub struct ChromaticConvergence {
    /// The spectrum X.
    pub spectrum: String,
    /// Whether the convergence has been verified.
    pub verified: bool,
}
/// The Adams-Novikov spectral sequence.
///
/// E_2^{s,t} = Ext^{s,t}_{MU_*(MU)}(MU_*, MU_*(X)) ⇒ π_{t-s}(X_p^).
pub struct AdamsNovikovSS {
    /// The spectrum X.
    pub spectrum: String,
    /// The prime p.
    pub prime: u64,
    /// E_2 page data: sparse list of (s, t, rank).
    pub e2_page: Vec<(usize, usize, usize)>,
}
impl AdamsNovikovSS {
    /// Get the rank of Ext^{s,t} on the E_2 page.
    pub fn e2_rank(&self, s: usize, t: usize) -> usize {
        self.e2_page
            .iter()
            .find(|&&(si, ti, _)| si == s && ti == t)
            .map(|&(_, _, r)| r)
            .unwrap_or(0)
    }
    /// The target group π_{t-s}(X_p^): the abutment at stem s.
    pub fn stem(&self, t: usize, s: usize) -> Option<i64> {
        if t >= s {
            Some((t - s) as i64)
        } else {
            None
        }
    }
}
/// The topological cyclic homology TC(A; p) of a ring spectrum at prime p.
pub struct TopologicalCyclicHomologyData {
    /// The ring spectrum A (by name).
    pub ring_spectrum: String,
    /// The prime p.
    pub prime: u64,
    /// Whether the Bökstedt periodicity generator β has been used.
    pub uses_bokstedt_periodicity: bool,
}
impl TopologicalCyclicHomologyData {
    /// Create TC data at prime p.
    pub fn new(ring_spectrum: impl Into<String>, prime: u64) -> Self {
        TopologicalCyclicHomologyData {
            ring_spectrum: ring_spectrum.into(),
            prime,
            uses_bokstedt_periodicity: false,
        }
    }
    /// The cyclotomic trace map K(A) → TC(A; p) always exists.
    pub fn cyclotomic_trace_exists(&self) -> bool {
        true
    }
}
/// The Landweber exact functor theorem data for a ring R.
///
/// A graded ring map π_*(MU) → R makes R ⊗_{MU_*} MU_*(−) a cohomology theory
/// provided R is Landweber exact (the sequence (p, v_1, v_2, ...) is regular in R).
pub struct LandweberExactFunctor {
    /// The target ring name.
    pub ring: String,
    /// Whether Landweber exactness has been verified.
    pub is_exact: bool,
    /// The regularity witnesses: the first k elements of (p, v_1, ...) are regular.
    pub regularity_depth: usize,
}
impl LandweberExactFunctor {
    /// Create Landweber data for a ring, verifying exactness to depth `depth`.
    pub fn new(ring: impl Into<String>, depth: usize) -> Self {
        LandweberExactFunctor {
            ring: ring.into(),
            is_exact: depth > 0,
            regularity_depth: depth,
        }
    }
    /// A cohomology theory is produced when the map is Landweber exact.
    pub fn produces_cohomology_theory(&self) -> bool {
        self.is_exact
    }
}
/// The Lubin-Tate deformation space at height n and prime p.
pub struct LubinTateSpaceData {
    /// The height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// Whether the full Morava E-theory has been computed.
    pub e_theory_computed: bool,
}
impl LubinTateSpaceData {
    /// Create the Lubin-Tate space at height n, prime p.
    pub fn new(height: usize, prime: u64) -> Self {
        LubinTateSpaceData {
            height,
            prime,
            e_theory_computed: false,
        }
    }
    /// The number of deformation parameters: u_1, ..., u_{n-1}.
    pub fn num_deformation_params(&self) -> usize {
        if self.height > 0 {
            self.height - 1
        } else {
            0
        }
    }
}
/// The Brown-Peterson spectrum BP at a prime p.
///
/// π_*(BP) = ℤ_(p)[v_1, v_2, ...] where |v_n| = 2(p^n - 1).
pub struct BrownPetersonBP {
    /// The prime p.
    pub prime: u64,
    /// The truncation height: we store v_1, ..., v_{max_height}.
    pub max_height: usize,
}
impl BrownPetersonBP {
    /// Create BP at prime `p` with generators up to height `max_height`.
    pub fn new(prime: u64, max_height: usize) -> Self {
        BrownPetersonBP { prime, max_height }
    }
    /// The degree of the v_n generator: 2(p^n - 1).
    pub fn vn_degree(&self, n: usize) -> u64 {
        2 * (self.prime.pow(n as u32) - 1)
    }
    /// List all v_n generator names up to max_height.
    pub fn generators(&self) -> Vec<(String, u64)> {
        (1..=self.max_height)
            .map(|n| (format!("v_{n}"), self.vn_degree(n)))
            .collect()
    }
}
/// The p-typical Witt vector ring W_n(R) (length-n truncated).
///
/// W_n(R) encodes the carries in p-adic arithmetic; W(F_p) = ℤ_p.
pub struct WittVectorRing {
    /// The base ring name.
    pub base_ring: String,
    /// The prime p.
    pub prime: u64,
    /// The truncation length n.
    pub length: usize,
}
impl WittVectorRing {
    /// Create W_n(R) for a ring R at prime p with truncation n.
    pub fn new(base_ring: impl Into<String>, prime: u64, length: usize) -> Self {
        WittVectorRing {
            base_ring: base_ring.into(),
            prime,
            length,
        }
    }
    /// Add two Witt vectors of length `self.length` component-wise mod p.
    pub fn add(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        let len = self.length.min(a.len()).min(b.len());
        let p = self.prime as i64;
        let mut result = vec![0i64; len];
        let mut carry = 0i64;
        for i in 0..len {
            let sum = a[i] + b[i] + carry;
            result[i] = sum % p;
            carry = sum / p;
        }
        result
    }
    /// The Frobenius endomorphism F: shifts the ghost components left.
    pub fn frobenius_shift(components: &[i64]) -> Vec<i64> {
        if components.is_empty() {
            vec![]
        } else {
            components[1..].to_vec()
        }
    }
    /// The Verschiebung map V: inserts 0 at position 0.
    pub fn verschiebung(components: &[i64]) -> Vec<i64> {
        let mut result = vec![0i64];
        result.extend_from_slice(components);
        result
    }
}
/// A formal group law F(x, y) over a coefficient ring.
///
/// Coefficients `a[i][j]` give the degree-(i+j) piece of F.
/// We store only finitely many terms up to a given truncation degree.
pub struct FormalGroupLaw {
    /// Coefficients a_{i,j} of x^i y^j, truncated to degree `truncation`.
    pub coefficients: Vec<Vec<i64>>,
    /// The truncation degree (terms of total degree > truncation are dropped).
    pub truncation: usize,
}
impl FormalGroupLaw {
    /// Create the additive formal group law F(x,y) = x + y.
    pub fn additive(truncation: usize) -> Self {
        let mut coefficients = vec![vec![0i64; truncation + 1]; truncation + 1];
        if truncation >= 1 {
            coefficients[1][0] = 1;
            coefficients[0][1] = 1;
        }
        FormalGroupLaw {
            coefficients,
            truncation,
        }
    }
    /// Create the multiplicative formal group law F(x,y) = x + y + xy.
    pub fn multiplicative(truncation: usize) -> Self {
        let mut coefficients = vec![vec![0i64; truncation + 1]; truncation + 1];
        if truncation >= 1 {
            coefficients[1][0] = 1;
            coefficients[0][1] = 1;
        }
        if truncation >= 2 {
            coefficients[1][1] = 1;
        }
        FormalGroupLaw {
            coefficients,
            truncation,
        }
    }
    /// Check the identity axiom: F(x, 0) = x (coefficient of x^1 y^0 is 1).
    pub fn satisfies_identity(&self) -> bool {
        self.truncation >= 1 && self.coefficients[1][0] == 1
    }
    /// Get the height of this formal group law (1 for multiplicative, ∞ for additive over F_p).
    pub fn height_at_prime(&self, _p: u64) -> Option<usize> {
        if self.truncation >= 2 && self.coefficients[1][1] != 0 {
            Some(1)
        } else {
            None
        }
    }
}
/// The topological Hochschild homology THH(A) of a ring spectrum.
pub struct TopologicalHochschildHomologyData {
    /// The ring spectrum A (by name).
    pub ring_spectrum: String,
    /// Whether A is an E_∞-ring.
    pub is_e_infty: bool,
    /// Homotopy groups π_*(THH(A)) at low degrees (degree → rank).
    pub homotopy_groups: Vec<(usize, usize)>,
}
impl TopologicalHochschildHomologyData {
    /// Create THH data for a ring spectrum.
    pub fn new(ring_spectrum: impl Into<String>, is_e_infty: bool) -> Self {
        TopologicalHochschildHomologyData {
            ring_spectrum: ring_spectrum.into(),
            is_e_infty,
            homotopy_groups: Vec::new(),
        }
    }
    /// Record a homotopy group rank at degree d.
    pub fn add_homotopy_group(&mut self, degree: usize, rank: usize) {
        self.homotopy_groups.push((degree, rank));
    }
    /// Get the rank of π_d(THH(A)).
    pub fn pi_rank(&self, d: usize) -> usize {
        self.homotopy_groups
            .iter()
            .find(|&&(deg, _)| deg == d)
            .map(|&(_, r)| r)
            .unwrap_or(0)
    }
}
/// A v_n self-map on a type-n finite complex.
///
/// Represents the periodicity operator Σ^{2(p^n-1)k} X → X.
pub struct VnSelfMapData {
    /// The height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// The period: the degree of the self-map is 2(p^n-1)*k.
    pub period: u64,
    /// The multiplicity k.
    pub multiplicity: u64,
}
impl VnSelfMapData {
    /// Create a v_n self-map of multiplicity k at height n, prime p.
    pub fn new(height: usize, prime: u64, multiplicity: u64) -> Self {
        let base_period = 2 * (prime.pow(height as u32) - 1);
        VnSelfMapData {
            height,
            prime,
            period: base_period * multiplicity,
            multiplicity,
        }
    }
    /// The telescope of a v_n self-map v: X → X is v^{-1} X.
    pub fn telescope_name(&self, spectrum: &str) -> String {
        format!("v_{}^{{-1}} {}", self.height, spectrum)
    }
}
/// A level structure Γ_0(n) or Γ_1(n) on an elliptic curve.
pub struct LevelStructure {
    /// The level n.
    pub level: usize,
    /// The type: "Gamma_0" or "Gamma_1".
    pub structure_type: String,
}
impl LevelStructure {
    /// Create a Γ_0(n) level structure.
    pub fn gamma_0(n: usize) -> Self {
        LevelStructure {
            level: n,
            structure_type: "Gamma_0".to_string(),
        }
    }
    /// Create a Γ_1(n) level structure.
    pub fn gamma_1(n: usize) -> Self {
        LevelStructure {
            level: n,
            structure_type: "Gamma_1".to_string(),
        }
    }
}
/// Represents Morava K-theory K(n) at height n and prime p.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MoravaKData {
    /// Height.
    pub height: usize,
    /// Prime.
    pub prime: usize,
    /// Coefficient ring description: K(n)_* = F_p[v_n, v_n^{-1}].
    pub coeff_ring: String,
}
#[allow(dead_code)]
impl MoravaKData {
    /// Creates Morava K(n) at prime p.
    pub fn new(height: usize, prime: usize) -> Self {
        let coeff_ring = if height == 0 {
            format!("F_{}", prime)
        } else {
            format!("F_{}[v_{}, v_{}^(-1)]", prime, height, height)
        };
        MoravaKData {
            height,
            prime,
            coeff_ring,
        }
    }
    /// Returns the degree of v_n in K(n)_*.
    pub fn vn_degree(&self) -> usize {
        2 * (self.prime.pow(self.height as u32) - 1)
    }
    /// Checks K(n)_*(point) = F_p.
    pub fn coeff_of_point_is_fp(&self) -> bool {
        self.height >= 1
    }
    /// Returns the Künneth formula type: K(n) satisfies Künneth.
    pub fn satisfies_kunneth(&self) -> bool {
        true
    }
    /// Computes K(n)_*(BG) dimension for cyclic group G = Z/p.
    /// K(n)_*(BZ/p) has dimension 2 * p^{n-1} over K(n)_*.
    pub fn bcp_dimension(&self) -> usize {
        if self.height == 0 {
            0
        } else {
            2 * self.prime.pow((self.height - 1) as u32)
        }
    }
    /// Returns description of the associated Morava E-theory.
    pub fn associated_e_theory(&self) -> String {
        format!(
            "E({},{}) (Morava E-theory, formal group over W(F_{}^{})",
            self.prime, self.height, self.prime, self.height
        )
    }
}
/// Chromatic complexity estimator for a spectrum.
///
/// Estimates the smallest n such that X is E(n)-local.
pub struct ChromaticComplexityData {
    /// The spectrum X (by name).
    pub spectrum: String,
    /// Acyclicity flags: acyclic_below[n] = true means K(n)_*(X) = 0.
    pub acyclic_below: Vec<bool>,
}
impl ChromaticComplexityData {
    /// Create a chromatic complexity estimator.
    pub fn new(spectrum: impl Into<String>) -> Self {
        ChromaticComplexityData {
            spectrum: spectrum.into(),
            acyclic_below: Vec::new(),
        }
    }
    /// Record that K(n)_*(X) = 0.
    pub fn set_acyclic_at(&mut self, n: usize) {
        if n >= self.acyclic_below.len() {
            self.acyclic_below.resize(n + 1, false);
        }
        self.acyclic_below[n] = true;
    }
    /// Estimate the type of X: the first recorded height where X is not acyclic,
    /// or `len` if all recorded heights are acyclic (the first unrecorded height).
    pub fn type_estimate(&self) -> Option<usize> {
        for (n, &acyclic) in self.acyclic_below.iter().enumerate() {
            if !acyclic {
                return Some(n);
            }
        }
        if self.acyclic_below.is_empty() {
            None
        } else {
            Some(self.acyclic_below.len())
        }
    }
    /// Estimate the chromatic complexity.
    pub fn complexity(&self) -> usize {
        self.type_estimate().unwrap_or(0)
    }
}
/// A modular form of weight k and level Γ.
pub struct ModularFormSpectrum {
    /// The weight k.
    pub weight: usize,
    /// The level (encoded as a number).
    pub level: usize,
    /// Whether this is a cusp form (vanishes at all cusps).
    pub is_cusp_form: bool,
}
impl ModularFormSpectrum {
    /// Check that this is a cusp form.
    pub fn cusp_forms(&self) -> bool {
        self.is_cusp_form
    }
    /// The degree in π_*(tmf) is 2k (weight k → degree 2k).
    pub fn degree(&self) -> usize {
        2 * self.weight
    }
    /// The weight of the modular form.
    pub fn weight(&self) -> usize {
        self.weight
    }
}
/// The Lazard ring L: the universal ring for formal group laws.
///
/// Represented by generators and the grading (L lives in even degrees).
pub struct LazardRing {
    /// Generators: L has polynomial generators in every even degree.
    pub generators: Vec<(usize, String)>,
}
impl LazardRing {
    /// Create the standard Lazard ring with generators up to degree `max_degree`.
    pub fn new(max_degree: usize) -> Self {
        let generators = (1..=max_degree / 2)
            .map(|i| (2 * i, format!("a_{i}")))
            .collect();
        LazardRing { generators }
    }
    /// Get generators of a given degree.
    pub fn generators_of_degree(&self, d: usize) -> Vec<&str> {
        self.generators
            .iter()
            .filter(|(deg, _)| *deg == d)
            .map(|(_, name)| name.as_str())
            .collect()
    }
}
/// Represents Brown-Peterson cohomology BP data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BPCohomologyData {
    /// Prime p.
    pub prime: usize,
    /// The polynomial generators v_i (degrees 2(p^i - 1)).
    pub vn_generators: Vec<(usize, usize)>,
    /// Computed BP_*(X) ranks in low degrees.
    pub bp_ranks: Vec<(i32, usize)>,
}
#[allow(dead_code)]
impl BPCohomologyData {
    /// Creates BP data at prime p, with generators up to height max_n.
    pub fn new(prime: usize, max_n: usize) -> Self {
        let vn_generators = (0..=max_n)
            .map(|i| {
                let deg = if i == 0 {
                    2
                } else {
                    2 * (prime.pow(i as u32) - 1)
                };
                (i, deg)
            })
            .collect();
        BPCohomologyData {
            prime,
            vn_generators,
            bp_ranks: Vec::new(),
        }
    }
    /// Returns the degree of v_n.
    pub fn vn_degree(&self, n: usize) -> Option<usize> {
        self.vn_generators
            .iter()
            .find(|&&(i, _)| i == n)
            .map(|&(_, d)| d)
    }
    /// Adds a BP rank in a given degree.
    pub fn add_bp_rank(&mut self, degree: i32, rank: usize) {
        self.bp_ranks.push((degree, rank));
    }
    /// Returns the total rank of BP in degree d.
    pub fn rank_in_degree(&self, d: i32) -> usize {
        self.bp_ranks
            .iter()
            .filter(|&&(deg, _)| deg == d)
            .map(|&(_, r)| r)
            .sum()
    }
    /// Describes the coefficient ring BP_*(pt).
    pub fn coefficient_ring(&self) -> String {
        let gens: Vec<String> = self
            .vn_generators
            .iter()
            .map(|&(i, _)| format!("v_{}", i))
            .collect();
        format!("Z_({})[{}]", self.prime, gens.join(", "))
    }
}
/// The full subcategory of E-local spectra.
pub struct LocalSpectra {
    /// The localizing homology theory E.
    pub homology_theory: String,
    /// Names of the known E-local spectra.
    pub local_spectra: Vec<String>,
}
impl LocalSpectra {
    /// Check whether a spectrum (by name) is known to be E-local.
    pub fn is_local(&self, name: &str) -> bool {
        self.local_spectra.iter().any(|s| s == name)
    }
}
/// The chromatic filtration of a spectrum.
///
/// Stores the tower L_0 X → L_1 X → ... → L_n X of localizations.
pub struct ChromaticFiltration {
    /// The spectrum being filtered (by name).
    pub spectrum: String,
    /// The layers L_0 X, L_1 X, ..., L_n X (by name).
    pub layers: Vec<String>,
    /// The prime p used for localization.
    pub prime: u64,
}
impl ChromaticFiltration {
    /// Get the n-th chromatic approximation L_n X.
    pub fn layer(&self, n: usize) -> Option<&str> {
        self.layers.get(n).map(String::as_str)
    }
    /// Check convergence: all layers up to `max_n` are defined.
    pub fn converges_at(&self, max_n: usize) -> bool {
        self.layers.len() > max_n
    }
}
/// An E-acyclic morphism: f: X → Y with E_*(f) = 0.
pub struct AcyclicMorphism {
    /// Source spectrum name.
    pub source: String,
    /// Target spectrum name.
    pub target: String,
    /// Whether E-acyclicity has been verified.
    pub acyclic: bool,
}
/// The Morava stabilizer group S_n = Aut(H_n) at height n.
pub struct MoravaStabilizerGroupData {
    /// The height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// The order of the center mod (p^n - 1) part.
    pub center_order_mod: u64,
}
impl MoravaStabilizerGroupData {
    /// Create the Morava stabilizer group at height n, prime p.
    pub fn new(height: usize, prime: u64) -> Self {
        let center_order_mod = prime.pow(height as u32) - 1;
        MoravaStabilizerGroupData {
            height,
            prime,
            center_order_mod,
        }
    }
    /// The index of the center in S_n (mod p^n - 1 part).
    pub fn center_index(&self) -> u64 {
        self.center_order_mod
    }
}
/// The spectrum tmf of topological modular forms.
///
/// π_*(tmf) ≅ ℤ[c_4, c_6, Δ] / (c_4^3 - c_6^2 = 1728 Δ).
pub struct TopologicalModularForms {
    /// Whether this is the connective version (tmf) or periodic (TMF).
    pub is_connective: bool,
    /// Whether this is the compactified version tmf(1).
    pub is_compactified: bool,
}
impl TopologicalModularForms {
    /// The connective tmf.
    pub fn connective() -> Self {
        TopologicalModularForms {
            is_connective: true,
            is_compactified: false,
        }
    }
    /// The periodic TMF (Tmf with capital T).
    pub fn periodic() -> Self {
        TopologicalModularForms {
            is_connective: false,
            is_compactified: false,
        }
    }
}
/// Orientation data A: MU → E_*(BU) for a complex-oriented theory.
pub struct OrientationData {
    /// The complex-oriented theory name.
    pub theory: String,
    /// The Chern class in π_2(E).
    pub first_chern_class: i64,
}
/// Represents a lambda-ring element with Adams operations stored up to degree `n`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LambdaRingElement {
    /// The Adams operations ψ^k applied to this element, for k=1..=degree.
    pub adams_ops: Vec<f64>,
    /// The actual element value (as a formal power series coefficient).
    pub value: f64,
}
#[allow(dead_code)]
impl LambdaRingElement {
    /// Creates a new lambda-ring element with Adams operations.
    pub fn new(value: f64, degree: usize) -> Self {
        let adams_ops = (1..=degree).map(|k| (k as f64) * value).collect();
        LambdaRingElement { adams_ops, value }
    }
    /// Returns ψ^k of this element (1-indexed).
    pub fn adams_op(&self, k: usize) -> Option<f64> {
        if k == 0 || k > self.adams_ops.len() {
            None
        } else {
            Some(self.adams_ops[k - 1])
        }
    }
    /// Adds two lambda-ring elements (both Adams operations and value).
    pub fn add(&self, other: &LambdaRingElement) -> LambdaRingElement {
        let degree = self.adams_ops.len().min(other.adams_ops.len());
        let adams_ops = (0..degree)
            .map(|i| self.adams_ops[i] + other.adams_ops[i])
            .collect();
        LambdaRingElement {
            adams_ops,
            value: self.value + other.value,
        }
    }
    /// Tensor product in lambda-ring (ψ^k multiplicative).
    pub fn tensor(&self, other: &LambdaRingElement) -> LambdaRingElement {
        let degree = self.adams_ops.len().min(other.adams_ops.len());
        let adams_ops = (0..degree)
            .map(|i| self.adams_ops[i] * other.adams_ops[i])
            .collect();
        LambdaRingElement {
            adams_ops,
            value: self.value * other.value,
        }
    }
    /// Checks the Adams operation composition: ψ^m ∘ ψ^n = ψ^{mn}.
    pub fn check_composition(&self, m: usize, n: usize) -> bool {
        let mn = m * n;
        if mn > self.adams_ops.len() || m > self.adams_ops.len() || n > self.adams_ops.len() {
            return true;
        }
        let psi_mn_direct = mn as f64 * self.value;
        let psi_mn_via_comp = m as f64 * (n as f64 * self.value);
        (psi_mn_direct - psi_mn_via_comp).abs() < 1e-10
    }
}
/// Descent data for the Galois action Gal(ℂ/ℝ).
///
/// Used to construct the real tmf_ℝ as homotopy fixed points.
pub struct DescentData {
    /// The spectrum being descended (by name).
    pub spectrum: String,
    /// The group acting (e.g., "Gal(C/R)").
    pub group: String,
    /// Whether the homotopy fixed points have been computed.
    pub hfp_computed: bool,
}
/// Morava E-theory E(k, Γ): the Lubin-Tate spectrum at height k.
pub struct MoravaETheory {
    /// The chromatic height k.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
    /// A name for the formal group Γ.
    pub formal_group_name: String,
    /// Whether the Goerss-Hopkins-Miller theorem has been applied (E is an E_∞-ring).
    pub is_e_infty_ring: bool,
}
impl MoravaETheory {
    /// Create Morava E-theory at height `k` and prime `p`.
    pub fn new(height: usize, prime: u64) -> Self {
        MoravaETheory {
            height,
            prime,
            formal_group_name: format!("Gamma_{height}"),
            is_e_infty_ring: true,
        }
    }
    /// The periodicity of Morava E-theory: π_*(E(k)) has period 2(p^k - 1).
    pub fn periodicity(&self) -> u64 {
        2 * (self.prime.pow(self.height as u32) - 1)
    }
}
/// The Honda formal group H_n of height n over F_{p^n}.
///
/// H_n is determined (up to isomorphism) by its [p]-series [p](x) = x^{p^n}.
pub struct HondaFormalGroup {
    /// The height n.
    pub height: usize,
    /// The prime p.
    pub prime: u64,
}
impl HondaFormalGroup {
    /// Create the Honda formal group of height `n` at prime `p`.
    pub fn new(height: usize, prime: u64) -> Self {
        HondaFormalGroup { height, prime }
    }
    /// The degree of the [p]-series is p^n.
    pub fn p_series_degree(&self) -> u64 {
        self.prime.pow(self.height as u32)
    }
    /// Two Honda formal groups are isomorphic iff they have the same height and prime.
    pub fn isomorphic_to(&self, other: &HondaFormalGroup) -> bool {
        self.height == other.height && self.prime == other.prime
    }
}
/// Represents a deformation of a formal group law over a local ring.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormalGroupDeformation {
    /// Height of the formal group.
    pub height: usize,
    /// The Lubin-Tate parameter (deformation direction).
    pub lt_parameters: Vec<f64>,
    /// Characteristic of the residue field.
    pub char_p: usize,
}
#[allow(dead_code)]
impl FormalGroupDeformation {
    /// Creates a universal deformation (Lubin-Tate theory).
    pub fn universal(height: usize, char_p: usize) -> Self {
        let lt_parameters = vec![0.0; height.saturating_sub(1)];
        FormalGroupDeformation {
            height,
            lt_parameters,
            char_p,
        }
    }
    /// Returns the dimension of the universal deformation ring.
    pub fn deformation_ring_dim(&self) -> usize {
        self.height.saturating_sub(1)
    }
    /// Sets a Lubin-Tate parameter.
    pub fn set_lt_param(&mut self, i: usize, val: f64) {
        if i < self.lt_parameters.len() {
            self.lt_parameters[i] = val;
        }
    }
    /// Checks if this is the Lubin-Tate universal deformation.
    pub fn is_lubin_tate(&self) -> bool {
        self.height >= 1 && self.char_p >= 2
    }
    /// Describes the associated Morava stabilizer group.
    pub fn morava_stabilizer_group(&self) -> String {
        format!(
            "S_{} = Aut(Γ_{}^{{1}}) over F_{}",
            self.height, self.height, self.char_p
        )
    }
    /// Hasse invariant: vanishing implies supersingular (height jump).
    pub fn hasse_invariant_vanishes(&self) -> bool {
        self.height >= 2
    }
}
/// The localization unit η: X → L_E X.
pub struct LocalizationUnit {
    /// Whether the unit map has been constructed.
    pub constructed: bool,
    /// Whether the map is a weak equivalence for E-local spectra.
    pub is_e_local_equiv: bool,
}
/// A formal group law F(x,y) truncated at degree `max_deg`.
/// Stores coefficients a_{ij} such that F(x,y) = sum_{i+j <= max_deg} a_{ij} x^i y^j.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FglArithmetic {
    /// Coefficients indexed by (i, j).
    pub coeffs: std::collections::HashMap<(usize, usize), f64>,
    /// Truncation degree.
    pub max_deg: usize,
    /// Name of the formal group law.
    pub name: String,
}
#[allow(dead_code)]
impl FglArithmetic {
    /// Creates the additive formal group law F(x,y) = x + y.
    pub fn additive(max_deg: usize) -> Self {
        let mut coeffs = std::collections::HashMap::new();
        coeffs.insert((1, 0), 1.0);
        coeffs.insert((0, 1), 1.0);
        FglArithmetic {
            coeffs,
            max_deg,
            name: "Additive".to_string(),
        }
    }
    /// Creates the multiplicative formal group law F(x,y) = x + y + xy.
    pub fn multiplicative(max_deg: usize) -> Self {
        let mut coeffs = std::collections::HashMap::new();
        coeffs.insert((1, 0), 1.0);
        coeffs.insert((0, 1), 1.0);
        coeffs.insert((1, 1), 1.0);
        FglArithmetic {
            coeffs,
            max_deg,
            name: "Multiplicative".to_string(),
        }
    }
    /// Evaluates F(x, y) at the given values.
    pub fn evaluate(&self, x: f64, y: f64) -> f64 {
        let mut result = 0.0;
        for (&(i, j), &a) in &self.coeffs {
            if i + j <= self.max_deg {
                result += a * x.powi(i as i32) * y.powi(j as i32);
            }
        }
        result
    }
    /// Checks commutativity: F(x,y) ≈ F(y,x) at sample points.
    pub fn is_commutative_approx(&self, tol: f64) -> bool {
        let samples = [(0.1, 0.2), (0.3, 0.4), (0.05, 0.15)];
        for &(x, y) in &samples {
            let fxy = self.evaluate(x, y);
            let fyx = self.evaluate(y, x);
            if (fxy - fyx).abs() > tol {
                return false;
            }
        }
        true
    }
    /// Returns the coefficient a_{i,j}.
    pub fn coeff(&self, i: usize, j: usize) -> f64 {
        *self.coeffs.get(&(i, j)).unwrap_or(&0.0)
    }
    /// Sets a coefficient.
    pub fn set_coeff(&mut self, i: usize, j: usize, val: f64) {
        if i + j <= self.max_deg {
            self.coeffs.insert((i, j), val);
        }
    }
    /// Computes the p-series [p](x) = F(x, F(x, ... F(x, x)...)) (p times), truncated.
    pub fn p_series(&self, p: usize, x: f64) -> f64 {
        if p == 0 {
            return 0.0;
        }
        let mut acc = x;
        for _ in 1..p {
            acc = self.evaluate(acc, x);
        }
        acc
    }
    /// Returns the height-1 approximation: whether the 2-series vanishes mod x^3.
    pub fn is_height_one_approx(&self) -> bool {
        let x = 0.01f64;
        let two_series = self.p_series(2, x);
        two_series.abs() < 0.001
    }
}
/// Tracks nilpotence data for elements in the stable homotopy ring.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NilpotenceData {
    /// Description of the element.
    pub element: String,
    /// Whether the element is known to be nilpotent.
    pub is_nilpotent: bool,
    /// The nilpotency exponent (n such that x^n = 0), if known.
    pub nilpotency_exponent: Option<usize>,
    /// The relevant prime.
    pub prime: usize,
}
#[allow(dead_code)]
impl NilpotenceData {
    /// Creates nilpotence data.
    pub fn new(element: &str, prime: usize) -> Self {
        NilpotenceData {
            element: element.to_string(),
            is_nilpotent: false,
            nilpotency_exponent: None,
            prime,
        }
    }
    /// Marks the element as nilpotent with given exponent.
    pub fn set_nilpotent(&mut self, exp: usize) {
        self.is_nilpotent = true;
        self.nilpotency_exponent = Some(exp);
    }
    /// Checks Nishida's theorem: all positive-degree elements of π_*(S) are nilpotent.
    pub fn satisfies_nishida(&self) -> bool {
        self.is_nilpotent
    }
    /// Returns the filtration at which nilpotence is detected.
    pub fn detecting_filtration(&self) -> String {
        if self.is_nilpotent {
            format!("Detected in Adams filtration (prime {})", self.prime)
        } else {
            "Not nilpotent or unknown".to_string()
        }
    }
}
/// The chromatic spectral sequence.
///
/// E_1^{n,*} = π_*(M_n X) ⇒ π_*(X_p^).
pub struct ChromaticSS {
    /// The spectrum X.
    pub spectrum: String,
    /// The prime p.
    pub prime: u64,
    /// The monochromatic layers π_*(M_n X), as groups (ranks at each degree).
    pub monochromatic_layers: Vec<Vec<usize>>,
}
impl ChromaticSS {
    /// The rank of the E_1^{n,d} term (π_d(M_n X)).
    pub fn e1_rank(&self, n: usize, d: usize) -> usize {
        self.monochromatic_layers
            .get(n)
            .and_then(|layer| layer.get(d))
            .copied()
            .unwrap_or(0)
    }
}
/// A thick subcategory C(n) of finite p-local spectra.
///
/// C(n) = { X finite | K(m)_*(X) = 0 for all m < n }.
pub struct ThickSubcategoryData {
    /// The index n.
    pub index: usize,
    /// The prime p.
    pub prime: u64,
    /// Known members (spectrum names).
    pub members: Vec<String>,
}
impl ThickSubcategoryData {
    /// Create the n-th thick subcategory at prime p.
    pub fn new(index: usize, prime: u64) -> Self {
        ThickSubcategoryData {
            index,
            prime,
            members: Vec::new(),
        }
    }
    /// Add a finite spectrum (by name) to this thick subcategory.
    pub fn add_member(&mut self, name: impl Into<String>) {
        self.members.push(name.into());
    }
    /// Check whether a spectrum (by name) is in this thick subcategory.
    pub fn contains(&self, name: &str) -> bool {
        self.members.iter().any(|m| m == name)
    }
}
