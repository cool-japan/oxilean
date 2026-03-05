//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap};

use super::functions::*;

/// A move in an arena.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArenaMove {
    /// Unique identifier.
    pub id: usize,
    /// Polarity.
    pub polarity: Polarity,
    /// Initial move flag (no enabler).
    pub initial: bool,
}
/// A partial order on a finite set represented as an adjacency relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinitePartialOrder {
    /// Number of elements (indexed 0..n).
    pub n: usize,
    /// Relation: `leq[i][j]` is true iff i ≤ j.
    pub leq: Vec<Vec<bool>>,
}
impl FinitePartialOrder {
    /// Create a discrete partial order (only reflexive pairs).
    pub fn discrete(n: usize) -> Self {
        let leq = (0..n).map(|i| (0..n).map(|j| i == j).collect()).collect();
        FinitePartialOrder { n, leq }
    }
    /// Create the flat order: ⊥ ≤ everything.
    pub fn flat(n: usize) -> Self {
        let k = n + 1;
        let leq = (0..k)
            .map(|i| (0..k).map(|j| i == j || i == 0).collect())
            .collect();
        FinitePartialOrder { n: k, leq }
    }
    /// Check whether the order satisfies reflexivity.
    pub fn is_reflexive(&self) -> bool {
        (0..self.n).all(|i| self.leq[i][i])
    }
    /// Check whether the order satisfies transitivity.
    pub fn is_transitive(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    if self.leq[i][j] && self.leq[j][k] && !self.leq[i][k] {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check whether the order satisfies antisymmetry.
    pub fn is_antisymmetric(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                if i != j && self.leq[i][j] && self.leq[j][i] {
                    return false;
                }
            }
        }
        true
    }
    /// Check that this is a valid partial order.
    pub fn is_valid(&self) -> bool {
        self.is_reflexive() && self.is_transitive() && self.is_antisymmetric()
    }
    /// Compute the least upper bound of two elements, if it exists.
    pub fn lub(&self, a: usize, b: usize) -> Option<usize> {
        let ubs: Vec<usize> = (0..self.n)
            .filter(|&x| self.leq[a][x] && self.leq[b][x])
            .collect();
        ubs.iter()
            .find(|&&x| ubs.iter().all(|&y| self.leq[x][y]))
            .copied()
    }
    /// Compute the greatest lower bound of two elements, if it exists.
    pub fn glb(&self, a: usize, b: usize) -> Option<usize> {
        let lbs: Vec<usize> = (0..self.n)
            .filter(|&x| self.leq[x][a] && self.leq[x][b])
            .collect();
        lbs.iter()
            .find(|&&x| lbs.iter().all(|&y| self.leq[y][x]))
            .copied()
    }
}
/// A concrete trace over an action type `A`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace<A: Clone + Eq> {
    /// The sequence of actions.
    pub actions: Vec<A>,
}
impl<A: Clone + Eq> Trace<A> {
    /// Create an empty trace.
    pub fn empty() -> Self {
        Trace { actions: vec![] }
    }
    /// Create a trace from a vector of actions.
    pub fn new(actions: Vec<A>) -> Self {
        Trace { actions }
    }
    /// Append one action at the end.
    pub fn extend(&self, a: A) -> Self {
        let mut acts = self.actions.clone();
        acts.push(a);
        Trace { actions: acts }
    }
    /// Concatenate two traces.
    pub fn concat(&self, other: &Self) -> Self {
        let mut acts = self.actions.clone();
        acts.extend_from_slice(&other.actions);
        Trace { actions: acts }
    }
    /// Check whether `self` is a prefix of `other`.
    pub fn is_prefix_of(&self, other: &Self) -> bool {
        other.actions.starts_with(&self.actions)
    }
    /// Return the length of the trace.
    pub fn len(&self) -> usize {
        self.actions.len()
    }
    /// Check whether the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
/// A finite model of a partial combinatory algebra pre-populated with K, S, I.
#[derive(Debug, Clone)]
pub struct KleenePCA {
    /// Registered elements by name -> index.
    pub combinators: HashMap<String, usize>,
    /// Application table: (f, x) -> result.
    pub apply_table: HashMap<(usize, usize), usize>,
    next_idx: usize,
}
impl KleenePCA {
    /// Create with K(0), S(1), I(2) and K-partial applications K*a (indices 3-5).
    pub fn with_ks() -> Self {
        let mut pca = KleenePCA {
            combinators: HashMap::new(),
            apply_table: HashMap::new(),
            next_idx: 6,
        };
        pca.combinators.insert("K".to_string(), 0);
        pca.combinators.insert("S".to_string(), 1);
        pca.combinators.insert("I".to_string(), 2);
        for a in 0usize..3 {
            let ka = 3 + a;
            pca.apply_table.insert((0, a), ka);
            for b in 0usize..3 {
                pca.apply_table.insert((ka, b), a);
            }
        }
        for a in 0usize..3 {
            pca.apply_table.insert((2, a), a);
        }
        pca
    }
    /// Look up a named element's index.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.combinators.get(name).copied()
    }
    /// Apply element `f` to element `x`.
    pub fn apply(&self, f: usize, x: usize) -> Option<usize> {
        self.apply_table.get(&(f, x)).copied()
    }
    /// Add a fresh named element, returning its index.
    pub fn add_element(&mut self, name: impl Into<String>) -> usize {
        let idx = self.next_idx;
        self.combinators.insert(name.into(), idx);
        self.next_idx += 1;
        idx
    }
    /// Define an application rule: f * x = result.
    pub fn define_app(&mut self, f: usize, x: usize, result: usize) {
        self.apply_table.insert((f, x), result);
    }
    /// Verify K * a * b = a for a, b in 0..3.
    pub fn check_k_law(&self) -> bool {
        let k = match self.lookup("K") {
            Some(v) => v,
            None => return false,
        };
        for a in 0usize..3 {
            let ka = match self.apply(k, a) {
                Some(v) => v,
                None => return false,
            };
            for b in 0usize..3 {
                match self.apply(ka, b) {
                    Some(r) if r == a => {}
                    _ => return false,
                }
            }
        }
        true
    }
    /// Verify I * a = a for a in 0..3.
    pub fn check_i_law(&self) -> bool {
        let i = match self.lookup("I") {
            Some(v) => v,
            None => return false,
        };
        (0..3).all(|a| self.apply(i, a) == Some(a))
    }
}
/// Polarity of a move: Opponent (O) or Proponent (P).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Opponent move.
    O,
    /// Proponent move.
    P,
}
/// A finite approximation to a bilimit: a chain of finite CPOs.
#[derive(Debug, Clone)]
pub struct BilimitApprox {
    /// Levels in the chain.
    pub levels: Vec<BilimitStep>,
}
impl BilimitApprox {
    /// Create a bilimit approximation from a sequence of steps.
    pub fn new(levels: Vec<BilimitStep>) -> Self {
        BilimitApprox { levels }
    }
    /// The depth of the approximation.
    pub fn depth(&self) -> usize {
        self.levels.len()
    }
    /// Project an element from level `l` all the way down to level 0.
    pub fn project_to_base(&self, mut x: usize, l: usize) -> usize {
        for step in self.levels.iter().take(l).rev() {
            x = step.project(x);
        }
        x
    }
}
/// A binary logical relation over a finite domain, indexed by type codes.
#[derive(Debug, Clone)]
pub struct LogicalRelation {
    /// For each type code, the set of related pairs (i, j).
    pub pairs: HashMap<String, Vec<(usize, usize)>>,
}
impl LogicalRelation {
    /// Create an empty logical relation.
    pub fn new() -> Self {
        LogicalRelation {
            pairs: HashMap::new(),
        }
    }
    /// Add a related pair (a, b) for type `ty_code`.
    pub fn add(&mut self, ty_code: impl Into<String>, a: usize, b: usize) {
        self.pairs.entry(ty_code.into()).or_default().push((a, b));
    }
    /// Check whether (a, b) is related at type `ty_code`.
    pub fn relates(&self, ty_code: &str, a: usize, b: usize) -> bool {
        self.pairs
            .get(ty_code)
            .map_or(false, |v| v.contains(&(a, b)))
    }
    /// Check reflexivity for all elements 0..domain_size.
    pub fn is_reflexive_on(&self, ty_code: &str, domain_size: usize) -> bool {
        (0..domain_size).all(|a| self.relates(ty_code, a, a))
    }
    /// Check symmetry for a given type code.
    pub fn is_symmetric_on(&self, ty_code: &str) -> bool {
        if let Some(pairs) = self.pairs.get(ty_code) {
            pairs.iter().all(|&(a, b)| pairs.contains(&(b, a)))
        } else {
            true
        }
    }
    /// Number of related pairs for a type code.
    pub fn size(&self, ty_code: &str) -> usize {
        self.pairs.get(ty_code).map_or(0, |v| v.len())
    }
}
/// A simple lambda-calculus term (de Bruijn) for CPS transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaTerm {
    /// Variable (de Bruijn index).
    Var(usize),
    /// Lambda abstraction.
    Lam(Box<LambdaTerm>),
    /// Application.
    App(Box<LambdaTerm>, Box<LambdaTerm>),
    /// Named constant.
    Const(String),
}
impl LambdaTerm {
    /// Create a variable.
    pub fn var(n: usize) -> Self {
        LambdaTerm::Var(n)
    }
    /// Create a lambda abstraction.
    pub fn lam(body: LambdaTerm) -> Self {
        LambdaTerm::Lam(Box::new(body))
    }
    /// Create an application.
    pub fn app(f: LambdaTerm, x: LambdaTerm) -> Self {
        LambdaTerm::App(Box::new(f), Box::new(x))
    }
    /// Create a named constant.
    pub fn cst(s: impl Into<String>) -> Self {
        LambdaTerm::Const(s.into())
    }
    /// Term size (number of constructors).
    pub fn size(&self) -> usize {
        match self {
            LambdaTerm::Var(_) | LambdaTerm::Const(_) => 1,
            LambdaTerm::Lam(b) => 1 + b.size(),
            LambdaTerm::App(f, x) => 1 + f.size() + x.size(),
        }
    }
    /// Check whether de Bruijn index `target` is free at binding depth `depth`.
    pub fn has_free_var(&self, depth: usize, target: usize) -> bool {
        match self {
            LambdaTerm::Var(n) => *n == target + depth,
            LambdaTerm::Const(_) => false,
            LambdaTerm::Lam(b) => b.has_free_var(depth + 1, target),
            LambdaTerm::App(f, x) => f.has_free_var(depth, target) || x.has_free_var(depth, target),
        }
    }
}
/// A monadic interpreter using Option to model partial PCF evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaybeInterp {
    /// Maximum evaluation fuel.
    pub fuel: u64,
}
impl MaybeInterp {
    /// Create a new interpreter.
    pub fn new(fuel: u64) -> Self {
        MaybeInterp { fuel }
    }
    /// Monadic unit: wrap a value.
    pub fn ret(v: PCFValue) -> Option<PCFValue> {
        Some(v)
    }
    /// Monadic bind.
    pub fn bind<F>(m: Option<PCFValue>, f: F) -> Option<PCFValue>
    where
        F: FnOnce(PCFValue) -> Option<PCFValue>,
    {
        m.and_then(f)
    }
    /// Evaluate a PCF term; return None if it diverges.
    pub fn eval(&self, term: &PCFTerm) -> Option<PCFValue> {
        let v = pcf_eval(term, self.fuel);
        if v.is_bottom() {
            None
        } else {
            Some(v)
        }
    }
    /// Guard: None if condition is false.
    pub fn guard(cond: bool) -> Option<()> {
        if cond {
            Some(())
        } else {
            None
        }
    }
    /// Sequence two evaluations, discarding the first result.
    pub fn seq(&self, t1: &PCFTerm, t2: &PCFTerm) -> Option<PCFValue> {
        Self::bind(self.eval(t1), |_| self.eval(t2))
    }
    /// Map over a monadic value.
    pub fn map<F>(m: Option<PCFValue>, f: F) -> Option<PCFValue>
    where
        F: FnOnce(PCFValue) -> PCFValue,
    {
        m.map(f)
    }
}
/// A Scott-open set on a finite flat domain {bottom=0, 1, ..., domain_size-1}.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScottOpen {
    /// Total domain size (bottom at index 0).
    pub domain_size: usize,
    /// Elements in the open set (sorted, all > 0).
    pub elements: Vec<usize>,
}
impl ScottOpen {
    /// Create a Scott-open set (bottom is automatically excluded).
    pub fn new(domain_size: usize, elems: impl IntoIterator<Item = usize>) -> Self {
        let mut v: Vec<usize> = elems
            .into_iter()
            .filter(|&x| x > 0 && x < domain_size)
            .collect();
        v.sort_unstable();
        v.dedup();
        ScottOpen {
            domain_size,
            elements: v,
        }
    }
    /// The whole domain minus bottom is Scott-open.
    pub fn top(domain_size: usize) -> Self {
        ScottOpen {
            domain_size,
            elements: (1..domain_size).collect(),
        }
    }
    /// The empty set is Scott-open.
    pub fn empty(domain_size: usize) -> Self {
        ScottOpen {
            domain_size,
            elements: vec![],
        }
    }
    /// Test membership.
    pub fn contains(&self, x: usize) -> bool {
        self.elements.binary_search(&x).is_ok()
    }
    /// Union (stays Scott-open).
    pub fn union(&self, other: &Self) -> Self {
        let mut v = self.elements.clone();
        for &x in &other.elements {
            if !self.contains(x) {
                v.push(x);
            }
        }
        v.sort_unstable();
        ScottOpen {
            domain_size: self.domain_size,
            elements: v,
        }
    }
    /// Intersection (stays Scott-open).
    pub fn intersection(&self, other: &Self) -> Self {
        let v: Vec<usize> = self
            .elements
            .iter()
            .filter(|&&x| other.contains(x))
            .copied()
            .collect();
        ScottOpen {
            domain_size: self.domain_size,
            elements: v,
        }
    }
    /// Verify Scott-openness: bottom must not be in the set.
    pub fn is_scott_open(&self) -> bool {
        !self.contains(0)
    }
    /// Characteristic membership function.
    pub fn characteristic(&self, x: usize) -> bool {
        self.contains(x)
    }
}
/// A game arena: a set of moves with a polarity function and enabling relation.
#[derive(Debug, Clone)]
pub struct GameArena {
    /// The moves in this arena.
    pub moves: Vec<ArenaMove>,
    /// Enabling relation: `enables[i]` = set of moves that move `i` enables.
    pub enables: HashMap<usize, Vec<usize>>,
}
impl GameArena {
    /// Create an arena with given moves.
    pub fn new(moves: Vec<ArenaMove>) -> Self {
        GameArena {
            moves,
            enables: HashMap::new(),
        }
    }
    /// Add an enabling: move with id `from` enables move with id `to`.
    pub fn add_enables(&mut self, from: usize, to: usize) {
        self.enables.entry(from).or_default().push(to);
    }
    /// Return all initial (O) moves.
    pub fn initial_moves(&self) -> Vec<&ArenaMove> {
        self.moves.iter().filter(|m| m.initial).collect()
    }
}
/// The result of evaluating a PCF term: either a value or divergence (⊥).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PCFValue {
    /// A natural number value.
    Num(u64),
    /// A Boolean value.
    Bool(bool),
    /// Divergence (bottom element ⊥).
    Bottom,
    /// A closure (lambda body + captured environment depth).
    Closure(Box<PCFTerm>, usize),
}
impl PCFValue {
    /// Check whether this value is bottom.
    pub fn is_bottom(&self) -> bool {
        matches!(self, PCFValue::Bottom)
    }
    /// Check whether this value is a natural number.
    pub fn is_nat(&self) -> bool {
        matches!(self, PCFValue::Num(_))
    }
}
/// A monotone endofunction on `usize` values (simulates a Scott-continuous map
/// on a finite CPO where elements are represented as `usize` indices).
#[derive(Clone)]
pub struct MonotoneMap {
    /// The underlying function table: `table[i] = f(i)`.
    pub table: Vec<usize>,
}
impl MonotoneMap {
    /// Create from an explicit table.
    pub fn new(table: Vec<usize>) -> Self {
        MonotoneMap { table }
    }
    /// Apply the function.
    pub fn apply(&self, x: usize) -> usize {
        self.table[x]
    }
    /// Check monotonicity with respect to a given partial order.
    pub fn is_monotone(&self, po: &FinitePartialOrder) -> bool {
        for i in 0..po.n.min(self.table.len()) {
            for j in 0..po.n.min(self.table.len()) {
                if po.leq[i][j] {
                    let fi = self.table[i];
                    let fj = self.table[j];
                    if fi < po.n && fj < po.n && !po.leq[fi][fj] {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A PCF term (surface AST).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PCFTerm {
    /// Variable (de Bruijn index).
    Var(usize),
    /// Zero : Nat.
    Zero,
    /// True : Bool.
    True,
    /// False : Bool.
    False,
    /// Successor.
    Succ(Box<PCFTerm>),
    /// Predecessor (Pred(Zero) = Zero by convention).
    Pred(Box<PCFTerm>),
    /// IsZero test.
    IsZero(Box<PCFTerm>),
    /// λ-abstraction.
    Lam(Box<PCFTerm>),
    /// Application.
    App(Box<PCFTerm>, Box<PCFTerm>),
    /// Fixed-point combinator Y.
    Fix(Box<PCFTerm>),
    /// Conditional.
    If(Box<PCFTerm>, Box<PCFTerm>, Box<PCFTerm>),
}
impl PCFTerm {
    /// Return a human-readable label for the outermost constructor.
    pub fn label(&self) -> &'static str {
        match self {
            PCFTerm::Var(_) => "Var",
            PCFTerm::Zero => "Zero",
            PCFTerm::True => "True",
            PCFTerm::False => "False",
            PCFTerm::Succ(_) => "Succ",
            PCFTerm::Pred(_) => "Pred",
            PCFTerm::IsZero(_) => "IsZero",
            PCFTerm::Lam(_) => "Lam",
            PCFTerm::App(_, _) => "App",
            PCFTerm::Fix(_) => "Fix",
            PCFTerm::If(_, _, _) => "If",
        }
    }
}
/// A step in building a bilimit: a finite CPO together with the sequence of
/// projection maps from each level to the previous.
#[derive(Debug, Clone)]
pub struct BilimitStep {
    /// The size of the CPO at this step.
    pub size: usize,
    /// The projection table `proj[x] = p(x)` mapping this level to the previous.
    pub proj: Vec<usize>,
}
impl BilimitStep {
    /// Create a bilimit step.
    pub fn new(size: usize, proj: Vec<usize>) -> Self {
        BilimitStep { size, proj }
    }
    /// Apply the projection.
    pub fn project(&self, x: usize) -> usize {
        self.proj.get(x).copied().unwrap_or(0)
    }
}
/// A PCF type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PCFType {
    /// The type of natural numbers.
    Nat,
    /// The type of Booleans.
    Bool,
    /// Function type.
    Arrow(Box<PCFType>, Box<PCFType>),
}
impl PCFType {
    /// Construct a function type τ₁ → τ₂.
    pub fn arrow(t1: PCFType, t2: PCFType) -> Self {
        PCFType::Arrow(Box::new(t1), Box::new(t2))
    }
    /// Return the string name of the type.
    pub fn name(&self) -> String {
        match self {
            PCFType::Nat => "Nat".to_string(),
            PCFType::Bool => "Bool".to_string(),
            PCFType::Arrow(a, b) => format!("({} → {})", a.name(), b.name()),
        }
    }
}
/// A finite sub-probability distribution over a set of elements.
#[derive(Debug, Clone)]
pub struct FiniteValuation {
    /// Probability mass for each element (indexed by element key).
    pub masses: BTreeMap<String, f64>,
}
impl FiniteValuation {
    /// Create an empty valuation (mass 0 everywhere).
    pub fn empty() -> Self {
        FiniteValuation {
            masses: BTreeMap::new(),
        }
    }
    /// Create a Dirac (point-mass) valuation at `x`.
    pub fn dirac(x: impl Into<String>) -> Self {
        let mut m = BTreeMap::new();
        m.insert(x.into(), 1.0);
        FiniteValuation { masses: m }
    }
    /// Total mass.
    pub fn total_mass(&self) -> f64 {
        self.masses.values().sum()
    }
    /// Check whether this is a sub-probability distribution (total mass ≤ 1).
    pub fn is_sub_probability(&self) -> bool {
        self.total_mass() <= 1.0 + 1e-9
    }
    /// Convex combination: (1 − p) * self + p * other.
    pub fn mix(&self, other: &Self, p: f64) -> Self {
        let mut m = BTreeMap::new();
        for (k, v) in &self.masses {
            *m.entry(k.clone()).or_insert(0.0) += (1.0 - p) * v;
        }
        for (k, v) in &other.masses {
            *m.entry(k.clone()).or_insert(0.0) += p * v;
        }
        FiniteValuation { masses: m }
    }
    /// Stochastic order: self ≤ other iff for every "up-set" U,
    /// self(U) ≤ other(U).  For a flat order this means self(x) ≤ other(x)
    /// for every x (since singletons are up-sets).
    pub fn stochastic_leq(&self, other: &Self) -> bool {
        for (k, v) in &self.masses {
            let w = other.masses.get(k).copied().unwrap_or(0.0);
            if *v > w + 1e-9 {
                return false;
            }
        }
        true
    }
}
/// A lower (Hoare) power domain element: a nonempty downward-closed set of
/// `usize` values with a Hoare ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoareElement {
    /// The underlying finite set (kept sorted for canonical representation).
    pub elems: Vec<usize>,
}
impl HoareElement {
    /// Create from an iterator, removing duplicates and sorting.
    pub fn new(iter: impl IntoIterator<Item = usize>) -> Self {
        let mut v: Vec<usize> = iter.into_iter().collect();
        v.sort_unstable();
        v.dedup();
        HoareElement { elems: v }
    }
    /// The Hoare (subset) ordering: `self ≤ other` iff `self ⊆ other`.
    pub fn hoare_leq(&self, other: &Self) -> bool {
        self.elems.iter().all(|x| other.elems.contains(x))
    }
    /// Union (join in Hoare order).
    pub fn union(&self, other: &Self) -> Self {
        let mut v = self.elems.clone();
        for x in &other.elems {
            if !v.contains(x) {
                v.push(*x);
            }
        }
        v.sort_unstable();
        HoareElement { elems: v }
    }
}
/// An innocent strategy: a prefix-closed set of P-views (encoded as sequences of
/// move ids) along with the P-response for each view.
#[derive(Debug, Clone)]
pub struct InnocentStrategy {
    /// Map from P-view (sequence of move ids) → P-move id chosen in response.
    pub responses: HashMap<Vec<usize>, usize>,
}
impl InnocentStrategy {
    /// Create an empty strategy.
    pub fn new() -> Self {
        InnocentStrategy {
            responses: HashMap::new(),
        }
    }
    /// Add a response: given the current P-view `view`, respond with `p_move`.
    pub fn add_response(&mut self, view: Vec<usize>, p_move: usize) {
        self.responses.insert(view, p_move);
    }
    /// Look up the response for a given P-view.
    pub fn respond(&self, view: &[usize]) -> Option<usize> {
        self.responses.get(view).copied()
    }
    /// Number of defined responses.
    pub fn size(&self) -> usize {
        self.responses.len()
    }
}
