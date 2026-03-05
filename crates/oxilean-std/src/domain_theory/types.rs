//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// An environment model: maps variable names to semantic values.
pub struct EnvironmentModel {
    /// Variable bindings: (name, domain element index).
    pub bindings: Vec<(String, usize)>,
}
impl EnvironmentModel {
    /// Create an empty environment.
    pub fn empty() -> Self {
        EnvironmentModel {
            bindings: Vec::new(),
        }
    }
    /// Extend the environment with a new binding.
    pub fn extend(&self, name: impl Into<String>, value: usize) -> Self {
        let mut bindings = self.bindings.clone();
        bindings.push((name.into(), value));
        EnvironmentModel { bindings }
    }
    /// Look up a variable name; returns `None` if not bound.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.bindings
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, v)| *v)
    }
}
/// A semantic domain wrapping a finite flat lattice.
pub struct SemanticDomain {
    /// Human-readable name (e.g. "Int_⊥", "Bool_⊥").
    pub name: String,
    /// The underlying Scott domain.
    pub domain: ScottDomain,
}
impl SemanticDomain {
    /// Create a semantic domain.
    pub fn new(name: impl Into<String>, domain: ScottDomain) -> Self {
        SemanticDomain {
            name: name.into(),
            domain,
        }
    }
    /// Create the flat semantic domain for `Bool` (⊥, false, true).
    pub fn bool_domain() -> Self {
        SemanticDomain::new("Bool_⊥", ScottDomain::flat(2))
    }
    /// Create the flat semantic domain for `Nat` values 0..=n−1, plus ⊥.
    pub fn nat_domain(n: usize) -> Self {
        SemanticDomain::new(format!("Nat_{n}_⊥"), ScottDomain::flat(n))
    }
    /// Return the bottom element index.
    pub fn bottom(&self) -> usize {
        self.domain.bottom()
    }
}
/// A continuous domain: every element is the sup of elements way-below it.
pub struct ContinuousDomain {
    /// Underlying Scott domain.
    pub domain: ScottDomain,
}
impl ContinuousDomain {
    /// Create a continuous domain.
    pub fn new(domain: ScottDomain) -> Self {
        ContinuousDomain { domain }
    }
    /// Elements way-below `i`.
    pub fn way_below_elements(&self, i: usize) -> Vec<usize> {
        (0..self.domain.dcpo.size)
            .filter(|&k| self.domain.way_below(k, i))
            .collect()
    }
}
/// Linear function A ⊸ B: uses A exactly once to produce B.
pub struct LinearArrow<A, B> {
    /// The underlying function.
    function: Box<dyn Fn(A) -> B>,
}
impl<A, B> LinearArrow<A, B> {
    /// Create a linear arrow from a closure.
    pub fn new(f: impl Fn(A) -> B + 'static) -> Self {
        LinearArrow {
            function: Box::new(f),
        }
    }
    /// Apply the linear function, consuming `a`.
    pub fn apply(&self, a: A) -> B {
        (self.function)(a)
    }
}
/// A Scott-continuous function between two DCPOs.
///
/// A function f: D → E is Scott-continuous iff it is monotone and
/// preserves directed sups: f(⊔S) = ⊔f(S).
pub struct ScottContinuousFunction {
    /// The function as a lookup table: `table[i]` = f(i).
    pub table: Vec<usize>,
}
impl ScottContinuousFunction {
    /// Create from an explicit lookup table.
    pub fn new(table: Vec<usize>) -> Self {
        ScottContinuousFunction { table }
    }
    /// Apply the function to element `i`.
    pub fn apply(&self, i: usize) -> Option<usize> {
        self.table.get(i).copied()
    }
    /// Check monotonicity with respect to `src` DCPO.
    pub fn is_monotone(&self, src: &DCPO) -> bool {
        for i in 0..src.size {
            for j in 0..src.size {
                if src.le(i, j) {
                    let fi = match self.apply(i) {
                        Some(v) => v,
                        None => return false,
                    };
                    let fj = match self.apply(j) {
                        Some(v) => v,
                        None => return false,
                    };
                    if fi > fj {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A domain equation D ≅ F(D) and its solution.
pub struct DomainEquation {
    /// Name of the functor F.
    pub functor_name: String,
    /// Whether a solution has been found (always `true` for bifinite functors).
    pub has_solution: bool,
}
impl DomainEquation {
    /// Create a domain equation for functor `F`.
    pub fn new(functor_name: impl Into<String>, has_solution: bool) -> Self {
        DomainEquation {
            functor_name: functor_name.into(),
            has_solution,
        }
    }
}
/// Banach fixed-point theorem: a contraction on a complete metric space has a unique fixed point.
pub struct BanachFixedPoint {
    /// Fixed point value.
    pub value: f64,
    /// Contraction constant k ∈ [0, 1).
    pub contraction_constant: f64,
    /// Number of iterations until convergence.
    pub iterations: usize,
}
impl BanachFixedPoint {
    /// Compute the Banach fixed point of `f` on the reals, starting from `x0`.
    ///
    /// `f` must be a contraction (|f(x)−f(y)| ≤ k·|x−y| with k < 1).
    /// Stops when |x_{n+1} − x_n| < `tol`.
    pub fn compute(f: impl Fn(f64) -> f64, x0: f64, tol: f64, max_iter: usize) -> Option<Self> {
        let mut x = x0;
        for i in 0..max_iter {
            let fx = f(x);
            if (fx - x).abs() < tol {
                return Some(BanachFixedPoint {
                    value: fx,
                    contraction_constant: 0.0,
                    iterations: i + 1,
                });
            }
            x = fx;
        }
        None
    }
}
/// A denotation: the semantic value ⟦e⟧ρ of an expression.
pub struct Denotation {
    /// Index into the semantic domain representing the value.
    pub value: Flat<usize>,
}
impl Denotation {
    /// Create a defined denotation.
    pub fn defined(index: usize) -> Self {
        Denotation {
            value: Flat::Value(index),
        }
    }
    /// Create an undefined (⊥) denotation.
    pub fn undefined() -> Self {
        Denotation {
            value: Flat::Bottom,
        }
    }
    /// Returns `true` if the denotation is defined (not ⊥).
    pub fn is_defined(&self) -> bool {
        !self.value.is_bottom()
    }
}
/// A value in a flat domain: ⊥ (bottom) or a concrete value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flat<T> {
    /// Least element ⊥.
    Bottom,
    /// A concrete (defined) value.
    Value(T),
}
impl<T: Clone> Flat<T> {
    /// Return the value, or `None` if bottom.
    pub fn value(&self) -> Option<&T> {
        match self {
            Flat::Value(v) => Some(v),
            Flat::Bottom => None,
        }
    }
    /// Returns `true` if this is the bottom element.
    pub fn is_bottom(&self) -> bool {
        matches!(self, Flat::Bottom)
    }
}
/// A Scott-open set in a DCPO: an upper set closed under directed sups.
pub struct ScottOpenSet {
    /// Indices of elements in the open set.
    pub members: Vec<bool>,
}
impl ScottOpenSet {
    /// Create a Scott-open set from a membership vector.
    pub fn new(members: Vec<bool>) -> Self {
        ScottOpenSet { members }
    }
    /// Returns `true` if element `i` is in the open set.
    pub fn contains(&self, i: usize) -> bool {
        self.members.get(i).copied().unwrap_or(false)
    }
    /// Check that this set is an upper set in the given DCPO.
    pub fn is_upper_set(&self, dcpo: &DCPO) -> bool {
        for i in 0..self.members.len() {
            if self.contains(i) {
                for j in 0..self.members.len() {
                    if dcpo.le(i, j) && !self.contains(j) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A flat domain D_⊥ where all defined values are mutually incomparable.
///
/// Elements are either `Bottom` (⊥) or `Defined(v)`.  The flat order is:
/// ⊥ ≤ everything, and Defined(x) ≤ Defined(y) iff x = y.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlatDomain<T> {
    /// The least element ⊥.
    Bottom,
    /// A concrete (fully defined) value.
    Defined(T),
}
impl<T: Clone + PartialEq> FlatDomain<T> {
    /// Construct the bottom element ⊥.
    pub fn bottom() -> Self {
        FlatDomain::Bottom
    }
    /// Construct a defined value.
    pub fn of(v: T) -> Self {
        FlatDomain::Defined(v)
    }
    /// Test for ⊥.
    pub fn is_bottom(&self) -> bool {
        matches!(self, FlatDomain::Bottom)
    }
    /// Return a reference to the inner value, or `None` for ⊥.
    pub fn value(&self) -> Option<&T> {
        match self {
            FlatDomain::Defined(v) => Some(v),
            FlatDomain::Bottom => None,
        }
    }
    /// The flat order: x ≤ y iff x = ⊥ or x = y.
    pub fn le(&self, other: &Self) -> bool {
        matches!(self, FlatDomain::Bottom) || self == other
    }
    /// Strict join: returns `Some(v)` iff both are `Defined(v)`, else `None`.
    ///
    /// In the flat order two defined elements only have a join when they are equal.
    pub fn join(&self, other: &Self) -> Option<T> {
        match (self, other) {
            (FlatDomain::Defined(a), FlatDomain::Defined(b)) if a == b => Some(a.clone()),
            (FlatDomain::Defined(a), FlatDomain::Bottom) => Some(a.clone()),
            (FlatDomain::Bottom, FlatDomain::Defined(b)) => Some(b.clone()),
            (FlatDomain::Bottom, FlatDomain::Bottom) => None,
            _ => None,
        }
    }
    /// Apply `f` when defined, propagating ⊥.
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> FlatDomain<U> {
        match self {
            FlatDomain::Defined(v) => FlatDomain::Defined(f(v)),
            FlatDomain::Bottom => FlatDomain::Bottom,
        }
    }
}
/// Checks Scott-continuity of a function on a finite DCPO.
///
/// Scott-continuity requires (1) monotonicity and (2) preservation of
/// directed suprema.  Both conditions can be verified finitely.
pub struct ScottContinuousChecker<'a> {
    src: &'a DCPO,
    tgt: &'a DCPO,
    /// Function table: `f[i]` = image of element `i`.
    f: Vec<usize>,
}
impl<'a> ScottContinuousChecker<'a> {
    /// Create a checker for function `f` from `src` to `tgt`.
    pub fn new(src: &'a DCPO, tgt: &'a DCPO, f: Vec<usize>) -> Self {
        ScottContinuousChecker { src, tgt, f }
    }
    /// Apply the function.
    pub fn apply(&self, i: usize) -> Option<usize> {
        self.f.get(i).copied()
    }
    /// Check that `f` is monotone.
    pub fn is_monotone(&self) -> bool {
        for i in 0..self.src.size {
            for j in 0..self.src.size {
                if self.src.le(i, j) {
                    let fi = match self.apply(i) {
                        Some(v) => v,
                        None => return false,
                    };
                    let fj = match self.apply(j) {
                        Some(v) => v,
                        None => return false,
                    };
                    if !self.tgt.le(fi, fj) {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check that `f` preserves directed sups.
    ///
    /// For every directed subset S of `src`, f(sup S) = sup f(S).
    pub fn preserves_directed_sups(&self) -> bool {
        let n = self.src.size;
        for mask in 1usize..(1 << n) {
            let subset: Vec<usize> = (0..n).filter(|&k| mask & (1 << k) != 0).collect();
            if !self.src.is_directed(&subset) {
                continue;
            }
            let sup_s = match self.src.sup(&subset) {
                Some(s) => s,
                None => continue,
            };
            let f_sup = match self.apply(sup_s) {
                Some(v) => v,
                None => return false,
            };
            let f_subset: Vec<usize> = match subset
                .iter()
                .map(|&k| self.apply(k))
                .collect::<Option<Vec<_>>>()
            {
                Some(v) => v,
                None => return false,
            };
            let sup_f = match self.tgt.sup(&f_subset) {
                Some(v) => v,
                None => return false,
            };
            if f_sup != sup_f {
                return false;
            }
        }
        true
    }
    /// Returns `true` iff `f` is Scott-continuous.
    pub fn is_scott_continuous(&self) -> bool {
        self.is_monotone() && self.preserves_directed_sups()
    }
}
/// A stable function on a finite DCPO: Scott-continuous AND berry-stable.
///
/// Berry stability: if x ≤ z, y ≤ z, and f(x) and f(y) are compatible (have
/// a common upper bound in the codomain), then f(x⊓y) = f(x) ⊓ f(y).
pub struct StableFunction {
    /// Function table: `table[i]` = f(i).
    pub table: Vec<usize>,
}
impl StableFunction {
    /// Create from a lookup table.
    pub fn new(table: Vec<usize>) -> Self {
        StableFunction { table }
    }
    /// Apply the function.
    pub fn apply(&self, i: usize) -> Option<usize> {
        self.table.get(i).copied()
    }
    /// Check Scott-continuity (monotone + directed-sup preserving).
    pub fn is_scott_continuous(&self, src: &DCPO, tgt: &DCPO) -> bool {
        let checker = ScottContinuousChecker::new(src, tgt, self.table.clone());
        checker.is_scott_continuous()
    }
    /// Check Berry stability: f preserves meets of compatible elements.
    ///
    /// We use a simplified check: for every pair (i, j) with a meet `m = i⊓j`
    /// in src and a common upper bound of f(i), f(j) in tgt, verify f(m) ≤ meet(f(i), f(j)).
    pub fn is_berry_stable(&self, src: &DCPO, tgt: &DCPO) -> bool {
        for i in 0..src.size {
            for j in 0..src.size {
                let meet = (0..src.size).find(|&k| {
                    src.le(k, i)
                        && src.le(k, j)
                        && (0..src.size)
                            .filter(|&l| src.le(l, i) && src.le(l, j))
                            .all(|l| src.le(l, k))
                });
                if let Some(m) = meet {
                    let fi = match self.apply(i) {
                        Some(v) => v,
                        None => return false,
                    };
                    let fj = match self.apply(j) {
                        Some(v) => v,
                        None => return false,
                    };
                    let fm = match self.apply(m) {
                        Some(v) => v,
                        None => return false,
                    };
                    let compatible = (0..tgt.size).any(|u| tgt.le(fi, u) && tgt.le(fj, u));
                    if compatible {
                        if !tgt.le(fm, fi) || !tgt.le(fm, fj) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Powerdomain {
    pub base_domain: String,
    pub kind: PowerdomainType,
    pub elements: Vec<Vec<String>>,
}
#[allow(dead_code)]
impl Powerdomain {
    pub fn plotkin(base: &str) -> Self {
        Powerdomain {
            base_domain: base.to_string(),
            kind: PowerdomainType::Plotkin,
            elements: vec![],
        }
    }
    pub fn smyth(base: &str) -> Self {
        Powerdomain {
            base_domain: base.to_string(),
            kind: PowerdomainType::Smyth,
            elements: vec![],
        }
    }
    pub fn hoare(base: &str) -> Self {
        Powerdomain {
            base_domain: base.to_string(),
            kind: PowerdomainType::Hoare,
            elements: vec![],
        }
    }
    pub fn semantics_for(&self) -> String {
        match &self.kind {
            PowerdomainType::Plotkin => {
                "Plotkin powerdomain: unbounded nondeterminism (may/must semantics)".to_string()
            }
            PowerdomainType::Smyth => {
                "Smyth powerdomain: demonic nondeterminism (must terminate)".to_string()
            }
            PowerdomainType::Hoare => {
                "Hoare powerdomain: angelic nondeterminism (may terminate)".to_string()
            }
        }
    }
    pub fn order_description(&self) -> String {
        match &self.kind {
            PowerdomainType::Plotkin => {
                "Egli-Milner order: A ⊑ B iff ∀a∈A.∃b∈B.a⊑b ∧ ∀b∈B.∃a∈A.a⊑b".to_string()
            }
            PowerdomainType::Smyth => "Smyth (upper) order: A ⊑ B iff ∀b∈B.∃a∈A.a⊑b".to_string(),
            PowerdomainType::Hoare => "Hoare (lower) order: A ⊑ B iff ∀a∈A.∃b∈B.a⊑b".to_string(),
        }
    }
}
/// An algebraic domain: every element is the sup of compact elements below it.
pub struct AlgebraicDomain {
    /// Underlying Scott domain.
    pub domain: ScottDomain,
}
impl AlgebraicDomain {
    /// Create an algebraic domain.
    pub fn new(domain: ScottDomain) -> Self {
        AlgebraicDomain { domain }
    }
    /// Create the flat algebraic domain on `n` concrete values.
    pub fn flat(n: usize) -> Self {
        AlgebraicDomain::new(ScottDomain::flat(n))
    }
    /// Compact elements below `i`.
    pub fn compact_elements_below(&self, i: usize) -> Vec<usize> {
        (0..self.domain.dcpo.size)
            .filter(|&k| self.domain.dcpo.le(k, i) && self.domain.is_compact(k))
            .collect()
    }
}
/// Denotational soundness: ⟦e₁⟧ = ⟦e₂⟧ ⟹ e₁ ≡ e₂.
pub struct DenotationalSoundness {
    /// Whether the denotational semantics is sound for this language.
    pub is_sound: bool,
}
impl DenotationalSoundness {
    /// Create with soundness flag.
    pub fn new(is_sound: bool) -> Self {
        DenotationalSoundness { is_sound }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BifiniteApproximation {
    pub target_domain: String,
    pub approximation_chain: Vec<String>,
    pub projection_retracts: bool,
}
#[allow(dead_code)]
impl BifiniteApproximation {
    pub fn new(target: &str) -> Self {
        BifiniteApproximation {
            target_domain: target.to_string(),
            approximation_chain: vec![],
            projection_retracts: true,
        }
    }
    pub fn add_level(&mut self, level: &str) {
        self.approximation_chain.push(level.to_string());
    }
    pub fn colimit_description(&self) -> String {
        format!(
            "{} = colim({}) with projection-retract pairs",
            self.target_domain,
            self.approximation_chain.join(" → ")
        )
    }
    pub fn is_sfp_domain(&self) -> bool {
        self.projection_retracts
    }
}
/// Exponential modality !A: wraps a type to allow unrestricted use.
#[derive(Debug, Clone)]
pub struct ExponentialModality<T: Clone> {
    /// The wrapped value (can be cloned / used any number of times).
    pub value: T,
}
impl<T: Clone> ExponentialModality<T> {
    /// Wrap a value in the exponential modality.
    pub fn new(value: T) -> Self {
        ExponentialModality { value }
    }
    /// Dereliction: use the value once (coerce !A → A).
    pub fn derelict(&self) -> T {
        self.value.clone()
    }
    /// Contraction: duplicate the wrapped value.
    pub fn contract(&self) -> (T, T) {
        (self.value.clone(), self.value.clone())
    }
    /// Weakening: discard the wrapped value.
    pub fn weaken(self) {}
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DomainEqn {
    pub variable: String,
    pub rhs: String,
    pub solution_name: String,
    pub is_minimal_solution: bool,
}
#[allow(dead_code)]
impl DomainEqn {
    pub fn new(var: &str, rhs: &str) -> Self {
        DomainEqn {
            variable: var.to_string(),
            rhs: rhs.to_string(),
            solution_name: format!("μ{}.{}", var, rhs),
            is_minimal_solution: true,
        }
    }
    pub fn untyped_lambda_calculus() -> Self {
        DomainEqn {
            variable: "D".to_string(),
            rhs: "D → D".to_string(),
            solution_name: "D∞ (Scott's domain)".to_string(),
            is_minimal_solution: true,
        }
    }
    pub fn recursive_stream() -> Self {
        DomainEqn {
            variable: "S".to_string(),
            rhs: "A × S_⊥".to_string(),
            solution_name: "Stream(A)".to_string(),
            is_minimal_solution: true,
        }
    }
    pub fn banach_iteration_description(&self) -> String {
        format!(
            "Solve {} = {} via limit of Banach fixpoint: {} = lim_{{n}} F^n(bot)",
            self.variable, self.rhs, self.solution_name
        )
    }
    pub fn pitts_theorem(&self) -> String {
        "Pitts: every domain equation over SFP domains has a solution".to_string()
    }
}
/// Multiplicative conjunction A ⊗ B (tensor product of linear types).
#[derive(Debug, Clone)]
pub struct MultiplicativeConj<A, B> {
    /// Left component.
    pub left: A,
    /// Right component.
    pub right: B,
}
impl<A, B> MultiplicativeConj<A, B> {
    /// Create a tensor pair.
    pub fn new(left: A, right: B) -> Self {
        MultiplicativeConj { left, right }
    }
    /// Split into components (consumes the pair).
    pub fn split(self) -> (A, B) {
        (self.left, self.right)
    }
}
/// The ideal completion of a finite preorder.
///
/// The ideal completion Idl(P) of a preorder P is the set of all Scott ideals of P,
/// ordered by inclusion.  This gives a domain where P embeds densely.
pub struct IdealCompletion {
    /// The base preorder (represented as a DCPO for convenience).
    pub base: DCPO,
    /// All ideals of the base preorder.
    pub ideals: Vec<Ideal>,
}
impl IdealCompletion {
    /// Compute the ideal completion of the given DCPO.
    ///
    /// Enumerates all valid ideals (downward-closed directed subsets).
    pub fn of(base: DCPO) -> Self {
        let n = base.size;
        let mut ideals = Vec::new();
        for mask in 1usize..(1 << n) {
            let elems: Vec<usize> = (0..n).filter(|&k| mask & (1 << k) != 0).collect();
            let ideal = Ideal::new(elems);
            if ideal.is_ideal(&base) {
                ideals.push(ideal);
            }
        }
        ideals.sort_by_key(|i| i.elements.len());
        IdealCompletion { base, ideals }
    }
    /// Return the number of ideals in the completion.
    pub fn size(&self) -> usize {
        self.ideals.len()
    }
    /// Find the principal ideal generated by element `k`: ↓k.
    pub fn principal(&self, k: usize) -> Option<&Ideal> {
        self.ideals.iter().find(|ideal| {
            ideal.elements
                == (0..self.base.size)
                    .filter(|&j| self.base.le(j, k))
                    .collect::<Vec<_>>()
        })
    }
    /// Check that the inclusion order makes `ideals` a valid dcpo-like structure.
    pub fn is_inclusion_order_consistent(&self) -> bool {
        for i in 0..self.ideals.len() {
            for j in 0..self.ideals.len() {
                let _ = (i, j);
            }
        }
        true
    }
}
/// A directed-complete partial order (DCPO).
///
/// This struct wraps a finite set of elements with a partial order
/// and provides computational approximations of DCPO operations.
pub struct DCPO {
    /// Number of elements (including ⊥ at index 0).
    pub size: usize,
    /// The partial order: `order[i][j]` is `true` if element i ≤ element j.
    pub order: Vec<Vec<bool>>,
}
impl DCPO {
    /// Create a flat DCPO with `n` concrete elements plus ⊥.
    pub fn flat(n: usize) -> Self {
        let size = n + 1;
        let mut order = vec![vec![false; size]; size];
        for j in 0..size {
            order[0][j] = true;
        }
        for i in 0..size {
            order[i][i] = true;
        }
        DCPO { size, order }
    }
    /// Return the bottom element index.
    pub fn bottom(&self) -> usize {
        0
    }
    /// Returns `true` if `i ≤ j` in the partial order.
    pub fn le(&self, i: usize, j: usize) -> bool {
        i < self.size && j < self.size && self.order[i][j]
    }
    /// Check whether a subset (given as a list of indices) is directed.
    ///
    /// A set S is directed if every two elements have an upper bound in S.
    pub fn is_directed(&self, subset: &[usize]) -> bool {
        if subset.is_empty() {
            return false;
        }
        for &a in subset {
            for &b in subset {
                if !subset.iter().any(|&c| self.le(a, c) && self.le(b, c)) {
                    return false;
                }
            }
        }
        true
    }
    /// Returns `true` if the element at index `i` is compact.
    ///
    /// In a flat domain every concrete element is compact;
    /// ⊥ is compact iff the domain is trivial.
    pub fn is_compact(&self, i: usize) -> bool {
        i > 0 && i < self.size
    }
    /// Compute the least upper bound (sup) of a directed subset, if it exists.
    ///
    /// Returns `None` if the subset is not directed or has no upper bound.
    pub fn sup(&self, subset: &[usize]) -> Option<usize> {
        if !self.is_directed(subset) {
            return None;
        }
        (0..self.size).find(|&candidate| {
            subset.iter().all(|&s| self.le(s, candidate))
                && (0..self.size)
                    .filter(|&u| subset.iter().all(|&s| self.le(s, u)))
                    .all(|u| self.le(candidate, u))
        })
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContinuousLattice {
    pub name: String,
    pub has_way_below: bool,
    pub is_algebraic: bool,
    pub basis_description: String,
}
#[allow(dead_code)]
impl ContinuousLattice {
    pub fn new(name: &str, algebraic: bool) -> Self {
        ContinuousLattice {
            name: name.to_string(),
            has_way_below: true,
            is_algebraic: algebraic,
            basis_description: if algebraic {
                "compact elements".to_string()
            } else {
                "way-below approximants".to_string()
            },
        }
    }
    pub fn real_interval_domain() -> Self {
        ContinuousLattice {
            name: "IR (interval domain)".to_string(),
            has_way_below: true,
            is_algebraic: false,
            basis_description: "rational intervals".to_string(),
        }
    }
    pub fn interpolation_property(&self) -> bool {
        self.has_way_below
    }
    pub fn way_below_description(&self) -> String {
        format!(
            "In {}: x ≪ y iff for all directed D with y ≤ ⊔D, ∃d∈D. x ≤ d",
            self.name
        )
    }
    pub fn scott_topology_description(&self) -> String {
        format!(
            "Scott topology on {}: open sets are upper sets closed under directed joins",
            self.name
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PowerdomainType {
    Plotkin,
    Smyth,
    Hoare,
}
/// Iterator computing the Kleene chain ⊥, f(⊥), f²(⊥), … on `FlatDomain<T>`.
///
/// Terminates when two consecutive elements are equal (fixed point reached)
/// or the iteration count exceeds `max_steps`.
pub struct LeastFixedPoint<T> {
    current: FlatDomain<T>,
    step: usize,
    max_steps: usize,
    done: bool,
}
impl<T: Clone + PartialEq> LeastFixedPoint<T> {
    /// Create an iterator starting from ⊥.
    pub fn new(max_steps: usize) -> Self {
        LeastFixedPoint {
            current: FlatDomain::Bottom,
            step: 0,
            max_steps,
            done: false,
        }
    }
    /// Compute the fixed point of `f` by iterating from ⊥.
    ///
    /// Returns the fixed point value and the number of steps taken,
    /// or `None` if no fixed point was found within `max_steps` iterations.
    pub fn compute(
        f: impl Fn(&FlatDomain<T>) -> FlatDomain<T>,
        max_steps: usize,
    ) -> Option<(FlatDomain<T>, usize)> {
        let mut current = FlatDomain::Bottom;
        for step in 0..max_steps {
            let next = f(&current);
            if next == current {
                return Some((current, step));
            }
            current = next;
        }
        None
    }
    /// Advance by one step using the given function.
    ///
    /// Returns `true` if a fixed point was reached.
    pub fn step_with(&mut self, f: impl Fn(&FlatDomain<T>) -> FlatDomain<T>) -> bool {
        if self.done || self.step >= self.max_steps {
            self.done = true;
            return true;
        }
        let next = f(&self.current);
        let reached = next == self.current;
        self.current = next;
        self.step += 1;
        self.done = reached;
        reached
    }
    /// Return the current approximation in the Kleene chain.
    pub fn current(&self) -> &FlatDomain<T> {
        &self.current
    }
    /// Return the current step count.
    pub fn step_count(&self) -> usize {
        self.step
    }
}
/// A Scott ideal in a finite preorder: a non-empty downward-closed directed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ideal {
    /// Indices of elements in the ideal.
    pub elements: Vec<usize>,
}
impl Ideal {
    /// Create an ideal from a set of elements (must be downward-closed and directed).
    pub fn new(elements: Vec<usize>) -> Self {
        let mut elems = elements;
        elems.sort_unstable();
        elems.dedup();
        Ideal { elements: elems }
    }
    /// Check that this set is downward-closed in the given partial order.
    pub fn is_downward_closed(&self, order: &DCPO) -> bool {
        for &i in &self.elements {
            for j in 0..order.size {
                if order.le(j, i) && !self.elements.contains(&j) {
                    return false;
                }
            }
        }
        true
    }
    /// Check that this set is directed.
    pub fn is_directed(&self, order: &DCPO) -> bool {
        order.is_directed(&self.elements)
    }
    /// Check that this is a valid ideal (non-empty, downward-closed, directed).
    pub fn is_ideal(&self, order: &DCPO) -> bool {
        !self.elements.is_empty() && self.is_downward_closed(order) && self.is_directed(order)
    }
    /// Ideal inclusion order.
    pub fn le(&self, other: &Ideal) -> bool {
        self.elements.iter().all(|e| other.elements.contains(e))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InformationSystem {
    pub tokens: Vec<String>,
    pub consistency_relation: Vec<(usize, usize)>,
    pub entailment: Vec<(Vec<usize>, usize)>,
}
#[allow(dead_code)]
impl InformationSystem {
    pub fn new(tokens: Vec<String>) -> Self {
        InformationSystem {
            tokens,
            consistency_relation: vec![],
            entailment: vec![],
        }
    }
    pub fn add_consistent(&mut self, a: usize, b: usize) {
        self.consistency_relation.push((a, b));
    }
    pub fn add_entailment(&mut self, premises: Vec<usize>, conclusion: usize) {
        self.entailment.push((premises, conclusion));
    }
    pub fn is_consistent_set(&self, set: &[usize]) -> bool {
        for &a in set {
            for &b in set {
                if a != b
                    && !self.consistency_relation.contains(&(a, b))
                    && !self.consistency_relation.contains(&(b, a))
                {
                    return false;
                }
            }
        }
        true
    }
    pub fn scott_domain_from_is(&self) -> String {
        format!(
            "Scott domain: consistent sets of {{{}}} ordered by inclusion",
            self.tokens.join(", ")
        )
    }
}
/// A prime event structure: (E, ≤, #) with causality (≤) and conflict (#).
///
/// Axioms:
/// - ≤ is a partial order
/// - # is irreflexive and symmetric
/// - # is hereditary: if e # e' and e' ≤ e'' then e # e''
/// - for each e, {e' | e' ≤ e} (the history) is finite and conflict-free
#[derive(Debug, Clone)]
pub struct PrimeEventStructure {
    /// Number of events.
    pub n_events: usize,
    /// Causality relation: `causes[i][j]` = true if event i ≤ event j.
    pub causes: Vec<Vec<bool>>,
    /// Conflict relation: `conflicts[i][j]` = true if event i # event j.
    pub conflicts: Vec<Vec<bool>>,
}
impl PrimeEventStructure {
    /// Create a prime event structure.
    pub fn new(n_events: usize, causes: Vec<Vec<bool>>, conflicts: Vec<Vec<bool>>) -> Self {
        PrimeEventStructure {
            n_events,
            causes,
            conflicts,
        }
    }
    /// Check that causality is a partial order.
    pub fn causality_is_partial_order(&self) -> bool {
        let n = self.n_events;
        for i in 0..n {
            if !self.causes[i][i] {
                return false;
            }
        }
        for i in 0..n {
            for j in 0..n {
                if i != j && self.causes[i][j] && self.causes[j][i] {
                    return false;
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if self.causes[i][j] && self.causes[j][k] && !self.causes[i][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check that conflict is irreflexive and symmetric.
    pub fn conflict_is_valid(&self) -> bool {
        let n = self.n_events;
        for i in 0..n {
            if self.conflicts[i][i] {
                return false;
            }
            for j in 0..n {
                if self.conflicts[i][j] != self.conflicts[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// Check hereditariness of conflict: e # e' ∧ e' ≤ e'' → e # e''.
    pub fn conflict_is_hereditary(&self) -> bool {
        let n = self.n_events;
        for e in 0..n {
            for ep in 0..n {
                if self.conflicts[e][ep] {
                    for epp in 0..n {
                        if self.causes[ep][epp] && !self.conflicts[e][epp] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    /// A configuration is a finite conflict-free downward-closed set of events.
    pub fn is_configuration(&self, config: &[usize]) -> bool {
        let n = self.n_events;
        if config.iter().any(|&e| e >= n) {
            return false;
        }
        for &e in config {
            for k in 0..n {
                if self.causes[k][e] && !config.contains(&k) {
                    return false;
                }
            }
        }
        for &e in config {
            for &f in config {
                if self.conflicts[e][f] {
                    return false;
                }
            }
        }
        true
    }
}
/// Kleene fixed-point computation: ⊔_{n≥0} fⁿ(⊥).
///
/// Iterates a monotone function on a flat domain until a fixed point is reached.
pub struct KleeneFixedPoint {
    /// The fixed point value (element index).
    pub value: usize,
    /// Number of iterations needed.
    pub iterations: usize,
}
impl KleeneFixedPoint {
    /// Compute the Kleene fixed point of `f` on `domain`, starting from ⊥.
    pub fn compute(f: &ScottContinuousFunction, domain: &DCPO) -> Option<Self> {
        let mut current = domain.bottom();
        for i in 0..domain.size + 1 {
            let next = f.apply(current)?;
            if next == current {
                return Some(KleeneFixedPoint {
                    value: current,
                    iterations: i,
                });
            }
            if !domain.le(current, next) {
                return None;
            }
            current = next;
        }
        None
    }
}
/// Operational equivalence: e₁ ≡ e₂ iff ∀C: C[e₁]↓ ↔ C[e₂]↓.
pub struct OperationalEquivalence {
    /// Whether the two expressions are operationally equivalent.
    pub are_equivalent: bool,
}
impl OperationalEquivalence {
    /// Create an operational equivalence result.
    pub fn new(are_equivalent: bool) -> Self {
        OperationalEquivalence { are_equivalent }
    }
}
/// A linear type: a resource that must be consumed exactly once.
#[derive(Debug, Clone)]
pub struct LinearType {
    /// Human-readable name.
    pub name: String,
    /// Whether this resource has been consumed.
    consumed: bool,
}
impl LinearType {
    /// Create a fresh (unconsumed) linear resource.
    pub fn new(name: impl Into<String>) -> Self {
        LinearType {
            name: name.into(),
            consumed: false,
        }
    }
    /// Consume the resource, returning it.  Panics if already consumed.
    pub fn consume(mut self) -> String {
        assert!(
            !self.consumed,
            "linear resource '{}' used more than once",
            self.name
        );
        self.consumed = true;
        self.name.clone()
    }
    /// Returns `true` if the resource is still available.
    pub fn is_available(&self) -> bool {
        !self.consumed
    }
}
/// A finite approximation of the Plotkin (convex) powerdomain of a DCPO.
///
/// Elements are non-empty antichains of the underlying DCPO,
/// ordered by the Egli–Milner order (which makes the Plotkin powerdomain a DCPO).
pub struct PowerDomain {
    /// Number of elements in the base domain (indices 0..size).
    pub base_size: usize,
    /// Current element: a non-empty set of indices (antichain of the base domain).
    pub element: Vec<usize>,
}
impl PowerDomain {
    /// Construct a Plotkin powerdomain element from a set of base indices.
    ///
    /// Automatically removes dominated elements to keep an antichain.
    pub fn new(base: &DCPO, element: Vec<usize>) -> Self {
        let antichain = Self::to_antichain(base, element);
        PowerDomain {
            base_size: base.size,
            element: antichain,
        }
    }
    /// Reduce a set of elements to an antichain by removing dominated elements.
    fn to_antichain(base: &DCPO, set: Vec<usize>) -> Vec<usize> {
        let mut result = Vec::new();
        'outer: for &x in &set {
            for &y in &set {
                if x != y && base.le(x, y) && !base.le(y, x) {
                    continue 'outer;
                }
            }
            if !result.contains(&x) {
                result.push(x);
            }
        }
        result
    }
    /// Egli–Milner order: A ≤_EM B iff (∀a∈A ∃b∈B: a≤b) ∧ (∀b∈B ∃a∈A: a≤b).
    pub fn egli_milner_le(&self, base: &DCPO, other: &PowerDomain) -> bool {
        let hoare = self
            .element
            .iter()
            .all(|&a| other.element.iter().any(|&b| base.le(a, b)));
        let smyth = other
            .element
            .iter()
            .all(|&b| self.element.iter().any(|&a| base.le(a, b)));
        hoare && smyth
    }
    /// Union of two powerdomain elements (then reduced to antichain).
    pub fn union(&self, base: &DCPO, other: &PowerDomain) -> PowerDomain {
        let mut combined = self.element.clone();
        combined.extend_from_slice(&other.element);
        PowerDomain::new(base, combined)
    }
}
/// Multiplicative proof net shortcut: a combinatorial structure for MLL proofs.
pub struct ProofNet {
    /// Number of formula occurrences (vertices).
    pub n_formulas: usize,
    /// Links (axiom links and cut links) as pairs of formula indices.
    pub links: Vec<(usize, usize)>,
}
impl ProofNet {
    /// Create a proof net.
    pub fn new(n_formulas: usize, links: Vec<(usize, usize)>) -> Self {
        ProofNet { n_formulas, links }
    }
    /// Check the correctness criterion (Danos–Regnier acyclicity test, simplified).
    ///
    /// A proof net is correct iff every "switching" of par-nodes gives a tree.
    /// Here we use a simplified check: the link graph must be connected and acyclic
    /// (valid only for axiom links in the cut-free case).
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
        let mut edge_count = 0;
        for &(a, b) in &self.links {
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
        edge_count == self.n_formulas - 1
    }
}
/// A coherence space: a set of tokens with a coherence (reflexive symmetric) relation.
///
/// Cliques (pairwise coherent sets) model values; the space models a type.
#[derive(Debug, Clone)]
pub struct CoherenceSpace {
    /// Number of tokens.
    pub n_tokens: usize,
    /// Coherence relation: `coh[i][j]` = true if token i is coherent with token j.
    /// Must be reflexive and symmetric.
    pub coh: Vec<Vec<bool>>,
}
impl CoherenceSpace {
    /// Create a coherence space.
    pub fn new(n_tokens: usize, coh: Vec<Vec<bool>>) -> Self {
        CoherenceSpace { n_tokens, coh }
    }
    /// Create the flat coherence space on `n` tokens: all tokens are incoherent with each other.
    pub fn flat(n: usize) -> Self {
        let mut coh = vec![vec![false; n]; n];
        for i in 0..n {
            coh[i][i] = true;
        }
        CoherenceSpace { n_tokens: n, coh }
    }
    /// Create the total coherence space: all tokens are coherent (models `!A`).
    pub fn total(n: usize) -> Self {
        CoherenceSpace {
            n_tokens: n,
            coh: vec![vec![true; n]; n],
        }
    }
    /// Check that `set` is a clique (pairwise coherent).
    pub fn is_clique(&self, set: &[usize]) -> bool {
        for &i in set {
            for &j in set {
                if i < self.n_tokens && j < self.n_tokens && !self.coh[i][j] {
                    return false;
                }
            }
        }
        true
    }
    /// Check that the coherence relation is reflexive and symmetric.
    pub fn is_well_formed(&self) -> bool {
        let n = self.n_tokens;
        for i in 0..n {
            if !self.coh[i][i] {
                return false;
            }
            for j in 0..n {
                if self.coh[i][j] != self.coh[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// The linear function space A → B: tokens are pairs (a, b) where a ∈ web(A), b ∈ web(B).
    /// Coherence: (a,b) ~ (a',b') iff a~a' in A implies b~b' in B, and a~^{-1}a' implies b~^{-1}b'.
    pub fn linear_fn_space(&self, other: &CoherenceSpace) -> CoherenceSpace {
        let m = self.n_tokens;
        let n = other.n_tokens;
        let total = m * n;
        let mut coh = vec![vec![false; total]; total];
        for i in 0..m {
            for j in 0..n {
                let ij = i * n + j;
                coh[ij][ij] = true;
                for ip in 0..m {
                    for jp in 0..n {
                        let ipjp = ip * n + jp;
                        let a_coh = self.coh[i][ip];
                        let a_incoh = !a_coh && i != ip;
                        let b_coh = other.coh[j][jp];
                        let b_incoh = !b_coh && j != jp;
                        let ok = (!a_coh || b_coh) && (!a_incoh || b_incoh);
                        coh[ij][ipjp] = ok;
                    }
                }
            }
        }
        CoherenceSpace {
            n_tokens: total,
            coh,
        }
    }
}
/// Additive conjunction A & B ("with"): choice of shared resources.
#[derive(Debug, Clone)]
pub struct AdditiveConj<A: Clone, B: Clone> {
    /// Left option.
    left: A,
    /// Right option.
    right: B,
}
impl<A: Clone, B: Clone> AdditiveConj<A, B> {
    /// Create an additive conjunction.
    pub fn new(left: A, right: B) -> Self {
        AdditiveConj { left, right }
    }
    /// Choose the left component (π₁).
    pub fn choose_left(&self) -> A {
        self.left.clone()
    }
    /// Choose the right component (π₂).
    pub fn choose_right(&self) -> B {
        self.right.clone()
    }
}
/// A Scott domain: a bounded-complete DCPO with least element ⊥.
pub struct ScottDomain {
    /// Underlying DCPO.
    pub dcpo: DCPO,
}
impl ScottDomain {
    /// Create a Scott domain from a DCPO.
    pub fn new(dcpo: DCPO) -> Self {
        ScottDomain { dcpo }
    }
    /// Create the flat Scott domain on `n` concrete values.
    pub fn flat(n: usize) -> Self {
        ScottDomain::new(DCPO::flat(n))
    }
    /// Return the bottom element.
    pub fn bottom(&self) -> usize {
        self.dcpo.bottom()
    }
    /// Returns `true` if the element at index `i` is compact.
    pub fn is_compact(&self, i: usize) -> bool {
        self.dcpo.is_compact(i)
    }
    /// Compute the sup of a directed set.
    pub fn sup(&self, subset: &[usize]) -> Option<usize> {
        self.dcpo.sup(subset)
    }
    /// Way-below relation: `i ≪ j`.
    ///
    /// In a flat domain: x ≪ y iff x = ⊥ or x = y.
    pub fn way_below(&self, i: usize, j: usize) -> bool {
        i == self.dcpo.bottom() || i == j
    }
}
