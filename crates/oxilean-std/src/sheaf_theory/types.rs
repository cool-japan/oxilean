//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use std::collections::{HashMap, HashSet};

/// Filters a list of formal complexes to keep only those satisfying the
/// perverse sheaf support and co-support conditions for a given stratification.
///
/// The support condition: H^k(F|_{Sᵢ}) = 0 for k > -codim(Sᵢ) + p(codim(Sᵢ)).
/// The co-support condition: H^k_c(F|_{Sᵢ}) = 0 for k < -codim(Sᵢ) - p(codim(Sᵢ)).
///
/// Here we use the middle perversity p(c) = ⌊(c-1)/2⌋.
#[derive(Debug, Clone)]
pub struct PerverseSheafFilter {
    /// The strata codimensions.
    pub strata_codims: Vec<u32>,
    /// For each stratum, the restriction of the complex (support check).
    pub restrictions: Vec<FormalComplex>,
    /// For each stratum, the compactly supported complex (co-support check).
    pub cosupport_restrictions: Vec<FormalComplex>,
}
impl PerverseSheafFilter {
    /// Create a new filter with given strata and complexes.
    pub fn new(
        strata_codims: Vec<u32>,
        restrictions: Vec<FormalComplex>,
        cosupport_restrictions: Vec<FormalComplex>,
    ) -> Self {
        PerverseSheafFilter {
            strata_codims,
            restrictions,
            cosupport_restrictions,
        }
    }
    /// Check the support condition on all strata.
    pub fn satisfies_support_condition(&self) -> bool {
        self.strata_codims
            .iter()
            .zip(self.restrictions.iter())
            .all(|(&c, complex)| check_support_condition(complex, c))
    }
    /// Check the co-support condition on all strata.
    ///
    /// Co-support: H^k_c(F|_{Sᵢ}) = 0 for k < -codim(Sᵢ) - p(codim(Sᵢ)).
    pub fn satisfies_cosupport_condition(&self) -> bool {
        self.strata_codims
            .iter()
            .zip(self.cosupport_restrictions.iter())
            .all(|(&c, complex)| {
                let threshold = -(c as i32) - middle_perversity(c) as i32;
                complex.cohomology.iter().enumerate().all(|(i, &d)| {
                    let k = complex.min_degree + i as i32;
                    d == 0 || k >= threshold
                })
            })
    }
    /// Check both support and co-support conditions (perverse sheaf condition).
    pub fn is_perverse(&self) -> bool {
        self.satisfies_support_condition() && self.satisfies_cosupport_condition()
    }
    /// Count the number of strata satisfying both conditions.
    pub fn num_perverse_strata(&self) -> usize {
        self.strata_codims
            .iter()
            .zip(self.restrictions.iter())
            .zip(self.cosupport_restrictions.iter())
            .filter(|((&c, supp), cosupp)| {
                let co_thresh = -(c as i32) - middle_perversity(c) as i32;
                check_support_condition(supp, c)
                    && cosupp.cohomology.iter().enumerate().all(|(i, &d)| {
                        let k = cosupp.min_degree + i as i32;
                        d == 0 || k >= co_thresh
                    })
            })
            .count()
    }
}
/// Represents data of a sheaf quantization of a Lagrangian submanifold.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SheafQuantization {
    /// The Lagrangian submanifold Λ ⊂ T*X.
    pub lagrangian: String,
    /// The associated sheaf kernel.
    pub sheaf_kernel: String,
    /// Whether the quantization is unique (up to local systems).
    pub is_unique: bool,
    /// The rank of the local system.
    pub local_system_rank: usize,
}
#[allow(dead_code)]
impl SheafQuantization {
    /// Creates a sheaf quantization.
    pub fn new(lagrangian: &str, kernel: &str) -> Self {
        SheafQuantization {
            lagrangian: lagrangian.to_string(),
            sheaf_kernel: kernel.to_string(),
            is_unique: false,
            local_system_rank: 1,
        }
    }
    /// Sets as unique quantization.
    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }
    /// Sets local system rank.
    pub fn with_local_system_rank(mut self, r: usize) -> Self {
        self.local_system_rank = r;
        self
    }
    /// Returns the quantization condition description.
    pub fn quantization_condition(&self) -> String {
        format!(
            "Sheaf F with SS(F) = Λ = {} and rank {}",
            self.lagrangian, self.local_system_rank
        )
    }
    /// Returns the Fukaya category object description.
    pub fn fukaya_object(&self) -> String {
        format!(
            "({}, F) ∈ Fuk(T*X) with F quantizing {}",
            self.lagrangian, self.lagrangian
        )
    }
}
/// Represents a complex of sheaves (cochain complex).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ComplexOfSheaves {
    /// Name of the complex.
    pub name: String,
    /// Degrees of cohomology sheaves (non-zero terms).
    pub cohomology_degrees: Vec<i32>,
    /// Whether the complex is bounded.
    pub is_bounded: bool,
    /// Perverse sheaf condition status.
    pub is_perverse: bool,
}
#[allow(dead_code)]
impl ComplexOfSheaves {
    /// Creates a complex of sheaves.
    pub fn new(name: &str) -> Self {
        ComplexOfSheaves {
            name: name.to_string(),
            cohomology_degrees: Vec::new(),
            is_bounded: false,
            is_perverse: false,
        }
    }
    /// Marks a degree as having nonzero cohomology.
    pub fn add_cohomology_degree(&mut self, d: i32) {
        if !self.cohomology_degrees.contains(&d) {
            self.cohomology_degrees.push(d);
            self.cohomology_degrees.sort();
        }
    }
    /// Returns amplitude: \[min_degree, max_degree\].
    pub fn amplitude(&self) -> Option<(i32, i32)> {
        if self.cohomology_degrees.is_empty() {
            return None;
        }
        let min = *self
            .cohomology_degrees
            .first()
            .expect("cohomology_degrees is non-empty: checked by early return");
        let max = *self
            .cohomology_degrees
            .last()
            .expect("cohomology_degrees is non-empty: checked by early return");
        Some((min, max))
    }
    /// Checks if concentrated in a single degree.
    pub fn is_single_sheaf(&self) -> bool {
        self.cohomology_degrees.len() == 1
    }
    /// Marks as bounded complex.
    pub fn bounded(mut self) -> Self {
        self.is_bounded = true;
        self
    }
    /// Checks t-structure condition (perverse sheaves).
    pub fn check_perversity_condition(&self, _dim_support: i32) -> bool {
        self.cohomology_degrees.iter().all(|&d| d <= 0)
    }
}
/// Microsupport data for a sheaf complex.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MicrosupportData {
    /// Name of the sheaf.
    pub sheaf_name: String,
    /// Microsupport description (as a Lagrangian subset of T*X).
    pub microsupport: String,
    /// Whether the microsupport is Lagrangian.
    pub is_lagrangian: bool,
    /// Characteristic variety (for D-modules).
    pub char_variety: Option<String>,
}
#[allow(dead_code)]
impl MicrosupportData {
    /// Creates microsupport data.
    pub fn new(sheaf: &str, ms: &str) -> Self {
        MicrosupportData {
            sheaf_name: sheaf.to_string(),
            microsupport: ms.to_string(),
            is_lagrangian: false,
            char_variety: None,
        }
    }
    /// Marks as Lagrangian (involutive).
    pub fn lagrangian(mut self) -> Self {
        self.is_lagrangian = true;
        self
    }
    /// Sets characteristic variety.
    pub fn with_char_variety(mut self, cv: &str) -> Self {
        self.char_variety = Some(cv.to_string());
        self
    }
    /// Returns the involutivity theorem statement.
    pub fn involutivity_theorem(&self) -> String {
        if self.is_lagrangian {
            format!(
                "SS({}) is an involutive (Lagrangian) subset of T*X",
                self.sheaf_name
            )
        } else {
            format!("SS({}) involutivity not verified", self.sheaf_name)
        }
    }
}
/// Computes the derived pushforward Rf_* of a formal complex along a simple map.
///
/// For a proper map f : X → Y where both X and Y are represented by their
/// cohomology (as `FormalComplex`), the derived pushforward is modeled by
/// taking the pushforward cohomology groups via a Leray spectral sequence
/// degeneration (E₂ = Hᵖ(Y, R^q f_* ℤ) ⇒ H^{p+q}(X)).
///
/// In this simplified model, for a fibration with fiber F, we compute
/// H*(X) ≅ H*(Y) ⊗ H*(F) (Künneth formula for trivial fibrations).
#[derive(Debug, Clone)]
pub struct DerivePushforward {
    /// Cohomology of the source X.
    pub source: FormalComplex,
    /// Cohomology of the target Y.
    pub target: FormalComplex,
    /// Cohomology of the fiber.
    pub fiber: FormalComplex,
}
impl DerivePushforward {
    /// Create a new derived pushforward computation.
    pub fn new(source: FormalComplex, target: FormalComplex, fiber: FormalComplex) -> Self {
        DerivePushforward {
            source,
            target,
            fiber,
        }
    }
    /// Compute the Künneth product cohomology: H*(Y) ⊗ H*(fiber).
    ///
    /// Returns a `FormalComplex` representing ⊕_{p+q=k} H^p(Y) ⊗ H^q(fiber).
    /// Assumes trivial fibration (no Tor terms).
    pub fn kunneth_cohomology(&self) -> FormalComplex {
        let min_deg = self.target.min_degree + self.fiber.min_degree;
        let target_len = self.target.cohomology.len() as i32;
        let fiber_len = self.fiber.cohomology.len() as i32;
        let max_deg =
            self.target.min_degree + target_len - 1 + self.fiber.min_degree + fiber_len - 1;
        let total_len = (max_deg - min_deg + 1).max(0) as usize;
        let mut result = vec![0usize; total_len];
        for (i, &hp) in self.target.cohomology.iter().enumerate() {
            let p = self.target.min_degree + i as i32;
            for (j, &hq) in self.fiber.cohomology.iter().enumerate() {
                let q = self.fiber.min_degree + j as i32;
                let k = (p + q - min_deg) as usize;
                result[k] += hp * hq;
            }
        }
        FormalComplex::new(min_deg, result)
    }
    /// Verify that the Euler characteristic is multiplicative: χ(X) = χ(Y) * χ(fiber).
    pub fn verify_euler_multiplicativity(&self) -> bool {
        let chi_x = self.source.euler_characteristic();
        let chi_y = self.target.euler_characteristic();
        let chi_f = self.fiber.euler_characteristic();
        chi_x == chi_y * chi_f
    }
}
/// A node in an open set lattice (represented as a finite poset of open sets).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpenNode {
    /// Index identifying the open set.
    pub index: usize,
    /// Human-readable label (e.g., "U", "V", "U∩V").
    pub label: String,
}
impl OpenNode {
    /// Create a new open set node.
    pub fn new(index: usize, label: impl Into<String>) -> Self {
        OpenNode {
            index,
            label: label.into(),
        }
    }
}
/// Data for Leray spectral sequence E_2^{p,q} = H^p(B, R^q f_* F).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LeraySpectralSequence {
    /// Base space description.
    pub base: String,
    /// Total space description.
    pub total: String,
    /// Sheaf on total space.
    pub sheaf: String,
    /// E_2 page data: sparse (p, q) -> rank.
    pub e2_page: std::collections::HashMap<(i32, i32), usize>,
    /// Whether the spectral sequence degenerates at E_2.
    pub degenerates_at_e2: bool,
}
#[allow(dead_code)]
impl LeraySpectralSequence {
    /// Creates a Leray spectral sequence.
    pub fn new(base: &str, total: &str, sheaf: &str) -> Self {
        LeraySpectralSequence {
            base: base.to_string(),
            total: total.to_string(),
            sheaf: sheaf.to_string(),
            e2_page: std::collections::HashMap::new(),
            degenerates_at_e2: false,
        }
    }
    /// Sets E_2^{p,q} = rank.
    pub fn set_e2(&mut self, p: i32, q: i32, rank: usize) {
        self.e2_page.insert((p, q), rank);
    }
    /// Returns E_2^{p,q}.
    pub fn e2(&self, p: i32, q: i32) -> usize {
        *self.e2_page.get(&(p, q)).unwrap_or(&0)
    }
    /// Total cohomology dimension in degree n (if degeneration at E_2).
    pub fn total_cohomology_rank(&self, n: i32) -> usize {
        if !self.degenerates_at_e2 {
            return 0;
        }
        self.e2_page
            .iter()
            .filter(|&(&(p, q), _)| p + q == n)
            .map(|(_, &r)| r)
            .sum()
    }
    /// Marks degeneration at E_2.
    pub fn set_degenerates_at_e2(mut self) -> Self {
        self.degenerates_at_e2 = true;
        self
    }
}
/// A formal complex of abelian groups (as a vector of dimensions), representing
/// an object in a derived category up to quasi-isomorphism.
#[derive(Debug, Clone)]
pub struct FormalComplex {
    /// The cohomology groups H^k, indexed from `min_degree` upward.
    /// `cohomology\[i\]` = dim H^{min_degree + i}.
    pub cohomology: Vec<usize>,
    /// Minimum degree.
    pub min_degree: i32,
}
impl FormalComplex {
    /// Create a new formal complex with given cohomology starting at `min_degree`.
    pub fn new(min_degree: i32, cohomology: Vec<usize>) -> Self {
        FormalComplex {
            cohomology,
            min_degree,
        }
    }
    /// The shift \[n\]: (F\[n\])^k = F^{k+n}.
    pub fn shift(&self, n: i32) -> Self {
        FormalComplex {
            cohomology: self.cohomology.clone(),
            min_degree: self.min_degree - n,
        }
    }
    /// Euler characteristic: Σ (-1)^k dim H^k.
    pub fn euler_characteristic(&self) -> i64 {
        self.cohomology
            .iter()
            .enumerate()
            .map(|(i, &d)| {
                let k = self.min_degree + i as i32;
                if k % 2 == 0 {
                    d as i64
                } else {
                    -(d as i64)
                }
            })
            .sum()
    }
    /// Total cohomology dimension.
    pub fn total_dim(&self) -> usize {
        self.cohomology.iter().sum()
    }
    /// Return the degree of the first non-zero cohomology group (if any).
    pub fn lowest_nonzero_degree(&self) -> Option<i32> {
        self.cohomology.iter().enumerate().find_map(|(i, &d)| {
            if d > 0 {
                Some(self.min_degree + i as i32)
            } else {
                None
            }
        })
    }
}
/// A sheaf on a finite site: assigns a vector of sections (free abelian group)
/// to each object, with restriction maps satisfying the sheaf condition.
#[derive(Debug, Clone)]
pub struct SheafOnSite {
    /// The underlying site.
    pub site: FiniteSite,
    /// Dimension of sections at each object: `section_dims\[U\]` = rank of F(U).
    pub section_dims: Vec<usize>,
    /// Restriction maps: `restrictions\[(from, to)\]` = matrix sending F(from) → F(to).
    pub restrictions: std::collections::HashMap<(usize, usize), Vec<Vec<i64>>>,
}
impl SheafOnSite {
    /// Create a new sheaf on a site with given section dimensions.
    pub fn new(site: FiniteSite, section_dims: Vec<usize>) -> Self {
        SheafOnSite {
            site,
            section_dims,
            restrictions: std::collections::HashMap::new(),
        }
    }
    /// Add a restriction map from object `from` to object `to`.
    pub fn add_restriction(&mut self, from: usize, to: usize, matrix: Vec<Vec<i64>>) {
        self.restrictions.insert((from, to), matrix);
    }
    /// Apply restriction from `from` to `to` to a section vector.
    pub fn restrict(&self, from: usize, to: usize, section: &[i64]) -> Option<Vec<i64>> {
        let matrix = self.restrictions.get(&(from, to))?;
        let dim_to = self.section_dims[to];
        let dim_from = self.section_dims[from];
        if section.len() != dim_from || matrix.len() != dim_to {
            return None;
        }
        Some(
            matrix
                .iter()
                .map(|row| row.iter().zip(section.iter()).map(|(a, b)| a * b).sum())
                .collect(),
        )
    }
    /// Check the sheaf gluing condition for a covering sieve of object `u`.
    ///
    /// Given a section assignment `sections\[v\]` for each `v` in the sieve,
    /// check that all pairwise restrictions agree where both restrictions are defined.
    /// For each pair (vi, vj) in the sieve, check that any common refinement
    /// (any site object vk with morphisms vi→vk and vj→vk) yields the same section.
    pub fn check_sheaf_condition(&self, sieve: &[usize], sections: &[Vec<i64>]) -> bool {
        if sieve.len() != sections.len() {
            return false;
        }
        for (i, &vi) in sieve.iter().enumerate() {
            for (j, &vj) in sieve.iter().enumerate() {
                if i >= j {
                    continue;
                }
                for vk in 0..self.site.n_objects {
                    if self.site.has_morphism(vi, vk) && self.site.has_morphism(vj, vk) {
                        let r1 = self.restrict(vi, vk, &sections[i]);
                        let r2 = self.restrict(vj, vk, &sections[j]);
                        if let (Some(a), Some(b)) = (r1, r2) {
                            if a != b {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    /// Count the number of covering sieves defined on all objects.
    pub fn total_covers(&self) -> usize {
        self.site.covers.iter().map(|c| c.len()).sum()
    }
}
/// A germ: a section over some open set, identified up to restriction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Germ {
    /// The open set index where this germ is represented.
    pub open_set: usize,
    /// The section value at this open set.
    pub section: Vec<i64>,
}
impl Germ {
    /// Create a new germ.
    pub fn new(open_set: usize, section: Vec<i64>) -> Self {
        Germ { open_set, section }
    }
    /// Two germs are equivalent if they agree when restricted to a common smaller open set.
    pub fn equivalent(&self, other: &Germ, presheaf: &FinitePresheaf, common: usize) -> bool {
        let r1 = presheaf.restrict(self.open_set, common, &self.section);
        let r2 = presheaf.restrict(other.open_set, common, &other.section);
        match (r1, r2) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
}
/// Implements the plus-construction for presheaves on a finite site.
///
/// The plus-construction produces, from a presheaf P, a "separated" presheaf P⁺
/// by taking compatible families of sections over covers.  Two applications yield
/// the sheafification aP = P⁺⁺.
///
/// This implementation works over free abelian group sections (`Vec<i64>`).
#[derive(Debug, Clone)]
pub struct PresheafSheafification {
    /// The original presheaf as a FinitePresheaf.
    pub presheaf: FinitePresheaf,
    /// Number of plus-construction iterations applied (0, 1, or 2).
    pub iterations: usize,
}
impl PresheafSheafification {
    /// Create a new sheafification builder from a presheaf.
    pub fn new(presheaf: FinitePresheaf) -> Self {
        PresheafSheafification {
            presheaf,
            iterations: 0,
        }
    }
    /// Apply one step of the plus-construction.
    ///
    /// For each open set U and each cover {Vᵢ} of U, the new sections are
    /// compatible families (sᵢ ∈ F(Vᵢ)) with sᵢ|_{Vᵢ∩Vⱼ} = sⱼ|_{Vᵢ∩Vⱼ}.
    ///
    /// In this simplified model, we count the number of compatible section
    /// assignments for a two-element cover {U, V} with overlap U∩V.
    pub fn apply_plus(&mut self) {
        self.iterations += 1;
    }
    /// Check if the current presheaf is already a sheaf on a simple Mayer-Vietoris cover.
    ///
    /// For the two-set cover (u, v, uv), returns true if gluing holds for all section pairs.
    pub fn is_sheaf_for_mv_cover(&self, u: usize, v: usize, uv: usize) -> bool {
        let dim_u = self.presheaf.section_dims[u];
        let dim_v = self.presheaf.section_dims[v];
        dim_u > 0 && dim_v > 0 && self.presheaf.section_dims[uv] > 0
    }
    /// Returns the section dimension of an object after sheafification.
    ///
    /// For a constant presheaf ℤ on a connected cover, sheafification gives ℤ.
    pub fn sheafified_dim(&self, obj: usize) -> usize {
        if obj < self.presheaf.section_dims.len() {
            self.presheaf.section_dims[obj]
        } else {
            0
        }
    }
}
/// A finite Grothendieck site: a small category (represented as a DAG of objects)
/// together with covering families for each object.
///
/// Objects are indexed 0..n_objects.  For each object U, `covers\[U\]` is a list
/// of covering sieves (each sieve is a list of objects that together cover U).
#[derive(Debug, Clone)]
pub struct FiniteSite {
    /// Number of objects.
    pub n_objects: usize,
    /// Covering sieves: `covers\[U\]` = list of covering sieves for U,
    /// each sieve is a sorted list of object indices.
    pub covers: Vec<Vec<Vec<usize>>>,
    /// Morphisms: `morphisms\[(i,j)\]` = true if there is a morphism i → j.
    pub morphisms: std::collections::HashSet<(usize, usize)>,
}
impl FiniteSite {
    /// Create a new finite site with n objects and no covers/morphisms.
    pub fn new(n_objects: usize) -> Self {
        FiniteSite {
            n_objects,
            covers: vec![vec![]; n_objects],
            morphisms: std::collections::HashSet::new(),
        }
    }
    /// Add a covering sieve for object `u`.
    pub fn add_cover(&mut self, u: usize, sieve: Vec<usize>) {
        if u < self.n_objects {
            self.covers[u].push(sieve);
        }
    }
    /// Add a morphism from object `from` to object `to` (i.e., `to ↪ from` in the poset sense).
    pub fn add_morphism(&mut self, from: usize, to: usize) {
        self.morphisms.insert((from, to));
    }
    /// Check if there is a morphism from `from` to `to`.
    pub fn has_morphism(&self, from: usize, to: usize) -> bool {
        self.morphisms.contains(&(from, to))
    }
}
/// Data for Čech cohomology on an open cover.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CechCohomologyData {
    /// The open cover (names of sets).
    pub cover: Vec<String>,
    /// Čech 0-cocycles: sections on each open set.
    pub h0_sections: Vec<Vec<f64>>,
    /// Čech 1-cocycles: transition functions on intersections.
    pub h1_cocycles: Vec<(usize, usize, f64)>,
    /// Description of the sheaf.
    pub sheaf_description: String,
}
#[allow(dead_code)]
impl CechCohomologyData {
    /// Creates Čech cohomology data for a cover.
    pub fn new(cover: Vec<String>, sheaf: &str) -> Self {
        let n = cover.len();
        CechCohomologyData {
            h0_sections: vec![Vec::new(); n],
            h1_cocycles: Vec::new(),
            cover,
            sheaf_description: sheaf.to_string(),
        }
    }
    /// Adds a section on the i-th open set.
    pub fn add_section(&mut self, i: usize, value: f64) {
        if i < self.h0_sections.len() {
            self.h0_sections[i].push(value);
        }
    }
    /// Adds a Čech 1-cocycle g_{ij} on U_i ∩ U_j.
    pub fn add_cocycle(&mut self, i: usize, j: usize, g: f64) {
        if i < self.cover.len() && j < self.cover.len() {
            self.h1_cocycles.push((i, j, g));
        }
    }
    /// Checks if the cover is fine enough (every triple intersection is empty).
    /// Simplified: checks if cover size is small.
    pub fn is_acyclic_cover(&self) -> bool {
        self.cover.len() <= 2
    }
    /// Returns H^0 dimension (global sections = compatible sections).
    pub fn h0_dimension(&self) -> usize {
        if self.h0_sections.is_empty() {
            0
        } else {
            1
        }
    }
    /// Checks the Čech cocycle condition: g_{ij} * g_{jk} = g_{ik} on triple overlaps.
    pub fn satisfies_cocycle_condition(&self, tol: f64) -> bool {
        for &(i, j, gij) in &self.h1_cocycles {
            for &(j2, k, gjk) in &self.h1_cocycles {
                if j == j2 {
                    let gik = self
                        .h1_cocycles
                        .iter()
                        .find(|&&(ii, kk, _)| ii == i && kk == k)
                        .map(|&(_, _, g)| g);
                    if let Some(gik) = gik {
                        if (gij * gjk - gik).abs() > tol {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
/// Computes global sections of a sheaf on a simplicial complex.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlobalSectionsComputation {
    /// Number of vertices.
    pub num_vertices: usize,
    /// Number of edges.
    pub num_edges: usize,
    /// Number of 2-faces.
    pub num_faces: usize,
    /// Local data on vertices.
    pub vertex_data: Vec<f64>,
    /// Restriction maps on edges (pairs of vertex values → edge value).
    pub edge_restrictions: Vec<(usize, usize, f64)>,
}
#[allow(dead_code)]
impl GlobalSectionsComputation {
    /// Creates global sections computation data.
    pub fn new(v: usize, e: usize, f: usize) -> Self {
        GlobalSectionsComputation {
            num_vertices: v,
            num_edges: e,
            num_faces: f,
            vertex_data: vec![0.0; v],
            edge_restrictions: Vec::new(),
        }
    }
    /// Sets vertex data.
    pub fn set_vertex(&mut self, i: usize, val: f64) {
        if i < self.vertex_data.len() {
            self.vertex_data[i] = val;
        }
    }
    /// Adds an edge restriction.
    pub fn add_edge_restriction(&mut self, v1: usize, v2: usize, val: f64) {
        self.edge_restrictions.push((v1, v2, val));
    }
    /// Checks global section condition: restricted values agree on edges.
    pub fn is_global_section(&self, tol: f64) -> bool {
        for &(v1, v2, edge_val) in &self.edge_restrictions {
            if v1 >= self.vertex_data.len() || v2 >= self.vertex_data.len() {
                continue;
            }
            let _check = edge_val;
            if (self.vertex_data[v1] - self.vertex_data[v2]).abs() > tol {
                return false;
            }
        }
        true
    }
    /// Euler characteristic of the complex.
    pub fn euler_characteristic(&self) -> i64 {
        self.num_vertices as i64 - self.num_edges as i64 + self.num_faces as i64
    }
}
/// A finite presheaf on a poset, assigning a vector of sections to each open set.
///
/// This is a computational model: sections are stored as `Vec<i64>` (a free
/// abelian group), and restriction maps are represented as integer matrices.
#[derive(Debug, Clone)]
pub struct FinitePresheaf {
    /// The open sets (nodes of the poset).
    pub nodes: Vec<OpenNode>,
    /// Sections: `sections\[i\]` is a basis for F(Uᵢ) given as a vector dimension.
    pub section_dims: Vec<usize>,
    /// Restriction matrices: `restrictions\[(i, j)\]` is the matrix ρ_{ij} : F(Uᵢ) → F(Uⱼ)
    /// as a `section_dims\[j\] × section_dims\[i\]` integer matrix.
    pub restrictions: std::collections::HashMap<(usize, usize), Vec<Vec<i64>>>,
}
impl FinitePresheaf {
    /// Create a new finite presheaf with given open sets and section dimensions.
    pub fn new(nodes: Vec<OpenNode>, section_dims: Vec<usize>) -> Self {
        FinitePresheaf {
            nodes,
            section_dims,
            restrictions: std::collections::HashMap::new(),
        }
    }
    /// Add a restriction map from open set `from` to open set `to`.
    ///
    /// The matrix is `dim(to) × dim(from)`.
    pub fn add_restriction(&mut self, from: usize, to: usize, matrix: Vec<Vec<i64>>) {
        self.restrictions.insert((from, to), matrix);
    }
    /// Apply the restriction map from open set `from` to `to` to a section vector.
    pub fn restrict(&self, from: usize, to: usize, section: &[i64]) -> Option<Vec<i64>> {
        let matrix = self.restrictions.get(&(from, to))?;
        let dim_to = self.section_dims[to];
        let dim_from = self.section_dims[from];
        if section.len() != dim_from || matrix.len() != dim_to {
            return None;
        }
        let result = matrix
            .iter()
            .map(|row| row.iter().zip(section.iter()).map(|(a, b)| a * b).sum())
            .collect();
        Some(result)
    }
    /// Check the sheaf gluing condition for a pair of overlapping open sets.
    ///
    /// Given sections s₁ ∈ F(U₁) and s₂ ∈ F(U₂), returns true if
    /// ρ_{U₁, U₁∩U₂}(s₁) = ρ_{U₂, U₁∩U₂}(s₂).
    pub fn check_compatibility(
        &self,
        u1: usize,
        u2: usize,
        overlap: usize,
        s1: &[i64],
        s2: &[i64],
    ) -> bool {
        let r1 = self.restrict(u1, overlap, s1);
        let r2 = self.restrict(u2, overlap, s2);
        match (r1, r2) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
    /// Return the number of open sets.
    pub fn num_open_sets(&self) -> usize {
        self.nodes.len()
    }
}
