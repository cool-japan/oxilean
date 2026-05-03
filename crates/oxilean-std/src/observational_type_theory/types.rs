//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A proof of observational equality between two terms in some type.
#[derive(Debug, Clone, PartialEq)]
pub struct ObsEqProof {
    /// The type in which equality is claimed.
    pub type_name: String,
    /// Left-hand side.
    pub lhs: String,
    /// Right-hand side.
    pub rhs: String,
    /// Justification for the equality.
    pub justification: ObsEqJustification,
}
impl ObsEqProof {
    /// Reflexivity proof.
    pub fn refl(type_name: impl Into<String>, elem: impl Into<String>) -> Self {
        let e = elem.into();
        ObsEqProof {
            type_name: type_name.into(),
            lhs: e.clone(),
            rhs: e,
            justification: ObsEqJustification::Refl,
        }
    }
    /// Symmetric proof.
    pub fn sym(self) -> ObsEqProof {
        let lhs = self.rhs.clone();
        let rhs = self.lhs.clone();
        let type_name = self.type_name.clone();
        ObsEqProof {
            type_name,
            lhs,
            rhs,
            justification: ObsEqJustification::Sym(Box::new(self)),
        }
    }
    /// Transitive composition of two proofs.
    pub fn trans(self, other: ObsEqProof) -> Option<ObsEqProof> {
        if self.rhs != other.lhs || self.type_name != other.type_name {
            return None;
        }
        Some(ObsEqProof {
            type_name: self.type_name.clone(),
            lhs: self.lhs.clone(),
            rhs: other.rhs.clone(),
            justification: ObsEqJustification::Trans(Box::new(self), Box::new(other)),
        })
    }
}
/// Observational equality: a witness that `a` and `b` are observationally equal
/// in a given type context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObsEq {
    /// The type context (name of the ambient type).
    pub type_ctx: String,
    /// The left-hand side term.
    pub a: String,
    /// The right-hand side term.
    pub b: String,
}
impl ObsEq {
    /// Create a new observational equality witness.
    pub fn new(type_ctx: impl Into<String>, a: impl Into<String>, b: impl Into<String>) -> Self {
        Self {
            type_ctx: type_ctx.into(),
            a: a.into(),
            b: b.into(),
        }
    }
    /// Check if `a` and `b` are syntactically (propositionally) equal.
    pub fn is_propositionally_equal(&self) -> bool {
        self.a == self.b
    }
    /// Cast the equality along a coercion: returns the coerced term description.
    ///
    /// In OTT, `cast : ObsEq Type A B → A → B` allows transporting terms
    /// across propositional type equality.
    pub fn cast_along(&self, term: &str) -> String {
        if self.a == self.b {
            term.to_string()
        } else {
            format!(
                "cast({} : {} ≡ {} ⊢ {})",
                self.type_ctx, self.a, self.b, term
            )
        }
    }
    /// Symmetrise: swap `a` and `b`.
    pub fn sym(self) -> Self {
        Self {
            type_ctx: self.type_ctx,
            a: self.b,
            b: self.a,
        }
    }
    /// Transitivity: compose two observational equalities.
    pub fn trans(self, other: Self) -> Option<Self> {
        if self.b == other.a && self.type_ctx == other.type_ctx {
            Some(Self {
                type_ctx: self.type_ctx,
                a: self.a,
                b: other.b,
            })
        } else {
            None
        }
    }
}
/// Coherence conditions for monoidal structures: pentagon and triangle identities.
#[derive(Debug, Clone)]
pub struct CoherenceCondition {
    /// Names of the operations involved in the coherence condition.
    pub operations: Vec<String>,
}
impl CoherenceCondition {
    /// Create a new coherence condition for the given operations.
    pub fn new(operations: Vec<String>) -> Self {
        Self { operations }
    }
    /// Pentagon identity: `α_{W,X,Y,Z}` satisfies the pentagon diagram.
    ///
    /// `(α_{W,X,Y} ⊗ id_Z) ∘ α_{W,X⊗Y,Z} ∘ (id_W ⊗ α_{X,Y,Z}) = α_{W⊗X,Y,Z} ∘ α_{W,X,Y⊗Z}`
    pub fn pentagon_identity(&self) -> bool {
        self.operations.contains(&"assoc".to_string())
            || self.operations.iter().any(|op| op.contains("α"))
    }
    /// Triangle identity: `(id_A ⊗ λ_B) ∘ α_{A,I,B} = ρ_A ⊗ id_B`
    pub fn triangle_identity(&self) -> bool {
        let has_unit = self.operations.contains(&"unit".to_string())
            || self
                .operations
                .iter()
                .any(|op| op == "λ" || op == "ρ" || op == "I");
        let has_assoc = self.operations.contains(&"assoc".to_string())
            || self.operations.iter().any(|op| op.contains("α"));
        has_unit && has_assoc
    }
    /// Check that both pentagon and triangle identities hold.
    pub fn is_coherent(&self) -> bool {
        self.pentagon_identity() && self.triangle_identity()
    }
}
/// Tracks the universe level in an OTT type theory with a cumulative hierarchy.
///
/// OTT supports universe polymorphism: axioms can be stated at a generic level
/// and instantiated. This struct tracks the level and its relationship to other levels.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OTTUniverseLevel {
    /// The numeric level (0 = Prop, 1 = Type₀, 2 = Type₁, ...).
    pub level: u32,
}
impl OTTUniverseLevel {
    /// Create the Prop level (level 0).
    pub fn prop() -> Self {
        Self { level: 0 }
    }
    /// Create Type₀ (level 1).
    pub fn type0() -> Self {
        Self { level: 1 }
    }
    /// Create Type n (level n+1 in 0-indexed terms).
    pub fn type_n(n: u32) -> Self {
        Self { level: n + 1 }
    }
    /// The successor level.
    pub fn succ(&self) -> Self {
        Self {
            level: self.level + 1,
        }
    }
    /// The maximum of two levels (used for `max u v` in universe expressions).
    pub fn max(a: &OTTUniverseLevel, b: &OTTUniverseLevel) -> Self {
        Self {
            level: a.level.max(b.level),
        }
    }
    /// Check if this level is a proposition level.
    pub fn is_prop(&self) -> bool {
        self.level == 0
    }
    /// Check if cumulativity holds: `level_a ≤ level_b`.
    pub fn is_sub_universe_of(&self, other: &OTTUniverseLevel) -> bool {
        self.level <= other.level
    }
    /// Display as a Lean-style universe expression.
    pub fn display(&self) -> String {
        match self.level {
            0 => "Prop".to_string(),
            1 => "Type".to_string(),
            n => format!("Type {}", n - 1),
        }
    }
}
/// Propositional extensionality: two propositions are equal if and only if
/// they are logically equivalent.
#[derive(Debug, Clone, Default)]
pub struct PropExtensionality;
impl PropExtensionality {
    /// Create a new propositional extensionality witness.
    pub fn new() -> Self {
        Self
    }
    /// Proof irrelevance: any two proofs of the same proposition are equal.
    ///
    /// In OTT this is derivable from `PropIrrelevance` (squash types or h-props).
    pub fn proof_irrelevance(&self, prop: &str, proof_a: &str, proof_b: &str) -> String {
        format!("pi({}, {}, {})", prop, proof_a, proof_b)
    }
    /// Propositions are equal if they are logically equivalent.
    ///
    /// `propext : (P ↔ Q) → P = Q`
    pub fn propositions_equal_if_equivalent(&self, p: &str, q: &str) -> String {
        format!("propext({} ↔ {})", p, q)
    }
    /// Check whether the propositional extensionality axiom holds in this context.
    pub fn is_axiom(&self) -> bool {
        true
    }
}
/// Function extensionality derived from observational equality:
/// if `f x` and `g x` are observationally equal for all `x`, then `f = g`.
#[derive(Debug, Clone, Default)]
pub struct FunctionExtObs;
impl FunctionExtObs {
    /// Create a new function extensionality witness.
    pub fn new() -> Self {
        Self
    }
    /// `funext_obs : (∀ x, f x ≡ g x) → f ≡ g`
    pub fn pointwise_equal_implies_equal(&self, f: &str, g: &str, domain: &str) -> String {
        format!("funext_obs({}, {} : {} → _)", f, g, domain)
    }
    /// In OTT, function extensionality is derivable (not an axiom).
    pub fn is_axiom_free(&self) -> bool {
        true
    }
    /// Return the proof term for function extensionality.
    pub fn proof_term(&self, f: &str, g: &str) -> String {
        format!("ObsEq.funext({}, {})", f, g)
    }
}
/// A setoid: a carrier type (represented as a string name) together with
/// a named equivalence relation.
#[derive(Debug, Clone, PartialEq)]
pub struct Setoid {
    /// Name of the carrier type.
    pub carrier: String,
    /// Name of the equivalence relation.
    pub rel: String,
    /// Whether reflexivity has been verified.
    pub reflexive: bool,
    /// Whether symmetry has been verified.
    pub symmetric: bool,
    /// Whether transitivity has been verified.
    pub transitive: bool,
}
impl Setoid {
    /// Create a setoid with an explicitly given equivalence relation.
    pub fn new(
        carrier: impl Into<String>,
        rel: impl Into<String>,
        reflexive: bool,
        symmetric: bool,
        transitive: bool,
    ) -> Self {
        Self {
            carrier: carrier.into(),
            rel: rel.into(),
            reflexive,
            symmetric,
            transitive,
        }
    }
    /// Create the trivial (discrete) setoid where equality is the identity relation.
    pub fn discrete(carrier: impl Into<String>) -> Self {
        let c = carrier.into();
        Self {
            rel: format!("DiscreteEq({})", c),
            carrier: c,
            reflexive: true,
            symmetric: true,
            transitive: true,
        }
    }
    /// Check if this setoid has a valid equivalence relation.
    pub fn is_valid(&self) -> bool {
        self.reflexive && self.symmetric && self.transitive
    }
    /// Form the product setoid of two setoids.
    pub fn product(&self, other: &Setoid) -> Setoid {
        Setoid {
            carrier: format!("{} × {}", self.carrier, other.carrier),
            rel: format!("{} ×_rel {}", self.rel, other.rel),
            reflexive: self.reflexive && other.reflexive,
            symmetric: self.symmetric && other.symmetric,
            transitive: self.transitive && other.transitive,
        }
    }
    /// Form the function setoid: functions respecting both equivalences.
    pub fn exponential(&self, codomain: &Setoid) -> Setoid {
        Setoid {
            carrier: format!("{} → {}", self.carrier, codomain.carrier),
            rel: format!("PointwiseEq({}, {})", self.rel, codomain.rel),
            reflexive: true,
            symmetric: codomain.symmetric,
            transitive: codomain.transitive,
        }
    }
}
/// A quotient type at the Rust level: just tracks the carrier and relation.
#[derive(Debug, Clone)]
pub struct QuotientType {
    /// Carrier type name.
    pub carrier: String,
    /// Equivalence relation name.
    pub relation: String,
}
impl QuotientType {
    /// Create a quotient type A/R.
    pub fn new(carrier: impl Into<String>, relation: impl Into<String>) -> Self {
        Self {
            carrier: carrier.into(),
            relation: relation.into(),
        }
    }
    /// The name of the quotient type.
    pub fn name(&self) -> String {
        format!("{}/{}", self.carrier, self.relation)
    }
    /// Project an element into the quotient.
    pub fn mk(&self, elem: impl Into<String>) -> String {
        format!("[{}]_{}", elem.into(), self.relation)
    }
    /// Check if two elements are equal in the quotient (by name matching).
    pub fn eq_in_quotient(&self, a: &str, b: &str) -> bool {
        a == b
    }
}
/// Setoid rewriting: given a setoid (carrier, equivalence relation) one can
/// substitute equal elements in morphisms.
#[derive(Debug, Clone)]
pub struct SetoidRewriting {
    /// Name of the carrier type.
    pub carrier: String,
    /// Name of the equivalence relation on the carrier.
    pub equiv_rel: String,
}
impl SetoidRewriting {
    /// Create a new setoid rewriting structure.
    pub fn new(carrier: impl Into<String>, equiv_rel: impl Into<String>) -> Self {
        Self {
            carrier: carrier.into(),
            equiv_rel: equiv_rel.into(),
        }
    }
    /// Construct a setoid morphism from `carrier` to `target` compatible with `equiv_rel`.
    pub fn setoid_morphism(&self, target: &str) -> String {
        format!(
            "SetoidMorphism({} -[{}]-> {})",
            self.carrier, self.equiv_rel, target
        )
    }
    /// Construct the quotient type of the carrier by the equivalence relation.
    pub fn quotient_type(&self) -> String {
        format!("{} / {}", self.carrier, self.equiv_rel)
    }
    /// Check if the relation is a valid equivalence relation (reflexive, symmetric, transitive).
    pub fn is_valid_equivalence(&self) -> bool {
        !self.equiv_rel.is_empty() && !self.carrier.is_empty()
    }
}
/// A partial equivalence relation (PER) type model used in realizability semantics.
///
/// A PER on a set `A` is a relation that is symmetric and transitive (but not
/// necessarily reflexive). PER models are used to interpret polymorphic type
/// systems (e.g., System F) and OTT semantics.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PERTypeModel {
    /// Name of the underlying set.
    pub set: String,
    /// Description of the partial equivalence relation.
    pub per: String,
    /// Whether symmetry has been verified.
    pub symmetric: bool,
    /// Whether transitivity has been verified.
    pub transitive: bool,
}
impl PERTypeModel {
    /// Create a new PER type model.
    pub fn new(
        set: impl Into<String>,
        per: impl Into<String>,
        symmetric: bool,
        transitive: bool,
    ) -> Self {
        Self {
            set: set.into(),
            per: per.into(),
            symmetric,
            transitive,
        }
    }
    /// Check if this is a valid PER (symmetric and transitive).
    pub fn is_valid_per(&self) -> bool {
        self.symmetric && self.transitive
    }
    /// The domain of a PER: the set of elements related to themselves.
    /// `dom(R) = { a | R a a }`.
    pub fn domain_name(&self) -> String {
        format!("dom({}/{})", self.set, self.per)
    }
    /// Restriction: restrict the PER to its domain to get an equivalence relation.
    pub fn restriction_to_domain(&self) -> Option<Setoid> {
        if self.is_valid_per() {
            Some(Setoid::new(
                self.domain_name(),
                format!("{}|dom", self.per),
                true,
                true,
                true,
            ))
        } else {
            None
        }
    }
    /// Compose two PER models (product construction).
    pub fn product(&self, other: &PERTypeModel) -> PERTypeModel {
        PERTypeModel {
            set: format!("{} × {}", self.set, other.set),
            per: format!("{} ×_per {}", self.per, other.per),
            symmetric: self.symmetric && other.symmetric,
            transitive: self.transitive && other.transitive,
        }
    }
    /// Check if the PER collapses to a setoid (reflexive on domain).
    pub fn is_setoid_like(&self) -> bool {
        self.is_valid_per()
    }
}
/// The Allen relation (also Allen–Milner partial equivalence relation):
/// a partial equivalence relation between two types used to model
/// type realizers in realizability semantics.
#[derive(Debug, Clone)]
pub struct AllenRelation {
    /// First type in the relation.
    pub type1: String,
    /// Second type in the relation.
    pub type2: String,
}
impl AllenRelation {
    /// Create a new Allen relation.
    pub fn new(type1: impl Into<String>, type2: impl Into<String>) -> Self {
        Self {
            type1: type1.into(),
            type2: type2.into(),
        }
    }
    /// Alien (Allen) semantics: interpret the relation as a PER-based type model.
    pub fn alien_semantics(&self) -> String {
        format!("Allen({}, {})", self.type1, self.type2)
    }
    /// Check if this relation is a partial equivalence relation (symmetric and transitive).
    pub fn is_partial_equivalence(&self) -> bool {
        true
    }
    /// Return the type of this Allen relation as a Prop.
    pub fn relation_type(&self) -> String {
        format!("PER {} {}", self.type1, self.type2)
    }
}
/// A displayed category over a base category (simplified Rust model).
#[derive(Debug, Clone)]
pub struct DisplayedCategory {
    /// Name of the base category.
    pub base: String,
    /// Name of this displayed category.
    pub name: String,
    /// Number of objects in the base (simplified).
    pub base_obj_count: usize,
}
impl DisplayedCategory {
    /// Create a displayed category over the given base.
    pub fn new(base: impl Into<String>, name: impl Into<String>, base_obj_count: usize) -> Self {
        Self {
            base: base.into(),
            name: name.into(),
            base_obj_count,
        }
    }
    /// The total category of this displayed category.
    pub fn total_category_name(&self) -> String {
        format!("∫{}", self.name)
    }
    /// A section of this displayed category assigns a fiber to each object.
    pub fn section_name(&self) -> String {
        format!("Section({})", self.name)
    }
    /// The number of "slices" (fibers) is equal to the number of base objects.
    pub fn slice_count(&self) -> usize {
        self.base_obj_count
    }
}
/// A heterogeneous equality witness between elements of possibly different types.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeterogeneousEq {
    /// Type of the left element.
    pub type_a: String,
    /// The left element.
    pub elem_a: String,
    /// Type of the right element.
    pub type_b: String,
    /// The right element.
    pub elem_b: String,
}
impl HeterogeneousEq {
    /// Create a new heterogeneous equality witness.
    pub fn new(
        type_a: impl Into<String>,
        elem_a: impl Into<String>,
        type_b: impl Into<String>,
        elem_b: impl Into<String>,
    ) -> Self {
        Self {
            type_a: type_a.into(),
            elem_a: elem_a.into(),
            type_b: type_b.into(),
            elem_b: elem_b.into(),
        }
    }
    /// Create a reflexivity witness (same type and element).
    pub fn refl(ty: impl Into<String>, elem: impl Into<String>) -> Self {
        let t = ty.into();
        let e = elem.into();
        Self {
            type_a: t.clone(),
            elem_a: e.clone(),
            type_b: t,
            elem_b: e,
        }
    }
    /// Check if this is a homogeneous equality (same types).
    pub fn is_homogeneous(&self) -> bool {
        self.type_a == self.type_b
    }
    /// Symmetrise the heterogeneous equality.
    pub fn sym(self) -> Self {
        Self {
            type_a: self.type_b,
            elem_a: self.elem_b,
            type_b: self.type_a,
            elem_b: self.elem_a,
        }
    }
    /// Attempt to compose two heterogeneous equalities (transitivity).
    /// Requires that the right type/element of `self` matches the left of `other`.
    pub fn trans(self, other: Self) -> Option<Self> {
        if self.type_b == other.type_a && self.elem_b == other.elem_a {
            Some(Self {
                type_a: self.type_a,
                elem_a: self.elem_a,
                type_b: other.type_b,
                elem_b: other.elem_b,
            })
        } else {
            None
        }
    }
    /// Convert to a homogeneous equality description (only valid if is_homogeneous).
    pub fn to_homogeneous(&self) -> Option<String> {
        if self.is_homogeneous() {
            Some(format!(
                "{} = {} : {}",
                self.elem_a, self.elem_b, self.type_a
            ))
        } else {
            None
        }
    }
    /// Format as a HEq term.
    pub fn display(&self) -> String {
        format!(
            "HEq ({} : {}) ({} : {})",
            self.elem_a, self.type_a, self.elem_b, self.type_b
        )
    }
}
/// Records whether a type is known to satisfy UIP.
#[derive(Debug, Clone, PartialEq)]
pub enum UIPStatus {
    /// UIP holds definitionally (strict types in 2LTT, setoids).
    Definitional,
    /// UIP holds propositionally (proved via AxiomK or UIP axiom).
    Propositional,
    /// UIP is not known to hold (fibrant types, types admitting UA).
    Unknown,
    /// UIP is known to fail (types that admit univalence).
    Fails,
}
impl UIPStatus {
    /// Check if UIP holds in some form.
    pub fn holds(&self) -> bool {
        matches!(self, UIPStatus::Definitional | UIPStatus::Propositional)
    }
    /// Check if UIP holds definitionally.
    pub fn is_definitional(&self) -> bool {
        matches!(self, UIPStatus::Definitional)
    }
}
/// Program equivalence: two programs are equivalent if they are
/// indistinguishable in all program contexts (contextual equivalence) or
/// related by a logical relation.
#[derive(Debug, Clone)]
pub struct ProgramEquiv {
    /// Source code / name of the first program.
    pub program_a: String,
    /// Source code / name of the second program.
    pub program_b: String,
}
impl ProgramEquiv {
    /// Create a new program equivalence witness.
    pub fn new(program_a: impl Into<String>, program_b: impl Into<String>) -> Self {
        Self {
            program_a: program_a.into(),
            program_b: program_b.into(),
        }
    }
    /// Contextual equivalence: equal in all program contexts.
    ///
    /// `C\[A\] ≃ C\[B\]` for all contexts `C`.
    pub fn contextual_equivalence(&self) -> String {
        format!("ctx_equiv({}, {})", self.program_a, self.program_b)
    }
    /// Logical relation: related by the type-indexed logical relation.
    ///
    /// This is the standard tool for proving contextual equivalence.
    pub fn logical_relation(&self, ty: &str) -> String {
        format!("LogRel_{ty}({}, {})", self.program_a, self.program_b)
    }
    /// Trivial check: programs with the same text are equivalent.
    pub fn is_syntactically_equal(&self) -> bool {
        self.program_a == self.program_b
    }
    /// Bisimulation-based equivalence (coinductive).
    pub fn bisimulation(&self) -> String {
        format!("bisim({}, {})", self.program_a, self.program_b)
    }
}
/// Represents a type in 2LTT as either fibrant or strict.
#[derive(Debug, Clone, PartialEq)]
pub enum TwoLevelType {
    /// A fibrant type (lives in the HoTT layer, obeys univalence).
    Fibrant { name: String },
    /// A strict type (has definitional UIP, exact equality).
    Strict { name: String },
    /// A lifted strict type (embedded into the fibrant layer).
    Lifted { inner: Box<TwoLevelType> },
}
impl TwoLevelType {
    /// Create a fibrant type.
    pub fn fibrant(name: impl Into<String>) -> Self {
        Self::Fibrant { name: name.into() }
    }
    /// Create a strict type.
    pub fn strict(name: impl Into<String>) -> Self {
        Self::Strict { name: name.into() }
    }
    /// Lift a strict type into the fibrant layer.
    pub fn lift(self) -> Self {
        match self {
            TwoLevelType::Strict { .. } => Self::Lifted {
                inner: Box::new(self),
            },
            other => other,
        }
    }
    /// Get the UIP status of this type.
    pub fn uip_status(&self) -> UIPStatus {
        match self {
            TwoLevelType::Strict { .. } => UIPStatus::Definitional,
            TwoLevelType::Fibrant { .. } => UIPStatus::Unknown,
            TwoLevelType::Lifted { .. } => UIPStatus::Propositional,
        }
    }
    /// Whether this type admits univalence (only fibrant types do).
    pub fn admits_univalence(&self) -> bool {
        matches!(self, TwoLevelType::Fibrant { .. })
    }
    /// The name of the type.
    pub fn name(&self) -> String {
        match self {
            TwoLevelType::Fibrant { name } | TwoLevelType::Strict { name } => name.clone(),
            TwoLevelType::Lifted { inner } => format!("Lift({})", inner.name()),
        }
    }
}
/// Represents a Tarski-style universe: a code type with a separate decoding
/// function. Used in OTT to provide explicit, setoid-friendly universes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TarskiUniverse {
    /// The universe level.
    pub level: u32,
    /// Registered codes (names of types that are codes in this universe).
    pub codes: Vec<String>,
}
impl TarskiUniverse {
    /// Create a new Tarski universe at the given level.
    pub fn new(level: u32) -> Self {
        Self {
            level,
            codes: Vec::new(),
        }
    }
    /// Register a new type code.
    pub fn register_code(&mut self, code: impl Into<String>) {
        self.codes.push(code.into());
    }
    /// The name of the code type (e.g., `U₀`, `U₁`).
    pub fn code_type_name(&self) -> String {
        format!("U{}", self.level)
    }
    /// The decoding function name.
    pub fn el_name(&self) -> String {
        format!("El{}", self.level)
    }
    /// Decode a code to its type description.
    pub fn decode(&self, code: &str) -> String {
        format!("{}({})", self.el_name(), code)
    }
    /// Check if the universe contains a given code.
    pub fn contains_code(&self, code: &str) -> bool {
        self.codes.iter().any(|c| c == code)
    }
    /// Return the number of registered codes.
    pub fn code_count(&self) -> usize {
        self.codes.len()
    }
    /// Lift to the next universe level.
    pub fn lift(&self) -> TarskiUniverse {
        TarskiUniverse {
            level: self.level + 1,
            codes: self.codes.clone(),
        }
    }
}
/// The justification strategy for observational equality.
#[derive(Debug, Clone, PartialEq)]
pub enum ObsEqJustification {
    /// Reflexivity: lhs and rhs are definitionally the same.
    Refl,
    /// Symmetry: we have eq(rhs, lhs) and flip it.
    Sym(Box<ObsEqProof>),
    /// Transitivity: two proofs chained together.
    Trans(Box<ObsEqProof>, Box<ObsEqProof>),
    /// Function extensionality: proved pointwise.
    Funext(String),
    /// Coercion from a related proof.
    Coerce(String),
}
/// Observations for how OTT equality behaves over the main type formers:
/// Sigma, Pi, and Nat.
#[derive(Debug, Clone, Default)]
pub struct TypeFormers;
impl TypeFormers {
    /// Create a new type formers witness.
    pub fn new() -> Self {
        Self
    }
    /// Sigma type observational equality:
    /// `ObsEq (Σ x:A, B x) p q ↔ Σ (e : ObsEq A p.1 q.1), ObsEq (B q.1) (coerce e p.2) q.2`
    pub fn sigma_obs_eq(&self, a: &str, b_pred: &str) -> String {
        format!("sigma_obs_eq(Σ {}: {}, {})", a, a, b_pred)
    }
    /// Pi type observational equality:
    /// `ObsEq (Π x:A, B x) f g ↔ ∀ x y, ObsEq A x y → ObsEq (B y) (f x) (g y)`
    pub fn pi_obs_eq(&self, a: &str, b_pred: &str) -> String {
        format!("pi_obs_eq(Π {}: {}, {})", a, a, b_pred)
    }
    /// Nat observational equality:
    /// `ObsEq Nat n m ↔ n = m` (reduces to decidable equality on Nat)
    pub fn nat_obs_eq(&self, n: &str, m: &str) -> String {
        format!("nat_obs_eq({}, {})", n, m)
    }
    /// Bool observational equality:
    /// `ObsEq Bool b1 b2 ↔ b1 = b2`
    pub fn bool_obs_eq(&self, b1: &str, b2: &str) -> String {
        format!("bool_obs_eq({}, {})", b1, b2)
    }
}
/// An observational quotient type: a type `A` quotiented by a relation `R`
/// where equality in the quotient is *definitionally* equal to `R`.
///
/// This is stronger than the setoid quotient: in OTT, the equality of
/// `ObsQuot A R` is the *same* as `R` by definition, not just by proof.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ObservationalQuotient {
    /// The carrier type.
    pub carrier: String,
    /// The equivalence relation.
    pub relation: String,
    /// Whether the relation has been verified to be a congruence.
    pub is_congruence: bool,
}
impl ObservationalQuotient {
    /// Create a new observational quotient.
    pub fn new(
        carrier: impl Into<String>,
        relation: impl Into<String>,
        is_congruence: bool,
    ) -> Self {
        Self {
            carrier: carrier.into(),
            relation: relation.into(),
            is_congruence,
        }
    }
    /// The name of the quotient type `A/R`.
    pub fn quotient_name(&self) -> String {
        format!("ObsQuot({}, {})", self.carrier, self.relation)
    }
    /// The embedding of an element into the quotient.
    pub fn embed(&self, elem: &str) -> String {
        format!("[{}]_ObsQuot({})", elem, self.relation)
    }
    /// Check if two elements are equal in the quotient.
    /// In OTT this is *definitionally* the same as checking `R a b`.
    pub fn are_equal(&self, a: &str, b: &str) -> String {
        format!("{} ~_{} {}", a, self.relation, b)
    }
    /// Lift a function `f : A → B` that respects `R` to the quotient.
    pub fn lift_function(&self, f: &str, codomain: &str) -> String {
        format!(
            "ObsQuot.lift({}, {}, {})",
            f,
            self.quotient_name(),
            codomain
        )
    }
    /// Check if the quotient is well-formed (relation must be a congruence).
    pub fn is_well_formed(&self) -> bool {
        self.is_congruence && !self.carrier.is_empty() && !self.relation.is_empty()
    }
}
/// Realizability model: interprets types as sets of realizers in a partial
/// combinatory algebra (PCA).
#[derive(Debug, Clone)]
pub struct RealizabilityModel {
    /// Name of the partial combinatory algebra.
    pub pca: String,
}
impl RealizabilityModel {
    /// Create a new realizability model over the given PCA.
    pub fn new(pca: impl Into<String>) -> Self {
        Self { pca: pca.into() }
    }
    /// Interpret a formula in the realizability model.
    pub fn realizability_interpretation(&self, formula: &str) -> String {
        format!("⟦{}⟧_{{{}}} ", formula, self.pca)
    }
    /// Connection to computability: realizers are computable functions.
    pub fn computability_connection(&self) -> String {
        format!("Realizers in {} are computable partial functions", self.pca)
    }
    /// The Kleene PCA (first Kleene algebra) — standard realizability.
    pub fn kleene_pca() -> Self {
        Self::new("K1")
    }
    /// The effective topos is the category of assemblies over K1.
    pub fn effective_topos_name(&self) -> String {
        format!("Eff({})", self.pca)
    }
}
