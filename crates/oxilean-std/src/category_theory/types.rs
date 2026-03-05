//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// (∞,1)-category (quasi-category model).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InftyCat {
    pub name: String,
    pub model: InftyCatModel,
}
impl InftyCat {
    #[allow(dead_code)]
    pub fn new(name: &str, model: InftyCatModel) -> Self {
        Self {
            name: name.to_string(),
            model,
        }
    }
    #[allow(dead_code)]
    pub fn grpd_enriched() -> Self {
        Self::new("Space (infinity-groupoid)", InftyCatModel::Quasicategory)
    }
    #[allow(dead_code)]
    pub fn stable_infty_cat() -> Self {
        Self::new("Stable infinity-category", InftyCatModel::Quasicategory)
    }
    #[allow(dead_code)]
    pub fn lurie_description(&self) -> String {
        match &self.model {
            InftyCatModel::Quasicategory => {
                format!("{}: simplicial set with inner horn filling", self.name)
            }
            InftyCatModel::CompleteSegals => {
                format!("{}: complete Segal space", self.name)
            }
            _ => format!("{}: infinity-category model", self.name),
        }
    }
    #[allow(dead_code)]
    pub fn limits_and_colimits_exist(&self) -> bool {
        true
    }
}
/// An enriched category (category enriched over a monoidal category V).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnrichedCategory {
    pub name: String,
    pub enriching_category: String,
}
impl EnrichedCategory {
    #[allow(dead_code)]
    pub fn new(name: &str, enriching: &str) -> Self {
        Self {
            name: name.to_string(),
            enriching_category: enriching.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set_enriched() -> Self {
        Self::new("ordinary Cat", "Set")
    }
    #[allow(dead_code)]
    pub fn abelian_group_enriched() -> Self {
        Self::new("additive Cat", "Ab (abelian groups)")
    }
    #[allow(dead_code)]
    pub fn chain_complex_enriched() -> Self {
        Self::new("dg-Cat", "Ch(k) (chain complexes)")
    }
    #[allow(dead_code)]
    pub fn composition_map(&self) -> String {
        format!(
            "comp: hom(B,C) tensor hom(A,B) -> hom(A,C) in {}",
            self.enriching_category
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnrichedCat {
    pub name: String,
    pub enriching_category: String,
    pub hom_objects: String,
    pub is_self_enriched: bool,
}
#[allow(dead_code)]
impl EnrichedCat {
    pub fn new(name: &str, enriching: &str, hom: &str) -> Self {
        EnrichedCat {
            name: name.to_string(),
            enriching_category: enriching.to_string(),
            hom_objects: hom.to_string(),
            is_self_enriched: false,
        }
    }
    pub fn ab_cat() -> Self {
        EnrichedCat {
            name: "Ab-Cat".to_string(),
            enriching_category: "Ab".to_string(),
            hom_objects: "Abelian groups".to_string(),
            is_self_enriched: false,
        }
    }
    pub fn simplicial_category() -> Self {
        EnrichedCat {
            name: "sSet-Cat".to_string(),
            enriching_category: "sSet".to_string(),
            hom_objects: "Simplicial sets".to_string(),
            is_self_enriched: false,
        }
    }
    pub fn cat_self_enriched() -> Self {
        EnrichedCat {
            name: "CAT".to_string(),
            enriching_category: "CAT".to_string(),
            hom_objects: "Functor categories [C,D]".to_string(),
            is_self_enriched: true,
        }
    }
    pub fn composition_law(&self) -> String {
        format!(
            "Composition in {}: ⊗-morphism {} ⊗ {} → {}",
            self.name, self.hom_objects, self.hom_objects, self.hom_objects
        )
    }
    pub fn yoneda_enriched(&self) -> String {
        format!(
            "Enriched Yoneda: 𝒞(-, A) : 𝒞^op → {} is representable",
            self.enriching_category
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Bicategory {
    pub name: String,
    pub zero_cells: String,
    pub one_cells: String,
    pub two_cells: String,
    pub is_strict: bool,
}
#[allow(dead_code)]
impl Bicategory {
    pub fn new(name: &str, zero: &str, one: &str, two: &str) -> Self {
        Bicategory {
            name: name.to_string(),
            zero_cells: zero.to_string(),
            one_cells: one.to_string(),
            two_cells: two.to_string(),
            is_strict: false,
        }
    }
    pub fn two_cat() -> Self {
        Bicategory {
            name: "2-Cat".to_string(),
            zero_cells: "small categories".to_string(),
            one_cells: "functors".to_string(),
            two_cells: "natural transformations".to_string(),
            is_strict: true,
        }
    }
    pub fn bicat_rings() -> Self {
        Bicategory {
            name: "Ring".to_string(),
            zero_cells: "rings".to_string(),
            one_cells: "bimodules".to_string(),
            two_cells: "bimodule maps".to_string(),
            is_strict: false,
        }
    }
    pub fn coherence_theorem(&self) -> String {
        if self.is_strict {
            format!(
                "Strict 2-cat {}: associativity and unit hold strictly",
                self.name
            )
        } else {
            format!(
                "Bicategory {}: coherence theorem → every bicategory is biequivalent to a strict 2-category",
                self.name
            )
        }
    }
    pub fn horizontal_composition(&self) -> String {
        format!(
            "Horizontal: {}-morphisms compose via {}",
            self.one_cells, self.zero_cells
        )
    }
    pub fn interchange_law(&self) -> String {
        "(f•g) ∘ (h•k) = (f∘h)•(g∘k) (interchange for 2-cells)".to_string()
    }
}
/// Fibration (Grothendieck fibration).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Fibration {
    pub total_category: String,
    pub base_category: String,
    pub functor_p: String,
}
impl Fibration {
    #[allow(dead_code)]
    pub fn new(total: &str, base: &str, p: &str) -> Self {
        Self {
            total_category: total.to_string(),
            base_category: base.to_string(),
            functor_p: p.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn cartesian_lifting_description(&self) -> String {
        format!(
            "p: {} -> {}: for every morphism f: I -> J in {} and a object over J, \
             exists a cartesian lift of f",
            self.total_category, self.base_category, self.base_category
        )
    }
    #[allow(dead_code)]
    pub fn fiber_over(&self, obj: &str) -> String {
        format!(
            "Fiber of {} over {}: preimage p^-1({})",
            self.functor_p, obj, obj
        )
    }
}
/// Topos.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Topos {
    pub name: String,
    pub is_grothendieck: bool,
    pub has_natural_number_object: bool,
    pub site_name: Option<String>,
}
impl Topos {
    #[allow(dead_code)]
    pub fn set_topos() -> Self {
        Self {
            name: "Set".to_string(),
            is_grothendieck: true,
            has_natural_number_object: true,
            site_name: None,
        }
    }
    #[allow(dead_code)]
    pub fn sheaves_on_site(site: &str) -> Self {
        Self {
            name: format!("Sh({})", site),
            is_grothendieck: true,
            has_natural_number_object: true,
            site_name: Some(site.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn classifying_topos(theory: &str) -> Self {
        Self {
            name: format!("B({})", theory),
            is_grothendieck: true,
            has_natural_number_object: true,
            site_name: Some(theory.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn subobject_classifier_description(&self) -> String {
        format!(
            "Omega in {}: Sub(A) = hom(A, Omega) for any A in topos",
            self.name
        )
    }
    #[allow(dead_code)]
    pub fn internal_logic(&self) -> String {
        format!(
            "Internal logic of {} is intuitionistic higher-order type theory",
            self.name
        )
    }
}
/// Lawvere doctrine (functorial semantics).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Doctrine {
    pub name: String,
    pub fiber_description: String,
}
impl Doctrine {
    #[allow(dead_code)]
    pub fn regular() -> Self {
        Self {
            name: "Regular doctrine".to_string(),
            fiber_description: "Heyting algebras with image factorization".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn coherent() -> Self {
        Self {
            name: "Coherent doctrine".to_string(),
            fiber_description: "Distributive lattices with finite colimits".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn first_order() -> Self {
        Self {
            name: "First-order doctrine".to_string(),
            fiber_description: "Complete Heyting algebras with Beck-Chevalley".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn classifying_category(&self) -> String {
        format!("Syntactic category of {} theory", self.name)
    }
}
/// Kan extension (left and right).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KanExtension {
    pub functor_f: String,
    pub functor_k: String,
    pub is_left: bool,
}
impl KanExtension {
    #[allow(dead_code)]
    pub fn left(f: &str, k: &str) -> Self {
        Self {
            functor_f: f.to_string(),
            functor_k: k.to_string(),
            is_left: true,
        }
    }
    #[allow(dead_code)]
    pub fn right(f: &str, k: &str) -> Self {
        Self {
            functor_f: f.to_string(),
            functor_k: k.to_string(),
            is_left: false,
        }
    }
    #[allow(dead_code)]
    pub fn universal_property(&self) -> String {
        if self.is_left {
            format!(
                "Lan_K F is left adjoint to precomposition with K for {} along {}",
                self.functor_f, self.functor_k
            )
        } else {
            format!(
                "Ran_K F is right adjoint to precomposition with K for {} along {}",
                self.functor_f, self.functor_k
            )
        }
    }
    #[allow(dead_code)]
    pub fn mac_lane_coend_formula(&self) -> String {
        if self.is_left {
            format!("Lan_K F(d) = coend^c F(c) * hom(K(c), d)")
        } else {
            format!("Ran_K F(d) = end_c [hom(d, K(c)), F(c)]")
        }
    }
}
/// Model category (Quillen).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModelCategory {
    pub name: String,
    pub cofibrations: String,
    pub fibrations: String,
    pub weak_equivalences: String,
}
impl ModelCategory {
    #[allow(dead_code)]
    pub fn new(name: &str, cof: &str, fib: &str, we: &str) -> Self {
        Self {
            name: name.to_string(),
            cofibrations: cof.to_string(),
            fibrations: fib.to_string(),
            weak_equivalences: we.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn top_model_category() -> Self {
        Self::new(
            "Top (topological spaces)",
            "Retracts of relative CW complexes",
            "Serre fibrations",
            "Weak homotopy equivalences",
        )
    }
    #[allow(dead_code)]
    pub fn chain_complex_model() -> Self {
        Self::new(
            "Ch(R) (chain complexes over R)",
            "Monomorphisms",
            "Epimorphisms with injective kernel",
            "Quasi-isomorphisms",
        )
    }
    #[allow(dead_code)]
    pub fn homotopy_category_description(&self) -> String {
        format!(
            "Ho({}) = {}[W^-1] (localize at weak equivalences)",
            self.name, self.name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocallyCartesianClosed {
    pub category: String,
    pub has_finite_limits: bool,
    pub pi_types_as_right_adjoints: bool,
    pub sigma_types_as_composition: bool,
    pub id_types: bool,
}
#[allow(dead_code)]
impl LocallyCartesianClosed {
    pub fn new(cat: &str) -> Self {
        LocallyCartesianClosed {
            category: cat.to_string(),
            has_finite_limits: true,
            pi_types_as_right_adjoints: true,
            sigma_types_as_composition: true,
            id_types: false,
        }
    }
    pub fn with_id_types(mut self) -> Self {
        self.id_types = true;
        self
    }
    pub fn seely_equivalence(&self) -> String {
        format!(
            "Seely: MLTT without Id ↔ locally Cartesian closed category ({})",
            self.category
        )
    }
    pub fn lawvere_comprehension(&self) -> String {
        "Lawvere: dependent types = fibrations; Σ = left adjoint, Π = right adjoint".to_string()
    }
    pub fn categorical_sigma_type(&self) -> String {
        "Σ(A, B) = Σ_f B for projection f: ΣA → A (composition of fibrations)".to_string()
    }
    pub fn categorical_pi_type(&self) -> String {
        "Π(A, B) = Π_f B for projection f: A → Γ (right adjoint to f*)".to_string()
    }
}
/// Monad in a 2-category.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Monad2Cat {
    pub category: String,
    pub functor_t: String,
    pub is_commutative: bool,
}
impl Monad2Cat {
    #[allow(dead_code)]
    pub fn new(cat: &str, t: &str, comm: bool) -> Self {
        Self {
            category: cat.to_string(),
            functor_t: t.to_string(),
            is_commutative: comm,
        }
    }
    #[allow(dead_code)]
    pub fn kleisli_category_description(&self) -> String {
        format!(
            "Kleisli({}, {}): morphisms A -> B are maps A -> {}(B)",
            self.category, self.functor_t, self.functor_t
        )
    }
    #[allow(dead_code)]
    pub fn eilenberg_moore_category_description(&self) -> String {
        format!(
            "EM({}, {}): algebras (A, a: {}(A)->A satisfying unit/mult laws)",
            self.category, self.functor_t, self.functor_t
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MonoidalCategory {
    pub name: String,
    pub tensor_product: String,
    pub unit_object: String,
    pub is_symmetric: bool,
    pub is_braided: bool,
    pub is_closed: bool,
}
#[allow(dead_code)]
impl MonoidalCategory {
    pub fn new(name: &str, tensor: &str, unit: &str) -> Self {
        MonoidalCategory {
            name: name.to_string(),
            tensor_product: tensor.to_string(),
            unit_object: unit.to_string(),
            is_symmetric: false,
            is_braided: false,
            is_closed: false,
        }
    }
    pub fn symmetric(mut self) -> Self {
        self.is_symmetric = true;
        self.is_braided = true;
        self
    }
    pub fn closed(mut self) -> Self {
        self.is_closed = true;
        self
    }
    pub fn vect_over_k() -> Self {
        MonoidalCategory::new("Vect_k", "⊗_k", "k")
            .symmetric()
            .closed()
    }
    pub fn mac_lane_coherence(&self) -> String {
        format!(
            "Mac Lane coherence for {}: all diagrams built from α, λ, ρ commute",
            self.name
        )
    }
    pub fn internal_hom(&self) -> String {
        if self.is_closed {
            format!(
                "[A, B] internal hom in {} (right adjoint to {} ⊗ -)",
                self.name, self.tensor_product
            )
        } else {
            format!("{} is not closed (no internal hom)", self.name)
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InftyCatModel {
    Quasicategory,
    CompleteSegals,
    SegalSpaces,
    SimplicialCats,
    RelativeCategories,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GluingConstruction {
    pub base_category: String,
    pub gluing_category: String,
    pub reflection: String,
    pub is_logical_relation: bool,
}
#[allow(dead_code)]
impl GluingConstruction {
    pub fn new(base: &str, gluing: &str, refl: &str) -> Self {
        GluingConstruction {
            base_category: base.to_string(),
            gluing_category: gluing.to_string(),
            reflection: refl.to_string(),
            is_logical_relation: true,
        }
    }
    pub fn normalization_by_evaluation(&self) -> String {
        format!(
            "NbE via gluing: {} → {} (reflection = {})",
            self.base_category, self.gluing_category, self.reflection
        )
    }
    pub fn canonicity_via_gluing(&self) -> String {
        "Canonicity theorem: every closed term of type Nat normalizes to a numeral".to_string()
    }
    pub fn sterling_method_description(&self) -> String {
        "Sterling's synthetic Tait computability: gluing at presheaf models".to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DoubleCategory {
    pub objects: String,
    pub horizontal_morphisms: String,
    pub vertical_morphisms: String,
    pub squares: String,
}
#[allow(dead_code)]
impl DoubleCategory {
    pub fn new(obj: &str, horiz: &str, vert: &str, sq: &str) -> Self {
        DoubleCategory {
            objects: obj.to_string(),
            horizontal_morphisms: horiz.to_string(),
            vertical_morphisms: vert.to_string(),
            squares: sq.to_string(),
        }
    }
    pub fn spans() -> Self {
        DoubleCategory {
            objects: "sets".to_string(),
            horizontal_morphisms: "spans".to_string(),
            vertical_morphisms: "functions".to_string(),
            squares: "maps of spans".to_string(),
        }
    }
    pub fn globular_cells_count(&self) -> usize {
        4
    }
    pub fn shulman_connection_to_fibrations(&self) -> String {
        "Shulman: double categories capture fibrant objects in appropriate model structure"
            .to_string()
    }
}
/// End and coend.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EndCoend {
    pub bifunctor: String,
    pub category: String,
    pub is_end: bool,
}
impl EndCoend {
    #[allow(dead_code)]
    pub fn end(bifunctor: &str, cat: &str) -> Self {
        Self {
            bifunctor: bifunctor.to_string(),
            category: cat.to_string(),
            is_end: true,
        }
    }
    #[allow(dead_code)]
    pub fn coend(bifunctor: &str, cat: &str) -> Self {
        Self {
            bifunctor: bifunctor.to_string(),
            category: cat.to_string(),
            is_end: false,
        }
    }
    #[allow(dead_code)]
    pub fn wedge_condition(&self) -> String {
        if self.is_end {
            format!("End_c T(c,c): for all f: c->d, dinat(d)*T(1,f)=dinat(c)*T(f,1)")
        } else {
            format!("Coend^c T(c,c): colimit of T(c,c) with codinat maps")
        }
    }
}
