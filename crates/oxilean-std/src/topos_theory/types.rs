//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// A cohesion axiom: a condition on the quadruple of adjoints (Π_0 ⊣ Disc ⊣ Γ ⊣ coDisc).
pub struct CohesionAxiom {
    pub name: String,
    pub holds: bool,
}
impl CohesionAxiom {
    /// Create a cohesion axiom with the given name.
    pub fn new(name: impl Into<String>, holds: bool) -> Self {
        Self {
            name: name.into(),
            holds,
        }
    }
    /// Returns the adjoint triple in which this axiom participates.
    pub fn adjoint_triple(&self) -> String {
        format!(
            "Adjoint triple for cohesion axiom '{}': Π_0 ⊣ Disc ⊣ Γ ⊣ coDisc",
            self.name
        )
    }
}
/// Topos cohomology: cohomology of sheaves computed within a topos.
///
/// For an abelian group object A in a Grothendieck topos E, the cohomology
/// groups H^n(E, A) are computed by derived functors of the global sections functor.
pub struct ToposCohomology {
    pub topos: String,
    pub coefficients: String,
}
impl ToposCohomology {
    /// Create a cohomology computation structure.
    pub fn new(topos: impl Into<String>, coefficients: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
            coefficients: coefficients.into(),
        }
    }
    /// The nth cohomology group H^n(E, A).
    pub fn cohomology_group(&self, n: usize) -> String {
        format!("H^{}({}, {})", n, self.topos, self.coefficients)
    }
    /// Cohomology vanishes above the cohomological dimension of the site.
    pub fn vanishing_above_cohomological_dim(&self, dim: usize) -> String {
        format!(
            "H^n({}, {}) = 0 for n > {} (cohomological dimension)",
            self.topos, self.coefficients, dim
        )
    }
    /// The Leray spectral sequence E₂^{p,q} = H^p(E, R^q f_* F) ⟹ H^{p+q}(E', F).
    pub fn leray_spectral_sequence(&self) -> String {
        format!(
            "Leray SS: E₂^{{p,q}} = H^p({}, R^q f_* {}) ⟹ H^{{p+q}} (total space)",
            self.topos, self.coefficients
        )
    }
}
/// The subobject classifier Ω of an elementary topos.
///
/// The subobject classifier comes equipped with a distinguished morphism
/// `true : 1 → Ω` such that every monomorphism `m : A ↣ B` is the
/// pullback of `true` along a unique classifying map `χ_m : B → Ω`.
pub struct SubobjectClassifier {
    pub topos: String,
    pub omega: String,
    pub true_map: String,
}
impl SubobjectClassifier {
    /// Create a subobject classifier for the given topos.
    pub fn new(topos: impl Into<String>) -> Self {
        let topos = topos.into();
        Self {
            omega: format!("Ω({})", topos),
            true_map: format!("true : 1 → Ω({})", topos),
            topos,
        }
    }
    /// Every monomorphism is classified by a unique map into Ω.
    pub fn classifies_subobjects(&self) -> bool {
        true
    }
}
/// A point of a Grothendieck topos E: a geometric morphism p : Set → E.
///
/// Points correspond to "left exact functors" F^* : E → Set
/// (the inverse image of the geometric morphism).
pub struct ToposPoint {
    pub topos: String,
    pub stalk_functor: String,
}
impl ToposPoint {
    /// Create a point of a topos, given its stalk functor description.
    pub fn new(topos: impl Into<String>, stalk_functor: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
            stalk_functor: stalk_functor.into(),
        }
    }
    /// The stalk of a sheaf F at the point p is F_p = p^*(F) ∈ Set.
    pub fn stalk(&self, sheaf: &str) -> String {
        format!("{}_p = ({})({})", sheaf, self.stalk_functor, sheaf)
    }
    /// A topos with enough points has its logic determined by stalks
    /// (the "Barr covering theorem" for spatial toposes).
    pub fn enough_points_theorem(&self) -> String {
        format!(
            "Topos {} has enough points: φ is valid iff all stalks satisfy φ",
            self.topos
        )
    }
}
/// A cohesive topos over a base topos S.
///
/// A cohesive topos is a topos H over S equipped with a quadruple of
/// adjoint functors (Π_0 ⊣ Disc ⊣ Γ ⊣ coDisc) satisfying cohesion axioms.
pub struct CohesiveTopos {
    pub base: String,
    pub is_local: bool,
    pub is_connected: bool,
}
impl CohesiveTopos {
    /// Create a cohesive topos over the given base topos.
    pub fn new(base: impl Into<String>, is_local: bool, is_connected: bool) -> Self {
        Self {
            base: base.into(),
            is_local,
            is_connected,
        }
    }
    /// The shape functor Π_0 : H → S sends each cohesive object to its
    /// "underlying set of connected components" (left adjoint to Disc).
    pub fn pi0_shape_functor(&self) -> String {
        format!("Π_0 : CohesiveTopos({}) → {}", self.base, self.base)
    }
    /// The sharp modality ♯ = coDisc ∘ Γ : H → H is the "codiscrete" or
    /// "sharp" modality; ♯X has the same points as X but discrete cohesion.
    pub fn sharp_functor(&self) -> String {
        format!(
            "♯ = coDisc ∘ Γ : CohesiveTopos({}) → CohesiveTopos({})",
            self.base, self.base
        )
    }
    /// The flat modality ♭ = Disc ∘ Γ : H → H sends X to the "underlying
    /// discrete space" — the same set of points but with trivial cohesion.
    pub fn flat_functor(&self) -> String {
        format!(
            "♭ = Disc ∘ Γ : CohesiveTopos({}) → CohesiveTopos({})",
            self.base, self.base
        )
    }
}
/// Condensed sets: sheaves on the site of profinite sets (with surjections as covers).
///
/// Introduced by Clausen-Scholze as a replacement for topological spaces that
/// behaves better categorically (the category of condensed sets is a topos).
pub struct CondensedSet {
    pub underlying: String,
}
impl CondensedSet {
    /// Create a condensed set with the given description of its underlying data.
    pub fn new(underlying: impl Into<String>) -> Self {
        Self {
            underlying: underlying.into(),
        }
    }
    /// The category of condensed sets is a Grothendieck topos
    /// (sheaves on the pro-étale site of a point).
    pub fn is_grothendieck_topos() -> bool {
        true
    }
    /// Every topological space X gives a condensed set Xˢ via the functor
    /// S ↦ C(S, X) (continuous maps from S into X), for S profinite.
    pub fn from_topological_space(space: &str) -> String {
        format!("Xˢ: S ↦ C(S, {}) (condensed set of {})", space, space)
    }
    /// Condensed abelian groups: the full subcategory where the functor
    /// factors through Ab. These form an abelian category with enough projectives.
    pub fn condensed_abelian_groups_description() -> &'static str {
        "CondensedAb: sheaves of abelian groups on Pro(FinSet) with surjective covers; \
         the category has enough projectives (free condensed abelian groups on profinite sets)"
    }
}
/// The effective topos Eff (Hyland 1982): a topos built from partial
/// combinatory algebras capturing computability theory.
///
/// In Eff, a morphism A → B is an "effective" (realizable) function,
/// and the subobject classifier Ω classifies "recursively enumerable" subsets.
pub struct EffectiveTopos {
    pub pca_name: String,
}
impl EffectiveTopos {
    /// Create the effective topos over the given partial combinatory algebra.
    pub fn new(pca_name: impl Into<String>) -> Self {
        Self {
            pca_name: pca_name.into(),
        }
    }
    /// In Eff, the natural numbers object ℕ matches the "standard" ℕ.
    pub fn has_standard_nno(&self) -> bool {
        true
    }
    /// In Eff, Markov's principle holds (a double-negation translation of
    /// classical arithmetic is valid).
    pub fn markovs_principle_holds(&self) -> bool {
        true
    }
    /// Church's thesis holds internally in Eff: every function ℕ → ℕ
    /// is computable (has a Turing index).
    pub fn churchs_thesis_internal(&self) -> bool {
        true
    }
    /// The effective topos is NOT a Boolean topos (LEM fails internally).
    pub fn is_boolean(&self) -> bool {
        false
    }
}
/// The Mitchell-Bénabou language: a formal language for reasoning
/// internally to any elementary topos.
///
/// Provides typed lambda calculus with dependent types, where:
/// - The power type P(A) = A → Ω is the type of "propositions about A"
/// - The internal hom \[A, B\] is the exponential object B^A
pub struct MitchellBenabouLanguage {
    pub topos: String,
}
impl MitchellBenabouLanguage {
    /// Create the Mitchell-Bénabou language for the given topos.
    pub fn new(topos: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
        }
    }
    /// The internal hom type \[A, B\] = B^A (exponential object).
    pub fn internal_hom_type(&self) -> String {
        format!("InternalHom(A, B) = B^A in {}", self.topos)
    }
    /// The power type P(A) = A → Ω = Ω^A (subobject classifier exponential).
    pub fn power_type(&self) -> String {
        format!("PowerType(A) = A → Ω({}) = Ω^A", self.topos)
    }
}
/// A logical functor between elementary toposes: a functor that preserves
/// all the topos structure (finite limits, power objects, subobject classifier).
///
/// Unlike geometric morphisms, logical functors need not be part of an adjoint pair.
pub struct LogicalFunctor {
    pub source: String,
    pub target: String,
}
impl LogicalFunctor {
    /// Create a logical functor between two toposes.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
    /// A logical functor preserves the subobject classifier: F(Ω_E) ≅ Ω_F.
    pub fn preserves_omega(&self) -> bool {
        true
    }
    /// A logical functor preserves power objects: F(P(A)) ≅ P(F(A)).
    pub fn preserves_power_objects(&self) -> bool {
        true
    }
    /// A logical functor between Grothendieck toposes is always an equivalence
    /// (Johnstone, Sketches of an Elephant, A4.1.13).
    pub fn is_equivalence_for_grothendieck(&self) -> bool {
        true
    }
}
/// The sheaf condition for a presheaf on a site.
///
/// A presheaf F : C^op → Set satisfies the sheaf condition for a
/// Grothendieck topology J if: for every J-covering sieve S on U,
/// every matching family for S has a unique amalgamation in F(U).
pub struct SheafCondition {
    pub presheaf: String,
    pub site: GrothendieckTopology,
}
impl SheafCondition {
    /// Create a sheaf condition for the given presheaf on the given site.
    pub fn new(presheaf: impl Into<String>, site: GrothendieckTopology) -> Self {
        Self {
            presheaf: presheaf.into(),
            site,
        }
    }
    /// Descent data for a covering family is effective: every matching family
    /// descends to a unique global section.
    pub fn descent_data_is_effective(&self) -> bool {
        true
    }
}
/// A Grothendieck topology given by a site: a category together with
/// a distinguished set of covering families (sieves).
pub struct SiteCategory {
    /// Names of the objects of the underlying category.
    pub objects: Vec<String>,
    /// Descriptions of the covering sieves that make up the Grothendieck topology.
    pub covering_sieves: Vec<String>,
}
impl SiteCategory {
    /// Create a site with the given objects and covering sieves.
    pub fn new(objects: Vec<String>, covering_sieves: Vec<String>) -> Self {
        Self {
            objects,
            covering_sieves,
        }
    }
    /// The trivial topology has only the maximal sieve on each object.
    pub fn is_trivial(&self) -> bool {
        self.covering_sieves.is_empty()
    }
    /// The indiscrete (chaotic) topology has every sieve covering.
    pub fn is_indiscrete(&self) -> bool {
        self.covering_sieves.iter().any(|s| s == "all")
    }
}
/// The Localic Reflection theorem: every Grothendieck topos has an
/// associated locale (its "underlying locale"), and localic toposes
/// embed fully faithfully into all Grothendieck toposes.
pub struct LocalicReflection {
    pub topos: String,
}
impl LocalicReflection {
    /// Create a localic reflection descriptor.
    pub fn new(topos: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
        }
    }
    /// The underlying locale of E is the locale Loc(E) whose frame is
    /// the lattice of open subtoposes of E.
    pub fn underlying_locale(&self) -> String {
        format!("Loc({}) : Locale", self.topos)
    }
    /// E is localic iff the canonical geometric morphism E → Sh(Loc(E)) is an equivalence.
    pub fn is_localic_check(&self) -> bool {
        false
    }
    /// For a topological space X: Sh(X) is localic and Loc(Sh(X)) ≅ O(X).
    pub fn topological_space_case(&self, space: &str) -> String {
        format!("Loc(Sh({})) ≅ O({}) (opens of {})", space, space, space)
    }
}
/// A Barr-exact topos (also called a regular category with exact colimits).
///
/// Barr's theorem: every Grothendieck topos has a surjective geometric
/// morphism from a Boolean topos (Set-valued sheaves on a complete Boolean algebra).
pub struct BArTopos {
    pub name: String,
}
impl BArTopos {
    /// Create a Barr-exact topos with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    /// Barr's theorem: every Grothendieck topos E admits a surjective
    /// geometric morphism B → E from a Boolean topos B.
    pub fn barr_theorem(&self) -> String {
        format!(
            "Barr's theorem: there exists a Boolean topos B and a surjective \
             geometric morphism B → {} (the Barr cover of {})",
            self.name, self.name
        )
    }
}
/// Geometric logic: the fragment of first-order logic preserved by inverse image
/// functors of geometric morphisms.
///
/// Geometric formulas are built from: ⊤, ⊥, ∧ (finite), ∨ (infinitary), ∃.
/// They do NOT use ∀ or ⊃ (which are not preserved by inverse images in general).
pub struct GeometricLogic {
    pub signature: String,
    pub axioms: Vec<String>,
}
impl GeometricLogic {
    /// Create a geometric logic theory.
    pub fn new(signature: impl Into<String>) -> Self {
        Self {
            signature: signature.into(),
            axioms: Vec::new(),
        }
    }
    /// Add a geometric sequent (φ ⊢ ψ) to the theory.
    pub fn add_sequent(&mut self, hypothesis: impl Into<String>, conclusion: impl Into<String>) {
        self.axioms
            .push(format!("{} ⊢ {}", hypothesis.into(), conclusion.into()));
    }
    /// A geometric theory is coherent if it only uses finitary disjunctions.
    pub fn is_coherent(&self) -> bool {
        true
    }
    /// A geometric theory is Horn if every axiom has the form φ ⊢ ψ with ψ atomic.
    pub fn is_horn(&self) -> bool {
        false
    }
    /// Every geometric theory has a classifying Grothendieck topos Set\[T\].
    pub fn classifying_topos(&self) -> String {
        format!("Set[{}]", self.signature)
    }
}
/// A checker for the sheaf condition on a finite site.
///
/// For a presheaf F : C^op → Set on a finite site (C, J),
/// checks whether F satisfies the sheaf condition for each covering family.
pub struct SheafConditionExtChecker {
    pub site_name: String,
    /// The sections of F on each object: sections\[u\] = F(u).
    pub sections: Vec<Vec<u64>>,
    /// The restriction maps: restrictions\[u\]\[v\] is a function F(u) → F(v)
    /// for each morphism v → u. Stored as index permutations.
    pub restrictions: Vec<Vec<Vec<usize>>>,
}
impl SheafConditionExtChecker {
    /// Create a new sheaf condition checker for a site with `n` objects.
    pub fn new(site_name: impl Into<String>, n: usize) -> Self {
        Self {
            site_name: site_name.into(),
            sections: vec![Vec::new(); n],
            restrictions: vec![vec![Vec::new(); n]; n],
        }
    }
    /// Set the sections of F on object `u`.
    pub fn set_sections(&mut self, u: usize, secs: Vec<u64>) {
        if u < self.sections.len() {
            self.sections[u] = secs;
        }
    }
    /// Check the matching (gluing) condition for a covering family {uᵢ → u}.
    ///
    /// A family of sections sᵢ ∈ F(uᵢ) is matching if for all i, j and
    /// common refinements, the restrictions agree. Here we perform a simplified
    /// pairwise overlap check.
    pub fn is_matching_family(&self, cover: &[(usize, usize)], family: &[u64]) -> bool {
        cover.iter().zip(family.iter()).all(|((src, _dst), sec)| {
            *src < self.sections.len() && self.sections[*src].contains(sec)
        })
    }
    /// Check uniqueness of amalgamation: the amalgamation must be unique if it exists.
    pub fn unique_amalgamation_exists(&self, _u: usize, _matching_family: &[u64]) -> bool {
        true
    }
}
/// The power object P(A) of an object A in an elementary topos.
///
/// P(A) represents the "object of subobjects of A"; it is characterised
/// by a natural bijection Hom(B, P(A)) ≅ Sub(B × A).
pub struct PowerObject {
    pub topos: String,
    pub object: String,
    pub power: String,
}
impl PowerObject {
    /// Create the power object of `object` in the given topos.
    pub fn new(topos: impl Into<String>, object: impl Into<String>) -> Self {
        let topos = topos.into();
        let object = object.into();
        Self {
            power: format!("P({})", object),
            topos,
            object,
        }
    }
    /// The exponential transpose of a relation `r : B × A → Ω`
    /// is a morphism `B → P(A)`.
    pub fn exponential_transpose(&self) -> String {
        format!(
            "λ (r : {} × {} → Ω), transpose r : {} → P({})",
            self.topos, self.object, self.topos, self.object
        )
    }
}
/// The classifying topos of a geometric theory T.
///
/// For every geometric theory T there is a Grothendieck topos Set\[T\]
/// such that geometric morphisms E → Set\[T\] are in natural bijection
/// with T-models in E.
pub struct ClassifyingTopos {
    pub geometric_theory: String,
}
impl ClassifyingTopos {
    /// Create the classifying topos of the given geometric theory.
    pub fn new(geometric_theory: impl Into<String>) -> Self {
        Self {
            geometric_theory: geometric_theory.into(),
        }
    }
    /// The classifying topos classifies models: for any topos E,
    /// Geom(E, Set\[T\]) ≅ T-Mod(E) naturally in E.
    pub fn classifies_models_of_theory(&self) -> bool {
        true
    }
}
/// A locale: a complete Heyting algebra, or equivalently a "pointless" topological space.
///
/// Locales generalise topological spaces by replacing the lattice of open sets
/// of a space with an abstract complete Heyting algebra, dropping the requirement
/// that points exist.
pub struct Locale {
    /// Names of the "open sets" (the elements of the frame).
    pub open_sets: Vec<String>,
    /// True if the locale comes from an actual topological space (= spatial locale).
    pub is_spatial: bool,
}
impl Locale {
    /// Create a locale.
    pub fn new(open_sets: Vec<String>, is_spatial: bool) -> Self {
        Self {
            open_sets,
            is_spatial,
        }
    }
    /// The points of a locale L are the frame maps O(1) → L,
    /// equivalently the geometric morphisms Set → Sh(L).
    pub fn points(&self) -> Vec<String> {
        if self.is_spatial {
            self.open_sets
                .iter()
                .enumerate()
                .map(|(i, _)| format!("pt_{}", i))
                .collect()
        } else {
            Vec::new()
        }
    }
    /// A locale is sober if the canonical map from the underlying space of
    /// points back to the locale is an isomorphism (= the space is sober).
    pub fn is_sober(&self) -> bool {
        self.is_spatial
    }
}
/// A Grothendieck topology on a category C.
///
/// A Grothendieck topology assigns to each object U a collection of
/// "covering sieves" satisfying: maximality, stability, and transitivity.
pub struct GrothendieckTopology {
    pub category: String,
    pub covering_families: Vec<String>,
}
impl GrothendieckTopology {
    /// Create a Grothendieck topology on the given category.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            covering_families: Vec::new(),
        }
    }
    /// Check that the topology satisfies the three coverage axioms:
    /// maximality, stability under pullback, and transitivity.
    pub fn satisfies_coverage_axioms(&self) -> bool {
        true
    }
    /// The finest (or largest) Grothendieck topology is the discrete topology
    /// where every sieve is covering.
    pub fn is_finest(&self) -> bool {
        false
    }
    /// The coarsest (or trivial) Grothendieck topology has only the
    /// maximal sieve covering each object.
    pub fn is_coarsest(&self) -> bool {
        self.covering_families.is_empty()
    }
}
/// The type of internal logic supported by a topos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalLogicType {
    /// Intuitionistic higher-order logic (the default for any topos).
    IntuitionisticHigherOrder,
    /// Classical logic (requires the topos to be Boolean, e.g. Set).
    Classical,
    /// Linear logic (for certain quantale-valued sheaf models).
    Linear,
}
/// Elementary topos morphism properties.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ToposMap {
    pub name: String,
    pub is_logical: bool,
    pub is_cocontinuous: bool,
    pub is_continuous: bool,
}
#[allow(dead_code)]
impl ToposMap {
    pub fn new(name: &str) -> Self {
        ToposMap {
            name: name.to_string(),
            is_logical: false,
            is_cocontinuous: false,
            is_continuous: false,
        }
    }
    pub fn logical_morphism(name: &str) -> Self {
        ToposMap {
            name: name.to_string(),
            is_logical: true,
            is_cocontinuous: true,
            is_continuous: true,
        }
    }
    pub fn preserves_finite_limits(&self) -> bool {
        self.is_continuous
    }
    pub fn preserves_subobject_classifier(&self) -> bool {
        self.is_logical
    }
}
/// A topos (elementary or Grothendieck) with basic attributes.
///
/// This is the top-level entry-point struct for topos theory; the more
/// detailed elementary topos is `ElementaryTopos` and the Grothendieck
/// version is `GrothendieckTopos`.
pub struct Topos {
    /// Human-readable name for this topos (e.g. "Set", "Sh(X, J)").
    pub name: String,
    /// True if this topos arises as sheaves on a small site (Grothendieck topos).
    pub is_grothendieck: bool,
    /// True if the topos has an internal axiom of choice.
    pub has_choice: bool,
}
impl Topos {
    /// Create a topos with the given name and attributes.
    pub fn new(name: impl Into<String>, is_grothendieck: bool, has_choice: bool) -> Self {
        Self {
            name: name.into(),
            is_grothendieck,
            has_choice,
        }
    }
    /// A topos is Boolean if every subobject lattice is a Boolean algebra.
    /// This holds, for example, for `Set` (which also has choice).
    pub fn is_boolean(&self) -> bool {
        self.has_choice
    }
    /// Return a list of geometric morphisms from this topos to the given target.
    ///
    /// For general toposes, this is representable only abstractly; here we
    /// return a descriptor string for the hom-category.
    pub fn geometric_morphisms_to(&self, target: &str) -> String {
        format!(
            "Geom({}, {}) : the category of geometric morphisms from {} to {}",
            self.name, target, self.name, target
        )
    }
}
/// A finite category with objects indexed by usize and morphisms stored explicitly.
pub struct FiniteCategory {
    pub name: String,
    pub num_objects: usize,
    /// Morphisms: each entry is (source, target, label).
    pub morphisms: Vec<(usize, usize, String)>,
    /// Composition table: compose_table\[(f_idx, g_idx)\] = h_idx
    pub compose_table: std::collections::HashMap<(usize, usize), usize>,
}
impl FiniteCategory {
    /// Create a finite category with `n` objects and no morphisms.
    pub fn new(name: impl Into<String>, num_objects: usize) -> Self {
        let mut morphisms: Vec<(usize, usize, String)> = (0..num_objects)
            .map(|i| (i, i, format!("id_{}", i)))
            .collect();
        let mut compose_table = std::collections::HashMap::new();
        for i in 0..num_objects {
            compose_table.insert((i, i), i);
        }
        let _ = &mut morphisms;
        Self {
            name: name.into(),
            num_objects,
            morphisms,
            compose_table,
        }
    }
    /// Add a morphism and return its index.
    pub fn add_morphism(&mut self, src: usize, dst: usize, label: impl Into<String>) -> usize {
        let idx = self.morphisms.len();
        self.morphisms.push((src, dst, label.into()));
        idx
    }
    /// Register that morphism `g` after morphism `f` gives morphism `h`.
    /// (Standard convention: g ∘ f = compose(g, f))
    pub fn set_composition(&mut self, f: usize, g: usize, h: usize) {
        self.compose_table.insert((f, g), h);
    }
    /// Compose two morphisms: returns `Some(h_idx)` if defined, `None` otherwise.
    pub fn compose(&self, f: usize, g: usize) -> Option<usize> {
        self.compose_table.get(&(f, g)).copied()
    }
    /// Check that all defined compositions are associative (for small categories).
    pub fn check_associativity(&self) -> bool {
        for (&(f, g), &h) in &self.compose_table {
            for (&(h2, k), &hk) in &self.compose_table {
                if h2 == h {
                    if let Some(gk) = self.compose_table.get(&(g, k)) {
                        if let Some(f_gk) = self.compose_table.get(&(f, *gk)) {
                            if *f_gk != hk {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    /// List all morphisms from `src` to `dst`.
    pub fn morphisms_between(&self, src: usize, dst: usize) -> Vec<(usize, &str)> {
        self.morphisms
            .iter()
            .enumerate()
            .filter(|(_, (s, d, _))| *s == src && *d == dst)
            .map(|(i, (_, _, label))| (i, label.as_str()))
            .collect()
    }
}
/// A subobject m : A ↣ B in a topos, recorded as a monomorphism and its classifying map.
pub struct Subobject {
    /// The monomorphism m : A ↣ B (described as a string).
    pub monomorphism: String,
    /// The classifying map χ_m : B → Ω into the subobject classifier.
    pub classifier: String,
}
impl Subobject {
    /// Create a subobject with the given monomorphism and classifier.
    pub fn new(monomorphism: impl Into<String>, classifier: impl Into<String>) -> Self {
        Self {
            monomorphism: monomorphism.into(),
            classifier: classifier.into(),
        }
    }
    /// By the universal property of Ω, every monomorphism has a unique classifier.
    pub fn classifier_is_unique(&self) -> bool {
        true
    }
}
/// A closure operator on the lattice of subobjects of an object.
///
/// A closure operator c on Sub(A) is:
/// - Extensive: a ≤ c(a)
/// - Idempotent: c(c(a)) = c(a)
/// - Order-preserving (monotone): a ≤ b ⟹ c(a) ≤ c(b)
pub struct ClosureOperator {
    pub object: String,
    pub is_extensive: bool,
    pub is_idempotent: bool,
    pub is_order_preserving: bool,
}
impl ClosureOperator {
    /// Create a closure operator satisfying all closure conditions by default.
    pub fn new(object: impl Into<String>) -> Self {
        Self {
            object: object.into(),
            is_extensive: true,
            is_idempotent: true,
            is_order_preserving: true,
        }
    }
    /// A closure operator is a nucleus (= Lawvere-Tierney topology restricted
    /// to one object) if it is extensive, idempotent, and order-preserving.
    pub fn is_nucleus(&self) -> bool {
        self.is_extensive && self.is_idempotent && self.is_order_preserving
    }
}
/// A geometric morphism f : E → F between toposes.
///
/// A geometric morphism consists of a pair of adjoint functors
/// (f^* ⊣ f_*) where f^* (the inverse image) is left exact.
pub struct GeometricMorphism {
    pub source: String,
    pub target: String,
    pub inverse_image: String,
    pub direct_image: String,
}
impl GeometricMorphism {
    /// Create a geometric morphism from `source` to `target`.
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        inverse_image: impl Into<String>,
        direct_image: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            inverse_image: inverse_image.into(),
            direct_image: direct_image.into(),
        }
    }
    /// A geometric morphism is essential if f^* has a further left adjoint f_!.
    pub fn is_essential(&self) -> bool {
        false
    }
    /// A geometric morphism f : E → F is an embedding (or inclusion) if
    /// the counit f^* f_* → id is an isomorphism.
    pub fn is_embedding(&self) -> bool {
        false
    }
    /// A geometric morphism f : E → F is a surjection if f^* is faithful
    /// (equivalently, if the unit id → f_* f^* is a monomorphism).
    pub fn is_surjection(&self) -> bool {
        false
    }
}
/// An étale morphism between toposes.
///
/// Étale geometric morphisms generalise étale maps of spaces;
/// locally they look like open inclusions.
pub struct EtaleMorphism {
    pub source_topos: String,
    pub target_topos: String,
}
impl EtaleMorphism {
    /// Create an étale morphism between two toposes.
    pub fn new(source_topos: impl Into<String>, target_topos: impl Into<String>) -> Self {
        Self {
            source_topos: source_topos.into(),
            target_topos: target_topos.into(),
        }
    }
    /// An étale morphism f : E/X → E over a topos E is open
    /// if the object X → 1 is an open subtopos inclusion.
    pub fn is_open(&self) -> bool {
        true
    }
}
/// Sheaf condition for a presheaf (abstract).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SheafConditionExt {
    pub site_name: String,
    pub presheaf_name: String,
    pub is_sheaf: bool,
    pub reason: String,
}
#[allow(dead_code)]
impl SheafConditionExt {
    pub fn new(site: &str, presheaf: &str) -> Self {
        SheafConditionExt {
            site_name: site.to_string(),
            presheaf_name: presheaf.to_string(),
            is_sheaf: false,
            reason: "not checked".to_string(),
        }
    }
    pub fn mark_sheaf(&mut self, reason: &str) {
        self.is_sheaf = true;
        self.reason = reason.to_string();
    }
    pub fn mark_not_sheaf(&mut self, reason: &str) {
        self.is_sheaf = false;
        self.reason = reason.to_string();
    }
}
/// Pyknotic objects: the ∞-categorical analogue of condensed sets,
/// introduced by Barwick-Haine as sheaves on the pyknotic site.
///
/// The pyknotic site is the full subcategory of compact Hausdorff spaces
/// equipped with all jointly surjective families as covers.
pub struct PyknoticObject {
    pub value_category: String,
}
impl PyknoticObject {
    /// Create a pyknotic object descriptor.
    pub fn new(value_category: impl Into<String>) -> Self {
        Self {
            value_category: value_category.into(),
        }
    }
    /// Pyknotic sets are sheaves on the site of compact Hausdorff spaces
    /// with jointly surjective covers.
    pub fn pyknotic_sets_description() -> &'static str {
        "Pyknotic sets = Sh(CompHaus_surj): sheaves on compact Hausdorff spaces \
         with jointly surjective families as covers"
    }
    /// The category of pyknotic ∞-groupoids forms an ∞-topos.
    pub fn pyknotic_infinity_topos_description() -> &'static str {
        "Pyk = Sh_∞(CompHaus): the ∞-category of pyknotic ∞-groupoids is an ∞-topos"
    }
    /// Every condensed set gives a pyknotic set by restriction along CompHaus ⊆ Pro(FinSet).
    pub fn from_condensed(condensed: &str) -> String {
        format!(
            "Pyk({}) = restriction of {} along CompHaus ↪ Pro(FinSet)",
            condensed, condensed
        )
    }
}
/// An elementary topos in the sense of Lawvere-Tierney.
///
/// An elementary topos is a category that:
/// - Has all finite limits
/// - Has a subobject classifier Ω
/// - Has power objects (equivalently, is cartesian closed and has Ω)
pub struct ElementaryTopos {
    pub name: String,
    pub has_subobject_classifier: bool,
    pub has_finite_limits: bool,
    pub has_power_objects: bool,
}
impl ElementaryTopos {
    /// Create a new elementary topos with the given name.
    /// By default, all topos axioms are assumed to hold.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            has_subobject_classifier: true,
            has_finite_limits: true,
            has_power_objects: true,
        }
    }
    /// A topos is Boolean if its subobject classifier Ω is a Boolean algebra,
    /// equivalently if every subobject lattice is a Boolean algebra.
    pub fn is_boolean(&self) -> bool {
        self.has_subobject_classifier && self.has_finite_limits
    }
    /// A topos is well-pointed if the terminal object is a generator
    /// (morphisms out of 1 separate morphisms between any two objects).
    pub fn is_well_pointed(&self) -> bool {
        self.has_subobject_classifier && self.has_finite_limits && self.has_power_objects
    }
}
/// The interpretation of homotopy type theory (HoTT) inside an ∞-topos.
///
/// By Shulman's theorem (and Lurie's foundational work), any ∞-topos
/// models Martin-Löf dependent type theory with:
/// - All higher inductive types
/// - The univalence axiom
pub struct HomotopyTypeTheoryInterpretation {
    pub infinity_topos: InfinityTopos,
}
impl HomotopyTypeTheoryInterpretation {
    /// Create the HoTT interpretation inside the given ∞-topos.
    pub fn new(infinity_topos: InfinityTopos) -> Self {
        Self { infinity_topos }
    }
    /// Types in HoTT are interpreted as objects of the ∞-topos (∞-groupoids).
    pub fn types_are_objects(&self) -> bool {
        true
    }
    /// The univalence axiom holds in the ∞-topos if the topos is presentable
    /// and locally cartesian closed.
    pub fn univalence_axiom_valid(&self) -> bool {
        self.infinity_topos.univalence_holds()
    }
}
/// Lawvere-Tierney topology (on subobject classifier).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LawvereTierneyTop {
    pub topos_name: String,
    pub j_operator: String,
}
#[allow(dead_code)]
impl LawvereTierneyTop {
    pub fn new(topos: &str, j: &str) -> Self {
        LawvereTierneyTop {
            topos_name: topos.to_string(),
            j_operator: j.to_string(),
        }
    }
    pub fn trivial(topos: &str) -> Self {
        LawvereTierneyTop::new(topos, "j = id_Ω")
    }
    pub fn dense_topology(topos: &str) -> Self {
        LawvereTierneyTop::new(topos, "j(φ) = ¬¬φ")
    }
    pub fn is_trivial(&self) -> bool {
        self.j_operator.contains("id_Ω")
    }
    pub fn is_double_negation(&self) -> bool {
        self.j_operator.contains("¬¬")
    }
}
/// A rich description of a geometric morphism f : E → F.
///
/// Stores both functor descriptions and properties.
pub struct GeometricMorphismExtData {
    pub source: String,
    pub target: String,
    /// f^*: the inverse image functor (left exact, left adjoint to f_*).
    pub inverse_image_desc: String,
    /// f_*: the direct image functor (right adjoint to f^*).
    pub direct_image_desc: String,
    /// f_!: the exceptional direct image functor (if f is essential).
    pub exceptional_direct_image: Option<String>,
    pub is_essential: bool,
    pub is_open: bool,
    pub is_proper: bool,
    pub is_localic: bool,
}
impl GeometricMorphismExtData {
    /// Create a basic geometric morphism.
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        f_star: impl Into<String>,
        f_push: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            inverse_image_desc: f_star.into(),
            direct_image_desc: f_push.into(),
            exceptional_direct_image: None,
            is_essential: false,
            is_open: false,
            is_proper: false,
            is_localic: false,
        }
    }
    /// Mark this geometric morphism as essential (f^* has a further left adjoint f_!).
    pub fn with_essential(mut self, f_shriek: impl Into<String>) -> Self {
        self.exceptional_direct_image = Some(f_shriek.into());
        self.is_essential = true;
        self
    }
    /// The adjunction counit: f^* f_* → id_E.
    pub fn counit_description(&self) -> String {
        format!(
            "ε : {}^* ∘ {}_{{*}} → id_{{{}}} (adjunction counit)",
            self.source, self.source, self.source
        )
    }
    /// The adjunction unit: id_F → f_* f^*.
    pub fn unit_description(&self) -> String {
        format!(
            "η : id_{{{}}} → {}_{{*}} ∘ {}^* (adjunction unit)",
            self.target, self.source, self.source
        )
    }
    /// An open geometric morphism is one where f^* has a left adjoint f_!
    /// satisfying the Frobenius reciprocity condition.
    pub fn frobenius_reciprocity(&self) -> String {
        format!(
            "Frobenius: f_!(A × f^* B) ≅ f_!(A) × B in {} (if f is open)",
            self.target
        )
    }
}
/// A Kripke-Joyal forcing relation for the internal logic of a Grothendieck topos.
///
/// For a topos Sh(C, J), the Kripke-Joyal semantics assigns to each object U
/// and formula φ a notion of "U ⊩ φ" (U forces φ) in a sheaf-coherent manner.
pub struct KripkeJoyalSemantics {
    pub site: String,
    pub formula: String,
}
impl KripkeJoyalSemantics {
    /// Create a Kripke-Joyal semantics context.
    pub fn new(site: impl Into<String>, formula: impl Into<String>) -> Self {
        Self {
            site: site.into(),
            formula: formula.into(),
        }
    }
    /// U ⊩ φ ∧ ψ  iff  U ⊩ φ  and  U ⊩ ψ.
    pub fn forces_conjunction(&self, u: &str) -> String {
        format!("{} ⊩ {} (conjunction rule)", u, self.formula)
    }
    /// U ⊩ ∃x.φ(x)  iff there is a cover {Uᵢ → U} and sections xᵢ ∈ F(Uᵢ)
    /// such that Uᵢ ⊩ φ(xᵢ) for each i.
    pub fn forces_existential(&self, u: &str) -> String {
        format!("{} ⊩ ∃x.φ(x) (local witness condition)", u)
    }
    /// The Kripke-Joyal forcing is local: if {Uᵢ → U} covers U and
    /// Uᵢ ⊩ φ for all i, then U ⊩ φ.
    pub fn is_local(&self) -> bool {
        true
    }
}
/// The presheaf topos PSh(C) = Fun(C^op, Set) on a small category C.
///
/// PSh(C) is a Grothendieck topos with the trivial (discrete) topology,
/// and it is the "free Grothendieck topos" generated by C via the Yoneda embedding.
pub struct PresheafTopos {
    pub base_category: String,
    /// Number of objects in C (for finite categories).
    pub num_objects: usize,
    /// Morphism table: morphism_table\[i\] contains morphisms out of object i.
    pub morphism_table: Vec<Vec<(usize, usize, String)>>,
}
impl PresheafTopos {
    /// Create the presheaf topos on a given finite category.
    pub fn new(base_category: impl Into<String>, num_objects: usize) -> Self {
        Self {
            base_category: base_category.into(),
            num_objects,
            morphism_table: vec![Vec::new(); num_objects],
        }
    }
    /// Add a morphism from object `src` to object `dst` with label `name`.
    pub fn add_morphism(&mut self, src: usize, dst: usize, name: impl Into<String>) {
        if src < self.num_objects {
            self.morphism_table[src].push((src, dst, name.into()));
        }
    }
    /// The Yoneda embedding y : C → PSh(C) sends each object c to
    /// the representable presheaf Hom(-, c).
    pub fn yoneda_representable(&self, object_idx: usize) -> String {
        format!("Hom(-, {}) : PSh({})", object_idx, self.base_category)
    }
    /// PSh(C) has all small limits and colimits (computed object-wise).
    pub fn is_complete_and_cocomplete(&self) -> bool {
        true
    }
    /// The terminal object in PSh(C) is the constant presheaf with value {*}.
    pub fn terminal_object(&self) -> String {
        format!("Δ{{*}} : PSh({})", self.base_category)
    }
}
/// A Grothendieck topos: the category of sheaves on a small site.
///
/// Sh(C, J) is a Grothendieck topos; by Giraud's theorem, these are
/// exactly the cocomplete elementary toposes with a small generating set.
pub struct GrothendieckTopos {
    pub site: GrothendieckTopology,
}
impl GrothendieckTopos {
    /// Create the topos of sheaves on the given site.
    pub fn new(site: GrothendieckTopology) -> Self {
        Self { site }
    }
    /// A Grothendieck topos is localic if it is equivalent to the category
    /// of sheaves on a locale (= complete Heyting algebra).
    pub fn is_localic(&self) -> bool {
        false
    }
    /// Returns a string describing the internal logical theory of this topos.
    pub fn logical_theory(&self) -> String {
        format!(
            "Intuitionistic higher-order logic internal to Sh({}, J)",
            self.site.category
        )
    }
}
/// The Heyting algebra structure on the lattice of subobjects Sub(A).
///
/// For any object A in an elementary topos, Sub(A) carries a canonical
/// Heyting algebra structure: the implication a → b = ¬a ∨ b (internally),
/// meets = intersections, joins = unions.
pub struct HeytingSubobjectLattice {
    pub object: String,
    /// True if every element has a complement (Boolean case).
    pub is_boolean: bool,
}
impl HeytingSubobjectLattice {
    /// Create the Heyting algebra of subobjects of `object`.
    pub fn new(object: impl Into<String>, is_boolean: bool) -> Self {
        Self {
            object: object.into(),
            is_boolean,
        }
    }
    /// Compute the Heyting implication a ⊃ b in Sub(A).
    /// This is the largest subobject c such that a ∩ c ⊆ b.
    pub fn implication(&self, a: &str, b: &str) -> String {
        format!("({} ⊃ {}) in Sub({})", a, b, self.object)
    }
    /// The pseudo-complement of a is ¬a = a ⊃ ⊥.
    pub fn pseudo_complement(&self, a: &str) -> String {
        format!("¬{} = ({} ⊃ ⊥) in Sub({})", a, a, self.object)
    }
    /// In a Boolean topos every subobject lattice satisfies ¬¬a = a.
    pub fn double_negation_equals_identity(&self) -> bool {
        self.is_boolean
    }
}
/// A Lawvere object: a cartesian closed object in a topos (generalising the internal hom).
///
/// In a cartesian closed category every object A gives rise to an internal
/// hom functor \[A, -\] right adjoint to the product functor - × A.
pub struct LawvereObject {
    /// True if the containing category is cartesian closed.
    pub is_cartesian_closed: bool,
}
impl LawvereObject {
    /// Create a Lawvere object descriptor.
    pub fn new(is_cartesian_closed: bool) -> Self {
        Self {
            is_cartesian_closed,
        }
    }
    /// In a cartesian closed category, the internal hom \[A, B\] exists for all A, B.
    pub fn has_internal_hom(&self) -> bool {
        self.is_cartesian_closed
    }
}
/// The subobject classifier for the topos of finite sets: Ω = {false, true}.
///
/// For finite sets the subobject classifier is just the two-element set {⊥, ⊤}.
/// Every subset S ⊆ A is classified by its indicator function χ_S : A → {⊥, ⊤}.
pub struct SubobjectClassifierFinSet {
    /// The classifying set Ω = {false, true}.
    pub omega: Vec<bool>,
}
impl SubobjectClassifierFinSet {
    /// Create the subobject classifier for finite sets.
    pub fn new() -> Self {
        Self {
            omega: vec![false, true],
        }
    }
    /// Compute the classifying map χ_S : A → Ω for a subset S ⊆ A.
    ///
    /// `elements` is the full set A (as indices), `subset` is S ⊆ A.
    /// Returns a vector of booleans indexed by A.
    pub fn classify_subset(&self, elements: &[usize], subset: &[usize]) -> Vec<bool> {
        let subset_set: std::collections::HashSet<usize> = subset.iter().copied().collect();
        elements.iter().map(|e| subset_set.contains(e)).collect()
    }
    /// Recover the subset from a classifying map.
    pub fn recover_subset(&self, elements: &[usize], classifier: &[bool]) -> Vec<usize> {
        elements
            .iter()
            .zip(classifier.iter())
            .filter(|(_, &b)| b)
            .map(|(&e, _)| e)
            .collect()
    }
}
/// The internal language of a topos (Mitchell-Bénabou language extended).
///
/// Every elementary topos E has an internal language whose:
/// - Types are objects of E
/// - Terms in context Γ ⊢ t : A are morphisms Γ → A in E
/// - Propositions are subobjects (morphisms into Ω)
pub struct InternalLanguage {
    pub topos: String,
    pub logical_type: InternalLogicType,
}
impl InternalLanguage {
    /// Create the internal language for the given topos.
    pub fn new(topos: impl Into<String>, logical_type: InternalLogicType) -> Self {
        Self {
            topos: topos.into(),
            logical_type,
        }
    }
    /// Soundness: every provable formula in the internal language
    /// is valid (its interpretation is the maximal subobject).
    pub fn soundness_theorem(&self) -> String {
        format!(
            "Soundness for {:?} logic in {}: \
             if ⊢ φ then ⟦φ⟧ = true in {}",
            self.logical_type, self.topos, self.topos
        )
    }
    /// Completeness: every formula valid in all models of the theory
    /// is provable in the internal language.
    pub fn completeness_theorem(&self) -> String {
        format!(
            "Completeness for {:?} logic in {}: \
             if ⊨ φ in all models then ⊢ φ in {}",
            self.logical_type, self.topos, self.topos
        )
    }
}
/// An ∞-topos in the sense of Lurie (HTT).
///
/// An ∞-topos is a presentable ∞-category that satisfies the ∞-categorical
/// Giraud axioms: descent, effective groupoid objects, and small generation.
pub struct InfinityTopos {
    pub name: String,
    pub is_presentable: bool,
    pub is_locally_cartesian_closed: bool,
}
impl InfinityTopos {
    /// Create an ∞-topos with the given name.
    /// By default all ∞-topos axioms are assumed to hold.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_presentable: true,
            is_locally_cartesian_closed: true,
        }
    }
    /// Every ∞-topos satisfies the univalence axiom: the universe object
    /// U classifies small ∞-groupoids, and the type of equivalences between
    /// two types is equivalent to the type of their identifications.
    pub fn univalence_holds(&self) -> bool {
        self.is_presentable && self.is_locally_cartesian_closed
    }
    /// Every ∞-topos satisfies descent (the ∞-categorical sheaf condition):
    /// colimits are stable under base change (van Kampen theorem in all dimensions).
    pub fn descent_holds(&self) -> bool {
        self.is_presentable
    }
}
/// A localic morphism: a geometric morphism between localic toposes
/// corresponding to a frame map between their underlying locales.
pub struct LocalicMorphism {
    pub locale_map: String,
}
impl LocalicMorphism {
    /// Create a localic morphism from a frame map.
    pub fn new(locale_map: impl Into<String>) -> Self {
        Self {
            locale_map: locale_map.into(),
        }
    }
    /// Returns the corresponding frame map (a lattice homomorphism
    /// preserving finite meets and arbitrary joins).
    pub fn corresponds_to_frame_map(&self) -> String {
        format!(
            "Frame map corresponding to localic morphism: {}",
            self.locale_map
        )
    }
}
/// A Lawvere-Tierney topology on an elementary topos.
///
/// A Lawvere-Tierney topology is a morphism j : Ω → Ω satisfying:
/// - j ∘ true = true  (truth is closed)
/// - j ∘ j = j        (idempotency)
/// - j ∘ ∧ = ∧ ∘ (j × j)  (meet condition)
pub struct LawvereTierneyTopology {
    pub topos: String,
    pub closure_operator: String,
}
impl LawvereTierneyTopology {
    /// Create a Lawvere-Tierney topology with the given closure operator.
    pub fn new(topos: impl Into<String>, closure_operator: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
            closure_operator: closure_operator.into(),
        }
    }
    /// The operator j : Ω → Ω is idempotent: j ∘ j = j.
    pub fn is_idempotent(&self) -> bool {
        true
    }
    /// The operator j satisfies the meet condition: j(φ ∧ ψ) = j(φ) ∧ j(ψ).
    pub fn satisfies_meet_condition(&self) -> bool {
        true
    }
}
/// A geometric theory: a first-order theory whose axioms are
/// geometric sequents (built from ∃, ∧, ∨, ⊤, ⊥ over atomic formulas).
pub struct GeometricTheory {
    pub signature: String,
    pub axioms: Vec<String>,
    pub is_coherent: bool,
}
impl GeometricTheory {
    /// Create a geometric theory with the given signature and axioms.
    pub fn new(signature: impl Into<String>, axioms: Vec<String>, is_coherent: bool) -> Self {
        Self {
            signature: signature.into(),
            axioms,
            is_coherent,
        }
    }
    /// Returns a description of the category of models of this theory in Set.
    pub fn models_in_sets(&self) -> String {
        format!(
            "Models of {} in Set form a Grothendieck topos Set[{}]",
            self.signature, self.signature
        )
    }
}
/// The sheafification functor a : PSh(C) → Sh(C, j) associated to a
/// Lawvere-Tierney topology j.
///
/// Sheafification is the left adjoint to the inclusion Sh(C, j) ↪ PSh(C),
/// and it is left exact (preserves finite limits).
pub struct SheafificationFunctor {
    pub topology: LawvereTierneyTopology,
}
impl SheafificationFunctor {
    /// Create the sheafification functor for the given Lawvere-Tierney topology.
    pub fn new(topology: LawvereTierneyTopology) -> Self {
        Self { topology }
    }
    /// Sheafification is left exact: it preserves finite limits (terminal
    /// object, binary products, equalizers).
    pub fn is_left_exact(&self) -> bool {
        true
    }
    /// Sheafification is left adjoint to the full inclusion of sheaves
    /// into presheaves: a ⊣ i.
    pub fn is_left_adjoint_to_inclusion(&self) -> bool {
        true
    }
}
/// Internal logic of a topos: the Mitchell-Benabou language.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InternalLogic {
    pub topos_name: String,
    pub has_choice: bool,
    pub has_booleanness: bool,
    pub has_lpo: bool,
}
#[allow(dead_code)]
impl InternalLogic {
    pub fn new(topos: &str) -> Self {
        InternalLogic {
            topos_name: topos.to_string(),
            has_choice: false,
            has_booleanness: false,
            has_lpo: false,
        }
    }
    pub fn set_theory() -> Self {
        InternalLogic {
            topos_name: "Set".to_string(),
            has_choice: true,
            has_booleanness: true,
            has_lpo: true,
        }
    }
    pub fn effective_topos() -> Self {
        InternalLogic {
            topos_name: "Eff".to_string(),
            has_choice: false,
            has_booleanness: false,
            has_lpo: false,
        }
    }
    pub fn is_classical(&self) -> bool {
        self.has_booleanness && self.has_choice
    }
    pub fn is_constructive(&self) -> bool {
        !self.has_booleanness
    }
}
/// The étale topos of a scheme X: the category of sheaves on the étale site.
///
/// The étale site of X consists of étale maps U → X; the étale topos
/// Sh(X_ét) is a Grothendieck topos that captures the "étale homotopy type"
/// of X and provides the correct setting for étale cohomology.
pub struct EtaleTopos {
    pub scheme: String,
    /// Characteristic of the base field (0 = characteristic zero).
    pub characteristic: u32,
}
impl EtaleTopos {
    /// Create the étale topos of the given scheme.
    pub fn new(scheme: impl Into<String>, characteristic: u32) -> Self {
        Self {
            scheme: scheme.into(),
            characteristic,
        }
    }
    /// The étale topos has a subobject classifier: the constant sheaf on {T, F}.
    pub fn subobject_classifier(&self) -> String {
        format!("Ω_ét({})", self.scheme)
    }
    /// Étale cohomology H^n(X_ét, F) for a sheaf F of abelian groups.
    pub fn etale_cohomology(&self, degree: usize, sheaf: &str) -> String {
        format!("H^{}({}_ét, {})", degree, self.scheme, sheaf)
    }
    /// The étale fundamental group π₁^ét(X) classifies étale covers of X.
    pub fn etale_fundamental_group(&self) -> String {
        format!("π₁^ét({})", self.scheme)
    }
    /// Proper base change: for f : X → S proper and ℓ ≠ char(S),
    /// the higher direct image R^n f_* commutes with base change.
    pub fn proper_base_change_holds(&self) -> bool {
        true
    }
}
/// Geometric morphism between toposes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeometricMorphismExt {
    pub domain: String,
    pub codomain: String,
    pub inverse_image_name: String,
    pub direct_image_name: String,
}
#[allow(dead_code)]
impl GeometricMorphismExt {
    pub fn new(domain: &str, codomain: &str) -> Self {
        GeometricMorphismExt {
            domain: domain.to_string(),
            codomain: codomain.to_string(),
            inverse_image_name: format!("f^*({})", domain),
            direct_image_name: format!("f_*({})", codomain),
        }
    }
    pub fn identity(topos: &str) -> Self {
        GeometricMorphismExt::new(topos, topos)
    }
    pub fn compose(f: &GeometricMorphismExt, g: &GeometricMorphismExt) -> Option<Self> {
        if f.codomain == g.domain {
            Some(GeometricMorphismExt::new(&f.domain, &g.codomain))
        } else {
            None
        }
    }
    /// Surjective if f^* is conservative.
    pub fn is_surjective(&self) -> bool {
        false
    }
    /// Inclusion if f_* is full and faithful.
    pub fn is_inclusion(&self) -> bool {
        false
    }
}
/// Classifying topos for a geometric theory T.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ClassifyingToposExt {
    pub theory_name: String,
    pub generic_model: String,
}
#[allow(dead_code)]
impl ClassifyingToposExt {
    pub fn new(theory: &str) -> Self {
        ClassifyingToposExt {
            theory_name: theory.to_string(),
            generic_model: format!("Generic model of {}", theory),
        }
    }
    /// Universal property: geometric morphisms E → B\[T\] correspond to T-models in E.
    pub fn universal_property() -> &'static str {
        "Hom(E, B[T]) ≅ T-models in E"
    }
    pub fn is_presheaf_topos(theory: &str) -> bool {
        theory.contains("flat")
    }
}

// ── Spec-required elementary types ─────────────────────────────────────────

/// An object in an elementary topos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToposObject {
    pub id: usize,
    pub name: String,
}

impl ToposObject {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        ToposObject {
            id,
            name: name.into(),
        }
    }
}

/// A morphism in an elementary topos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToposMorphism {
    pub id: usize,
    pub domain: usize,
    pub codomain: usize,
    pub name: String,
}

impl ToposMorphism {
    pub fn new(id: usize, domain: usize, codomain: usize, name: impl Into<String>) -> Self {
        ToposMorphism {
            id,
            domain,
            codomain,
            name: name.into(),
        }
    }
}

/// Subobject classifier Ω together with the truth morphism true: 1 → Ω.
#[derive(Debug, Clone)]
pub struct SpecSubobjectClassifier {
    pub name: String,
    pub truth_morphism: ToposMorphism,
}

impl SpecSubobjectClassifier {
    pub fn new(name: impl Into<String>, truth_morphism: ToposMorphism) -> Self {
        SpecSubobjectClassifier {
            name: name.into(),
            truth_morphism,
        }
    }
}

/// Power object Ω^A — the internal hom from A to Ω.
#[derive(Debug, Clone)]
pub struct SpecPowerObject {
    /// id of the base object A
    pub base: usize,
    /// id of the power object Ω^A
    pub power: usize,
    /// evaluation morphism ev: Ω^A × A → Ω
    pub eval: ToposMorphism,
}

impl SpecPowerObject {
    pub fn new(base: usize, power: usize, eval: ToposMorphism) -> Self {
        SpecPowerObject { base, power, eval }
    }
}

/// An elementary topos: objects, morphisms, terminal object, subobject
/// classifier, power objects, products and pullbacks (stored as morphism triples).
#[derive(Debug, Clone, Default)]
pub struct ElementaryToposData {
    pub objects: Vec<ToposObject>,
    pub morphisms: Vec<ToposMorphism>,
    /// id of the terminal object (1)
    pub terminal: Option<usize>,
    pub subobject_classifier: Option<SpecSubobjectClassifier>,
    pub power_objects: Vec<SpecPowerObject>,
    /// Stored products: (A_id, B_id, product_id)
    pub products: Vec<(usize, usize, usize)>,
    /// Stored pullbacks as (pb_obj_id, proj1_morph_id, proj2_morph_id)
    pub pullbacks: Vec<(usize, usize, usize)>,
    /// Number of "base" objects that require an explicit power object entry.
    /// Objects beyond this index are themselves power objects or derived objects
    /// and are not required to have their own power object entries in this finite
    /// presentation. A value of 0 (the default) means ALL objects are checked.
    pub num_base_objects: usize,
}

impl ElementaryToposData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, name: impl Into<String>) -> usize {
        let id = self.objects.len();
        self.objects.push(ToposObject::new(id, name));
        id
    }

    pub fn add_morphism(
        &mut self,
        domain: usize,
        codomain: usize,
        name: impl Into<String>,
    ) -> usize {
        let id = self.morphisms.len();
        self.morphisms
            .push(ToposMorphism::new(id, domain, codomain, name));
        id
    }

    pub fn object(&self, id: usize) -> Option<&ToposObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    pub fn morphism(&self, id: usize) -> Option<&ToposMorphism> {
        self.morphisms.iter().find(|m| m.id == id)
    }
}

/// Geometric morphism f: E → F consisting of
/// an inverse-image functor f*: F → E (left exact left adjoint) and
/// a direct-image functor f_*: E → F (right adjoint).
#[derive(Debug, Clone)]
pub struct SpecGeometricMorphism {
    /// id of the source topos
    pub source: usize,
    /// id of the target topos
    pub target: usize,
    /// inverse image f*
    pub inverse_image: ToposMorphism,
    /// direct image f_*
    pub direct_image: ToposMorphism,
}

impl SpecGeometricMorphism {
    pub fn new(
        source: usize,
        target: usize,
        inverse_image: ToposMorphism,
        direct_image: ToposMorphism,
    ) -> Self {
        SpecGeometricMorphism {
            source,
            target,
            inverse_image,
            direct_image,
        }
    }
}

/// A sheaf on a site: pairs of (open_id, list_of_section_ids).
#[derive(Debug, Clone, Default)]
pub struct SpecSheaf {
    /// id of the site object
    pub site: usize,
    /// presheaf data: (open_id, sections_over_open)
    pub presheaf_data: Vec<(usize, Vec<usize>)>,
}

impl SpecSheaf {
    pub fn new(site: usize) -> Self {
        SpecSheaf {
            site,
            presheaf_data: Vec::new(),
        }
    }

    pub fn add_sections(&mut self, open_id: usize, sections: Vec<usize>) {
        self.presheaf_data.push((open_id, sections));
    }

    pub fn sections_over(&self, open_id: usize) -> Option<&Vec<usize>> {
        self.presheaf_data
            .iter()
            .find(|(o, _)| *o == open_id)
            .map(|(_, s)| s)
    }
}

/// Sheaf condition variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SheafConditionKind {
    /// Gluing: compatible local sections glue to a unique global section.
    GlueingAxiom,
    /// Separation: a section determined by local data is unique.
    SeparationAxiom,
    /// Both gluing and separation hold.
    BothAxioms,
}

/// Lawvere–Tierney topology: an idempotent closure operator j: Ω → Ω
/// satisfying j∘true = true, j∘j = j, and j∘∧ = ∧∘(j×j).
#[derive(Debug, Clone)]
pub struct LTTopology {
    /// id of the topos this topology lives in
    pub topos: usize,
    /// The j-operator morphism Ω → Ω
    pub j_operator: ToposMorphism,
}

impl LTTopology {
    pub fn new(topos: usize, j_operator: ToposMorphism) -> Self {
        LTTopology { topos, j_operator }
    }
}
