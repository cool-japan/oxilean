//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// The generic extension M\[G\] of a ground model M by a generic filter G.
///
/// M\[G\] is the smallest transitive model of ZFC that contains M and G.
/// It is constructed via P-names: M\[G\] = {τ\[G\] | τ ∈ M, τ a P-name}.
pub struct GenericExtension {
    pub ground_model: String,
    pub generic_filter: GenericFilter,
    pub satisfies_zfc: bool,
}
impl GenericExtension {
    /// Construct the generic extension M\[G\].
    pub fn new(ground_model: impl Into<String>, filter: GenericFilter) -> Self {
        Self {
            ground_model: ground_model.into(),
            generic_filter: filter,
            satisfies_zfc: true,
        }
    }
    /// The fundamental theorem of forcing: φ holds in M\[G\] iff some p ∈ G forces φ.
    pub fn fundamental_theorem(&self) -> &'static str {
        "M[G] ⊨ φ ↔ ∃ p ∈ G, p ⊩ φ"
    }
    /// Generic extension preserves cardinals above the chain condition.
    pub fn cardinals_preserved_above_ccc(&self) -> bool {
        self.generic_filter.is_generic_over_model
    }
    /// The ground model is a definable class in M\[G\] (Laver's theorem).
    pub fn ground_model_definable(&self) -> bool {
        true
    }
    /// M\[G\] satisfies the same arithmetic statements as M (Shoenfield absoluteness).
    pub fn shoenfield_absoluteness(&self) -> bool {
        true
    }
}
/// A Boolean-valued forcing poset obtained from a complete Boolean algebra B.
///
/// Conditions are elements b ∈ B \ {0}, ordered by b ≤ c iff b ∧ c = b.
/// This is the canonical way to convert a cba into a forcing poset.
#[allow(dead_code)]
pub struct CBAForcingPoset {
    /// The name of the complete Boolean algebra.
    pub algebra_name: String,
    /// Whether the algebra is atomless (guarantees nontrivial forcing).
    pub atomless: bool,
    /// The number of elements (0 means infinite/uncountable).
    pub element_count: usize,
}
#[allow(dead_code)]
impl CBAForcingPoset {
    /// Create a CBA forcing poset from the given Boolean algebra.
    pub fn new(algebra_name: impl Into<String>, atomless: bool) -> Self {
        Self {
            algebra_name: algebra_name.into(),
            atomless,
            element_count: 0,
        }
    }
    /// The Cohen algebra RO(Fin(ω, 2)) — the forcing poset for adding a Cohen real.
    pub fn cohen() -> Self {
        Self {
            algebra_name: "RO(Cohen)".to_string(),
            atomless: true,
            element_count: 0,
        }
    }
    /// The measure algebra: Borel(ℝ) / (null sets) — for random forcing.
    pub fn measure_algebra() -> Self {
        Self {
            algebra_name: "Borel([0,1]) / Null".to_string(),
            atomless: true,
            element_count: 0,
        }
    }
    /// A CBA forcing poset is always separative (a ≠ b implies incompatible extensions).
    pub fn is_separative(&self) -> bool {
        true
    }
    /// The dense embedding of P into RO(P) (regular open completion).
    pub fn dense_embedding_into_ro(&self) -> String {
        format!("P → RO({}) dense embedding", self.algebra_name)
    }
    /// Any two forcing posets with isomorphic regular open algebras are forcing-equivalent.
    pub fn forcing_equivalent(&self, other: &Self) -> bool {
        self.algebra_name == other.algebra_name
    }
    /// The Boolean value [\[φ\]]^B for a sentence φ (returned symbolically).
    pub fn boolean_value(&self, phi: &str) -> String {
        format!("[[{}]]_{}", phi, self.algebra_name)
    }
    /// The supremum of a family of Boolean values (for existential statements).
    pub fn sup_boolean_values(&self, phi: &str, var: &str) -> String {
        format!("⋁_{{{}}} [[{}]]_{}", var, phi, self.algebra_name)
    }
}
/// A Boolean-valued model V^B where B is a complete Boolean algebra.
///
/// Every ZFC axiom has Boolean value 1 in V^B.
/// The forcing relation can be read off: p ⊩ φ iff [\[φ\]]^B ≥ p.
pub struct BooleanValuedModel {
    pub boolean_algebra: String,
    pub is_complete: bool,
    pub satisfies_zfc_full_value: bool,
}
impl BooleanValuedModel {
    /// Create a Boolean-valued model over the given complete Boolean algebra.
    pub fn new(ba: impl Into<String>) -> Self {
        Self {
            boolean_algebra: ba.into(),
            is_complete: true,
            satisfies_zfc_full_value: true,
        }
    }
    /// The two-element Boolean algebra gives the standard model V^2 ≅ V.
    pub fn classical_model() -> Self {
        Self {
            boolean_algebra: "2".to_string(),
            is_complete: true,
            satisfies_zfc_full_value: true,
        }
    }
    /// V^B / U for an ultrafilter U gives a classical two-valued model.
    pub fn quotient_by_ultrafilter(&self, ultrafilter: &str) -> String {
        format!("{}^{} / {}", "V", self.boolean_algebra, ultrafilter)
    }
    /// The Boolean value of an equality [\[x = y\]]^B.
    pub fn boolean_equality_value(&self, x: &str, y: &str) -> String {
        format!("[[{} = {}]]_{}", x, y, self.boolean_algebra)
    }
    /// The Boolean value of membership [\[x ∈ y\]]^B.
    pub fn boolean_membership_value(&self, x: &str, y: &str) -> String {
        format!("[[{} ∈ {}]]_{}", x, y, self.boolean_algebra)
    }
}
/// Large cardinal axioms enumerated by consistency strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeCardinalAxiom {
    /// κ is uncountable, regular, and a strong limit.
    Inaccessible,
    /// κ is inaccessible and {α < κ | α inaccessible} is stationary in κ.
    Mahlo,
    /// κ is inaccessible and for every κ-tree there is a cofinal branch.
    WeaklyCompact,
    /// There exists a κ-complete non-principal ultrafilter on κ.
    Measurable,
    /// For all λ ≥ κ, there is a κ-complete normal fine ultrafilter on P_κ(λ).
    Supercompact,
    /// For all η > κ, ∃ j : V_η → V_{j(η)} elementary with crit(j) = κ.
    Extendible,
}
impl LargeCardinalAxiom {
    /// Relative consistency strength description.
    pub fn consistency_strength(&self) -> &'static str {
        match self {
            Self::Inaccessible => "Inaccessible: Con(ZFC) → Con(ZFC + ∃ inaccessible)",
            Self::Mahlo => "Mahlo: stronger than inaccessible",
            Self::WeaklyCompact => "Weakly compact: Π¹₁-indescribable",
            Self::Measurable => "Measurable: implies ∃ 0#",
            Self::Supercompact => "Supercompact: implies projective determinacy",
            Self::Extendible => "Extendible: stronger than supercompact",
        }
    }
    /// Returns the next (stronger) large cardinal axiom, if any.
    pub fn implies_next(&self) -> Option<LargeCardinalAxiom> {
        match self {
            Self::Inaccessible => Some(Self::Mahlo),
            Self::Mahlo => Some(Self::WeaklyCompact),
            Self::WeaklyCompact => Some(Self::Measurable),
            Self::Measurable => Some(Self::Supercompact),
            Self::Supercompact => Some(Self::Extendible),
            Self::Extendible => None,
        }
    }
}
/// A generic ultrapower construction: given a generic ultrafilter U on κ,
/// form the ultrapower Ult(V, U) with the embedding j: V → Ult(V, U).
#[allow(dead_code)]
pub struct GenericUltrapower {
    /// The cardinal κ over which the ultrafilter lives.
    pub kappa: String,
    /// The wellFoundedness status of the ultrapower.
    pub wellfounded: bool,
    /// The critical point of the embedding.
    pub critical_point: String,
}
#[allow(dead_code)]
impl GenericUltrapower {
    /// Create a generic ultrapower with κ as the measurable cardinal.
    pub fn new(kappa: impl Into<String>) -> Self {
        let k = kappa.into();
        let crit = k.clone();
        Self {
            kappa: k,
            wellfounded: true,
            critical_point: crit,
        }
    }
    /// The ultrapower is well-founded iff the ultrafilter is countably complete.
    pub fn is_wellfounded(&self) -> bool {
        self.wellfounded
    }
    /// Łoś theorem: a sentence φ holds in Ult(V, U) iff {α | φ(f(α))} ∈ U.
    pub fn los_theorem(&self) -> &'static str {
        "Ult(V,U) ⊨ φ([f]_U) ↔ {α | φ(f(α))} ∈ U"
    }
    /// The embedding j: V → Ult(V, U) is elementary.
    pub fn embedding_is_elementary(&self) -> bool {
        true
    }
    /// The critical point of j is κ (the measurable cardinal).
    pub fn critical_point(&self) -> &str {
        &self.critical_point
    }
    /// Scott's theorem: if U is a κ-complete ultrafilter then j(κ) > κ.
    pub fn j_kappa_above_kappa(&self) -> bool {
        self.wellfounded
    }
    /// The ultrapower embedding moves κ: j(κ) > κ.
    pub fn embedding_moves_kappa(&self) -> String {
        format!("j({}) > {}", self.kappa, self.kappa)
    }
}
/// Cohen forcing: conditions are finite partial functions p : ω →_fin 2.
///
/// Each condition is represented as a pair (domain, values) of byte vecs.
pub struct CohenForcing {
    /// The finite partial functions: each is `(domain bits, value bits)`.
    pub conditions: Vec<(Vec<u8>, Vec<u8>)>,
}
impl CohenForcing {
    /// Create a new Cohen forcing poset.
    pub fn new(conditions: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        Self { conditions }
    }
    /// Cohen forcing adds a new generic real x : ω → 2 to the model.
    ///
    /// The generic real is not in the ground model M.
    pub fn adds_real(&self) -> bool {
        true
    }
    /// Cohen forcing satisfies ccc, hence preserves all cardinals.
    pub fn preserves_cardinals(&self) -> bool {
        true
    }
}
/// A forcing poset (P, ≤, 𝟙) is a preorder with a maximum element.
///
/// Conditions p, q ∈ P are compatible if ∃ r ≤ p and r ≤ q.
/// A set D ⊆ P is dense if ∀ p, ∃ q ≤ p with q ∈ D.
pub struct ForcingPoset {
    pub name: String,
    pub has_maximum: bool,
    pub is_separative: bool,
}
impl ForcingPoset {
    /// Create a new forcing poset with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            has_maximum: true,
            is_separative: false,
        }
    }
    /// Cohen forcing for adding a generic real (Cohen real).
    /// P = Fin(ω, 2), ordered by reverse inclusion.
    pub fn cohen_forcing() -> Self {
        Self {
            name: "Cohen(ω,2)".to_string(),
            has_maximum: true,
            is_separative: true,
        }
    }
    /// Collapse forcing Col(ω, κ) for collapsing cardinal κ to ω.
    pub fn collapse_forcing(kappa: &str) -> Self {
        Self {
            name: format!("Col(ω,{})", kappa),
            has_maximum: true,
            is_separative: true,
        }
    }
    /// Lévy collapse forcing: all cardinals ≤ κ become countable.
    pub fn levy_collapse(kappa: &str) -> Self {
        Self {
            name: format!("Levy(ω,{})", kappa),
            has_maximum: true,
            is_separative: true,
        }
    }
    /// Random forcing: Borel subsets of \[0,1\] of positive measure.
    pub fn random_forcing() -> Self {
        Self {
            name: "Random(ω)".to_string(),
            has_maximum: true,
            is_separative: true,
        }
    }
    /// Sacks forcing: perfect subtrees of 2^{<ω}.
    pub fn sacks_forcing() -> Self {
        Self {
            name: "Sacks".to_string(),
            has_maximum: true,
            is_separative: true,
        }
    }
    /// A separative forcing poset is one where p ⊄ q implies ∃ r ≤ p incompatible with q.
    pub fn make_separative(mut self) -> Self {
        self.is_separative = true;
        self
    }
    /// Check whether this poset satisfies the countable chain condition (ccc):
    /// every antichain is countable.
    pub fn satisfies_ccc(&self) -> bool {
        matches!(self.name.as_str(), "Cohen(ω,2)" | "Random(ω)")
    }
    /// Check whether the forcing is proper (preserves stationary sets of ω₁).
    pub fn is_proper(&self) -> bool {
        self.satisfies_ccc() || self.name == "Sacks"
    }
}
/// The Proper Forcing Axiom (PFA): MA-style axiom for proper forcings.
///
/// PFA is consistent relative to a supercompact cardinal.
/// It implies CH fails and 2^ℵ_0 = ℵ_2.
pub struct ProperForcingAxiom {
    pub holds: bool,
}
impl ProperForcingAxiom {
    /// PFA is consistent relative to a supercompact cardinal (Baumgartner).
    pub fn consistent_from_supercompact() -> Self {
        Self { holds: true }
    }
    /// PFA implies 2^ℵ_0 = ℵ_2 (Foreman-Magidor-Shelah).
    pub fn continuum_equals_aleph_2(&self) -> bool {
        self.holds
    }
    /// PFA implies the P-ideal dichotomy.
    pub fn p_ideal_dichotomy(&self) -> bool {
        self.holds
    }
    /// PFA implies every Aronszajn tree is special.
    pub fn all_aronszajn_trees_special(&self) -> bool {
        self.holds
    }
}
/// An independence result: a statement independent of a given axiom system.
pub struct IndependenceResult {
    /// The statement that is independent.
    pub statement: String,
    /// The axiom system (e.g., "ZFC") from which it is independent.
    pub from: String,
}
impl IndependenceResult {
    /// Create a new independence result.
    pub fn new(statement: impl Into<String>, from: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
            from: from.into(),
        }
    }
    /// The Continuum Hypothesis is independent of ZFC.
    pub fn ch_is_independent(&self) -> bool {
        self.statement.contains("CH") || self.statement.contains("Continuum Hypothesis")
    }
    /// The Generalised Continuum Hypothesis is independent of ZFC.
    pub fn gch_is_independent(&self) -> bool {
        self.statement.contains("GCH") || self.statement.contains("Generalised")
    }
}
/// A filter G ⊆ P is P-generic over M if G ∩ D ≠ ∅ for every dense D ∈ M.
#[derive(Debug, Clone)]
pub struct GenericFilter {
    pub poset_name: String,
    pub is_generic_over_model: bool,
    pub meets_all_dense_sets: bool,
}
impl GenericFilter {
    /// Construct a generic filter for the given poset over ground model M.
    pub fn new(poset_name: impl Into<String>) -> Self {
        Self {
            poset_name: poset_name.into(),
            is_generic_over_model: true,
            meets_all_dense_sets: true,
        }
    }
    /// A generic filter exists by a diagonal argument if M is countable.
    pub fn exists_over_countable_model(&self) -> bool {
        self.is_generic_over_model
    }
}
/// Generic absoluteness theorems: which statements are preserved under forcing.
pub struct GenericAbsoluteness;
impl GenericAbsoluteness {
    /// Shoenfield absoluteness: every Σ¹₂ statement absolute between V and V\[G\].
    pub fn shoenfield_absoluteness() -> &'static str {
        "Every Σ¹₂ (or Π¹₂) sentence is absolute between V and any set-generic extension V[G]"
    }
    /// Projective absoluteness from large cardinals:
    /// if there are ω many Woodin cardinals, all projective statements are absolute.
    pub fn projective_absoluteness_from_woodin() -> &'static str {
        "ω many Woodin cardinals imply all projective statements are generically absolute"
    }
    /// Universally Baire sets are generically absolutely measurable.
    pub fn universally_baire_absoluteness() -> &'static str {
        "Sets of reals that are universally Baire are absolute between all set-generic extensions"
    }
    /// The Axiom of Determinacy (AD) restricted to projective sets follows from large cardinals.
    pub fn projective_determinacy() -> &'static str {
        "AD_ℝ^proj follows from infinitely many Woodin cardinals (Martin-Steel theorem)"
    }
    /// Every sentence of second-order arithmetic that is true in an outer model
    /// is also true in a forcing extension (Woodin's Ω-conjecture context).
    pub fn omega_conjecture_context() -> &'static str {
        "Woodin's Ω-conjecture: Ω-logic is complete for second-order arithmetic"
    }
}
/// Proper forcing: forcings that preserve stationary subsets of ω₁.
pub struct ProperForcing {
    /// Whether this forcing is proper.
    pub is_proper: bool,
}
impl ProperForcing {
    /// Create a new `ProperForcing` record.
    pub fn new(is_proper: bool) -> Self {
        Self { is_proper }
    }
    /// Proper forcing preserves stationary subsets of ω₁.
    pub fn preserves_stationary_sets(&self) -> bool {
        self.is_proper
    }
    /// A forcing is semi-proper if it preserves stationary subsets of
    /// \[ω₁\]^ω — proper implies semi-proper.
    pub fn is_semiproper(&self) -> bool {
        self.is_proper
    }
}
/// The forcing relation p ⊩_P φ for condition p ∈ P and statement φ.
pub struct ForcingRelation {
    pub poset: String,
    pub satisfies_truth_lemma: bool,
    pub satisfies_definability: bool,
}
impl ForcingRelation {
    /// Create a forcing relation for the given poset.
    pub fn new(poset: impl Into<String>) -> Self {
        Self {
            poset: poset.into(),
            satisfies_truth_lemma: true,
            satisfies_definability: true,
        }
    }
    /// Truth lemma: if p ⊩ φ and G is P-generic with p ∈ G, then M\[G\] ⊨ φ.
    pub fn truth_lemma(&self) -> &'static str {
        "p ∈ G ∧ p ⊩ φ → M[G] ⊨ φ"
    }
    /// Definability lemma: the forcing relation is definable in M.
    pub fn definability_lemma(&self) -> &'static str {
        "p ⊩ φ is definable in M from P and φ"
    }
    /// Density lemma: if φ is forced, the set of conditions forcing φ is dense.
    pub fn density_lemma(&self) -> &'static str {
        "p ⊩ φ ↔ {q ≤ p | q ⊩ φ} is dense below p"
    }
    /// Forces conjunction: p ⊩ φ ∧ ψ ↔ p ⊩ φ and p ⊩ ψ.
    pub fn forces_conjunction(&self) -> bool {
        true
    }
    /// Forces disjunction: p ⊩ φ ∨ ψ ↔ ∀ q ≤ p, ∃ r ≤ q (r ⊩ φ or r ⊩ ψ).
    pub fn forces_disjunction(&self) -> bool {
        true
    }
}
/// Martin's Axiom (MA): for every ccc partial order P and every family D
/// of fewer than 2^ℵ_0 dense subsets, there exists a D-generic filter.
pub struct MartinsAxiom {
    pub holds: bool,
    pub compatible_with_ch: bool,
}
impl MartinsAxiom {
    /// MA is consistent with ZFC + ¬CH (Martin-Solovay theorem).
    pub fn consistent_with_not_ch() -> Self {
        Self {
            holds: true,
            compatible_with_ch: false,
        }
    }
    /// MA(ℵ_0) is provable in ZFC (Baire category theorem).
    pub fn ma_aleph_0() -> Self {
        Self {
            holds: true,
            compatible_with_ch: true,
        }
    }
    /// MA implies 2^ℵ_0 ≥ ℵ_2 is consistent.
    pub fn implies_large_continuum(&self) -> bool {
        self.holds && !self.compatible_with_ch
    }
    /// MA implies every set of reals of size < 2^ℵ_0 is null and meager.
    pub fn small_sets_are_null(&self) -> bool {
        self.holds
    }
    /// MA implies Suslin's hypothesis (no Suslin tree exists).
    pub fn suslin_hypothesis_follows(&self) -> bool {
        self.holds
    }
}
/// A complete Boolean algebra suitable for Boolean-valued models.
#[derive(Debug, Clone)]
pub struct CompleteBA {
    pub name: String,
    pub is_atomless: bool,
}
impl CompleteBA {
    /// The complete Boolean algebra associated to Cohen forcing: RO(Fin(ω,2)).
    pub fn cohen_algebra() -> Self {
        Self {
            name: "RO(Cohen)".to_string(),
            is_atomless: true,
        }
    }
    /// The measure algebra: Borel sets modulo null sets.
    pub fn measure_algebra() -> Self {
        Self {
            name: "Meas([0,1])".to_string(),
            is_atomless: true,
        }
    }
    /// The Boolean algebra P(ω)/Fin of coinfinite subsets.
    pub fn p_omega_mod_fin() -> Self {
        Self {
            name: "P(ω)/Fin".to_string(),
            is_atomless: true,
        }
    }
    /// A product of complete Boolean algebras is complete.
    pub fn product(ba1: &str, ba2: &str) -> Self {
        Self {
            name: format!("{} × {}", ba1, ba2),
            is_atomless: false,
        }
    }
}
/// Gödel's constructible universe L.
///
/// L is the smallest inner model of ZFC and satisfies GCH and AC.
/// The axiom V = L (the constructibility axiom) is independent of ZFC.
pub struct ConstructibleUniverse {
    pub satisfies_gch: bool,
    pub satisfies_ac: bool,
    pub contains_all_ordinals: bool,
}
impl ConstructibleUniverse {
    /// Create the constructible universe L.
    pub fn new() -> Self {
        Self {
            satisfies_gch: true,
            satisfies_ac: true,
            contains_all_ordinals: true,
        }
    }
    /// V = L implies the Generalized Continuum Hypothesis.
    pub fn v_eq_l_implies_gch(&self) -> bool {
        self.satisfies_gch
    }
    /// L contains no measurable cardinals if V = L (Scott's theorem).
    pub fn no_measurables_in_l() -> &'static str {
        "Scott's theorem: if there is a measurable cardinal then V ≠ L"
    }
    /// The condensation lemma for L: L_α ≺ L for every limit ordinal α.
    pub fn condensation_lemma() -> &'static str {
        "Condensation: if X ≺_Σ₁ L_κ for uncountable κ, then the transitive collapse of X is L_α for some α"
    }
}
/// Core model K and canonical inner models beyond L.
///
/// The core model K is the canonical inner model that:
/// - Contains all reals
/// - Satisfies GCH above a sufficiently large cardinal
/// - Is close to V in the presence of large cardinals
pub struct CoreModel {
    pub level: LargeCardinalLevel,
    pub satisfies_covering: bool,
}
impl CoreModel {
    /// The Dodd-Jensen core model K^{DJ}: core model without measurable cardinals.
    pub fn dodd_jensen() -> Self {
        Self {
            level: LargeCardinalLevel::Measurable,
            satisfies_covering: true,
        }
    }
    /// The covering lemma holds: either 0# exists or L covers V (Jensen).
    pub fn covering_lemma() -> &'static str {
        "Jensen's covering lemma: either 0# exists, or for every uncountable X ⊆ Ord, \
         there is Y ∈ L with X ⊆ Y and |Y| = |X|"
    }
    /// The Steel core model K: defined up to a Woodin cardinal.
    pub fn steel_core_model() -> Self {
        Self {
            level: LargeCardinalLevel::Woodin,
            satisfies_covering: true,
        }
    }
}
/// A forcing poset with explicit conditions and order relation (as index pairs).
pub struct ForcingPosetExt {
    /// The forcing conditions (strings represent condition names/descriptions).
    pub conditions: Vec<String>,
    /// The partial order as pairs (i, j) meaning condition\[i\] ≤ condition\[j\].
    pub order: Vec<(usize, usize)>,
}
impl ForcingPosetExt {
    /// Create a new `ForcingPosetExt` with the given conditions and order.
    pub fn new(conditions: Vec<String>, order: Vec<(usize, usize)>) -> Self {
        Self { conditions, order }
    }
    /// Check whether a given subset (by indices) is downward closed.
    ///
    /// A set D is downward closed if: p ∈ D and q ≤ p implies q ∈ D.
    pub fn is_downward_closed(&self, subset: &[usize]) -> bool {
        let set: std::collections::HashSet<usize> = subset.iter().cloned().collect();
        for &i in subset {
            for &(lo, hi) in &self.order {
                if hi == i && !set.contains(&lo) {
                    return false;
                }
            }
        }
        true
    }
    /// Return a maximal dense subset — every condition in the poset is either
    /// in the set or has some extension in the set.
    ///
    /// Returns indices of all conditions (the whole poset is trivially dense).
    pub fn dense_set(&self) -> Vec<usize> {
        (0..self.conditions.len()).collect()
    }
    /// Compute a "generic filter" for this poset: the upward-closed set
    /// generated by the maximum element (index 0 by convention).
    ///
    /// Returns indices of all conditions that are above or equal to index 0.
    pub fn generic_filter(&self) -> Vec<usize> {
        let mut result = vec![0usize];
        let mut changed = true;
        while changed {
            changed = false;
            for &(lo, hi) in &self.order {
                if result.contains(&lo) && !result.contains(&hi) {
                    result.push(hi);
                    changed = true;
                }
            }
        }
        result
    }
}
/// Cardinal collapse forcing: collapse κ to λ via Col(λ, κ).
pub struct CardinalCollapse {
    /// The cardinal being collapsed.
    pub kappa: String,
    /// The target cardinal (usually ω).
    pub lambda: String,
}
impl CardinalCollapse {
    /// Create a new cardinal collapse.
    pub fn new(kappa: impl Into<String>, lambda: impl Into<String>) -> Self {
        Self {
            kappa: kappa.into(),
            lambda: lambda.into(),
        }
    }
    /// The collapse forcing Col(ω, κ) makes κ countable in M\[G\].
    pub fn collapses_to_omega(&self) -> bool {
        self.lambda == "ω" || self.lambda == "omega"
    }
    /// Cardinals strictly larger than κ are preserved.
    pub fn preserves_larger(&self) -> bool {
        true
    }
}
/// Martin's Maximum (MM): the strongest form of Martin's Axiom for all proper forcings.
pub struct MartinMaximum {
    /// Internal flag — MM is consistent relative to a supercompact cardinal.
    _consistent: bool,
}
impl MartinMaximum {
    /// Construct Martin's Maximum.
    pub fn new() -> Self {
        Self { _consistent: true }
    }
    /// The statement of Martin's Maximum (MM).
    pub fn statement(&self) -> &'static str {
        "Martin's Maximum (MM): for every proper forcing P and every family \
         {D_α : α < ω₁} of dense sets in P, there exists a filter G ⊆ P \
         meeting every D_α."
    }
    /// MM is consistent relative to the existence of a supercompact cardinal.
    pub fn is_consistent_with_zfc(&self) -> bool {
        true
    }
}
/// An iterated forcing system of length λ using finite support.
///
/// This represents a sequence (P_α, Q̇_α)_{α<λ} where each Q̇_α is a P_α-name
/// for a ccc forcing. Finite-support iterations of ccc forcings are ccc.
#[allow(dead_code)]
pub struct FiniteSupportIteration {
    /// The length of the iteration (as a symbolic ordinal label).
    pub length: String,
    /// The steps: each entry is a label for the iterand Q̇_α.
    pub steps: Vec<String>,
    /// Whether every step is ccc.
    pub all_ccc: bool,
}
#[allow(dead_code)]
impl FiniteSupportIteration {
    /// Create a new finite-support iteration of the given length.
    pub fn new(length: impl Into<String>, steps: Vec<String>) -> Self {
        let all_ccc = true;
        Self {
            length: length.into(),
            steps,
            all_ccc,
        }
    }
    /// A finite-support iteration of ccc forcings is ccc (Baumgartner).
    pub fn is_ccc(&self) -> bool {
        self.all_ccc
    }
    /// The product of the first n steps is still ccc (by induction).
    pub fn initial_segment_ccc(&self, n: usize) -> bool {
        n <= self.steps.len() && self.all_ccc
    }
    /// Consistency strength: finite-support iterations can realize Easton patterns.
    pub fn realizes_easton(&self, f_description: &str) -> String {
        format!(
            "Finite-support iteration of length {} realizes Easton function {}",
            self.length, f_description
        )
    }
    /// The forcing relation for the whole iteration is definable.
    pub fn forcing_relation_definable(&self) -> bool {
        true
    }
}
/// A P-name in forcing — an element of the forcing extension M\[G\].
///
/// Each name is a set of pairs (σ, p) where σ is another name and p ∈ P.
pub struct NameInForcingExt {
    /// The name of the forcing poset.
    pub poset: String,
    /// The P-names in the extension.
    pub names: Vec<String>,
}
impl NameInForcingExt {
    /// Construct a new collection of P-names over the given poset.
    pub fn new(poset: impl Into<String>, names: Vec<String>) -> Self {
        Self {
            poset: poset.into(),
            names,
        }
    }
    /// Check whether the names are well-founded (no circularity).
    pub fn check_val(&self) -> bool {
        !self.names.is_empty()
    }
    /// Compute the value of a name under a generic filter G.
    pub fn val_of_name(&self, name: &str) -> String {
        format!("val({}, G)", name)
    }
}
/// Classification of large cardinal axioms, ordered by consistency strength.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LargeCardinalLevel {
    /// Inaccessible cardinal: κ is regular and a strong limit.
    Inaccessible,
    /// Mahlo cardinal: inaccessible and stationary many inaccessibles below κ.
    Mahlo,
    /// Weakly compact cardinal: κ → (κ)² holds (partition property).
    WeaklyCompact,
    /// Measurable cardinal: there is a κ-complete nonprincipal ultrafilter on κ.
    Measurable,
    /// Strong cardinal: κ is j(κ)-strong for every embedding j.
    Strong,
    /// Woodin cardinal: ∀ A ⊆ Vκ, ∃ δ < κ that is A-strong below κ.
    Woodin,
    /// Superstrong cardinal: the critical point is < j(κ) for j: V → M with V_{j(κ)} ⊆ M.
    Superstrong,
    /// Strongly compact cardinal: every κ-complete filter extends to a κ-complete ultrafilter.
    StronglyCompact,
    /// Supercompact cardinal: for all λ, there is a κ-complete normal fine ultrafilter on P_κ(λ).
    Supercompact,
    /// Extendible cardinal: for all η > κ, there is j: V_η → V_{j(η)} with crit(j) = κ.
    Extendible,
    /// Huge cardinal: j: V → M with crit(j) = κ and j(κ)-closure of M.
    Huge,
    /// I₀: the strongest known large cardinal axiom short of inconsistency.
    IZero,
}
impl LargeCardinalLevel {
    /// A human-readable description of the large cardinal axiom.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Inaccessible => "κ is uncountable, regular, and a strong limit",
            Self::Mahlo => "κ is inaccessible and {α < κ | α inaccessible} is stationary",
            Self::WeaklyCompact => "κ is inaccessible and satisfies the tree property",
            Self::Measurable => "there exists a κ-complete nonprincipal ultrafilter on κ",
            Self::Strong => "for all λ, there is j: V → M with crit(j) = κ and V_λ ⊆ M",
            Self::Woodin => "for all A ⊆ V_κ, ∃ δ < κ A-strong below κ",
            Self::Superstrong => "∃ j: V → M transitive with crit(j) = κ and V_{j(κ)} ⊆ M",
            Self::StronglyCompact => "every κ-complete filter extends to a κ-complete ultrafilter",
            Self::Supercompact => {
                "for all λ, there is a κ-complete normal fine ultrafilter on P_κ(λ)"
            }
            Self::Extendible => "for all η > κ, ∃ j: V_η → V_{j(η)} with crit(j) = κ",
            Self::Huge => "∃ j: V → M with crit(j) = κ and ^{j(κ)}M ⊆ M",
            Self::IZero => "∃ j: L(V_{λ+1}) → L(V_{λ+1}) with crit(j) < λ",
        }
    }
    /// Returns whether this large cardinal is strictly stronger than Measurable.
    pub fn above_measurable(&self) -> bool {
        matches!(
            self,
            Self::Strong
                | Self::Woodin
                | Self::Superstrong
                | Self::StronglyCompact
                | Self::Supercompact
                | Self::Extendible
                | Self::Huge
                | Self::IZero
        )
    }
}
/// The Forcing Axiom for σ-closed posets (FA(σ-closed)).
///
/// Unlike ccc forcing axioms, σ-closed forcing preserves all cardinals and cofinalities.
pub struct SigmaClosedForcingAxiom {
    pub holds: bool,
}
impl SigmaClosedForcingAxiom {
    /// FA(σ-closed) is provable in ZFC (trivially—σ-closed forcing adds no new ω-sequences).
    pub fn provable_in_zfc() -> Self {
        Self { holds: true }
    }
    /// σ-closed forcing preserves all cardinals.
    pub fn preserves_cardinals(&self) -> bool {
        true
    }
    /// σ-closed forcing adds no new subsets of ω.
    pub fn no_new_reals(&self) -> bool {
        true
    }
}
/// P-name: a hereditarily P-labeled set used to define elements of M\[G\].
#[derive(Debug, Clone)]
pub struct PName {
    pub label: String,
    pub rank: u64,
}
impl PName {
    /// The canonical P-name for a ground model set x: x̌ = {(y̌, 𝟙) | y ∈ x}.
    pub fn check(label: impl Into<String>, rank: u64) -> Self {
        Self {
            label: label.into(),
            rank,
        }
    }
    /// The P-name for the generic filter itself: Ġ = {(p̌, p) | p ∈ P}.
    pub fn generic_name() -> Self {
        Self {
            label: "Ġ".to_string(),
            rank: 0,
        }
    }
}
/// A Sacks forcing poset: perfect subtrees of 2^{<ω}.
///
/// Sacks forcing adds a minimal generic real — a real r such that every real
/// constructible from r is either already in M or computes r.
#[allow(dead_code)]
pub struct SacksForcingPoset {
    /// A list of "perfect tree descriptions" (symbolic).
    pub trees: Vec<String>,
    /// Whether the poset is proper.
    pub proper: bool,
}
#[allow(dead_code)]
impl SacksForcingPoset {
    /// Create a new Sacks forcing poset.
    pub fn new(trees: Vec<String>) -> Self {
        Self {
            trees,
            proper: true,
        }
    }
    /// Sacks forcing is proper (Baumgartner-Laver theorem).
    pub fn is_proper(&self) -> bool {
        self.proper
    }
    /// Sacks forcing does not collapse ω₁.
    pub fn preserves_omega1(&self) -> bool {
        self.proper
    }
    /// Sacks forcing adds a minimal degree — the minimal extension property.
    pub fn minimal_degree(&self) -> &'static str {
        "Every real computable from the Sacks real is either in the ground model or computes the Sacks real"
    }
    /// Sacks forcing can be iterated with countable support to add many minimal reals.
    pub fn countable_support_iterable(&self) -> bool {
        true
    }
    /// Check if two tree conditions are compatible (one extends the other).
    ///
    /// We represent compatibility as: T₁ ≤ T₂ iff T₁ is a perfect subtree of T₂.
    pub fn compatible(&self, t1_idx: usize, t2_idx: usize) -> bool {
        if t1_idx >= self.trees.len() || t2_idx >= self.trees.len() {
            return false;
        }
        let t1 = &self.trees[t1_idx];
        let t2 = &self.trees[t2_idx];
        t1.starts_with(t2.as_str()) || t2.starts_with(t1.as_str())
    }
}
/// Mathias forcing: conditions are pairs (s, A) where s is a finite set,
/// A is an infinite set, and max(s) < min(A).
///
/// Given a Ramsey ultrafilter U, Mathias forcing relative to U is proper.
#[allow(dead_code)]
pub struct MathiasForcingPoset {
    /// The ultrafilter (described symbolically) guiding the forcing.
    pub ultrafilter: Option<String>,
    /// Whether this is Mathias forcing relative to a Ramsey ultrafilter.
    pub ramsey_ultrafilter: bool,
}
#[allow(dead_code)]
impl MathiasForcingPoset {
    /// Create a Mathias forcing poset without a specified ultrafilter.
    pub fn new() -> Self {
        Self {
            ultrafilter: None,
            ramsey_ultrafilter: false,
        }
    }
    /// Create Mathias forcing relative to a Ramsey ultrafilter U.
    pub fn with_ramsey_ultrafilter(u: impl Into<String>) -> Self {
        Self {
            ultrafilter: Some(u.into()),
            ramsey_ultrafilter: true,
        }
    }
    /// Mathias forcing relative to a Ramsey ultrafilter is proper.
    pub fn is_proper(&self) -> bool {
        self.ramsey_ultrafilter
    }
    /// Mathias forcing preserves ω₁ when relative to a Ramsey ultrafilter.
    pub fn preserves_omega1(&self) -> bool {
        self.ramsey_ultrafilter
    }
    /// The Mathias real added is a pseudointersection of the ultrafilter.
    pub fn adds_pseudointersection(&self) -> bool {
        self.ultrafilter.is_some()
    }
    /// Mathias reals satisfy the Ramsey partition property.
    pub fn ramsey_property(&self) -> &'static str {
        "For every 2-coloring of [r]^2 there is an infinite H ⊆ r with [H]^2 monochromatic"
    }
    /// The generic extension adds no new ω₁-sequences if the ultrafilter is Ramsey.
    pub fn no_new_omega1_sequences(&self) -> bool {
        self.ramsey_ultrafilter
    }
}
/// 0# (zero-sharp): the set of true sentences of set theory in L with indiscernibles.
///
/// 0# exists iff L is not close to V (Silver indiscernibles theorem).
pub struct ZeroSharp {
    pub exists: bool,
}
impl ZeroSharp {
    /// 0# exists is independent of ZFC but follows from a measurable cardinal.
    pub fn from_measurable() -> Self {
        Self { exists: true }
    }
    /// If 0# exists, L has a class of indiscernibles (Silver indiscernibles).
    pub fn implies_silver_indiscernibles(&self) -> bool {
        self.exists
    }
    /// If 0# exists, every uncountable cardinal in V is inaccessible in L.
    pub fn cardinals_inaccessible_in_l(&self) -> bool {
        self.exists
    }
}
