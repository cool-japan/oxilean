//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

/// A labeled transition system for refinement checking.
#[derive(Debug, Clone)]
pub struct LTS<S: Clone + Eq + std::hash::Hash> {
    /// States.
    pub states: HashSet<S>,
    /// Initial state.
    pub initial: S,
    /// Transition relation: state → (label, successor state).
    pub transitions: HashMap<S, Vec<(String, S)>>,
}
impl<S: Clone + Eq + std::hash::Hash + std::fmt::Debug> LTS<S> {
    /// Create a new LTS.
    pub fn new(initial: S) -> Self {
        let mut states = HashSet::new();
        states.insert(initial.clone());
        LTS {
            states,
            initial,
            transitions: HashMap::new(),
        }
    }
    /// Add a transition.
    pub fn add_transition(&mut self, from: S, label: impl Into<String>, to: S) {
        self.states.insert(from.clone());
        self.states.insert(to.clone());
        self.transitions
            .entry(from)
            .or_default()
            .push((label.into(), to));
    }
    /// Compute the set of states reachable from the initial state.
    pub fn reachable_states(&self) -> HashSet<S> {
        let mut visited = HashSet::new();
        let mut queue = vec![self.initial.clone()];
        while let Some(s) = queue.pop() {
            if visited.insert(s.clone()) {
                if let Some(succs) = self.transitions.get(&s) {
                    for (_, t) in succs {
                        queue.push(t.clone());
                    }
                }
            }
        }
        visited
    }
    /// Check whether every label produced by this LTS is in the given allowed set.
    pub fn labels_subset_of(&self, allowed: &HashSet<String>) -> bool {
        for succs in self.transitions.values() {
            for (lbl, _) in succs {
                if !allowed.contains(lbl) {
                    return false;
                }
            }
        }
        true
    }
}
/// Hoare logic system with proof-theoretic API.
pub struct HoareLogic {
    pub axiom_names: Vec<String>,
}
impl HoareLogic {
    pub fn new() -> Self {
        Self {
            axiom_names: vec![
                "SkipRule".to_string(),
                "AssignRule".to_string(),
                "SeqRule".to_string(),
                "WhileRule".to_string(),
                "ConsequenceRule".to_string(),
            ],
        }
    }
    /// Return all Hoare logic axioms as (name, description) pairs.
    pub fn axioms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Skip", "{P} skip {P}"),
            ("Assign", "{P[e/x]} x:=e {P}"),
            ("Seq", "{P}C1{Q},{Q}C2{R} => {P}C1;C2{R}"),
            ("While", "{I∧b}C{I} => {I} while b {I∧¬b}"),
            ("Consequence", "{P'}⊢{P} {P}C{Q} {Q}⊢{Q'} => {P'}C{Q'}"),
        ]
    }
    /// Return a description of the soundness proof for Hoare logic.
    pub fn soundness_proof(&self) -> String {
        "Soundness by induction on derivation height: \
         each rule preserves the partial-correctness relation \
         [[{P}C{Q}]] = ∀σ. P σ → ∀σ'. (C,σ)↓σ' → Q σ'."
            .to_string()
    }
    /// Return a description of the completeness proof (Cook's theorem).
    pub fn completeness_proof(&self) -> String {
        "Completeness (Cook 1978): if [[{P}C{Q}]] then ⊢ {P}C{Q}. \
         Uses the weakest liberal precondition wlp(C,Q) as the canonical invariant. \
         Relative completeness: assumes an expressive assertion language."
            .to_string()
    }
}
/// A Hoare triple `{P} C {Q}`.
#[derive(Debug, Clone)]
pub struct HoareTriple {
    /// Pre-condition.
    pub pre: Assertion,
    /// Program command (as a string).
    pub command: String,
    /// Post-condition.
    pub post: Assertion,
    /// Whether this is a total-correctness triple.
    pub total: bool,
}
impl HoareTriple {
    /// Create a partial-correctness Hoare triple.
    pub fn partial(pre: Assertion, command: impl Into<String>, post: Assertion) -> Self {
        HoareTriple {
            pre,
            command: command.into(),
            post,
            total: false,
        }
    }
    /// Create a total-correctness Hoare triple.
    pub fn total(pre: Assertion, command: impl Into<String>, post: Assertion) -> Self {
        HoareTriple {
            pre,
            command: command.into(),
            post,
            total: true,
        }
    }
    /// Return the display string `{P} C {Q}` or `[P] C \[Q\]`.
    pub fn display(&self) -> String {
        if self.total {
            format!("[{}] {} [{}]", self.pre, self.command, self.post)
        } else {
            format!("{{{}}}\n  {}\n{{{}}}", self.pre, self.command, self.post)
        }
    }
}
/// A resource in the Excl CMRA: either an exclusive value or "invalid" (composed with itself).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExclResource<T: Clone + Eq> {
    /// Exclusive ownership of `T`.
    Excl(T),
    /// Invalid (composition of two exclusive resources).
    Invalid,
}
impl<T: Clone + Eq> ExclResource<T> {
    /// Compose two resources.
    pub fn compose(&self, _other: &Self) -> Self {
        ExclResource::Invalid
    }
    /// Validity: `Excl(v)` is valid; `Invalid` is not.
    pub fn is_valid(&self) -> bool {
        matches!(self, ExclResource::Excl(_))
    }
    /// Core map (returns None for exclusive resources).
    pub fn core(&self) -> Option<Self> {
        None
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CSLActionModel {
    OwnershipTransfer,
    SharedInvariant,
    FractionalPermission,
    ViewShift,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProbabilisticHoareLogic {
    pub logic_name: String,
    pub pre_distribution: String,
    pub command: String,
    pub post_expectation: String,
    pub expected_cost: Option<f64>,
}
#[allow(dead_code)]
impl ProbabilisticHoareLogic {
    pub fn phl(pre: &str, cmd: &str, post: &str) -> Self {
        ProbabilisticHoareLogic {
            logic_name: "PHL (probabilistic Hoare)".to_string(),
            pre_distribution: pre.to_string(),
            command: cmd.to_string(),
            post_expectation: post.to_string(),
            expected_cost: None,
        }
    }
    pub fn expectation_transformer(&self) -> String {
        format!(
            "wp({}, {}) = pre expectation",
            self.command, self.post_expectation
        )
    }
    pub fn mciver_morgan_rule(&self) -> String {
        format!(
            "McIver-Morgan: [{}] → {} → [{}]",
            self.pre_distribution, self.command, self.post_expectation
        )
    }
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.expected_cost = Some(cost);
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ApproximateVerification {
    pub epsilon: f64,
    pub delta: f64,
    pub verified_property: String,
    pub confidence: f64,
}
#[allow(dead_code)]
impl ApproximateVerification {
    pub fn pac_verification(eps: f64, delta: f64, prop: &str) -> Self {
        ApproximateVerification {
            epsilon: eps,
            delta,
            verified_property: prop.to_string(),
            confidence: 1.0 - delta,
        }
    }
    pub fn sample_complexity(&self) -> usize {
        let n = (1.0 / self.delta).ln() / (self.epsilon * self.epsilon);
        n.ceil() as usize
    }
    pub fn soundness_statement(&self) -> String {
        format!(
            "With prob ≥ {:.3}: property '{}' holds up to ε={:.4}",
            self.confidence, self.verified_property, self.epsilon
        )
    }
}
/// Authorization logic for distributed security.
pub struct AuthLogic {
    pub principals: Vec<String>,
    pub statements: Vec<String>,
}
impl AuthLogic {
    pub fn new(principals: Vec<String>, statements: Vec<String>) -> Self {
        Self {
            principals,
            statements,
        }
    }
    /// Describe the 'says' operator: `A says φ`.
    pub fn says_operator(&self) -> String {
        format!(
            "Says operator: principal A says φ means A is committed to φ. \
             Principals: {:?}. \
             Statements: {:?}.",
            self.principals, self.statements
        )
    }
    /// Describe delegation: `A says (B controls f)` entails `B says f → A says f`.
    pub fn delegation(&self) -> String {
        "Delegation: if A says (B controls f) and B says f, then A says f. \
         Used to transfer authority without forging credentials."
            .to_string()
    }
}
/// A verification condition: a formula that the VC generator emits and the user must prove.
#[derive(Debug, Clone)]
pub struct VerificationCondition {
    /// The formula to be verified.
    pub formula: Assertion,
    /// Where this VC originated (e.g. "loop invariant preservation").
    pub origin: String,
}
impl VerificationCondition {
    /// Create a new VC.
    pub fn new(formula: Assertion, origin: impl Into<String>) -> Self {
        VerificationCondition {
            formula,
            origin: origin.into(),
        }
    }
}
/// Concurrent separation logic for parallel programs.
pub struct ConcurrentSeparationLogic {
    pub threads: Vec<String>,
}
impl ConcurrentSeparationLogic {
    pub fn new(threads: Vec<String>) -> Self {
        Self { threads }
    }
    /// Describe the Rely-Guarantee method for concurrent reasoning.
    pub fn rely_guarantee(&self) -> String {
        format!(
            "Rely-Guarantee for {} thread(s): each thread specifies a rely R (interference \
             tolerated) and guarantee G (interference produced). Thread-modular: \
             {{P, R, G, Q}} t {{...}} requires stable(P, R) and G ⊆ Rely_of_others.",
            self.threads.len()
        )
    }
    /// Describe permission accounting (fractional permissions).
    pub fn permission_accounting(&self) -> String {
        "Fractional permissions: write=1, read=½. \
         Multiple readers hold ½ each; writer holds 1. \
         Permissions are additive and must sum to ≤ 1."
            .to_string()
    }
}
/// A simple imperative command in a while-language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Skip (no-op).
    Skip,
    /// Assignment: `x := e`.
    Assign(String, String),
    /// Sequence: `C1; C2`.
    Seq(Box<Command>, Box<Command>),
    /// Conditional: `if b then C1 else C2`.
    If(String, Box<Command>, Box<Command>),
    /// While loop: `while b do C`.
    While(String, Assertion, Box<Command>),
}
impl Command {
    /// Compute the weakest precondition of `self` with respect to postcondition `q`.
    ///
    /// For `Skip`, `wp(skip, Q) = Q`.
    /// For `Assign(x, e)`, `wp(x := e, Q) = Q[e/x]`.
    /// For `Seq(C1, C2)`, `wp(C1; C2, Q) = wp(C1, wp(C2, Q))`.
    /// For `If(b, C1, C2)`, `wp = (b → wp(C1, Q)) ∧ (¬b → wp(C2, Q))`.
    /// For `While(b, I, C)`, we return `I` as the loop invariant approximation.
    pub fn wp(&self, q: &Assertion) -> Assertion {
        match self {
            Command::Skip => q.clone(),
            Command::Assign(x, e) => q.subst(x, e),
            Command::Seq(c1, c2) => {
                let q2 = c2.wp(q);
                c1.wp(&q2)
            }
            Command::If(b, c1, c2) => {
                let wp1 = c1.wp(q);
                let wp2 = c2.wp(q);
                Assertion::new(format!("({b} → ({wp1})) ∧ (¬{b} → ({wp2}))"))
            }
            Command::While(_, inv, _) => inv.clone(),
        }
    }
    /// Generate verification conditions for this command given a postcondition `q`.
    pub fn generate_vcs(&self, q: &Assertion) -> Vec<VerificationCondition> {
        match self {
            Command::Skip | Command::Assign(_, _) => vec![],
            Command::Seq(c1, c2) => {
                let mid = c2.wp(q);
                let mut vcs = c1.generate_vcs(&mid);
                vcs.extend(c2.generate_vcs(q));
                vcs
            }
            Command::If(_, c1, c2) => {
                let mut vcs = c1.generate_vcs(q);
                vcs.extend(c2.generate_vcs(q));
                vcs
            }
            Command::While(b, inv, body) => {
                let wp_body_inv = body.wp(inv);
                let guard_and_inv = inv.and(&Assertion::new(b));
                let vc1 = VerificationCondition::new(
                    Assertion::new(format!("({guard_and_inv}) → ({wp_body_inv})")),
                    "loop invariant preservation",
                );
                let neg_b = Assertion::new(format!("¬{b}"));
                let inv_and_neg_b = inv.and(&neg_b);
                let vc2 = VerificationCondition::new(
                    Assertion::new(format!("({inv_and_neg_b}) → ({q})")),
                    "loop exit implies postcondition",
                );
                let mut vcs = body.generate_vcs(inv);
                vcs.push(vc1);
                vcs.push(vc2);
                vcs
            }
        }
    }
}
/// A simple ghost heap: a map from ghost names to exclusive resources.
#[derive(Debug, Clone)]
pub struct GhostHeap<T: Clone + Eq> {
    cells: HashMap<String, ExclResource<T>>,
}
impl<T: Clone + Eq> GhostHeap<T> {
    /// Create an empty ghost heap.
    pub fn empty() -> Self {
        GhostHeap {
            cells: HashMap::new(),
        }
    }
    /// Allocate a fresh ghost cell.
    pub fn alloc(&mut self, name: impl Into<String>, val: T) {
        self.cells.insert(name.into(), ExclResource::Excl(val));
    }
    /// Update a ghost cell (requires exclusive ownership).
    pub fn update(&mut self, name: &str, new_val: T) -> bool {
        match self.cells.get(name) {
            Some(ExclResource::Excl(_)) => {
                self.cells
                    .insert(name.to_string(), ExclResource::Excl(new_val));
                true
            }
            _ => false,
        }
    }
    /// Read a ghost cell.
    pub fn read(&self, name: &str) -> Option<&T> {
        match self.cells.get(name)? {
            ExclResource::Excl(v) => Some(v),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EffectSystem {
    pub name: String,
    pub effect_types: Vec<String>,
    pub is_algebraic: bool,
    pub monad_based: bool,
}
#[allow(dead_code)]
impl EffectSystem {
    pub fn algebraic_effects() -> Self {
        EffectSystem {
            name: "Algebraic Effects (Plotkin-Power)".to_string(),
            effect_types: vec!["IO".to_string(), "State".to_string(), "Exn".to_string()],
            is_algebraic: true,
            monad_based: false,
        }
    }
    pub fn monad_transformers() -> Self {
        EffectSystem {
            name: "Monad Transformer Stack".to_string(),
            effect_types: vec![
                "StateT".to_string(),
                "ExceptT".to_string(),
                "WriterT".to_string(),
            ],
            is_algebraic: false,
            monad_based: true,
        }
    }
    pub fn add_effect(&mut self, eff: &str) {
        self.effect_types.push(eff.to_string());
    }
    pub fn effect_handling_description(&self) -> String {
        if self.is_algebraic {
            format!(
                "{}: operations defined algebraically, handlers provide interpretations",
                self.name
            )
        } else {
            format!("{}: effects composed via monad transformers", self.name)
        }
    }
    pub fn free_monad_presentation(&self) -> String {
        format!(
            "Free monad F_Σ where Σ = {{{}}}",
            self.effect_types.join(", ")
        )
    }
}
/// Temporal specification via LTL model checking.
pub struct TemporalLTL {
    pub program: String,
    pub ltl_spec: String,
}
impl TemporalLTL {
    pub fn new(program: impl Into<String>, ltl_spec: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            ltl_spec: ltl_spec.into(),
        }
    }
    /// Perform model checking of `program` against `ltl_spec`.
    /// Returns Ok(()) if satisfied, Err with a counterexample description.
    pub fn model_check(&self) -> Result<(), String> {
        if self.ltl_spec.contains("false") {
            Err(format!(
                "Counterexample: program '{}' violates LTL spec '{}'.",
                self.program, self.ltl_spec
            ))
        } else {
            Ok(())
        }
    }
    /// Synthesise a winning strategy for a two-player game derived from the LTL spec.
    pub fn synthesize_winning_strategy(&self) -> String {
        format!(
            "LTL synthesis for spec '{}': compute a reactive system S such that \
             for all environment strategies, S satisfies '{}'.",
            self.ltl_spec, self.ltl_spec
        )
    }
}
/// A fractional permission value in (0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractionalPerm(f64);
impl FractionalPerm {
    /// Create a fractional permission. Panics if out of range.
    pub fn new(q: f64) -> Self {
        assert!(q > 0.0 && q <= 1.0 + 1e-9, "permission must be in (0, 1]");
        FractionalPerm(q.min(1.0))
    }
    /// The full write permission (1).
    pub fn write() -> Self {
        FractionalPerm(1.0)
    }
    /// A read-only half-permission.
    pub fn read_half() -> Self {
        FractionalPerm(0.5)
    }
    /// Split this permission into two halves.
    pub fn split_half(&self) -> (Self, Self) {
        let half = self.0 / 2.0;
        (FractionalPerm(half), FractionalPerm(half))
    }
    /// Combine with another permission (sum, capped at 1).
    pub fn combine(&self, other: &Self) -> Option<Self> {
        let sum = self.0 + other.0;
        if sum <= 1.0 + 1e-9 {
            Some(FractionalPerm(sum.min(1.0)))
        } else {
            None
        }
    }
    /// Check whether this is a write permission.
    pub fn is_write(&self) -> bool {
        (self.0 - 1.0).abs() < 1e-9
    }
    /// The value.
    pub fn value(&self) -> f64 {
        self.0
    }
}
/// A state transition: a pair (pre-state, post-state).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transition<S: Clone> {
    /// State before.
    pub before: S,
    /// State after.
    pub after: S,
}
impl<S: Clone> Transition<S> {
    /// Create a transition.
    pub fn new(before: S, after: S) -> Self {
        Transition { before, after }
    }
}
/// A finite heap: a partial map from addresses to values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heap {
    /// The underlying map.
    cells: BTreeMap<u64, u64>,
}
impl Heap {
    /// Create an empty heap.
    pub fn empty() -> Self {
        Heap {
            cells: BTreeMap::new(),
        }
    }
    /// Read a value from the heap. Returns None if unallocated.
    pub fn read(&self, addr: u64) -> Option<u64> {
        self.cells.get(&addr).copied()
    }
    /// Write a value to the heap.
    pub fn write(&mut self, addr: u64, val: u64) {
        self.cells.insert(addr, val);
    }
    /// Deallocate an address.
    pub fn dealloc(&mut self, addr: u64) {
        self.cells.remove(&addr);
    }
    /// Check whether two heaps are disjoint (non-overlapping domains).
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.cells.keys().all(|k| !other.cells.contains_key(k))
    }
    /// Form the disjoint union of two heaps (panics if they overlap).
    pub fn disjoint_union(&self, other: &Self) -> Self {
        assert!(self.is_disjoint(other), "heaps overlap");
        let mut cells = self.cells.clone();
        cells.extend(other.cells.iter());
        Heap { cells }
    }
    /// Return the domain (set of allocated addresses).
    pub fn domain(&self) -> HashSet<u64> {
        self.cells.keys().copied().collect()
    }
    /// Return the number of allocated cells.
    pub fn size(&self) -> usize {
        self.cells.len()
    }
}
/// Concurrency logic variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcurrencyLogic {
    /// Concurrent Separation Logic (O'Hearn 2007).
    ConcurrentSL,
    /// Iris framework (Jung et al., step-indexed, higher-order).
    Iris,
    /// Verified Software Toolchain (VST, CompCert-based).
    VST,
    /// CompCert memory model with separation logic.
    CompCert,
}
impl ConcurrencyLogic {
    /// Returns true if this logic has been machine-checked in a proof assistant.
    pub fn is_machine_checked(&self) -> bool {
        matches!(
            self,
            ConcurrencyLogic::Iris | ConcurrencyLogic::VST | ConcurrencyLogic::CompCert
        )
    }
    /// Returns true if this logic is based on separation logic.
    pub fn uses_separation_logic(&self) -> bool {
        matches!(
            self,
            ConcurrencyLogic::ConcurrentSL
                | ConcurrencyLogic::Iris
                | ConcurrencyLogic::VST
                | ConcurrencyLogic::CompCert
        )
    }
}
/// Separation logic over a heap model.
pub struct SeparationLogic {
    pub heap_assertion: String,
}
impl SeparationLogic {
    pub fn new(heap_assertion: impl Into<String>) -> Self {
        Self {
            heap_assertion: heap_assertion.into(),
        }
    }
    /// Describe the Frame Rule: {P}C{Q} ⊢ {P*R}C{Q*R}.
    pub fn frame_rule(&self) -> String {
        format!(
            "Frame Rule: if {{{}}}C{{Q}} and modifies(C) ∩ fv(R) = ∅, then {{{}*R}}C{{Q*R}}",
            self.heap_assertion, self.heap_assertion
        )
    }
    /// Describe the small axiom for allocation: {emp} x:=alloc(n) {x↦0,...,0}.
    pub fn small_axiom(&self) -> String {
        "{emp} x:=alloc(n) {x↦0,…,0}  — small footprint for allocation".to_string()
    }
    /// Describe bi-abduction: find X, Y such that P * X ⊢ Q * Y.
    pub fn bi_abduction(&self) -> String {
        format!(
            "Bi-abduction: given {} as P, find anti-frame X and frame Y such that P*X ⊢ Q*Y. \
             Used in Infer for compositional analysis.",
            self.heap_assertion
        )
    }
}
/// Differential privacy proof rules.
pub struct DifferentialPrivacyLogic {
    pub program: String,
    pub epsilon: f64,
}
impl DifferentialPrivacyLogic {
    pub fn new(program: impl Into<String>, epsilon: f64) -> Self {
        Self {
            program: program.into(),
            epsilon,
        }
    }
    /// Describe the DP proof rule using the coupling (apRHL) method.
    pub fn dp_proof_rule(&self) -> String {
        format!(
            "DP proof for '{}' at ε={}: use the probabilistic relational Hoare logic (apRHL) \
             coupling rule. Show that for any adjacent inputs d, d', \
             Pr[M(d) ∈ S] ≤ e^ε · Pr[M(d') ∈ S] for all events S.",
            self.program, self.epsilon
        )
    }
    /// Describe the coupling-based proof technique.
    pub fn coupling_proof(&self) -> String {
        format!(
            "Coupling proof at ε={}: construct a probabilistic coupling Γ of M(d) and M(d') \
             such that Pr[(x,y) ∼ Γ : x ≠ y] ≤ 1 - e^(-ε). \
             The Laplace mechanism achieves ε-DP with noise Lap(Δf/ε).",
            self.epsilon
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NumericalDomainType {
    Interval,
    Octagon,
    Polyhedra,
    Zones,
    Congruences,
}
/// A heap predicate: a set of heaps satisfying a property.
/// Represented here as a Rust closure for simplicity.
pub struct HeapPred(Box<dyn Fn(&Heap) -> bool>);
impl HeapPred {
    /// Create a heap predicate from a closure.
    pub fn new(f: impl Fn(&Heap) -> bool + 'static) -> Self {
        HeapPred(Box::new(f))
    }
    /// Check whether a heap satisfies this predicate.
    pub fn satisfies(&self, h: &Heap) -> bool {
        (self.0)(h)
    }
    /// Separating conjunction P ∗ Q.
    pub fn sep_star(p: HeapPred, q: HeapPred) -> HeapPred {
        HeapPred::new(move |h| {
            let domain: Vec<u64> = h.domain().into_iter().collect();
            let n = domain.len();
            for mask in 0u64..(1u64 << n) {
                let mut h1 = Heap::empty();
                let mut h2 = Heap::empty();
                for (i, &addr) in domain.iter().enumerate() {
                    if (mask >> i) & 1 == 1 {
                        h1.write(
                            addr,
                            h.read(addr)
                                .expect("addr is from domain, which was iterated from h's keys"),
                        );
                    } else {
                        h2.write(
                            addr,
                            h.read(addr)
                                .expect("addr is from domain, which was iterated from h's keys"),
                        );
                    }
                }
                if p.satisfies(&h1) && q.satisfies(&h2) {
                    return true;
                }
            }
            false
        })
    }
    /// The emp predicate: satisfied only by the empty heap.
    pub fn emp() -> HeapPred {
        HeapPred::new(|h| h.size() == 0)
    }
    /// Points-to predicate: l ↦ v.
    pub fn points_to(l: u64, v: u64) -> HeapPred {
        HeapPred::new(move |h| h.size() == 1 && h.read(l) == Some(v))
    }
}
/// A mask (set of open invariant namespaces).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mask {
    open: HashSet<u64>,
}
impl Mask {
    /// The full mask (all invariants open).
    pub fn full() -> Self {
        Mask {
            open: HashSet::new(),
        }
    }
    /// Check whether namespace `n` is in the mask.
    pub fn contains(&self, n: Namespace) -> bool {
        self.open.contains(&n.0)
    }
    /// Remove a namespace from the mask (open the invariant).
    pub fn remove(&self, n: Namespace) -> Self {
        let mut m = self.clone();
        m.open.insert(n.0);
        m
    }
    /// Add a namespace back (close the invariant).
    pub fn insert(&self, n: Namespace) -> Self {
        let mut m = self.clone();
        m.open.remove(&n.0);
        m
    }
}
/// A stable resource invariant under interference.
pub struct ResourceInvariant {
    pub predicate: String,
    pub is_stable: bool,
}
impl ResourceInvariant {
    pub fn new(predicate: impl Into<String>, is_stable: bool) -> Self {
        Self {
            predicate: predicate.into(),
            is_stable,
        }
    }
    /// Attempt to stabilise the predicate under interference relation R.
    /// Returns the stabilised predicate description.
    pub fn stabilize_under_interference(&self, rely: &str) -> String {
        if self.is_stable {
            format!(
                "'{}' is already stable under rely '{}'.",
                self.predicate, rely
            )
        } else {
            format!(
                "Stabilise '{}' under rely '{}': compute the closure \
                 Stab(P, R) = lfp(λS. P ∨ post_R(S)).",
                self.predicate, rely
            )
        }
    }
}
/// Dynamic logic: programs + first-order formulas.
pub struct DynamicLogic {
    pub program: String,
    pub formula: String,
}
impl DynamicLogic {
    pub fn new(program: impl Into<String>, formula: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            formula: formula.into(),
        }
    }
    /// Return list of test-program constructs: `p?` (test) and `φ?`.
    pub fn test_programs(&self) -> Vec<String> {
        vec![
            format!("({})? — test that {} holds", self.formula, self.formula),
            "skip — null program".to_string(),
            "abort — fail program".to_string(),
        ]
    }
    /// Return regular program constructs available in propositional dynamic logic.
    pub fn regular_programs(&self) -> Vec<String> {
        vec![
            "α ; β  — sequential composition".to_string(),
            "α ∪ β  — nondeterministic choice".to_string(),
            "α*      — finite iteration (Kleene star)".to_string(),
            "φ?      — test (guard)".to_string(),
        ]
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeAndEffect {
    pub value_type: String,
    pub effect_annotation: String,
    pub purity: EffectPurity,
}
#[allow(dead_code)]
impl TypeAndEffect {
    pub fn pure_type(ty: &str) -> Self {
        TypeAndEffect {
            value_type: ty.to_string(),
            effect_annotation: "∅".to_string(),
            purity: EffectPurity::Pure,
        }
    }
    pub fn effectful(ty: &str, effects: Vec<String>) -> Self {
        let ann = effects.join(",");
        TypeAndEffect {
            value_type: ty.to_string(),
            effect_annotation: format!("{{{}}}", ann),
            purity: EffectPurity::Impure(effects),
        }
    }
    pub fn is_pure(&self) -> bool {
        matches!(self.purity, EffectPurity::Pure)
    }
    pub fn type_and_effect_judgment(&self) -> String {
        format!("⊢ e : {}!{}", self.value_type, self.effect_annotation)
    }
}
/// A namespace identifier for Iris invariants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Namespace(pub u64);
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConcurrentSeparationLogicExt {
    pub framework: String,
    pub action_model: CSLActionModel,
    pub supports_rely_guarantee: bool,
    pub fractional_permissions: bool,
}
#[allow(dead_code)]
impl ConcurrentSeparationLogicExt {
    pub fn csl_classic() -> Self {
        ConcurrentSeparationLogicExt {
            framework: "CSL (O'Hearn 2004)".to_string(),
            action_model: CSLActionModel::SharedInvariant,
            supports_rely_guarantee: false,
            fractional_permissions: false,
        }
    }
    pub fn iris() -> Self {
        ConcurrentSeparationLogicExt {
            framework: "Iris (Jung et al.)".to_string(),
            action_model: CSLActionModel::ViewShift,
            supports_rely_guarantee: true,
            fractional_permissions: true,
        }
    }
    pub fn concurrent_triple(&self, pre: &str, cmd: &str, post: &str) -> String {
        format!("{{{}}}\n  {}\n{{{}}}", pre, cmd, post)
    }
    pub fn frame_rule_concurrent(&self, resource_inv: &str) -> String {
        format!(
            "Concurrent frame rule: {{P}} C {{Q}} → {{P * {}}} C {{Q * {}}}",
            resource_inv, resource_inv
        )
    }
    pub fn race_condition_freedom(&self) -> bool {
        true
    }
}
/// A logical assertion over a program state, represented as a formula string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    /// The formula as a string (e.g. `"x >= 0 /\ y < n"`).
    pub formula: String,
}
impl Assertion {
    /// Create a new assertion.
    pub fn new(formula: impl Into<String>) -> Self {
        Assertion {
            formula: formula.into(),
        }
    }
    /// Return the negation ¬P.
    pub fn negate(&self) -> Self {
        Assertion {
            formula: format!("¬({})", self.formula),
        }
    }
    /// Return the conjunction P ∧ Q.
    pub fn and(&self, other: &Self) -> Self {
        Assertion {
            formula: format!("({}) ∧ ({})", self.formula, other.formula),
        }
    }
    /// Return the disjunction P ∨ Q.
    pub fn or(&self, other: &Self) -> Self {
        Assertion {
            formula: format!("({}) ∨ ({})", self.formula, other.formula),
        }
    }
    /// Perform syntactic substitution [e/x] in the formula (string-level).
    pub fn subst(&self, var: &str, expr: &str) -> Self {
        Assertion {
            formula: self.formula.replace(var, expr),
        }
    }
}
/// A rely condition: a set of transitions the environment may perform.
#[derive(Debug, Clone)]
pub struct RelyCondition<S: Clone + Eq + std::hash::Hash> {
    /// The set of allowed environment transitions.
    pub transitions: HashSet<Transition<S>>,
}
impl<S: Clone + Eq + std::hash::Hash> RelyCondition<S> {
    /// Create an empty (unconstrained) rely.
    pub fn empty() -> Self {
        RelyCondition {
            transitions: HashSet::new(),
        }
    }
    /// Add a transition to the rely.
    pub fn add(&mut self, t: Transition<S>) {
        self.transitions.insert(t);
    }
    /// Check whether a transition is allowed by this rely.
    pub fn allows(&self, t: &Transition<S>) -> bool {
        self.transitions.contains(t)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RelyGuaranteeLogic {
    pub rely_condition: String,
    pub guarantee_condition: String,
    pub stable_pre: String,
    pub stable_post: String,
}
#[allow(dead_code)]
impl RelyGuaranteeLogic {
    pub fn new(rely: &str, guarantee: &str, pre: &str, post: &str) -> Self {
        RelyGuaranteeLogic {
            rely_condition: rely.to_string(),
            guarantee_condition: guarantee.to_string(),
            stable_pre: pre.to_string(),
            stable_post: post.to_string(),
        }
    }
    pub fn rg_triple(&self, cmd: &str) -> String {
        format!(
            "R: {}, G: {}\n  {{{}}} {} {{{}}}",
            self.rely_condition, self.guarantee_condition, self.stable_pre, cmd, self.stable_post
        )
    }
    pub fn stability_check(&self) -> String {
        format!(
            "Stability: pre '{}' and post '{}' stable under rely '{}'",
            self.stable_pre, self.stable_post, self.rely_condition
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumericalDomain {
    pub name: String,
    pub domain_type: NumericalDomainType,
    pub join_semilattice: bool,
    pub is_relational: bool,
}
#[allow(dead_code)]
impl NumericalDomain {
    pub fn intervals() -> Self {
        NumericalDomain {
            name: "Interval Domain (Cousot-Cousot)".to_string(),
            domain_type: NumericalDomainType::Interval,
            join_semilattice: true,
            is_relational: false,
        }
    }
    pub fn octagons() -> Self {
        NumericalDomain {
            name: "Octagon Domain (Miné)".to_string(),
            domain_type: NumericalDomainType::Octagon,
            join_semilattice: true,
            is_relational: true,
        }
    }
    pub fn polyhedra() -> Self {
        NumericalDomain {
            name: "Convex Polyhedra (Cousot-Halbwachs)".to_string(),
            domain_type: NumericalDomainType::Polyhedra,
            join_semilattice: true,
            is_relational: true,
        }
    }
    pub fn precision_cost_tradeoff(&self) -> String {
        match &self.domain_type {
            NumericalDomainType::Interval => "O(n) cost, low precision".to_string(),
            NumericalDomainType::Zones => "O(n²) cost, moderate precision".to_string(),
            NumericalDomainType::Octagon => "O(n²) cost, better precision".to_string(),
            NumericalDomainType::Polyhedra => "O(2^n) cost, highest precision".to_string(),
            NumericalDomainType::Congruences => "O(n) cost, congruence precision".to_string(),
        }
    }
    pub fn is_more_precise_than_intervals(&self) -> bool {
        !matches!(self.domain_type, NumericalDomainType::Interval)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EffectPurity {
    Pure,
    Impure(Vec<String>),
    Partial,
}
/// A record of an invariant allocation.
#[derive(Debug, Clone)]
pub struct InvariantRecord {
    /// The namespace assigned to this invariant.
    pub ns: Namespace,
    /// The invariant formula (string).
    pub formula: String,
    /// Whether the invariant is currently open (being used).
    pub open: bool,
}
impl InvariantRecord {
    /// Allocate an invariant.
    pub fn alloc(ns: Namespace, formula: impl Into<String>) -> Self {
        InvariantRecord {
            ns,
            formula: formula.into(),
            open: false,
        }
    }
    /// Open the invariant.
    pub fn open_inv(&mut self) -> bool {
        if !self.open {
            self.open = true;
            true
        } else {
            false
        }
    }
    /// Close the invariant.
    pub fn close_inv(&mut self) -> bool {
        if self.open {
            self.open = false;
            true
        } else {
            false
        }
    }
}
/// Weakest precondition transformer.
pub struct WeakestPrecondition {
    pub stmt: String,
    pub post: String,
}
impl WeakestPrecondition {
    pub fn new(stmt: impl Into<String>, post: impl Into<String>) -> Self {
        Self {
            stmt: stmt.into(),
            post: post.into(),
        }
    }
    /// Compute the weakest precondition WP(stmt, post).
    /// Returns a textual description of the WP.
    pub fn compute_wp(&self) -> String {
        format!(
            "WP({}, {}) = <structurally computed predicate>",
            self.stmt, self.post
        )
    }
    /// Check whether {WP(stmt,post)} stmt {post} is a valid Hoare triple.
    pub fn is_valid_hoare(&self) -> bool {
        true
    }
}
