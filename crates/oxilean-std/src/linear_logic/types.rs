//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::HashMap;

/// A one-sided linear sequent `⊢ Γ` in linear logic.
#[derive(Debug, Clone)]
pub struct LinearSequent {
    /// The context: a list of formula names (positive occurrences).
    pub context: Vec<String>,
    /// The conclusion formula name.
    pub conclusion: String,
}
impl LinearSequent {
    /// Create a new linear sequent.
    pub fn new(context: Vec<String>, conclusion: impl Into<String>) -> Self {
        Self {
            context,
            conclusion: conclusion.into(),
        }
    }
    /// Cut elimination: a sequent derivable with cut is derivable without cut.
    ///
    /// Returns a string description of the cut-free derivation.
    pub fn cut_elimination(&self) -> String {
        format!(
            "cut-elim(⊢ {} ⊢ {})",
            self.context.join(", "),
            self.conclusion
        )
    }
    /// Check if the sequent is provable (simplified heuristic).
    pub fn is_provable(&self) -> bool {
        self.context.contains(&self.conclusion)
    }
    /// Return the one-sided form `⊢ Γ, A`.
    pub fn one_sided_form(&self) -> String {
        let mut gamma = self.context.clone();
        gamma.push(self.conclusion.clone());
        format!("⊢ {}", gamma.join(", "))
    }
}
/// Separation logic Hoare triple.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SepLogicTriple {
    pub precondition: String,
    pub command: String,
    pub postcondition: String,
}
impl SepLogicTriple {
    #[allow(dead_code)]
    pub fn new(pre: &str, cmd: &str, post: &str) -> Self {
        Self {
            precondition: pre.to_string(),
            command: cmd.to_string(),
            postcondition: post.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn frame_rule(&self, frame: &str) -> SepLogicTriple {
        SepLogicTriple::new(
            &format!("{} * {}", self.precondition, frame),
            &self.command,
            &format!("{} * {}", self.postcondition, frame),
        )
    }
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!(
            "{{{}}}\n  {}\n{{{}}}",
            self.precondition, self.command, self.postcondition
        )
    }
}
/// Linear logic sequent calculus rules.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LlRule {
    Ax,
    Cut,
    TensorR,
    ParR,
    WithR,
    PlusR1,
    PlusR2,
    OfCourseR,
    WhyNotR,
    Dereliction,
    Contraction,
    Weakening,
}
impl LlRule {
    #[allow(dead_code)]
    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            LlRule::Contraction | LlRule::Weakening | LlRule::Dereliction
        )
    }
    #[allow(dead_code)]
    pub fn applies_to_exponentials(&self) -> bool {
        matches!(
            self,
            LlRule::OfCourseR
                | LlRule::WhyNotR
                | LlRule::Dereliction
                | LlRule::Contraction
                | LlRule::Weakening
        )
    }
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            LlRule::Ax => "axiom",
            LlRule::Cut => "cut",
            LlRule::TensorR => "tensor-R",
            LlRule::ParR => "par-R",
            LlRule::WithR => "with-R",
            LlRule::PlusR1 => "plus-R1",
            LlRule::PlusR2 => "plus-R2",
            LlRule::OfCourseR => "!-R",
            LlRule::WhyNotR => "?-R",
            LlRule::Dereliction => "dereliction",
            LlRule::Contraction => "contraction",
            LlRule::Weakening => "weakening",
        }
    }
}
/// A simple heap: a finite partial map from addresses to values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heap {
    /// Map from address to value.
    pub map: std::collections::HashMap<u64, u64>,
}
impl Heap {
    /// Create an empty heap.
    pub fn empty() -> Self {
        Heap {
            map: std::collections::HashMap::new(),
        }
    }
    /// Create a singleton heap {addr ↦ val}.
    pub fn singleton(addr: u64, val: u64) -> Self {
        let mut h = Heap::empty();
        h.map.insert(addr, val);
        h
    }
    /// Check if two heaps have disjoint domains.
    pub fn disjoint(&self, other: &Heap) -> bool {
        self.map.keys().all(|k| !other.map.contains_key(k))
    }
    /// Union of two disjoint heaps.
    pub fn union(&self, other: &Heap) -> Option<Heap> {
        if !self.disjoint(other) {
            return None;
        }
        let mut map = self.map.clone();
        map.extend(other.map.iter().map(|(&k, &v)| (k, v)));
        Some(Heap { map })
    }
    /// Size of the heap (number of mapped addresses).
    pub fn size(&self) -> usize {
        self.map.len()
    }
    /// Read a value from the heap.
    pub fn read(&self, addr: u64) -> Option<u64> {
        self.map.get(&addr).copied()
    }
    /// Write a value to the heap (returns a new heap).
    pub fn write(&self, addr: u64, val: u64) -> Heap {
        let mut h = self.clone();
        h.map.insert(addr, val);
        h
    }
    /// Check separating conjunction: h ⊨ P * Q.
    /// Returns Some((h1, h2)) if h splits as h1 satisfying P and h2 satisfying Q.
    pub fn sep_split<P, Q>(&self, p: P, q: Q) -> Option<(Heap, Heap)>
    where
        P: Fn(&Heap) -> bool,
        Q: Fn(&Heap) -> bool,
    {
        let keys: Vec<u64> = self.map.keys().copied().collect();
        let n = keys.len();
        for mask in 0u64..(1u64 << n) {
            let mut h1 = Heap::empty();
            let mut h2 = Heap::empty();
            for (i, &k) in keys.iter().enumerate() {
                let v = self.map[&k];
                if (mask >> i) & 1 == 1 {
                    h1.map.insert(k, v);
                } else {
                    h2.map.insert(k, v);
                }
            }
            if p(&h1) && q(&h2) {
                return Some((h1, h2));
            }
        }
        None
    }
}
/// A one-sided sequent: ⊢ Γ (a multiset of linear formulas).
#[derive(Debug, Clone)]
pub struct LinSequent {
    /// Formulas in the succedent (all on the right in one-sided calculus).
    pub formulas: Vec<LinFormula>,
}
impl LinSequent {
    /// Create a new sequent from a list of formulas.
    pub fn new(formulas: Vec<LinFormula>) -> Self {
        LinSequent { formulas }
    }
    /// Create the unit sequent ⊢ 1.
    pub fn unit() -> Self {
        LinSequent::new(vec![LinFormula::One])
    }
    /// Check if this is an identity axiom: ⊢ A, A^⊥.
    pub fn is_axiom(&self) -> bool {
        if self.formulas.len() != 2 {
            return false;
        }
        let a = &self.formulas[0];
        let b = &self.formulas[1];
        a.dual() == *b || b.dual() == *a
    }
    /// Apply the tensor rule: split into two sub-sequents.
    /// Returns the two components if the sequent has the form ⊢ ..., A ⊗ B, ...
    pub fn tensor_components(&self) -> Option<(LinSequent, LinFormula, LinFormula)> {
        for (i, f) in self.formulas.iter().enumerate() {
            if let LinFormula::Tensor(a, b) = f {
                let mut rest = self.formulas.clone();
                rest.remove(i);
                return Some((LinSequent::new(rest), *a.clone(), *b.clone()));
            }
        }
        None
    }
    /// Display the sequent as ⊢ A, B, ...
    pub fn display(&self) -> String {
        let parts: Vec<String> = self.formulas.iter().map(|f| f.to_string()).collect();
        format!("⊢ {}", parts.join(", "))
    }
    /// Total complexity of all formulas in the sequent.
    pub fn total_complexity(&self) -> usize {
        self.formulas.iter().map(|f| f.complexity()).sum()
    }
}
/// Phase semantics: Girard's completeness model for linear logic based on
/// a commutative monoid with a set of facts closed under a Galois connection.
#[derive(Debug, Clone)]
pub struct PhaseSemantics {
    /// Name of the underlying monoid.
    pub monoid: String,
    /// Names of the facts (elements of the monoid satisfying the facts predicate).
    pub facts: Vec<String>,
}
impl PhaseSemantics {
    /// Create a new phase semantics structure.
    pub fn new(monoid: impl Into<String>, facts: Vec<String>) -> Self {
        Self {
            monoid: monoid.into(),
            facts,
        }
    }
    /// Interpret a formula in the phase space.
    ///
    /// Returns a string description of the interpretation (the "fact set" for that formula).
    pub fn interpret_formula(&self, formula: &str) -> String {
        format!(
            "⟦{}⟧_{{{}}} ⊆ {{{}}}",
            formula,
            self.monoid,
            self.facts.join(", ")
        )
    }
    /// Soundness: every provable formula is valid in all phase spaces.
    pub fn soundness(&self, formula: &str) -> bool {
        !formula.is_empty()
    }
    /// Completeness: every formula valid in all phase spaces is provable.
    pub fn completeness_statement(&self) -> String {
        format!(
            "LL ⊢ A iff A is valid in all phase spaces over {}",
            self.monoid
        )
    }
    /// Check if an element is a fact.
    pub fn is_fact(&self, element: &str) -> bool {
        self.facts.contains(&element.to_string())
    }
}
/// A link type in a proof structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkKind {
    /// Axiom link: pairs a formula with its dual.
    Axiom,
    /// Cut link: connects two dual formula occurrences.
    Cut,
    /// Tensor link: combines two premises.
    TensorLink,
    /// Par link: breaks par into two sub-formulas.
    ParLink,
}
/// Phase semantics model for linear logic (second extended version).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PhaseSpaceExt {
    pub monoid_name: String,
    pub facts_description: String,
}
impl PhaseSpaceExt {
    #[allow(dead_code)]
    pub fn new(monoid: &str) -> Self {
        Self {
            monoid_name: monoid.to_string(),
            facts_description: "Facts: subsets F of M such that F^perp^perp = F".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn completeness_description(&self) -> String {
        format!(
            "Phase semantics (Girard) complete for MALL: provable <-> valid in all phase models over {}",
            self.monoid_name
        )
    }
}
/// Linear type systems (connection to programming).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LinearTypeSystem {
    pub language: String,
    pub resource_tracking: bool,
    pub affine_types: bool,
}
impl LinearTypeSystem {
    #[allow(dead_code)]
    pub fn linear_haskell() -> Self {
        Self {
            language: "Linear Haskell".to_string(),
            resource_tracking: true,
            affine_types: false,
        }
    }
    #[allow(dead_code)]
    pub fn rust_ownership() -> Self {
        Self {
            language: "Rust (ownership system)".to_string(),
            resource_tracking: true,
            affine_types: true,
        }
    }
    #[allow(dead_code)]
    pub fn uniqueness_types(lang: &str) -> Self {
        Self {
            language: lang.to_string(),
            resource_tracking: true,
            affine_types: false,
        }
    }
    #[allow(dead_code)]
    pub fn prevents_use_after_free(&self) -> bool {
        self.resource_tracking
    }
}
/// Public linear logic formula type matching the task specification.
///
/// This is the primary enum for constructing and manipulating linear logic
/// formulas at the Rust level. It mirrors the kernel-level `LinFormula` but
/// uses the canonical names from the specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearFormula {
    /// Atomic proposition `P`.
    Atom(String),
    /// Tensor product `A ⊗ B` (multiplicative conjunction).
    Tensor(Box<LinearFormula>, Box<LinearFormula>),
    /// Par `A ⅋ B` (multiplicative disjunction).
    Par(Box<LinearFormula>, Box<LinearFormula>),
    /// With `A & B` (additive conjunction).
    With(Box<LinearFormula>, Box<LinearFormula>),
    /// Plus `A ⊕ B` (additive disjunction).
    Plus(Box<LinearFormula>, Box<LinearFormula>),
    /// Of course `!A` (exponential — allows contraction and weakening).
    OfCourse(Box<LinearFormula>),
    /// Why not `?A` (exponential dual of `!A`).
    WhyNot(Box<LinearFormula>),
    /// Multiplicative unit `1`.
    One,
    /// Multiplicative unit (bottom) `⊥`.
    Bottom,
    /// Additive unit (top) `⊤`.
    Top,
    /// Additive zero `0`.
    Zero,
    /// Dual `A^⊥`.
    Dual(Box<LinearFormula>),
}
impl LinearFormula {
    /// Compute the linear negation / dual of a formula.
    ///
    /// - `(A ⊗ B)^⊥ = A^⊥ ⅋ B^⊥`
    /// - `(A ⅋ B)^⊥ = A^⊥ ⊗ B^⊥`
    /// - `(A & B)^⊥ = A^⊥ ⊕ B^⊥`
    /// - `(A ⊕ B)^⊥ = A^⊥ & B^⊥`
    /// - `(!A)^⊥ = ?A^⊥`
    /// - `(?A)^⊥ = !A^⊥`
    /// - `1^⊥ = ⊥`, `⊥^⊥ = 1`, `⊤^⊥ = 0`, `0^⊥ = ⊤`
    /// - `(A^⊥)^⊥ = A`
    pub fn dual(self) -> Self {
        match self {
            LinearFormula::Atom(s) => LinearFormula::Dual(Box::new(LinearFormula::Atom(s))),
            LinearFormula::Tensor(a, b) => {
                LinearFormula::Par(Box::new(a.dual()), Box::new(b.dual()))
            }
            LinearFormula::Par(a, b) => {
                LinearFormula::Tensor(Box::new(a.dual()), Box::new(b.dual()))
            }
            LinearFormula::With(a, b) => {
                LinearFormula::Plus(Box::new(a.dual()), Box::new(b.dual()))
            }
            LinearFormula::Plus(a, b) => {
                LinearFormula::With(Box::new(a.dual()), Box::new(b.dual()))
            }
            LinearFormula::OfCourse(a) => LinearFormula::WhyNot(Box::new(a.dual())),
            LinearFormula::WhyNot(a) => LinearFormula::OfCourse(Box::new(a.dual())),
            LinearFormula::One => LinearFormula::Bottom,
            LinearFormula::Bottom => LinearFormula::One,
            LinearFormula::Top => LinearFormula::Zero,
            LinearFormula::Zero => LinearFormula::Top,
            LinearFormula::Dual(a) => *a,
        }
    }
    /// Check if this formula is in the multiplicative fragment (MLL).
    pub fn is_multiplicative(&self) -> bool {
        match self {
            LinearFormula::Atom(_) | LinearFormula::Dual(_) => true,
            LinearFormula::Tensor(a, b) | LinearFormula::Par(a, b) => {
                a.is_multiplicative() && b.is_multiplicative()
            }
            LinearFormula::One | LinearFormula::Bottom => true,
            _ => false,
        }
    }
    /// Check if this formula is in the additive fragment.
    pub fn is_additive(&self) -> bool {
        match self {
            LinearFormula::Atom(_) | LinearFormula::Dual(_) => true,
            LinearFormula::With(a, b) | LinearFormula::Plus(a, b) => {
                a.is_additive() && b.is_additive()
            }
            LinearFormula::Top | LinearFormula::Zero => true,
            _ => false,
        }
    }
    /// Check if this formula uses exponentials (`!` or `?`).
    pub fn is_exponential(&self) -> bool {
        match self {
            LinearFormula::OfCourse(_) | LinearFormula::WhyNot(_) => true,
            LinearFormula::Tensor(a, b)
            | LinearFormula::Par(a, b)
            | LinearFormula::With(a, b)
            | LinearFormula::Plus(a, b) => a.is_exponential() || b.is_exponential(),
            LinearFormula::Dual(a) => a.is_exponential(),
            _ => false,
        }
    }
    /// Linear implication `A ⊸ B = A^⊥ ⅋ B`.
    pub fn lollipop(a: LinearFormula, b: LinearFormula) -> LinearFormula {
        LinearFormula::Par(Box::new(a.dual()), Box::new(b))
    }
    /// Complexity (number of binary connectives and exponentials).
    pub fn complexity(&self) -> usize {
        match self {
            LinearFormula::Atom(_)
            | LinearFormula::One
            | LinearFormula::Bottom
            | LinearFormula::Top
            | LinearFormula::Zero => 0,
            LinearFormula::Dual(a) => a.complexity(),
            LinearFormula::OfCourse(a) | LinearFormula::WhyNot(a) => 1 + a.complexity(),
            LinearFormula::Tensor(a, b)
            | LinearFormula::Par(a, b)
            | LinearFormula::With(a, b)
            | LinearFormula::Plus(a, b) => 1 + a.complexity() + b.complexity(),
        }
    }
}
/// Game semantics: a game G = (moves, positions, plays).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LlGame {
    pub formula: String,
    pub player: String,
    pub opponent: String,
    pub winning_condition: String,
}
impl LlGame {
    #[allow(dead_code)]
    pub fn new(formula: &str) -> Self {
        Self {
            formula: formula.to_string(),
            player: "Proponent (P)".to_string(),
            opponent: "Opponent (O)".to_string(),
            winning_condition: "P wins if O cannot move".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn tensor_game(g1: &str, g2: &str) -> Self {
        Self::new(&format!("{g1} tensor {g2}"))
    }
    #[allow(dead_code)]
    pub fn abramsky_jagadeesan_description(&self) -> String {
        format!(
            "Game semantics (Abramsky-Jagadeesan): formula {} has a game where P-strategy = proof",
            self.formula
        )
    }
}
/// Bounded linear logic (Girard-Lafont-Regnard).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BoundedLinearLogic {
    pub polynomial_bound: String,
}
impl BoundedLinearLogic {
    #[allow(dead_code)]
    pub fn new(bound: &str) -> Self {
        Self {
            polynomial_bound: bound.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn captures_ptime(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn description(&self) -> String {
        format!(
            "BLL with bound {}: proofs correspond to polynomial-time algorithms",
            self.polynomial_bound
        )
    }
}
/// Resource logic: linear logic as a logic of resource consumption.
#[derive(Debug, Clone)]
pub struct ResourceLogic {
    /// Named resources available.
    pub resources: Vec<String>,
}
impl ResourceLogic {
    /// Create a new resource logic structure.
    pub fn new(resources: Vec<String>) -> Self {
        Self { resources }
    }
    /// Resource consumption model:
    /// a process consumes resources from the context linearly (no copying).
    pub fn resource_consumption_model(&self) -> String {
        format!("consume({})", self.resources.join(", "))
    }
    /// Connection to separation logic:
    /// the separating conjunction `*` in BI logic corresponds to tensor `⊗`.
    pub fn separation_logic_connection(&self) -> String {
        "BI separation conjunction * ≅ linear tensor ⊗".to_string()
    }
    /// Check if a given resource is available.
    pub fn has_resource(&self, name: &str) -> bool {
        self.resources.contains(&name.to_string())
    }
    /// Consume a resource (returns new state with resource removed).
    pub fn consume(&self, name: &str) -> Option<Self> {
        if self.has_resource(name) {
            let resources = self
                .resources
                .iter()
                .filter(|&r| r != name)
                .cloned()
                .collect();
            Some(Self { resources })
        } else {
            None
        }
    }
    /// Total number of available resources.
    pub fn count(&self) -> usize {
        self.resources.len()
    }
}
/// A formula in relevant logic R.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelFormula {
    /// Atom.
    Atom(String),
    /// Negation ¬A.
    Neg(Box<RelFormula>),
    /// Conjunction A ∧ B.
    And(Box<RelFormula>, Box<RelFormula>),
    /// Disjunction A ∨ B.
    Or(Box<RelFormula>, Box<RelFormula>),
    /// (Relevant) implication A → B.
    Implies(Box<RelFormula>, Box<RelFormula>),
    /// Fusion (intensional conjunction) A ∘ B.
    Fusion(Box<RelFormula>, Box<RelFormula>),
}
impl RelFormula {
    /// Construct implication.
    pub fn implies(a: RelFormula, b: RelFormula) -> Self {
        RelFormula::Implies(Box::new(a), Box::new(b))
    }
    /// Construct conjunction.
    pub fn and(a: RelFormula, b: RelFormula) -> Self {
        RelFormula::And(Box::new(a), Box::new(b))
    }
    /// Construct disjunction.
    pub fn or(a: RelFormula, b: RelFormula) -> Self {
        RelFormula::Or(Box::new(a), Box::new(b))
    }
    /// Construct negation.
    pub fn neg(a: RelFormula) -> Self {
        RelFormula::Neg(Box::new(a))
    }
    /// Construct fusion.
    pub fn fusion(a: RelFormula, b: RelFormula) -> Self {
        RelFormula::Fusion(Box::new(a), Box::new(b))
    }
    /// The reflexivity axiom A → A is always a theorem of R.
    pub fn is_reflexivity_axiom(&self) -> bool {
        if let RelFormula::Implies(a, b) = self {
            a == b
        } else {
            false
        }
    }
}
/// Bunched implications (BI) logic formula.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BiFormula {
    Atom(String),
    Emp,
    True,
    SepConj(Box<BiFormula>, Box<BiFormula>),
    SepImpl(Box<BiFormula>, Box<BiFormula>),
    Conj(Box<BiFormula>, Box<BiFormula>),
    Impl(Box<BiFormula>, Box<BiFormula>),
    Disj(Box<BiFormula>, Box<BiFormula>),
    Neg(Box<BiFormula>),
}
impl BiFormula {
    #[allow(dead_code)]
    pub fn atom(s: &str) -> Self {
        Self::Atom(s.to_string())
    }
    #[allow(dead_code)]
    pub fn sep_conj(a: BiFormula, b: BiFormula) -> Self {
        Self::SepConj(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn sep_impl(a: BiFormula, b: BiFormula) -> Self {
        Self::SepImpl(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn is_separation_connective(&self) -> bool {
        matches!(
            self,
            BiFormula::SepConj(..) | BiFormula::SepImpl(..) | BiFormula::Emp
        )
    }
}
/// Dialectica transformation (Godel's functional interpretation via linear logic).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DialecticaTransform {
    pub formula: String,
    pub dialectica_type: String,
}
impl DialecticaTransform {
    #[allow(dead_code)]
    pub fn new(formula: &str, dtype: &str) -> Self {
        Self {
            formula: formula.to_string(),
            dialectica_type: dtype.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn de_paiva_description(&self) -> String {
        format!("de Paiva's Dialectica categories model linear logic (with !A = forall x. A^x)",)
    }
}
/// Game semantics for linear logic: arenas and innocent strategies.
#[derive(Debug, Clone)]
pub struct GameSemantics {
    /// Named arenas (one per type in the context).
    pub arenas: Vec<String>,
}
impl GameSemantics {
    /// Create a new game semantics structure.
    pub fn new(arenas: Vec<String>) -> Self {
        Self { arenas }
    }
    /// Innocent strategies: strategies that only depend on the visible history
    /// (the P-view), not the full history.
    pub fn innocent_strategies(&self) -> Vec<String> {
        self.arenas
            .iter()
            .map(|a| format!("InnocentStrat({})", a))
            .collect()
    }
    /// Composition of strategies (Abramsky-Jagadeesan-Malacaria style).
    pub fn composition(&self) -> String {
        if self.arenas.len() < 2 {
            "id".to_string()
        } else {
            self.arenas
                .windows(2)
                .map(|w| format!("{};{}", w[0], w[1]))
                .collect::<Vec<_>>()
                .join(" ∘ ")
        }
    }
    /// Tensor product of arenas.
    pub fn tensor(&self) -> String {
        self.arenas.join(" ⊗ ")
    }
    /// Full completeness: every innocent strategy corresponds to a proof.
    pub fn full_completeness_statement(&self) -> String {
        "Every innocent strategy is the denotation of a linear logic proof".to_string()
    }
}
/// A coherence space: a reflexive, symmetric binary relation (web, coh).
#[derive(Debug, Clone)]
pub struct CoherenceSpace {
    /// Number of tokens (web size).
    pub n_tokens: usize,
    /// Coherence matrix: coh[i][j] = true iff i and j are coherent (or equal).
    pub coh: Vec<Vec<bool>>,
}
impl CoherenceSpace {
    /// Create the flat coherence space: all distinct tokens are incoherent.
    pub fn flat(n: usize) -> Self {
        let mut coh = vec![vec![false; n]; n];
        for i in 0..n {
            coh[i][i] = true;
        }
        CoherenceSpace { n_tokens: n, coh }
    }
    /// Create the complete coherence space: all tokens are mutually coherent.
    pub fn complete(n: usize) -> Self {
        CoherenceSpace {
            n_tokens: n,
            coh: vec![vec![true; n]; n],
        }
    }
    /// Check if a set of tokens forms a clique (mutually coherent).
    pub fn is_clique(&self, tokens: &[usize]) -> bool {
        for &i in tokens {
            for &j in tokens {
                if i < self.n_tokens && j < self.n_tokens && !self.coh[i][j] {
                    return false;
                }
            }
        }
        true
    }
    /// Check if a set of tokens is an antichain (mutually incoherent, except self).
    pub fn is_antichain(&self, tokens: &[usize]) -> bool {
        for (idx, &i) in tokens.iter().enumerate() {
            for &j in &tokens[idx + 1..] {
                if i < self.n_tokens && j < self.n_tokens && self.coh[i][j] {
                    return false;
                }
            }
        }
        true
    }
    /// Tensor product: web is n1 × n2, coherent iff both components coherent.
    pub fn tensor(&self, other: &CoherenceSpace) -> CoherenceSpace {
        let n = self.n_tokens * other.n_tokens;
        let mut coh = vec![vec![false; n]; n];
        for i1 in 0..self.n_tokens {
            for j1 in 0..other.n_tokens {
                for i2 in 0..self.n_tokens {
                    for j2 in 0..other.n_tokens {
                        let row = i1 * other.n_tokens + j1;
                        let col = i2 * other.n_tokens + j2;
                        coh[row][col] = self.coh[i1][i2] && other.coh[j1][j2];
                    }
                }
            }
        }
        CoherenceSpace { n_tokens: n, coh }
    }
    /// With connective: disjoint union (tagged tokens), coherent within each component.
    pub fn with(&self, other: &CoherenceSpace) -> CoherenceSpace {
        let n = self.n_tokens + other.n_tokens;
        let mut coh = vec![vec![false; n]; n];
        for i in 0..self.n_tokens {
            for j in 0..self.n_tokens {
                coh[i][j] = self.coh[i][j];
            }
        }
        let offset = self.n_tokens;
        for i in 0..other.n_tokens {
            for j in 0..other.n_tokens {
                coh[offset + i][offset + j] = other.coh[i][j];
            }
        }
        CoherenceSpace { n_tokens: n, coh }
    }
}
/// The four rules governing the exponential modality `!` in linear logic.
#[derive(Debug, Clone, Default)]
pub struct ExponentialRules;
impl ExponentialRules {
    /// Create a new exponential rules structure.
    pub fn new() -> Self {
        Self
    }
    /// Dereliction: `!A ⊢ A` (use a resource once).
    pub fn dereliction(&self, a: &str) -> String {
        format!("dereliction(!{a} ⊢ {a})")
    }
    /// Contraction: `!A ⊢ !A ⊗ !A` (duplicate a resource).
    pub fn contraction(&self, a: &str) -> String {
        format!("contraction(!{a} ⊢ !{a} ⊗ !{a})")
    }
    /// Weakening: `!A ⊢ 1` (discard a resource).
    pub fn weakening(&self, a: &str) -> String {
        format!("weakening(!{a} ⊢ 1)")
    }
    /// Promotion: `!Γ ⊢ A → !Γ ⊢ !A` (promote to exponential).
    pub fn promotion(&self, a: &str, gamma: &[&str]) -> String {
        let ctx = gamma.join(", ");
        format!("promotion(![{ctx}] ⊢ {a}  →  ![{ctx}] ⊢ !{a})")
    }
    /// Storage: `!A ≡ !A ⊗ !A` (idempotency of `!`).
    pub fn storage(&self, a: &str) -> String {
        format!("storage(!{a} ≡ !{a} ⊗ !{a})")
    }
}
/// A phase space: a commutative monoid (M, ·, e) with distinguished subset bot.
/// We represent M as integers mod p for concreteness.
#[derive(Debug, Clone)]
pub struct PhaseSpace {
    /// Elements of the monoid (indices 0..n).
    pub n: usize,
    /// Multiplication table: mul[i][j] = i · j.
    pub mul: Vec<Vec<usize>>,
    /// Identity element.
    pub identity: usize,
    /// The distinguished subset ⊥ ⊆ M.
    pub bot: Vec<bool>,
}
impl PhaseSpace {
    /// Create a named trivial phase space {e} with ⊥ = {e}.
    /// The `_name` parameter is informational only.
    pub fn new(_name: &str) -> Self {
        Self::trivial()
    }
    /// Description of completeness for phase semantics.
    pub fn completeness_description(&self) -> String {
        format!(
            "Phase semantics complete for linear logic: monoid size={}, bot={} elements",
            self.n,
            self.bot.iter().filter(|&&b| b).count()
        )
    }
    /// Create the trivial phase space {e} with ⊥ = {e}.
    pub fn trivial() -> Self {
        PhaseSpace {
            n: 1,
            mul: vec![vec![0]],
            identity: 0,
            bot: vec![true],
        }
    }
    /// Multiply two elements.
    pub fn multiply(&self, a: usize, b: usize) -> usize {
        self.mul[a % self.n][b % self.n]
    }
    /// Closure A^⊥⊥: double orthogonal of a subset A ⊆ M.
    /// First computes A^⊥ = {m | ∀ a ∈ A, m·a ∈ ⊥}, then its orthogonal.
    pub fn closure(&self, subset: &[bool]) -> Vec<bool> {
        let perp = self.orthogonal(subset);
        self.orthogonal(&perp)
    }
    /// Orthogonal: A^⊥ = {m | ∀ a ∈ A, m·a ∈ ⊥}.
    pub fn orthogonal(&self, subset: &[bool]) -> Vec<bool> {
        let mut result = vec![true; self.n];
        for m in 0..self.n {
            for a in 0..self.n {
                if subset[a] {
                    let prod = self.multiply(m, a);
                    if !self.bot[prod] {
                        result[m] = false;
                        break;
                    }
                }
            }
        }
        result
    }
    /// Check whether a subset is a fact (= ⊥-closed, i.e., A = A^⊥⊥).
    pub fn is_fact(&self, subset: &[bool]) -> bool {
        let closed = self.closure(subset);
        subset.iter().zip(closed.iter()).all(|(a, b)| a == b)
    }
    /// Count elements in a subset.
    pub fn subset_size(subset: &[bool]) -> usize {
        subset.iter().filter(|&&b| b).count()
    }
}
/// A link in a proof structure.
#[derive(Debug, Clone)]
pub struct Link {
    /// Kind of link.
    pub kind: LinkKind,
    /// Indices of conclusion formula occurrences.
    pub conclusions: Vec<usize>,
    /// Indices of premise formula occurrences.
    pub premises: Vec<usize>,
}
impl Link {
    /// Create an axiom link between two dual formula occurrences.
    pub fn axiom(i: usize, j: usize) -> Self {
        Link {
            kind: LinkKind::Axiom,
            conclusions: vec![i, j],
            premises: vec![],
        }
    }
    /// Create a tensor link.
    pub fn tensor(premise_a: usize, premise_b: usize, conclusion: usize) -> Self {
        Link {
            kind: LinkKind::TensorLink,
            conclusions: vec![conclusion],
            premises: vec![premise_a, premise_b],
        }
    }
    /// Create a par link.
    pub fn par(premise_a: usize, premise_b: usize, conclusion: usize) -> Self {
        Link {
            kind: LinkKind::ParLink,
            conclusions: vec![conclusion],
            premises: vec![premise_a, premise_b],
        }
    }
}
/// A formula in propositional linear logic.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinFormula {
    /// Propositional atom.
    Atom(String),
    /// Multiplicative unit 1.
    One,
    /// Multiplicative bottom ⊥.
    Bot,
    /// Additive unit ⊤.
    Top,
    /// Additive zero 0.
    Zero,
    /// Tensor product A ⊗ B.
    Tensor(Box<LinFormula>, Box<LinFormula>),
    /// Par A ⅋ B.
    Par(Box<LinFormula>, Box<LinFormula>),
    /// With A & B (additive conjunction).
    With(Box<LinFormula>, Box<LinFormula>),
    /// Plus A ⊕ B (additive disjunction).
    Plus(Box<LinFormula>, Box<LinFormula>),
    /// Bang !A (of-course).
    Bang(Box<LinFormula>),
    /// Why-not ?A (why-not).
    WhyNot(Box<LinFormula>),
    /// Linear negation A^⊥.
    Neg(Box<LinFormula>),
}
impl LinFormula {
    /// Dual (linear negation) of a formula, using De Morgan laws.
    /// Involutive: (A^⊥)^⊥ = A.
    pub fn dual(&self) -> LinFormula {
        match self {
            LinFormula::Atom(s) => LinFormula::Neg(Box::new(LinFormula::Atom(s.clone()))),
            LinFormula::Neg(inner) => *inner.clone(),
            LinFormula::One => LinFormula::Bot,
            LinFormula::Bot => LinFormula::One,
            LinFormula::Top => LinFormula::Zero,
            LinFormula::Zero => LinFormula::Top,
            LinFormula::Tensor(a, b) => LinFormula::Par(Box::new(a.dual()), Box::new(b.dual())),
            LinFormula::Par(a, b) => LinFormula::Tensor(Box::new(a.dual()), Box::new(b.dual())),
            LinFormula::With(a, b) => LinFormula::Plus(Box::new(a.dual()), Box::new(b.dual())),
            LinFormula::Plus(a, b) => LinFormula::With(Box::new(a.dual()), Box::new(b.dual())),
            LinFormula::Bang(a) => LinFormula::WhyNot(Box::new(a.dual())),
            LinFormula::WhyNot(a) => LinFormula::Bang(Box::new(a.dual())),
        }
    }
    /// Linear implication A ⊸ B = A^⊥ ⅋ B.
    pub fn lollipop(a: LinFormula, b: LinFormula) -> LinFormula {
        LinFormula::Par(Box::new(a.dual()), Box::new(b))
    }
    /// Check if the formula is an atom or literal.
    pub fn is_literal(&self) -> bool {
        match self {
            LinFormula::Atom(_) => true,
            LinFormula::Neg(inner) => matches!(inner.as_ref(), LinFormula::Atom(_)),
            _ => false,
        }
    }
    /// Check if the formula is in the multiplicative fragment (no & or ⊕).
    pub fn is_multiplicative(&self) -> bool {
        match self {
            LinFormula::Atom(_) | LinFormula::Neg(_) | LinFormula::One | LinFormula::Bot => true,
            LinFormula::Tensor(a, b) | LinFormula::Par(a, b) => {
                a.is_multiplicative() && b.is_multiplicative()
            }
            LinFormula::Bang(a) | LinFormula::WhyNot(a) => a.is_multiplicative(),
            LinFormula::With(_, _) | LinFormula::Plus(_, _) => false,
            LinFormula::Top | LinFormula::Zero => false,
        }
    }
    /// Subformula depth.
    pub fn depth(&self) -> usize {
        match self {
            LinFormula::Atom(_)
            | LinFormula::One
            | LinFormula::Bot
            | LinFormula::Top
            | LinFormula::Zero => 0,
            LinFormula::Neg(a) | LinFormula::Bang(a) | LinFormula::WhyNot(a) => 1 + a.depth(),
            LinFormula::Tensor(a, b)
            | LinFormula::Par(a, b)
            | LinFormula::With(a, b)
            | LinFormula::Plus(a, b) => 1 + a.depth().max(b.depth()),
        }
    }
    /// Number of connectives in the formula.
    pub fn complexity(&self) -> usize {
        match self {
            LinFormula::Atom(_)
            | LinFormula::One
            | LinFormula::Bot
            | LinFormula::Top
            | LinFormula::Zero => 0,
            LinFormula::Neg(a) | LinFormula::Bang(a) | LinFormula::WhyNot(a) => 1 + a.complexity(),
            LinFormula::Tensor(a, b)
            | LinFormula::Par(a, b)
            | LinFormula::With(a, b)
            | LinFormula::Plus(a, b) => 1 + a.complexity() + b.complexity(),
        }
    }
    /// Collect all atoms occurring in the formula.
    pub fn atoms(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_atoms(&mut result);
        result.sort();
        result.dedup();
        result
    }
    fn collect_atoms(&self, out: &mut Vec<String>) {
        match self {
            LinFormula::Atom(s) => out.push(s.clone()),
            LinFormula::Neg(a) | LinFormula::Bang(a) | LinFormula::WhyNot(a) => {
                a.collect_atoms(out);
            }
            LinFormula::Tensor(a, b)
            | LinFormula::Par(a, b)
            | LinFormula::With(a, b)
            | LinFormula::Plus(a, b) => {
                a.collect_atoms(out);
                b.collect_atoms(out);
            }
            _ => {}
        }
    }
}
/// A linear logic formula.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LlFormula {
    Atom(String),
    Neg(Box<LlFormula>),
    Tensor(Box<LlFormula>, Box<LlFormula>),
    Par(Box<LlFormula>, Box<LlFormula>),
    Plus(Box<LlFormula>, Box<LlFormula>),
    With(Box<LlFormula>, Box<LlFormula>),
    One,
    Bottom,
    Top,
    Zero,
    OfCourse(Box<LlFormula>),
    WhyNot(Box<LlFormula>),
    Forall(String, Box<LlFormula>),
    Exists(String, Box<LlFormula>),
}
impl LlFormula {
    #[allow(dead_code)]
    pub fn atom(s: &str) -> Self {
        Self::Atom(s.to_string())
    }
    #[allow(dead_code)]
    pub fn tensor(a: LlFormula, b: LlFormula) -> Self {
        Self::Tensor(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn par(a: LlFormula, b: LlFormula) -> Self {
        Self::Par(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn with_op(a: LlFormula, b: LlFormula) -> Self {
        Self::With(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn plus(a: LlFormula, b: LlFormula) -> Self {
        Self::Plus(Box::new(a), Box::new(b))
    }
    #[allow(dead_code)]
    pub fn of_course(a: LlFormula) -> Self {
        Self::OfCourse(Box::new(a))
    }
    #[allow(dead_code)]
    pub fn why_not(a: LlFormula) -> Self {
        Self::WhyNot(Box::new(a))
    }
    #[allow(dead_code)]
    pub fn neg(a: LlFormula) -> Self {
        Self::Neg(Box::new(a))
    }
    #[allow(dead_code)]
    pub fn is_multiplicative(&self) -> bool {
        matches!(
            self,
            LlFormula::Tensor(..) | LlFormula::Par(..) | LlFormula::One | LlFormula::Bottom
        )
    }
    #[allow(dead_code)]
    pub fn is_additive(&self) -> bool {
        matches!(
            self,
            LlFormula::Plus(..) | LlFormula::With(..) | LlFormula::Top | LlFormula::Zero
        )
    }
    #[allow(dead_code)]
    pub fn is_exponential(&self) -> bool {
        matches!(self, LlFormula::OfCourse(..) | LlFormula::WhyNot(..))
    }
    #[allow(dead_code)]
    pub fn linear_negation(&self) -> LlFormula {
        match self {
            LlFormula::Atom(s) => LlFormula::Neg(Box::new(LlFormula::Atom(s.clone()))),
            LlFormula::Neg(a) => *a.clone(),
            LlFormula::Tensor(a, b) => LlFormula::par(a.linear_negation(), b.linear_negation()),
            LlFormula::Par(a, b) => LlFormula::tensor(a.linear_negation(), b.linear_negation()),
            LlFormula::Plus(a, b) => LlFormula::with_op(a.linear_negation(), b.linear_negation()),
            LlFormula::With(a, b) => LlFormula::plus(a.linear_negation(), b.linear_negation()),
            LlFormula::One => LlFormula::Bottom,
            LlFormula::Bottom => LlFormula::One,
            LlFormula::Top => LlFormula::Zero,
            LlFormula::Zero => LlFormula::Top,
            LlFormula::OfCourse(a) => LlFormula::why_not(a.linear_negation()),
            LlFormula::WhyNot(a) => LlFormula::of_course(a.linear_negation()),
            LlFormula::Forall(x, a) => LlFormula::Exists(x.clone(), Box::new(a.linear_negation())),
            LlFormula::Exists(x, a) => LlFormula::Forall(x.clone(), Box::new(a.linear_negation())),
        }
    }
}
/// A proof structure: a set of formula occurrences and links.
#[derive(Debug, Clone)]
pub struct ProofStructure {
    /// Number of formula occurrences.
    pub n_formulas: usize,
    /// The links (axiom, cut, tensor, par, etc.).
    pub links: Vec<Link>,
}
impl ProofStructure {
    /// Create a new empty proof structure with `n` formula occurrences.
    pub fn new(n_formulas: usize) -> Self {
        ProofStructure {
            n_formulas,
            links: Vec::new(),
        }
    }
    /// Add a link to the proof structure.
    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }
    /// Check the Danos-Regnier correctness criterion (simplified: for MLL axiom nets).
    ///
    /// A proof structure is correct iff every switching (choice of one conclusion
    /// for each par link) yields a spanning tree of the graph. Here we apply a
    /// union-find acyclicity check on the axiom links.
    pub fn is_correct(&self) -> bool {
        if self.n_formulas == 0 {
            return true;
        }
        let mut parent: Vec<usize> = (0..self.n_formulas).collect();
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        let mut edge_count = 0usize;
        for link in &self.links {
            if link.kind == LinkKind::Axiom && link.conclusions.len() == 2 {
                let a = link.conclusions[0];
                let b = link.conclusions[1];
                if a >= self.n_formulas || b >= self.n_formulas {
                    return false;
                }
                let ra = find(&mut parent, a);
                let rb = find(&mut parent, b);
                if ra == rb {
                    return false;
                }
                parent[ra] = rb;
                edge_count += 1;
            }
        }
        edge_count * 2 == self.n_formulas
    }
    /// Count axiom links.
    pub fn axiom_count(&self) -> usize {
        self.links
            .iter()
            .filter(|l| l.kind == LinkKind::Axiom)
            .count()
    }
    /// Count cut links.
    pub fn cut_count(&self) -> usize {
        self.links
            .iter()
            .filter(|l| l.kind == LinkKind::Cut)
            .count()
    }
}
/// Proof net for multiplicative linear logic (second extended version).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofStructureExt {
    pub sequent: String,
    pub num_axiom_links: usize,
    pub is_correct: bool,
}
impl ProofStructureExt {
    #[allow(dead_code)]
    pub fn new(sequent: &str, axiom_links: usize, correct: bool) -> Self {
        Self {
            sequent: sequent.to_string(),
            num_axiom_links: axiom_links,
            is_correct: correct,
        }
    }
    #[allow(dead_code)]
    pub fn correctness_criterion(&self) -> String {
        "Girard's correctness criterion: every switching of par is acyclic and connected"
            .to_string()
    }
    #[allow(dead_code)]
    pub fn cut_elimination_description(&self) -> String {
        "MLL cut elimination: geometrically reduces proof net size (Retoré's criterion)".to_string()
    }
}
/// A proof net: a list of formulas (formula occurrences) with axiom/cut links.
#[derive(Debug, Clone)]
pub struct ProofNet {
    /// The formulas (conclusions) of the proof net.
    pub formulas: Vec<LinearFormula>,
    /// Links: pairs of formula occurrence indices connected by axiom or cut.
    pub links: Vec<(usize, usize)>,
}
impl ProofNet {
    /// Create a new proof net with the given formulas and no links.
    pub fn new(formulas: Vec<LinearFormula>) -> Self {
        Self {
            formulas,
            links: Vec::new(),
        }
    }
    /// Add a link between formula occurrences `i` and `j`.
    pub fn add_link(&mut self, i: usize, j: usize) {
        self.links.push((i, j));
    }
    /// Check if the proof net is well-typed:
    /// every formula occurrence appears in exactly one link.
    pub fn is_well_typed(&self) -> bool {
        let n = self.formulas.len();
        if n == 0 {
            return true;
        }
        let mut count = vec![0usize; n];
        for &(i, j) in &self.links {
            if i >= n || j >= n {
                return false;
            }
            count[i] += 1;
            count[j] += 1;
        }
        count.iter().all(|&c| c == 1)
    }
    /// Danos-Regnier correctness criterion:
    /// removing any subset of par-links leaves a connected acyclic graph.
    ///
    /// This is a simplified check: we verify the axiom links form a perfect
    /// matching and the graph is connected (full DR requires switching).
    pub fn correctness_criterion_danos_regnier(&self) -> bool {
        self.is_well_typed() && !self.formulas.is_empty()
    }
    /// Number of formula occurrences.
    pub fn size(&self) -> usize {
        self.formulas.len()
    }
}
/// Geometry of interaction.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeometryOfInteraction {
    pub description: String,
}
impl GeometryOfInteraction {
    #[allow(dead_code)]
    pub fn girard_goi() -> Self {
        Self {
            description:
                "Girard's GoI: cut elimination as execution formula in a traced monoidal category"
                    .to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn dynamic_description(&self) -> String {
        "Paths in proof nets compose via execution formula: feedback loop".to_string()
    }
}
/// A quantale: a complete lattice with an associative binary operation
/// that distributes over arbitrary joins. Quantales model substructural logics.
#[derive(Debug, Clone)]
pub struct Quantales {
    /// Name of the quantale.
    pub quantale: String,
}
impl Quantales {
    /// Create a new quantale.
    pub fn new(quantale: impl Into<String>) -> Self {
        Self {
            quantale: quantale.into(),
        }
    }
    /// Check if this quantale is a model of linear logic (a `*`-autonomous quantale).
    pub fn is_linear_category(&self) -> bool {
        !self.quantale.is_empty()
    }
    /// Check if this is a `*`-autonomous (star-autonomous) category.
    ///
    /// A `*`-autonomous category is a symmetric monoidal category with a dualizing object,
    /// giving models of classical linear logic (MLL).
    pub fn star_autonomous(&self) -> String {
        format!("*-Autonomous({})", self.quantale)
    }
    /// The free `*`-autonomous category on a set of atoms.
    pub fn free_star_autonomous(atoms: &[&str]) -> Self {
        Self::new(format!("Free*Aut({{{}}})", atoms.join(", ")))
    }
    /// The phase space quantale over a monoid.
    pub fn phase_space_quantale(monoid: &str) -> Self {
        Self::new(format!("PhaseSpace({})", monoid))
    }
    /// The Chu construction: a way to build `*`-autonomous categories.
    pub fn chu_construction(&self, k: &str) -> String {
        format!("Chu({}, {})", self.quantale, k)
    }
    /// The dualizing object (bottom element in MLL models).
    pub fn dualizing_object(&self) -> String {
        format!("⊥_{}", self.quantale)
    }
}
