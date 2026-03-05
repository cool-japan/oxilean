//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// A rewriting logic theory (equational + rewrite rules).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RewritingLogicTheory {
    /// Equational axioms (identity up to).
    pub equations: Vec<(String, String)>,
    /// Rewrite rules (labeled, one-directional).
    pub rw_rules: Vec<(String, String, String)>,
    /// Sort hierarchy.
    pub sorts: Vec<String>,
}
#[allow(dead_code)]
impl RewritingLogicTheory {
    /// Creates an empty theory.
    pub fn new() -> Self {
        RewritingLogicTheory {
            equations: Vec::new(),
            rw_rules: Vec::new(),
            sorts: Vec::new(),
        }
    }
    /// Adds a sort.
    pub fn add_sort(&mut self, sort: &str) {
        if !self.sorts.contains(&sort.to_string()) {
            self.sorts.push(sort.to_string());
        }
    }
    /// Adds an equation.
    pub fn add_equation(&mut self, lhs: &str, rhs: &str) {
        self.equations.push((lhs.to_string(), rhs.to_string()));
    }
    /// Adds a rewrite rule.
    pub fn add_rw_rule(&mut self, label: &str, lhs: &str, rhs: &str) {
        self.rw_rules
            .push((label.to_string(), lhs.to_string(), rhs.to_string()));
    }
    /// Returns the signature size.
    pub fn signature_size(&self) -> usize {
        self.equations.len() + self.rw_rules.len()
    }
    /// Checks if the theory is a pure equational theory (no rw rules).
    pub fn is_equational(&self) -> bool {
        self.rw_rules.is_empty()
    }
    /// Generates the entailment description.
    pub fn entailment_description(&self) -> String {
        format!(
            "Rewriting logic theory with {} sorts, {} equations, {} rules",
            self.sorts.len(),
            self.equations.len(),
            self.rw_rules.len()
        )
    }
}
/// A rewrite rule `lhs → rhs`.
///
/// Variables in `lhs` range over `Term::Var(i)`.  The rule is valid when
/// every variable in `rhs` also occurs in `lhs`.
#[derive(Debug, Clone)]
pub struct Rule {
    pub lhs: Term,
    pub rhs: Term,
}
impl Rule {
    /// Creates a new rewrite rule.
    pub fn new(lhs: Term, rhs: Term) -> Self {
        Rule { lhs, rhs }
    }
    /// Returns `true` if every variable in `rhs` occurs in `lhs`.
    pub fn is_valid(&self) -> bool {
        self.rhs.vars().is_subset(&self.lhs.vars())
    }
    /// Returns `true` if `lhs` is linear (each variable occurs at most once).
    pub fn is_left_linear(&self) -> bool {
        let mut seen = HashSet::new();
        fn check(t: &Term, seen: &mut HashSet<u32>) -> bool {
            match t {
                Term::Var(i) => seen.insert(*i),
                Term::Fun(_, args) => args.iter().all(|a| check(a, seen)),
            }
        }
        check(&self.lhs, &mut seen)
    }
    /// Returns a renamed copy of this rule with variables shifted by `offset`.
    pub fn rename(&self, offset: u32) -> Rule {
        fn shift(t: &Term, off: u32) -> Term {
            match t {
                Term::Var(i) => Term::Var(i + off),
                Term::Fun(f, args) => {
                    Term::Fun(f.clone(), args.iter().map(|a| shift(a, off)).collect())
                }
            }
        }
        Rule {
            lhs: shift(&self.lhs, offset),
            rhs: shift(&self.rhs, offset),
        }
    }
}
/// An E-graph for equality saturation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EGraph {
    /// E-classes by ID.
    pub classes: Vec<EClass>,
    /// Union-find parent array.
    pub parent: Vec<usize>,
    /// Number of e-nodes total.
    pub total_nodes: usize,
}
#[allow(dead_code)]
impl EGraph {
    /// Creates an empty E-graph.
    pub fn new() -> Self {
        EGraph {
            classes: Vec::new(),
            parent: Vec::new(),
            total_nodes: 0,
        }
    }
    /// Adds a new e-class with a single node.
    pub fn add_node(&mut self, node: &str) -> usize {
        let id = self.classes.len();
        self.classes.push(EClass {
            id,
            nodes: vec![node.to_string()],
            size: 1,
        });
        self.parent.push(id);
        self.total_nodes += 1;
        id
    }
    /// Finds the canonical class ID (path-compressed union-find).
    pub fn find(&mut self, id: usize) -> usize {
        if self.parent[id] == id {
            return id;
        }
        let root = self.find(self.parent[id]);
        self.parent[id] = root;
        root
    }
    /// Merges two e-classes.
    pub fn union(&mut self, id1: usize, id2: usize) {
        let r1 = self.find(id1);
        let r2 = self.find(id2);
        if r1 == r2 {
            return;
        }
        self.parent[r2] = r1;
        let nodes2 = self.classes[r2].nodes.clone();
        let size2 = self.classes[r2].size;
        self.classes[r1].nodes.extend(nodes2);
        self.classes[r1].size += size2;
    }
    /// Checks if two nodes are in the same e-class.
    pub fn are_equal(&mut self, id1: usize, id2: usize) -> bool {
        self.find(id1) == self.find(id2)
    }
    /// Returns the number of e-classes.
    pub fn num_classes(&self) -> usize {
        self.classes.len()
    }
}
/// A first-order term over a signature.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    /// A variable `x_i`.
    Var(u32),
    /// A function application `f(t1, ..., tn)`.
    Fun(String, Vec<Term>),
}
impl Term {
    /// Returns the set of variable indices occurring in this term.
    pub fn vars(&self) -> HashSet<u32> {
        match self {
            Term::Var(i) => {
                let mut s = HashSet::new();
                s.insert(*i);
                s
            }
            Term::Fun(_, args) => args.iter().flat_map(|a| a.vars()).collect(),
        }
    }
    /// Returns `true` if this term is ground (contains no variables).
    pub fn is_ground(&self) -> bool {
        self.vars().is_empty()
    }
    /// Returns the depth of this term.
    pub fn depth(&self) -> usize {
        match self {
            Term::Var(_) => 0,
            Term::Fun(_, args) => 1 + args.iter().map(|a| a.depth()).max().unwrap_or(0),
        }
    }
    /// Apply a substitution to this term.
    pub fn apply(&self, subst: &Substitution) -> Term {
        match self {
            Term::Var(i) => subst.map.get(i).cloned().unwrap_or(Term::Var(*i)),
            Term::Fun(f, args) => {
                Term::Fun(f.clone(), args.iter().map(|a| a.apply(subst)).collect())
            }
        }
    }
    /// Returns `true` if the term `other` occurs in this term.
    pub fn contains(&self, other: &Term) -> bool {
        if self == other {
            return true;
        }
        match self {
            Term::Var(_) => false,
            Term::Fun(_, args) => args.iter().any(|a| a.contains(other)),
        }
    }
    /// Returns the subterm at the given position (empty = root).
    pub fn subterm_at(&self, pos: &[usize]) -> Option<&Term> {
        if pos.is_empty() {
            return Some(self);
        }
        match self {
            Term::Fun(_, args) => {
                let idx = pos[0];
                args.get(idx)?.subterm_at(&pos[1..])
            }
            Term::Var(_) => None,
        }
    }
    /// Replace the subterm at `pos` with `replacement`, returning the new term.
    pub fn replace_at(&self, pos: &[usize], replacement: Term) -> Term {
        if pos.is_empty() {
            return replacement;
        }
        match self {
            Term::Fun(f, args) => {
                let idx = pos[0];
                let mut new_args = args.clone();
                if idx < new_args.len() {
                    new_args[idx] = new_args[idx].replace_at(&pos[1..], replacement);
                }
                Term::Fun(f.clone(), new_args)
            }
            Term::Var(_) => self.clone(),
        }
    }
}
/// A narrowing system: computes all possible narrowing steps of a term w.r.t. a TRS.
///
/// Narrowing generalises rewriting to terms with variables: a term `t` narrows
/// to `t'` if there is a substitution σ and a rule `l → r` such that
/// `t[σ]_p = l[σ]` for some non-variable position `p`, and `t' = t[r]_p[σ]`.
#[derive(Debug, Clone)]
pub struct NarrowingSystem {
    /// The underlying TRS used for narrowing steps.
    pub trs: Trs,
    /// Variable counter for generating fresh variable names.
    pub var_counter: u32,
}
impl NarrowingSystem {
    /// Create a new narrowing system wrapping the given TRS.
    pub fn new(trs: Trs) -> Self {
        NarrowingSystem {
            trs,
            var_counter: 10000,
        }
    }
    /// Fresh variable index (offset to avoid clashing with term variables).
    fn fresh_var(&mut self) -> u32 {
        let v = self.var_counter;
        self.var_counter += 1;
        v
    }
    /// Collect all non-variable positions in term `t`.
    fn non_var_positions(t: &Term) -> Vec<Vec<usize>> {
        match t {
            Term::Var(_) => vec![],
            Term::Fun(_, args) => {
                let mut out = vec![vec![]];
                for (i, a) in args.iter().enumerate() {
                    for mut p in Self::non_var_positions(a) {
                        let mut full = vec![i];
                        full.append(&mut p);
                        out.push(full);
                    }
                }
                out
            }
        }
    }
    /// Compute one level of narrowing steps from term `t`.
    ///
    /// Returns a list of `(substitution, narrowed_term)` pairs.
    pub fn narrow_step(&mut self, t: &Term) -> Vec<(Substitution, Term)> {
        let mut results = Vec::new();
        let positions = Self::non_var_positions(t);
        for pos in &positions {
            if let Some(sub) = t.subterm_at(pos) {
                for rule in self.trs.rules.clone() {
                    let offset = self.fresh_var();
                    let renamed = rule.rename(offset);
                    if let Some(sigma) = unify(sub, &renamed.lhs) {
                        let new_term = t.replace_at(pos, renamed.rhs.apply(&sigma)).apply(&sigma);
                        results.push((sigma, new_term));
                    }
                }
            }
        }
        results
    }
    /// Basic narrowing: perform up to `depth` levels of narrowing from `t`.
    ///
    /// Returns all reachable (substitution, term) pairs.
    pub fn basic_narrow(&mut self, t: &Term, depth: usize) -> Vec<(Substitution, Term)> {
        if depth == 0 {
            return vec![(Substitution::new(), t.clone())];
        }
        let steps = self.narrow_step(t);
        if steps.is_empty() {
            return vec![(Substitution::new(), t.clone())];
        }
        let mut all = Vec::new();
        for (sigma, t2) in steps {
            let deeper = self.basic_narrow(&t2, depth - 1);
            for (sigma2, t3) in deeper {
                let combined = sigma2.compose(&sigma);
                all.push((combined, t3));
            }
        }
        all
    }
    /// Unification via narrowing: tries to unify `s` and `t` by narrowing `s` toward `t`.
    pub fn narrowing_unify(&mut self, s: &Term, t: &Term, depth: usize) -> Option<Substitution> {
        let narrowings = self.basic_narrow(s, depth);
        for (sigma, s_narrowed) in narrowings {
            if let Some(sigma2) = unify(&s_narrowed, t) {
                return Some(sigma2.compose(&sigma));
            }
        }
        None
    }
}
/// Knuth-Bendix completion state.
pub struct KBState {
    /// Current set of rules.
    pub rules: Vec<Rule>,
    /// Pending equations to orient/simplify.
    pub equations: VecDeque<(Term, Term)>,
    /// Maximum completion steps.
    pub max_steps: usize,
}
impl KBState {
    /// Creates a new KB state from an initial set of equations.
    pub fn new(equations: Vec<(Term, Term)>, max_steps: usize) -> Self {
        KBState {
            rules: Vec::new(),
            equations: VecDeque::from(equations),
            max_steps,
        }
    }
    /// Runs the Knuth-Bendix completion algorithm.
    ///
    /// Returns `Ok(trs)` if completion succeeds, `Err(msg)` otherwise.
    pub fn complete(&mut self, order: TermOrdering) -> Result<Trs, String> {
        let mut steps = 0;
        while let Some((s, t)) = self.equations.pop_front() {
            if steps >= self.max_steps {
                return Err("KB completion: exceeded max steps".into());
            }
            steps += 1;
            let trs = Trs {
                rules: self.rules.clone(),
            };
            let s = trs.normalize_innermost(&s, 200);
            let t = trs.normalize_innermost(&t, 200);
            if s == t {
                continue;
            }
            let (lhs, rhs) = match order(&s, &t) {
                std::cmp::Ordering::Greater => (s, t),
                std::cmp::Ordering::Less => (t, s),
                std::cmp::Ordering::Equal => {
                    return Err(format!("KB completion: cannot orient {} = {}", s, t));
                }
            };
            let new_rule = Rule::new(lhs.clone(), rhs.clone());
            let mut new_rules: Vec<Rule> = Vec::new();
            let mut deferred: Vec<(Term, Term)> = Vec::new();
            for r in &self.rules {
                let nr = Trs {
                    rules: vec![new_rule.clone()],
                };
                let lhs2 = nr.normalize_innermost(&r.lhs, 200);
                let rhs2 = nr.normalize_innermost(&r.rhs, 200);
                if lhs2 == r.lhs && rhs2 == r.rhs {
                    new_rules.push(r.clone());
                } else {
                    deferred.push((lhs2, rhs2));
                }
            }
            self.rules = new_rules;
            self.rules.push(new_rule.clone());
            let all_rules = self.rules.clone();
            for (i, r) in all_rules.iter().enumerate() {
                for pair in critical_pairs(&new_rule, r, (i * 2000 + 1) as u32) {
                    self.equations.push_back(pair);
                }
                for pair in critical_pairs(r, &new_rule, (i * 2000 + 3001) as u32) {
                    self.equations.push_back(pair);
                }
            }
            for eq in deferred {
                self.equations.push_back(eq);
            }
        }
        Ok(Trs {
            rules: self.rules.clone(),
        })
    }
}
/// A node in the dependency pair graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyPairNode {
    /// The left-hand side of the dependency pair (root symbol of a subterm call).
    pub lhs_root: String,
    /// The right-hand side root symbol (the called function).
    pub rhs_root: String,
}
/// A Term Rewriting System: a list of rewrite rules.
#[derive(Debug, Clone, Default)]
pub struct Trs {
    pub rules: Vec<Rule>,
}
impl Trs {
    /// Creates an empty TRS.
    pub fn new() -> Self {
        Trs { rules: Vec::new() }
    }
    /// Adds a rule to the TRS.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
    /// Returns `true` if all rules are left-linear.
    pub fn is_left_linear(&self) -> bool {
        self.rules.iter().all(|r| r.is_left_linear())
    }
    /// One-step innermost reduction: reduces the leftmost-innermost redex.
    pub fn reduce_innermost(&self, term: &Term) -> Option<Term> {
        if let Term::Fun(f, args) = term {
            for (i, arg) in args.iter().enumerate() {
                if let Some(reduced) = self.reduce_innermost(arg) {
                    let mut new_args = args.clone();
                    new_args[i] = reduced;
                    return Some(Term::Fun(f.clone(), new_args));
                }
            }
        }
        for rule in &self.rules {
            let renamed = rule.rename(1000);
            if let Some(subst) = unify(&renamed.lhs, term) {
                return Some(renamed.rhs.apply(&subst));
            }
        }
        None
    }
    /// One-step outermost reduction: reduces the leftmost-outermost redex.
    pub fn reduce_outermost(&self, term: &Term) -> Option<Term> {
        for rule in &self.rules {
            let renamed = rule.rename(1000);
            if let Some(subst) = unify(&renamed.lhs, term) {
                return Some(renamed.rhs.apply(&subst));
            }
        }
        if let Term::Fun(f, args) = term {
            for (i, arg) in args.iter().enumerate() {
                if let Some(reduced) = self.reduce_outermost(arg) {
                    let mut new_args = args.clone();
                    new_args[i] = reduced;
                    return Some(Term::Fun(f.clone(), new_args));
                }
            }
        }
        None
    }
    /// Fully reduces a term to normal form under innermost strategy (up to `limit` steps).
    pub fn normalize_innermost(&self, term: &Term, limit: usize) -> Term {
        let mut current = term.clone();
        for _ in 0..limit {
            match self.reduce_innermost(&current) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Fully reduces a term to normal form under outermost strategy (up to `limit` steps).
    pub fn normalize_outermost(&self, term: &Term, limit: usize) -> Term {
        let mut current = term.clone();
        for _ in 0..limit {
            match self.reduce_outermost(&current) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Checks whether `t` is a normal form (no rule applies at any position).
    pub fn is_normal_form(&self, t: &Term) -> bool {
        self.reduce_outermost(t).is_none()
    }
}
/// Available reduction strategies for a TRS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Innermost (leftmost-innermost): reduce innermost redexes first.
    Innermost,
    /// Outermost (leftmost-outermost): reduce outermost redexes first.
    Outermost,
    /// Parallel: reduce all outermost redexes simultaneously.
    Parallel,
    /// Lazy: outermost with sharing (call-by-need approximation).
    Lazy,
}
/// A polynomial interpretation for proving termination of a TRS.
///
/// Each function symbol `f` of arity `n` is mapped to a polynomial
/// p_f(x_1, …, x_n) with natural-number coefficients.  A TRS is terminating
/// if for every rule `l → r`, the polynomial interpretation satisfies
/// Pol(l) > Pol(r) (as natural numbers for all assignments ≥ 0).
#[derive(Debug, Clone)]
pub struct PolynomialInterpretation {
    /// Maps function symbol names to their polynomial coefficients.
    ///
    /// For a unary symbol the vector `[c0, c1]` represents c0 + c1 * x1.
    /// For a binary symbol `[c0, c1, c2]` represents c0 + c1*x1 + c2*x2.
    /// Constants (arity 0) are represented as `[c0]`.
    pub interpretations: HashMap<String, Vec<i64>>,
}
impl PolynomialInterpretation {
    /// Create an empty polynomial interpretation.
    pub fn new() -> Self {
        PolynomialInterpretation {
            interpretations: HashMap::new(),
        }
    }
    /// Register an interpretation for symbol `f`: coefficients `[c0, c1, …]`.
    pub fn set(&mut self, symbol: impl Into<String>, coefficients: Vec<i64>) {
        self.interpretations.insert(symbol.into(), coefficients);
    }
    /// Evaluate the polynomial interpretation of term `t` given variable assignment.
    ///
    /// Variable `Var(i)` maps to `assignment[i]`.
    pub fn eval(&self, t: &Term, assignment: &[i64]) -> i64 {
        match t {
            Term::Var(i) => assignment.get(*i as usize).copied().unwrap_or(0),
            Term::Fun(f, args) => {
                let arg_vals: Vec<i64> = args.iter().map(|a| self.eval(a, assignment)).collect();
                if let Some(coeffs) = self.interpretations.get(f) {
                    let mut result = coeffs.first().copied().unwrap_or(0);
                    for (k, &c) in coeffs.iter().skip(1).enumerate() {
                        result += c * arg_vals.get(k).copied().unwrap_or(0);
                    }
                    result
                } else {
                    0
                }
            }
        }
    }
    /// Check whether rule `lhs → rhs` is oriented by this interpretation for
    /// all variable assignments in `[0, max_val]^n`.
    pub fn orients_rule(&self, rule: &Rule, max_val: i64) -> bool {
        let vars: HashSet<u32> = rule.lhs.vars().union(&rule.rhs.vars()).copied().collect();
        let n = vars.iter().copied().max().map(|m| m + 1).unwrap_or(0) as usize;
        let mut assignment = vec![0i64; n];
        loop {
            let lv = self.eval(&rule.lhs, &assignment);
            let rv = self.eval(&rule.rhs, &assignment);
            if lv <= rv {
                return false;
            }
            let mut carry = true;
            for a in assignment.iter_mut() {
                if carry {
                    *a += 1;
                    if *a > max_val {
                        *a = 0;
                    } else {
                        carry = false;
                    }
                }
            }
            if carry {
                break;
            }
        }
        true
    }
    /// Check whether this interpretation proves termination of the given TRS
    /// (all rules are strictly oriented for assignments in [0, max_val]^n).
    pub fn proves_termination(&self, trs: &Trs, max_val: i64) -> bool {
        trs.rules.iter().all(|r| self.orients_rule(r, max_val))
    }
}
/// Knuth-Bendix completion data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KnuthBendixData {
    /// Current set of rules.
    pub rules: Vec<(String, String)>,
    /// Critical pairs found.
    pub critical_pairs: Vec<(String, String)>,
    /// Whether the system is confluent.
    pub is_confluent: bool,
    /// Termination order description.
    pub order: String,
}
#[allow(dead_code)]
impl KnuthBendixData {
    /// Creates Knuth-Bendix data.
    pub fn new(order: &str) -> Self {
        KnuthBendixData {
            rules: Vec::new(),
            critical_pairs: Vec::new(),
            is_confluent: false,
            order: order.to_string(),
        }
    }
    /// Adds a rule (already oriented).
    pub fn add_oriented_rule(&mut self, lhs: &str, rhs: &str) {
        self.rules.push((lhs.to_string(), rhs.to_string()));
    }
    /// Registers a critical pair.
    pub fn add_critical_pair(&mut self, left: &str, right: &str) {
        self.critical_pairs
            .push((left.to_string(), right.to_string()));
    }
    /// Marks as confluent (after resolving all critical pairs).
    pub fn mark_confluent(&mut self) {
        self.is_confluent = true;
    }
    /// Returns the convergent TRS description.
    pub fn description(&self) -> String {
        format!(
            "KB({} rules, {} crit pairs, confluent={})",
            self.rules.len(),
            self.critical_pairs.len(),
            self.is_confluent
        )
    }
    /// Counts joinable critical pairs.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }
}
/// Represents a reduction strategy for a term rewriting system.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ReductionStrategy {
    /// Leftmost-outermost (normal order).
    LeftmostOutermost,
    /// Leftmost-innermost (applicative order).
    LeftmostInnermost,
    /// Rightmost-outermost.
    RightmostOutermost,
    /// Parallel outermost.
    ParallelOutermost,
    /// Needed reduction (lazy evaluation).
    Needed,
}
#[allow(dead_code)]
impl ReductionStrategy {
    /// Returns the name of the strategy.
    pub fn name(&self) -> &str {
        match self {
            ReductionStrategy::LeftmostOutermost => "Leftmost-Outermost (Normal Order)",
            ReductionStrategy::LeftmostInnermost => "Leftmost-Innermost (Applicative Order)",
            ReductionStrategy::RightmostOutermost => "Rightmost-Outermost",
            ReductionStrategy::ParallelOutermost => "Parallel Outermost",
            ReductionStrategy::Needed => "Needed Reduction (Lazy)",
        }
    }
    /// Checks if this strategy is complete (finds normal form if it exists).
    pub fn is_complete(&self) -> bool {
        matches!(
            self,
            ReductionStrategy::LeftmostOutermost | ReductionStrategy::Needed
        )
    }
    /// Checks if this strategy is normalizing for orthogonal TRS.
    pub fn normalizing_for_orthogonal(&self) -> bool {
        matches!(
            self,
            ReductionStrategy::LeftmostOutermost
                | ReductionStrategy::Needed
                | ReductionStrategy::ParallelOutermost
        )
    }
    /// Returns the corresponding lambda calculus evaluation order.
    pub fn lambda_calculus_analog(&self) -> &str {
        match self {
            ReductionStrategy::LeftmostOutermost => "Normal order reduction",
            ReductionStrategy::LeftmostInnermost => "Call-by-value",
            ReductionStrategy::Needed => "Call-by-need (lazy)",
            _ => "No direct analog",
        }
    }
}
/// A substitution: partial map from variable indices to terms.
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    pub map: HashMap<u32, Term>,
}
impl Substitution {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        Substitution {
            map: HashMap::new(),
        }
    }
    /// Binds variable `v` to `t`.
    pub fn bind(&mut self, v: u32, t: Term) {
        self.map.insert(v, t);
    }
    /// Compose `self` after `other`: `(self ∘ other)(x) = self(other(x))`.
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = Substitution::new();
        for (&v, t) in &other.map {
            result.bind(v, t.apply(self));
        }
        for (&v, t) in &self.map {
            if !other.map.contains_key(&v) {
                result.bind(v, t.clone());
            }
        }
        result
    }
}
/// Represents an E-class in an E-graph.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EClass {
    /// Canonical ID of this e-class.
    pub id: usize,
    /// E-nodes (terms) in this e-class.
    pub nodes: Vec<String>,
    /// Size of the e-class.
    pub size: usize,
}
/// A (bottom-up) tree automaton for recognizing regular tree languages.
///
/// States are `usize` indices.  Transitions are of the form
/// `f(q_1, …, q_n) → q` meaning: if children of `f` are in states q_1, …, q_n
/// then `f(…)` can be assigned state `q`.
#[derive(Debug, Clone)]
pub struct TreeAutomaton {
    /// Number of states.
    pub num_states: usize,
    /// Set of accepting (final) states.
    pub final_states: HashSet<usize>,
    /// Transitions: maps `(symbol, child_states)` to a set of target states.
    pub transitions: HashMap<(String, Vec<usize>), HashSet<usize>>,
}
impl TreeAutomaton {
    /// Create an automaton with `num_states` states.
    pub fn new(num_states: usize) -> Self {
        TreeAutomaton {
            num_states,
            final_states: HashSet::new(),
            transitions: HashMap::new(),
        }
    }
    /// Mark state `q` as a final (accepting) state.
    pub fn add_final(&mut self, q: usize) {
        self.final_states.insert(q);
    }
    /// Add a transition: when `f` is applied to children in states `child_states`,
    /// allow reaching state `target`.
    pub fn add_transition(
        &mut self,
        symbol: impl Into<String>,
        child_states: Vec<usize>,
        target: usize,
    ) {
        self.transitions
            .entry((symbol.into(), child_states))
            .or_default()
            .insert(target);
    }
    /// Run the automaton bottom-up on term `t`.
    ///
    /// Returns the set of states reachable at the root.
    pub fn run(&self, t: &Term) -> HashSet<usize> {
        match t {
            Term::Var(_) => (0..self.num_states).collect(),
            Term::Fun(f, args) => {
                let arg_states: Vec<HashSet<usize>> = args.iter().map(|a| self.run(a)).collect();
                let mut result = HashSet::new();
                if args.is_empty() {
                    if let Some(targets) = self.transitions.get(&(f.clone(), vec![])) {
                        result.extend(targets);
                    }
                } else {
                    let combinations = Self::cartesian(&arg_states);
                    for combo in combinations {
                        if let Some(targets) = self.transitions.get(&(f.clone(), combo)) {
                            result.extend(targets);
                        }
                    }
                }
                result
            }
        }
    }
    /// Cartesian product of sets for computing state combinations.
    fn cartesian(sets: &[HashSet<usize>]) -> Vec<Vec<usize>> {
        if sets.is_empty() {
            return vec![vec![]];
        }
        let mut result = vec![vec![]];
        for set in sets {
            let mut new_result = Vec::new();
            for combo in &result {
                let mut sorted: Vec<usize> = set.iter().copied().collect();
                sorted.sort_unstable();
                for &state in &sorted {
                    let mut new_combo = combo.clone();
                    new_combo.push(state);
                    new_result.push(new_combo);
                }
            }
            result = new_result;
        }
        result
    }
    /// Check whether term `t` is accepted (root reachable in a final state).
    pub fn accepts(&self, t: &Term) -> bool {
        let states = self.run(t);
        states.iter().any(|s| self.final_states.contains(s))
    }
    /// Returns `true` if the language is empty (no ground term is accepted).
    ///
    /// This is a simple fixpoint check: compute the set of states reachable
    /// by any ground term, then check if any final state is reachable.
    pub fn is_empty(&self) -> bool {
        let mut reachable: HashSet<usize> = HashSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for ((_, child_req), targets) in &self.transitions {
                if child_req.iter().all(|s| reachable.contains(s)) {
                    for &t in targets {
                        if reachable.insert(t) {
                            changed = true;
                        }
                    }
                }
            }
        }
        !reachable.iter().any(|s| self.final_states.contains(s))
    }
}
/// A string rewriting system rule: `lhs → rhs` over a string alphabet.
#[derive(Debug, Clone)]
pub struct SrsRule {
    pub lhs: String,
    pub rhs: String,
}
/// A string rewriting system (monoid presentation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StringRewritingSystem {
    /// Alphabet.
    pub alphabet: Vec<char>,
    /// Rules: (lhs, rhs) as strings over the alphabet.
    pub rules: Vec<(String, String)>,
}
#[allow(dead_code)]
impl StringRewritingSystem {
    /// Creates a new SRS.
    pub fn new(alphabet: Vec<char>) -> Self {
        StringRewritingSystem {
            alphabet,
            rules: Vec::new(),
        }
    }
    /// Adds a rule.
    pub fn add_rule(&mut self, lhs: &str, rhs: &str) {
        self.rules.push((lhs.to_string(), rhs.to_string()));
    }
    /// Applies one step of rewriting to a string (leftmost first match).
    pub fn rewrite_step(&self, s: &str) -> Option<String> {
        for (lhs, rhs) in &self.rules {
            if let Some(pos) = s.find(lhs.as_str()) {
                let result = format!("{}{}{}", &s[..pos], rhs, &s[pos + lhs.len()..]);
                return Some(result);
            }
        }
        None
    }
    /// Applies rewriting until normal form (limit iterations).
    pub fn normalize(&self, s: &str, max_steps: usize) -> String {
        let mut current = s.to_string();
        for _ in 0..max_steps {
            match self.rewrite_step(&current) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Checks if two strings are equal modulo rewriting (up to max_steps).
    pub fn are_equal_modulo(&self, s1: &str, s2: &str, max_steps: usize) -> bool {
        let n1 = self.normalize(s1, max_steps);
        let n2 = self.normalize(s2, max_steps);
        n1 == n2
    }
    /// Returns the number of rules.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }
}
/// An equational theory presented by equations.
#[derive(Debug, Clone, Default)]
pub struct EquationalTheory {
    /// Equational axioms `(lhs, rhs)`.
    pub axioms: Vec<(Term, Term)>,
}
impl EquationalTheory {
    /// Creates an empty equational theory.
    pub fn new() -> Self {
        EquationalTheory { axioms: Vec::new() }
    }
    /// Adds an axiom `lhs = rhs` to the theory.
    pub fn add_axiom(&mut self, lhs: Term, rhs: Term) {
        self.axioms.push((lhs, rhs));
    }
    /// Naive E-unification by closure: attempts to unify `s` and `t` modulo
    /// the equational theory by rewriting.  Returns a substitution if found.
    pub fn e_unify(&self, s: &Term, t: &Term, depth_limit: usize) -> Option<Substitution> {
        let mut trs = Trs::new();
        for (lhs, rhs) in &self.axioms {
            trs.add_rule(Rule::new(lhs.clone(), rhs.clone()));
            trs.add_rule(Rule::new(rhs.clone(), lhs.clone()));
        }
        let s_nf = trs.normalize_innermost(s, depth_limit);
        let t_nf = trs.normalize_innermost(t, depth_limit);
        unify(&s_nf, &t_nf)
    }
}
/// A String Rewriting System.
#[derive(Debug, Clone)]
pub struct Srs {
    pub rules: Vec<SrsRule>,
}
impl Srs {
    /// Creates an empty SRS.
    pub fn new() -> Self {
        Srs { rules: Vec::new() }
    }
    /// Adds a rule `lhs → rhs`.
    pub fn add_rule(&mut self, lhs: impl Into<String>, rhs: impl Into<String>) {
        self.rules.push(SrsRule {
            lhs: lhs.into(),
            rhs: rhs.into(),
        });
    }
    /// One-step reduction: applies the first applicable rule at any position.
    pub fn step(&self, s: &str) -> Option<String> {
        for rule in &self.rules {
            if let Some(pos) = s.find(&rule.lhs) {
                let result = format!("{}{}{}", &s[..pos], rule.rhs, &s[pos + rule.lhs.len()..]);
                return Some(result);
            }
        }
        None
    }
    /// Fully reduce a string to its normal form (up to `limit` steps).
    pub fn normalize(&self, s: &str, limit: usize) -> String {
        let mut current = s.to_owned();
        for _ in 0..limit {
            match self.step(&current) {
                Some(next) => current = next,
                None => break,
            }
        }
        current
    }
    /// Checks whether two strings are equivalent under the congruence generated
    /// by this SRS (compares normal forms).
    pub fn word_equivalent(&self, s: &str, t: &str, limit: usize) -> bool {
        self.normalize(s, limit) == self.normalize(t, limit)
    }
}
/// A dependency pair graph for a TRS — used to prove termination.
///
/// In the dependency pair method a "dependency pair" is derived from each rule
/// `f(l) → C[g(r)]` where `g` is a defined symbol.  Termination is equivalent
/// to the non-existence of infinite chains in the dependency pair graph.
#[derive(Debug, Clone, Default)]
pub struct DependencyPairGraph {
    /// The dependency pairs (nodes of the graph).
    pub pairs: Vec<DependencyPairNode>,
    /// Edges: `edges[i]` is the set of indices j such that pair i may precede pair j.
    pub edges: Vec<HashSet<usize>>,
}
impl DependencyPairGraph {
    /// Create an empty dependency pair graph.
    pub fn new() -> Self {
        DependencyPairGraph {
            pairs: Vec::new(),
            edges: Vec::new(),
        }
    }
    /// Add a dependency pair (lhs_root, rhs_root) and return its index.
    pub fn add_pair(&mut self, lhs_root: impl Into<String>, rhs_root: impl Into<String>) -> usize {
        let idx = self.pairs.len();
        self.pairs.push(DependencyPairNode {
            lhs_root: lhs_root.into(),
            rhs_root: rhs_root.into(),
        });
        self.edges.push(HashSet::new());
        idx
    }
    /// Add an edge from pair `from` to pair `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.edges.len() {
            self.edges[from].insert(to);
        }
    }
    /// Find all strongly connected components (SCCs) via a simple iterative DFS.
    ///
    /// Returns a list of SCCs, each represented as a list of pair indices.
    pub fn sccs(&self) -> Vec<Vec<usize>> {
        let n = self.pairs.len();
        let mut visited = vec![false; n];
        let mut finish_order: Vec<usize> = Vec::new();
        for start in 0..n {
            if !visited[start] {
                let mut stack: Vec<(usize, bool)> = vec![(start, false)];
                while let Some((node, done)) = stack.pop() {
                    if done {
                        finish_order.push(node);
                        continue;
                    }
                    if visited[node] {
                        continue;
                    }
                    visited[node] = true;
                    stack.push((node, true));
                    for &next in &self.edges[node] {
                        if !visited[next] {
                            stack.push((next, false));
                        }
                    }
                }
            }
        }
        let mut rev_edges: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for i in 0..n {
            for &j in &self.edges[i] {
                rev_edges[j].insert(i);
            }
        }
        let mut component_id = vec![usize::MAX; n];
        let mut sccs: Vec<Vec<usize>> = Vec::new();
        let mut visited2 = vec![false; n];
        for &start in finish_order.iter().rev() {
            if visited2[start] {
                continue;
            }
            let scc_id = sccs.len();
            let mut component: Vec<usize> = Vec::new();
            let mut stack: Vec<usize> = vec![start];
            while let Some(node) = stack.pop() {
                if visited2[node] {
                    continue;
                }
                visited2[node] = true;
                component_id[node] = scc_id;
                component.push(node);
                for &prev in &rev_edges[node] {
                    if !visited2[prev] {
                        stack.push(prev);
                    }
                }
            }
            sccs.push(component);
        }
        let _ = component_id;
        sccs
    }
    /// Returns `true` if all SCCs are trivial (size ≤ 1 and no self-loops).
    ///
    /// A TRS is terminating iff its dependency pair graph has no infinite chains,
    /// which holds when all non-trivial SCCs can be removed by a reduction pair.
    pub fn all_sccs_trivial(&self) -> bool {
        for scc in self.sccs() {
            if scc.len() > 1 {
                return false;
            }
            if let Some(&node) = scc.first() {
                if self.edges[node].contains(&node) {
                    return false;
                }
            }
        }
        true
    }
    /// Derive dependency pairs from a Trs by inspecting rule root symbols.
    pub fn from_trs(trs: &Trs) -> Self {
        let mut graph = DependencyPairGraph::new();
        let defined: HashSet<String> = trs
            .rules
            .iter()
            .filter_map(|r| {
                if let Term::Fun(f, _) = &r.lhs {
                    Some(f.clone())
                } else {
                    None
                }
            })
            .collect();
        fn collect_calls(t: &Term, defined: &HashSet<String>, calls: &mut Vec<String>) {
            if let Term::Fun(f, args) = t {
                if defined.contains(f.as_str()) {
                    calls.push(f.clone());
                }
                for a in args {
                    collect_calls(a, defined, calls);
                }
            }
        }
        for rule in &trs.rules {
            if let Term::Fun(lhs_f, _) = &rule.lhs {
                if defined.contains(lhs_f.as_str()) {
                    let mut calls = Vec::new();
                    collect_calls(&rule.rhs, &defined, &mut calls);
                    for rhs_f in calls {
                        graph.add_pair(lhs_f.clone(), rhs_f);
                    }
                }
            }
        }
        let n = graph.pairs.len();
        for i in 0..n {
            for j in 0..n {
                if graph.pairs[i].rhs_root == graph.pairs[j].lhs_root {
                    graph.add_edge(i, j);
                }
            }
        }
        graph
    }
}
