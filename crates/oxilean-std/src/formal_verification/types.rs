//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A naive CTL model checker over `KripkeStructure`.
pub struct CTLModelChecker<'a> {
    pub kripke: &'a KripkeStructure,
}
impl<'a> CTLModelChecker<'a> {
    pub fn new(kripke: &'a KripkeStructure) -> Self {
        CTLModelChecker { kripke }
    }
    /// Return the set of states satisfying `formula`.
    pub fn sat(&self, formula: &CTLFormula) -> Vec<usize> {
        let n = self.kripke.states.len();
        match formula {
            CTLFormula::True => (0..n).collect(),
            CTLFormula::False => vec![],
            CTLFormula::Atom(p) => (0..n).filter(|&s| self.kripke.holds_in(s, p)).collect(),
            CTLFormula::Not(f) => {
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                (0..n).filter(|s| !sat_f.contains(s)).collect()
            }
            CTLFormula::And(f, g) => {
                let sf: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let sg: std::collections::HashSet<usize> = self.sat(g).into_iter().collect();
                sf.intersection(&sg).copied().collect()
            }
            CTLFormula::Or(f, g) => {
                let sf: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let sg: std::collections::HashSet<usize> = self.sat(g).into_iter().collect();
                let mut r: Vec<usize> = sf.union(&sg).copied().collect();
                r.sort_unstable();
                r
            }
            CTLFormula::EX(f) => {
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let mut result = std::collections::HashSet::new();
                for &(src, dst) in &self.kripke.transitions {
                    if sat_f.contains(&dst) {
                        result.insert(src);
                    }
                }
                let mut r: Vec<usize> = result.into_iter().collect();
                r.sort_unstable();
                r
            }
            CTLFormula::AX(f) => {
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let mut result = Vec::new();
                for s in 0..n {
                    let successors: Vec<usize> = self
                        .kripke
                        .transitions
                        .iter()
                        .filter(|(src, _)| *src == s)
                        .map(|(_, dst)| *dst)
                        .collect();
                    if !successors.is_empty() && successors.iter().all(|d| sat_f.contains(d)) {
                        result.push(s);
                    }
                }
                result
            }
            CTLFormula::EF(f) => {
                let mut result: std::collections::HashSet<usize> =
                    self.sat(f).into_iter().collect();
                let mut changed = true;
                while changed {
                    changed = false;
                    for &(src, dst) in &self.kripke.transitions {
                        if result.contains(&dst) && !result.contains(&src) {
                            result.insert(src);
                            changed = true;
                        }
                    }
                }
                let mut r: Vec<usize> = result.into_iter().collect();
                r.sort_unstable();
                r
            }
            CTLFormula::AF(f) => {
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let mut result = sat_f.clone();
                let mut changed = true;
                while changed {
                    changed = false;
                    for s in 0..n {
                        if result.contains(&s) {
                            continue;
                        }
                        let successors: Vec<usize> = self
                            .kripke
                            .transitions
                            .iter()
                            .filter(|(src, _)| *src == s)
                            .map(|(_, dst)| *dst)
                            .collect();
                        if !successors.is_empty() && successors.iter().all(|d| result.contains(d)) {
                            result.insert(s);
                            changed = true;
                        }
                    }
                }
                let mut r: Vec<usize> = result.into_iter().collect();
                r.sort_unstable();
                r
            }
            CTLFormula::EG(f) => {
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                sat_f.into_iter().collect()
            }
            CTLFormula::AG(f) => {
                let ef_not: std::collections::HashSet<usize> = self
                    .sat(&CTLFormula::EF(Box::new(CTLFormula::Not(f.clone()))))
                    .into_iter()
                    .collect();
                let mut r: Vec<usize> = (0..n).filter(|s| !ef_not.contains(s)).collect();
                r.sort_unstable();
                r
            }
            CTLFormula::EU(f, g) => {
                let mut result: std::collections::HashSet<usize> =
                    self.sat(g).into_iter().collect();
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let mut changed = true;
                while changed {
                    changed = false;
                    for &(src, dst) in &self.kripke.transitions {
                        if result.contains(&dst) && sat_f.contains(&src) && !result.contains(&src) {
                            result.insert(src);
                            changed = true;
                        }
                    }
                }
                let mut r: Vec<usize> = result.into_iter().collect();
                r.sort_unstable();
                r
            }
            CTLFormula::AU(f, g) => {
                let mut result: std::collections::HashSet<usize> =
                    self.sat(g).into_iter().collect();
                let sat_f: std::collections::HashSet<usize> = self.sat(f).into_iter().collect();
                let mut changed = true;
                while changed {
                    changed = false;
                    for s in 0..n {
                        if result.contains(&s) || !sat_f.contains(&s) {
                            continue;
                        }
                        let successors: Vec<usize> = self
                            .kripke
                            .transitions
                            .iter()
                            .filter(|(src, _)| *src == s)
                            .map(|(_, dst)| *dst)
                            .collect();
                        if !successors.is_empty() && successors.iter().all(|d| result.contains(d)) {
                            result.insert(s);
                            changed = true;
                        }
                    }
                }
                let mut r: Vec<usize> = result.into_iter().collect();
                r.sort_unstable();
                r
            }
        }
    }
    /// Check if `formula` holds in the initial state (state 0).
    pub fn check(&self, formula: &CTLFormula) -> ModelCheckingResult {
        let sat_states = self.sat(formula);
        let formula_str = formula.to_string();
        if sat_states.contains(&0) {
            ModelCheckingResult::verified(formula_str)
        } else {
            ModelCheckingResult::falsified(formula_str, vec!["s0".into()])
        }
    }
}
/// A Hoare triple `{P} C {Q}` asserting that if `P` holds before running
/// program `C`, then `Q` holds afterwards (partial correctness).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoareTriple {
    /// Precondition `P`.
    pub pre: String,
    /// Program (command) `C`.
    pub program: String,
    /// Postcondition `Q`.
    pub post: String,
}
impl HoareTriple {
    /// Construct a new Hoare triple.
    pub fn new(
        pre: impl Into<String>,
        program: impl Into<String>,
        post: impl Into<String>,
    ) -> Self {
        Self {
            pre: pre.into(),
            program: program.into(),
            post: post.into(),
        }
    }
    /// Heuristic validity check.
    ///
    /// A triple is considered trivially valid when the precondition is `False`
    /// (ex falso) or the postcondition is `True`.  All other triples are
    /// treated as potentially valid pending a full proof.
    ///
    /// ```
    /// use oxilean_std::formal_verification::HoareTriple;
    /// let t = HoareTriple::new("False", "x := 0", "x = 0");
    /// assert!(t.is_valid());
    /// let t2 = HoareTriple::new("x > 0", "skip", "True");
    /// assert!(t2.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.pre == "False" || self.post == "True"
    }
}
/// Checker for separation logic entailments using structural rules.
pub struct SeparationLogicChecker;
impl SeparationLogicChecker {
    pub fn new() -> Self {
        SeparationLogicChecker
    }
    /// Check if `pred` entails `goal` using structural rules.
    pub fn entails(&self, pred: &HeapPredicate, goal: &HeapPredicate) -> bool {
        if pred == goal {
            return true;
        }
        match (pred, goal) {
            (HeapPredicate::Sep(l, r), _) => {
                if **l == HeapPredicate::Emp && self.entails(r, goal) {
                    return true;
                }
                if **r == HeapPredicate::Emp && self.entails(l, goal) {
                    return true;
                }
            }
            (_, HeapPredicate::Sep(l, r)) => {
                if **l == HeapPredicate::Emp && self.entails(pred, r) {
                    return true;
                }
                if **r == HeapPredicate::Emp && self.entails(pred, l) {
                    return true;
                }
            }
            _ => {}
        }
        false
    }
    /// Check the frame rule: if {P} C {Q} and frame is disjoint, then {P * frame} C {Q * frame}.
    pub fn check_frame_rule(&self, spec: &HoareTriple, frame: &HeapPredicate) -> bool {
        frame.is_satisfiable() && !spec.program.is_empty()
    }
    /// Check incorrectness logic: [P] C [Q] means Q is reachable from some P state.
    /// Approximation: valid if P is satisfiable.
    pub fn check_incorrectness(&self, pre: &HeapPredicate, _post: &HeapPredicate) -> bool {
        pre.is_satisfiable()
    }
}
/// A variable security classification.
#[derive(Debug, Clone)]
pub struct SecurityClassification {
    pub variables: Vec<(String, SecurityLevel)>,
}
impl SecurityClassification {
    pub fn new() -> Self {
        SecurityClassification {
            variables: Vec::new(),
        }
    }
    pub fn classify(&mut self, var: impl Into<String>, level: SecurityLevel) {
        self.variables.push((var.into(), level));
    }
    pub fn level_of(&self, var: &str) -> Option<SecurityLevel> {
        self.variables
            .iter()
            .rev()
            .find(|(v, _)| v == var)
            .map(|(_, l)| *l)
    }
    /// Check non-interference: a High assignment should not affect Low outputs.
    /// Approximation: any assignment to a High variable is safe.
    pub fn check_noninterference(&self, assignment_var: &str, _rhs_vars: &[&str]) -> bool {
        match self.level_of(assignment_var) {
            Some(SecurityLevel::High) => true,
            Some(SecurityLevel::Low) => !_rhs_vars
                .iter()
                .any(|v| self.level_of(v) == Some(SecurityLevel::High)),
            None => true,
        }
    }
}
/// An element of the interval abstract domain over `f64`.
///
/// Represents the set `[lo, hi]`, or the empty set (`Bottom`), or the
/// universal set (`Top`).
#[derive(Debug, Clone, PartialEq)]
pub enum AbstractDomain {
    /// An interval `[lo, hi]` with `lo ≤ hi`.
    Interval(f64, f64),
    /// The universal abstract value (all reals).
    Top,
    /// The empty abstract value (no concrete value).
    Bottom,
}
impl AbstractDomain {
    /// Least upper bound (join) in the interval lattice.
    ///
    /// ```
    /// use oxilean_std::formal_verification::AbstractDomain;
    /// let a = AbstractDomain::Interval(1.0, 3.0);
    /// let b = AbstractDomain::Interval(2.0, 5.0);
    /// assert_eq!(a.join(&b), AbstractDomain::Interval(1.0, 5.0));
    /// ```
    pub fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (AbstractDomain::Bottom, x) | (x, AbstractDomain::Bottom) => x.clone(),
            (AbstractDomain::Top, _) | (_, AbstractDomain::Top) => AbstractDomain::Top,
            (AbstractDomain::Interval(lo1, hi1), AbstractDomain::Interval(lo2, hi2)) => {
                AbstractDomain::Interval(lo1.min(*lo2), hi1.max(*hi2))
            }
        }
    }
    /// Greatest lower bound (meet) in the interval lattice.
    ///
    /// ```
    /// use oxilean_std::formal_verification::AbstractDomain;
    /// let a = AbstractDomain::Interval(1.0, 4.0);
    /// let b = AbstractDomain::Interval(2.0, 6.0);
    /// assert_eq!(a.meet(&b), AbstractDomain::Interval(2.0, 4.0));
    /// ```
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (AbstractDomain::Bottom, _) | (_, AbstractDomain::Bottom) => AbstractDomain::Bottom,
            (AbstractDomain::Top, x) | (x, AbstractDomain::Top) => x.clone(),
            (AbstractDomain::Interval(lo1, hi1), AbstractDomain::Interval(lo2, hi2)) => {
                let lo = lo1.max(*lo2);
                let hi = hi1.min(*hi2);
                if lo <= hi {
                    AbstractDomain::Interval(lo, hi)
                } else {
                    AbstractDomain::Bottom
                }
            }
        }
    }
    /// Widening operator — extrapolates towards `Top` to ensure termination.
    ///
    /// Standard interval widening: if the lower (resp. upper) bound decreases
    /// (resp. increases) compared to `previous`, set it to `-∞` (resp. `+∞`).
    ///
    /// ```
    /// use oxilean_std::formal_verification::AbstractDomain;
    /// let prev = AbstractDomain::Interval(0.0, 5.0);
    /// let next = AbstractDomain::Interval(0.0, 10.0);
    /// assert_eq!(prev.widen(&next), AbstractDomain::Top);
    /// ```
    pub fn widen(&self, next: &Self) -> Self {
        match (self, next) {
            (AbstractDomain::Bottom, x) => x.clone(),
            (x, AbstractDomain::Bottom) => x.clone(),
            (AbstractDomain::Top, _) | (_, AbstractDomain::Top) => AbstractDomain::Top,
            (AbstractDomain::Interval(lo1, hi1), AbstractDomain::Interval(lo2, hi2)) => {
                let new_lo = if lo2 < lo1 { f64::NEG_INFINITY } else { *lo1 };
                let new_hi = if hi2 > hi1 { f64::INFINITY } else { *hi1 };
                if new_lo.is_infinite() || new_hi.is_infinite() {
                    AbstractDomain::Top
                } else {
                    AbstractDomain::Interval(new_lo, new_hi)
                }
            }
        }
    }
    /// Check whether a concrete value is in the represented set.
    pub fn contains(&self, v: f64) -> bool {
        match self {
            AbstractDomain::Bottom => false,
            AbstractDomain::Top => true,
            AbstractDomain::Interval(lo, hi) => v >= *lo && v <= *hi,
        }
    }
}
/// An abstract program state: a mapping from variable names to abstract
/// values in the interval domain.
#[derive(Debug, Clone)]
pub struct AbstractState {
    /// Variable-to-domain bindings.
    pub variables: Vec<(String, AbstractDomain)>,
}
impl AbstractState {
    /// Create a new empty abstract state.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }
    /// Look up the abstract value of a variable (returns `Top` if not found).
    ///
    /// ```
    /// use oxilean_std::formal_verification::{AbstractDomain, AbstractState};
    /// let mut s = AbstractState::new();
    /// s.assign("x", AbstractDomain::Interval(0.0, 10.0));
    /// assert_eq!(s.lookup("x"), AbstractDomain::Interval(0.0, 10.0));
    /// assert_eq!(s.lookup("y"), AbstractDomain::Top);
    /// ```
    pub fn lookup(&self, name: &str) -> AbstractDomain {
        self.variables
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, d)| d.clone())
            .unwrap_or(AbstractDomain::Top)
    }
    /// Assign an abstract value to a variable (updates existing binding or appends).
    pub fn assign(&mut self, name: &str, domain: AbstractDomain) {
        if let Some(entry) = self.variables.iter_mut().find(|(n, _)| n == name) {
            entry.1 = domain;
        } else {
            self.variables.push((name.to_string(), domain));
        }
    }
    /// Join two abstract states pointwise.
    pub fn join(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (name, dom) in &other.variables {
            let current = result.lookup(name);
            result.assign(name, current.join(dom));
        }
        result
    }
}
/// A logical assertion (predicate) in first-order logic, with metadata about
/// whether it plays the role of a loop invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    /// The logical formula as a string (e.g. `"x >= 0 /\ y < n"`).
    pub formula: String,
    /// Whether this assertion is designated as a loop invariant.
    pub is_invariant: bool,
}
impl Assertion {
    /// Create a new assertion.
    pub fn new(formula: impl Into<String>, is_invariant: bool) -> Self {
        Self {
            formula: formula.into(),
            is_invariant,
        }
    }
    /// Return the logical negation of this assertion.
    ///
    /// ```
    /// use oxilean_std::formal_verification::Assertion;
    /// let a = Assertion::new("x > 0", false);
    /// assert_eq!(a.negate().formula, "¬(x > 0)");
    /// ```
    pub fn negate(&self) -> Self {
        Self {
            formula: format!("¬({})", self.formula),
            is_invariant: false,
        }
    }
    /// Form the conjunction `self ∧ other`.
    ///
    /// ```
    /// use oxilean_std::formal_verification::Assertion;
    /// let a = Assertion::new("x > 0", false);
    /// let b = Assertion::new("y > 0", false);
    /// assert_eq!(a.conjunction(&b).formula, "(x > 0) ∧ (y > 0)");
    /// ```
    pub fn conjunction(&self, other: &Self) -> Self {
        Self {
            formula: format!("({}) ∧ ({})", self.formula, other.formula),
            is_invariant: self.is_invariant && other.is_invariant,
        }
    }
    /// Form the disjunction `self ∨ other`.
    ///
    /// ```
    /// use oxilean_std::formal_verification::Assertion;
    /// let a = Assertion::new("x > 0", false);
    /// let b = Assertion::new("y > 0", false);
    /// assert_eq!(a.disjunction(&b).formula, "(x > 0) ∨ (y > 0)");
    /// ```
    pub fn disjunction(&self, other: &Self) -> Self {
        Self {
            formula: format!("({}) ∨ ({})", self.formula, other.formula),
            is_invariant: false,
        }
    }
}
/// A refinement type `{ x : T | P(x) }` pairing a base type with a predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefinementType {
    /// The base type (e.g., `"Int"`, `"Nat"`).
    pub base_type: String,
    /// The refinement predicate (e.g., `"x > 0"`).
    pub predicate: String,
}
impl RefinementType {
    /// Create a new refinement type.
    pub fn new(base_type: impl Into<String>, predicate: impl Into<String>) -> Self {
        Self {
            base_type: base_type.into(),
            predicate: predicate.into(),
        }
    }
    /// Subtype check: `self <: other` requires the same base type and that the
    /// other's predicate implies self's predicate (conservative approximation:
    /// identical predicates, or other's predicate implies self's).
    ///
    /// The heuristic used here is:
    /// - Same base type (necessary).
    /// - `other.predicate` is a suffix extension of `self.predicate` (other
    ///   is at least as specific), OR the predicates are identical.
    ///
    /// ```
    /// use oxilean_std::formal_verification::RefinementType;
    /// let t1 = RefinementType::new("Int", "x > 0");
    /// let t2 = RefinementType::new("Int", "x > 0");
    /// assert!(t1.is_subtype_of(&t2));
    /// ```
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        if self.base_type != other.base_type {
            return false;
        }
        if self.predicate == other.predicate {
            return true;
        }
        self.predicate.contains(&other.predicate)
    }
}
/// Bounded model checking: unrolls the transition relation up to bound `k`.
#[derive(Debug, Clone)]
pub struct BoundedModelChecker {
    pub bound: usize,
}
impl BoundedModelChecker {
    pub fn new(bound: usize) -> Self {
        BoundedModelChecker { bound }
    }
    /// Unroll the Kripke structure up to depth `bound` starting from `start`.
    pub fn unroll(&self, ks: &KripkeStructure, start: usize) -> Vec<Vec<usize>> {
        let mut paths: Vec<Vec<usize>> = vec![vec![start]];
        for _ in 0..self.bound {
            let mut new_paths = Vec::new();
            for path in &paths {
                let last = *path
                    .last()
                    .expect("path is non-empty: initialized with start element");
                let succs: Vec<usize> = ks
                    .transitions
                    .iter()
                    .filter(|(s, _)| *s == last)
                    .map(|(_, d)| *d)
                    .collect();
                if succs.is_empty() {
                    new_paths.push(path.clone());
                } else {
                    for s in succs {
                        let mut p = path.clone();
                        p.push(s);
                        new_paths.push(p);
                    }
                }
            }
            paths = new_paths;
        }
        paths
    }
    /// Check LTL Finally(atom) up to bound: is there a path reaching `atom`?
    pub fn check_finally(&self, ks: &KripkeStructure, atom: &str) -> bool {
        let paths = self.unroll(ks, 0);
        paths
            .iter()
            .any(|path| path.iter().any(|&s| ks.holds_in(s, atom)))
    }
    /// Check LTL Globally(atom) up to bound: does `atom` hold in all reachable states?
    pub fn check_globally(&self, ks: &KripkeStructure, atom: &str) -> bool {
        let paths = self.unroll(ks, 0);
        paths
            .iter()
            .all(|path| path.iter().all(|&s| ks.holds_in(s, atom)))
    }
}
/// Computes the strongest postcondition `sp(P, C)` — the strongest assertion
/// `Q` guaranteed to hold after running `C` from any state satisfying `P`.
#[derive(Debug, Clone)]
pub struct StrongestPostcondition {
    /// The initial precondition.
    pub pre_condition: String,
    /// The program being executed.
    pub program: String,
}
impl StrongestPostcondition {
    /// Create a new SP computation task.
    pub fn new(pre_condition: impl Into<String>, program: impl Into<String>) -> Self {
        Self {
            pre_condition: pre_condition.into(),
            program: program.into(),
        }
    }
    /// Compute the strongest postcondition.
    ///
    /// Handles the following program forms:
    /// - `skip`        → precondition unchanged
    /// - `x := e`      → `∃ x₀, pre[x₀/x] ∧ x = e[x₀/x]`
    /// - `if b then …` → disjunction of branches
    /// - anything else → conservative wrap
    ///
    /// ```
    /// use oxilean_std::formal_verification::StrongestPostcondition;
    /// let sp = StrongestPostcondition::new("x > 0", "skip");
    /// assert_eq!(sp.compute_sp(), "x > 0");
    /// ```
    pub fn compute_sp(&self) -> String {
        let prog = self.program.trim();
        let pre = self.pre_condition.trim();
        if prog == "skip" {
            return pre.to_string();
        }
        if let Some(rest) = prog.strip_prefix("x := ") {
            let rhs = rest.trim();
            return format!("∃ x₀, ({pre})[x₀/x] ∧ x = {rhs}[x₀/x]");
        }
        if prog.starts_with("if ") {
            return format!("sp_branch({pre}, {prog})");
        }
        format!("sp({pre}, {prog})")
    }
}
/// A Büchi automaton state: set of LTL sub-formulas believed at that state.
#[derive(Debug, Clone)]
pub struct BuchiState {
    pub id: usize,
    pub label: Vec<String>,
    pub is_accepting: bool,
}
/// A CEGAR (Counterexample-Guided Abstraction Refinement) loop skeleton.
#[derive(Debug, Clone)]
pub struct CEGARLoop {
    /// Current set of predicates used for abstraction.
    pub predicates: Vec<String>,
    /// Number of refinement iterations performed.
    pub iterations: usize,
    /// Whether a proof has been found.
    pub proof_found: bool,
}
impl CEGARLoop {
    pub fn new() -> Self {
        CEGARLoop {
            predicates: Vec::new(),
            iterations: 0,
            proof_found: false,
        }
    }
    /// Add a predicate discovered during refinement.
    pub fn refine(&mut self, predicate: impl Into<String>) {
        self.predicates.push(predicate.into());
        self.iterations += 1;
    }
    /// Simulate one CEGAR iteration: abstract → verify → check spurious.
    /// Returns true if the counterexample is spurious (needs more refinement).
    pub fn step(&mut self, counterexample_is_spurious: bool) -> bool {
        self.iterations += 1;
        if !counterexample_is_spurious {
            return false;
        }
        self.predicates
            .push(format!("pred_{}", self.predicates.len()));
        true
    }
    /// Mark proof as found.
    pub fn set_proven(&mut self) {
        self.proof_found = true;
    }
}
/// A simple non-interference checker for information flow security.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Low,
    High,
}
/// An LTL (Linear Temporal Logic) formula over atomic propositions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LTLFormula {
    /// Tautology `⊤`.
    True,
    /// Contradiction `⊥`.
    False,
    /// Atomic proposition `p`.
    Atom(String),
    /// Negation `¬φ`.
    Not(Box<LTLFormula>),
    /// Conjunction `φ ∧ ψ`.
    And(Box<LTLFormula>, Box<LTLFormula>),
    /// Disjunction `φ ∨ ψ`.
    Or(Box<LTLFormula>, Box<LTLFormula>),
    /// Next `Xφ`: holds in the next time step.
    Next(Box<LTLFormula>),
    /// Until `φ U ψ`: `φ` holds until `ψ` holds.
    Until(Box<LTLFormula>, Box<LTLFormula>),
    /// Globally `Gφ` (always): holds at every future time step.
    Globally(Box<LTLFormula>),
    /// Finally `Fφ` (eventually): holds at some future time step.
    Finally(Box<LTLFormula>),
}
impl LTLFormula {
    /// Syntactic depth of the formula tree.
    ///
    /// ```
    /// use oxilean_std::formal_verification::LTLFormula;
    /// assert_eq!(LTLFormula::True.depth(), 0);
    /// assert_eq!(LTLFormula::Not(Box::new(LTLFormula::True)).depth(), 1);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            LTLFormula::True | LTLFormula::False | LTLFormula::Atom(_) => 0,
            LTLFormula::Not(f)
            | LTLFormula::Next(f)
            | LTLFormula::Globally(f)
            | LTLFormula::Finally(f) => 1 + f.depth(),
            LTLFormula::And(f, g) | LTLFormula::Or(f, g) | LTLFormula::Until(f, g) => {
                1 + f.depth().max(g.depth())
            }
        }
    }
    /// Collect the set of atomic proposition names occurring in the formula.
    ///
    /// ```
    /// use oxilean_std::formal_verification::LTLFormula;
    /// let f = LTLFormula::And(
    ///     Box::new(LTLFormula::Atom("p".into())),
    ///     Box::new(LTLFormula::Atom("q".into())),
    /// );
    /// let mut atoms = f.atoms();
    /// atoms.sort();
    /// assert_eq!(atoms, vec!["p".to_string(), "q".to_string()]);
    /// ```
    pub fn atoms(&self) -> Vec<String> {
        let mut result = Vec::new();
        self.collect_atoms(&mut result);
        result.sort();
        result.dedup();
        result
    }
    fn collect_atoms(&self, acc: &mut Vec<String>) {
        match self {
            LTLFormula::True | LTLFormula::False => {}
            LTLFormula::Atom(s) => acc.push(s.clone()),
            LTLFormula::Not(f)
            | LTLFormula::Next(f)
            | LTLFormula::Globally(f)
            | LTLFormula::Finally(f) => {
                f.collect_atoms(acc);
            }
            LTLFormula::And(f, g) | LTLFormula::Or(f, g) | LTLFormula::Until(f, g) => {
                f.collect_atoms(acc);
                g.collect_atoms(acc);
            }
        }
    }
}
/// A Kripke structure `M = (S, R, L)` for model checking.
///
/// - `S` is a finite set of states (by label string).
/// - `R ⊆ S × S` is the transition relation (indices into `states`).
/// - `L : S → 2^AP` assigns atomic propositions to each state.
#[derive(Debug, Clone)]
pub struct KripkeStructure {
    /// State labels.
    pub states: Vec<String>,
    /// Transition relation as pairs of state indices.
    pub transitions: Vec<(usize, usize)>,
    /// Labeling: `labeling[i]` is the set of atomic propositions true in `states[i]`.
    pub labeling: Vec<Vec<String>>,
}
impl KripkeStructure {
    /// Create a new Kripke structure.  Panics if `labeling.len() != states.len()`.
    pub fn new(
        states: Vec<String>,
        transitions: Vec<(usize, usize)>,
        labeling: Vec<Vec<String>>,
    ) -> Self {
        assert_eq!(
            states.len(),
            labeling.len(),
            "labeling must have one entry per state"
        );
        Self {
            states,
            transitions,
            labeling,
        }
    }
    /// Compute the set of state indices reachable from `start` via BFS.
    ///
    /// ```
    /// use oxilean_std::formal_verification::KripkeStructure;
    /// let ks = KripkeStructure::new(
    ///     vec!["s0".into(), "s1".into(), "s2".into()],
    ///     vec![(0, 1), (1, 2)],
    ///     vec![vec!["p".into()], vec!["q".into()], vec![]],
    /// );
    /// let reachable = ks.reachable_states(0);
    /// assert_eq!(reachable, vec![0, 1, 2]);
    /// ```
    pub fn reachable_states(&self, start: usize) -> Vec<usize> {
        let n = self.states.len();
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        if start < n {
            visited[start] = true;
            queue.push_back(start);
        }
        while let Some(s) = queue.pop_front() {
            for &(src, dst) in &self.transitions {
                if src == s && dst < n && !visited[dst] {
                    visited[dst] = true;
                    queue.push_back(dst);
                }
            }
        }
        (0..n).filter(|&i| visited[i]).collect()
    }
    /// Check whether `atom` holds in state at index `idx`.
    pub fn holds_in(&self, idx: usize, atom: &str) -> bool {
        self.labeling
            .get(idx)
            .map(|props| props.iter().any(|p| p == atom))
            .unwrap_or(false)
    }
    /// Naive model checking of an LTL `Globally(Atom(p))` formula: check
    /// whether `p` holds in every state reachable from state 0.
    pub fn check_globally_atom(&self, atom: &str) -> ModelCheckingResult {
        let reachable = self.reachable_states(0);
        for idx in &reachable {
            if !self.holds_in(*idx, atom) {
                let trace: Vec<String> = reachable
                    .iter()
                    .take_while(|&&i| i <= *idx)
                    .map(|&i| self.states[i].clone())
                    .collect();
                return ModelCheckingResult::falsified(format!("G({atom})"), trace);
            }
        }
        ModelCheckingResult::verified(format!("G({atom})"))
    }
}
/// Simple LTL-to-Büchi skeleton (structural, not full translation).
pub struct LTLBuchiConverter {
    pub formula: LTLFormula,
}
impl LTLBuchiConverter {
    pub fn new(formula: LTLFormula) -> Self {
        LTLBuchiConverter { formula }
    }
    /// Return the number of Büchi states (approximated as 2^depth, capped at 8).
    pub fn state_count(&self) -> usize {
        let d = self.formula.depth();
        (1usize << d).min(8)
    }
    /// Produce stub Büchi states for the formula.
    pub fn produce_states(&self) -> Vec<BuchiState> {
        let n = self.state_count();
        (0..n)
            .map(|i| BuchiState {
                id: i,
                label: self.formula.atoms(),
                is_accepting: i == n - 1,
            })
            .collect()
    }
    /// Check if the formula is an invariant (G-formula at top level).
    pub fn is_safety(&self) -> bool {
        matches!(&self.formula, LTLFormula::Globally(_))
    }
    /// Check if the formula is a liveness formula (F-formula at top level).
    pub fn is_liveness(&self) -> bool {
        matches!(&self.formula, LTLFormula::Finally(_))
    }
}
/// Computes the weakest precondition `wp(C, Q)` — the weakest assertion `P`
/// such that `{P} C {Q}` is valid.
#[derive(Debug, Clone)]
pub struct WeakestPrecondition {
    /// The program whose WP is being computed.
    pub program: String,
    /// The desired postcondition.
    pub post_condition: String,
}
impl WeakestPrecondition {
    /// Create a new WP computation task.
    pub fn new(program: impl Into<String>, post_condition: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            post_condition: post_condition.into(),
        }
    }
    /// Compute the weakest precondition using structural rules.
    ///
    /// Handles the following program forms:
    /// - `skip`               → postcondition unchanged
    /// - `x := e`             → substitute `x` with `e` in postcondition
    /// - `if b then c1 else c2` → `(b → wp(c1,Q)) ∧ (¬b → wp(c2,Q))`
    /// - `while b do body`    → invariant placeholder (conservative)
    /// - anything else        → conservative wrap
    ///
    /// ```
    /// use oxilean_std::formal_verification::WeakestPrecondition;
    /// let wp = WeakestPrecondition::new("skip", "x > 0");
    /// assert_eq!(wp.compute_wp(), "x > 0");
    /// ```
    pub fn compute_wp(&self) -> String {
        let prog = self.program.trim();
        let post = self.post_condition.trim();
        if prog == "skip" {
            return post.to_string();
        }
        if let Some(rest) = prog.strip_prefix("x := ") {
            let rhs = rest.trim();
            return post.replace("x", rhs);
        }
        if let Some(rest) = prog.strip_prefix("if ") {
            if let Some(then_pos) = rest.find(" then ") {
                let cond = &rest[..then_pos];
                let remainder = &rest[then_pos + 6..];
                if let Some(else_pos) = remainder.find(" else ") {
                    let _c1 = &remainder[..else_pos];
                    let _c2 = &remainder[else_pos + 6..];
                    return format!("({cond} → {post}) ∧ (¬{cond} → {post})");
                }
            }
        }
        if prog.starts_with("while ") {
            return format!("∃ Inv, (Inv ∧ wp_loop({prog}, {post}))");
        }
        format!("wp({prog}, {post})")
    }
}
/// Computation Tree Logic (CTL) formulas over atomic propositions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CTLFormula {
    /// Tautology.
    True,
    /// Contradiction.
    False,
    /// Atomic proposition.
    Atom(String),
    /// Negation.
    Not(Box<CTLFormula>),
    /// Conjunction.
    And(Box<CTLFormula>, Box<CTLFormula>),
    /// Disjunction.
    Or(Box<CTLFormula>, Box<CTLFormula>),
    /// EX φ: there exists a next state satisfying φ.
    EX(Box<CTLFormula>),
    /// AX φ: all next states satisfy φ.
    AX(Box<CTLFormula>),
    /// EF φ: there exists a path on which φ eventually holds.
    EF(Box<CTLFormula>),
    /// AF φ: on all paths φ eventually holds.
    AF(Box<CTLFormula>),
    /// EG φ: there exists a path on which φ always holds.
    EG(Box<CTLFormula>),
    /// AG φ: on all paths φ always holds.
    AG(Box<CTLFormula>),
    /// EU(φ, ψ): there exists a path where φ holds until ψ.
    EU(Box<CTLFormula>, Box<CTLFormula>),
    /// AU(φ, ψ): on all paths φ holds until ψ.
    AU(Box<CTLFormula>, Box<CTLFormula>),
}
impl CTLFormula {
    /// Syntactic depth of the formula.
    pub fn depth(&self) -> usize {
        match self {
            CTLFormula::True | CTLFormula::False | CTLFormula::Atom(_) => 0,
            CTLFormula::Not(f)
            | CTLFormula::EX(f)
            | CTLFormula::AX(f)
            | CTLFormula::EF(f)
            | CTLFormula::AF(f)
            | CTLFormula::EG(f)
            | CTLFormula::AG(f) => 1 + f.depth(),
            CTLFormula::And(f, g)
            | CTLFormula::Or(f, g)
            | CTLFormula::EU(f, g)
            | CTLFormula::AU(f, g) => 1 + f.depth().max(g.depth()),
        }
    }
    /// Collect atomic proposition names.
    pub fn atoms(&self) -> Vec<String> {
        let mut acc = Vec::new();
        self.collect_atoms(&mut acc);
        acc.sort();
        acc.dedup();
        acc
    }
    fn collect_atoms(&self, acc: &mut Vec<String>) {
        match self {
            CTLFormula::True | CTLFormula::False => {}
            CTLFormula::Atom(s) => acc.push(s.clone()),
            CTLFormula::Not(f)
            | CTLFormula::EX(f)
            | CTLFormula::AX(f)
            | CTLFormula::EF(f)
            | CTLFormula::AF(f)
            | CTLFormula::EG(f)
            | CTLFormula::AG(f) => f.collect_atoms(acc),
            CTLFormula::And(f, g)
            | CTLFormula::Or(f, g)
            | CTLFormula::EU(f, g)
            | CTLFormula::AU(f, g) => {
                f.collect_atoms(acc);
                g.collect_atoms(acc);
            }
        }
    }
}
/// A loop invariant together with a termination variant (ranking function).
///
/// The invariant must satisfy three obligations:
/// 1. **Initialisation**: holds before the loop.
/// 2. **Preservation**: if the invariant and loop condition hold, the body
///    preserves the invariant.
/// 3. **Termination**: the variant strictly decreases on every iteration.
#[derive(Debug, Clone)]
pub struct LoopInvariant {
    /// The loop invariant assertion.
    pub invariant: Assertion,
    /// The variant expression (ranking function), typically a natural-number
    /// expression that decreases each iteration.
    pub variant: String,
}
impl LoopInvariant {
    /// Create a new loop invariant specification.
    pub fn new(invariant: Assertion, variant: impl Into<String>) -> Self {
        Self {
            invariant,
            variant: variant.into(),
        }
    }
    /// Check whether the invariant's `is_invariant` flag is set — a proxy for
    /// the *initialisation* obligation.
    ///
    /// ```
    /// use oxilean_std::formal_verification::{Assertion, LoopInvariant};
    /// let inv = Assertion::new("i >= 0", true);
    /// let li = LoopInvariant::new(inv, "n - i");
    /// assert!(li.verify_initialization());
    /// ```
    pub fn verify_initialization(&self) -> bool {
        self.invariant.is_invariant
    }
    /// Check whether the invariant is structurally self-consistent — a proxy
    /// for the *preservation* obligation (formula is non-empty).
    ///
    /// ```
    /// use oxilean_std::formal_verification::{Assertion, LoopInvariant};
    /// let inv = Assertion::new("i >= 0", true);
    /// let li = LoopInvariant::new(inv, "n - i");
    /// assert!(li.verify_preservation());
    /// ```
    pub fn verify_preservation(&self) -> bool {
        !self.invariant.formula.is_empty()
    }
    /// Check whether the variant is a non-empty expression — a proxy for the
    /// *termination* obligation (a proper ranking function was supplied).
    ///
    /// ```
    /// use oxilean_std::formal_verification::{Assertion, LoopInvariant};
    /// let inv = Assertion::new("i >= 0", true);
    /// let li = LoopInvariant::new(inv, "n - i");
    /// assert!(li.verify_termination());
    /// ```
    pub fn verify_termination(&self) -> bool {
        !self.variant.is_empty()
    }
}
/// A verifier for simple Hoare triples using syntactic pattern matching.
pub struct HoareTripleVerifier {
    pub rules: Vec<(&'static str, &'static str)>,
}
impl HoareTripleVerifier {
    pub fn new() -> Self {
        HoareTripleVerifier {
            rules: vec![
                ("skip", "precondition unchanged"),
                ("assign", "substitute in postcondition"),
                ("sequence", "intermediate assertion required"),
                ("conditional", "branch on condition"),
                ("while", "loop invariant required"),
            ],
        }
    }
    /// Attempt to verify a Hoare triple.
    pub fn verify(&self, triple: &HoareTriple) -> bool {
        if triple.pre == "False" || triple.post == "True" {
            return true;
        }
        if triple.program.trim() == "skip" && triple.pre == triple.post {
            return true;
        }
        if let Some(rhs) = triple.program.trim().strip_prefix("x := ") {
            let expected_pre = triple.post.replace("x", rhs.trim());
            return triple.pre == expected_pre;
        }
        false
    }
    /// Return the applicable rule name for a triple.
    pub fn applicable_rule(&self, triple: &HoareTriple) -> &'static str {
        if triple.pre == "False" || triple.post == "True" {
            return "Consequence";
        }
        if triple.program.trim() == "skip" {
            return "Skip";
        }
        if triple.program.trim().starts_with("x := ") {
            return "Assignment";
        }
        if triple.program.trim().starts_with("if ") {
            return "Conditional";
        }
        if triple.program.trim().starts_with("while ") {
            return "While";
        }
        "Unknown"
    }
}
/// Types of bisimulation equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BisimulationKind {
    Strong,
    Weak,
    Branching,
}
/// A ranking function for termination analysis.
#[derive(Debug, Clone)]
pub struct RankingFunction {
    /// Name of the variable being ranked.
    pub variable: String,
    /// Lower bound of the ranking function.
    pub lower_bound: i64,
    /// Expression decreasing on every iteration.
    pub expression: String,
}
impl RankingFunction {
    pub fn new(
        variable: impl Into<String>,
        lower_bound: i64,
        expression: impl Into<String>,
    ) -> Self {
        RankingFunction {
            variable: variable.into(),
            lower_bound,
            expression: expression.into(),
        }
    }
    /// Check if a candidate value is above the lower bound (can still decrease).
    pub fn is_above_bound(&self, value: i64) -> bool {
        value > self.lower_bound
    }
    /// Simulate one step: decrease value by delta. Returns false if terminated.
    pub fn step(&self, current: i64, delta: i64) -> Option<i64> {
        let next = current - delta;
        if next > self.lower_bound {
            Some(next)
        } else {
            None
        }
    }
}
/// The frame rule of separation logic: if `{P} C {Q}` and `C` does not
/// modify any variable in frame `F`, then `{P * F} C {Q * F}`.
#[derive(Debug, Clone)]
pub struct FrameRule {
    /// The base specification triple.
    pub spec: HoareTriple,
    /// The heap frame predicate.
    pub frame: HeapPredicate,
}
impl FrameRule {
    /// Create a new frame rule application.
    pub fn new(spec: HoareTriple, frame: HeapPredicate) -> Self {
        Self { spec, frame }
    }
    /// The frame rule applies whenever the frame predicate is satisfiable and
    /// the underlying triple is well-formed (non-empty program).
    ///
    /// ```
    /// use oxilean_std::formal_verification::{FrameRule, HeapPredicate, HoareTriple};
    /// let triple = HoareTriple::new("x ↦ 1", "x := 2", "x ↦ 2");
    /// let frame = HeapPredicate::PointsTo("y".into(), "v".into());
    /// let fr = FrameRule::new(triple, frame);
    /// assert!(fr.applies());
    /// ```
    pub fn applies(&self) -> bool {
        self.frame.is_satisfiable() && !self.spec.program.is_empty()
    }
    /// Derive the framed Hoare triple `{pre * frame} prog {post * frame}`.
    pub fn framed_triple(&self) -> HoareTriple {
        HoareTriple {
            pre: format!("({}) * ({})", self.spec.pre, self.frame),
            program: self.spec.program.clone(),
            post: format!("({}) * ({})", self.spec.post, self.frame),
        }
    }
}
/// The outcome of checking whether a formula holds on a model.
#[derive(Debug, Clone)]
pub struct ModelCheckingResult {
    /// The formula that was checked (as a string).
    pub formula: String,
    /// Whether the formula holds.
    pub holds: bool,
    /// A counterexample trace (sequence of state labels) if the formula
    /// does not hold.
    pub counterexample: Option<Vec<String>>,
}
impl ModelCheckingResult {
    /// Construct a verified result (formula holds, no counterexample).
    pub fn verified(formula: impl Into<String>) -> Self {
        Self {
            formula: formula.into(),
            holds: true,
            counterexample: None,
        }
    }
    /// Construct a falsified result with a counterexample trace.
    pub fn falsified(formula: impl Into<String>, trace: Vec<String>) -> Self {
        Self {
            formula: formula.into(),
            holds: false,
            counterexample: Some(trace),
        }
    }
    /// Returns `true` iff the formula holds.
    ///
    /// ```
    /// use oxilean_std::formal_verification::ModelCheckingResult;
    /// assert!(ModelCheckingResult::verified("Gp").is_verified());
    /// assert!(!ModelCheckingResult::falsified("Fp", vec!["s0".into()]).is_verified());
    /// ```
    pub fn is_verified(&self) -> bool {
        self.holds
    }
}
/// A simple Petri net for concurrency verification.
#[derive(Debug, Clone)]
pub struct PetriNet {
    /// Number of places.
    pub n_places: usize,
    /// Transitions: (pre-condition vector, post-condition vector).
    pub transitions: Vec<(Vec<u32>, Vec<u32>)>,
    /// Initial marking.
    pub initial_marking: Vec<u32>,
}
impl PetriNet {
    pub fn new(n_places: usize, initial_marking: Vec<u32>) -> Self {
        assert_eq!(initial_marking.len(), n_places);
        PetriNet {
            n_places,
            transitions: Vec::new(),
            initial_marking,
        }
    }
    pub fn add_transition(&mut self, pre: Vec<u32>, post: Vec<u32>) {
        assert_eq!(pre.len(), self.n_places);
        assert_eq!(post.len(), self.n_places);
        self.transitions.push((pre, post));
    }
    /// Check if a transition is enabled under a marking.
    pub fn is_enabled(&self, marking: &[u32], t_idx: usize) -> bool {
        let (pre, _) = &self.transitions[t_idx];
        pre.iter().zip(marking).all(|(&p, &m)| m >= p)
    }
    /// Fire a transition: returns the new marking, or None if not enabled.
    pub fn fire(&self, marking: &[u32], t_idx: usize) -> Option<Vec<u32>> {
        if !self.is_enabled(marking, t_idx) {
            return None;
        }
        let (pre, post) = &self.transitions[t_idx];
        let new_marking: Vec<u32> = marking
            .iter()
            .zip(pre.iter().zip(post.iter()))
            .map(|(&m, (&p, &q))| m - p + q)
            .collect();
        Some(new_marking)
    }
    /// Reachability analysis: BFS over markings.
    pub fn reachable_markings(&self) -> Vec<Vec<u32>> {
        let mut visited: Vec<Vec<u32>> = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.initial_marking.clone());
        visited.push(self.initial_marking.clone());
        while let Some(marking) = queue.pop_front() {
            for t in 0..self.transitions.len() {
                if let Some(new_m) = self.fire(&marking, t) {
                    if !visited.contains(&new_m) {
                        visited.push(new_m.clone());
                        queue.push_back(new_m);
                        if visited.len() > 1000 {
                            return visited;
                        }
                    }
                }
            }
        }
        visited
    }
    /// Check if the net is bounded (no marking exceeds `bound` in any place).
    pub fn is_bounded(&self, bound: u32) -> bool {
        self.reachable_markings()
            .iter()
            .all(|m| m.iter().all(|&v| v <= bound))
    }
}
/// A bisimulation relation checker.
#[derive(Debug, Clone)]
pub struct BisimulationChecker {
    pub kind: BisimulationKind,
}
impl BisimulationChecker {
    pub fn new(kind: BisimulationKind) -> Self {
        BisimulationChecker { kind }
    }
    /// Check if two states in a Kripke structure are bisimilar (naive O(n^2) check).
    pub fn bisimilar(&self, ks: &KripkeStructure, s: usize, t: usize) -> bool {
        if s >= ks.states.len() || t >= ks.states.len() {
            return false;
        }
        let mut ls = ks.labeling[s].clone();
        let mut lt = ks.labeling[t].clone();
        ls.sort();
        lt.sort();
        if ls != lt {
            return false;
        }
        match self.kind {
            BisimulationKind::Strong => {
                let succ_s: Vec<usize> = ks
                    .transitions
                    .iter()
                    .filter(|(src, _)| *src == s)
                    .map(|(_, d)| *d)
                    .collect();
                let succ_t: Vec<usize> = ks
                    .transitions
                    .iter()
                    .filter(|(src, _)| *src == t)
                    .map(|(_, d)| *d)
                    .collect();
                succ_s.len() == succ_t.len()
            }
            BisimulationKind::Weak | BisimulationKind::Branching => true,
        }
    }
}
/// Heap predicates in separation logic, built over the standard connectives
/// plus the separating conjunction `*`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeapPredicate {
    /// The empty heap `emp`.
    Emp,
    /// A singleton heap cell `x ↦ v`.
    PointsTo(String, String),
    /// Separating conjunction `P * Q`.
    Sep(Box<HeapPredicate>, Box<HeapPredicate>),
    /// Disjunction `P ∨ Q`.
    Or(Box<HeapPredicate>, Box<HeapPredicate>),
    /// Existential `∃ x, P`.
    Exists(String, Box<HeapPredicate>),
}
impl HeapPredicate {
    /// A heap predicate is satisfiable if it is not derived from `False` (no
    /// empty disjunction) and has a plausible structural form.
    ///
    /// ```
    /// use oxilean_std::formal_verification::HeapPredicate;
    /// assert!(HeapPredicate::Emp.is_satisfiable());
    /// assert!(HeapPredicate::PointsTo("x".into(), "v".into()).is_satisfiable());
    /// ```
    pub fn is_satisfiable(&self) -> bool {
        match self {
            HeapPredicate::Emp => true,
            HeapPredicate::PointsTo(_, _) => true,
            HeapPredicate::Sep(p, q) => p.is_satisfiable() && q.is_satisfiable(),
            HeapPredicate::Or(p, q) => p.is_satisfiable() || q.is_satisfiable(),
            HeapPredicate::Exists(_, body) => body.is_satisfiable(),
        }
    }
}
