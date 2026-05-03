//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::functions::*;

/// Computational record for Fourier-Mukai transforms between derived categories.
///
/// Tracks the kernel object and the derived pushforward/pullback operations
/// in terms of sheaf cohomology ranks.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FourierMukaiData {
    /// Name of the source scheme X.
    pub source_scheme: String,
    /// Name of the target scheme Y.
    pub target_scheme: String,
    /// Kernel object label P ∈ D^b(X×Y).
    pub kernel_label: String,
    /// Cohomology ranks H^n(X, P|_{X×y}) for a generic point y,
    /// indexed by cohomological degree n.
    pub source_cohomology: Vec<usize>,
    /// Cohomology ranks H^n(Y, P|_{x×Y}) for a generic point x.
    pub target_cohomology: Vec<usize>,
}
impl FourierMukaiData {
    /// Create a new Fourier-Mukai data record.
    pub fn new(source: &str, target: &str, kernel: &str) -> Self {
        Self {
            source_scheme: source.to_string(),
            target_scheme: target.to_string(),
            kernel_label: kernel.to_string(),
            source_cohomology: Vec::new(),
            target_cohomology: Vec::new(),
        }
    }
    /// Set the source-fibre cohomology ranks.
    pub fn set_source_cohomology(&mut self, ranks: Vec<usize>) {
        self.source_cohomology = ranks;
    }
    /// Set the target-fibre cohomology ranks.
    pub fn set_target_cohomology(&mut self, ranks: Vec<usize>) {
        self.target_cohomology = ranks;
    }
    /// Check the point-object condition on the source side:
    /// H^0 = 1 and H^n = 0 for n ≠ 0.
    pub fn is_point_like_source(&self) -> bool {
        self.source_cohomology.iter().enumerate().all(
            |(n, &r)| {
                if n == 0 {
                    r == 1
                } else {
                    r == 0
                }
            },
        )
    }
    /// Check the point-object condition on the target side.
    pub fn is_point_like_target(&self) -> bool {
        self.target_cohomology.iter().enumerate().all(
            |(n, &r)| {
                if n == 0 {
                    r == 1
                } else {
                    r == 0
                }
            },
        )
    }
    /// A sufficient criterion for Φ_P to be an equivalence:
    /// point-like on both sides (Bondal-Van den Bergh type criterion).
    pub fn is_likely_equivalence(&self) -> bool {
        self.is_point_like_source() && self.is_point_like_target()
    }
    /// Euler characteristic χ(P|_{X×y}) = Σ (-1)^n rank H^n.
    pub fn source_euler_characteristic(&self) -> i64 {
        self.source_cohomology
            .iter()
            .enumerate()
            .map(|(n, &r)| if n % 2 == 0 { r as i64 } else { -(r as i64) })
            .sum()
    }
}
/// A finite model-category-like structure tracking the three classes of morphisms
/// (weak equivalences, fibrations, cofibrations) on a finite set of morphisms.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ModelCategoryData {
    /// Object labels in this (finite presentation of a) model category.
    pub objects: Vec<String>,
    /// All morphisms as (source_idx, target_idx, label).
    pub morphisms: Vec<(usize, usize, String)>,
    /// Indices of morphisms that are weak equivalences.
    pub weak_equivalences: Vec<usize>,
    /// Indices of morphisms that are fibrations.
    pub fibrations: Vec<usize>,
    /// Indices of morphisms that are cofibrations.
    pub cofibrations: Vec<usize>,
}
impl ModelCategoryData {
    /// Create an empty model category.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an object, returning its index.
    pub fn add_object(&mut self, name: &str) -> usize {
        let idx = self.objects.len();
        self.objects.push(name.to_string());
        idx
    }
    /// Add a morphism from `src` to `tgt` with label, returning its index.
    pub fn add_morphism(&mut self, src: usize, tgt: usize, label: &str) -> usize {
        let idx = self.morphisms.len();
        self.morphisms.push((src, tgt, label.to_string()));
        idx
    }
    /// Mark morphism `m_idx` as a weak equivalence.
    pub fn mark_weak_equivalence(&mut self, m_idx: usize) {
        if !self.weak_equivalences.contains(&m_idx) {
            self.weak_equivalences.push(m_idx);
        }
    }
    /// Mark morphism `m_idx` as a fibration.
    pub fn mark_fibration(&mut self, m_idx: usize) {
        if !self.fibrations.contains(&m_idx) {
            self.fibrations.push(m_idx);
        }
    }
    /// Mark morphism `m_idx` as a cofibration.
    pub fn mark_cofibration(&mut self, m_idx: usize) {
        if !self.cofibrations.contains(&m_idx) {
            self.cofibrations.push(m_idx);
        }
    }
    /// Check whether morphism `m_idx` is an acyclic fibration
    /// (weak equivalence AND fibration).
    pub fn is_acyclic_fibration(&self, m_idx: usize) -> bool {
        self.weak_equivalences.contains(&m_idx) && self.fibrations.contains(&m_idx)
    }
    /// Check whether morphism `m_idx` is an acyclic cofibration
    /// (weak equivalence AND cofibration).
    pub fn is_acyclic_cofibration(&self, m_idx: usize) -> bool {
        self.weak_equivalences.contains(&m_idx) && self.cofibrations.contains(&m_idx)
    }
    /// An object is cofibrant if all incoming morphisms from the initial
    /// object position (index 0) are cofibrations.
    pub fn is_cofibrant(&self, obj_idx: usize) -> bool {
        self.morphisms
            .iter()
            .enumerate()
            .filter(|(_, (src, tgt, _))| *src == 0 && *tgt == obj_idx)
            .all(|(m_idx, _)| self.cofibrations.contains(&m_idx))
    }
    /// Count weak equivalences, fibrations, cofibrations.
    pub fn counts(&self) -> (usize, usize, usize) {
        (
            self.weak_equivalences.len(),
            self.fibrations.len(),
            self.cofibrations.len(),
        )
    }
}
/// A chain complex C_• with boundary maps d_n : C_n → C_{n-1}.
///
/// Stored as a sequence of groups and the matrices representing the boundary maps.
/// `boundaries\[i\]` is the matrix of d_{i+1} : C_{i+1} → C_i (rows = rank C_i).
#[derive(Debug, Clone, Default)]
pub struct ChainComplex {
    /// The chain groups C_0, C_1, …, C_n.
    pub groups: Vec<ChainGroup>,
    /// Boundary matrices d_1, d_2, …, d_n (one fewer than groups).
    pub boundaries: Vec<Vec<Vec<i64>>>,
}
impl ChainComplex {
    /// Create an empty chain complex.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a chain group C_k with the given rank and label.
    pub fn add_group(&mut self, rank: usize, name: &str) {
        self.groups.push(ChainGroup::new(rank, name));
    }
    /// Append a boundary matrix d_k : C_k → C_{k-1}.
    ///
    /// The matrix is stored as a list of rows (each row has `rank(C_{k-1})` entries
    /// and there are `rank(C_k)` columns).
    pub fn add_boundary(&mut self, matrix: Vec<Vec<i64>>) {
        self.boundaries.push(matrix);
    }
    /// Compute the Betti numbers β_n = dim ker(d_n) - dim im(d_{n+1}).
    ///
    /// Convention: `boundaries\[k\]` = d_{k+1}: C_{k+1} → C_k.
    /// So d_i = boundaries[i-1] and d_{i+1} = boundaries\[i\].
    ///
    /// Returns one Betti number per chain group.
    pub fn compute_betti_numbers(&self) -> Vec<i64> {
        let n = self.groups.len();
        (0..n)
            .map(|i| {
                let ker = if i > 0 && i - 1 < self.boundaries.len() {
                    let d = &self.boundaries[i - 1];
                    let cols = self.groups[i].rank;
                    kernel_rank(d, cols) as i64
                } else {
                    self.groups[i].rank as i64
                };
                let img = if i < self.boundaries.len() && i + 1 < self.groups.len() {
                    let d = &self.boundaries[i];
                    let cols = self.groups[i + 1].rank;
                    image_rank(d, cols) as i64
                } else {
                    0
                };
                ker - img
            })
            .collect()
    }
    /// Compute the Euler characteristic χ = ∑_n (-1)^n β_n.
    pub fn euler_characteristic(&self) -> i64 {
        self.compute_betti_numbers()
            .iter()
            .enumerate()
            .map(|(i, &b)| if i % 2 == 0 { b } else { -b })
            .sum()
    }
    /// Check whether the complex is exact at position n (0-indexed).
    ///
    /// Exactness at C_n means im(d_{n+1}) = ker(d_n), i.e. β_n = 0.
    pub fn is_exact_at(&self, n: usize) -> bool {
        let betti = self.compute_betti_numbers();
        betti.get(n).copied().unwrap_or(0) == 0
    }
}
/// A single graded component C_n of a chain complex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainGroup {
    /// The rank (number of generators) of this free abelian group.
    pub rank: usize,
    /// A human-readable label (e.g. "C_2").
    pub name: String,
}
impl ChainGroup {
    /// Create a new chain group with the given rank and label.
    pub fn new(rank: usize, name: &str) -> Self {
        Self {
            rank,
            name: name.to_string(),
        }
    }
}
/// A spectral sequence {E^r_{p,q}} converging to H_{p+q}.
///
/// Each page is a bigraded collection of abelian-group ranks.
#[derive(Debug, Clone, Default)]
pub struct SpectralSequence {
    /// The pages E^r, indexed from r=0. `pages\[r\]\[(p,q)\]` = rank of E^r_{p,q}.
    pub pages: Vec<HashMap<(i32, i32), usize>>,
    /// The current (highest stored) page number.
    pub page_num: usize,
}
impl SpectralSequence {
    /// Create an empty spectral sequence.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a new page to the spectral sequence.
    pub fn add_page(&mut self, page: HashMap<(i32, i32), usize>) {
        self.pages.push(page);
        self.page_num = self.pages.len().saturating_sub(1);
    }
    /// Look up the rank of E^r_{p,q} on a given page.
    ///
    /// Returns `None` if the page or position has not been recorded.
    pub fn e_term(&self, page: usize, p: i32, q: i32) -> Option<usize> {
        self.pages.get(page)?.get(&(p, q)).copied()
    }
    /// Return the page number r at which the spectral sequence stabilises.
    ///
    /// The sequence converges at the first page r where consecutive pages are
    /// identical (i.e. all differentials d^r are zero).  If fewer than two
    /// pages have been stored the last recorded page is returned.
    pub fn converges_at(&self) -> usize {
        if self.pages.len() < 2 {
            return self.page_num;
        }
        for r in 1..self.pages.len() {
            if self.pages[r] == self.pages[r - 1] {
                return r;
            }
        }
        self.page_num
    }
}
/// A DG-category representation storing morphism complexes between objects.
///
/// The morphism complex Hom(X, Y) is stored as a `ChainComplex`.
#[derive(Debug, Clone, Default)]
pub struct DGCategoryData {
    /// Object labels.
    pub objects: Vec<String>,
    /// Morphism complexes: `hom_complexes\[(i, j)\]` = Hom(objects\[i\], objects\[j\]).
    pub hom_complexes: HashMap<(usize, usize), ChainComplex>,
}
impl DGCategoryData {
    /// Create an empty DG-category.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an object and return its index.
    pub fn add_object(&mut self, name: &str) -> usize {
        let idx = self.objects.len();
        self.objects.push(name.to_string());
        idx
    }
    /// Set the morphism complex Hom(i, j).
    pub fn set_hom(&mut self, i: usize, j: usize, complex: ChainComplex) {
        self.hom_complexes.insert((i, j), complex);
    }
    /// Get the morphism complex Hom(i, j), if defined.
    pub fn get_hom(&self, i: usize, j: usize) -> Option<&ChainComplex> {
        self.hom_complexes.get(&(i, j))
    }
    /// Compute Ext^n(i, j) = H^n(Hom(i, j)) as a Betti number.
    ///
    /// Returns `None` if the morphism complex is not defined.
    pub fn ext_degree(&self, i: usize, j: usize, n: usize) -> Option<i64> {
        let cx = self.get_hom(i, j)?;
        let betti = cx.compute_betti_numbers();
        betti.get(n).copied()
    }
    /// Check whether Ext^n for n > 0 all vanish (quasi-isomorphic objects).
    pub fn are_quasi_isomorphic(&self, i: usize, j: usize) -> bool {
        if let Some(cx) = self.get_hom(i, j) {
            let betti = cx.compute_betti_numbers();
            betti.iter().enumerate().all(|(n, &b)| n == 0 || b == 0)
        } else {
            false
        }
    }
}
/// A Chow group representation tracking algebraic cycles modulo rational equivalence.
///
/// Stores cycles as formal integer linear combinations of subvariety labels.
#[derive(Debug, Clone, Default)]
pub struct ChowGroupData {
    /// The ambient scheme name.
    pub scheme: String,
    /// Codimension of the cycles.
    pub codimension: usize,
    /// Subvariety generators: label → coefficient in the cycle group.
    pub cycles: HashMap<String, i64>,
}
impl ChowGroupData {
    /// Create a new Chow group CH^p(X).
    pub fn new(scheme: &str, codimension: usize) -> Self {
        Self {
            scheme: scheme.to_string(),
            codimension,
            cycles: HashMap::new(),
        }
    }
    /// Add a cycle n·\[Z\] to the Chow group.
    pub fn add_cycle(&mut self, subvariety: &str, coefficient: i64) {
        let entry = self.cycles.entry(subvariety.to_string()).or_insert(0);
        *entry += coefficient;
    }
    /// Compute the degree of the zero-cycle (sum of all coefficients).
    pub fn degree(&self) -> i64 {
        self.cycles.values().sum()
    }
    /// Check whether this cycle is numerically zero (all coefficients are zero).
    pub fn is_zero(&self) -> bool {
        self.cycles.values().all(|&c| c == 0)
    }
    /// Return the intersection number of two zero-cycles (product of degrees).
    pub fn intersection_number(&self, other: &ChowGroupData) -> i64 {
        self.degree() * other.degree()
    }
    /// Apply the cycle class map to ℤ: returns the degree as an integer cohomology class.
    pub fn cycle_class(&self) -> i64 {
        self.degree()
    }
}
/// A spectral sequence page with differentials, supporting computation of E^{r+1}.
#[derive(Debug, Clone, Default)]
pub struct SpectralSequencePage {
    /// The page number r.
    pub r: usize,
    /// E^r_{p,q} groups: map from (p,q) to rank.
    pub groups: HashMap<(i32, i32), usize>,
    /// Differentials d^r_{p,q}: (source (p,q), target, image rank).
    pub differentials: Vec<((i32, i32), (i32, i32), usize)>,
}
impl SpectralSequencePage {
    /// Create a new page with page number r.
    pub fn new(r: usize) -> Self {
        Self {
            r,
            groups: HashMap::new(),
            differentials: Vec::new(),
        }
    }
    /// Set the rank of E^r_{p,q}.
    pub fn set_group(&mut self, p: i32, q: i32, rank: usize) {
        self.groups.insert((p, q), rank);
    }
    /// Record a differential d^r : E^r_{p,q} → E^r_{p-r, q+r-1} with given image rank.
    pub fn add_differential(&mut self, p: i32, q: i32, image_rank: usize) {
        let target = (p - self.r as i32, q + self.r as i32 - 1);
        self.differentials.push(((p, q), target, image_rank));
    }
    /// Compute the next page E^{r+1}_{p,q} = ker(d^r_{p,q}) / im(d^r_{p+r, q-r+1}).
    ///
    /// Returns a map (p,q) → rank of E^{r+1}_{p,q}.
    pub fn compute_next_page(&self) -> HashMap<(i32, i32), usize> {
        let mut next: HashMap<(i32, i32), usize> = HashMap::new();
        for (&(p, q), &rank) in &self.groups {
            let incoming_src = (p + self.r as i32, q - self.r as i32 + 1);
            let incoming_im: usize = self
                .differentials
                .iter()
                .filter(|(src, tgt, _)| *src == incoming_src && *tgt == (p, q))
                .map(|(_, _, im)| *im)
                .sum();
            let outgoing_im: usize = self
                .differentials
                .iter()
                .filter(|(src, _, _)| *src == (p, q))
                .map(|(_, _, im)| *im)
                .sum();
            let ker_rank = rank.saturating_sub(outgoing_im);
            let next_rank = ker_rank.saturating_sub(incoming_im);
            next.insert((p, q), next_rank);
        }
        next
    }
    /// Check whether this page has already degenerated (all differentials are zero).
    pub fn is_degenerate(&self) -> bool {
        self.differentials.iter().all(|(_, _, im)| *im == 0)
    }
}
/// An Ext group Ext^n(M, N) represented by its degree and rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtGroup {
    /// Name of the first module M.
    pub m_name: String,
    /// Name of the second module N.
    pub n_name: String,
    /// The cohomological degree n.
    pub degree: usize,
    /// The rank of Ext^n(M, N) as an abelian group.
    pub rank: usize,
}
impl ExtGroup {
    /// Create a new Ext group.
    pub fn new(m: &str, n: &str, degree: usize, rank: usize) -> Self {
        Self {
            m_name: m.to_string(),
            n_name: n.to_string(),
            degree,
            rank,
        }
    }
    /// An Ext group is zero iff its rank is 0.
    pub fn is_zero(&self) -> bool {
        self.rank == 0
    }
}
/// Tilting theory data for a finite-dimensional algebra, tracking the
/// tilting module T, its endomorphism algebra B = End_A(T), and the
/// derived equivalence data.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TiltingTheoryData {
    /// Name of the original algebra A.
    pub algebra_a: String,
    /// Name of the tilting module T (as an A-module).
    pub tilting_module: String,
    /// Computed name of the endomorphism algebra B = End_A(T).
    pub algebra_b: String,
    /// Indecomposable summands of T.
    pub indecomposable_summands: Vec<String>,
    /// Self-Ext vanishing: Ext^n_A(T, T) for n = 1, 2, …
    pub self_ext_ranks: Vec<usize>,
}
impl TiltingTheoryData {
    /// Create a new tilting theory record.
    pub fn new(algebra_a: &str, tilting_module: &str) -> Self {
        Self {
            algebra_a: algebra_a.to_string(),
            tilting_module: tilting_module.to_string(),
            algebra_b: format!("End_{}({})", algebra_a, tilting_module),
            indecomposable_summands: Vec::new(),
            self_ext_ranks: Vec::new(),
        }
    }
    /// Add an indecomposable summand.
    pub fn add_summand(&mut self, summand: &str) {
        self.indecomposable_summands.push(summand.to_string());
    }
    /// Record Ext^n(T,T) rank for n = 1, 2, …
    pub fn set_self_ext(&mut self, n: usize, rank: usize) {
        if n == 0 {
            return;
        }
        let idx = n - 1;
        if idx >= self.self_ext_ranks.len() {
            self.self_ext_ranks.resize(idx + 1, 0);
        }
        self.self_ext_ranks[idx] = rank;
    }
    /// Check the tilting condition: Ext^n_A(T, T) = 0 for all n > 0.
    pub fn satisfies_tilting_condition(&self) -> bool {
        self.self_ext_ranks.iter().all(|&r| r == 0)
    }
    /// Number of indecomposable summands (should equal # of simple A-modules).
    pub fn n_summands(&self) -> usize {
        self.indecomposable_summands.len()
    }
    /// A naive check that T is a progenerator: Hom_A(T, A) generates mod-B.
    /// Here we approximate this by checking that the module is nonzero.
    pub fn is_progenerator(&self) -> bool {
        !self.indecomposable_summands.is_empty()
    }
}
/// A triangulated category record tracking distinguished triangles and
/// the suspension autoequivalence on a finite set of objects.
#[derive(Debug, Clone, Default)]
pub struct TriangulatedCategoryData {
    /// Object labels in this category.
    pub objects: Vec<String>,
    /// Distinguished triangles stored as (X_idx, Y_idx, Z_idx).
    pub triangles: Vec<(usize, usize, usize)>,
}
impl TriangulatedCategoryData {
    /// Create a new triangulated category with no objects or triangles.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an object and return its index.
    pub fn add_object(&mut self, name: &str) -> usize {
        let idx = self.objects.len();
        self.objects.push(name.to_string());
        idx
    }
    /// Record a distinguished triangle (X, Y, Z).
    ///
    /// Panics if any index is out of range.
    pub fn add_triangle(&mut self, x: usize, y: usize, z: usize) {
        assert!(
            x < self.objects.len() && y < self.objects.len() && z < self.objects.len(),
            "triangle vertex index out of range"
        );
        self.triangles.push((x, y, z));
    }
    /// Check whether a triple (x, y, z) forms a distinguished triangle.
    pub fn is_distinguished(&self, x: usize, y: usize, z: usize) -> bool {
        self.triangles.contains(&(x, y, z))
    }
    /// Return the number of recorded distinguished triangles.
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
    /// Verify the octahedral axiom for a composable pair:
    /// given triangles on cone_xy, cone_xz, cone_yz, check that
    /// (cone_xy, cone_xz, cone_yz) is also a distinguished triangle.
    pub fn check_octahedral(&self, cone_xy: usize, cone_xz: usize, cone_yz: usize) -> bool {
        self.is_distinguished(cone_xy, cone_xz, cone_yz)
    }
}
/// A mixed Hodge structure with finite-dimensional Hodge and weight filtrations.
///
/// Stores the (p, q)-Hodge numbers h^{p,q} = dim H^{p,q} for a pure weight-n MHS.
#[derive(Debug, Clone, Default)]
pub struct MixedHodgeStructureData {
    /// The weight of the pure Hodge structure.
    pub weight: i32,
    /// Hodge numbers h^{p,q}: `hodge_numbers\[(p, q)\]` = dim H^{p,q}.
    pub hodge_numbers: HashMap<(i32, i32), usize>,
}
impl MixedHodgeStructureData {
    /// Create a new pure Hodge structure of the given weight.
    pub fn new(weight: i32) -> Self {
        Self {
            weight,
            hodge_numbers: HashMap::new(),
        }
    }
    /// Set h^{p,q} = value.  Also enforces h^{q,p} = h^{p,q} by conjugate symmetry.
    pub fn set_hodge_number(&mut self, p: i32, q: i32, value: usize) {
        self.hodge_numbers.insert((p, q), value);
        self.hodge_numbers.insert((q, p), value);
    }
    /// Return h^{p,q}.
    pub fn hodge_number(&self, p: i32, q: i32) -> usize {
        *self.hodge_numbers.get(&(p, q)).unwrap_or(&0)
    }
    /// Compute the total Hodge number = sum of all h^{p,q}, counting conjugate pairs once.
    pub fn total_dimension(&self) -> usize {
        let mut dim = 0usize;
        for (&(p, q), &h) in &self.hodge_numbers {
            if p <= q {
                if p == q {
                    dim += h;
                } else {
                    dim += 2 * h;
                }
            }
        }
        dim
    }
    /// Check the Hodge symmetry: h^{p,q} = h^{q,p} for all stored pairs.
    pub fn satisfies_hodge_symmetry(&self) -> bool {
        for (&(p, q), &h) in &self.hodge_numbers {
            if self.hodge_number(q, p) != h {
                return false;
            }
        }
        true
    }
    /// Verify that all nonzero Hodge numbers lie on the anti-diagonal p+q = weight.
    pub fn is_pure(&self) -> bool {
        self.hodge_numbers
            .iter()
            .all(|(&(p, q), &h)| h == 0 || p + q == self.weight)
    }
}
/// A semi-orthogonal decomposition D = ⟨A_1, …, A_n⟩ stored as a finite list
/// of admissible subcategory labels and the Hom-vanishing matrix.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SemiorthogonalDecompositionData {
    /// Labels of the components A_1, …, A_n (in order).
    pub components: Vec<String>,
    /// `hom_nonzero\[i\]\[j\]` = true if Hom(A_j, A_i) ≠ 0 (i.e. j < i allowed).
    pub hom_nonzero: Vec<Vec<bool>>,
}
impl SemiorthogonalDecompositionData {
    /// Create a new SOD with `n` components (all Homs initially zero).
    pub fn new(component_labels: Vec<String>) -> Self {
        let n = component_labels.len();
        Self {
            components: component_labels,
            hom_nonzero: vec![vec![false; n]; n],
        }
    }
    /// Mark Hom(A_j, A_i) ≠ 0.
    pub fn set_hom_nonzero(&mut self, i: usize, j: usize) {
        if i < self.components.len() && j < self.components.len() {
            self.hom_nonzero[i][j] = true;
        }
    }
    /// Verify the SOD axiom: Hom(A_j, A_i) = 0 for all j > i.
    /// Returns `true` if the semi-orthogonality condition holds.
    pub fn is_semiorthogonal(&self) -> bool {
        let n = self.components.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if self.hom_nonzero[i][j] {
                    return false;
                }
            }
        }
        true
    }
    /// Number of components.
    pub fn n_components(&self) -> usize {
        self.components.len()
    }
    /// Return all pairs (i, j) where Hom(A_j, A_i) ≠ 0.
    pub fn nonzero_hom_pairs(&self) -> Vec<(usize, usize)> {
        let n = self.components.len();
        let mut pairs = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if self.hom_nonzero[i][j] {
                    pairs.push((i, j));
                }
            }
        }
        pairs
    }
}
/// A record tracking Bridgeland stability data for a triangulated category
/// represented by finitely many objects.
///
/// Stores the central charge Z(E) ∈ ℂ as a pair (re, im) for each object index.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BridgelandStabilityData {
    /// Object labels.
    pub objects: Vec<String>,
    /// Central charges Z(E) = (re, im): `charges\[i\]` = Z(objects\[i\]).
    pub charges: Vec<(f64, f64)>,
}
impl BridgelandStabilityData {
    /// Create an empty stability data record.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an object with central charge Z = re + im·i.
    pub fn add_object(&mut self, name: &str, re: f64, im: f64) {
        self.objects.push(name.to_string());
        self.charges.push((re, im));
    }
    /// Phase of the central charge: φ(E) = (1/π) arg Z(E) ∈ (0, 1].
    pub fn phase(&self, idx: usize) -> f64 {
        if idx >= self.charges.len() {
            return 0.0;
        }
        let (re, im) = self.charges[idx];
        let arg = im.atan2(re);
        let phi = arg / std::f64::consts::PI;
        if phi <= 0.0 {
            phi + 1.0
        } else {
            phi
        }
    }
    /// Mass of an object: |Z(E)|.
    pub fn mass(&self, idx: usize) -> f64 {
        if idx >= self.charges.len() {
            return 0.0;
        }
        let (re, im) = self.charges[idx];
        (re * re + im * im).sqrt()
    }
    /// Check the support property: Z(E) ≠ 0 for all nonzero objects.
    pub fn support_property_holds(&self) -> bool {
        self.charges
            .iter()
            .all(|(re, im)| re.abs() > 1e-12 || im.abs() > 1e-12)
    }
    /// Determine whether object `j` destabilises object `i`:
    /// `j` destabilises `i` if j is a subobject with phase φ(j) > φ(i).
    pub fn destabilises(&self, j: usize, i: usize) -> bool {
        self.phase(j) > self.phase(i)
    }
    /// Find all semistable objects (those not destabilised by any other object).
    pub fn semistable_objects(&self) -> Vec<usize> {
        (0..self.objects.len())
            .filter(|&i| !(0..self.objects.len()).any(|j| j != i && self.destabilises(j, i)))
            .collect()
    }
}

// ── Spec-required types ─────────────────────────────────────────────────────

/// An abelian group presented by generators and integer relation matrix.
///
/// The group is G = Z^|generators| / im(relations), where each row of
/// `relations` is a linear combination that equals zero.
#[derive(Debug, Clone, Default)]
pub struct AbelianGroup {
    pub generators: Vec<String>,
    /// Each row is a relation (vector over generators).
    pub relations: Vec<Vec<i64>>,
}

impl AbelianGroup {
    pub fn new(generators: Vec<String>, relations: Vec<Vec<i64>>) -> Self {
        AbelianGroup {
            generators,
            relations,
        }
    }

    /// Free abelian group on `n` generators with no relations.
    pub fn free(n: usize) -> Self {
        let gens = (0..n).map(|i| format!("e{}", i)).collect();
        AbelianGroup {
            generators: gens,
            relations: Vec::new(),
        }
    }

    /// The trivial group (one generator, relation \[1\]).
    pub fn trivial() -> Self {
        AbelianGroup {
            generators: vec!["e".into()],
            relations: vec![vec![1]],
        }
    }

    /// Cyclic group Z/nZ.
    pub fn cyclic(n: i64) -> Self {
        AbelianGroup {
            generators: vec!["t".into()],
            relations: vec![vec![n]],
        }
    }

    pub fn rank(&self) -> usize {
        self.generators.len()
    }
}

/// A chain complex C_• with differentials d_n: C_n → C_{n-1}.
///
/// `groups\[n\]` is C_n, `differentials\[n\]` is the matrix of d_{n+1}: C_{n+1} → C_n
/// (rows = dim C_n, cols = dim C_{n+1}).
#[derive(Debug, Clone, Default)]
pub struct SpecChainComplex {
    pub groups: Vec<AbelianGroup>,
    /// differentials\[i\] : groups[i+1] → groups\[i\]
    pub differentials: Vec<Vec<Vec<i64>>>,
}

impl SpecChainComplex {
    pub fn new(groups: Vec<AbelianGroup>, differentials: Vec<Vec<Vec<i64>>>) -> Self {
        SpecChainComplex {
            groups,
            differentials,
        }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

/// A cochain complex C^• with coboundary operators d^n: C^n → C^{n+1}.
#[derive(Debug, Clone, Default)]
pub struct CochainComplex {
    pub groups: Vec<AbelianGroup>,
    /// coboundaries\[i\] : groups\[i\] → groups[i+1]
    pub coboundaries: Vec<Vec<Vec<i64>>>,
}

impl CochainComplex {
    pub fn new(groups: Vec<AbelianGroup>, coboundaries: Vec<Vec<Vec<i64>>>) -> Self {
        CochainComplex {
            groups,
            coboundaries,
        }
    }
}

/// A homology group H_n = Z^r ⊕ Z/t_1 ⊕ … ⊕ Z/t_k.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HomologyGroup {
    pub degree: i32,
    pub free_rank: usize,
    /// Torsion orders (each > 1).
    pub torsion: Vec<u64>,
}

impl HomologyGroup {
    pub fn new(degree: i32, free_rank: usize, torsion: Vec<u64>) -> Self {
        HomologyGroup {
            degree,
            free_rank,
            torsion,
        }
    }

    pub fn zero(degree: i32) -> Self {
        HomologyGroup {
            degree,
            free_rank: 0,
            torsion: Vec::new(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.free_rank == 0 && self.torsion.is_empty()
    }
}

/// A chain map f_•: C_• → D_• — a collection of matrices f_n: C_n → D_n
/// commuting with the differentials.
#[derive(Debug, Clone, Default)]
pub struct SpecChainMap {
    /// id of the source complex (external reference)
    pub source: usize,
    /// id of the target complex (external reference)
    pub target: usize,
    /// maps\[n\] is the matrix f_n: C_n → D_n
    pub maps: Vec<Vec<Vec<i64>>>,
}

impl SpecChainMap {
    pub fn new(source: usize, target: usize, maps: Vec<Vec<Vec<i64>>>) -> Self {
        SpecChainMap {
            source,
            target,
            maps,
        }
    }
}

/// A short exact sequence 0 → A → B → C → 0.
#[derive(Debug, Clone)]
pub struct SpecShortExactSequence {
    pub groups: [AbelianGroup; 3],
    /// maps\[0\]: A → B, maps\[1\]: B → C
    pub maps: [Vec<Vec<i64>>; 2],
}

impl SpecShortExactSequence {
    pub fn new(
        a: AbelianGroup,
        b: AbelianGroup,
        c: AbelianGroup,
        f: Vec<Vec<i64>>,
        g: Vec<Vec<i64>>,
    ) -> Self {
        SpecShortExactSequence {
            groups: [a, b, c],
            maps: [f, g],
        }
    }
}

/// A long exact sequence.
#[derive(Debug, Clone, Default)]
pub struct LongExactSequence {
    pub groups: Vec<AbelianGroup>,
    pub maps: Vec<Vec<Vec<i64>>>,
}

impl LongExactSequence {
    pub fn new(groups: Vec<AbelianGroup>, maps: Vec<Vec<Vec<i64>>>) -> Self {
        LongExactSequence { groups, maps }
    }
}

/// Ext^p(A, B) group.
#[derive(Debug, Clone)]
pub struct SpecExtGroup {
    pub p: usize,
    pub q: usize,
    pub group: HomologyGroup,
}

impl SpecExtGroup {
    pub fn new(p: usize, q: usize, group: HomologyGroup) -> Self {
        SpecExtGroup { p, q, group }
    }
}

/// Tor_p(A, B) group.
#[derive(Debug, Clone)]
pub struct TorGroup {
    pub p: usize,
    pub q: usize,
    pub group: HomologyGroup,
}

impl TorGroup {
    pub fn new(p: usize, q: usize, group: HomologyGroup) -> Self {
        TorGroup { p, q, group }
    }
}
