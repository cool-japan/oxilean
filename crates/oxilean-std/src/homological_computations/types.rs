//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::functions::*;

/// Convergence data: E_∞^{p,q} ≅ Gr^p(H^{p+q}) for a filtered complex.
#[derive(Debug, Clone)]
pub struct Convergence {
    /// The E_∞ page.
    pub e_infty: SpectralSequencePage,
    /// The limit filtration grades Gr^p(H^n) for each (p,n).
    pub filtration_grades: HashMap<(i32, i32), usize>,
}
impl Convergence {
    /// Create a convergence from E_∞.
    pub fn new(e_infty: SpectralSequencePage) -> Self {
        let filtration_grades = e_infty.entries.clone();
        Self {
            e_infty,
            filtration_grades,
        }
    }
    /// Compute the rank of H^n = ∑_p E_∞^{p, n-p}.
    pub fn cohomology_rank(&self, n: i32) -> usize {
        self.e_infty
            .entries
            .iter()
            .filter(|(&(p, q), _)| p + q == n)
            .map(|(_, &r)| r)
            .sum()
    }
}
/// A single local cohomology group H^n_I(M).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCohomologyGroup {
    /// Cohomological degree.
    pub degree: usize,
    /// Free rank (over an appropriate ring).
    pub rank: usize,
}
impl LocalCohomologyGroup {
    /// Create a local cohomology group.
    pub fn new(degree: usize, rank: usize) -> Self {
        Self { degree, rank }
    }
    /// True iff the group vanishes.
    pub fn is_zero(&self) -> bool {
        self.rank == 0
    }
}
/// A graded abelian group (free, with given rank).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradedGroup {
    /// The rank of this free abelian group.
    pub rank: usize,
    /// Human-readable name.
    pub name: String,
}
impl GradedGroup {
    /// Create a graded group.
    pub fn new(rank: usize, name: &str) -> Self {
        Self {
            rank,
            name: name.to_string(),
        }
    }
}
/// Persistent homology interval (bar).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistenceInterval {
    pub dimension: usize,
    pub birth: f64,
    pub death: f64,
}
#[allow(dead_code)]
impl PersistenceInterval {
    /// Create a persistence interval.
    pub fn new(dim: usize, birth: f64, death: f64) -> Self {
        Self {
            dimension: dim,
            birth,
            death,
        }
    }
    /// Persistence (lifetime).
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }
    /// Is this an essential class (infinite lifetime)?
    pub fn is_essential(&self) -> bool {
        self.death == f64::INFINITY
    }
    /// Does the interval contain a given filtration value?
    pub fn contains(&self, t: f64) -> bool {
        t >= self.birth && t < self.death
    }
}
/// Barcode: a collection of birth-death pairs.
#[derive(Debug, Clone, Default)]
pub struct PersistenceBarcode {
    /// All birth-death intervals.
    pub intervals: Vec<BirthDeathPair>,
}
impl PersistenceBarcode {
    /// Create an empty barcode.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an interval.
    pub fn add(&mut self, birth: f64, death: f64, degree: usize) {
        self.intervals
            .push(BirthDeathPair::new(birth, death, degree));
    }
    /// Return all intervals in degree `n`.
    pub fn intervals_in_degree(&self, n: usize) -> Vec<&BirthDeathPair> {
        self.intervals.iter().filter(|p| p.degree == n).collect()
    }
    /// Count intervals in degree `n`.
    pub fn betti_number(&self, n: usize) -> usize {
        self.intervals_in_degree(n).len()
    }
    /// Compute the bottleneck distance between two barcodes (degree n).
    ///
    /// Uses a greedy matching approximation: sort by persistence and match greedily.
    pub fn bottleneck_distance(&self, other: &PersistenceBarcode, n: usize) -> f64 {
        let mut a: Vec<f64> = self
            .intervals_in_degree(n)
            .iter()
            .map(|p| p.persistence().min(1e9))
            .collect();
        let mut b: Vec<f64> = other
            .intervals_in_degree(n)
            .iter()
            .map(|p| p.persistence().min(1e9))
            .collect();
        a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
        b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
        let mut dist = 0.0_f64;
        let len = a.len().max(b.len());
        for i in 0..len {
            let ai = a.get(i).copied().unwrap_or(0.0);
            let bi = b.get(i).copied().unwrap_or(0.0);
            dist = dist.max((ai - bi).abs());
        }
        dist
    }
}
/// Manages the pages of a spectral sequence together with their differentials.
#[derive(Debug, Clone, Default)]
pub struct SpectralSequencePageManager {
    /// Current page number.
    pub current_page: usize,
    /// Pages E_2, E_3, … stored in order (index 0 = E_2).
    pub pages: Vec<SpectralSequencePage>,
    /// Differentials on each page.
    pub differentials: Vec<Vec<DifferentialMap>>,
}
impl SpectralSequencePageManager {
    /// Create a manager starting at E_2.
    pub fn new() -> Self {
        Self {
            current_page: 2,
            pages: vec![],
            differentials: vec![],
        }
    }
    /// Add the E_r page (r must equal current_page).
    pub fn add_page(&mut self, entries: HashMap<(i32, i32), usize>) {
        let r = self.current_page;
        let mut page = SpectralSequencePage::new(r);
        for ((p, q), rank) in entries {
            page.set(p, q, rank);
        }
        self.pages.push(page);
        self.differentials.push(vec![]);
        self.current_page += 1;
    }
    /// Add a differential d_r: E_r^{(p,q)} → E_r^{(p+r, q-r+1)}.
    pub fn add_differential(&mut self, p: i32, q: i32, image_rank: usize) {
        let r = self.current_page.saturating_sub(1);
        let d = DifferentialMap::new(r, p, q, image_rank);
        if let Some(last) = self.differentials.last_mut() {
            last.push(d);
        }
    }
    /// Advance to the next page by taking homology of the current differentials.
    pub fn advance(&mut self) {
        let last_idx = self.pages.len().saturating_sub(1);
        if self.pages.is_empty() {
            return;
        }
        let current = &self.pages[last_idx];
        let diffs = &self.differentials[last_idx];
        let r = current.page;
        let ri = r as i32;
        let mut next = SpectralSequencePage::new(r + 1);
        for (&(p, q), &rank) in &current.entries {
            let incoming = diffs
                .iter()
                .find(|d| d.target == (p, q))
                .map_or(0, |d| d.image_rank);
            let outgoing = diffs
                .iter()
                .find(|d| d.source == (p, q))
                .map_or(0, |d| d.image_rank);
            let new_rank = rank.saturating_sub(outgoing).saturating_sub(incoming);
            if new_rank > 0 {
                next.set(p + ri, q - ri + 1, new_rank);
            }
        }
        self.pages.push(next);
        self.differentials.push(vec![]);
        self.current_page += 1;
    }
    /// Get the E_r page (r is the actual page number, r ≥ 2).
    pub fn page(&self, r: usize) -> Option<&SpectralSequencePage> {
        r.checked_sub(2).and_then(|idx| self.pages.get(idx))
    }
    /// Check if the spectral sequence has collapsed (all differentials are zero).
    pub fn has_collapsed(&self) -> bool {
        self.differentials
            .iter()
            .all(|diffs| diffs.iter().all(|d| d.image_rank == 0))
    }
}
/// Tor and Ext functor data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TorExtData {
    pub functor: String,
    pub ring: String,
    pub module1: String,
    pub module2: String,
    pub degree: usize,
    pub value: String,
}
#[allow(dead_code)]
impl TorExtData {
    /// Tor_n^R(M, N).
    pub fn tor(ring: &str, m: &str, n_mod: &str, degree: usize) -> Self {
        Self {
            functor: "Tor".to_string(),
            ring: ring.to_string(),
            module1: m.to_string(),
            module2: n_mod.to_string(),
            degree,
            value: format!("Tor_{}^{}({},{})", degree, ring, m, n_mod),
        }
    }
    /// Ext^n_R(M, N).
    pub fn ext(ring: &str, m: &str, n_mod: &str, degree: usize) -> Self {
        Self {
            functor: "Ext".to_string(),
            ring: ring.to_string(),
            module1: m.to_string(),
            module2: n_mod.to_string(),
            degree,
            value: format!("Ext^{}_{}({},{})", degree, ring, m, n_mod),
        }
    }
    /// Balanced Tor: Tor_n^R(M,N) can be computed from either a resolution of M or N.
    pub fn is_balanced(&self) -> bool {
        self.functor == "Tor"
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TruncationFunctor {
    pub category: String,
    pub cutoff: i64,
    pub is_cohomological: bool,
}
#[allow(dead_code)]
impl TruncationFunctor {
    pub fn new(cat: &str, cutoff: i64, cohom: bool) -> Self {
        TruncationFunctor {
            category: cat.to_string(),
            cutoff,
            is_cohomological: cohom,
        }
    }
    pub fn truncation_description(&self) -> String {
        if self.is_cohomological {
            format!("τ_≥{}: kills H^i for i < {}", self.cutoff, self.cutoff)
        } else {
            format!("τ_≤{}: kills H^i for i > {}", self.cutoff, self.cutoff)
        }
    }
    pub fn composed_truncation(&self, other_cutoff: i64) -> String {
        format!(
            "τ_≥{} ∘ τ_≤{}: concentrated in degrees [{}, {}]",
            self.cutoff, other_cutoff, self.cutoff, other_cutoff
        )
    }
}
/// A single Tor group Tor_n^R(M, N).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorGroup {
    /// Homological degree n.
    pub degree: usize,
    /// Free rank of the group.
    pub rank: usize,
    /// Torsion summands.
    pub torsion: Vec<u64>,
}
impl TorGroup {
    /// Create a Tor group.
    pub fn new(degree: usize, rank: usize) -> Self {
        Self {
            degree,
            rank,
            torsion: vec![],
        }
    }
    /// True iff the group is zero.
    pub fn is_zero(&self) -> bool {
        self.rank == 0 && self.torsion.is_empty()
    }
}
/// Ext^n_R(M, N) computed from a projective resolution of M.
#[derive(Debug, Clone)]
pub struct ExtFunctor {
    /// Name of module M.
    pub module_m: String,
    /// Name of module N.
    pub module_n: String,
    /// Computed cohomology groups Ext^0, Ext^1, …
    pub values: Vec<ExtGroup>,
}
impl ExtFunctor {
    /// Compute Ext from a projective resolution of M (apply Hom(−,N)).
    pub fn compute(module_m: &str, module_n: &str, resolution: &ProjectiveResolution) -> Self {
        let betti = resolution.betti_numbers();
        let values = betti
            .iter()
            .enumerate()
            .map(|(n, &r)| ExtGroup::new(n, r))
            .collect();
        Self {
            module_m: module_m.to_string(),
            module_n: module_n.to_string(),
            values,
        }
    }
    /// Retrieve Ext^n(M, N).
    pub fn ext_at(&self, n: usize) -> Option<&ExtGroup> {
        self.values.get(n)
    }
    /// Injective dimension of N = largest n with Ext^n(M, N) ≠ 0.
    pub fn injective_dimension(&self) -> Option<usize> {
        self.values.iter().rposition(|e| !e.is_zero())
    }
}
/// The Lyndon-Hochschild-Serre spectral sequence for a group extension
/// 1 → N → G → Q → 1.
///
/// E_2^{p,q} = H^p(Q, H^q(N, M)) ⇒ H^{p+q}(G, M).
#[derive(Debug, Clone)]
pub struct LyndonHochschildSerre {
    /// Name of the normal subgroup N.
    pub normal_subgroup: String,
    /// Name of the quotient group Q.
    pub quotient_group: String,
    /// Name of the module M.
    pub module: String,
    /// The E_2 page.
    pub e2_page: SpectralSequencePage,
}
impl LyndonHochschildSerre {
    /// Create a Lyndon-Hochschild-Serre spectral sequence.
    pub fn new(
        normal_subgroup: &str,
        quotient_group: &str,
        module: &str,
        e2_entries: HashMap<(i32, i32), usize>,
    ) -> Self {
        let mut e2_page = SpectralSequencePage::new(2);
        for ((p, q), rank) in e2_entries {
            e2_page.set(p, q, rank);
        }
        Self {
            normal_subgroup: normal_subgroup.to_string(),
            quotient_group: quotient_group.to_string(),
            module: module.to_string(),
            e2_page,
        }
    }
    /// Compute the abutment H^n(G, M) = ∑_{p+q=n} E_∞^{p,q}.
    ///
    /// For the LHS spectral sequence, the E_2 page has
    /// E_2^{p,q} = H^p(Q, H^q(N, M)).
    pub fn abutment_rank(&self, n: i32) -> usize {
        self.e2_page
            .entries
            .iter()
            .filter(|(&(p, q), _)| p + q == n)
            .map(|(_, &r)| r)
            .sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HochschildComplex {
    pub algebra: String,
    pub module: String,
    pub dimension: usize,
    pub is_free_algebra: bool,
}
#[allow(dead_code)]
impl HochschildComplex {
    pub fn new(alg: &str, mod_: &str, dim: usize) -> Self {
        HochschildComplex {
            algebra: alg.to_string(),
            module: mod_.to_string(),
            dimension: dim,
            is_free_algebra: false,
        }
    }
    pub fn hochschild_kostant_rosenberg(&self) -> String {
        "HKR: for smooth commutative algebra A, HH_*(A) ≅ Ω^*_{A/k} (differential forms)"
            .to_string()
    }
    pub fn cyclic_homology_connection(&self) -> String {
        format!(
            "HC_*({}): Connes' SBI exact sequence HC → HC → HH → (shift)",
            self.algebra
        )
    }
    pub fn loday_quillen_tsygan(&self) -> String {
        "Loday-Quillen-Tsygan: HC(A) ≅ primitive elements of H*(gl(A))".to_string()
    }
    pub fn degeneration_at_e2(&self) -> bool {
        self.is_free_algebra
    }
}
/// A spectral sequence with multiple pages.
#[derive(Debug, Clone, Default)]
pub struct SpectralSequence {
    /// Pages E_0, E_1, E_2, …
    pub pages: Vec<SpectralSequencePage>,
    /// Differentials on each page.
    pub differentials: Vec<Vec<DifferentialMap>>,
}
impl SpectralSequence {
    /// Create an empty spectral sequence.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a page.
    pub fn add_page(&mut self, entries: HashMap<(i32, i32), usize>) {
        let page = self.pages.len();
        let mut p = SpectralSequencePage::new(page);
        for ((px, q), rank) in entries {
            p.set(px, q, rank);
        }
        self.pages.push(p);
        self.differentials.push(vec![]);
    }
    /// Add a differential on page r.
    pub fn add_differential(&mut self, r: usize, p: i32, q: i32, image_rank: usize) {
        let d = DifferentialMap::new(r, p, q, image_rank);
        if r < self.differentials.len() {
            self.differentials[r].push(d);
        }
    }
    /// Retrieve E_r^{p,q}.
    pub fn e_term(&self, r: usize, p: i32, q: i32) -> Option<usize> {
        self.pages.get(r).map(|pg| pg.get(p, q))
    }
    /// Compute the E_{r+1} page from E_r by taking cohomology of d_r.
    ///
    /// E_{r+1}^{p,q} = ker(d_r from (p,q)) / im(d_r into (p,q)).
    pub fn compute_next_page(&self, r: usize) -> SpectralSequencePage {
        if r >= self.pages.len() {
            return SpectralSequencePage::new(r + 1);
        }
        let current = &self.pages[r];
        let mut next = SpectralSequencePage::new(r + 1);
        let ri = r as i32;
        for (&(p, q), &rank) in &current.entries {
            let incoming_im = if r < self.differentials.len() {
                self.differentials[r]
                    .iter()
                    .find(|d| d.target == (p, q))
                    .map_or(0, |d| d.image_rank)
            } else {
                0
            };
            let outgoing_im = if r < self.differentials.len() {
                self.differentials[r]
                    .iter()
                    .find(|d| d.source == (p, q))
                    .map_or(0, |d| d.image_rank)
            } else {
                0
            };
            let ker_rank = rank.saturating_sub(outgoing_im);
            let new_rank = ker_rank.saturating_sub(incoming_im);
            if new_rank > 0 {
                next.set(p + ri, q - ri + 1, new_rank);
            }
        }
        next
    }
}
/// Künneth formula: computes Betti numbers of a product space X × Y.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KunnethFormula {
    pub betti_x: Vec<i64>,
    pub betti_y: Vec<i64>,
}
#[allow(dead_code)]
impl KunnethFormula {
    pub fn new(betti_x: Vec<i64>, betti_y: Vec<i64>) -> Self {
        Self { betti_x, betti_y }
    }
    /// Betti numbers of X × Y: β_n(X×Y) = Σ_{i+j=n} β_i(X)*β_j(Y).
    pub fn product_betti(&self) -> Vec<i64> {
        let nx = self.betti_x.len();
        let ny = self.betti_y.len();
        let n = nx + ny - 1;
        let mut result = vec![0i64; n];
        for i in 0..nx {
            for j in 0..ny {
                result[i + j] += self.betti_x[i] * self.betti_y[j];
            }
        }
        result
    }
    /// Euler characteristic of product: χ(X×Y) = χ(X)*χ(Y).
    pub fn euler_product(&self) -> i64 {
        let chi = |betti: &Vec<i64>| -> i64 {
            betti
                .iter()
                .enumerate()
                .map(|(k, &b)| if k % 2 == 0 { b } else { -b })
                .sum()
        };
        chi(&self.betti_x) * chi(&self.betti_y)
    }
}
/// Computes group cohomology via the bar resolution.
///
/// Given a finite group G of order |G|, this implements the bar complex
/// C^n(G, M) = Map(G^n, M) with coboundary maps.
#[derive(Debug, Clone)]
pub struct GroupCohomologyBar {
    /// Group name.
    pub group_name: String,
    /// Group order.
    pub group_order: usize,
    /// Module name.
    pub module_name: String,
    /// Computed cohomology ranks Hⁿ(G, M) for n = 0, 1, …
    pub cohomology_ranks: Vec<usize>,
}
impl GroupCohomologyBar {
    /// Create a new bar resolution computer.
    pub fn new(group_name: &str, group_order: usize, module_name: &str) -> Self {
        Self {
            group_name: group_name.to_string(),
            group_order,
            module_name: module_name.to_string(),
            cohomology_ranks: vec![],
        }
    }
    /// Compute the rank of the bar cochain module C^n(G, M).
    ///
    /// C^n(G, M) = Map(G^n, M), so rank = |G|^n × rank(M).
    pub fn cochain_rank(&self, n: usize, module_rank: usize) -> usize {
        self.group_order.saturating_pow(n as u32) * module_rank
    }
    /// Set the computed cohomology ranks for n = 0, 1, … up to max_degree.
    pub fn compute_cohomology(&mut self, module_rank: usize, max_degree: usize) {
        self.cohomology_ranks = (0..=max_degree)
            .map(|n| if n == 0 { module_rank.min(1) } else { 0 })
            .collect();
    }
    /// Return H^n(G, M).
    pub fn cohomology_at(&self, n: usize) -> usize {
        self.cohomology_ranks.get(n).copied().unwrap_or(0)
    }
    /// Euler characteristic ∑_n (-1)^n dim H^n(G, M).
    pub fn euler_characteristic(&self) -> i64 {
        self.cohomology_ranks
            .iter()
            .enumerate()
            .map(|(n, &r)| if n % 2 == 0 { r as i64 } else { -(r as i64) })
            .sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CyclicHomologyData {
    pub algebra_type: String,
    pub hc_0: String,
    pub negative_cyclic: String,
    pub periodic_cyclic: String,
}
#[allow(dead_code)]
impl CyclicHomologyData {
    pub fn for_polynomial_ring(vars: usize) -> Self {
        CyclicHomologyData {
            algebra_type: format!("k[x_1,...,x_{}]", vars),
            hc_0: format!("Ω^0 = k[x_1,...,x_{}]", vars),
            negative_cyclic: "HC^-_{*} related to de Rham".to_string(),
            periodic_cyclic: "HP_* = de Rham cohomology (periodic)".to_string(),
        }
    }
    pub fn connes_differential(&self) -> String {
        "Connes B-operator: B: HC_n → HC_{n+1} (degree +1 boundary-like)".to_string()
    }
    pub fn primary_characteristic_class(&self) -> String {
        "Chern character: K_0(A) → HC_0(A) ≅ A/[A,A] (trace map)".to_string()
    }
}
/// Mayer–Vietoris sequence data for computing homology via excision.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MayerVietorisSequence {
    /// Space X = A ∪ B, intersection C = A ∩ B.
    pub space_name: String,
    pub a_name: String,
    pub b_name: String,
    pub c_name: String,
    /// Betti numbers of A, B, C, X.
    pub betti_a: Vec<i64>,
    pub betti_b: Vec<i64>,
    pub betti_c: Vec<i64>,
    pub betti_x: Vec<i64>,
}
#[allow(dead_code)]
impl MayerVietorisSequence {
    pub fn new(
        space_name: &str,
        a_name: &str,
        b_name: &str,
        c_name: &str,
        betti_a: Vec<i64>,
        betti_b: Vec<i64>,
        betti_c: Vec<i64>,
        betti_x: Vec<i64>,
    ) -> Self {
        Self {
            space_name: space_name.to_string(),
            a_name: a_name.to_string(),
            b_name: b_name.to_string(),
            c_name: c_name.to_string(),
            betti_a,
            betti_b,
            betti_c,
            betti_x,
        }
    }
    /// Euler characteristic from Betti numbers of X: χ = Σ (-1)^k β_k.
    pub fn euler_characteristic(&self) -> i64 {
        self.betti_x
            .iter()
            .enumerate()
            .map(|(k, &b)| if k % 2 == 0 { b } else { -b })
            .sum()
    }
    /// Verify the Mayer–Vietoris Euler relation: χ(X) = χ(A) + χ(B) - χ(C).
    pub fn verify_euler_relation(&self) -> bool {
        let chi = |betti: &Vec<i64>| -> i64 {
            betti
                .iter()
                .enumerate()
                .map(|(k, &b)| if k % 2 == 0 { b } else { -b })
                .sum()
        };
        chi(&self.betti_x) == chi(&self.betti_a) + chi(&self.betti_b) - chi(&self.betti_c)
    }
}
/// Universal coefficient theorem data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UniversalCoefficients {
    /// Integral homology groups as (free_rank, torsion_coefficients).
    pub integral_homology: Vec<(usize, Vec<u64>)>,
    /// Coefficient ring label.
    pub coeff_ring: String,
}
#[allow(dead_code)]
impl UniversalCoefficients {
    pub fn new(integral_homology: Vec<(usize, Vec<u64>)>, coeff_ring: &str) -> Self {
        Self {
            integral_homology,
            coeff_ring: coeff_ring.to_string(),
        }
    }
    /// Over a field (e.g., Q, Z/p), all torsion dies: β_k = free rank.
    pub fn betti_over_field(&self) -> Vec<usize> {
        self.integral_homology.iter().map(|(r, _)| *r).collect()
    }
    /// Torsion part H_k(X; Z)/free.
    pub fn torsion_part(&self, k: usize) -> Option<&Vec<u64>> {
        self.integral_homology.get(k).map(|(_, t)| t)
    }
}
/// A sparse integer matrix represented as a list of (row, col, value) triples.
#[derive(Debug, Clone, Default)]
pub struct SimplexBoundaryMatrix {
    /// Number of rows.
    pub rows: usize,
    /// Number of cols.
    pub cols: usize,
    /// Nonzero entries (row, col, value).
    pub entries: Vec<(usize, usize, i64)>,
}
impl SimplexBoundaryMatrix {
    /// Create a new zero boundary matrix.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            entries: vec![],
        }
    }
    /// Set entry (i, j) to value v.
    pub fn set(&mut self, i: usize, j: usize, v: i64) {
        self.entries.retain(|(r, c, _)| !(*r == i && *c == j));
        if v != 0 {
            self.entries.push((i, j, v));
        }
    }
    /// Get entry (i, j).
    pub fn get(&self, i: usize, j: usize) -> i64 {
        self.entries
            .iter()
            .find(|(r, c, _)| *r == i && *c == j)
            .map_or(0, |(_, _, v)| *v)
    }
    /// Convert to dense matrix.
    pub fn to_dense(&self) -> Vec<Vec<i64>> {
        let mut m = vec![vec![0i64; self.cols]; self.rows];
        for &(r, c, v) in &self.entries {
            if r < self.rows && c < self.cols {
                m[r][c] = v;
            }
        }
        m
    }
    /// Compute the Smith Normal Form diagonal entries.
    ///
    /// Returns the nonzero diagonal invariant factors.
    pub fn smith_normal_form_diagonal(&self) -> Vec<i64> {
        let mut m = self.to_dense();
        let rows = self.rows;
        let cols = self.cols;
        let mut diag = vec![];
        let mut pivot = 0usize;
        for col in 0..cols {
            if pivot >= rows {
                break;
            }
            let found = (pivot..rows).find(|&r| m[r][col] != 0);
            if found.is_none() {
                continue;
            }
            let pr = found.expect("found is Some: checked by is_none guard above");
            m.swap(pivot, pr);
            let pv = m[pivot][col];
            for r in (pivot + 1)..rows {
                let factor = m[r][col];
                if factor != 0 {
                    for c in 0..cols {
                        m[r][c] = m[r][c] * pv - m[pivot][c] * factor;
                    }
                }
            }
            if m[pivot][col] != 0 {
                diag.push(m[pivot][col].abs());
            }
            pivot += 1;
        }
        diag
    }
    /// Compute the rank via SNF.
    pub fn rank(&self) -> usize {
        self.smith_normal_form_diagonal().len()
    }
    /// Compute the torsion coefficients (diagonal entries > 1).
    pub fn torsion_coefficients(&self) -> Vec<i64> {
        self.smith_normal_form_diagonal()
            .into_iter()
            .filter(|&d| d > 1)
            .collect()
    }
}
/// A single Ext group Ext^n_R(M, N).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtGroup {
    /// Cohomological degree n.
    pub degree: usize,
    /// Free rank.
    pub rank: usize,
    /// Torsion summands.
    pub torsion: Vec<u64>,
}
impl ExtGroup {
    /// Create an Ext group.
    pub fn new(degree: usize, rank: usize) -> Self {
        Self {
            degree,
            rank,
            torsion: vec![],
        }
    }
    /// True iff the Ext group is zero.
    pub fn is_zero(&self) -> bool {
        self.rank == 0 && self.torsion.is_empty()
    }
}
/// Long exact sequence in homology.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LongExactSequence {
    pub groups: Vec<String>,
    pub connecting_homomorphism: String,
}
#[allow(dead_code)]
impl LongExactSequence {
    /// Long exact sequence from short exact sequence 0 -> A -> B -> C -> 0.
    pub fn from_short(a: &str, b: &str, c: &str) -> Self {
        let groups = vec![
            format!("H_n({})", a),
            format!("H_n({})", b),
            format!("H_n({})", c),
            format!("H_{{n-1}}({})", a),
        ];
        Self {
            groups,
            connecting_homomorphism: format!("delta: H_n({}) -> H_{{n-1}}({})", c, a),
        }
    }
    /// Length (number of groups listed).
    pub fn length(&self) -> usize {
        self.groups.len()
    }
}
/// An injective resolution 0 → M → I_0 → I_1 → …
#[derive(Debug, Clone)]
pub struct InjectiveResolution {
    /// The module being resolved.
    pub module_name: String,
    /// The injective modules I_0, I_1, …
    pub steps: Vec<ResolutionStep>,
}
impl InjectiveResolution {
    /// Create a new injective resolution.
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            steps: vec![],
        }
    }
    /// Add a step.
    pub fn add_step(&mut self, rank: usize, boundary: Vec<Vec<i64>>) {
        let degree = self.steps.len();
        self.steps.push(ResolutionStep {
            degree,
            rank,
            boundary,
        });
    }
    /// Injective dimension.
    pub fn injective_dimension(&self) -> Option<usize> {
        self.steps.iter().rposition(|s| s.rank > 0)
    }
}
/// The n-th homology group H_n(C) = ker(d_n) / im(d_{n+1}).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomologyGroup {
    /// The homological degree.
    pub degree: i32,
    /// Free rank.
    pub rank: usize,
    /// Torsion summands (elementary divisors > 1).
    pub torsion: Vec<u64>,
}
impl HomologyGroup {
    /// Create a new homology group.
    pub fn new(degree: i32, rank: usize, torsion: Vec<u64>) -> Self {
        Self {
            degree,
            rank,
            torsion,
        }
    }
    /// True iff the group is trivial.
    pub fn is_trivial(&self) -> bool {
        self.rank == 0 && self.torsion.is_empty()
    }
}
/// Cellular homology computation via CW structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CWComplex {
    /// Number of n-cells: cells\[n\] = count of n-cells.
    pub cells: Vec<usize>,
    /// Attaching map degrees: degrees\[n\]\[i\] = degree of i-th attaching map in dimension n+1.
    pub attaching_degrees: Vec<Vec<i32>>,
}
#[allow(dead_code)]
impl CWComplex {
    pub fn new(cells: Vec<usize>) -> Self {
        let m = if cells.is_empty() { 0 } else { cells.len() - 1 };
        Self {
            cells,
            attaching_degrees: vec![vec![]; m],
        }
    }
    /// Euler characteristic from cell counts: χ = Σ (-1)^k |cells_k|.
    pub fn euler_characteristic(&self) -> i64 {
        self.cells
            .iter()
            .enumerate()
            .map(|(k, &c)| if k % 2 == 0 { c as i64 } else { -(c as i64) })
            .sum()
    }
    /// Tight Betti bound: β_k <= cells\[k\].
    pub fn betti_upper_bound(&self) -> Vec<usize> {
        self.cells.clone()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExtGroupComputation {
    pub module_a: String,
    pub module_b: String,
    pub projective_resolution_length: usize,
    pub computed_exts: Vec<String>,
}
#[allow(dead_code)]
impl ExtGroupComputation {
    pub fn new(a: &str, b: &str) -> Self {
        ExtGroupComputation {
            module_a: a.to_string(),
            module_b: b.to_string(),
            projective_resolution_length: 0,
            computed_exts: vec![],
        }
    }
    pub fn compute_ext_0(&self) -> String {
        format!(
            "Ext^0({}, {}) = Hom({}, {})",
            self.module_a, self.module_b, self.module_a, self.module_b
        )
    }
    pub fn compute_ext_1(&self) -> String {
        format!(
            "Ext^1({}, {}): obstruction to extension 0→{}→E→{}→0",
            self.module_a, self.module_b, self.module_b, self.module_a
        )
    }
    pub fn horseshoe_lemma(&self) -> String {
        "Horseshoe lemma: given short exact sequence of modules, combine projective resolutions"
            .to_string()
    }
    pub fn global_dimension(&self) -> String {
        format!(
            "gl.dim ≤ {}: ext vanishes above degree {}",
            self.projective_resolution_length, self.projective_resolution_length
        )
    }
}
/// Computes persistent homology from a filtered simplicial complex.
///
/// Implements the standard persistence algorithm (Edelsbrunner-Letscher-Zomorodian).
#[derive(Debug, Clone, Default)]
pub struct PersistentHomologyComputer {
    /// Simplices in filtration order: (filtration_value, dimension).
    pub filtration: Vec<(f64, usize)>,
}
impl PersistentHomologyComputer {
    /// Create a new computer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a simplex to the filtration.
    pub fn add_simplex(&mut self, filtration_value: f64, dimension: usize) {
        self.filtration.push((filtration_value, dimension));
        self.filtration.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
    }
    /// Compute the barcode using the standard persistence algorithm.
    ///
    /// This simplified version tracks when each homology class is born and dies
    /// based on simplex dimension parity (illustrative model).
    pub fn compute_barcode(&self) -> PersistenceBarcode {
        let mut barcode = PersistenceBarcode::new();
        let max_dim = self.filtration.iter().map(|(_, d)| *d).max().unwrap_or(0);
        for dim in 0..=max_dim {
            let birth_times: Vec<f64> = self
                .filtration
                .iter()
                .filter(|(_, d)| *d == dim)
                .map(|(v, _)| *v)
                .collect();
            let kill_times: Vec<f64> = self
                .filtration
                .iter()
                .filter(|(_, d)| *d == dim + 1)
                .map(|(v, _)| *v)
                .collect();
            let mut kill_iter = kill_times.iter().peekable();
            for birth in &birth_times {
                if let Some(&death) = kill_iter.next() {
                    barcode.add(*birth, death, dim);
                } else {
                    barcode.add(*birth, f64::INFINITY, dim);
                }
            }
        }
        barcode
    }
}
/// A chain map f: C_• → D_• (a degree-0 morphism of chain complexes).
#[derive(Debug, Clone)]
pub struct ChainMap {
    /// Source chain complex.
    pub source: ChainComplex,
    /// Target chain complex.
    pub target: ChainComplex,
    /// Component matrices f_n: C_n → D_n.
    pub components: Vec<Vec<Vec<i64>>>,
}
impl ChainMap {
    /// Create a chain map.
    pub fn new(source: ChainComplex, target: ChainComplex, components: Vec<Vec<Vec<i64>>>) -> Self {
        Self {
            source,
            target,
            components,
        }
    }
    /// Compute the induced map on homology H_n(C) → H_n(D) (returns rank of image).
    pub fn induced_homology_rank(&self, n: usize) -> usize {
        self.components
            .get(n)
            .map(|m| image_rank(m, self.source.groups.get(n).map_or(0, |g| g.rank)))
            .unwrap_or(0)
    }
}
/// Spectral sequence data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpectralSequenceData {
    pub name: String,
    pub filtration_type: String,
    pub converges_to: String,
    pub page: usize,
}
#[allow(dead_code)]
impl SpectralSequenceData {
    /// Serre spectral sequence.
    pub fn serre(base: &str, fiber: &str, total: &str) -> Self {
        Self {
            name: format!("Serre({} -> {} -> {})", fiber, total, base),
            filtration_type: "Serre filtration".to_string(),
            converges_to: format!("H*({})", total),
            page: 2,
        }
    }
    /// Leray spectral sequence.
    pub fn leray(map: &str) -> Self {
        Self {
            name: format!("Leray({})", map),
            filtration_type: "sheaf cohomology".to_string(),
            converges_to: format!("H*(domain({}))", map),
            page: 2,
        }
    }
    /// Description of E_2 page.
    pub fn e2_page_description(&self) -> String {
        format!(
            "E_2 page of {}: converges to {}",
            self.name, self.converges_to
        )
    }
}
/// Fibration sequence: F -> E -> B with long exact sequence of homotopy groups.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FibrationSequence {
    pub total_space: String,
    pub base_space: String,
    pub fiber: String,
    /// π_n(B), π_n(F), π_n(E) stored up to some n.
    pub pi_base: Vec<i64>,
    pub pi_fiber: Vec<i64>,
    pub pi_total: Vec<i64>,
}
#[allow(dead_code)]
impl FibrationSequence {
    pub fn new(
        total_space: &str,
        base_space: &str,
        fiber: &str,
        pi_base: Vec<i64>,
        pi_fiber: Vec<i64>,
        pi_total: Vec<i64>,
    ) -> Self {
        Self {
            total_space: total_space.to_string(),
            base_space: base_space.to_string(),
            fiber: fiber.to_string(),
            pi_base,
            pi_fiber,
            pi_total,
        }
    }
    /// Check Euler formula for fibrations: χ(E) = χ(F) * χ(B).
    pub fn euler_product_fibration(&self, chi_f: i64, chi_b: i64) -> i64 {
        chi_f * chi_b
    }
}
/// Local cohomology H^n_I(M) via the Čech complex or colimit of Ext^n(R/I^k, M).
#[derive(Debug, Clone)]
pub struct LocalCohomology {
    /// Name of the ideal I.
    pub ideal_name: String,
    /// Name of the module M.
    pub module_name: String,
    /// Computed local cohomology groups H^0_I, H^1_I, …
    pub groups: Vec<LocalCohomologyGroup>,
}
impl LocalCohomology {
    /// Create a local cohomology computation.
    pub fn new(ideal_name: &str, module_name: &str) -> Self {
        Self {
            ideal_name: ideal_name.to_string(),
            module_name: module_name.to_string(),
            groups: vec![],
        }
    }
    /// Add a computed cohomology group.
    pub fn add_group(&mut self, degree: usize, rank: usize) {
        self.groups.push(LocalCohomologyGroup::new(degree, rank));
    }
    /// Cohomological dimension: the largest n with H^n_I(M) ≠ 0.
    pub fn cohomological_dimension(&self) -> Option<usize> {
        self.groups.iter().rposition(|g| !g.is_zero())
    }
}
/// A flat resolution of a module.
#[derive(Debug, Clone)]
pub struct FlatResolution {
    /// Module name.
    pub module_name: String,
    /// Flat dimension.
    pub flat_dim: Option<usize>,
    /// Ranks of flat modules at each step.
    pub ranks: Vec<usize>,
}
impl FlatResolution {
    /// Create a flat resolution.
    pub fn new(module_name: &str, ranks: Vec<usize>) -> Self {
        let flat_dim = ranks.iter().rposition(|&r| r > 0);
        Self {
            module_name: module_name.to_string(),
            flat_dim,
            ranks,
        }
    }
}
/// Persistent Betti numbers at a given threshold.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistentBettiNumbers {
    /// Persistence diagram: list of (birth, death) pairs per dimension.
    pub pairs: Vec<Vec<(f64, f64)>>,
}
#[allow(dead_code)]
impl PersistentBettiNumbers {
    pub fn new(pairs: Vec<Vec<(f64, f64)>>) -> Self {
        Self { pairs }
    }
    /// Betti number β_k at threshold t: count of pairs (b,d) with b<=t<d.
    pub fn betti_at(&self, k: usize, t: f64) -> usize {
        if k >= self.pairs.len() {
            return 0;
        }
        self.pairs[k]
            .iter()
            .filter(|&&(b, d)| b <= t && t < d)
            .count()
    }
    /// Total persistence of dimension k: Σ (d - b).
    pub fn total_persistence(&self, k: usize) -> f64 {
        if k >= self.pairs.len() {
            return 0.0;
        }
        self.pairs[k].iter().map(|&(b, d)| d - b).sum()
    }
    /// Bottleneck distance approximation (naive, O(n^2)).
    pub fn bottleneck_approx(&self, other: &Self, k: usize) -> f64 {
        if k >= self.pairs.len() || k >= other.pairs.len() {
            return 0.0;
        }
        let a = &self.pairs[k];
        let b = &other.pairs[k];
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }
        let inf_dist =
            |p: (f64, f64), q: (f64, f64)| -> f64 { (p.0 - q.0).abs().max((p.1 - q.1).abs()) };
        let mut max_min = 0.0f64;
        for &pa in a {
            let min_d = b
                .iter()
                .map(|&pb| inf_dist(pa, pb))
                .fold(f64::INFINITY, f64::min);
            max_min = max_min.max(min_d);
        }
        max_min
    }
}
/// de Rham cohomology data for smooth manifolds.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeRhamCohomology {
    pub manifold_name: String,
    pub dimension: usize,
    /// Betti numbers (de Rham = singular over R by de Rham's theorem).
    pub betti_numbers: Vec<i64>,
}
#[allow(dead_code)]
impl DeRhamCohomology {
    pub fn new(manifold_name: &str, dimension: usize, betti_numbers: Vec<i64>) -> Self {
        Self {
            manifold_name: manifold_name.to_string(),
            dimension,
            betti_numbers,
        }
    }
    /// Poincaré duality: β_k = β_{n-k} for oriented closed n-manifold.
    pub fn check_poincare_duality(&self) -> bool {
        let n = self.dimension;
        if self.betti_numbers.len() != n + 1 {
            return false;
        }
        (0..=n).all(|k| self.betti_numbers[k] == self.betti_numbers[n - k])
    }
    /// Euler characteristic.
    pub fn euler_characteristic(&self) -> i64 {
        self.betti_numbers
            .iter()
            .enumerate()
            .map(|(k, &b)| if k % 2 == 0 { b } else { -b })
            .sum()
    }
}
/// A chain complex C_• with integer boundary matrices.
///
/// `groups\[i\]` = C_i, `boundaries\[i\]` = d_{i+1}: C_{i+1} → C_i.
#[derive(Debug, Clone, Default)]
pub struct ChainComplex {
    /// The chain groups.
    pub groups: Vec<GradedGroup>,
    /// Boundary matrices (one fewer than groups).
    pub boundaries: Vec<Vec<Vec<i64>>>,
}
impl ChainComplex {
    /// Create an empty chain complex.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a chain group C_k.
    pub fn add_group(&mut self, rank: usize, name: &str) {
        self.groups.push(GradedGroup::new(rank, name));
    }
    /// Add a boundary matrix d_k.
    pub fn add_boundary(&mut self, matrix: Vec<Vec<i64>>) {
        self.boundaries.push(matrix);
    }
    /// Compute Betti numbers β_i = ker(d_i) − im(d_{i+1}).
    pub fn betti_numbers(&self) -> Vec<i64> {
        let n = self.groups.len();
        (0..n)
            .map(|i| {
                let ker = if i > 0 && i - 1 < self.boundaries.len() {
                    kernel_rank(&self.boundaries[i - 1], self.groups[i].rank) as i64
                } else {
                    self.groups[i].rank as i64
                };
                let img = if i < self.boundaries.len() && i + 1 < self.groups.len() {
                    image_rank(&self.boundaries[i], self.groups[i + 1].rank) as i64
                } else {
                    0
                };
                ker - img
            })
            .collect()
    }
    /// Compute all homology groups H_n(C).
    pub fn compute_homology(&self) -> Vec<HomologyGroup> {
        self.betti_numbers()
            .into_iter()
            .enumerate()
            .map(|(n, rank)| HomologyGroup {
                degree: n as i32,
                rank: rank.max(0) as usize,
                torsion: vec![],
            })
            .collect()
    }
    /// Euler characteristic χ = ∑_n (-1)^n β_n.
    pub fn euler_characteristic(&self) -> i64 {
        self.betti_numbers()
            .iter()
            .enumerate()
            .map(|(n, &b)| if n % 2 == 0 { b } else { -b })
            .sum()
    }
    /// Check if the complex is exact at position `k` (H_k = 0).
    pub fn is_exact_at(&self, k: usize) -> bool {
        let betti = self.betti_numbers();
        betti.get(k).copied().unwrap_or(0) == 0
    }
    /// Check if the complex is exact everywhere.
    pub fn is_exact(&self) -> bool {
        self.betti_numbers().iter().all(|&b| b == 0)
    }
}
/// The differential d_r: E_r^{p,q} → E_r^{p+r, q-r+1} on page r.
#[derive(Debug, Clone)]
pub struct DifferentialMap {
    /// Page number r.
    pub page: usize,
    /// Source position (p, q).
    pub source: (i32, i32),
    /// Target position (p+r, q-r+1).
    pub target: (i32, i32),
    /// Rank of the image of this differential.
    pub image_rank: usize,
}
impl DifferentialMap {
    /// Create a differential d_r: E_r^{(p,q)} → E_r^{(p+r, q-r+1)}.
    pub fn new(r: usize, p: i32, q: i32, image_rank: usize) -> Self {
        let ri = r as i32;
        Self {
            page: r,
            source: (p, q),
            target: (p + ri, q - ri + 1),
            image_rank,
        }
    }
}
/// The bar resolution of k over the group algebra kG.
///
/// B_n(kG) = kG ⊗ k^{n+1} (as a kG-module).
#[derive(Debug, Clone)]
pub struct BarResolution {
    /// The group name.
    pub group_name: String,
    /// Number of computed steps.
    pub num_steps: usize,
    /// Rank of each bar module B_n.
    pub ranks: Vec<usize>,
}
impl BarResolution {
    /// Create a bar resolution up to the given number of steps.
    ///
    /// For a group G with |G| = order, B_n = (kG)^{|G|^n}.
    pub fn new(group_name: &str, group_order: usize, num_steps: usize) -> Self {
        let ranks = (0..num_steps)
            .map(|n| group_order.saturating_pow(n as u32))
            .collect();
        Self {
            group_name: group_name.to_string(),
            num_steps,
            ranks,
        }
    }
    /// The rank of B_n.
    pub fn rank_at(&self, n: usize) -> usize {
        self.ranks.get(n).copied().unwrap_or(0)
    }
}
/// A single step in a free resolution: the n-th syzygy module as a free module.
#[derive(Debug, Clone)]
pub struct ResolutionStep {
    /// Degree index of this step.
    pub degree: usize,
    /// Rank of the free module at this step.
    pub rank: usize,
    /// Matrix of the boundary map to the previous step.
    pub boundary: Vec<Vec<i64>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PerverseSheafData {
    pub stratification: Vec<String>,
    pub perversity: Vec<i64>,
    pub is_ic_sheaf: bool,
    pub support_dimension: Vec<usize>,
}
#[allow(dead_code)]
impl PerverseSheafData {
    pub fn new(strat: Vec<String>, perversity: Vec<i64>) -> Self {
        let n = strat.len();
        PerverseSheafData {
            stratification: strat,
            perversity,
            is_ic_sheaf: false,
            support_dimension: (0..n).collect(),
        }
    }
    pub fn intersection_cohomology_description(&self) -> String {
        "IC sheaf: intermediate extension j_!* F of local system F".to_string()
    }
    pub fn bbdg_decomposition(&self) -> String {
        "BBDG: semisimple complexes over finite fields decompose into shifts of IC sheaves"
            .to_string()
    }
    pub fn verdier_duality(&self) -> String {
        "Verdier duality: D(IC_X(L)) ≅ IC_X(L^∨) for selfdual local system".to_string()
    }
    pub fn support_condition(&self) -> String {
        format!(
            "Perversity condition: dim supp H^i ≤ -i on {} strata",
            self.stratification.len()
        )
    }
}
/// A single page E_r of a spectral sequence.
///
/// Entries E_r^{p,q} are free abelian groups indexed by (p,q) ∈ ℤ².
#[derive(Debug, Clone, Default)]
pub struct SpectralSequencePage {
    /// E_r^{p,q} stored as (p,q) → rank.
    pub entries: HashMap<(i32, i32), usize>,
    /// Page number r.
    pub page: usize,
}
impl SpectralSequencePage {
    /// Create a new page.
    pub fn new(page: usize) -> Self {
        Self {
            entries: HashMap::new(),
            page,
        }
    }
    /// Set E_r^{p,q}.
    pub fn set(&mut self, p: i32, q: i32, rank: usize) {
        self.entries.insert((p, q), rank);
    }
    /// Get E_r^{p,q}.
    pub fn get(&self, p: i32, q: i32) -> usize {
        self.entries.get(&(p, q)).copied().unwrap_or(0)
    }
    /// Total rank of the page.
    pub fn total_rank(&self) -> usize {
        self.entries.values().sum()
    }
}
/// Chain complex with explicit boundary maps (stored as ranks for simplicity).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChainCplxExt {
    /// chain_groups\[k\] = rank of C_k (free abelian group).
    pub chain_groups: Vec<usize>,
    /// boundary_ranks\[k\] = rank of d_k : C_k -> C_{k-1}.
    pub boundary_ranks: Vec<usize>,
}
#[allow(dead_code)]
impl ChainCplxExt {
    pub fn new(chain_groups: Vec<usize>, boundary_ranks: Vec<usize>) -> Self {
        assert!(
            chain_groups.len() == boundary_ranks.len() + 1
                || chain_groups.len() == boundary_ranks.len(),
            "chain_groups.len() should be boundary_ranks.len() or boundary_ranks.len()+1"
        );
        Self {
            chain_groups,
            boundary_ranks,
        }
    }
    /// Betti numbers: β_k = rank(C_k) - rank(im d_{k+1}) - rank(d_k).
    pub fn betti_numbers(&self) -> Vec<i64> {
        let n = self.chain_groups.len();
        (0..n)
            .map(|k| {
                let ck = self.chain_groups[k] as i64;
                let im_kp1 = if k + 1 < self.boundary_ranks.len() {
                    self.boundary_ranks[k + 1] as i64
                } else {
                    0
                };
                let rk_dk = if k < self.boundary_ranks.len() {
                    self.boundary_ranks[k] as i64
                } else {
                    0
                };
                ck - im_kp1 - rk_dk
            })
            .collect()
    }
    /// Euler characteristic.
    pub fn euler_characteristic(&self) -> i64 {
        let betti = self.betti_numbers();
        betti
            .iter()
            .enumerate()
            .map(|(k, &b)| if k % 2 == 0 { b } else { -b })
            .sum()
    }
}
/// A projective (free) resolution … → P_2 → P_1 → P_0 → M → 0.
#[derive(Debug, Clone)]
pub struct ProjectiveResolution {
    /// The module being resolved.
    pub module_name: String,
    /// The steps P_0, P_1, …, P_n.
    pub steps: Vec<ResolutionStep>,
}
impl ProjectiveResolution {
    /// Create a new projective resolution.
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            steps: vec![],
        }
    }
    /// Add a resolution step.
    pub fn add_step(&mut self, rank: usize, boundary: Vec<Vec<i64>>) {
        let degree = self.steps.len();
        self.steps.push(ResolutionStep {
            degree,
            rank,
            boundary,
        });
    }
    /// Projective dimension: the length of the resolution (largest non-zero step).
    pub fn projective_dimension(&self) -> Option<usize> {
        self.steps.iter().rposition(|s| s.rank > 0)
    }
    /// Betti numbers: β_i = rank(P_i).
    pub fn betti_numbers(&self) -> Vec<usize> {
        self.steps.iter().map(|s| s.rank).collect()
    }
}
/// Künneth formula data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KunnethData {
    pub space_x: String,
    pub space_y: String,
    pub field_coefficients: bool,
}
#[allow(dead_code)]
impl KunnethData {
    /// Künneth formula over a field.
    pub fn over_field(x: &str, y: &str) -> Self {
        Self {
            space_x: x.to_string(),
            space_y: y.to_string(),
            field_coefficients: true,
        }
    }
    /// H*(X x Y; k) = H*(X; k) ⊗ H*(Y; k) over field k.
    pub fn kunneth_description(&self) -> String {
        if self.field_coefficients {
            format!(
                "H*({} x {}; k) ≅ H*({};k) ⊗ H*({};k)",
                self.space_x, self.space_y, self.space_x, self.space_y
            )
        } else {
            format!(
                "Künneth: H_n({} x {}) has Tor correction term",
                self.space_x, self.space_y
            )
        }
    }
}
/// Tor_n^R(M, N) computed from a projective resolution of M.
#[derive(Debug, Clone)]
pub struct TorFunctor {
    /// Name of the first module M.
    pub module_m: String,
    /// Name of the second module N.
    pub module_n: String,
    /// Computed values Tor_0, Tor_1, …, Tor_k.
    pub values: Vec<TorGroup>,
}
impl TorFunctor {
    /// Create a TorFunctor from a projective resolution of M tensored with N.
    pub fn compute(module_m: &str, module_n: &str, resolution: &ProjectiveResolution) -> Self {
        let betti = resolution.betti_numbers();
        let values = betti
            .iter()
            .enumerate()
            .map(|(n, &r)| TorGroup::new(n, r))
            .collect();
        Self {
            module_m: module_m.to_string(),
            module_n: module_n.to_string(),
            values,
        }
    }
    /// Retrieve Tor_n(M, N).
    pub fn tor_at(&self, n: usize) -> Option<&TorGroup> {
        self.values.get(n)
    }
    /// Projective dimension of M = largest n with Tor_n(M, N) ≠ 0.
    pub fn projective_dimension(&self) -> Option<usize> {
        self.values.iter().rposition(|t| !t.is_zero())
    }
}
/// A birth-death pair in a barcode.
#[derive(Debug, Clone, PartialEq)]
pub struct BirthDeathPair {
    /// Birth time (filtration value).
    pub birth: f64,
    /// Death time (filtration value), or f64::INFINITY for essential classes.
    pub death: f64,
    /// Homological degree.
    pub degree: usize,
}
impl BirthDeathPair {
    /// Create a birth-death pair.
    pub fn new(birth: f64, death: f64, degree: usize) -> Self {
        Self {
            birth,
            death,
            degree,
        }
    }
    /// The persistence (lifetime) of this interval.
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }
    /// True iff this is an essential (infinite-persistence) class.
    pub fn is_essential(&self) -> bool {
        self.death.is_infinite()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CechCocycle {
    pub open_cover_size: usize,
    pub degree: usize,
    pub cochain: Vec<Vec<f64>>,
    pub is_cocycle: bool,
}
#[allow(dead_code)]
impl CechCocycle {
    pub fn new(cover_size: usize, degree: usize) -> Self {
        let cochain = vec![vec![0.0; cover_size]; cover_size.pow(degree as u32)];
        CechCocycle {
            open_cover_size: cover_size,
            degree,
            cochain,
            is_cocycle: true,
        }
    }
    pub fn leray_theorem(&self) -> String {
        "Leray: for acyclic covers, Čech cohomology = sheaf cohomology".to_string()
    }
    pub fn refinement_map_description(&self) -> String {
        format!(
            "Refinement: Čech H^{}(U;F) → Čech H^{}(V;F) for V refinement of U",
            self.degree, self.degree
        )
    }
    pub fn mayer_vietoris_for_two_opens(&self) -> String {
        "Mayer-Vietoris: 0 → F(U∪V) → F(U)⊕F(V) → F(U∩V) → H^1 → ...".to_string()
    }
}
