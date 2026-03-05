//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Decidable;
use super::functions::*;
use std::fmt;

/// A decision tagged with a human-readable name for debugging.
#[derive(Debug, Clone)]
pub struct NamedDecision {
    /// The name of the proposition.
    pub name: String,
    /// The decision result.
    pub decision: Decision<()>,
}
impl NamedDecision {
    /// Create a new named decision.
    pub fn new(name: impl Into<String>, decision: Decision<()>) -> Self {
        Self {
            name: name.into(),
            decision,
        }
    }
    /// Returns `true` if the decision is positive.
    pub fn is_true(&self) -> bool {
        self.decision.is_true()
    }
    /// Display a summary of the named decision.
    pub fn summary(&self) -> String {
        let verdict = if self.is_true() { "✓" } else { "✗" };
        format!("[{verdict}] {}", self.name)
    }
}
/// Reflection between `bool` and decidable propositions.
///
/// Given a `Decision<()>`, `BoolReflect` bridges the computational `Bool`
/// world and the propositional `Prop` world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolReflect {
    /// The boolean is `true` and the proposition holds.
    IsTrue,
    /// The boolean is `false` and the proposition does not hold.
    IsFalse,
}
impl BoolReflect {
    /// Convert from a `Decision<()>`.
    pub fn from_decision(d: &Decision<()>) -> Self {
        if d.is_true() {
            BoolReflect::IsTrue
        } else {
            BoolReflect::IsFalse
        }
    }
    /// Convert to a plain `bool`.
    pub fn to_bool(&self) -> bool {
        *self == BoolReflect::IsTrue
    }
}
/// A counter that tracks how many decisions have been made and their outcomes.
#[derive(Clone, Debug, Default)]
pub struct DecidableCounter {
    /// Number of positive decisions.
    pub true_count: usize,
    /// Number of negative decisions.
    pub false_count: usize,
}
impl DecidableCounter {
    /// Create a new counter.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a decision.
    pub fn record(&mut self, d: &Decision<()>) {
        if d.is_true() {
            self.true_count += 1;
        } else {
            self.false_count += 1;
        }
    }
    /// Total decisions recorded.
    pub fn total(&self) -> usize {
        self.true_count + self.false_count
    }
    /// Percentage of true decisions (0.0 to 100.0).
    pub fn true_rate(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.true_count as f64 / self.total() as f64) * 100.0
        }
    }
    /// Whether all decisions were positive.
    pub fn all_true(&self) -> bool {
        self.false_count == 0 && self.total() > 0
    }
    /// Whether all decisions were negative.
    pub fn all_false(&self) -> bool {
        self.true_count == 0 && self.total() > 0
    }
    /// Reset the counter.
    pub fn reset(&mut self) {
        self.true_count = 0;
        self.false_count = 0;
    }
}
/// A decision about a proposition `P`.
///
/// `Decision::IsTrue` carries a witness (proof) that `P` holds.
/// `Decision::IsFalse` carries an explanation that it does not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision<P> {
    /// The proposition holds.
    IsTrue(P),
    /// The proposition does not hold; the string is an explanatory message.
    IsFalse(String),
}
impl<P> Decision<P> {
    /// Returns `true` if the decision is positive.
    pub fn is_true(&self) -> bool {
        matches!(self, Decision::IsTrue(_))
    }
    /// Returns `true` if the decision is negative.
    pub fn is_false(&self) -> bool {
        matches!(self, Decision::IsFalse(_))
    }
    /// Unwrap the positive proof, panicking if negative.
    pub fn unwrap(self) -> P {
        match self {
            Decision::IsTrue(p) => p,
            Decision::IsFalse(msg) => panic!("Decision::unwrap on IsFalse: {msg}"),
        }
    }
    /// Convert to `Option`, dropping the negative message.
    pub fn into_option(self) -> Option<P> {
        match self {
            Decision::IsTrue(p) => Some(p),
            Decision::IsFalse(_) => None,
        }
    }
    /// Map the positive witness through `f`.
    pub fn map<Q>(self, f: impl FnOnce(P) -> Q) -> Decision<Q> {
        match self {
            Decision::IsTrue(p) => Decision::IsTrue(f(p)),
            Decision::IsFalse(msg) => Decision::IsFalse(msg),
        }
    }
    /// Combine with another `Decision` via conjunction.
    pub fn and<Q>(self, other: Decision<Q>) -> Decision<(P, Q)> {
        match (self, other) {
            (Decision::IsTrue(p), Decision::IsTrue(q)) => Decision::IsTrue((p, q)),
            (Decision::IsFalse(m), _) | (_, Decision::IsFalse(m)) => Decision::IsFalse(m),
        }
    }
    /// Combine with another `Decision` via disjunction (left-biased).
    pub fn or(self, other: Decision<P>) -> Decision<P> {
        match self {
            Decision::IsTrue(_) => self,
            Decision::IsFalse(_) => other,
        }
    }
    /// Negate the decision.
    pub fn negate(self) -> Decision<String>
    where
        P: std::fmt::Debug,
    {
        match self {
            Decision::IsTrue(p) => Decision::IsFalse(format!("expected false but got: {p:?}")),
            Decision::IsFalse(msg) => Decision::IsTrue(msg),
        }
    }
    /// Apply `f` if true, returning a new decision.
    pub fn flat_map<Q>(self, f: impl FnOnce(P) -> Decision<Q>) -> Decision<Q> {
        match self {
            Decision::IsTrue(p) => f(p),
            Decision::IsFalse(msg) => Decision::IsFalse(msg),
        }
    }
    /// Return `default` if false.
    pub fn unwrap_or(self, default: P) -> P {
        match self {
            Decision::IsTrue(p) => p,
            Decision::IsFalse(_) => default,
        }
    }
    /// Apply a fallback function if false.
    pub fn unwrap_or_else(self, f: impl FnOnce(String) -> P) -> P {
        match self {
            Decision::IsTrue(p) => p,
            Decision::IsFalse(msg) => f(msg),
        }
    }
}
impl Decision<bool> {
    /// Convert a simple bool decision to a native `bool`.
    pub fn to_bool(&self) -> bool {
        match self {
            Decision::IsTrue(b) => *b,
            Decision::IsFalse(_) => false,
        }
    }
}
/// A sequential chain of decisions where each step can depend on the previous.
#[derive(Clone, Debug)]
pub struct DecisionChain {
    steps: Vec<(String, Decision<()>)>,
}
impl DecisionChain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    /// Add a step to the chain.
    pub fn step(mut self, name: impl Into<String>, d: Decision<()>) -> Self {
        self.steps.push((name.into(), d));
        self
    }
    /// Whether all steps passed.
    pub fn all_passed(&self) -> bool {
        self.steps.iter().all(|(_, d)| d.is_true())
    }
    /// The first failing step, if any.
    pub fn first_failure(&self) -> Option<&str> {
        self.steps
            .iter()
            .find(|(_, d)| d.is_false())
            .map(|(name, _)| name.as_str())
    }
    /// Number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Count passing steps.
    pub fn passed_count(&self) -> usize {
        self.steps.iter().filter(|(_, d)| d.is_true()).count()
    }
    /// Count failing steps.
    pub fn failed_count(&self) -> usize {
        self.steps.iter().filter(|(_, d)| d.is_false()).count()
    }
}
/// Decision procedure (DPLL, tableaux, resolution, etc.).
#[allow(dead_code)]
pub struct DecisionProcedureExt {
    /// Name of the procedure
    procedure_name: String,
    /// Completeness: if formula is satisfiable, procedure finds it
    complete: bool,
    /// Soundness: if procedure says satisfiable, it is
    sound: bool,
    /// Complexity class
    complexity: String,
}
/// A negation proof: evidence that `P` does not hold.
///
/// Wraps an explanatory message at the Rust meta-level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not<P> {
    /// Message explaining why `P` is false.
    pub message: String,
    _marker: std::marker::PhantomData<P>,
}
impl<P> Not<P> {
    /// Construct a negation proof with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            _marker: std::marker::PhantomData,
        }
    }
    /// Double negation elimination: `¬¬P` is not constructively valid in
    /// general, but we model it here as just returning the message.
    pub fn message(&self) -> &str {
        &self.message
    }
}
/// Extended decidable proposition structure.
#[allow(dead_code)]
pub struct DecidablePropExt {
    /// Name of the proposition
    name: String,
    /// Whether this is constructively decidable (witness-carrying)
    constructive: bool,
    /// Associated Boolean reflection
    bool_reflect: Option<bool>,
}
/// Decidable membership for a finite set.
#[allow(dead_code)]
pub struct DecidableSetExt<T: Eq> {
    elements: Vec<T>,
    /// Decidable equality on T used for membership
    dec_eq: std::marker::PhantomData<T>,
}
/// A decidable equality check wrapper.
#[derive(Clone, Debug)]
pub struct EqDecision<T> {
    pub(super) lhs: T,
    pub(super) rhs: T,
}
impl<T: PartialEq + std::fmt::Debug> EqDecision<T> {
    /// Create a new equality decision.
    pub fn new(lhs: T, rhs: T) -> Self {
        Self { lhs, rhs }
    }
}
/// A finite set with decidable membership.
///
/// Backed by a `Vec<T>` with no duplicates (in insertion order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteSet<T: PartialEq> {
    elems: Vec<T>,
}
impl<T: PartialEq> FiniteSet<T> {
    /// Create an empty finite set.
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }
    /// Insert `x` if not already present. Returns `true` if inserted.
    pub fn insert(&mut self, x: T) -> bool {
        if self.elems.contains(&x) {
            false
        } else {
            self.elems.push(x);
            true
        }
    }
    /// Returns `true` if `x` is in the set.
    pub fn contains(&self, x: &T) -> bool {
        self.elems.contains(x)
    }
    /// Remove `x` from the set. Returns `true` if it was present.
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(pos) = self.elems.iter().position(|e| e == x) {
            self.elems.remove(pos);
            true
        } else {
            false
        }
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.elems.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
    /// Iterate over elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elems.iter()
    }
    /// Compute the union with another set.
    pub fn union(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = self.clone();
        for x in &other.elems {
            result.insert(x.clone());
        }
        result
    }
    /// Compute the intersection with another set.
    pub fn intersection(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        Self {
            elems: self
                .elems
                .iter()
                .filter(|x| other.contains(x))
                .cloned()
                .collect(),
        }
    }
    /// Compute the set difference `self \ other`.
    pub fn difference(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        Self {
            elems: self
                .elems
                .iter()
                .filter(|x| !other.contains(x))
                .cloned()
                .collect(),
        }
    }
    /// Subset check: is `self ⊆ other`?
    pub fn is_subset(&self, other: &Self) -> bool {
        self.elems.iter().all(|x| other.contains(x))
    }
}
/// A lookup table of named decidable propositions.
///
/// Useful for the elaborator to cache computed decisions.
#[derive(Debug, Clone, Default)]
pub struct DecisionTable {
    entries: Vec<(String, Decision<()>)>,
}
impl DecisionTable {
    /// Create an empty table.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert or overwrite a named decision.
    pub fn insert(&mut self, name: impl Into<String>, d: Decision<()>) {
        let name = name.into();
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &name) {
            entry.1 = d;
        } else {
            self.entries.push((name, d));
        }
    }
    /// Look up a named decision.
    pub fn lookup(&self, name: &str) -> Option<&Decision<()>> {
        self.entries.iter().find(|(k, _)| k == name).map(|(_, v)| v)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Decision<()>)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v))
    }
}
/// A function-based decidable predicate.
#[derive(Clone)]
pub struct FnPred<A, F>(pub(super) F, pub(super) std::marker::PhantomData<A>)
where
    F: Fn(&A) -> bool;
impl<A, F: Fn(&A) -> bool> FnPred<A, F> {
    /// Wrap a closure as a decidable predicate.
    pub fn new(f: F) -> Self {
        Self(f, std::marker::PhantomData)
    }
}
/// Semi-decidable (recursively enumerable) predicate.
#[allow(dead_code)]
pub struct SemiDecidableExt<T> {
    /// The underlying predicate (returns Some on positive witness, diverges otherwise)
    predicate: std::marker::PhantomData<T>,
    /// True if this is a proper decision procedure (terminates on both yes/no)
    is_total: bool,
}
/// A decidable ordering check wrapper.
#[derive(Clone, Debug)]
pub struct LeDecision<T> {
    pub(super) lhs: T,
    pub(super) rhs: T,
}
impl<T: PartialOrd> LeDecision<T> {
    /// Create a new `≤` decision.
    pub fn new(lhs: T, rhs: T) -> Self {
        Self { lhs, rhs }
    }
}
/// Halting oracle (exists only non-constructively, for axiom purposes).
#[allow(dead_code)]
pub struct HaltingOracleExt {
    /// Name of the oracle
    oracle_name: String,
    /// This oracle is undecidable by Church-Turing thesis
    undecidable: bool,
}
/// A closed integer interval `[lo, hi]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interval {
    /// Lower bound (inclusive).
    pub lo: i64,
    /// Upper bound (inclusive).
    pub hi: i64,
}
impl Interval {
    /// Create an interval.
    pub fn new(lo: i64, hi: i64) -> Self {
        Self { lo, hi }
    }
    /// A single-element interval.
    pub fn point(x: i64) -> Self {
        Self { lo: x, hi: x }
    }
    /// Whether `x` is in the interval.
    pub fn contains(&self, x: i64) -> bool {
        x >= self.lo && x <= self.hi
    }
    /// Whether the interval is empty (lo > hi).
    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
    /// Length of the interval.
    pub fn len(&self) -> u64 {
        if self.is_empty() {
            0
        } else {
            (self.hi - self.lo) as u64 + 1
        }
    }
    /// Intersect with another interval.
    pub fn intersect(&self, other: &Interval) -> Interval {
        Interval {
            lo: self.lo.max(other.lo),
            hi: self.hi.min(other.hi),
        }
    }
    /// Union with another interval (assumes overlapping or adjacent).
    pub fn union(&self, other: &Interval) -> Interval {
        Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }
    /// Decide whether `x` is in the interval.
    pub fn decide_contains(&self, x: i64) -> Decision<()> {
        if self.contains(x) {
            Decision::IsTrue(())
        } else {
            Decision::IsFalse(format!("{x} not in [{}, {}]", self.lo, self.hi))
        }
    }
}
