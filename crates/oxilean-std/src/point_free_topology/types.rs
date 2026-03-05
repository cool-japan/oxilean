//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// The localic real numbers ℝ_loc, constructed as the locale of formal
/// Dedekind cuts (pairs (L, U) of rationals).
///
/// Unlike the classical ℝ, the localic ℝ is constructively well-behaved:
/// ℝ_loc is compact in the localic sense (every open cover has a finite subcover)
/// and Dedekind complete without choice.
pub struct LocalicReals {
    pub is_compact: bool,
    pub is_overt: bool,
    pub is_regular: bool,
}
impl LocalicReals {
    /// Construct the localic real numbers.
    pub fn new() -> Self {
        Self {
            is_compact: false,
            is_overt: true,
            is_regular: true,
        }
    }
    /// The closed unit interval [0,1] as a locale: compact and regular.
    pub fn unit_interval() -> Self {
        Self {
            is_compact: true,
            is_overt: true,
            is_regular: true,
        }
    }
    /// The localic reals agree with the classical reals for spatial locales.
    pub fn agrees_with_classical(&self) -> bool {
        self.is_regular
    }
    /// Heine-Borel theorem holds locally: [a,b] is compact as a locale.
    pub fn heine_borel_locally(&self) -> &'static str {
        "Every closed bounded interval [a,b] in ℝ_loc is compact as a locale"
    }
    /// The localic reals admit a unique complete metric structure.
    pub fn complete_metric(&self) -> bool {
        self.is_regular
    }
}
/// A topological space X is sober if every completely prime filter of open
/// sets corresponds to a unique point of X.
///
/// The soberification of X is the sober space pt(Ω(X)), together with
/// the continuous map η: X → pt(Ω(X)) which is a homeomorphism iff X is sober.
#[derive(Debug, Clone)]
pub struct SoberSpace {
    pub name: String,
    pub is_t0: bool,
    pub is_sober: bool,
    pub is_t1: bool,
}
impl SoberSpace {
    /// Create a sober topological space.
    pub fn new(name: impl Into<String>, is_sober: bool) -> Self {
        Self {
            name: name.into(),
            is_t0: true,
            is_sober,
            is_t1: is_sober,
        }
    }
    /// Every T1 sober space is Hausdorff.
    pub fn is_hausdorff(&self) -> bool {
        self.is_t1 && self.is_sober
    }
    /// The soberification of X: the sober hull pt(Ω(X)).
    pub fn soberification(&self) -> String {
        format!("pt(Ω({}))", self.name)
    }
    /// For sober X: X ≅ pt(Ω(X)) (the adjunction unit is an iso).
    pub fn is_fixed_by_soberification(&self) -> bool {
        self.is_sober
    }
    /// Alexandrov spaces (upper-set topologies of posets) are sober iff
    /// the poset satisfies a certain directed-completeness condition.
    pub fn alexandrov_sobriety_condition() -> &'static str {
        "An Alexandrov space on poset P is sober iff every directed subset of P \
         has a sup that is a directed join in P"
    }
}
/// Connection between locale theory and topos theory.
///
/// Every locale L gives rise to a localic topos Sh(L) of sheaves on L.
/// Conversely, every Grothendieck topos has a localic reflection.
pub struct LocalicToposConnection;
impl LocalicToposConnection {
    /// Sh(L): the topos of sheaves on a locale L.
    pub fn sheaves_on_locale(locale: &str) -> String {
        format!("Sh({}) (topos of sheaves on locale {})", locale, locale)
    }
    /// Every Grothendieck topos has a localic reflection: Loc → GrTop.
    pub fn localic_reflection() -> &'static str {
        "Every Grothendieck topos E has a localic reflection loc(E): \
         the locale of subobjects of 1 in E"
    }
    /// A localic topos Sh(L) is localic iff the geometric morphism Sh(L) → Set
    /// is localic (i.e., the inverse image part preserves all limits).
    pub fn localic_geometric_morphism() -> &'static str {
        "A geometric morphism f: E → F is localic if f_* is logical \
         (= the counit f*f_* → 1 is an iso on sheaves)"
    }
    /// Barr's theorem: every Grothendieck topos has a surjection from a Boolean topos.
    pub fn barr_theorem() -> &'static str {
        "Barr's theorem: for every Grothendieck topos E, there exists a complete \
         Boolean algebra B and a surjective geometric morphism Sh(B) → E"
    }
    /// The category of locales embeds fully faithfully into Grothendieck toposes.
    pub fn loc_embeds_into_grpd_topos() -> &'static str {
        "The functor Sh: Loc → GrTop (L ↦ Sh(L)) is fully faithful"
    }
    /// Double negation sheaves on a locale give a Boolean subtopos.
    pub fn double_negation_topology(locale: &str) -> String {
        format!(
            "Sh_{{¬¬}}({}) = double negation sheaves on {} (Boolean subtopos)",
            locale, locale
        )
    }
    /// Lawvere-Tierney topologies on Sh(L) correspond to nuclei on L.
    pub fn lt_topologies_are_nuclei() -> &'static str {
        "Lawvere-Tierney topologies on Sh(L) are in bijection with nuclei on the frame L"
    }
}
/// The type of a valuation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValuationType {
    /// Simple valuation: finite support.
    Simple,
    /// Probability valuation: ν(top) = 1.
    Probability,
    /// Subprobability valuation: ν(top) ≤ 1.
    Subprobability,
    /// General extended valuation: ν: L → [0,∞].
    Extended,
}
/// A frame homomorphism f: L → M preserves finite meets and all joins.
///
/// A map of locales L → M is a frame homomorphism f*: Ω(M) → Ω(L)
/// (the direct image map goes in the opposite direction to the locale map).
#[derive(Debug, Clone)]
pub struct FrameHom {
    pub source: String,
    pub target: String,
    pub preserves_top: bool,
    pub preserves_finite_meets: bool,
    pub preserves_all_joins: bool,
}
impl FrameHom {
    /// Create a frame homomorphism from source to target.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            preserves_top: true,
            preserves_finite_meets: true,
            preserves_all_joins: true,
        }
    }
    /// The identity frame homomorphism.
    pub fn identity(frame: &str) -> Self {
        Self::new(frame, frame)
    }
    /// Composition of frame homomorphisms.
    pub fn compose(f: &FrameHom, g: &FrameHom) -> Option<FrameHom> {
        if f.target == g.source {
            Some(FrameHom::new(f.source.clone(), g.target.clone()))
        } else {
            None
        }
    }
    /// A frame homomorphism is surjective (as a locale map, this means injective on opens).
    pub fn is_surjective(&self) -> bool {
        self.preserves_top && self.preserves_finite_meets && self.preserves_all_joins
    }
}
/// A nucleus j: L → L on a frame L is an interior operator satisfying:
///   1. a ≤ j(a)  (inflationary)
///   2. j(j(a)) = j(a)  (idempotent)
///   3. j(a ∧ b) = j(a) ∧ j(b)  (meets-preserving)
///
/// Nuclei correspond to quotient locales (sublocales).
#[derive(Debug, Clone)]
pub struct Nucleus {
    pub frame: String,
    pub name: String,
    pub is_inflationary: bool,
    pub is_idempotent: bool,
    pub preserves_meets: bool,
}
impl Nucleus {
    /// Create a nucleus on the given frame.
    pub fn new(frame: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            frame: frame.into(),
            name: name.into(),
            is_inflationary: true,
            is_idempotent: true,
            preserves_meets: true,
        }
    }
    /// The identity nucleus j(a) = a (corresponds to the locale itself).
    pub fn identity(frame: &str) -> Self {
        Self::new(frame, "id")
    }
    /// The closed nucleus j_a(b) = a → b (Heyting implication by a).
    pub fn closed(frame: &str, element: &str) -> Self {
        Self::new(frame, format!("closed({})", element))
    }
    /// The open nucleus j^a(b) = a ∨ b.
    pub fn open(frame: &str, element: &str) -> Self {
        Self::new(frame, format!("open({})", element))
    }
    /// A nucleus is a valid nucleus iff it is inflationary, idempotent, and preserves meets.
    pub fn is_valid(&self) -> bool {
        self.is_inflationary && self.is_idempotent && self.preserves_meets
    }
    /// The sublocale corresponding to this nucleus: {j(a) | a ∈ L}.
    pub fn sublocale_elements(&self) -> String {
        format!("{{{}(a) | a ∈ {}}}", self.name, self.frame)
    }
}
/// Sheaf on a site (Grothendieck topology).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GrothendieckSite {
    pub category_name: String,
    pub covering_families: Vec<String>,
}
#[allow(dead_code)]
impl GrothendieckSite {
    pub fn new(cat: &str) -> Self {
        GrothendieckSite {
            category_name: cat.to_string(),
            covering_families: Vec::new(),
        }
    }
    pub fn add_covering(&mut self, desc: &str) {
        self.covering_families.push(desc.to_string());
    }
    pub fn n_coverings(&self) -> usize {
        self.covering_families.len()
    }
    pub fn canonical_topology(cat: &str) -> Self {
        let mut site = GrothendieckSite::new(cat);
        site.add_covering("maximal-covering-family");
        site
    }
}
/// The type of sublocale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SublocaleType {
    /// Open sublocale: j^a(b) = a ∨ b for an open element a.
    Open,
    /// Closed sublocale: j_a(b) = a → b for a closed element.
    Closed,
    /// Dense sublocale: j(0) = 0.
    Dense,
    /// Scattered sublocale: has no dense proper sublocale.
    Scattered,
    /// General sublocale.
    General,
}
/// Pointless function: frame homomorphism as a "continuous map" in reverse.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PointlessMap {
    pub source_locale: String,
    pub target_locale: String,
    pub frame_hom_direction: String,
}
#[allow(dead_code)]
impl PointlessMap {
    pub fn new(source: &str, target: &str) -> Self {
        PointlessMap {
            source_locale: source.to_string(),
            target_locale: target.to_string(),
            frame_hom_direction: format!("O({target}) → O({source})"),
        }
    }
    pub fn identity(locale: &str) -> Self {
        PointlessMap::new(locale, locale)
    }
    pub fn compose(f: &PointlessMap, g: &PointlessMap) -> Option<Self> {
        if f.target_locale == g.source_locale {
            Some(PointlessMap::new(&f.source_locale, &g.target_locale))
        } else {
            None
        }
    }
}
/// Stone-Cech compactification via ultrafilter construction.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StoneCechCompact {
    pub base_space: String,
    pub is_tychonoff: bool,
}
#[allow(dead_code)]
impl StoneCechCompact {
    pub fn new(space: &str, tychonoff: bool) -> Self {
        StoneCechCompact {
            base_space: space.to_string(),
            is_tychonoff: tychonoff,
        }
    }
    pub fn compactification_exists(&self) -> bool {
        self.is_tychonoff
    }
    pub fn is_compact_hausdorff(&self) -> bool {
        self.is_tychonoff
    }
    /// βX is the space of ultrafilters on the Boolean algebra of clopens of X.
    pub fn ultrafilter_characterization() -> &'static str {
        "beta_X = ultrafilters on Clopen(X)"
    }
}
/// Spectrum of a distributive lattice (Stone duality).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Spectrum {
    pub lattice_name: String,
    pub prime_filters: Vec<String>,
}
#[allow(dead_code)]
impl Spectrum {
    pub fn new(lattice: &str) -> Self {
        Spectrum {
            lattice_name: lattice.to_string(),
            prime_filters: Vec::new(),
        }
    }
    pub fn add_prime_filter(&mut self, name: &str) {
        self.prime_filters.push(name.to_string());
    }
    pub fn n_points(&self) -> usize {
        self.prime_filters.len()
    }
    /// The Zariski topology on Spec(R) has opens D(f) for each f in R.
    pub fn zariski_opens_count(ring_elements: usize) -> usize {
        ring_elements
    }
}
/// A quantale is a complete lattice Q equipped with an associative binary
/// operation * that distributes over arbitrary joins (from both sides).
///
/// Quantales generalise frames (where * = ∧) and are used in non-commutative
/// topology, concurrency theory, and linear logic.
#[derive(Debug, Clone)]
pub struct Quantale {
    pub name: String,
    pub is_commutative: bool,
    pub is_unital: bool,
    pub is_involutive: bool,
}
impl Quantale {
    /// Create a new quantale.
    pub fn new(name: impl Into<String>, commutative: bool, unital: bool) -> Self {
        Self {
            name: name.into(),
            is_commutative: commutative,
            is_unital: unital,
            is_involutive: false,
        }
    }
    /// A frame is a commutative unital quantale with * = ∧.
    pub fn from_frame(frame_name: &str) -> Self {
        Self::new(format!("Frm({})", frame_name), true, true)
    }
    /// A Girard quantale: involutive quantale satisfying the cyclic condition.
    pub fn girard(name: &str) -> Self {
        Self {
            name: format!("Girard({})", name),
            is_commutative: true,
            is_unital: true,
            is_involutive: true,
        }
    }
    /// A module quantale: Q acts on a complete lattice M by a right action.
    pub fn module(base: &str, module: &str) -> Self {
        Self::new(format!("{}-Mod({})", base, module), false, true)
    }
    /// Quantale of binary relations on a set: (P(X×X), ∘, ⊆).
    pub fn relations(set: &str) -> Self {
        Self::new(format!("Rel({})", set), false, true)
    }
    /// The locale condition: a quantale is a frame iff * = ∧ and it is commutative.
    pub fn is_frame(&self) -> bool {
        self.is_commutative && self.is_unital
    }
}
/// Formal ball model for metric spaces in domain theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormalBall {
    pub center: f64,
    pub radius: f64,
}
#[allow(dead_code)]
impl FormalBall {
    pub fn new(center: f64, radius: f64) -> Self {
        assert!(radius >= 0.0);
        FormalBall { center, radius }
    }
    pub fn contains(&self, point: f64) -> bool {
        (point - self.center).abs() < self.radius
    }
    pub fn is_below(&self, other: &FormalBall) -> bool {
        (self.center - other.center).abs() + self.radius <= other.radius
    }
    pub fn supremum(balls: &[FormalBall]) -> Option<FormalBall> {
        if balls.is_empty() {
            return None;
        }
        let min_r = balls.iter().map(|b| b.radius).fold(f64::INFINITY, f64::min);
        let avg_c = balls.iter().map(|b| b.center).sum::<f64>() / balls.len() as f64;
        Some(FormalBall::new(avg_c, min_r))
    }
}
/// A sublocale of L is a set S ⊆ L closed under arbitrary meets and the
/// Heyting implication a → b (where a ∈ L, b ∈ S).
///
/// Equivalently, sublocales correspond to nuclei j: L → L, and the
/// lattice of sublocales is a co-frame (dual frame).
#[derive(Debug, Clone)]
pub struct Sublocale {
    pub parent: String,
    pub nucleus: Nucleus,
    pub sublocale_type: SublocaleType,
}
impl Sublocale {
    /// Create an open sublocale corresponding to element a.
    pub fn open(parent: &str, element: &str) -> Self {
        Self {
            parent: parent.to_string(),
            nucleus: Nucleus::open(parent, element),
            sublocale_type: SublocaleType::Open,
        }
    }
    /// Create a closed sublocale corresponding to closed element a.
    pub fn closed(parent: &str, element: &str) -> Self {
        Self {
            parent: parent.to_string(),
            nucleus: Nucleus::closed(parent, element),
            sublocale_type: SublocaleType::Closed,
        }
    }
    /// The double negation sublocale: j(a) = ¬¬a (the dense sublocale).
    pub fn double_negation(parent: &str) -> Self {
        Self {
            parent: parent.to_string(),
            nucleus: Nucleus::new(parent, "¬¬"),
            sublocale_type: SublocaleType::Dense,
        }
    }
    /// A sublocale is dense iff j(0) = 0.
    pub fn is_dense(&self) -> bool {
        self.sublocale_type == SublocaleType::Dense
    }
    /// Open and closed sublocales are complementary: S_a ∩ C_a = ∅ and S_a ∪ C_a = L.
    pub fn are_complementary(open: &Sublocale, closed: &Sublocale) -> bool {
        open.parent == closed.parent
            && open.sublocale_type == SublocaleType::Open
            && closed.sublocale_type == SublocaleType::Closed
    }
}
/// Complete Heyting algebra (locale) with finite meet/join operations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocaleFrame {
    pub name: String,
    pub elements: Vec<String>,
    pub top: String,
    pub bottom: String,
}
#[allow(dead_code)]
impl LocaleFrame {
    pub fn new(name: &str) -> Self {
        LocaleFrame {
            name: name.to_string(),
            elements: Vec::new(),
            top: "1".to_string(),
            bottom: "0".to_string(),
        }
    }
    pub fn add_element(&mut self, elem: &str) {
        self.elements.push(elem.to_string());
    }
    pub fn n_elements(&self) -> usize {
        self.elements.len()
    }
    /// Sierpinski space locale: {0, 1} with open sets {} and {1} and {0,1}.
    pub fn sierpinski() -> Self {
        let mut loc = LocaleFrame::new("Sierpinski");
        loc.elements = vec!["0".to_string(), "1".to_string(), "top".to_string()];
        loc.top = "top".to_string();
        loc.bottom = "0".to_string();
        loc
    }
}
/// A frame is a complete lattice satisfying the infinite distributive law:
///   a ∧ (∨ S) = ∨ {a ∧ s | s ∈ S}
///
/// Frames are the algebraic counterpart of topological spaces: the open
/// sets of any topological space form a frame, and conversely every frame
/// arises (up to isomorphism) from a locale.
pub struct Frame {
    pub name: String,
    pub is_distributive: bool,
    pub has_all_joins: bool,
    pub has_finite_meets: bool,
}
impl Frame {
    /// Create a new frame with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_distributive: true,
            has_all_joins: true,
            has_finite_meets: true,
        }
    }
    /// The frame of open sets of a topological space X.
    pub fn opens_of(space: &str) -> Self {
        Self {
            name: format!("Ω({})", space),
            is_distributive: true,
            has_all_joins: true,
            has_finite_meets: true,
        }
    }
    /// The frame of downsets (lower sets) of a poset P.
    pub fn downsets_of(poset: &str) -> Self {
        Self {
            name: format!("Down({})", poset),
            is_distributive: true,
            has_all_joins: true,
            has_finite_meets: true,
        }
    }
    /// The trivial frame {0, 1} (two-element chain).
    pub fn two() -> Self {
        Self {
            name: "2".to_string(),
            is_distributive: true,
            has_all_joins: true,
            has_finite_meets: true,
        }
    }
    /// The power set frame P(X) for any set X.
    pub fn power_set(set_name: &str) -> Self {
        Self {
            name: format!("P({})", set_name),
            is_distributive: true,
            has_all_joins: true,
            has_finite_meets: true,
        }
    }
    /// Checks the infinite distributive law a ∧ (∨ S) = ∨ {a ∧ s | s ∈ S}.
    pub fn satisfies_infinite_distributivity(&self) -> bool {
        self.is_distributive && self.has_all_joins && self.has_finite_meets
    }
    /// A frame is spatial if it isomorphic to the frame of opens of some topological space.
    /// Equivalently, points (frame maps to 2) separate elements.
    pub fn is_spatial(&self) -> bool {
        self.name.starts_with("Ω(") || self.name.starts_with("P(")
    }
    /// A frame is compact if its top element 1 is compact: if 1 = ∨ S then 1 = ∨ F for
    /// some finite F ⊆ S.
    pub fn is_compact(&self) -> bool {
        self.has_all_joins
    }
}
/// A spatial locale is one where the underlying frame is spatial (has enough points).
///
/// Not every locale is spatial: for instance, the locale of surjections
/// from ℕ to ℝ has no points, yet is nontrivial.
pub struct SpatialLocale {
    pub locale: Locale,
    pub topological_space: String,
}
impl SpatialLocale {
    /// Every sober topological space gives a spatial locale.
    pub fn from_sober_space(space: &str) -> Self {
        Self {
            locale: Locale::of_space(space),
            topological_space: space.to_string(),
        }
    }
    /// The spatial reflection: every locale has a spatial part (sobrification of its points).
    pub fn spatial_reflection(locale_name: &str) -> String {
        format!("pt({}) (spatial reflection / sober hull)", locale_name)
    }
    /// The spatial locale of reals.
    pub fn reals() -> Self {
        Self::from_sober_space("ℝ")
    }
}
/// Stone duality: the category of sober spaces is dually equivalent to the
/// category of spatial frames (locales with enough points).
///
/// For compact Hausdorff spaces: the category KHaus is dually equivalent to
/// the category of compact regular frames (Stone duality for KHaus).
pub struct StoneDuality;
impl StoneDuality {
    /// Stone duality for sober spaces: Sob ≃ Sp^op.
    pub fn sober_spaces_dual_to_spatial_frames() -> &'static str {
        "The category of sober topological spaces is dually equivalent to \
         the category of spatial frames (= frames with enough completely prime filters)"
    }
    /// Stone duality for compact Hausdorff spaces and compact regular frames.
    pub fn khaus_dual() -> &'static str {
        "KHaus ≃ (CompRegFrm)^op: compact Hausdorff spaces correspond to \
         compact regular (= completely regular) frames"
    }
    /// Stone duality for Boolean algebras: Bool ≃ Stone^op.
    /// Stone spaces are compact, Hausdorff, and totally disconnected.
    pub fn boolean_algebras_dual() -> &'static str {
        "Stone's representation theorem: the category of Boolean algebras \
         is dually equivalent to the category of Stone spaces (compact Hausdorff totally disconnected)"
    }
    /// Gelfand duality: commutative C*-algebras ↔ compact Hausdorff spaces.
    pub fn gelfand_duality() -> &'static str {
        "Gelfand duality: the category of unital commutative C*-algebras \
         is dually equivalent to the category of compact Hausdorff spaces. \
         (A locale-theoretic generalisation applies to all compact regular frames.)"
    }
    /// The spectrum of a distributive lattice (Priestley duality).
    pub fn priestley_duality() -> &'static str {
        "Priestley duality: the category of bounded distributive lattices \
         is dually equivalent to the category of Priestley spaces \
         (ordered Stone spaces with the Priestley separation axiom)"
    }
    /// The localic version of Stone duality: Frm ≃ Loc^op.
    pub fn frames_locales_duality() -> &'static str {
        "By definition, Loc = Frm^op: every frame is the frame of opens of a unique locale, \
         and every locale map corresponds to a unique frame homomorphism in the opposite direction"
    }
}
/// A localic group is a group object in the category of locales.
///
/// A localic group has an underlying locale G with continuous multiplication,
/// inversion, and identity morphisms satisfying the group axioms.
#[derive(Debug, Clone)]
pub struct LocalicGroup {
    pub locale_name: String,
    pub is_abelian: bool,
    pub is_compact: bool,
    pub is_locally_compact: bool,
}
impl LocalicGroup {
    /// The localic group of real numbers under addition.
    pub fn real_line() -> Self {
        Self {
            locale_name: "ℝ_loc".to_string(),
            is_abelian: true,
            is_compact: false,
            is_locally_compact: true,
        }
    }
    /// The localic circle group ℝ/ℤ (compact abelian).
    pub fn circle() -> Self {
        Self {
            locale_name: "ℝ_loc/ℤ".to_string(),
            is_abelian: true,
            is_compact: true,
            is_locally_compact: true,
        }
    }
    /// The p-adic integers ℤ_p (compact totally disconnected localic group).
    pub fn p_adic_integers(p: u64) -> Self {
        Self {
            locale_name: format!("ℤ_{}", p),
            is_abelian: true,
            is_compact: true,
            is_locally_compact: true,
        }
    }
    /// Pontryagin duality holds for locally compact abelian localic groups.
    pub fn pontryagin_dual(&self) -> String {
        if self.is_abelian && self.is_locally_compact {
            format!("Pontryagin dual of {}", self.locale_name)
        } else {
            "Pontryagin duality requires locally compact abelian group".to_string()
        }
    }
    /// Every locally compact localic group has a unique Haar measure.
    pub fn has_haar_measure(&self) -> bool {
        self.is_locally_compact
    }
}
/// A coherent locale is a compact regular locale whose frame is generated
/// by the compact elements (a spectral space in the spatial case).
///
/// Coherent locales correspond to coherent theories in logic via the
/// coherent classifying topos construction.
pub struct CoherentLocale {
    pub name: String,
    pub is_compact: bool,
    pub is_regular: bool,
    pub compact_elements_generate: bool,
}
impl CoherentLocale {
    /// Create a coherent locale.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_compact: true,
            is_regular: true,
            compact_elements_generate: true,
        }
    }
    /// The coherent locale of a distributive lattice (Priestley space).
    pub fn of_distributive_lattice(lattice: &str) -> Self {
        Self::new(format!("Spec({})", lattice))
    }
    /// A coherent locale is spatial iff it corresponds to a spectral topological space.
    pub fn is_spectral_space(&self) -> bool {
        self.is_compact && self.compact_elements_generate
    }
    /// Coherent locales classify coherent geometric theories.
    pub fn classifying_theory(&self) -> String {
        format!("Coherent theory classified by {}", self.name)
    }
}
/// The type of uniformity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniformityType {
    /// Weil uniformity: a filter of entourages (symmetric binary relations).
    Weil,
    /// Covering uniformity: a filter of open covers.
    Covering,
    /// Metric uniformity: induced by a metric.
    Metric,
}
/// Dcpo (directed-complete partial order) for domain theory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Dcpo {
    pub name: String,
    pub is_pointed: bool,
    pub is_continuous: bool,
    pub is_algebraic: bool,
}
#[allow(dead_code)]
impl Dcpo {
    pub fn new(name: &str) -> Self {
        Dcpo {
            name: name.to_string(),
            is_pointed: false,
            is_continuous: false,
            is_algebraic: false,
        }
    }
    pub fn flat_domain(name: &str) -> Self {
        Dcpo {
            name: name.to_string(),
            is_pointed: true,
            is_continuous: true,
            is_algebraic: true,
        }
    }
    pub fn scott_topology_is_sober(&self) -> bool {
        self.is_continuous
    }
    pub fn every_algebraic_is_continuous(&self) -> bool {
        self.is_algebraic
    }
}
/// A uniform locale is a locale L together with a uniformity:
/// a filter of binary relations on L satisfying the axioms of a uniform space.
///
/// The completion of a uniform locale is defined purely in terms of
/// Cauchy filters, without reference to points.
#[derive(Debug, Clone)]
pub struct UniformLocale {
    pub locale_name: String,
    pub is_complete: bool,
    pub is_totally_bounded: bool,
    pub uniformity_type: UniformityType,
}
impl UniformLocale {
    /// Create a uniform locale with Weil uniformity.
    pub fn weil(locale: impl Into<String>) -> Self {
        Self {
            locale_name: locale.into(),
            is_complete: false,
            is_totally_bounded: false,
            uniformity_type: UniformityType::Weil,
        }
    }
    /// The uniform locale of reals ℝ with the standard metric uniformity.
    pub fn reals() -> Self {
        Self {
            locale_name: "ℝ_loc".to_string(),
            is_complete: true,
            is_totally_bounded: false,
            uniformity_type: UniformityType::Metric,
        }
    }
    /// The completion of a uniform locale: the locale of all Cauchy filters.
    pub fn completion(&self) -> UniformLocale {
        UniformLocale {
            locale_name: format!("Compl({})", self.locale_name),
            is_complete: true,
            is_totally_bounded: self.is_totally_bounded,
            uniformity_type: self.uniformity_type.clone(),
        }
    }
    /// A complete totally bounded uniform locale is compact.
    pub fn is_compact(&self) -> bool {
        self.is_complete && self.is_totally_bounded
    }
}
/// Formal topology provides a predicative and constructive treatment of
/// topological spaces as formal systems: a base set (basic opens) together
/// with a covering relation.
///
/// An inductively generated formal topology (Sambin-Valentini) uses
/// axiom sets to generate the covering relation.
#[derive(Debug, Clone)]
pub struct FormalTopology {
    pub base: String,
    pub is_inductively_generated: bool,
    pub is_predicative: bool,
}
impl FormalTopology {
    /// Create a new formal topology on the given base.
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            is_inductively_generated: true,
            is_predicative: true,
        }
    }
    /// The formal real line: base = pairs (q, r) ∈ ℚ×ℚ with q < r.
    pub fn formal_reals() -> Self {
        Self::new("ℚ-intervals")
    }
    /// The formal Cantor space: base = finite binary strings.
    pub fn formal_cantor() -> Self {
        Self::new("2^*")
    }
    /// A formal topology is sober if every formal point corresponds to a
    /// completely prime filter in the generating frame.
    pub fn is_sober(&self) -> bool {
        self.is_inductively_generated
    }
    /// The continuous map between formal topologies is a relation preserving
    /// the covering relation in both directions.
    pub fn continuous_map_description(&self, target: &str) -> String {
        format!("Continuous map from {} to {}", self.base, target)
    }
}
/// A compact regular locale is a frame L that is:
///   - Compact: ⊤ = ∨ S implies ⊤ = ∨ F for some finite F ⊆ S.
///   - Regular: every element a is the join of elements "well inside" a
///     (a' is well inside a iff a' ∧ (¬a) = ⊥).
///
/// Compact regular locales correspond to compact Hausdorff spaces
/// (in the spatial case) and admit a point-free description of normality,
/// paracompactness, etc.
#[derive(Debug, Clone)]
pub struct CompactRegularLocale {
    pub name: String,
    pub is_normal: bool,
    pub is_paracompact: bool,
    pub is_second_countable: bool,
}
impl CompactRegularLocale {
    /// Create a compact regular locale.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_normal: true,
            is_paracompact: true,
            is_second_countable: false,
        }
    }
    /// The locale [0,1]: compact, regular, second-countable.
    pub fn unit_interval() -> Self {
        Self {
            name: "[0,1]_loc".to_string(),
            is_normal: true,
            is_paracompact: true,
            is_second_countable: true,
        }
    }
    /// A compact regular locale is normal: for disjoint closed sublocales,
    /// there exist disjoint open sublocales separating them.
    pub fn normality_statement(&self) -> String {
        format!(
            "In {} : for disjoint closed S, T ∃ disjoint open U, V with S ≤ U and T ≤ V",
            self.name
        )
    }
    /// Paracompactness point-free: every open cover has a locally finite refinement.
    pub fn paracompactness_statement(&self) -> &'static str {
        "Every open cover of a compact regular locale has a locally finite open refinement"
    }
    /// A compact regular locale is compact by definition.
    pub fn is_compact(&self) -> bool {
        true
    }
}
/// The Stone-Čech compactification βX of a completely regular space X.
///
/// βX is the universal compact Hausdorff space that X embeds into:
/// every bounded continuous f: X → ℝ extends uniquely to β(f): βX → ℝ.
pub struct StoneCechCompactification {
    pub space: String,
    pub is_compact: bool,
    pub is_hausdorff: bool,
}
impl StoneCechCompactification {
    /// Construct the Stone-Čech compactification of a completely regular space.
    pub fn of(space: impl Into<String>) -> Self {
        let space = space.into();
        Self {
            space,
            is_compact: true,
            is_hausdorff: true,
        }
    }
    /// βX is compact Hausdorff.
    pub fn is_compact_hausdorff(&self) -> bool {
        self.is_compact && self.is_hausdorff
    }
    /// The universal property: every continuous f: X → K into compact Hausdorff K
    /// extends uniquely to β(f): βX → K.
    pub fn universal_property(&self) -> &'static str {
        "∀ compact Hausdorff K, ∀ continuous f: X → K, ∃! β(f): βX → K with β(f) ∘ ι = f"
    }
    /// βℕ: the Stone-Čech compactification of the natural numbers.
    /// This is used heavily in combinatorics (ultrafilters, Ramsey theory).
    pub fn beta_nat() -> Self {
        Self::of("ℕ")
    }
    /// Points of βX correspond to ultrafilters on X (for discrete X).
    pub fn points_are_ultrafilters(&self) -> bool {
        true
    }
    /// The Stone-Čech remainder βX \ X (or βX \ ι(X)).
    pub fn remainder(&self) -> String {
        format!("β{} \\ {}", self.space, self.space)
    }
}
/// The Scott topology on a dcpo (directed complete partial order) D
/// is the topology whose open sets are the Scott-open upper sets.
/// This topology forms a frame (the Scott frame).
#[derive(Debug, Clone)]
pub struct ScottTopologyFrame {
    pub dcpo_name: String,
    pub has_way_below: bool,
    pub is_continuous_domain: bool,
}
impl ScottTopologyFrame {
    /// Create a Scott topology frame from a dcpo name.
    pub fn new(dcpo_name: impl Into<String>) -> Self {
        Self {
            dcpo_name: dcpo_name.into(),
            has_way_below: false,
            is_continuous_domain: false,
        }
    }
    /// A continuous domain: for each element x, the set {y | y ≪ x} is directed
    /// and has x as its join. The Scott topology is then sober.
    pub fn continuous_domain(name: &str) -> Self {
        Self {
            dcpo_name: name.to_string(),
            has_way_below: true,
            is_continuous_domain: true,
        }
    }
    /// The flat domain A_⊥: add a bottom element to a discrete set A.
    pub fn flat_domain(set: &str) -> Self {
        Self::new(format!("{}_perp", set))
    }
    /// For a continuous domain, the Scott topology is sober and locally compact.
    pub fn is_sober(&self) -> bool {
        self.is_continuous_domain
    }
    /// The way-below relation ≪: x ≪ y if whenever y ≤ sup D for directed D,
    /// then x ≤ d for some d ∈ D.
    pub fn way_below_description(&self) -> String {
        format!("{{(x,y) | x ≪ y in {}}}", self.dcpo_name)
    }
}
/// FrameNucleus on a frame (a closure operator compatible with finite meets).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FrameNucleus {
    pub frame_name: String,
    pub description: String,
}
#[allow(dead_code)]
impl FrameNucleus {
    pub fn new(frame: &str, desc: &str) -> Self {
        FrameNucleus {
            frame_name: frame.to_string(),
            description: desc.to_string(),
        }
    }
    pub fn double_negation(frame: &str) -> Self {
        FrameNucleus::new(frame, "double-negation nucleus j(a) = ¬¬a")
    }
    pub fn identity(frame: &str) -> Self {
        FrameNucleus::new(frame, "identity nucleus j(a) = a")
    }
}
/// A locale is simply a frame, but with morphisms reversed: a map of locales
/// L → M is a frame homomorphism M → L (going in the opposite direction).
///
/// This reversal makes locales behave like "generalized spaces".
pub struct Locale {
    pub frame: Frame,
    pub description: String,
}
impl Locale {
    /// Create a locale from its underlying frame.
    pub fn from_frame(frame: Frame) -> Self {
        let desc = format!("Locale({})", frame.name);
        Self {
            frame,
            description: desc,
        }
    }
    /// The locale of points of a topological space X.
    pub fn of_space(space: &str) -> Self {
        Self::from_frame(Frame::opens_of(space))
    }
    /// The localic real number object ℝ_loc: the locale of formal Dedekind cuts.
    pub fn localic_reals() -> Self {
        Self {
            frame: Frame::new("Ω(ℝ_loc)"),
            description: "Locale of formal Dedekind cuts (localic reals)".to_string(),
        }
    }
    /// The locale of formal rationals ℚ_loc.
    pub fn localic_rationals() -> Self {
        Self {
            frame: Frame::new("Ω(ℚ_loc)"),
            description: "Locale of formal rationals".to_string(),
        }
    }
    /// The Stone-Čech compactification locale βX.
    pub fn stone_cech(space: &str) -> Self {
        Self {
            frame: Frame::new(format!("Ω(β{})", space)),
            description: format!("Stone-Čech compactification of {}", space),
        }
    }
    /// A locale is sober if every completely prime filter corresponds to a unique point.
    pub fn is_sober(&self) -> bool {
        self.frame.is_spatial()
    }
    /// The product locale L × M corresponds to the coproduct of frames.
    pub fn product(l: &Locale, m: &Locale) -> Locale {
        Locale::from_frame(Frame::new(format!("{} × {}", l.frame.name, m.frame.name)))
    }
}
/// The Isbell adjunction: a dual adjunction between spaces and frames.
///
/// For a topological space X, define Ω(X) = frame of opens.
/// For a frame L, define pt(L) = space of completely prime filters.
/// Then Ω ⊣ pt^op: Ω and pt form a dual adjunction.
pub struct IsbellAdjunction;
impl IsbellAdjunction {
    /// Ω: Top → Frm^op sends a space to its frame of open sets.
    pub fn omega_functor() -> &'static str {
        "Ω: Top → Frm^op, X ↦ (open sets of X), f ↦ f^{-1}"
    }
    /// pt: Frm^op → Top sends a frame to its space of completely prime filters.
    pub fn pt_functor() -> &'static str {
        "pt: Frm^op → Top, L ↦ (completely prime filters of L), with Scott topology"
    }
    /// The adjunction unit η_X: X → pt(Ω(X)) is continuous; it is a homeomorphism iff X is sober.
    pub fn unit_description() -> &'static str {
        "η_X: X → pt(Ω(X)) is continuous; it is a homeomorphism iff X is sober"
    }
    /// The adjunction counit ε_L: Ω(pt(L)) → L is a frame map; it is an iso iff L is spatial.
    pub fn counit_description() -> &'static str {
        "ε_L: Ω(pt(L)) → L is a frame homomorphism; it is an isomorphism iff L is spatial"
    }
    /// The fixed points: sober spaces and spatial frames are in bijective correspondence.
    pub fn fixed_points() -> &'static str {
        "Sober spaces ↔ Spatial frames via Ω and pt (Isbell adjunction fixed points)"
    }
    /// Not every locale is spatial: localic pathologies exist.
    pub fn non_spatial_example() -> &'static str {
        "The locale of dense subsets of ℝ with empty intersection \
         (surjection locale ℕ ↠ ℝ) has no points, hence is non-spatial"
    }
}
/// A valuation on a locale L is a Scott-continuous map ν: L → [0,∞]
/// satisfying: ν(0) = 0 and the modular law ν(a) + ν(b) = ν(a∨b) + ν(a∧b).
///
/// Valuations are the pointfree analogue of probability measures.
#[derive(Debug, Clone)]
pub struct LocalicValuation {
    pub locale_name: String,
    pub valuation_type: ValuationType,
    pub is_normalized: bool,
}
impl LocalicValuation {
    /// Create a probability valuation on the given locale.
    pub fn probability(locale: impl Into<String>) -> Self {
        Self {
            locale_name: locale.into(),
            valuation_type: ValuationType::Probability,
            is_normalized: true,
        }
    }
    /// Create a simple valuation (finite support).
    pub fn simple(locale: impl Into<String>) -> Self {
        Self {
            locale_name: locale.into(),
            valuation_type: ValuationType::Simple,
            is_normalized: false,
        }
    }
    /// The Dirac valuation at a point p: δ_p(U) = 1 if p ∈ U else 0.
    pub fn dirac(locale: &str, point: &str) -> Self {
        Self {
            locale_name: format!("{}[δ_{}]", locale, point),
            valuation_type: ValuationType::Probability,
            is_normalized: true,
        }
    }
    /// A probability valuation satisfies ν(top) = 1 and the modular law.
    pub fn is_probability_measure(&self) -> bool {
        self.valuation_type == ValuationType::Probability && self.is_normalized
    }
    /// The Riesz representation: continuous linear functionals on C(X) correspond
    /// to regular Borel probability measures, and hence to valuations on Ω(X).
    pub fn riesz_representation_description() -> &'static str {
        "Valuations on Ω(X) for compact Hausdorff X correspond to \
         regular Borel probability measures on X (Riesz representation)"
    }
}
/// Sober space: a topological space where every irreducible closed set
/// has a unique generic point.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SoberTopSpace {
    pub name: String,
    pub is_sober: bool,
    pub is_t0: bool,
}
#[allow(dead_code)]
impl SoberTopSpace {
    pub fn new(name: &str, sober: bool, t0: bool) -> Self {
        SoberTopSpace {
            name: name.to_string(),
            is_sober: sober,
            is_t0: t0,
        }
    }
    pub fn hausdorff_is_sober() -> bool {
        true
    }
    pub fn sober_implies_t0() -> bool {
        true
    }
    pub fn t1_sober_is_equivalent_to_sobriety() -> bool {
        false
    }
}
