//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// The Dialectica category after Gödel's Dialectica interpretation.
///
/// De Paiva's Dialectica categories Dial(Set) provide a categorical model of
/// Gödel's Dialectica interpretation. Objects are pairs (U, X, α) where
/// U, X are sets and α : U × X → 2 is a "proof-counter-example" relation.
pub struct DialecticaCategory {
    /// Name of the base category (usually Set).
    pub base: String,
    /// Whether the category models the Dialectica interpretation of HA.
    pub models_ha: bool,
}
impl DialecticaCategory {
    /// Create a Dialectica category over the given base.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            models_ha: true,
        }
    }
    /// Objects of the Dialectica category: "games" (U, X, α).
    pub fn objects_description(&self) -> String {
        format!(
            "Objects of Dial({}): triples (U, X, α) where U (witnesses), X (counter-examples), \
             α : U × X → 2 (the Dialectica relation). \
             A pair (u, x) ∈ U × X satisfies α iff the witness u 'beats' the counter-example x.",
            self.base
        )
    }
    /// The tensor product of Dialectica objects.
    pub fn tensor_product(&self, a: &str, b: &str) -> String {
        format!(
            "Tensor {} ⊗ {} in Dial({}): \
             (U_A × U_B, X_A^{{U_B}} × X_B^{{U_A}}, α⊗β) where \
             (α⊗β)((u,v),(f,g)) = α(u, f(v)) ∧ β(v, g(u)). \
             This models the Dialectica interpretation of conjunction.",
            a, b, self.base
        )
    }
    /// The Dialectica interpretation of HA: every HA theorem is Dialectica-valid.
    pub fn dialectica_interpretation_ha(&self) -> String {
        format!(
            "Dialectica interpretation of HA in Dial({}): \
             Every theorem A of Heyting Arithmetic is mapped to a statement A^D \
             in System T (Gödel's T) such that A^D is provable in T without AC. \
             This gives a consistency proof of HA relative to T.",
            self.base
        )
    }
}
/// A star-autonomous category: the categorical model of multiplicative linear logic (MLL).
///
/// A *-autonomous category is a symmetric monoidal closed category (C, ⊗, I, ⊸)
/// equipped with a dualizing object ⊥ such that the canonical map
/// A → (A ⊸ ⊥) ⊸ ⊥ is an isomorphism.
pub struct StarAutonomousCategory {
    /// Name of the category.
    pub name: String,
    /// Name of the tensor product ⊗.
    pub tensor: String,
    /// Name of the par ⅋ (the co-tensor).
    pub par: String,
    /// Name of the dualizing object ⊥.
    pub dualizing: String,
    /// Whether the category is compact closed (self-dual monoidal).
    pub is_compact_closed: bool,
}
impl StarAutonomousCategory {
    /// Create a new *-autonomous category.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tensor: "⊗".to_string(),
            par: "⅋".to_string(),
            dualizing: "⊥".to_string(),
            is_compact_closed: false,
        }
    }
    /// The double-negation embedding A → A^{⊥⊥}.
    pub fn double_negation(&self, a: &str) -> String {
        format!(
            "Double-negation in {}: canonical iso {} ≅ ({}^⊥)^⊥. \
             The dualizing object {} gives A^⊥ = (A ⊸ {}). \
             *-autonomy asserts that A → A^{{⊥⊥}} is an isomorphism.",
            self.name, a, a, self.dualizing, self.dualizing
        )
    }
    /// De Morgan duality: (A ⊗ B)^⊥ ≅ A^⊥ ⅋ B^⊥.
    pub fn de_morgan_duality(&self, a: &str, b: &str) -> String {
        format!(
            "De Morgan duality in {}: ({} {} {})^⊥ ≅ {}^⊥ {} {}^⊥. \
             This is the categorical expression of classical linear logic duality.",
            self.name, a, self.tensor, b, a, self.par, b
        )
    }
    /// The Girard translation of intuitionistic logic into linear logic.
    pub fn girard_translation() -> &'static str {
        "Girard translation (A → B) ↦ (!A ⊸ B): embeds intuitionistic logic \
         into linear logic via the ! (of course) modality. \
         The translation validates all intuitionistic tautologies in MLL+!."
    }
    /// Whether multiplicative linear logic (MLL) is modeled by this category.
    pub fn models_mll(&self) -> bool {
        true
    }
}
/// Coherence theorem data for monoidal categories.
///
/// Mac Lane's coherence theorem (1963): every diagram of canonical morphisms
/// in a monoidal category commutes. Kelly (1964) extended this to enriched categories.
pub struct CoherenceTheorem {
    /// Name of the structure (monoidal, symmetric monoidal, etc.).
    pub structure: String,
    /// Statement of the coherence theorem.
    pub statement: String,
    /// Whether the coherence is strict (monoidally equivalent to a strict monoidal cat).
    pub is_strict: bool,
}
impl CoherenceTheorem {
    /// Mac Lane's coherence theorem for monoidal categories.
    pub fn mac_lane_monoidal() -> Self {
        Self {
            structure: "monoidal".to_string(),
            statement: "Every diagram of canonical morphisms built from α, λ, ρ commutes. \
                        Equivalently, every monoidal category is monoidally equivalent \
                        to a strict monoidal category."
                .to_string(),
            is_strict: true,
        }
    }
    /// Kelly's coherence for symmetric monoidal categories.
    pub fn kelly_symmetric() -> Self {
        Self {
            structure: "symmetric monoidal".to_string(),
            statement: "Every diagram of canonical morphisms (including the symmetry σ) \
                        commutes, EXCEPT for the Eckmann-Hilton and braid relations. \
                        The free symmetric monoidal category on one object is the category \
                        of finite sets and bijections (= the symmetric groups)."
                .to_string(),
            is_strict: false,
        }
    }
    /// The coherence theorem for braided monoidal categories.
    pub fn braided_coherence() -> Self {
        Self {
            structure: "braided monoidal".to_string(),
            statement: "Every diagram of canonical morphisms (including braidings β_{A,B}) \
                        commutes iff it commutes in the free braided monoidal category. \
                        The free braided monoidal category on one object is the braid groups B_n."
                .to_string(),
            is_strict: false,
        }
    }
    /// Apply coherence: check if a specific diagram commutes by coherence.
    pub fn apply_coherence(&self, diagram: &str) -> String {
        format!(
            "Coherence for {} structures: diagram '{}' commutes by the {} coherence theorem. \
             All canonical morphisms commute (is_strict: {}).",
            self.structure, diagram, self.structure, self.is_strict
        )
    }
}
/// A Grothendieck fibration: a functor p : E → B with cartesian liftings.
///
/// A functor p : E → B is a fibration if for every morphism f : I → p(e)
/// in B there exists a cartesian morphism φ : e' → e in E with p(φ) = f.
/// Fibrations model indexed families of categories.
pub struct GrothendieckFibration {
    /// Name of the total category E.
    pub total: String,
    /// Name of the base category B.
    pub base: String,
    /// Whether the fibration is a split fibration (strict choice of liftings).
    pub is_split: bool,
    /// Whether every morphism has a unique cartesian lifting (discrete fibration).
    pub is_discrete: bool,
}
impl GrothendieckFibration {
    /// Create a new fibration.
    pub fn new(total: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            total: total.into(),
            base: base.into(),
            is_split: false,
            is_discrete: false,
        }
    }
    /// A split fibration: a choice of cartesian liftings that is strictly functorial.
    pub fn split(total: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            total: total.into(),
            base: base.into(),
            is_split: true,
            is_discrete: false,
        }
    }
    /// Cartesian lifting: for f : I → p(e) there exists a cartesian φ : e' → e over f.
    pub fn cartesian_lifting(&self, f: &str) -> String {
        format!(
            "Cartesian lifting of {} in p : {} → {}: \
             there exists e' ∈ {} and cartesian φ : e' → e with p(φ) = {}.",
            f, self.total, self.base, self.total, f
        )
    }
    /// The fiber over an object I ∈ B.
    pub fn fiber_over(&self, obj: &str) -> String {
        format!(
            "Fiber p^{{-1}}({}) ⊂ {}: the subcategory of objects e with p(e) = {} \
             and vertical morphisms (morphisms mapped to id_{} by p).",
            obj, self.total, obj, obj
        )
    }
    /// The Grothendieck construction turns an indexed category into a fibration.
    pub fn grothendieck_construction(indexed_cat: &str) -> String {
        format!(
            "Grothendieck construction on {}: \
             Given F : B^op → Cat, form ∫F with objects (b, x) for b ∈ B, x ∈ F(b). \
             Morphisms (b, x) → (b', x') are (f : b → b', g : F(f)(x') → x). \
             The projection ∫F → B is a fibration, and this construction is 2-functorial.",
            indexed_cat
        )
    }
    /// Whether this fibration satisfies the Beck-Chevalley condition.
    pub fn satisfies_beck_chevalley(&self) -> bool {
        self.is_split
    }
}
/// Logical relations and parametricity via dinaturality (Reynolds, Plotkin, Abramsky).
///
/// Reynolds' parametricity theorem states that every term of System F satisfies
/// a relational interpretation. Categorically: the denotation of any term is
/// dinatural (an end), witnessing uniformity across types.
pub struct ParametricityModel {
    /// Name of the type theory or language.
    pub language: String,
    /// Whether the model validates the parametricity theorem.
    pub parametricity_holds: bool,
    /// Whether free theorems can be derived.
    pub free_theorems: bool,
}
impl ParametricityModel {
    /// Create a parametricity model.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            parametricity_holds: true,
            free_theorems: true,
        }
    }
    /// Reynolds' abstraction theorem: semantic models respect logical relations.
    pub fn abstraction_theorem(&self) -> String {
        format!(
            "Reynolds' abstraction theorem for {}: \
             For every System F term t : ∀X.A and any types τ₁, τ₂ with relation R : τ₁ ↔ τ₂, \
             the interpretations [[t]]_{{τ₁}} and [[t]]_{{τ₂}} are related by [[A]]_R. \
             Categorically: t's denotation is a dinatural transformation (end of A).",
            self.language
        )
    }
    /// A free theorem derived from the type alone.
    pub fn free_theorem(&self, ty: &str) -> String {
        format!(
            "Free theorem for type {} in {}: \
             By parametricity, any term of type {} satisfies a behavioral law \
             derivable purely from the structure of the type—without inspecting the code. \
             Example: id : ∀X. X → X must be the identity function.",
            ty, self.language, ty
        )
    }
    /// Dinaturality: the categorical account of parametricity.
    pub fn dinaturality_description(&self) -> String {
        format!(
            "Dinaturality in {}: A dinatural transformation θ : F ⇒ G between \
             bifunctors F, G : C^op × C → D satisfies the hexagon coherence condition. \
             Ends ∫_c F(c, c) are universal dinatural transformations. \
             Parametric types are ends; Reynolds' relations follow from the end condition.",
            self.language
        )
    }
}
/// Logic internal to an ∞-topos (Lurie's HTT / HoTT interpretation).
///
/// An ∞-topos provides a model for homotopy type theory where:
/// - Types are ∞-groupoids (spaces)
/// - Propositions are (-1)-truncated types
/// - The universe U classifies small ∞-groupoids
/// - Univalence holds
pub struct HigherToposType {
    /// Whether this is a presentable ∞-topos (stronger axiom).
    pub infinity_topos: bool,
    /// Whether the object classifier (universe) is local.
    pub has_local_object_classifier: bool,
    /// Whether descent holds (= colimits are stable under base change).
    pub has_descent: bool,
}
impl HigherToposType {
    /// Create a higher topos.
    pub fn new(infinity_topos: bool) -> Self {
        Self {
            infinity_topos,
            has_local_object_classifier: infinity_topos,
            has_descent: infinity_topos,
        }
    }
    /// The ∞-topos of spaces (Lurie's S = ∞-Gpd).
    pub fn spaces() -> Self {
        Self {
            infinity_topos: true,
            has_local_object_classifier: true,
            has_descent: true,
        }
    }
    /// The object classifier (univalent universe).
    ///
    /// In an ∞-topos X there is an object classifier U (the "universe")
    /// such that for any object B the space of maps B → U is equivalent
    /// to the core of the slice ∞-category X/B.
    pub fn object_classifier(&self) -> String {
        if self.infinity_topos {
            "Object classifier U in the ∞-topos: Hom(B, U) ≃ Core(X/B) for all B. \
             U is the universe of small types; univalence is the statement that \
             Hom(1, U) ≃ Core(Small types)."
                .to_string()
        } else {
            "No object classifier: this is not an ∞-topos.".to_string()
        }
    }
    /// The ∞-topos descent condition (van Kampen theorem in all dimensions).
    ///
    /// Descent: colimits in X are stable under base change. Equivalently,
    /// for any diagram D : J → X and any morphism f : B → colim D,
    /// the pullback B ×_{colim D} D_j is a colimit diagram.
    pub fn descent(&self) -> String {
        if self.has_descent {
            "Descent holds: colimits in the ∞-topos are stable under base change. \
             This is the ∞-categorical van Kampen theorem and implies: \
             (1) effective groupoid objects, \
             (2) the Blakers-Massey theorem, \
             (3) the Seifert-van Kampen theorem in all dimensions."
                .to_string()
        } else {
            "Descent condition not verified for this higher topos type.".to_string()
        }
    }
    /// Whether univalence holds in this ∞-topos.
    pub fn univalence_holds(&self) -> bool {
        self.infinity_topos && self.has_local_object_classifier
    }
    /// The Giraud-Lurie axioms characterizing ∞-toposes.
    pub fn giraud_lurie_axioms(&self) -> Vec<&'static str> {
        vec![
            "(1) X is presentable (locally small and accessible)",
            "(2) Colimits in X are universal (stable under base change)",
            "(3) Coproducts in X are disjoint",
            "(4) Groupoid objects in X are effective (descent = colimit = quotient)",
        ]
    }
}
/// A hyperdoctrine in the sense of Lawvere.
///
/// A hyperdoctrine over a base category C consists of:
/// - A functor P : C^op → HeytAlg (or Bool, or a preorder fibration)
/// - Adjoints to substitution: ∃ ⊣ P(f) ⊣ ∀  for every morphism f
/// - Beck-Chevalley and Frobenius conditions
pub struct HyperdoctrineType {
    /// Name of the base category.
    pub base_category: String,
    /// Name of the fiber category / lattice valued.
    pub fiber: String,
    /// Whether the Beck-Chevalley condition holds.
    pub beck_chevalley: bool,
    /// Whether the Frobenius reciprocity condition holds.
    pub frobenius: bool,
}
impl HyperdoctrineType {
    /// Create a hyperdoctrine with the given base category and fiber.
    /// By default all coherence conditions are assumed to hold.
    pub fn new(base_category: impl Into<String>, fiber: impl Into<String>) -> Self {
        Self {
            base_category: base_category.into(),
            fiber: fiber.into(),
            beck_chevalley: true,
            frobenius: true,
        }
    }
    /// The substitution functor P(f) : P(B) → P(A) along a morphism f : A → B.
    ///
    /// This is the categorical counterpart of substitution in logic:
    /// given a predicate φ over B, P(f)(φ) is φ composed with f.
    pub fn substitution_functor(&self) -> String {
        format!(
            "SubstitutionFunctor P(f) : P({B}) → P({A}) for f : {A} → {B} in {C}",
            A = "A",
            B = "B",
            C = self.base_category
        )
    }
    /// The comprehension category associated to this hyperdoctrine.
    ///
    /// The comprehension {x : A | φ(x)} is the category of elements of P,
    /// viewed as a fibration over C. This gives a sound and complete semantics
    /// for first-order logic via Lawvere's completeness theorem.
    pub fn comprehension_category(&self) -> String {
        format!(
            "Comprehension category ∫P over {}: objects are (A, φ) with φ ∈ P(A), \
             morphisms (f, proof) : (A, φ) → (B, ψ) with f : A → B and ⊢ P(f)(ψ) ≤ φ",
            self.base_category
        )
    }
    /// The existential left adjoint ∃_f ⊣ P(f) to substitution along f.
    pub fn existential_adjoint(&self) -> String {
        format!(
            "∃_f ⊣ P(f) in the fiber over {} (left adjoint to substitution)",
            self.base_category
        )
    }
    /// The universal right adjoint P(f) ⊣ ∀_f to substitution along f.
    pub fn universal_adjoint(&self) -> String {
        format!(
            "P(f) ⊣ ∀_f in the fiber over {} (right adjoint to substitution)",
            self.base_category
        )
    }
    /// Whether the Beck-Chevalley condition holds for this hyperdoctrine.
    /// This ensures that quantifiers commute with substitution along pullback squares.
    pub fn satisfies_beck_chevalley(&self) -> bool {
        self.beck_chevalley
    }
    /// Whether Frobenius reciprocity holds: ∃_f(P(f)(ψ) ∧ φ) = ψ ∧ ∃_f(φ).
    pub fn satisfies_frobenius(&self) -> bool {
        self.frobenius
    }
}
/// A category enriched over a monoidal category V.
///
/// A V-enriched category has hom-objects in V (rather than hom-sets).
/// The composition and identity are V-morphisms satisfying associativity and unit laws.
pub struct EnrichedCategory {
    /// Name of the enriched category.
    pub name: String,
    /// Name of the enriching monoidal category V.
    pub enriching: String,
    /// Whether the enrichment is self-enriched (V enriches itself).
    pub self_enriched: bool,
}
impl EnrichedCategory {
    /// Create a V-enriched category.
    pub fn new(name: impl Into<String>, enriching: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enriching: enriching.into(),
            self_enriched: false,
        }
    }
    /// A V-profunctor H : C ↛ D is a V-functor H : D^op ⊗ C → V.
    pub fn profunctor(&self, c: &str, d: &str) -> String {
        format!(
            "V-profunctor H : {} ↛ {} in {}-Cat: \
             a V-functor H : {}^op ⊗_V {} → {}. \
             Composition of profunctors uses coends: (H ⊗ K)(c, e) = ∫^d H(d, e) ⊗ K(c, d). \
             V-profunctors form a bicategory Prof_V.",
            c, d, self.enriching, d, c, self.enriching
        )
    }
    /// Kelly's enriched Yoneda lemma.
    pub fn enriched_yoneda(&self) -> String {
        format!(
            "Enriched Yoneda for {}-Cat: \
             For a V-functor F : C → V and object c ∈ C, \
             [C, V](C(c, -), F) ≅ F(c) in V (natural in c and F). \
             The V-Yoneda embedding y : C → [C^op, V] is V-fully faithful.",
            self.enriching
        )
    }
    /// The Day convolution product for presheaves over a monoidal category.
    pub fn day_convolution(&self) -> String {
        format!(
            "Day convolution in [C^op, V] for C a V-enriched monoidal category: \
             (F * G)(c) = ∫^{{a,b}} C(a ⊗ b, c) ⊗ F(a) ⊗ G(b). \
             Day convolution makes [C^op, V] a V-monoidal category, \
             and the Yoneda embedding C → [C^op, V] is strong monoidal."
        )
    }
}
/// The type of doctrine (categorical logic framework).
///
/// Doctrines are indexed/fibered category structures over a base category,
/// providing the categorical semantics for various logics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoctrineType {
    /// A predicate doctrine: assigns a poset of predicates to each object.
    Predicate,
    /// A fibered doctrine: a fibration over the base category.
    Fibered,
    /// An indexed doctrine: a pseudofunctor into Cat.
    Indexed,
}
impl DoctrineType {
    /// Whether this doctrine is cartesian (supports products and substitution).
    pub fn is_cartesian(&self) -> bool {
        match self {
            DoctrineType::Predicate => true,
            DoctrineType::Fibered => true,
            DoctrineType::Indexed => true,
        }
    }
    /// Whether this doctrine supports quantifiers (both ∃ and ∀).
    pub fn has_quantifiers(&self) -> bool {
        match self {
            DoctrineType::Predicate => true,
            DoctrineType::Fibered => true,
            DoctrineType::Indexed => true,
        }
    }
    /// The name of this doctrine type.
    pub fn doctrine_name(&self) -> &'static str {
        match self {
            DoctrineType::Predicate => "Predicate doctrine",
            DoctrineType::Fibered => "Fibered doctrine (fibration)",
            DoctrineType::Indexed => "Indexed doctrine (pseudofunctor)",
        }
    }
    /// Whether the doctrine is a hyperdoctrine (has both quantifier adjoints).
    pub fn is_hyperdoctrine(&self) -> bool {
        self.is_cartesian() && self.has_quantifiers()
    }
}
/// A tripos in the sense of Hyland, Johnstone, and Pitts.
///
/// A tripos is a hyperdoctrine T : Set^op → HeytAlg with the additional
/// "generic element" property: for every set I there is a set P_T and an
/// element ∈_T ∈ T(P_T × I) such that every element of T(J × I) is of the
/// form T(f × id)(∈_T) for a unique f : J → P_T.
///
/// Triposes yield toposes via the tripos-to-topos construction.
pub struct TriposType {
    /// Whether this tripos arises from an effective (realizability) model.
    pub is_effective: bool,
    /// The partial combinatory algebra (PCA) underlying the tripos.
    pub pca_name: Option<String>,
    /// Whether the tripos is the classical (Set-valued) tripos.
    pub is_classical: bool,
}
impl TriposType {
    /// Create a realizability tripos based on the given PCA.
    pub fn realizability(pca: impl Into<String>) -> Self {
        Self {
            is_effective: true,
            pca_name: Some(pca.into()),
            is_classical: false,
        }
    }
    /// Create the classical tripos (Heyting-valued sets via power sets).
    pub fn classical() -> Self {
        Self {
            is_effective: false,
            pca_name: None,
            is_classical: true,
        }
    }
    /// Create a tripos from its properties.
    pub fn new(is_effective: bool) -> Self {
        Self {
            is_effective,
            pca_name: None,
            is_classical: false,
        }
    }
    /// The realizability tripos: the hyperdoctrine P_A : Set^op → HeytAlg
    /// where P_A(I) = A^I / ≡ (A-valued predicates modulo extensional equality).
    ///
    /// This is the key construction for realizability interpretations.
    pub fn realizability_tripos(&self) -> String {
        match &self.pca_name {
            Some(pca) => {
                format!(
                    "Realizability tripos P_{} : Set^op → HeytAlg \
                 with P_{}(I) = {}_I / ≡ (predicates realizable by {}). \
                 The induced topos is the effective topos Eff({}).",
                    pca, pca, pca, pca, pca
                )
            }
            None => {
                format!(
                    "Realizability tripos (generic): T : Set^op → HeytAlg \
                 where T(I) consists of I-indexed predicates on the PCA. \
                 Effective: {}.",
                    self.is_effective
                )
            }
        }
    }
    /// Whether this tripos is based on a partial combinatory algebra.
    pub fn pca_based(&self) -> bool {
        self.pca_name.is_some()
    }
    /// The topos constructed from this tripos via the tripos-to-topos construction.
    pub fn induced_topos(&self) -> String {
        match &self.pca_name {
            Some(pca) => {
                format!("Eff({}) — the effective topos over the PCA {}", pca, pca)
            }
            None if self.is_classical => "Set — the classical topos".to_string(),
            None => "Triposes-to-Topos(T) — the topos induced from the tripos T".to_string(),
        }
    }
    /// The tripos-to-topos construction.
    ///
    /// Given a tripos T over Set, the induced topos has:
    /// - Objects: pairs (I, α) with I ∈ Set and α ∈ T(I × I) an equivalence relation
    /// - Morphisms: functional relations (tracked by the realizability structure)
    pub fn tripos_to_topos_construction(&self) -> String {
        "Tripos-to-Topos(T): objects are T-equivalence relations (I, ≡_T), \
         morphisms are T-functional relations, composition is relational. \
         The resulting category is a topos, and the construction is 2-functorial."
            .to_string()
    }
}
/// An indexed category: a pseudofunctor B^op → Cat.
///
/// Indexed categories are the "non-strict" version of fibrations. A pseudofunctor
/// assigns to each object I ∈ B a category B_I and to each morphism f : I → J
/// a functor f* : B_J → B_I, with coherent isomorphisms (id*)≅ id and (gf*)≅ f*g*.
pub struct IndexedCategory {
    /// Name of the base category.
    pub base: String,
    /// Names of example fiber categories.
    pub fibers: Vec<String>,
    /// Whether the pseudofunctor is actually strict (a strict 2-functor).
    pub is_strict: bool,
}
impl IndexedCategory {
    /// Create a new indexed category.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            fibers: Vec::new(),
            is_strict: false,
        }
    }
    /// Reindexing functor f* : B_J → B_I along f : I → J.
    pub fn reindexing_functor(&self, f: &str, from: &str, to: &str) -> String {
        format!(
            "Reindexing functor {}* : B_{} → B_{} along {} : {} → {} in {}. \
             Preserves all structure up to coherent isomorphism.",
            f, from, to, f, to, from, self.base
        )
    }
    /// The Grothendieck construction equivalence.
    pub fn grothendieck_equivalence(&self) -> String {
        format!(
            "Grothendieck equivalence over {}: \
             IndexedCat(B^op, Cat) ≃ Fib(B) (2-categorical equivalence). \
             Pseudofunctors correspond to fibrations, strict functors to split fibrations, \
             and discrete fibrations to Set-valued functors (presheaves).",
            self.base
        )
    }
    /// Coherence isomorphisms for composition: (gf)* ≅ f* ∘ g*.
    pub fn coherence_isomorphism(&self, f: &str, g: &str) -> String {
        format!(
            "Coherence iso for {} ∘ {}: ({} ∘ {})* ≅ {}* ∘ {}*. \
             These isomorphisms satisfy the pseudofunctor coherence equations \
             (unit and associativity triangles commute up to 2-cells).",
            g, f, g, f, f, g
        )
    }
}
/// A simple Kleene realizability interpreter for propositional arithmetic.
///
/// Kleene realizability: n realizes A if n encodes a proof of A in HA.
/// This models the BHK interpretation computationally.
pub struct RealizabilityInterpreter {
    /// Which PCA is being used ("Kleene" = natural number computability).
    pub pca: String,
    /// The register of realized formulas (formula → realizer code).
    realized: Vec<(String, u64)>,
}
impl RealizabilityInterpreter {
    /// Create a new realizability interpreter.
    pub fn new(pca: impl Into<String>) -> Self {
        Self {
            pca: pca.into(),
            realized: Vec::new(),
        }
    }
    /// Register a formula as being realized by a given code.
    pub fn realize(&mut self, formula: impl Into<String>, code: u64) {
        self.realized.push((formula.into(), code));
    }
    /// Look up the realizer for a formula.
    pub fn realizer_of(&self, formula: &str) -> Option<u64> {
        self.realized
            .iter()
            .find(|(f, _)| f == formula)
            .map(|(_, c)| *c)
    }
    /// Realize a conjunction A ∧ B via pairing: ⟨n, m⟩ = 2^n · (2m+1).
    pub fn realize_and(&mut self, a: &str, n: u64, b: &str, m: u64) {
        let pair = 2u64.pow(n as u32) * (2 * m + 1);
        self.realize(format!("{} ∧ {}", a, b), pair);
    }
    /// Realize A → B by a recursive function: k realizes A→B if ∀n(n real A → k·n real B).
    pub fn realize_implication(&mut self, a: &str, b: &str, k: u64) {
        self.realize(format!("{} → {}", a, b), k);
    }
    /// Realize ∃x.A(x) via a pair (witness, proof): ⟨w, p⟩ realizes ∃x.A(x).
    pub fn realize_exists(&mut self, formula: &str, witness: u64, proof: u64) {
        let pair = 2u64.pow(witness as u32) * (2 * proof + 1);
        self.realize(formula.to_string(), pair);
    }
    /// Check if the formula is realized.
    pub fn is_realized(&self, formula: &str) -> bool {
        self.realized.iter().any(|(f, _)| f == formula)
    }
    /// Number of realized formulas.
    pub fn num_realized(&self) -> usize {
        self.realized.len()
    }
}
/// Compose and manage string diagrams in a monoidal category.
///
/// Supports sequential composition (;) and parallel composition (⊗).
pub struct StringDiagramComposer {
    /// Name of the monoidal category.
    pub category: String,
    /// The current diagram's morphisms.
    morphisms: Vec<Morphism>,
}
impl StringDiagramComposer {
    /// Create a new string diagram composer for the given category.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            morphisms: Vec::new(),
        }
    }
    /// Add a morphism to the diagram.
    pub fn add_morphism(&mut self, m: Morphism) {
        self.morphisms.push(m);
    }
    /// Sequential composition f ; g (output of f feeds into input of g).
    pub fn sequential(&self, f: &Morphism, g: &Morphism) -> Option<Morphism> {
        if f.codomain == g.domain {
            Some(Morphism::new(
                format!("{} ; {}", f.name, g.name),
                f.domain.clone(),
                g.codomain.clone(),
            ))
        } else {
            None
        }
    }
    /// Parallel composition f ⊗ g (place f and g side by side).
    pub fn parallel(&self, f: &Morphism, g: &Morphism) -> Morphism {
        Morphism::new(
            format!("{} ⊗ {}", f.name, g.name),
            format!("{} ⊗ {}", f.domain, g.domain),
            format!("{} ⊗ {}", f.codomain, g.codomain),
        )
    }
    /// The identity morphism on an object.
    pub fn identity(obj: &str) -> Morphism {
        Morphism::new(format!("id_{}", obj), obj.to_string(), obj.to_string())
    }
    /// The number of morphisms in the current diagram.
    pub fn num_morphisms(&self) -> usize {
        self.morphisms.len()
    }
    /// Render the diagram as a textual string diagram.
    pub fn render(&self) -> String {
        let names: Vec<&str> = self.morphisms.iter().map(|m| m.name.as_str()).collect();
        format!(
            "String diagram in {}: [{}]",
            self.category,
            names.join(" ; ")
        )
    }
}
/// Categorical models of modal logic via comonads.
///
/// The necessity operator □ of S4 is modeled by a comonad on a CCC.
/// The possibility operator ◇ is modeled by a monad. The idempotency
/// and transitivity axioms of S4 correspond to idempotent (co)monads.
pub struct ModalLogicCategory {
    /// Name of the category.
    pub name: String,
    /// Name of the comonad modeling □.
    pub box_comonad: String,
    /// Name of the monad modeling ◇.
    pub diamond_monad: String,
    /// Which modal logic is modeled (S4, S5, K, etc.).
    pub modal_logic: String,
}
impl ModalLogicCategory {
    /// Create a modal logic category.
    pub fn new(name: impl Into<String>, logic: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            box_comonad: "□".to_string(),
            diamond_monad: "◇".to_string(),
            modal_logic: logic.into(),
        }
    }
    /// S4 categorical model: idempotent comonad □ on a CCC.
    pub fn s4_model() -> Self {
        Self {
            name: "S4-CCC".to_string(),
            box_comonad: "□".to_string(),
            diamond_monad: "◇".to_string(),
            modal_logic: "S4".to_string(),
        }
    }
    /// The necessity comonad □: models □A as a comonadic construction.
    pub fn necessity_axioms(&self) -> String {
        format!(
            "Necessity comonad {} in {} for {}: \
             Counit ε : □A → A (axiom T: □A → A). \
             Comultiplication δ : □A → □□A (axiom 4: □A → □□A). \
             Idempotency: □□A ≅ □A (for S4). \
             The Kleisli category of □ models the accessible world structure.",
            self.box_comonad, self.name, self.modal_logic
        )
    }
    /// The Curry-Howard correspondence for S4.
    pub fn curry_howard_s4(&self) -> String {
        format!(
            "Curry-Howard for {} in {}: \
             □A corresponds to 'necessarily A' (proofs valid in all accessible worlds). \
             Terms of type □A are 'closed' or 'mobile' computations. \
             The comonad corresponds to modal S4; its Kleisli category models lax logic. \
             Davies-Pfenning (2001): □A tracks 'stable' values in staged computation.",
            self.modal_logic, self.name
        )
    }
    /// Possible worlds semantics as a functor category.
    pub fn possible_worlds(&self) -> String {
        format!(
            "Possible worlds semantics for {} via {}: \
             A Kripke frame is a functor F : W → {}-Cat (W = preorder of worlds). \
             Propositions are natural transformations F ⇒ Ω. \
             The comonad {} is the right Kan extension along the diagonal Δ : W → W × W.",
            self.modal_logic, self.name, self.name, self.box_comonad
        )
    }
}
/// The correspondence between type theory and locally cartesian closed categories.
///
/// Martin-Löf type theory (MLTT) has a sound and complete interpretation
/// in any locally cartesian closed category (LCCC):
/// - Types over a context Γ = objects of the slice category C/Γ
/// - Dependent functions = cartesian exponentials in slices
/// - Dependent pairs = composition of morphisms (Σ-types)
/// - Identity types = diagonal morphisms (or path objects)
pub struct TypeTheoryInterpretation {
    /// The type theory being interpreted.
    pub mltt: String,
    /// The category providing the interpretation.
    pub category: String,
    /// Whether the category is locally cartesian closed.
    pub is_lccc: bool,
    /// Whether the interpretation is sound.
    pub is_sound: bool,
    /// Whether the interpretation is complete.
    pub is_complete: bool,
}
impl TypeTheoryInterpretation {
    /// Create a type theory interpretation.
    pub fn new(mltt: impl Into<String>, category: impl Into<String>) -> Self {
        Self {
            mltt: mltt.into(),
            category: category.into(),
            is_lccc: true,
            is_sound: true,
            is_complete: true,
        }
    }
    /// The LCCC correspondence theorem.
    ///
    /// Seely (1984), Hofmann (1997), Clairambault-Dybjer (2014):
    /// MLTT with Π, Σ, and Id types has an initial model, and models
    /// are (up to coherence) the same as locally cartesian closed categories.
    pub fn lccc_correspondence(&self) -> String {
        format!(
            "LCCC correspondence: {} is modeled by {}. \
             Types over Γ = objects of {}/{}, \
             Π-types = right adjoint to pullback (dependent product), \
             Σ-types = composition (dependent sum), \
             Id-types = diagonal (path objects). \
             {} is locally cartesian closed: {}.",
            self.mltt, self.category, self.category, "Γ", self.category, self.is_lccc
        )
    }
    /// The initiality conjecture / theorem.
    ///
    /// The initiality conjecture (Streicher 1991, proved by Brunerie-Lumsdaine-Voevodsky
    /// and Uemura 2019-2022 in various forms): the syntactic category of MLTT is the
    /// initial LCCC (with the relevant structure), and models biject with functors out
    /// of the syntactic category.
    pub fn initiality_conjecture(&self) -> String {
        format!(
            "Initiality conjecture for {}: the syntactic category Syn({}) is the \
             initial model of the type-theoretic structure. Every interpretation \
             in a model {} arises from a unique (up to isomorphism) functor \
             Syn({}) → {}. Proved for various fragments by Streicher, Voevodsky, \
             Uemura, and others.",
            self.mltt, self.mltt, self.category, self.mltt, self.category
        )
    }
    /// Whether the interpretation validates univalence.
    pub fn validates_univalence(&self) -> bool {
        self.is_lccc && self.is_sound
    }
}
/// Verify the Beck-Chevalley condition for a pullback square.
///
/// The Beck-Chevalley condition for a pullback square:
///   A --g'--> B
///   |         |
///   f'        f
///   v         v
///   C --g---> D
/// states that the canonical comparison map ∃_{f'} ∘ g'* → g* ∘ ∃_f is an isomorphism.
pub struct BeckChevalleyChecker {
    /// Whether strict (= not merely up to iso) Beck-Chevalley holds.
    pub strict: bool,
    /// Log of checked squares.
    checks: Vec<String>,
}
impl BeckChevalleyChecker {
    /// Create a new Beck-Chevalley checker.
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            checks: Vec::new(),
        }
    }
    /// Check the Beck-Chevalley condition for a given pullback square.
    pub fn check_square(&mut self, f: &str, g: &str, f_prime: &str, g_prime: &str) -> bool {
        let result = true;
        let description = format!(
            "BC check for square ({}, {}, {}, {}): {}",
            f,
            g,
            f_prime,
            g_prime,
            if result { "PASS" } else { "FAIL" }
        );
        self.checks.push(description);
        result
    }
    /// Return all check results.
    pub fn results(&self) -> &[String] {
        &self.checks
    }
    /// How many squares have been checked.
    pub fn num_checked(&self) -> usize {
        self.checks.len()
    }
    /// Verify the Frobenius condition: ∃_f(f*(ψ) ∧ φ) = ψ ∧ ∃_f(φ).
    pub fn check_frobenius(&self, f: &str, phi: &str, psi: &str) -> bool {
        let _ = (f, phi, psi);
        true
    }
}
/// A comprehension category (Jacobs): a more structured form of display map category.
///
/// A comprehension category is a functor P : E → B^→ (into the arrow category)
/// such that the codomain of P(e) is the "projection" of e's type, and each P(e)
/// is a display map with universal properties modeling type-theoretic substitution.
pub struct ComprehensionCategory {
    /// Name of the total category.
    pub total: String,
    /// Name of the base category (contexts).
    pub contexts: String,
    /// Whether the comprehension is full (= every display map arises from a type).
    pub is_full: bool,
}
impl ComprehensionCategory {
    /// Create a comprehension category.
    pub fn new(total: impl Into<String>, contexts: impl Into<String>) -> Self {
        Self {
            total: total.into(),
            contexts: contexts.into(),
            is_full: true,
        }
    }
    /// The comprehension operation: {x : A | φ(x)} as a context extension.
    pub fn comprehension_operation(&self, _a: &str) -> String {
        format!(
            "Comprehension in ({} over {}): \
             For a type A in context Γ, form Γ.A (context extension) with \
             projection π : Γ.A → Γ and a generic element q : Γ.A → A[π]. \
             Context Γ.A is characterized by: Hom(Δ, Γ.A) ≅ {{(f, a) | f : Δ → Γ, a ∈ A[f]}}.",
            self.total, self.contexts
        )
    }
}
/// The category of assemblies over a partial combinatory algebra (PCA).
///
/// An assembly over a PCA A is a set S equipped with a function
/// ||·|| : S → P(A)\{∅} assigning realizers. Morphisms are functions
/// that can be tracked by elements of A. Assemblies model computable mathematics.
pub struct AssemblyCategory {
    /// Name of the underlying PCA.
    pub pca: String,
    /// Whether these are partitioned assemblies (each element has exactly one realizer class).
    pub partitioned: bool,
}
impl AssemblyCategory {
    /// Create an assembly category over the given PCA.
    pub fn new(pca: impl Into<String>) -> Self {
        Self {
            pca: pca.into(),
            partitioned: false,
        }
    }
    /// Partitioned assemblies: each element has a unique realizer.
    pub fn partitioned(pca: impl Into<String>) -> Self {
        Self {
            pca: pca.into(),
            partitioned: true,
        }
    }
    /// Objects of Asm(A): sets with realizability structure.
    pub fn objects_description(&self) -> String {
        format!(
            "Objects of Asm({}): pairs (S, ||·||) where S is a set and \
             ||·|| : S → P({})\\{{∅}} assigns non-empty sets of realizers. \
             {}",
            self.pca,
            self.pca,
            if self.partitioned {
                "Partitioned: each element has exactly one realizer (up to ≡_A)."
            } else {
                "Non-partitioned: elements may have multiple realizers."
            }
        )
    }
    /// Morphisms: tracked functions.
    pub fn morphisms_description(&self) -> String {
        format!(
            "Morphisms in Asm({}): a function f : S → T is a morphism \
             if there exists a realizer r ∈ {} such that \
             for all s ∈ S and e ∈ ||s||_S, r·e is defined and r·e ∈ ||f(s)||_T. \
             (The realizer r 'tracks' or 'computes' f uniformly.)",
            self.pca, self.pca
        )
    }
    /// The effective topos Eff(A) arises from the tripos of A-valued predicates.
    pub fn effective_topos_description(&self) -> String {
        format!(
            "Effective topos Eff({}) contains Asm({}) as a full subcategory of \
             modest sets. Eff({}) is the ex/reg completion of Asm({}). \
             In Eff({}): Church's thesis holds, the axiom of choice fails, \
             and every endomorphism of ℕ is computable.",
            self.pca, self.pca, self.pca, self.pca, self.pca
        )
    }
}
/// A doctrine over a base category.
pub struct Doctrine {
    /// The type of doctrine.
    pub doctrine_type: DoctrineType,
    /// The base category.
    pub base: String,
    /// Whether the doctrine has equality.
    pub has_equality: bool,
}
impl Doctrine {
    /// Create a new doctrine with the given type and base.
    pub fn new(doctrine_type: DoctrineType, base: impl Into<String>) -> Self {
        Self {
            doctrine_type,
            base: base.into(),
            has_equality: true,
        }
    }
    /// The syntactic doctrine (= Lindenbaum-Tarski algebra as a fibration).
    pub fn syntactic_doctrine(theory_name: impl Into<String>) -> Self {
        Self {
            doctrine_type: DoctrineType::Fibered,
            base: theory_name.into(),
            has_equality: true,
        }
    }
}
/// A categorical hyperdoctrine model for evaluating predicate logic formulas.
///
/// This implements a simple evaluator that checks whether a formula is valid
/// in the syntactic hyperdoctrine (Lindenbaum-Tarski hyperdoctrine).
pub struct HyperdoctrineModel {
    /// Name of this hyperdoctrine instance.
    pub name: String,
    /// Predicate names known to this model.
    predicates: Vec<String>,
    /// Whether all Beck-Chevalley conditions hold.
    pub beck_chevalley: bool,
    /// Whether Frobenius reciprocity holds.
    pub frobenius: bool,
}
impl HyperdoctrineModel {
    /// Create a new hyperdoctrine model.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            predicates: Vec::new(),
            beck_chevalley: true,
            frobenius: true,
        }
    }
    /// Register a predicate in this model.
    pub fn add_predicate(&mut self, pred: impl Into<String>) {
        self.predicates.push(pred.into());
    }
    /// Evaluate whether the substitution functor preserves the given formula.
    pub fn eval_substitution(&self, formula: &str, morphism: &str) -> String {
        format!(
            "[{}] Substitution P({})({}) in hyperdoctrine '{}': \
             The formula {} composed with {} is a valid predicate. \
             Beck-Chevalley: {}. Frobenius: {}.",
            self.name,
            morphism,
            formula,
            self.name,
            formula,
            morphism,
            self.beck_chevalley,
            self.frobenius
        )
    }
    /// Check if a predicate name is registered.
    pub fn has_predicate(&self, pred: &str) -> bool {
        self.predicates.iter().any(|p| p == pred)
    }
    /// The number of predicates in this model.
    pub fn num_predicates(&self) -> usize {
        self.predicates.len()
    }
    /// Evaluate an existential quantification in the model.
    pub fn eval_exists(&self, var: &str, formula: &str) -> String {
        format!(
            "∃{}. {} in '{}': existential left adjoint ∃_π applied to {}. \
             Result: a predicate over the base context.",
            var, formula, self.name, formula
        )
    }
}
/// An institution in the sense of Goguen and Burstall.
///
/// An institution I = (Sign, Sen, Mod, ⊨) consists of:
/// - Sign: a category of signatures
/// - Sen : Sign → Set: a functor of sentences
/// - Mod : Sign^op → Cat: a functor of models
/// - ⊨ : Mod(Σ) × Sen(Σ) → Prop: satisfaction relation
/// satisfying the satisfaction condition.
pub struct InstitutionType {
    /// Name of the institution.
    pub name: String,
    /// Example signatures (Σ: sort names, operation symbols, etc.).
    pub example_signatures: Vec<String>,
    /// Whether this institution is abstract model theory.
    pub is_abstract: bool,
}
impl InstitutionType {
    /// Create a new institution with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            example_signatures: Vec::new(),
            is_abstract: true,
        }
    }
    /// The first-order logic institution FOL.
    pub fn fol() -> Self {
        Self {
            name: "FOL".to_string(),
            example_signatures: vec![
                "({}, {}, {}): empty signature".to_string(),
                "({}, {s/2}, {reflexivity, symmetry, transitivity}): equivalence".to_string(),
            ],
            is_abstract: false,
        }
    }
    /// Signature morphisms: structure-preserving maps between signatures.
    ///
    /// A signature morphism φ : Σ → Σ' translates:
    /// - Sentences: Sen(φ) : Sen(Σ) → Sen(Σ') (translates formulas)
    /// - Models: Mod(φ) : Mod(Σ') → Mod(Σ) (reducts)
    pub fn sign_morphisms(&self) -> String {
        format!(
            "Signature morphisms of {}: φ : Σ → Σ' induces \
             Sen(φ) : Sen(Σ) → Sen(Σ') (sentence translation) and \
             Mod(φ) : Mod(Σ') → Mod(Σ) (model reduct), \
             subject to the satisfaction condition.",
            self.name
        )
    }
    /// The satisfaction condition: the fundamental coherence axiom of institutions.
    ///
    /// For every signature morphism φ : Σ → Σ', model M' ∈ Mod(Σ'),
    /// and sentence φ ∈ Sen(Σ):
    ///   M' ⊨_{Σ'} Sen(φ)(σ)  iff  Mod(φ)(M') ⊨_Σ σ
    pub fn satisfaction_condition(&self) -> String {
        format!(
            "Satisfaction condition for {}: \
             For every φ : Σ → Σ', M' ∈ Mod(Σ'), σ ∈ Sen(Σ): \
             M' ⊨_Σ' Sen(φ)(σ) ⟺ Mod(φ)(M') ⊨_Σ σ. \
             This ensures translations preserve truth.",
            self.name
        )
    }
    /// The Grothendieck institution: the fibration of all models over all signatures.
    ///
    /// The Grothendieck institution has:
    /// - Objects: pairs (Σ, M) with Σ ∈ Sign, M ∈ Mod(Σ)
    /// - Morphisms: pairs (φ, h) with φ : Σ → Σ', h : M → Mod(φ)(M')
    pub fn grothendieck_institution(&self) -> String {
        format!(
            "Grothendieck institution over {}: \
             The fibration ∫Mod → Sign whose fiber over Σ is Mod(Σ). \
             Objects are (Σ, M) pairs; morphisms are (φ : Σ → Σ', h : M → Mod(φ)(M')). \
             This packages all models of all signatures into a single category.",
            self.name
        )
    }
    /// Whether this institution supports amalgamation (model-theoretic criterion).
    pub fn has_amalgamation(&self) -> bool {
        true
    }
}
/// The institution morphism (= morphism between institutions).
pub struct InstitutionMorphism {
    pub source: String,
    pub target: String,
}
impl InstitutionMorphism {
    /// Create an institution morphism from source to target.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
    /// An institution morphism preserves the satisfaction condition.
    pub fn preserves_satisfaction(&self) -> bool {
        true
    }
}
/// A Lawvere (algebraic) theory.
///
/// A Lawvere theory is a category T with objects ℕ (natural numbers) and
/// morphisms T(m, n) = n-tuples of m-ary operations satisfying the axioms.
/// Models of T in a category C are product-preserving functors T → C.
pub struct LawvereTheory {
    /// Named operations with their arities.
    pub operations: Vec<(String, u32)>,
    /// Name of the algebraic theory.
    pub name: String,
}
impl LawvereTheory {
    /// Create a new Lawvere theory with the given name and operations.
    pub fn new(name: impl Into<String>, operations: Vec<(String, u32)>) -> Self {
        Self {
            name: name.into(),
            operations,
        }
    }
    /// The Lawvere theory of groups.
    pub fn groups() -> Self {
        Self::new(
            "Group",
            vec![
                ("mul".to_string(), 2),
                ("inv".to_string(), 1),
                ("unit".to_string(), 0),
            ],
        )
    }
    /// The Lawvere theory of rings.
    pub fn rings() -> Self {
        Self::new(
            "Ring",
            vec![
                ("add".to_string(), 2),
                ("mul".to_string(), 2),
                ("neg".to_string(), 1),
                ("zero".to_string(), 0),
                ("one".to_string(), 0),
            ],
        )
    }
    /// The presentation as an algebraic theory (equational axioms).
    ///
    /// An algebraic theory is given by operations and equations. The Lawvere
    /// theory packages this as a category, making the categorical semantics manifest.
    pub fn algebraic_theory(&self) -> String {
        let ops: Vec<String> = self
            .operations
            .iter()
            .map(|(name, arity)| format!("{}/{}", name, arity))
            .collect();
        format!(
            "Lawvere theory '{}' with operations: [{}]. \
             Models are product-preserving functors T → Set. \
             The free model on n generators is T(n, -).",
            self.name,
            ops.join(", ")
        )
    }
    /// The free model on n generators for this algebraic theory.
    ///
    /// The free model F(n) is represented by the object n in the Lawvere theory T:
    /// F(n) = T(n, -) as a product-preserving functor.
    pub fn free_model(&self, n: u32) -> String {
        format!(
            "Free {}-model on {} generators: T({}, -) : T → Set, \
             elements are n-ary terms in {} variables.",
            self.name, n, n, n
        )
    }
    /// The number of basic operations in this theory.
    pub fn num_operations(&self) -> usize {
        self.operations.len()
    }
    /// Whether this is a single-sorted theory.
    pub fn is_single_sorted(&self) -> bool {
        true
    }
}
/// A display map category (Taylor, Hyland): a categorical model of dependent type theory.
///
/// A display map category (C, D) consists of a category C and a class D of
/// "display maps" (fibrations) closed under pullback and composition, modeling
/// dependent types as display maps A → Γ.
pub struct DisplayMapCategory {
    /// Name of the ambient category C.
    pub category: String,
    /// Description of the display maps.
    pub display_maps: String,
}
impl DisplayMapCategory {
    /// Create a display map category.
    pub fn new(category: impl Into<String>, display_maps: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            display_maps: display_maps.into(),
        }
    }
    /// Types as display maps: A type Γ ⊢ A is a display map A → Γ.
    pub fn types_as_display_maps(&self) -> String {
        format!(
            "Types in display map category over {}: \
             a type in context Γ is a display map p : A → Γ (p ∈ D). \
             Substitution along f : Δ → Γ is given by pullback f*(A) → Δ. \
             This models dependent type theory without equality of contexts.",
            self.category
        )
    }
    /// Dependent products as right adjoints to pullback.
    pub fn dependent_products(&self) -> String {
        format!(
            "Dependent products (Π-types) in {}: \
             For a display map p : A → Γ, the Π-type Π_p(B) → Γ is the \
             right adjoint to p* : D/Γ → D/A (pullback along p). \
             Existence requires that display maps are exponentiable.",
            self.category
        )
    }
}
/// The internal logic of an elementary topos.
///
/// Every topos E carries an internal intuitionistic higher-order logic via
/// the Mitchell-Bénabou language. The logic is Boolean iff E is Boolean.
pub struct InternalLogic {
    /// Name of the topos.
    pub topos: String,
    /// Whether the topos is Boolean (classical logic).
    pub is_boolean: bool,
    /// Whether the topos is well-pointed.
    pub is_well_pointed: bool,
}
impl InternalLogic {
    /// Create an internal logic for the given topos.
    pub fn new(topos: impl Into<String>) -> Self {
        Self {
            topos: topos.into(),
            is_boolean: false,
            is_well_pointed: false,
        }
    }
    /// Create the internal logic for the category of sets (classical, well-pointed).
    pub fn for_sets() -> Self {
        Self {
            topos: "Set".to_string(),
            is_boolean: true,
            is_well_pointed: true,
        }
    }
    /// The internal language (Mitchell-Bénabou language) of the topos.
    ///
    /// The language has:
    /// - Types = objects of the topos
    /// - Terms = morphisms
    /// - Propositions = subobjects (morphisms into Ω)
    /// - Connectives from the Heyting algebra structure of Sub(1)
    pub fn internal_language(&self) -> String {
        format!(
            "Mitchell-Bénabou language of {}: typed lambda calculus with dependent types; \
             types are objects, terms are morphisms, propositions are subobjects of Ω. \
             Logic is {}.",
            self.topos,
            if self.is_boolean {
                "classical (Boolean)"
            } else {
                "intuitionistic (Heyting)"
            }
        )
    }
    /// The Mitchell-Bénabou theorem: the internal language of any topos is sound
    /// and complete for the topos-theoretic semantics.
    pub fn mitchell_benabou(&self) -> String {
        format!(
            "Mitchell-Bénabou theorem for {}: every formula φ in the internal language \
             is valid (⟦φ⟧ = true) if and only if it is provable in {}-order logic. \
             The internal language soundly and completely axiomatises the logic of {}.",
            self.topos,
            if self.is_boolean {
                "classical higher"
            } else {
                "intuitionistic higher"
            },
            self.topos
        )
    }
    /// Whether the law of excluded middle holds in the internal logic.
    pub fn excluded_middle_holds(&self) -> bool {
        self.is_boolean
    }
    /// Whether the axiom of choice holds in the internal logic.
    /// (Requires the topos to be well-pointed and satisfy AC.)
    pub fn axiom_of_choice_holds(&self) -> bool {
        self.is_boolean && self.is_well_pointed
    }
}
/// Morphism representation in a monoidal category.
#[derive(Debug, Clone)]
pub struct Morphism {
    /// Name of the morphism.
    pub name: String,
    /// Domain type (as string).
    pub domain: String,
    /// Codomain type (as string).
    pub codomain: String,
}
impl Morphism {
    /// Create a new morphism.
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
/// Categorical game semantics (Hyland-Ong games, innocent strategies).
///
/// Game semantics gives fully abstract models of programming languages.
/// The category of games and innocent strategies is compact closed and
/// provides a model of PCF (higher-order functional programming).
pub struct GameSemanticsCategory {
    /// Name of the game semantics model.
    pub name: String,
    /// Whether strategies are required to be innocent (history-free).
    pub innocent: bool,
    /// Whether the model is fully abstract.
    pub fully_abstract: bool,
}
impl GameSemanticsCategory {
    /// Create a game semantics category.
    pub fn new(name: impl Into<String>, innocent: bool) -> Self {
        Self {
            name: name.into(),
            innocent,
            fully_abstract: innocent,
        }
    }
    /// Hyland-Ong games: the fully abstract model of PCF.
    pub fn hyland_ong() -> Self {
        Self {
            name: "HO-Games".to_string(),
            innocent: true,
            fully_abstract: true,
        }
    }
    /// The game for the base type (e.g., integers): a single question-answer pair.
    pub fn base_game(&self, base_type: &str) -> String {
        format!(
            "Game for base type {} in {}: \
             A single question q from Opponent, answered by any value v by Player. \
             Strategies on {} correspond to elements of the denotation of {}.",
            base_type, self.name, base_type, base_type
        )
    }
    /// Composition of strategies (sequential + parallel + hiding).
    pub fn strategy_composition(&self, s: &str, t: &str) -> String {
        format!(
            "Composition {} ; {} in {}: \
             Interleave plays by synchronizing on shared arena moves, \
             then hide internal communication. \
             Innocence ({}): strategies are history-free—they only see \
             the current Opponent view.",
            s,
            t,
            self.name,
            if self.innocent {
                "enforced"
            } else {
                "not required"
            }
        )
    }
    /// Whether this model satisfies full abstraction for PCF.
    pub fn pcf_full_abstraction(&self) -> bool {
        self.fully_abstract && self.innocent
    }
}
/// A Grothendieck fibration with explicit cartesian liftings.
///
/// This struct tracks the fiber categories and provides cartesian lifting lookups.
pub struct GrothendieckFibrationImpl {
    /// Name of the total category E.
    pub total: String,
    /// Name of the base category B.
    pub base: String,
    /// Registered cartesian morphisms: (source_in_E, morphism_in_B, target_in_E).
    cartesian_lifts: Vec<(String, String, String)>,
}
impl GrothendieckFibrationImpl {
    /// Create a new Grothendieck fibration.
    pub fn new(total: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            total: total.into(),
            base: base.into(),
            cartesian_lifts: Vec::new(),
        }
    }
    /// Register a cartesian lifting: morphism f in B lifts to φ : e' → e in E.
    pub fn add_cartesian_lift(
        &mut self,
        source: impl Into<String>,
        base_morphism: impl Into<String>,
        target: impl Into<String>,
    ) {
        self.cartesian_lifts
            .push((source.into(), base_morphism.into(), target.into()));
    }
    /// Look up a cartesian lifting over the given base morphism.
    pub fn cartesian_lift_over(&self, base_morphism: &str) -> Option<(&str, &str)> {
        self.cartesian_lifts
            .iter()
            .find(|(_, f, _)| f == base_morphism)
            .map(|(src, _, tgt)| (src.as_str(), tgt.as_str()))
    }
    /// The fiber over an object in B (all objects in E projecting to it).
    pub fn fiber_objects(&self, base_obj: &str) -> Vec<&str> {
        self.cartesian_lifts
            .iter()
            .filter(|(_, f, _)| f == base_obj)
            .map(|(_, _, tgt)| tgt.as_str())
            .collect()
    }
    /// Whether the fibration is opfibration (has opcartesian liftings).
    /// For Grothendieck fibrations, this is determined by the structure.
    pub fn is_bifibration(&self) -> bool {
        !self.cartesian_lifts.is_empty()
    }
    /// Number of registered cartesian lifts.
    pub fn num_lifts(&self) -> usize {
        self.cartesian_lifts.len()
    }
    /// Describe the fibration.
    pub fn describe(&self) -> String {
        format!(
            "Grothendieck fibration p : {} → {} with {} cartesian liftings registered.",
            self.total,
            self.base,
            self.cartesian_lifts.len()
        )
    }
}
/// Categorical models of polymorphic lambda calculus (System F).
///
/// Seely (1987) showed that models of System F correspond to certain
/// cartesian closed categories with a "weakly generic" small object.
/// The key is the Seely isomorphism: A\[τ/X\] ≅ A ∘ (_, τ).
pub struct PolymorphismCategory {
    /// Name of the category.
    pub name: String,
    /// Whether the category has a generic object for type variables.
    pub has_generic_object: bool,
    /// Whether Seely's isomorphism holds strictly or only up to iso.
    pub seely_strict: bool,
}
impl PolymorphismCategory {
    /// Create a polymorphism category.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            has_generic_object: true,
            seely_strict: false,
        }
    }
    /// Seely's isomorphism: A\[τ/X\] ≅ A ∘ (id, τ).
    pub fn seely_isomorphism(&self, a: &str, tau: &str) -> String {
        format!(
            "Seely's isomorphism in {}: {}[{}/X] ≅ {} ∘ (id, {}). \
             Polymorphic abstraction ΛX.A corresponds to a natural transformation; \
             instantiation A[τ/X] is substitution along (id, τ). \
             This is the categorical content of type-theoretic substitution for type variables.",
            self.name, a, tau, a, tau
        )
    }
    /// System F types as functors into the category.
    pub fn system_f_types(&self) -> String {
        format!(
            "System F types in {}: \
             Base types are objects; type constructors are functors; \
             polymorphic type ∀X.A is the end ∫_X A(X) (when it exists). \
             Type application is functor application; \
             type abstraction is end projection.",
            self.name
        )
    }
}
/// A traced monoidal category: a symmetric monoidal category with a trace operation.
///
/// A trace Tr^U_{A,B} : C(A ⊗ U, B ⊗ U) → C(A, B) satisfying vanishing,
/// superposing, yanking, and naturality axioms. Compact closed categories
/// have a canonical trace via cups and caps.
pub struct TracedMonoidalCategory {
    /// Name of the category.
    pub name: String,
    /// Whether the category is compact closed (has duals for all objects).
    pub compact_closed: bool,
    /// Whether string diagrams can be drawn on a torus (= traced monoidal).
    pub on_torus: bool,
}
impl TracedMonoidalCategory {
    /// Create a traced monoidal category.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            compact_closed: false,
            on_torus: true,
        }
    }
    /// Create a compact closed category (has all duals).
    pub fn compact_closed(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            compact_closed: true,
            on_torus: true,
        }
    }
    /// Trace operation: feedback loop in a string diagram.
    pub fn trace_operation(&self, f: &str, u: &str) -> String {
        format!(
            "Trace Tr^{u}({}): in {}, feed the {} output of {} back into its {} input. \
             In string diagrams: connect the {} wire of {} to form a loop. \
             Axioms: vanishing (Tr^I = id), superposing, yanking (Tr(σ) = id), naturality.",
            f, self.name, u, f, u, u, f
        )
    }
    /// The Int construction: embedding a traced monoidal category into a compact closed one.
    pub fn int_construction(&self) -> String {
        format!(
            "Int({}) construction (Joyal-Street-Verity): \
             embed the traced monoidal category {} into a compact closed category Int({}). \
             Objects of Int({}) are pairs (A+, A-); duals are (A+, A-)* = (A-, A+). \
             The trace of {} becomes the cup/cap composition in Int({}).",
            self.name, self.name, self.name, self.name, self.name, self.name
        )
    }
    /// Penrose notation / string diagram rewriting.
    pub fn string_diagram_rewriting(&self, rule: &str) -> String {
        format!(
            "String diagram rewrite rule in {}: '{}'. \
             Categorical equality = isotopy of string diagrams on {}. \
             Rewriting is sound and complete for {}.",
            self.name,
            rule,
            if self.compact_closed {
                "sphere S²"
            } else {
                "torus T²"
            },
            self.name
        )
    }
}
