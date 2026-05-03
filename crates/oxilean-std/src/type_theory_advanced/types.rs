//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// The distinction between extensional and intensional type theory.
///
/// - Intensional (MLTT): definitional equality is undecidable in the presence of
///   UIP, but weak function extensionality holds. Type checking is decidable.
/// - Extensional (ETT): the equality reflection rule is added, collapsing
///   propositional and definitional equality. Type checking becomes undecidable.
pub struct ExtensionalVsIntensional {
    /// Whether the equality reflection rule is assumed.
    pub has_equality_reflection: bool,
    /// Whether function extensionality holds as an axiom.
    pub has_function_extensionality: bool,
    /// Whether uniqueness of identity proofs (K/UIP) is assumed.
    pub has_uip: bool,
}
impl ExtensionalVsIntensional {
    /// Intensional MLTT (default constructive type theory).
    pub fn intensional() -> Self {
        Self {
            has_equality_reflection: false,
            has_function_extensionality: false,
            has_uip: false,
        }
    }
    /// Extensional type theory (Martin-Löf 1979 ETT).
    pub fn extensional() -> Self {
        Self {
            has_equality_reflection: true,
            has_function_extensionality: true,
            has_uip: true,
        }
    }
    /// Create with explicit settings.
    pub fn new() -> Self {
        Self::intensional()
    }
    /// The equality reflection rule: propositional equality implies definitional equality.
    ///
    /// Eq-Refl: If p : Id_A(a, b) then a ≡ b (definitionally).
    ///
    /// This makes the theory extensional but renders type checking undecidable
    /// (since it can encode arbitrary computation into the definitional equality).
    pub fn equality_reflection(&self) -> String {
        if self.has_equality_reflection {
            "Equality reflection rule (ETT): \
             If Γ ⊢ p : Id_A(a, b) then Γ ⊢ a ≡ b (definitionally). \
             Consequence: any two proofs of the same identity are definitionally equal \
             (proof irrelevance for equalities). \
             Cost: type checking becomes undecidable. \
             Benefit: extensionality principles hold definitionally."
                .to_string()
        } else {
            "Intensional type theory: no equality reflection. \
             Propositional identity Id_A(a, b) does not imply definitional equality a ≡ b. \
             Type checking remains decidable (strong normalization + confluence). \
             Propositional equalities (e.g. funext) must be stated as axioms."
                .to_string()
        }
    }
    /// The undecidability of type checking in extensional type theory.
    ///
    /// Streicher (1993): Type checking in extensional MLTT is undecidable,
    /// because the equality reflection rule can encode arbitrary halting problems.
    pub fn undecidable_type_checking_extensional(&self) -> String {
        if self.has_equality_reflection {
            "Undecidability (Streicher 1993): Type checking in extensional MLTT is undecidable. \
             The equality reflection rule allows encoding the halting problem: \
             given a program P, one can construct a type whose inhabitants exist \
             iff P halts. Therefore no algorithm can decide Γ ⊢ t : A in general \
             in extensional type theory."
                .to_string()
        } else {
            "Type checking is decidable in this theory (no equality reflection). \
             Strong normalization ensures every reduction terminates, \
             and confluence ensures normal forms are unique. \
             These together give a decision procedure for definitional equality."
                .to_string()
        }
    }
    /// Whether function extensionality holds.
    pub fn funext_holds(&self) -> bool {
        self.has_equality_reflection || self.has_function_extensionality
    }
    /// Whether uniqueness of identity proofs (UIP / Axiom K) holds.
    pub fn uip_holds(&self) -> bool {
        self.has_equality_reflection || self.has_uip
    }
}
/// Resource grade in QTT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    /// Erased (computationally irrelevant, grade 0).
    Zero,
    /// Linear (used exactly once, grade 1).
    Linear,
    /// Unrestricted (used any number of times, grade ω).
    Unrestricted,
}
impl Grade {
    /// Add two grades in the resource semiring.
    pub fn add(self, other: Grade) -> Grade {
        match (self, other) {
            (Grade::Zero, g) | (g, Grade::Zero) => g,
            (Grade::Linear, Grade::Linear) => Grade::Unrestricted,
            _ => Grade::Unrestricted,
        }
    }
    /// Multiply (scale) two grades.
    pub fn mul(self, other: Grade) -> Grade {
        match (self, other) {
            (Grade::Zero, _) | (_, Grade::Zero) => Grade::Zero,
            (Grade::Linear, Grade::Linear) => Grade::Linear,
            _ => Grade::Unrestricted,
        }
    }
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Grade::Zero => "0 (erased)",
            Grade::Linear => "1 (linear)",
            Grade::Unrestricted => "ω (unrestricted)",
        }
    }
}
/// A modal context for modal type theory.
///
/// Modal type theory (Gratzer-Sterling-Birkedal, Shulman) extends MLTT with
/// necessity (□) and possibility (◇) modalities.
#[derive(Debug, Clone)]
pub struct ModalContext {
    /// Named modalities available in this context.
    pub modalities: Vec<String>,
    /// Whether the necessity □ is a comonad (S4 modality).
    pub necessity_is_comonad: bool,
    /// Whether the possibility ◇ is a monad.
    pub possibility_is_monad: bool,
}
impl ModalContext {
    /// Create a standard modal type theory context (S4 / lax logic).
    pub fn standard() -> Self {
        Self {
            modalities: vec!["□".to_string(), "◇".to_string()],
            necessity_is_comonad: true,
            possibility_is_monad: true,
        }
    }
    /// Create a crisp/cohesive modal context.
    pub fn cohesive() -> Self {
        Self {
            modalities: vec!["♭".to_string(), "♯".to_string(), "∫".to_string()],
            necessity_is_comonad: true,
            possibility_is_monad: true,
        }
    }
    /// Whether the context has the given modality name.
    pub fn has_modality(&self, name: &str) -> bool {
        self.modalities.iter().any(|m| m == name)
    }
    /// Describe the modal axioms (K, T, 4 for S4).
    pub fn modal_axioms(&self) -> Vec<String> {
        vec![
            "K: □(A → B) → □A → □B".to_string(),
            "T: □A → A (truth)".to_string(),
            "4: □A → □□A (transitivity)".to_string(),
        ]
    }
}
/// The Calculus of Constructions (CoC) of Coquand and Huet.
///
/// CoC is a Pure Type System (PTS) with:
/// - Two sorts: Prop and Type
/// - Polymorphic Π-types in all four combinations: (Prop, Prop), (Type, Prop),
///   (Prop, Type), (Type, Type)
/// - The Curry-Howard correspondence
///
/// CoC = λω in Barendregt's λ-cube (top of the cube).
pub struct CoC {
    /// Named terms inhabiting the theory.
    pub terms: Vec<String>,
    /// Whether the system includes universe levels beyond {Prop, Type}.
    pub has_extended_hierarchy: bool,
}
impl CoC {
    /// Create the basic Calculus of Constructions.
    pub fn new(terms: Vec<String>) -> Self {
        Self {
            terms,
            has_extended_hierarchy: false,
        }
    }
    /// Create an empty CoC.
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
    /// The Curry-Howard correspondence for CoC.
    ///
    /// In CoC:
    /// - Propositions are types (in Prop)
    /// - Proofs are terms (inhabiting propositions)
    /// - Logical operations correspond to type constructors:
    ///   A ∧ B = A × B, A ∨ B = ∀C:Prop, (A → C) → (B → C) → C, ¬A = A → ⊥
    pub fn curry_howard(&self) -> String {
        "Curry-Howard correspondence in CoC: \
         propositions = types (in Prop), proofs = terms (programs). \
         (A ∧ B) ↔ (A × B) — products as conjunction; \
         (A → B) ↔ (A ⊃ B) — functions as implication; \
         (∀ x:A, B x) ↔ (Π x:A, B x) — dependent functions as universal quantification; \
         ⊥ ↔ ∀C:Prop, C — the empty type (ex falso); \
         ¬A ↔ A → ⊥ — negation as function into empty type."
            .to_string()
    }
    /// Propositions-as-types (PAT) / proofs-as-programs.
    pub fn propositions_as_types(&self) -> String {
        "Propositions-as-types (Howard 1969, Curry 1934): \
         Every proposition P has a type [[P]] such that proofs of P \
         correspond exactly to terms of type [[P]]. \
         In CoC: [[A ∧ B]] = A × B, [[A ⊃ B]] = A → B, [[∀x, P x]] = Π x, [[P x]], \
         [[A ∨ B]] = ∀C, (A → C) → (B → C) → C (Church encoding), \
         [[∃x, P x]] = ∀C, (∀x, P x → C) → C (Church encoding)."
            .to_string()
    }
    /// The expressive power of CoC.
    ///
    /// CoC can express:
    /// - All of second-order logic (System F = λ2 is a fragment)
    /// - All predicates definable in Peano arithmetic (and more)
    /// - Church encodings of all algebraic data types
    /// - System F and its extensions up to ω in Barendregt's cube
    pub fn expressive_power(&self) -> String {
        "Expressive power of CoC (= λω in the λ-cube): \
         (1) Subsumes System F (second-order polymorphic λ-calculus = λ2); \
         (2) Subsumes λω (type operators); \
         (3) Subsumes λP (dependent types over Set); \
         (4) Encodes all algebraic data types via Church encodings; \
         (5) Proof-theoretically equivalent to System F_ω; \
         (6) Does not prove all true Π⁰₁ statements (unlike PA); \
         (7) Strong normalization and Church-Rosser hold."
            .to_string()
    }
    /// The four PTS rules of CoC (Barendregt's λ-cube, corner λω).
    pub fn pts_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Prop", "Prop", "implication/∀ over Prop"),
            ("Type", "Prop", "second-order quantification"),
            ("Prop", "Type", "dependent types over Prop"),
            ("Type", "Type", "type operators and polymorphism"),
        ]
    }
    /// Whether CoC is strongly normalizing.
    pub fn is_strongly_normalizing(&self) -> bool {
        true
    }
}
/// The identity type Id_A(a, b) : Type (Martin-Löf identity type).
pub struct IdentityTypeScheme {
    /// The ambient type.
    pub ambient_type: String,
    /// Whether this is the intensional (path) or extensional identity.
    pub is_intensional: bool,
}
impl IdentityTypeScheme {
    /// Create an intensional identity type.
    pub fn intensional(ambient_type: impl Into<String>) -> Self {
        Self {
            ambient_type: ambient_type.into(),
            is_intensional: true,
        }
    }
    /// The eliminator (J-rule / path induction).
    pub fn j_rule(&self) -> String {
        format!(
            "J-rule (path induction) for Id_{}:\
             J : ∀(C : ∀(a b:{A}), Id(a,b) → U), \
             (∀(a:{A}), C a a refl) → \
             ∀(a b:{A})(p : Id(a,b)), C a b p. \
             Computation: J C c a a refl ≡ c a.",
            self.ambient_type,
            A = self.ambient_type
        )
    }
}
/// An inductive type definition.
///
/// An inductive type is specified by:
/// - A type former: the type constructor
/// - A list of constructors (introduction rules)
/// - The induction principle (elimination rule)
/// - The computation rules (β-reductions for the eliminator)
pub struct Induction {
    /// The type former (e.g. "Nat", "List", "Tree").
    pub type_former: String,
    /// The constructor names (e.g. \["zero", "succ"\]).
    pub constructors: Vec<String>,
    /// Whether this is a recursive type (has recursive occurrences).
    pub is_recursive: bool,
    /// Whether this is a strictly positive inductive type.
    pub is_strictly_positive: bool,
}
impl Induction {
    /// Create an inductive type definition.
    pub fn new(type_former: impl Into<String>, constructors: Vec<String>) -> Self {
        Self {
            type_former: type_former.into(),
            constructors,
            is_recursive: true,
            is_strictly_positive: true,
        }
    }
    /// The natural numbers (the canonical inductive type).
    pub fn nat() -> Self {
        Self::new("Nat", vec!["zero".to_string(), "succ".to_string()])
    }
    /// Boolean (non-recursive inductive type).
    pub fn bool_type() -> Self {
        Self {
            type_former: "Bool".to_string(),
            constructors: vec!["true".to_string(), "false".to_string()],
            is_recursive: false,
            is_strictly_positive: true,
        }
    }
    /// Lists (polymorphic inductive type).
    pub fn list() -> Self {
        Self::new("List", vec!["nil".to_string(), "cons".to_string()])
    }
    /// The induction principle for this type.
    ///
    /// The induction principle is the categorical recursor for the type,
    /// giving both elimination and computation rules.
    pub fn induction_principle(&self) -> String {
        let ctor_desc: Vec<String> = self
            .constructors
            .iter()
            .map(|c| format!("case {}(...) : C(...)", c))
            .collect();
        format!(
            "Induction principle for {}: \
             ind-{} : ∀(C : {} → U), \
             ({}), \
             ∀(x : {}), C(x). \
             Computation rules: one β-rule per constructor ({} rules total).",
            self.type_former,
            self.type_former,
            self.type_former,
            ctor_desc.join("; "),
            self.type_former,
            self.constructors.len()
        )
    }
    /// The uniqueness principle (η-rule) for this type.
    ///
    /// The η-rule for inductive types states that any two elements satisfying
    /// the same induction hypothesis are equal. For function types:
    /// η(f) : f ≡ λx, f x.
    pub fn uniqueness_principle(&self) -> String {
        format!(
            "Uniqueness principle (η-rule) for {}: \
             Any function f : {} → C satisfying all case conditions \
             is uniquely determined by them (up to propositional equality). \
             Equivalently: ind-{} C (f ∘ ctor₁) ... (f ∘ ctorₙ) = f \
             for every f in Hom({}, C).",
            self.type_former, self.type_former, self.type_former, self.type_former
        )
    }
    /// Whether this inductive type has a decidable equality.
    pub fn has_decidable_equality(&self) -> bool {
        !self.is_recursive || self.type_former == "Nat" || self.type_former == "Bool"
    }
    /// Whether this is a higher inductive type (HIT).
    pub fn is_hit(&self) -> bool {
        false
    }
}
/// Realizability assembly: a set with a realizability predicate.
///
/// In PCA realizability (van Oosten, Longley-Normann), each element of a
/// type is "realized" by a code in the PCA. This struct tracks simple
/// realizability evidence.
#[derive(Debug, Clone)]
pub struct RealizabilityAssembly {
    /// Name of this assembly.
    pub name: String,
    /// Whether every element has at least one realizer (totalness).
    pub is_total: bool,
    /// Number of elements (abstract size).
    pub size: usize,
}
impl RealizabilityAssembly {
    /// Create a new assembly.
    pub fn new(name: impl Into<String>, is_total: bool, size: usize) -> Self {
        Self {
            name: name.into(),
            is_total,
            size,
        }
    }
    /// Whether this is a partitioned assembly (each element has exactly one realizer class).
    pub fn is_partitioned(&self) -> bool {
        self.is_total
    }
    /// The effective topos: assemblies form a topos with natural numbers object.
    pub fn effective_topos_description() -> String {
        "Effective topos (Hyland 1982): The category of assemblies over Kleene's \
         first algebra K1 forms a topos. It contains the realizability interpretation \
         of constructive mathematics and satisfies Church's thesis internally."
            .to_string()
    }
}
/// A normalization theorem for a type theory.
///
/// Strong normalization (SN): every reduction sequence terminates.
/// Confluence (CR): if t →* u and t →* v, then there exists w with u →* w and v →* w.
pub struct NormalizationThm {
    /// The type theory for which normalization holds.
    pub type_theory: String,
    /// Whether strong normalization has been proved.
    pub strong_normalization_proved: bool,
    /// Whether confluence has been proved.
    pub confluence_proved: bool,
    /// The proof technique used for SN.
    pub sn_proof_technique: String,
}
impl NormalizationThm {
    /// Create a normalization theorem for the given type theory.
    pub fn new(type_theory: impl Into<String>) -> Self {
        let tt = type_theory.into();
        let technique = if tt == "STLC" || tt == "λ→" {
            "Tait's reducibility candidates (logical relations)"
        } else if tt.contains("CIC") || tt.contains("CoC") {
            "Girard-Tait reducibility candidates extended to dependent types"
        } else {
            "Logical relations / reducibility candidates"
        };
        Self {
            type_theory: tt,
            strong_normalization_proved: true,
            confluence_proved: true,
            sn_proof_technique: technique.to_string(),
        }
    }
    /// The strong normalization theorem.
    ///
    /// SN: Every well-typed term has a normal form (no infinite reduction sequences).
    /// This is proved using reducibility candidates (Girard's method).
    pub fn strong_normalization(&self) -> String {
        format!(
            "Strong Normalization theorem for {}: \
             Every β-reduction sequence t₀ →_β t₁ →_β t₂ →_β ... \
             starting from a well-typed term t₀ : A in {} is finite. \
             Proof technique: {}. \
             Consequences: decidable type checking (in pure {}), \
             consistency (no closed proof of ⊥).",
            self.type_theory, self.type_theory, self.sn_proof_technique, self.type_theory
        )
    }
    /// The confluence (Church-Rosser) theorem.
    ///
    /// CR: If t →* u and t →* v, then there exists w such that u →* w and v →* w.
    /// This implies uniqueness of normal forms.
    pub fn confluence(&self) -> String {
        format!(
            "Confluence (Church-Rosser) theorem for {}: \
             If t →* u and t →* v, then ∃w, u →* w ∧ v →* w (diamond property). \
             Consequences: (1) normal forms are unique (up to α-equivalence); \
             (2) definitional equality is decidable (via normalization); \
             (3) the quotient of terms by =_βη is well-defined. \
             Proof: parallel reduction (Takahashi's complete development).",
            self.type_theory
        )
    }
    /// The decidability of type checking (from SN + CR).
    pub fn decidable_type_checking(&self) -> bool {
        self.strong_normalization_proved && self.confluence_proved
    }
}
/// Computer for setoid quotients.
///
/// A setoid is a pair (A, ~) where ~ is an equivalence relation.
/// The setoid quotient A/~ is computed by choosing a canonical representative
/// from each equivalence class.
#[derive(Debug, Clone)]
pub struct SetoidQuotientComputer {
    /// Elements as string labels.
    pub elements: Vec<String>,
    /// Equivalence relation as pairs of indices.
    pub equiv_pairs: Vec<(usize, usize)>,
}
impl SetoidQuotientComputer {
    /// Create a setoid quotient computer.
    pub fn new(elements: Vec<String>) -> Self {
        let n = elements.len();
        let equiv_pairs = (0..n).map(|i| (i, i)).collect();
        Self {
            elements,
            equiv_pairs,
        }
    }
    /// Add an equivalence: declare elements\[i\] ~ elements\[j\].
    /// Automatically adds (j, i) for symmetry.
    pub fn add_equiv(&mut self, i: usize, j: usize) {
        if i < self.elements.len() && j < self.elements.len() {
            self.equiv_pairs.push((i, j));
            self.equiv_pairs.push((j, i));
        }
    }
    /// Find the equivalence class of element i (union-find via Floyd's method).
    pub fn equiv_class(&self, i: usize) -> Vec<usize> {
        let n = self.elements.len();
        let mut cls = Vec::new();
        for j in 0..n {
            if self
                .equiv_pairs
                .iter()
                .any(|&(a, b)| (a == i && b == j) || (a == j && b == i))
            {
                cls.push(j);
            }
        }
        if cls.is_empty() {
            cls.push(i);
        }
        cls.sort();
        cls.dedup();
        cls
    }
    /// Number of equivalence classes (approximate, via canonical representatives).
    pub fn num_classes(&self) -> usize {
        let n = self.elements.len();
        let mut seen = vec![false; n];
        let mut count = 0;
        for i in 0..n {
            if !seen[i] {
                count += 1;
                for j in self.equiv_class(i) {
                    seen[j] = true;
                }
            }
        }
        count
    }
    /// Canonical representative of the class of i (smallest index in class).
    pub fn canonical(&self, i: usize) -> usize {
        self.equiv_class(i).into_iter().min().unwrap_or(i)
    }
}
/// Synthetic domain theory: dominance and partial elements.
///
/// Synthetic domain theory (SDT, Rosolini 1986) axiomatizes domain theory
/// inside type theory by postulating a dominance Σ ⊆ Prop and the lift
/// monad L(A) = { (P : Σ, A^P) }.
#[derive(Debug, Clone)]
pub struct SyntheticDomainTheory {
    /// Name of the dominance (classifying object for partial maps).
    pub dominance_name: String,
    /// Whether the dominance contains ⊤ (total elements).
    pub has_top: bool,
    /// Whether the dominance is closed under ∧ (finite meets).
    pub closed_under_meets: bool,
}
impl SyntheticDomainTheory {
    /// Create the standard SDT structure.
    pub fn standard() -> Self {
        Self {
            dominance_name: "Sigma".to_string(),
            has_top: true,
            closed_under_meets: true,
        }
    }
    /// The lift monad description: L(A) = Σ-partial elements of A.
    pub fn lift_description(&self) -> String {
        format!(
            "Lift monad L(A) with dominance {}: \
             L(A) = {{ (P : {}, a : A^P) }}. \
             Unit: η : A → L(A), η(a) = (⊤, const a). \
             Kleisli extension: (>>=) : L(A) → (A → L(B)) → L(B). \
             Fixed-point: every endo L(A) → L(A) has a least fixed point.",
            self.dominance_name, self.dominance_name
        )
    }
    /// Partial map classifier: partial maps A ⇀ B = total maps A → L(B).
    pub fn partial_map_description(&self) -> String {
        format!(
            "Partial map A ⇀ B via dominance {}: \
             Defined as total maps A → L(B). \
             The category Par(Eff) of partial maps is enriched over domains. \
             Scott topology on L(A) makes it a ω-cppo.",
            self.dominance_name
        )
    }
    /// Whether the dominance axioms are satisfied (consistency check).
    pub fn is_consistent(&self) -> bool {
        self.has_top && self.closed_under_meets
    }
}
/// Type inference for quantitative (graded / resourced) types.
///
/// In Quantitative Type Theory (McBride, Atkey 2018), every variable usage
/// is annotated with a resource grade from a semiring (0 = erased, 1 = linear,
/// ω = unrestricted). The inference engine tracks grades and checks linearity.
#[derive(Debug, Clone)]
pub struct QuantitativeTypeInference {
    /// Context: list of (variable name, type name, resource grade).
    pub context: Vec<(String, String, u32)>,
    /// Whether to use the ω (omega / unrestricted) semiring.
    pub semiring_has_omega: bool,
}
impl QuantitativeTypeInference {
    /// Create a new QTT inference engine.
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            semiring_has_omega: true,
        }
    }
    /// Add a variable binding with grade.
    pub fn bind(&mut self, var: impl Into<String>, ty: impl Into<String>, grade: u32) {
        self.context.push((var.into(), ty.into(), grade));
    }
    /// Look up the grade of a variable (0 if not found).
    pub fn grade_of(&self, var: &str) -> u32 {
        self.context
            .iter()
            .rev()
            .find(|(v, _, _)| v == var)
            .map(|(_, _, g)| *g)
            .unwrap_or(0)
    }
    /// Check if a variable is used linearly (grade exactly 1).
    pub fn is_linear(&self, var: &str) -> bool {
        self.grade_of(var) == 1
    }
    /// Check if a variable is computationally erased (grade 0).
    pub fn is_erased(&self, var: &str) -> bool {
        self.grade_of(var) == 0
    }
    /// Check if a variable is unrestricted (grade ≥ 2 or ω).
    pub fn is_unrestricted(&self, var: &str) -> bool {
        self.grade_of(var) >= 2
    }
    /// Return a description of the typing context.
    pub fn context_description(&self) -> String {
        self.context
            .iter()
            .map(|(v, t, g)| format!("{v} :_{g} {t}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
/// Checker for two-level type theory (2LTT) judgments.
///
/// 2LTT (Annenkov-Capriotti-Kraus-Schreiber) has two layers:
/// - The inner (fibrant) layer: HoTT / univalent type theory.
/// - The outer (strict) layer: a metatheory with strict equality.
///
/// This allows working with simplicial sets and other presheaf models
/// while maintaining computational content.
#[derive(Debug, Clone)]
pub struct TwoLevelTypeChecker {
    /// Types registered as fibrant (inner layer).
    pub fibrant_types: Vec<String>,
    /// Types registered as strict (outer layer only).
    pub strict_types: Vec<String>,
    /// Whether the coherence axiom between layers holds.
    pub coherence_holds: bool,
}
impl TwoLevelTypeChecker {
    /// Create a two-level checker with coherence enabled.
    pub fn new() -> Self {
        Self {
            fibrant_types: Vec::new(),
            strict_types: Vec::new(),
            coherence_holds: true,
        }
    }
    /// Register a type as fibrant.
    pub fn register_fibrant(&mut self, ty: impl Into<String>) {
        self.fibrant_types.push(ty.into());
    }
    /// Register a type as strict (outer layer).
    pub fn register_strict(&mut self, ty: impl Into<String>) {
        self.strict_types.push(ty.into());
    }
    /// Check whether a given type name is fibrant.
    pub fn is_fibrant(&self, ty: &str) -> bool {
        self.fibrant_types.iter().any(|t| t == ty)
    }
    /// Check whether a given type name is strictly typed.
    pub fn is_strict(&self, ty: &str) -> bool {
        self.strict_types.iter().any(|t| t == ty)
    }
    /// Every fibrant type can be coerced to a strict type (downward inclusion).
    pub fn coerce_to_strict(&self, ty: &str) -> String {
        if self.is_fibrant(ty) {
            format!("Strict({ty}) [coerced from fibrant]")
        } else {
            format!("{ty} [already strict or not registered]")
        }
    }
    /// Summary of the 2LTT structure.
    pub fn summary(&self) -> String {
        format!(
            "2LTT: {} fibrant types, {} strict types, coherence: {}",
            self.fibrant_types.len(),
            self.strict_types.len(),
            self.coherence_holds
        )
    }
}
/// A dependent type in the sense of Martin-Löf.
///
/// A dependent type is a family of types indexed by a base type:
/// - Base type A : Type
/// - Family B : A → Type  (or equivalently B : A → U)
/// - Total space Σ(x : A), B(x)  (the Σ-type)
/// - Pi type Π(x : A), B(x)       (the Π-type)
pub struct DependentType {
    /// The base type (domain of the indexing).
    pub base: String,
    /// The type family (a function from base to types).
    pub fiber_over: String,
    /// Whether the type family is uniform (constant fiber).
    pub is_uniform: bool,
}
impl DependentType {
    /// Create a dependent type with the given base and fiber.
    pub fn new(base: impl Into<String>, fiber_over: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            fiber_over: fiber_over.into(),
            is_uniform: false,
        }
    }
    /// Create a simple (non-dependent) type (uniform family).
    pub fn simple(ty: impl Into<String>) -> Self {
        let ty = ty.into();
        Self {
            base: ty.clone(),
            fiber_over: ty,
            is_uniform: true,
        }
    }
    /// The total space of the dependent type (Σ-type).
    ///
    /// The total space is { (a, b) | a : A, b : B(a) },
    /// written Σ(x : A), B(x) or (x : A) × B(x).
    pub fn total_space(&self) -> String {
        format!(
            "TotalSpace(Σ(x : {A}), {B}(x)): \
             the type of pairs (a : {A}, b : {B}(a)). \
             First projection fst : Σ(x:{A}), {B}(x) → {A}; \
             second projection snd : ∀(p : Σ(x:{A}), {B}(x)), {B}(fst p).",
            A = self.base,
            B = self.fiber_over
        )
    }
    /// The Π-type (dependent function type).
    ///
    /// Π(x : A), B(x) is the type of dependent functions f : (x : A) → B(x).
    /// When B is constant, this is the ordinary function type A → B.
    pub fn pi_type(&self) -> String {
        let uniform_note = if self.is_uniform {
            format!(
                "This is the ordinary function type {} → {}.",
                self.base, self.fiber_over
            )
        } else {
            "This is a proper dependent function type.".to_string()
        };
        format!(
            "Π-type Π(x : {A}), {B}(x): \
             the type of dependent functions; \
             introduction: λ(x:{A}), t(x) : Π(x:{A}), {B}(x); \
             elimination: f a : {B}(a) for a : {A}; \
             computation: (λ(x:{A}), t(x)) a ≡ t(a) [β-reduction]. \
             {note}",
            A = self.base,
            B = self.fiber_over,
            note = uniform_note,
        )
    }
    /// The Σ-type (dependent pair / sum type).
    ///
    /// Σ(x : A), B(x) is the type of dependent pairs (a, b) with a : A, b : B(a).
    /// This is also called the dependent sum or existential type.
    pub fn sigma_type(&self) -> String {
        format!(
            "Σ-type Σ(x : {A}), {B}(x): \
             the type of dependent pairs; \
             introduction: (a, b) : Σ(x:{A}), {B}(x) for a:{A}, b:{B}(a); \
             elimination: ind-Σ : ∀(C : Σ(x:{A}), {B}(x) → U), \
             (∀(a:{A})(b:{B}(a)), C(a,b)) → ∀(p : Σ(x:{A}), {B}(x)), C(p); \
             computation: ind-Σ C c (a, b) ≡ c a b [β-reduction].",
            A = self.base,
            B = self.fiber_over
        )
    }
    /// Whether the dependent type is fibrant (all fibers are types, not props).
    pub fn is_fibrant(&self) -> bool {
        true
    }
}
/// Universe polymorphism in type theory.
///
/// Universe polymorphism allows definitions and theorems to be stated
/// uniformly for types at any universe level. This avoids universe
/// bumping and code duplication.
///
/// In CIC with universes: Type₀ : Type₁ : Type₂ : ...
/// Universe polymorphism: definitions are parameterized by universe levels u, v, ...
pub struct UniversePolymorphism {
    /// The universe levels used in this polymorphic definition.
    pub levels: Vec<u32>,
    /// Whether the universe hierarchy is cumulative.
    pub is_cumulative: bool,
    /// Whether level inference is available.
    pub has_level_inference: bool,
}
impl UniversePolymorphism {
    /// Create a universe polymorphic definition with the given levels.
    pub fn new(levels: Vec<u32>) -> Self {
        Self {
            levels,
            is_cumulative: true,
            has_level_inference: true,
        }
    }
    /// Single-universe polymorphism (most common case).
    pub fn single() -> Self {
        Self::new(vec![0])
    }
    /// The cumulativity property: Type_n is a subtype of Type_{n+1}.
    ///
    /// Cumulativity: if A : Type_n then A : Type_{n+1} (and every higher level).
    /// This is implemented via a coercion or a subtyping rule.
    pub fn cumulativity(&self) -> String {
        if self.is_cumulative {
            let top = self.levels.iter().max().copied().unwrap_or(0);
            format!(
                "Cumulativity: Type₀ ⊆ Type₁ ⊆ ... ⊆ Type_{} ⊆ ... \
                 If A : Type_n then A : Type_{{n+1}} (by the cumul rule). \
                 Used levels in this definition: {:?}. \
                 Cumulativity avoids repeated lifting of types through the hierarchy.",
                top, self.levels
            )
        } else {
            "Non-cumulative: types at different levels are distinct (Russell-style).".to_string()
        }
    }
    /// Universe level lifting: lifting a type A : Type_n to A : Type_{n+1}.
    ///
    /// Level lifting (also called ULift or universe lifting) is the operation
    /// that moves a type to a higher universe level without changing its elements.
    pub fn level_lifting(&self) -> String {
        format!(
            "Level lifting (ULift): For each universe level n, \
             ULift_n : Type_n → Type_{{n+1}} embeds types into the next universe. \
             ULift A = {{ down : A }} with ULift.up : A → ULift A and ULift.down : ULift A → A. \
             Used in universe polymorphic definitions to handle levels {:?} uniformly.",
            self.levels
        )
    }
    /// The number of universe levels in this polymorphic definition.
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }
    /// Whether this definition is universe-monomorphic (single fixed level).
    pub fn is_monomorphic(&self) -> bool {
        self.levels.len() == 1
    }
}
/// The Calculus of Inductive Constructions (CIC).
///
/// CIC extends the CoC with:
/// - A predicative hierarchy of universes: Prop : Type₀ : Type₁ : ...
/// - Inductive types with their eliminators
/// - The impredicative universe Prop (for logical propositions)
///
/// CIC is the theoretical foundation of the Coq proof assistant.
pub struct CIC {
    /// The universe names (e.g. \["Prop", "Type₀", "Type₁", ...\]).
    pub universes: Vec<String>,
    /// Whether the Prop universe is impredicative.
    pub has_impredicative_prop: bool,
    /// Whether proof irrelevance holds for Prop.
    pub proof_irrelevance: bool,
}
impl CIC {
    /// Create the standard CIC (as in Coq).
    pub fn standard() -> Self {
        Self {
            universes: vec![
                "Prop".to_string(),
                "Type₀".to_string(),
                "Type₁".to_string(),
                "Type₂".to_string(),
            ],
            has_impredicative_prop: true,
            proof_irrelevance: true,
        }
    }
    /// Create CIC with the given universe hierarchy.
    pub fn new(universes: Vec<String>) -> Self {
        Self {
            universes,
            has_impredicative_prop: true,
            proof_irrelevance: true,
        }
    }
    /// The CIC type system: rules, universes, and inductive types.
    ///
    /// CIC extends the Pure Type System (PTS) λPω with inductive types.
    /// The typing judgment Γ ⊢ t : T is defined by:
    /// - Sort rules (Prop : Type₀ : Type₁ : ...)
    /// - Var, App, Abs, Prod rules (from λPω)
    /// - Inductive type rules (ind-types and their eliminators)
    pub fn calculus_of_inductive_constructions(&self) -> String {
        let levels: Vec<String> = self.universes.iter().take(4).cloned().collect();
        format!(
            "CIC (Calculus of Inductive Constructions): \
             Universe hierarchy: {} ⊂ ... (cumulative). \
             {} \
             Inductive types: every strictly positive type scheme is permitted. \
             Eliminators: Prop-elim restricted to Prop (proof irrelevance: {}). \
             Founded on the Pure Type System λPω + inductive schemes.",
            levels.join(" : "),
            if self.has_impredicative_prop {
                "Prop is impredicative: ∀(P : Prop → Prop), P is in Prop."
            } else {
                "No impredicativity: all universes are predicative."
            },
            self.proof_irrelevance
        )
    }
    /// Whether CIC (with Prop) is logically consistent.
    ///
    /// CIC is consistent relative to ZFC + infinitely many inaccessible cardinals.
    /// The key consistency result is Werner's set-theoretic model (1994).
    pub fn is_consistent(&self) -> bool {
        self.has_impredicative_prop || !self.universes.is_empty()
    }
    /// The Werner model: a set-theoretic model of CIC.
    pub fn werner_model(&self) -> String {
        "Werner model (1994): CIC has a set-theoretic model in ZFC + \
         ω inaccessible cardinals. The model interprets universes as \
         Grothendieck universes and inductive types as W-types. \
         Consistency of CIC follows from consistency of ZFC + ω inaccessibles."
            .to_string()
    }
    /// The principal typing property: every well-typed term has a unique minimal type.
    pub fn has_principal_typing(&self) -> bool {
        true
    }
}
/// Checker for observational equality in OTT.
///
/// In Observational Type Theory (Altenkirch-McBride 2006), equality is defined
/// by observation: two values are equal iff they cannot be distinguished by any
/// context. For functions, a = b iff ∀x, a x = b x (functional extensionality
/// holds definitionally).
#[derive(Debug, Clone)]
pub struct ObservationalEqualityChecker {
    /// Whether heterogeneous equality (John Major equality) is enabled.
    pub heterogeneous: bool,
    /// Whether congruence closure is used to propagate equalities.
    pub use_congruence_closure: bool,
    /// Equality assertions recorded as (lhs_repr, rhs_repr) string pairs.
    pub equalities: Vec<(String, String)>,
}
impl ObservationalEqualityChecker {
    /// Create a new checker (homogeneous, with congruence closure).
    pub fn new() -> Self {
        Self {
            heterogeneous: false,
            use_congruence_closure: true,
            equalities: Vec::new(),
        }
    }
    /// Enable heterogeneous equality (a : A ≅ b : B).
    pub fn with_heterogeneous(mut self) -> Self {
        self.heterogeneous = true;
        self
    }
    /// Record an equality assertion `lhs = rhs`.
    pub fn assert_equal(&mut self, lhs: impl Into<String>, rhs: impl Into<String>) {
        self.equalities.push((lhs.into(), rhs.into()));
    }
    /// Check if two string representations are directly equal (syntactic check).
    pub fn is_equal(&self, lhs: &str, rhs: &str) -> bool {
        lhs == rhs
            || self
                .equalities
                .iter()
                .any(|(l, r)| (l == lhs && r == rhs) || (l == rhs && r == lhs))
    }
    /// Functorial congruence: if a = b and f is a type former, then f(a) = f(b).
    pub fn congruence(&self, former: &str, lhs: &str, rhs: &str) -> String {
        if self.is_equal(lhs, rhs) {
            format!("{former}({lhs}) = {former}({rhs}) [by congruence]")
        } else {
            format!("cannot conclude {former}({lhs}) = {former}({rhs}): {lhs} ≠ {rhs}")
        }
    }
    /// Number of recorded equality assumptions.
    pub fn num_equalities(&self) -> usize {
        self.equalities.len()
    }
}
/// The structural rules for type theory contexts.
///
/// The four structural rules governing contexts and variable usage:
/// - Variable rule: use a variable in scope
/// - Weakening: add unused assumptions
/// - Substitution: substitute equal terms
/// - Exchange: reorder assumptions (with care for dependencies)
pub struct VarRules {
    /// Whether the theory supports dependent types (affects exchange).
    pub is_dependent: bool,
    /// Whether proof irrelevance is used (affects substitution).
    pub proof_irrelevance: bool,
}
impl VarRules {
    /// Create variable rules for a dependent type theory.
    pub fn dependent() -> Self {
        Self {
            is_dependent: true,
            proof_irrelevance: false,
        }
    }
    /// Create variable rules for a simple (non-dependent) type theory.
    pub fn simple() -> Self {
        Self {
            is_dependent: false,
            proof_irrelevance: false,
        }
    }
    /// Create VarRules with explicit settings.
    pub fn new() -> Self {
        Self {
            is_dependent: true,
            proof_irrelevance: false,
        }
    }
    /// The variable rule (Var): a variable is typeable by its declared type.
    ///
    /// Var: Γ, x : A ⊢ x : A
    /// (Any variable in context can be used at its declared type.)
    pub fn variable_rule(&self) -> String {
        "Variable rule (Var): \
         If x : A ∈ Γ (Γ is a valid context containing x : A), \
         then Γ ⊢ x : A. \
         This is the axiom that allows using hypotheses in the context."
            .to_string()
    }
    /// The weakening rule: unused assumptions can be added.
    ///
    /// Weak: If Γ ⊢ t : A and Γ ⊢ B : Type, then Γ, x : B ⊢ t : A.
    pub fn weakening(&self) -> String {
        "Weakening rule (Weak): \
         If Γ ⊢ t : A and Γ ⊢ B : Type, then Γ, x : B ⊢ t : A. \
         Adding unused hypotheses does not invalidate a judgment. \
         In dependent type theory, weakening is admissible (derivable)."
            .to_string()
    }
    /// The substitution rule: equal terms can be substituted.
    ///
    /// Subst: If Γ ⊢ t : A and Γ, x : A ⊢ u : B,
    ///        then Γ ⊢ u[t/x] : B[t/x].
    pub fn substitution(&self) -> String {
        "Substitution rule (Subst): \
         If Γ ⊢ t : A and Γ, x : A, Δ ⊢ u : B, \
         then Γ, Δ[t/x] ⊢ u[t/x] : B[t/x]. \
         Substituting a well-typed term for a variable preserves typability. \
         This is the key rule enabling β-reduction of application."
            .to_string()
    }
    /// The exchange rule: independent assumptions can be reordered.
    ///
    /// Exchange: If Γ, x : A, y : B, Δ ⊢ t : C  and  x ∉ FV(B),
    ///           then Γ, y : B, x : A, Δ ⊢ t : C.
    /// (In dependent type theory, exchange only holds for independent variables.)
    pub fn exchange(&self) -> String {
        if self.is_dependent {
            "Exchange rule (for dependent types): \
             If Γ, x : A, y : B, Δ ⊢ t : C and x ∉ FV(B), \
             then Γ, y : B, x : A, Δ[swap] ⊢ t[swap] : C[swap]. \
             Exchange requires independence: x must not appear free in B. \
             In dependent contexts, reordering may require proof-relevant conversions."
                .to_string()
        } else {
            "Exchange rule (simple types): \
             Γ, x : A, y : B, Δ ⊢ t : C implies Γ, y : B, x : A, Δ ⊢ t : C. \
             For simply-typed theories, exchange holds unconditionally."
                .to_string()
        }
    }
}
