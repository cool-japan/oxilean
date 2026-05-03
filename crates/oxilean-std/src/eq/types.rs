//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::env_builder::*;
use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::fmt;

/// A database of known equalities, used during elaboration.
///
/// This stores equalities as `(lhs_name, rhs_name, proof_term)` triples,
/// allowing the elaborator to look up existing proofs of equality rather than
/// re-deriving them.
#[derive(Debug, Clone, Default)]
pub struct EqualityDatabase {
    entries: Vec<(Name, Name, Expr)>,
}
impl EqualityDatabase {
    /// Create an empty database.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an equality `lhs = rhs` with the given proof term.
    pub fn register(&mut self, lhs: Name, rhs: Name, proof: Expr) {
        self.entries.push((lhs, rhs, proof));
    }
    /// Look up a proof of `lhs = rhs`.
    ///
    /// Returns the stored proof term if found, searching also the symmetric
    /// direction and wrapping with `Eq.symm` as needed.
    pub fn lookup(&self, lhs: &Name, rhs: &Name) -> Option<Expr> {
        for (l, r, p) in &self.entries {
            if l == lhs && r == rhs {
                return Some(p.clone());
            }
            if l == rhs && r == lhs {
                return Some(app(
                    app(app(app(var("Eq.symm"), var("_")), var("_")), var("_")),
                    p.clone(),
                ));
            }
        }
        None
    }
    /// Look up a transitive equality `a = c` via `a = b` and `b = c`.
    pub fn lookup_trans(&self, a: &Name, c: &Name) -> Option<Expr> {
        for (l1, r1, p1) in &self.entries {
            if l1 == a {
                for (l2, r2, p2) in &self.entries {
                    if l2 == r1 && r2 == c {
                        return Some(eq_trans(
                            var("_"),
                            var(l1.to_string().as_str()),
                            var(r1.to_string().as_str()),
                            var(r2.to_string().as_str()),
                            p1.clone(),
                            p2.clone(),
                        ));
                    }
                }
            }
        }
        None
    }
    /// Total number of registered equalities.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if no equalities are registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over all registered equality triples.
    pub fn iter(&self) -> impl Iterator<Item = &(Name, Name, Expr)> {
        self.entries.iter()
    }
    /// Clear all registered equalities.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// A setoid morphism: a function respecting a setoid equivalence.
#[allow(dead_code)]
pub struct SetoidMorphism<A, B> {
    /// The underlying function.
    pub func: fn(A) -> B,
    /// Predicate asserting `f` respects the source equivalence.
    pub respects: fn(&A, &A, bool) -> bool,
}
#[allow(dead_code)]
impl<A: Clone, B> SetoidMorphism<A, B> {
    /// Create a new setoid morphism.
    pub fn new(func: fn(A) -> B, respects: fn(&A, &A, bool) -> bool) -> Self {
        Self { func, respects }
    }
    /// Apply the morphism.
    pub fn apply(&self, a: A) -> B {
        (self.func)(a)
    }
    /// Verify this morphism respects equivalence for a pair.
    pub fn verify_respects(&self, a1: &A, a2: &A, are_equiv: bool) -> bool {
        (self.respects)(a1, a2, are_equiv)
    }
}
/// An equality-based rewrite rule.
#[derive(Debug, Clone)]
pub struct EqRewriteRule {
    /// The name of this rule.
    pub name: oxilean_kernel::Name,
    /// Left-hand side pattern.
    pub lhs: oxilean_kernel::Expr,
    /// Right-hand side replacement.
    pub rhs: oxilean_kernel::Expr,
    /// Whether this rule can be applied in reverse.
    pub reversible: bool,
}
impl EqRewriteRule {
    /// Create a new rewrite rule.
    pub fn new(
        name: oxilean_kernel::Name,
        lhs: oxilean_kernel::Expr,
        rhs: oxilean_kernel::Expr,
    ) -> Self {
        Self {
            name,
            lhs,
            rhs,
            reversible: false,
        }
    }
    /// Mark the rule as reversible.
    pub fn make_reversible(mut self) -> Self {
        self.reversible = true;
        self
    }
    /// The reversed rule (rhs → lhs).
    pub fn reversed(&self) -> Option<Self> {
        if self.reversible {
            Some(Self {
                name: self.name.clone(),
                lhs: self.rhs.clone(),
                rhs: self.lhs.clone(),
                reversible: true,
            })
        } else {
            None
        }
    }
    /// Check if the rule matches a given expression (syntactically).
    pub fn matches(&self, expr: &oxilean_kernel::Expr) -> bool {
        &self.lhs == expr
    }
    /// Apply the rule to an expression, returning the rewritten form.
    pub fn apply(&self, expr: &oxilean_kernel::Expr) -> Option<oxilean_kernel::Expr> {
        if self.matches(expr) {
            Some(self.rhs.clone())
        } else {
            None
        }
    }
}
/// A setoid: a Rust type equipped with an explicit equivalence relation.
///
/// Mirrors the Lean 4 `Setoid` typeclass for use at the meta level.
#[allow(dead_code)]
pub struct SetoidInstance<T> {
    /// A representative finite sample of carrier elements.
    pub carrier: Vec<T>,
    /// The equivalence relation: `equiv(a, b)` iff `a ~ b`.
    pub equiv: fn(&T, &T) -> bool,
}
#[allow(dead_code)]
impl<T> SetoidInstance<T> {
    /// Create a new setoid from a carrier sample and an equivalence function.
    pub fn new(carrier: Vec<T>, equiv: fn(&T, &T) -> bool) -> Self {
        Self { carrier, equiv }
    }
    /// Check whether `a` and `b` are equivalent.
    pub fn are_equiv(&self, a: &T, b: &T) -> bool {
        (self.equiv)(a, b)
    }
    /// Verify reflexivity for all carrier elements.
    pub fn check_refl(&self) -> bool {
        self.carrier.iter().all(|a| (self.equiv)(a, a))
    }
    /// Verify symmetry for all pairs in the carrier.
    pub fn check_symm(&self) -> bool {
        for a in &self.carrier {
            for b in &self.carrier {
                if (self.equiv)(a, b) && !(self.equiv)(b, a) {
                    return false;
                }
            }
        }
        true
    }
    /// Verify transitivity for all triples in the carrier.
    pub fn check_trans(&self) -> bool {
        for a in &self.carrier {
            for b in &self.carrier {
                for c in &self.carrier {
                    if (self.equiv)(a, b) && (self.equiv)(b, c) && !(self.equiv)(a, c) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A chain of propositional equalities `a₀ = a₁ = … = aₙ`.
///
/// Useful for representing `calc`-block-style equality chains during
/// elaboration.
#[derive(Debug, Clone)]
pub struct EqChain {
    /// The type of all elements.
    pub ty: Expr,
    /// The sequence of steps; `steps\[i\].rhs == steps[i+1].lhs`.
    pub steps: Vec<PropEq>,
}
impl EqChain {
    /// Create an empty chain for a given type.
    pub fn new(ty: Expr) -> Self {
        Self {
            ty,
            steps: Vec::new(),
        }
    }
    /// Extend the chain with a new equality step.
    ///
    /// Panics if the step's lhs does not match the last rhs.
    pub fn push(&mut self, step: PropEq) {
        if let Some(last) = self.steps.last() {
            assert_eq!(last.rhs, step.lhs, "equality chain is not connected");
        }
        self.steps.push(step);
    }
    /// Collapse the chain to a single equality (if non-empty).
    pub fn collapse(self) -> Option<PropEq> {
        let mut iter = self.steps.into_iter();
        let first = iter.next()?;
        iter.try_fold(first, |acc, step| acc.trans(step))
    }
    /// Returns `true` if the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Number of steps in the chain.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
/// The result of a decidable proposition.
///
/// This is the Rust-level analogue of Lean 4's `Decidable` type, capturing
/// whether a proposition holds along with the computational evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionResult<P> {
    /// The proposition holds with proof `p`.
    IsTrue(P),
    /// The proposition does not hold with proof of negation.
    IsFalse(String),
}
impl<P> DecisionResult<P> {
    /// Returns `true` if the proposition holds.
    pub fn is_true(&self) -> bool {
        matches!(self, DecisionResult::IsTrue(_))
    }
    /// Returns `true` if the proposition does not hold.
    pub fn is_false(&self) -> bool {
        matches!(self, DecisionResult::IsFalse(_))
    }
    /// Convert to `Option`, keeping the proof if present.
    pub fn into_option(self) -> Option<P> {
        match self {
            DecisionResult::IsTrue(p) => Some(p),
            DecisionResult::IsFalse(_) => None,
        }
    }
    /// Map the positive proof.
    pub fn map<Q>(self, f: impl FnOnce(P) -> Q) -> DecisionResult<Q> {
        match self {
            DecisionResult::IsTrue(p) => DecisionResult::IsTrue(f(p)),
            DecisionResult::IsFalse(msg) => DecisionResult::IsFalse(msg),
        }
    }
    /// Combine two positive decisions with `And`.
    pub fn and<Q>(self, other: DecisionResult<Q>) -> DecisionResult<(P, Q)> {
        match (self, other) {
            (DecisionResult::IsTrue(p), DecisionResult::IsTrue(q)) => {
                DecisionResult::IsTrue((p, q))
            }
            (DecisionResult::IsFalse(msg), _) | (_, DecisionResult::IsFalse(msg)) => {
                DecisionResult::IsFalse(msg)
            }
        }
    }
    /// Combine two decisions with `Or` (left bias).
    pub fn or(self, other: DecisionResult<P>) -> DecisionResult<P> {
        match self {
            DecisionResult::IsTrue(_) => self,
            DecisionResult::IsFalse(_) => other,
        }
    }
}
impl<P: std::fmt::Display> DecisionResult<P> {
    /// Display the decision result as a human-readable string.
    pub fn display(&self) -> String {
        match self {
            DecisionResult::IsTrue(p) => format!("isTrue ({p})"),
            DecisionResult::IsFalse(msg) => format!("isFalse ({msg})"),
        }
    }
}
/// A Leibniz equality coercion at the Rust meta level.
///
/// `LeibnizEq<A, B>` encodes a coercion `A → B` witnessing type equality.
#[allow(dead_code)]
pub struct LeibnizEq<A, B> {
    /// The coercion function.
    pub coerce: fn(A) -> B,
}
#[allow(dead_code)]
impl<T> LeibnizEq<T, T> {
    /// Construct a trivial `LeibnizEq` by reflexivity (identity coercion).
    pub fn refl() -> Self {
        Self { coerce: |x| x }
    }
    /// Apply the coercion.
    pub fn apply(&self, a: T) -> T {
        (self.coerce)(a)
    }
}
/// A witness that two Rust values are equal.
///
/// Constructed only when the equality holds, providing a type-indexed proof
/// analogous to Lean 4's `Eq.refl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualityWitness<T> {
    /// The value for which equality was witnessed.
    pub value: T,
}
impl<T: PartialEq + Clone> EqualityWitness<T> {
    /// Attempt to construct an `EqualityWitness` for `a == b`.
    ///
    /// Returns `Some` only when `a == b`.
    pub fn try_new(a: &T, b: &T) -> Option<Self> {
        if a == b {
            Some(EqualityWitness { value: a.clone() })
        } else {
            None
        }
    }
    /// Use the witness to cast one reference to another.
    ///
    /// Because the witness guarantees equality, we can use `a` as `b`.
    pub fn cast<'a>(&self, a: &'a T) -> &'a T {
        a
    }
}
/// Propositional equality record, carrying the type and both sides.
///
/// Used to represent equality propositions `a = b : α` as data structures
/// during elaboration and pretty-printing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropEq {
    /// The common type of both sides.
    pub ty: Expr,
    /// The left-hand side.
    pub lhs: Expr,
    /// The right-hand side.
    pub rhs: Expr,
}
impl PropEq {
    /// Construct a new propositional equality.
    pub fn new(ty: Expr, lhs: Expr, rhs: Expr) -> Self {
        Self { ty, lhs, rhs }
    }
    /// Construct the reflexivity equality `a = a`.
    pub fn refl(ty: Expr, a: Expr) -> Self {
        Self::new(ty, a.clone(), a)
    }
    /// Returns `true` if this is a reflexivity equality (`lhs == rhs`).
    pub fn is_refl(&self) -> bool {
        self.lhs == self.rhs
    }
    /// Swap sides: `a = b` becomes `b = a`.
    pub fn symm(self) -> Self {
        Self::new(self.ty, self.rhs, self.lhs)
    }
    /// Chain two equalities: `a = b` and `b = c` become `a = c`.
    ///
    /// Returns `None` if the rhs of `self` differs from the lhs of `other`.
    pub fn trans(self, other: Self) -> Option<Self> {
        if self.rhs == other.lhs && self.ty == other.ty {
            Some(Self::new(self.ty, self.lhs, other.rhs))
        } else {
            None
        }
    }
    /// Display as a human-readable string.
    pub fn display(&self) -> String {
        format!("{:?} = {:?}", self.lhs, self.rhs)
    }
}
/// A decidable equality instance wrapper for a specific Rust type.
#[allow(dead_code)]
pub struct DecidableEqInstance<T> {
    /// The name of the type.
    pub type_name: &'static str,
    /// The decision procedure.
    pub decide: fn(&T, &T) -> DecisionResult<()>,
}
#[allow(dead_code)]
impl<T: PartialEq> DecidableEqInstance<T> {
    /// Construct a `DecidableEqInstance` for any `PartialEq` type.
    pub fn for_type(type_name: &'static str) -> Self {
        Self {
            type_name,
            decide: |a, b| {
                if a == b {
                    DecisionResult::IsTrue(())
                } else {
                    DecisionResult::IsFalse("values differ".to_string())
                }
            },
        }
    }
    /// Apply the decision procedure.
    pub fn decide_eq(&self, a: &T, b: &T) -> DecisionResult<()> {
        (self.decide)(a, b)
    }
    /// Return `true` iff `a == b`.
    pub fn is_eq(&self, a: &T, b: &T) -> bool {
        self.decide_eq(a, b).is_true()
    }
}
/// A builder for equality proofs chained via transitivity.
#[derive(Debug, Clone)]
pub struct EqBuilder {
    ty: oxilean_kernel::Expr,
    current: oxilean_kernel::Expr,
    steps: Vec<(oxilean_kernel::Expr, oxilean_kernel::Expr)>,
}
impl EqBuilder {
    /// Start from an expression.
    pub fn start(ty: oxilean_kernel::Expr, start: oxilean_kernel::Expr) -> Self {
        Self {
            ty,
            current: start,
            steps: Vec::new(),
        }
    }
    /// Add a step `current = next` with the given proof.
    pub fn step(mut self, next: oxilean_kernel::Expr, proof: oxilean_kernel::Expr) -> Self {
        self.steps.push((next.clone(), proof));
        self.current = next;
        self
    }
    /// Build the combined propositional equality chain.
    pub fn build(self) -> Option<PropEq> {
        if self.steps.is_empty() {
            return None;
        }
        let first_rhs = self.steps[0].0.clone();
        let mut chain = PropEq::new(self.ty.clone(), self.current.clone(), first_rhs);
        for (rhs, _proof) in self.steps.into_iter().skip(1) {
            let next_eq = PropEq::new(self.ty.clone(), chain.rhs.clone(), rhs);
            chain = chain.trans(next_eq)?;
        }
        Some(chain)
    }
    /// Current expression.
    pub fn current(&self) -> &oxilean_kernel::Expr {
        &self.current
    }
    /// Number of steps.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
}
/// A database of equality rewrite rules.
#[derive(Debug, Clone, Default)]
pub struct RewriteRuleDb {
    rules: Vec<EqRewriteRule>,
}
impl RewriteRuleDb {
    /// Create an empty database.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a rule.
    pub fn add(&mut self, rule: EqRewriteRule) {
        self.rules.push(rule);
    }
    /// Find the first rule that matches `expr`.
    pub fn find_match(&self, expr: &oxilean_kernel::Expr) -> Option<&EqRewriteRule> {
        self.rules.iter().find(|r| r.matches(expr))
    }
    /// Apply all matching rules once (returns all results).
    pub fn apply_all(&self, expr: &oxilean_kernel::Expr) -> Vec<oxilean_kernel::Expr> {
        self.rules.iter().filter_map(|r| r.apply(expr)).collect()
    }
    /// Number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Remove a rule by name.
    pub fn remove(&mut self, name: &oxilean_kernel::Name) {
        self.rules.retain(|r| &r.name != name);
    }
}
/// A heterogeneous equality witness between values of potentially different types.
#[allow(dead_code)]
pub struct HeterogeneousEq<A, B> {
    /// The left-hand value.
    pub left: A,
    /// The right-hand value.
    pub right: B,
    /// A human-readable justification.
    pub justification: &'static str,
}
#[allow(dead_code)]
impl<A, B> HeterogeneousEq<A, B> {
    /// Construct a heterogeneous equality witness.
    pub fn new(left: A, right: B, justification: &'static str) -> Self {
        Self {
            left,
            right,
            justification,
        }
    }
}
#[allow(dead_code)]
impl<A: Clone + PartialEq> HeterogeneousEq<A, A> {
    /// When both sides have the same type, check propositional equality.
    pub fn is_homogeneous_eq(&self) -> bool {
        self.left == self.right
    }
    /// Extract a homogeneous `EqualityWitness` if values are equal.
    pub fn to_homogeneous(&self) -> Option<EqualityWitness<A>> {
        EqualityWitness::try_new(&self.left, &self.right)
    }
}
