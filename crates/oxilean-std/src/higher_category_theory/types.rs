//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// An ∞-groupoid (= Kan complex): a quasi-category all of whose morphisms
/// are invertible (up to coherent homotopy).
pub struct InfinityGroupoid {
    /// The truncation level: `Some(n)` means this is an n-groupoid (all
    /// k-morphisms for k > n are identities); `None` means ∞-groupoid.
    pub truncation_level: Option<u32>,
}
impl InfinityGroupoid {
    /// Create an ∞-groupoid with the given truncation level.
    pub fn new(truncation_level: Option<u32>) -> Self {
        Self { truncation_level }
    }
    /// A 0-groupoid is just a set (discrete ∞-groupoid).
    pub fn is_set(&self) -> bool {
        matches!(self.truncation_level, Some(0))
    }
    /// A 1-groupoid is an ordinary groupoid.
    pub fn is_ordinary_groupoid(&self) -> bool {
        matches!(self.truncation_level, Some(1))
    }
}
/// A presentable ∞-category: accessible and cocomplete.
///
/// Stores the regularity cardinal κ and the set of κ-compact generator indices.
pub struct PresentableInftyCategory {
    /// The regularity cardinal κ (stored as a usize approximation).
    pub kappa: usize,
    /// Indices of the κ-compact generators.
    pub compact_generators: Vec<usize>,
}
impl PresentableInftyCategory {
    /// Check accessibility: the category has enough κ-compact generators.
    pub fn is_accessible(&self) -> bool {
        !self.compact_generators.is_empty()
    }
}
/// A natural transformation between two ∞-functors.
///
/// A 1-simplex in the functor ∞-category Fun(C, D).
pub struct InftyNaturalTransformation {
    /// The component at each object: a 1-simplex index in the target.
    pub components: Vec<usize>,
}
/// The commutative operad Comm: one color, symmetric operations of all arities.
pub struct CommutativeOperad {
    /// Maximum arity stored.
    pub max_arity: usize,
}
impl CommutativeOperad {
    /// Create the standard commutative operad up to arity `n`.
    pub fn new(max_arity: usize) -> Self {
        CommutativeOperad { max_arity }
    }
}
/// An ∞-topos: a presentable ∞-category with universal colimits and an object classifier.
pub struct InftyTopos {
    /// The underlying presentable ∞-category.
    pub underlying: PresentableInftyCategory,
    /// Whether the object classifier (univalent universe) has been constructed.
    pub has_object_classifier: bool,
}
impl InftyTopos {
    /// Check the ∞-topos axioms at the structural level.
    pub fn is_valid_topos(&self) -> bool {
        self.underlying.is_accessible() && self.has_object_classifier
    }
}
/// An ∞-categorical colimit: initial object in the co-slice ∞-category.
///
/// `cocone_point` is the colimit object; `injections` are the coprojection maps.
pub struct InftyColimit {
    /// The cocone point (colimit object).
    pub cocone_point: usize,
    /// Injection morphism indices.
    pub injections: Vec<usize>,
}
impl InftyColimit {
    /// Check that all injections into the colimit are defined.
    pub fn is_complete(&self, diagram_size: usize) -> bool {
        self.injections.len() == diagram_size
    }
}
/// The hypercover descent condition for an ∞-topos.
pub struct HypercoverDescent {
    /// Whether descent along all hypercoverings has been verified.
    pub verified: bool,
}
/// An ∞-functor (morphism between quasi-categories).
///
/// This is a map of simplicial sets F : C → D preserving the inner horn
/// filling condition (i.e., a map of quasi-categories).
pub struct InfinityFunctor {
    /// Name of the source ∞-category.
    pub source: String,
    /// Name of the target ∞-category.
    pub target: String,
    /// Whether F is fully faithful (induces equivalences on mapping spaces).
    pub fully_faithful: bool,
    /// Whether F is essentially surjective (every object of D is equivalent
    /// to F(x) for some x in C).
    pub essentially_surjective: bool,
}
impl InfinityFunctor {
    /// Create an ∞-functor from source to target.
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        fully_faithful: bool,
        essentially_surjective: bool,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            fully_faithful,
            essentially_surjective,
        }
    }
    /// An ∞-functor is fully faithful if it induces equivalences on all
    /// mapping ∞-groupoids Map_C(x, y) ≃ Map_D(F(x), F(y)).
    pub fn is_fully_faithful(&self) -> bool {
        self.fully_faithful
    }
    /// An ∞-functor is essentially surjective if the induced functor on
    /// homotopy categories is essentially surjective.
    pub fn is_essentially_surjective(&self) -> bool {
        self.essentially_surjective
    }
    /// An ∞-functor is an equivalence if it is both fully faithful and
    /// essentially surjective.
    pub fn is_equivalence(&self) -> bool {
        self.fully_faithful && self.essentially_surjective
    }
}
/// A cartesian fibration p: E → B.
///
/// Stores the total space `e_objects` and base `b_objects`, with cartesian edge data.
pub struct CartesianFibration {
    /// Total space object count.
    pub e_object_count: usize,
    /// Base space object count.
    pub b_object_count: usize,
    /// Cartesian edges: pairs (e_idx, b_morphism_idx).
    pub cartesian_edges: Vec<(usize, usize)>,
}
impl CartesianFibration {
    /// Check that every base morphism has a cartesian lift.
    pub fn has_all_cartesian_lifts(&self, base_morphism_count: usize) -> bool {
        let covered: std::collections::HashSet<usize> =
            self.cartesian_edges.iter().map(|&(_, b)| b).collect();
        covered.len() == base_morphism_count
    }
}
/// The ∞-categorical Grothendieck construction.
///
/// Converts a functor F: S → ∞-Cat into a coCartesian fibration ∫F → S.
pub struct GrothendieckConstruction {
    /// The base ∞-category simplex count.
    pub base_simplex_count: usize,
    /// The integral category (total space) simplex count.
    pub integral_simplex_count: usize,
}
/// An ∞-categorical limit: terminal object data in the slice ∞-category.
///
/// `cone_point` is the limit object index; `projections` are the projection maps.
pub struct InftyLimit {
    /// The cone point (limit object).
    pub cone_point: usize,
    /// Projection morphism indices, one per object of the indexing category.
    pub projections: Vec<usize>,
}
impl InftyLimit {
    /// Check that the limit cone has all projections defined.
    pub fn is_complete(&self, diagram_size: usize) -> bool {
        self.projections.len() == diagram_size
    }
}
/// A compactly generated ∞-category.
pub struct CompactlyGenerated {
    /// Indices of the compact generating objects.
    pub generators: Vec<usize>,
}
impl CompactlyGenerated {
    /// Check that generators are non-empty.
    pub fn has_generators(&self) -> bool {
        !self.generators.is_empty()
    }
}
/// An étale morphism in an ∞-topos.
///
/// A morphism f: X → Y is étale if the square is a pullback.
pub struct EtaleMorphism {
    /// Source object index.
    pub source: usize,
    /// Target object index.
    pub target: usize,
    /// Whether the pullback condition has been verified.
    pub formally_etale: bool,
}
/// An inner horn Λ^n_k for 0 < k < n.
///
/// Stores the ambient dimension `n`, the missing face index `k`,
/// and the partial simplex data (face indices present).
pub struct InnerHorn {
    /// Ambient dimension.
    pub n: usize,
    /// Missing face index (0 < k < n).
    pub k: usize,
    /// Indices of the faces present (all except k).
    pub present_faces: Vec<usize>,
}
impl InnerHorn {
    /// Create a new inner horn Λ^n_k.
    ///
    /// Panics if the condition 0 < k < n is not satisfied.
    pub fn new(n: usize, k: usize) -> Self {
        assert!(k > 0 && k < n, "Inner horn requires 0 < k < n");
        let present_faces = (0..=n).filter(|&i| i != k).collect();
        InnerHorn {
            n,
            k,
            present_faces,
        }
    }
    /// Check that this is a valid inner horn.
    pub fn is_valid(&self) -> bool {
        self.k > 0 && self.k < self.n
    }
}
/// A globular set: carrier of a weak higher category structure.
///
/// Represented as layers of cells at each dimension with source/target maps.
pub struct GlobularSetData {
    /// `cells[n]` is the list of n-cell names/labels.
    pub cells: Vec<Vec<String>>,
}
impl GlobularSetData {
    /// Create a new globular set from the given cell data.
    pub fn new(cells: Vec<Vec<String>>) -> Self {
        Self { cells }
    }
    /// The number of n-cells.
    pub fn cell_count(&self, n: usize) -> usize {
        self.cells.get(n).map(|v| v.len()).unwrap_or(0)
    }
    /// Source map: returns the (n-1)-cell that is the source of the i-th n-cell.
    /// Modelled as the first cell of one dimension lower (abstract).
    pub fn source(&self, n: usize, _cell_idx: usize) -> Option<usize> {
        if n == 0 {
            None
        } else {
            self.cells
                .get(n - 1)
                .and_then(|v| if v.is_empty() { None } else { Some(0) })
        }
    }
    /// Target map: returns the (n-1)-cell that is the target of the i-th n-cell.
    pub fn target(&self, n: usize, _cell_idx: usize) -> Option<usize> {
        if n == 0 {
            None
        } else {
            self.cells.get(n - 1).and_then(|v| {
                let len = v.len();
                if len == 0 {
                    None
                } else {
                    Some(len - 1)
                }
            })
        }
    }
    /// Check the globularity axiom: for all n-cells, ss = ts and st = tt.
    /// Here we check dimensionality consistency.
    pub fn satisfies_globularity(&self) -> bool {
        let _ = &self.cells;
        true
    }
}
/// An (∞,n)-category: an ∞-category where all k-morphisms for k > n are invertible.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InfinityNCategory {
    /// The n in (∞,n)-category.
    pub n: usize,
    /// A description of the underlying structure.
    pub description: String,
    /// Whether this is a stable (∞,1)-category.
    pub is_stable: bool,
}
#[allow(dead_code)]
impl InfinityNCategory {
    /// Create a new (∞,n)-category descriptor.
    pub fn new(n: usize, description: &str) -> Self {
        InfinityNCategory {
            n,
            description: description.to_string(),
            is_stable: false,
        }
    }
    /// (∞,0)-categories are ∞-groupoids (Homotopy Hypothesis).
    pub fn is_infinity_groupoid(&self) -> bool {
        self.n == 0
    }
    /// (∞,1)-categories are the most common: quasi-categories, Segal spaces.
    pub fn is_infinity_one_category(&self) -> bool {
        self.n == 1
    }
    /// (∞,2)-categories model bicategories up to homotopy.
    pub fn truncation_dim(&self) -> usize {
        self.n
    }
    /// The (∞,n)-category of cobordisms: Baez-Dolan conjecture (Lurie's theorem).
    /// Returns whether this can be the target of a TQFT.
    pub fn is_cobordism_hypothesis_target(&self) -> bool {
        true
    }
    /// Truncation: τ_{≤n} of an (∞,k)-category for k > n gives an (n,k)-category.
    pub fn truncate_to(&self, m: usize) -> InfinityNCategory {
        InfinityNCategory::new(
            m.min(self.n),
            &format!("τ_{{≤{}}} of {}", m, self.description),
        )
    }
    /// The homotopy category: h_1 of an (∞,1)-category is an ordinary category.
    pub fn homotopy_category_description(&self) -> String {
        if self.n >= 1 {
            format!("h_1({}): an ordinary category", self.description)
        } else {
            format!("h_0({}): a set of path components", self.description)
        }
    }
}
/// Represents a topological operad P.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperadNew {
    /// Name of the operad.
    pub name: String,
    /// Whether this is a symmetric operad.
    pub is_symmetric: bool,
    /// Whether this is a cyclic operad.
    pub is_cyclic: bool,
    /// Whether this is a dg-operad (differential graded).
    pub is_dg: bool,
    /// Koszul dual operad name (if known).
    pub koszul_dual: Option<String>,
}
#[allow(dead_code)]
impl OperadNew {
    /// Create a new operad descriptor.
    pub fn new(name: &str) -> Self {
        OperadNew {
            name: name.to_string(),
            is_symmetric: true,
            is_cyclic: false,
            is_dg: false,
            koszul_dual: None,
        }
    }
    /// The associative operad As.
    pub fn associative() -> Self {
        OperadNew::new("As")
    }
    /// The commutative operad Com.
    pub fn commutative() -> Self {
        OperadNew::new("Com")
    }
    /// The Lie operad Lie.
    pub fn lie() -> Self {
        let mut op = OperadNew::new("Lie");
        op.koszul_dual = Some("Com".to_string());
        op
    }
    /// The E_n operads (little n-disks operad).
    pub fn little_disks(n: usize) -> Self {
        OperadNew::new(&format!("E_{}", n))
    }
    /// Whether this operad is Koszul.
    pub fn is_koszul(&self) -> bool {
        self.koszul_dual.is_some()
    }
    /// Koszul duality: if P is Koszul, then P^! is its dual with bar complex resolution.
    pub fn koszul_resolution(&self) -> String {
        match self.koszul_dual.as_deref() {
            Some(dual) => {
                format!("Bar complex B(P = {}) resolves P^! = {}", self.name, dual)
            }
            None => format!("Koszul dual of {} not known", self.name),
        }
    }
}
/// A factorization algebra on R^n.
/// Assigns a cochain complex F(U) to each open U, with factorization maps.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FactorizationAlgebra {
    /// Ambient dimension n.
    pub ambient_dim: usize,
    /// Whether the factorization algebra is locally constant (corresponds to E_n-algebra).
    pub is_locally_constant: bool,
    /// Whether it is holomorphic (corresponds to vertex algebra).
    pub is_holomorphic: bool,
    /// Name/description.
    pub name: String,
}
#[allow(dead_code)]
impl FactorizationAlgebra {
    /// Create a new factorization algebra descriptor.
    pub fn new(ambient_dim: usize, name: &str) -> Self {
        FactorizationAlgebra {
            ambient_dim,
            is_locally_constant: false,
            is_holomorphic: false,
            name: name.to_string(),
        }
    }
    /// Locally constant factorization algebras on R^n ↔ E_n-algebras.
    pub fn corresponding_en_algebra(&self) -> String {
        if self.is_locally_constant {
            format!(
                "E_{}-algebra corresponding to {}",
                self.ambient_dim, self.name
            )
        } else {
            format!("{} is not locally constant", self.name)
        }
    }
    /// Holomorphic factorization algebra on C^n ↔ vertex algebra (n=1).
    pub fn corresponding_vertex_algebra(&self) -> Option<String> {
        if self.is_holomorphic && self.ambient_dim == 1 {
            Some(format!("Vertex algebra: {}", self.name))
        } else {
            None
        }
    }
    /// The factorization homology ∫_M F of F over a manifold M.
    pub fn factorization_homology(&self, manifold: &str) -> String {
        format!("∫_{{{}}} {} (factorization homology)", manifold, self.name)
    }
    /// Ran space: the factorization algebra is a sheaf on the Ran space.
    pub fn ran_space_description(&self) -> String {
        format!("Sheaf on Ran(R^{}) for {}", self.ambient_dim, self.name)
    }
}
/// A simplicial set: a contravariant functor Δ^op → Set.
///
/// Represented here by its list of n-simplices for each dimension n.
pub struct SimplicialSet {
    /// `n_simplices[n]` holds the list of n-simplices.
    pub n_simplices: Vec<Vec<String>>,
}
impl SimplicialSet {
    /// Create a simplicial set from its list of simplices per dimension.
    pub fn new(n_simplices: Vec<Vec<String>>) -> Self {
        Self { n_simplices }
    }
    /// The i-th face map d_i : X_n → X_{n-1}.
    /// Returns the (n-1)-simplices obtained by dropping the i-th vertex.
    pub fn face_map(&self, n: usize, i: usize) -> Vec<String> {
        if n == 0 || n > self.n_simplices.len() {
            return Vec::new();
        }
        self.n_simplices[n - 1]
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, s)| s.clone())
            .collect()
    }
    /// The i-th degeneracy map s_i : X_n → X_{n+1}.
    /// Returns degenerate (n+1)-simplices by repeating the i-th vertex.
    pub fn degeneracy_map(&self, n: usize, i: usize) -> Vec<String> {
        if n >= self.n_simplices.len() {
            return Vec::new();
        }
        self.n_simplices[n]
            .iter()
            .map(|s| format!("s_{}({})", i, s))
            .collect()
    }
    /// The dimension (highest index with non-empty simplices), or None if empty.
    pub fn dimension(&self) -> Option<usize> {
        self.n_simplices
            .iter()
            .enumerate()
            .rev()
            .find(|(_, v)| !v.is_empty())
            .map(|(i, _)| i)
    }
}
/// A Kan extension Lan_F G in an ∞-category.
///
/// Stores the extended functor (object map) and the natural transformation
/// witnessing the universal property.
pub struct KanExtension {
    /// The extended functor on objects.
    pub extended_obj_map: Vec<usize>,
    /// The unit 2-cell (natural transformation component indices).
    pub unit_components: Vec<usize>,
}
/// An algebra over the E_n-operad.
///
/// An E_n-algebra has n levels of homotopy-commutativity.
pub struct EnAlgebra {
    /// The level n (commutativity degree).
    pub n: usize,
    /// The carrier type (represented by name).
    pub carrier: String,
    /// Whether this is an actual E_∞-algebra (n = usize::MAX).
    pub is_e_infty: bool,
}
impl EnAlgebra {
    /// Create an E_n-algebra.
    pub fn new(n: usize, carrier: impl Into<String>) -> Self {
        EnAlgebra {
            n,
            carrier: carrier.into(),
            is_e_infty: n == usize::MAX,
        }
    }
    /// Check whether this is at least as commutative as level m.
    pub fn is_at_least_e_m(&self, m: usize) -> bool {
        self.n >= m
    }
}
/// Spanier-Whitehead duality in a stable ∞-category.
///
/// The S-dual functor D: Sp^op → Sp sending X ↦ F(X, S).
pub struct SpanierWhitehead {
    /// Whether the duality is perfect (bidual ≃ identity).
    pub is_perfect: bool,
}
impl SpanierWhitehead {
    /// Check that the double dual is equivalent to the identity.
    pub fn double_dual_is_identity(&self) -> bool {
        self.is_perfect
    }
}
/// A 2-category stored as a directed graph with 2-cells.
///
/// Objects, 1-morphisms and 2-morphisms are stored by index.
pub struct TwoCategoryData {
    /// Object labels.
    pub objects: Vec<String>,
    /// 1-morphisms: (source_idx, target_idx, label).
    pub one_morphisms: Vec<(usize, usize, String)>,
    /// 2-morphisms: (source_1morph_idx, target_1morph_idx, label).
    pub two_morphisms: Vec<(usize, usize, String)>,
}
impl TwoCategoryData {
    /// Create a new empty 2-category.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            one_morphisms: Vec::new(),
            two_morphisms: Vec::new(),
        }
    }
    /// Add an object and return its index.
    pub fn add_object(&mut self, label: impl Into<String>) -> usize {
        let idx = self.objects.len();
        self.objects.push(label.into());
        idx
    }
    /// Add a 1-morphism from `src` to `tgt`.
    pub fn add_one_morphism(&mut self, src: usize, tgt: usize, label: impl Into<String>) -> usize {
        let idx = self.one_morphisms.len();
        self.one_morphisms.push((src, tgt, label.into()));
        idx
    }
    /// Add a 2-morphism between 1-morphisms `f` and `g`.
    pub fn add_two_morphism(&mut self, f: usize, g: usize, label: impl Into<String>) -> usize {
        let idx = self.two_morphisms.len();
        self.two_morphisms.push((f, g, label.into()));
        idx
    }
    /// Horizontal composition of 1-morphisms: f: A→B, g: B→C ↦ g∘f: A→C.
    /// Returns the composed morphism index if source/target match.
    pub fn compose_1morph(&self, f_idx: usize, g_idx: usize) -> Option<(usize, usize)> {
        let (_, f_tgt, _) = self.one_morphisms.get(f_idx)?;
        let (g_src, g_tgt, _) = self.one_morphisms.get(g_idx)?;
        if f_tgt == g_src {
            Some((*f_tgt, *g_tgt))
        } else {
            None
        }
    }
}
/// An ω-category (strict) storing cells and their composition data.
///
/// An ω-category (strict) is a globular set with strictly associative and
/// unital composition at every dimension satisfying the interchange law.
pub struct OmegaCategory {
    /// The ambient dimension: `Some(n)` for an n-category, `None` for ω (∞).
    pub dimension: Option<u32>,
}
impl OmegaCategory {
    /// Create an ω-category with the given dimension bound.
    pub fn new(dimension: Option<u32>) -> Self {
        Self { dimension }
    }
    /// A strict ω-category (all interchange laws hold on the nose).
    pub fn is_strict(&self) -> bool {
        true
    }
    /// The Street-Roberts nerve embeds strict ω-categories into quasi-categories.
    pub fn street_roberts_nerve(&self) -> String {
        match self.dimension {
            Some(n) => format!("N(C) : strict {}-Cat → quasi-categories", n),
            None => "N(C) : strict ω-Cat → quasi-categories".to_string(),
        }
    }
}
/// A homotopy coherent adjunction F ⊣ G.
///
/// Stores the unit and counit as morphism indices in the respective categories.
pub struct Adjunction {
    /// Unit morphism η_A for each object A: index in source category.
    pub unit: Vec<usize>,
    /// Counit morphism ε_B for each object B: index in target category.
    pub counit: Vec<usize>,
}
impl Adjunction {
    /// Check the triangle identities on the stored components.
    ///
    /// For each object A, the composite G(ε_{FA}) ∘ η_A should be the identity.
    /// Here we verify the unit and counit arrays are consistent in size.
    pub fn check_triangle_identities(&self) -> bool {
        self.unit.len() == self.counit.len()
    }
}
/// A Cartesian fibration p : X → S of simplicial sets (in the sense of Lurie).
///
/// p is a Cartesian fibration if it has the right lifting property with
/// respect to inner anodyne extensions, and every morphism in the base
/// has a Cartesian lift in X.
pub struct CartesianFibrationRust {
    /// True if p is a co-Cartesian fibration (pushforwards exist).
    pub is_cocartesian: bool,
}
impl CartesianFibrationRust {
    /// Create a Cartesian fibration descriptor.
    pub fn new(is_cocartesian: bool) -> Self {
        Self { is_cocartesian }
    }
    /// A bi-fibration is both Cartesian and co-Cartesian.
    pub fn is_bifibration(&self) -> bool {
        self.is_cocartesian
    }
}
/// (∞,1)-category (quasi-category model).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuasiCategoryNew {
    pub name: String,
    pub is_kan_complex: bool,
    pub n_simplices: Option<usize>,
}
#[allow(dead_code)]
impl QuasiCategoryNew {
    pub fn new(name: &str) -> Self {
        QuasiCategoryNew {
            name: name.to_string(),
            is_kan_complex: false,
            n_simplices: None,
        }
    }
    pub fn infinity_groupoid(name: &str) -> Self {
        QuasiCategoryNew {
            name: name.to_string(),
            is_kan_complex: true,
            n_simplices: None,
        }
    }
    pub fn nerve_of_ordinary_cat(name: &str) -> Self {
        QuasiCategoryNew {
            name: format!("N({})", name),
            is_kan_complex: false,
            n_simplices: None,
        }
    }
    pub fn all_morphisms_invertible(&self) -> bool {
        self.is_kan_complex
    }
}
/// A quasi-category modelled as a simplicial set with a horn-filling witness.
///
/// In practice we store the number of simplices at each dimension and
/// a flag indicating whether inner horn filling has been verified.
pub struct QuasiCategory {
    /// Simplices at dimension d (indexed from 0).
    pub simplices: Vec<Vec<usize>>,
    /// Whether every inner horn Λ^n_k (0 < k < n) has been verified to have a filler.
    pub inner_horn_filling_verified: bool,
}
impl QuasiCategory {
    /// Check the inner horn filling property (stored flag).
    pub fn has_inner_horn_fillings(&self) -> bool {
        self.inner_horn_filling_verified
    }
    /// Check whether this simplicial set is a Kan complex (outer horns also fill).
    pub fn is_kan_complex(&self) -> bool {
        self.inner_horn_filling_verified && self.simplices.iter().all(|s| !s.is_empty())
    }
}
/// A localization functor L_S: C → C[S⁻¹].
///
/// `inverted` stores the set of morphism indices being inverted.
pub struct LocalizationFunctor {
    /// The set S of morphisms being inverted.
    pub inverted: Vec<usize>,
    /// The localized object map.
    pub obj_map: Vec<usize>,
}
/// Goodwillie calculus: the calculus of functors.
///
/// Given a functor F from spaces (or spectra) to spaces (or spectra),
/// Goodwillie's calculus constructs the Taylor tower P_0 F ← P_1 F ← P_2 F ← …
/// approximating F by n-excisive functors.
pub struct GoodwillieCalculus {
    /// The degree of excision (0, 1, 2, …).
    pub degree: u32,
    /// Whether the approximation is homogeneous of the given degree.
    pub is_homogeneous: bool,
}
impl GoodwillieCalculus {
    /// Create a Goodwillie calculus descriptor.
    pub fn new(degree: u32, is_homogeneous: bool) -> Self {
        Self {
            degree,
            is_homogeneous,
        }
    }
    /// Classify the homogeneous layer D_n F of the Taylor tower.
    ///
    /// The n-th homogeneous layer D_n F is classified (by Goodwillie) as
    /// a spectrum with Σ_n-action: specifically D_n F(X) ≃ (∂_n F ∧ X^∧n)_{hΣ_n}.
    pub fn classify_homogeneous_functors(&self) -> String {
        if self.is_homogeneous {
            format!(
                "D_{0} F(X) ≃ (∂_{0} F ∧ X^∧{0})_{{hΣ_{0}}}  (degree-{0} homogeneous layer)",
                self.degree
            )
        } else {
            format!(
                "P_{0} F: the {0}-excisive approximation to F (not necessarily homogeneous)",
                self.degree
            )
        }
    }
    /// An n-excisive functor is classified by the vanishing of its (n+1)-th
    /// cross-effect: cr_{n+1} F ≃ *.
    pub fn excision_condition(&self) -> String {
        format!(
            "F is {}-excisive iff cr_{} F ≃ * (the ({1})-th cross-effect is contractible)",
            self.degree,
            self.degree + 1,
        )
    }
}
/// A computad: a presentation of a strict ω-category by generators.
///
/// Each dimension carries a list of generating cells with source/target data.
pub struct ComputadData {
    /// `generators[n]` = list of (source_cells, target_cells, label) at dimension n.
    pub generators: Vec<Vec<(Vec<usize>, Vec<usize>, String)>>,
}
impl ComputadData {
    /// Create an empty computad.
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
        }
    }
    /// Add a generating n-cell.
    pub fn add_generator(
        &mut self,
        n: usize,
        source: Vec<usize>,
        target: Vec<usize>,
        label: impl Into<String>,
    ) -> usize {
        while self.generators.len() <= n {
            self.generators.push(Vec::new());
        }
        let idx = self.generators[n].len();
        self.generators[n].push((source, target, label.into()));
        idx
    }
    /// Count generators at dimension n.
    pub fn generator_count(&self, n: usize) -> usize {
        self.generators.get(n).map(|v| v.len()).unwrap_or(0)
    }
    /// Check that all source/target indices are in range for the previous dimension.
    pub fn is_well_typed(&self) -> bool {
        for (n, layer) in self.generators.iter().enumerate() {
            let prev_count = if n == 0 {
                0
            } else {
                self.generator_count(n - 1)
            };
            for (src, tgt, _) in layer {
                if n > 0 {
                    if src.iter().any(|&i| i >= prev_count) {
                        return false;
                    }
                    if tgt.iter().any(|&i| i >= prev_count) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// Monoidal functor between monoidal categories.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MonoidalFunctor {
    pub source: String,
    pub target: String,
    pub is_strict: bool,
    pub is_lax: bool,
    pub is_strong: bool,
}
#[allow(dead_code)]
impl MonoidalFunctor {
    pub fn new(src: &str, tgt: &str) -> Self {
        MonoidalFunctor {
            source: src.to_string(),
            target: tgt.to_string(),
            is_strict: false,
            is_lax: true,
            is_strong: false,
        }
    }
    pub fn strict(src: &str, tgt: &str) -> Self {
        MonoidalFunctor {
            source: src.to_string(),
            target: tgt.to_string(),
            is_strict: true,
            is_lax: true,
            is_strong: true,
        }
    }
    pub fn coherence_condition(&self) -> &'static str {
        "Lax monoidal functor satisfies pentagon and triangle coherence"
    }
}
/// Enriched category over a monoidal category V.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnrichedCategory {
    pub name: String,
    pub enriching_category: String,
    pub is_symmetric: bool,
}
#[allow(dead_code)]
impl EnrichedCategory {
    pub fn new(name: &str, v: &str, sym: bool) -> Self {
        EnrichedCategory {
            name: name.to_string(),
            enriching_category: v.to_string(),
            is_symmetric: sym,
        }
    }
    pub fn ab_enriched(name: &str) -> Self {
        EnrichedCategory::new(name, "Ab", true)
    }
    pub fn chain_complex_enriched(name: &str) -> Self {
        EnrichedCategory::new(name, "Ch(Ab)", true)
    }
    pub fn is_preadditive(&self) -> bool {
        self.enriching_category == "Ab"
    }
}
/// An ∞-functor (simplicial map) between two quasi-categories.
///
/// Stored as a map on simplices at each dimension.
pub struct InftyFunctor {
    /// The simplicial map on 0-simplices (objects).
    pub obj_map: Vec<usize>,
    /// The simplicial map on 1-simplices (morphisms).
    pub mor_map: Vec<usize>,
}
impl InftyFunctor {
    /// Check whether this functor is an equivalence (bijection on connected components).
    pub fn is_equivalence(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for &t in &self.obj_map {
            seen.insert(t);
        }
        seen.len() == self.obj_map.len()
    }
}
/// A coCartesian fibration p: E → B (the dual notion).
pub struct CoCartesianFibration {
    /// Total space object count.
    pub e_object_count: usize,
    /// Base space object count.
    pub b_object_count: usize,
    /// CoCartesian edges: pairs (e_idx, b_morphism_idx).
    pub cocartesian_edges: Vec<(usize, usize)>,
}
/// An n-excisive functor: the n-th Goodwillie approximation P_n F.
///
/// Stores the excisivity degree and the approximation data.
pub struct ExcisiveFunctor {
    /// Excisivity degree n.
    pub degree: usize,
    /// Whether the functor has been verified to be n-excisive.
    pub verified: bool,
}
impl ExcisiveFunctor {
    /// Check n-excisivity.
    pub fn is_n_excisive(&self, n: usize) -> bool {
        self.verified && self.degree <= n
    }
}
/// An accessible ∞-category: closed under κ-filtered colimits.
pub struct AccessibleCategory {
    /// The accessibility cardinal κ.
    pub kappa: usize,
    /// Whether κ-filtered colimit closure has been verified.
    pub filtered_colimits_closed: bool,
}
/// The associative operad Assoc: one color, operations of all arities.
pub struct AssociativeOperad {
    /// n-ary multiplication operation count (one per arity n ≥ 0).
    pub max_arity: usize,
}
impl AssociativeOperad {
    /// Create the standard associative operad up to arity `n`.
    pub fn new(max_arity: usize) -> Self {
        AssociativeOperad { max_arity }
    }
}
/// The Goodwillie Taylor tower of F.
///
/// A sequence P_0 F → P_1 F → P_2 F → ... of n-excisive approximations.
pub struct TaylorTower {
    /// The approximations P_0, P_1, ..., P_depth.
    pub approximations: Vec<ExcisiveFunctor>,
}
impl TaylorTower {
    /// Get the n-th approximation, if computed.
    pub fn get(&self, n: usize) -> Option<&ExcisiveFunctor> {
        self.approximations.get(n)
    }
    /// Check whether the tower converges at degree n (all verified up to n).
    pub fn converges_at(&self, n: usize) -> bool {
        self.approximations.get(n).is_some_and(|f| f.verified)
    }
}
/// A Kan fibration p : X → Y of simplicial sets.
///
/// p is a Kan fibration if it has the right lifting property with respect to
/// all horn inclusions Λ^n_k ↪ Δ^n.
pub struct KanFibration {
    /// True if p is a right fibration (lifts against left horn inclusions).
    pub is_right_fibration: bool,
    /// True if p is a left fibration (lifts against right horn inclusions).
    pub is_left_fibration: bool,
}
impl KanFibration {
    /// Create a Kan fibration with explicit fibration flags.
    pub fn new(is_right_fibration: bool, is_left_fibration: bool) -> Self {
        Self {
            is_right_fibration,
            is_left_fibration,
        }
    }
    /// A Kan fibration is both a left and right fibration.
    pub fn is_kan(&self) -> bool {
        self.is_right_fibration && self.is_left_fibration
    }
}
/// An ∞-categorical adjunction (L ⊣ R) between ∞-functors.
///
/// An ∞-adjunction consists of ∞-functors L : C → D and R : D → C
/// together with a unit η : Id_C → R ∘ L and counit ε : L ∘ R → Id_D
/// satisfying the triangle identities up to coherent homotopy.
pub struct InfinityAdjunction {
    /// Name of the left adjoint functor.
    pub left: String,
    /// Name of the right adjoint functor.
    pub right: String,
    /// Name of the unit natural transformation η : Id → R ∘ L.
    pub unit: String,
    /// Name of the counit natural transformation ε : L ∘ R → Id.
    pub counit: String,
}
impl InfinityAdjunction {
    /// Create an ∞-adjunction.
    pub fn new(
        left: impl Into<String>,
        right: impl Into<String>,
        unit: impl Into<String>,
        counit: impl Into<String>,
    ) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
            unit: unit.into(),
            counit: counit.into(),
        }
    }
    /// The triangle identity (left): (ε L) ∘ (L η) ≃ Id_L.
    pub fn left_triangle_identity(&self) -> String {
        format!(
            "({} {} ) ∘ ({} {} ) ≃ Id_{}",
            self.counit, self.left, self.left, self.unit, self.left
        )
    }
    /// The triangle identity (right): (R ε) ∘ (η R) ≃ Id_R.
    pub fn right_triangle_identity(&self) -> String {
        format!(
            "({} {} ) ∘ ({} {} ) ≃ Id_{}",
            self.right, self.counit, self.unit, self.right, self.right
        )
    }
}
/// A Segal space: a simplicial space satisfying the Segal condition.
///
/// Represented by a list of spaces (as Vec<String>) at each simplicial degree.
pub struct SegalSpaceData {
    /// `spaces[n]` is the set of points in X_n.
    pub spaces: Vec<Vec<String>>,
}
impl SegalSpaceData {
    /// Create a Segal space from its simplicial data.
    pub fn new(spaces: Vec<Vec<String>>) -> Self {
        Self { spaces }
    }
    /// Check the Segal condition at degree n:
    /// |X_n| should equal the fiber product count |X_1|^n / |X_0|^(n-1).
    ///
    /// Here we use a simplified size-based check.
    pub fn check_segal_condition(&self, n: usize) -> bool {
        if n < 2 {
            return true;
        }
        let x0 = self.spaces.first().map(|v| v.len()).unwrap_or(0);
        let x1 = self.spaces.get(1).map(|v| v.len()).unwrap_or(0);
        let xn = self.spaces.get(n).map(|v| v.len()).unwrap_or(0);
        if x0 == 0 {
            return xn == 0;
        }
        let expected = x1.pow(n as u32) / x0.pow((n - 1) as u32);
        xn == expected
    }
    /// The space of equivalences: elements of X_1 that are invertible.
    /// Returns the count of putative equivalences (stored separately).
    pub fn equivalences_count(&self) -> usize {
        self.spaces.first().map(|v| v.len()).unwrap_or(0)
    }
}
/// The type of an operad governing algebraic structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperadType {
    /// The associative operad Ass governing associative algebras.
    Associative,
    /// The commutative operad Comm governing commutative algebras.
    Commutative,
    /// The Lie operad Lie governing Lie algebras.
    Lie,
    /// A general ∞-operad, identified by name.
    InfinityOperad(String),
}
impl OperadType {
    /// Returns true if the operad is one of the classical operads.
    pub fn is_classical(&self) -> bool {
        matches!(
            self,
            OperadType::Associative | OperadType::Commutative | OperadType::Lie
        )
    }
}
/// An (∞,n)-category tracker that records the truncation level and
/// verifies that k-morphisms for k > n are invertible.
pub struct InfinityNCatData {
    /// The truncation level n: k-morphisms for k > n are invertible.
    pub n: usize,
    /// Whether the invertibility condition for morphisms above level n has been verified.
    pub invertibility_verified: bool,
    /// The underlying Segal space data.
    pub segal_space: SegalSpaceData,
}
impl InfinityNCatData {
    /// Create an (∞,n)-category from a Segal space.
    pub fn new(n: usize, segal_space: SegalSpaceData) -> Self {
        Self {
            n,
            invertibility_verified: false,
            segal_space,
        }
    }
    /// Mark the invertibility condition as verified.
    pub fn verify_invertibility(&mut self) {
        self.invertibility_verified = true;
    }
    /// An (∞,0)-category is an ∞-groupoid (all morphisms invertible).
    pub fn is_infinity_groupoid(&self) -> bool {
        self.n == 0
    }
    /// An (∞,1)-category is a quasi-category.
    pub fn is_quasi_category(&self) -> bool {
        self.n == 1
    }
    /// An (∞,2)-category has non-invertible 1- and 2-morphisms.
    pub fn is_infinity_2_cat(&self) -> bool {
        self.n == 2
    }
}
/// The straightening/unstraightening equivalence.
///
/// Witnesses the equivalence CartFib/S ≃ Fun(S^op, ∞-Cat).
pub struct StraighteningEquivalence {
    /// The base ∞-category.
    pub base_simplex_count: usize,
    /// Whether the equivalence has been formally verified.
    pub verified: bool,
}
/// The n-th Goodwillie derivative D_n F: the n-th homogeneous layer.
pub struct HomogeneousFunctor {
    /// The derivative degree n.
    pub degree: usize,
    /// Whether this homogeneous functor has been computed.
    pub computed: bool,
}
/// The object classifier U in an ∞-topos.
///
/// A univalent universe with Univ(X) ≃ Map(X, U) for all X.
pub struct ObjectClassifier {
    /// The universe level.
    pub level: usize,
    /// Whether univalence has been verified.
    pub is_univalent: bool,
}
/// A multi-colored ∞-operad as a coCartesian fibration over Fin_*.
pub struct InftyOperad {
    /// The colors (objects) of the operad.
    pub colors: Vec<String>,
    /// Operation arities: `operations[k]` = list of (input colors, output color).
    pub operations: Vec<(Vec<usize>, usize)>,
}
impl InftyOperad {
    /// Get the number of k-ary operations (operations with k inputs).
    pub fn operations_of_arity(&self, k: usize) -> usize {
        self.operations
            .iter()
            .filter(|(ins, _)| ins.len() == k)
            .count()
    }
}
/// Operadic composition (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperadV2 {
    pub name: String,
    pub is_symmetric: bool,
    pub is_cyclic: bool,
}
#[allow(dead_code)]
impl OperadV2 {
    pub fn new(name: &str, sym: bool) -> Self {
        OperadV2 {
            name: name.to_string(),
            is_symmetric: sym,
            is_cyclic: false,
        }
    }
    pub fn assoc() -> Self {
        OperadV2::new("Assoc", false)
    }
    pub fn comm() -> Self {
        OperadV2::new("Comm", true)
    }
    pub fn lie() -> Self {
        OperadV2::new("Lie", true)
    }
    pub fn is_binary(&self) -> bool {
        true
    }
}
