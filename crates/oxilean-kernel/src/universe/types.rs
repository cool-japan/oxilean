//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::level::{self, LevelMVarId};
use crate::{Level, Name};
use std::collections::{HashMap, HashSet};

use super::functions::{add_succs, collect_nf_comps, format_level, substitute_level_param};

/// Represents a rewrite rule `lhs → rhs`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RewriteRule {
    /// The name of the rule.
    pub name: String,
    /// A string representation of the LHS pattern.
    pub lhs: String,
    /// A string representation of the RHS.
    pub rhs: String,
    /// Whether this is a conditional rule (has side conditions).
    pub conditional: bool,
}
#[allow(dead_code)]
impl RewriteRule {
    /// Creates an unconditional rewrite rule.
    pub fn unconditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: false,
        }
    }
    /// Creates a conditional rewrite rule.
    pub fn conditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: true,
        }
    }
    /// Returns a textual representation.
    pub fn display(&self) -> String {
        format!("{}: {} → {}", self.name, self.lhs, self.rhs)
    }
}
/// Universe satisfiability checker.
#[derive(Debug, Clone, Default)]
pub struct UnivSatChecker {
    lower_bounds: std::collections::HashMap<Name, u32>,
    upper_bounds: std::collections::HashMap<Name, u32>,
    unsatisfiable: bool,
}
impl UnivSatChecker {
    /// Create new.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add param >= n.
    pub fn add_lower_bound(&mut self, param: Name, n: u32) {
        let entry = self.lower_bounds.entry(param.clone()).or_insert(0);
        *entry = (*entry).max(n);
        if let Some(ub) = self.upper_bounds.get(&param) {
            if *entry > *ub {
                self.unsatisfiable = true;
            }
        }
    }
    /// Add param <= n.
    pub fn add_upper_bound(&mut self, param: Name, n: u32) {
        let entry = self.upper_bounds.entry(param.clone()).or_insert(u32::MAX);
        *entry = (*entry).min(n);
        if let Some(lb) = self.lower_bounds.get(&param) {
            if *lb > *entry {
                self.unsatisfiable = true;
            }
        }
    }
    /// Check satisfiability.
    pub fn is_satisfiable(&self) -> bool {
        !self.unsatisfiable
    }
    /// Get an assignment if satisfiable.
    pub fn get_assignment(&self) -> Option<std::collections::HashMap<Name, u32>> {
        if self.unsatisfiable {
            return None;
        }
        let mut m = std::collections::HashMap::new();
        for (p, lb) in &self.lower_bounds {
            m.insert(p.clone(), *lb);
        }
        for p in self.upper_bounds.keys() {
            m.entry(p.clone()).or_insert(0);
        }
        Some(m)
    }
}
/// A universe constraint.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnivConstraint {
    /// u < v (strict ordering)
    Lt(Level, Level),
    /// u <= v (non-strict ordering)
    Le(Level, Level),
    /// u = v (equality)
    Eq(Level, Level),
}
/// A versioned record that stores a history of values.
#[allow(dead_code)]
pub struct VersionedRecord<T: Clone> {
    history: Vec<T>,
}
#[allow(dead_code)]
impl<T: Clone> VersionedRecord<T> {
    /// Creates a new record with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }
    /// Updates the record with a new version.
    pub fn update(&mut self, val: T) {
        self.history.push(val);
    }
    /// Returns the current (latest) value.
    pub fn current(&self) -> &T {
        self.history
            .last()
            .expect("VersionedRecord history is always non-empty after construction")
    }
    /// Returns the value at version `n` (0-indexed), or `None`.
    pub fn at_version(&self, n: usize) -> Option<&T> {
        self.history.get(n)
    }
    /// Returns the version number of the current value.
    pub fn version(&self) -> usize {
        self.history.len() - 1
    }
    /// Returns `true` if more than one version exists.
    pub fn has_history(&self) -> bool {
        self.history.len() > 1
    }
}
/// A window iterator that yields overlapping windows of size `n`.
#[allow(dead_code)]
pub struct WindowIterator<'a, T> {
    pub(super) data: &'a [T],
    pub(super) pos: usize,
    pub(super) window: usize,
}
#[allow(dead_code)]
impl<'a, T> WindowIterator<'a, T> {
    /// Creates a new window iterator.
    pub fn new(data: &'a [T], window: usize) -> Self {
        Self {
            data,
            pos: 0,
            window,
        }
    }
}
/// A simple key-value store backed by a sorted Vec for small maps.
#[allow(dead_code)]
pub struct SmallMap<K: Ord + Clone, V: Clone> {
    entries: Vec<(K, V)>,
}
#[allow(dead_code)]
impl<K: Ord + Clone, V: Clone> SmallMap<K, V> {
    /// Creates a new empty small map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: K, val: V) {
        match self.entries.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(i) => self.entries[i].1 = val,
            Err(i) => self.entries.insert(i, (key, val)),
        }
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .binary_search_by_key(&key, |(k, _)| k)
            .ok()
            .map(|i| &self.entries[i].1)
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Returns all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
    /// Returns all values.
    pub fn values(&self) -> Vec<&V> {
        self.entries.iter().map(|(_, v)| v).collect()
    }
}
/// A set of rewrite rules.
#[allow(dead_code)]
pub struct RewriteRuleSet {
    rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteRuleSet {
    /// Creates an empty rule set.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Adds a rule.
    pub fn add(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Returns the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Returns all conditional rules.
    pub fn conditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| r.conditional).collect()
    }
    /// Returns all unconditional rules.
    pub fn unconditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| !r.conditional).collect()
    }
    /// Looks up a rule by name.
    pub fn get(&self, name: &str) -> Option<&RewriteRule> {
        self.rules.iter().find(|r| r.name == name)
    }
}
/// Universe level comparison table.
///
/// Precomputes and caches level comparison results for a fixed list of levels.
#[derive(Debug, Clone, Default)]
pub struct LevelComparisonTable {
    levels: Vec<Level>,
    /// (i, j) → geq(levels[i], levels[j])
    geq_cache: std::collections::HashMap<(usize, usize), bool>,
}
impl LevelComparisonTable {
    /// Create a new table for a set of levels.
    pub fn new(levels: Vec<Level>) -> Self {
        let mut table = Self {
            levels,
            geq_cache: Default::default(),
        };
        table.precompute();
        table
    }
    fn precompute(&mut self) {
        let checker = UnivChecker::new();
        let n = self.levels.len();
        for i in 0..n {
            for j in 0..n {
                let result = checker.is_geq(&self.levels[i].clone(), &self.levels[j].clone());
                self.geq_cache.insert((i, j), result);
            }
        }
    }
    /// Check geq(levels\[i\], levels\[j\]).
    pub fn geq(&self, i: usize, j: usize) -> Option<bool> {
        self.geq_cache.get(&(i, j)).copied()
    }
    /// Get the maximum level index.
    pub fn max_idx(&self) -> Option<usize> {
        (0..self.levels.len()).max_by(|&i, &j| {
            let checker = UnivChecker::new();
            if checker.is_geq(&self.levels[i], &self.levels[j]) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        })
    }
    /// Number of levels.
    pub fn len(&self) -> usize {
        self.levels.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }
}
/// A tagged union for representing a simple two-case discriminated union.
#[allow(dead_code)]
pub enum Either2<A, B> {
    /// The first alternative.
    First(A),
    /// The second alternative.
    Second(B),
}
#[allow(dead_code)]
impl<A, B> Either2<A, B> {
    /// Returns `true` if this is the first alternative.
    pub fn is_first(&self) -> bool {
        matches!(self, Either2::First(_))
    }
    /// Returns `true` if this is the second alternative.
    pub fn is_second(&self) -> bool {
        matches!(self, Either2::Second(_))
    }
    /// Returns the first value if present.
    pub fn first(self) -> Option<A> {
        match self {
            Either2::First(a) => Some(a),
            _ => None,
        }
    }
    /// Returns the second value if present.
    pub fn second(self) -> Option<B> {
        match self {
            Either2::Second(b) => Some(b),
            _ => None,
        }
    }
    /// Maps over the first alternative.
    pub fn map_first<C, F: FnOnce(A) -> C>(self, f: F) -> Either2<C, B> {
        match self {
            Either2::First(a) => Either2::First(f(a)),
            Either2::Second(b) => Either2::Second(b),
        }
    }
}
/// A write-once cell.
#[allow(dead_code)]
pub struct WriteOnce<T> {
    value: std::cell::Cell<Option<T>>,
}
#[allow(dead_code)]
impl<T: Copy> WriteOnce<T> {
    /// Creates an empty write-once cell.
    pub fn new() -> Self {
        Self {
            value: std::cell::Cell::new(None),
        }
    }
    /// Writes a value.  Returns `false` if already written.
    pub fn write(&self, val: T) -> bool {
        if self.value.get().is_some() {
            return false;
        }
        self.value.set(Some(val));
        true
    }
    /// Returns the value if written.
    pub fn read(&self) -> Option<T> {
        self.value.get()
    }
    /// Returns `true` if the value has been written.
    pub fn is_written(&self) -> bool {
        self.value.get().is_some()
    }
}
/// A pair of `StatSummary` values tracking before/after a transformation.
#[allow(dead_code)]
pub struct TransformStat {
    before: StatSummary,
    after: StatSummary,
}
#[allow(dead_code)]
impl TransformStat {
    /// Creates a new transform stat recorder.
    pub fn new() -> Self {
        Self {
            before: StatSummary::new(),
            after: StatSummary::new(),
        }
    }
    /// Records a before value.
    pub fn record_before(&mut self, v: f64) {
        self.before.record(v);
    }
    /// Records an after value.
    pub fn record_after(&mut self, v: f64) {
        self.after.record(v);
    }
    /// Returns the mean reduction ratio (after/before).
    pub fn mean_ratio(&self) -> Option<f64> {
        let b = self.before.mean()?;
        let a = self.after.mean()?;
        if b.abs() < f64::EPSILON {
            return None;
        }
        Some(a / b)
    }
}
/// A token bucket rate limiter.
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_per_ms: u64,
    last_refill: std::time::Instant,
}
#[allow(dead_code)]
impl TokenBucket {
    /// Creates a new token bucket.
    pub fn new(capacity: u64, refill_per_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_ms,
            last_refill: std::time::Instant::now(),
        }
    }
    /// Attempts to consume `n` tokens.  Returns `true` on success.
    pub fn try_consume(&mut self, n: u64) -> bool {
        self.refill();
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_refill).as_millis() as u64;
        if elapsed_ms > 0 {
            let new_tokens = elapsed_ms * self.refill_per_ms;
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
    /// Returns the number of currently available tokens.
    pub fn available(&self) -> u64 {
        self.tokens
    }
    /// Returns the bucket capacity.
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}
/// Level normal form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelNormalForm {
    /// The components (base, succ_count).
    pub components: Vec<(Option<Name>, u32)>,
}
impl LevelNormalForm {
    /// Create from level.
    pub fn from_level(l: &Level) -> Self {
        let mut comps = Vec::new();
        collect_nf_comps(l, 0, &mut comps);
        comps.dedup();
        LevelNormalForm { components: comps }
    }
    /// Convert back to level.
    pub fn to_level(&self) -> Level {
        if self.components.is_empty() {
            return Level::zero();
        }
        let levels: Vec<Level> = self
            .components
            .iter()
            .map(|(base, succs)| {
                let base_l = match base {
                    Some(name) => Level::param(name.clone()),
                    None => Level::zero(),
                };
                add_succs(base_l, *succs)
            })
            .collect();
        levels
            .into_iter()
            .reduce(Level::max)
            .unwrap_or(Level::zero())
    }
}
/// Universe polymorphic declaration signature.
///
/// Tracks the universe parameters and their constraints for a declaration.
#[derive(Debug, Clone)]
pub struct UnivPolySignature {
    /// Universe parameter names.
    pub params: Vec<Name>,
    /// Constraints on those parameters.
    pub constraints: UnivConstraintSet,
}
impl UnivPolySignature {
    /// Create a signature with given parameters and no constraints.
    pub fn new(params: Vec<Name>) -> Self {
        Self {
            params,
            constraints: UnivConstraintSet::new(),
        }
    }
    /// Add a constraint.
    pub fn add_constraint(&mut self, c: UnivConstraint) {
        self.constraints.add(c);
    }
    /// Number of universe parameters.
    pub fn arity(&self) -> usize {
        self.params.len()
    }
    /// Instantiate the signature with concrete levels.
    ///
    /// Returns `None` if the wrong number of arguments is provided.
    pub fn instantiate(&self, args: &[Level]) -> Option<UniverseInstantiation> {
        if args.len() != self.params.len() {
            return None;
        }
        let mut inst = UniverseInstantiation::new();
        for (p, l) in self.params.iter().zip(args.iter()) {
            inst.add(p.clone(), l.clone());
        }
        Some(inst)
    }
    /// Check if the given instantiation satisfies the constraints.
    pub fn check_instantiation(&self, inst: &UniverseInstantiation) -> Result<(), String> {
        let mut checker = UnivChecker::new();
        for p in &self.params {
            checker.add_univ_var(p.clone());
        }
        for c in &self.constraints.constraints {
            let c_inst = match c {
                UnivConstraint::Lt(u, v) => UnivConstraint::Lt(inst.apply(u), inst.apply(v)),
                UnivConstraint::Le(u, v) => UnivConstraint::Le(inst.apply(u), inst.apply(v)),
                UnivConstraint::Eq(u, v) => UnivConstraint::Eq(inst.apply(u), inst.apply(v)),
            };
            checker.add_constraint(c_inst);
        }
        checker.check()
    }
}
/// A sparse vector: stores only non-default elements.
#[allow(dead_code)]
pub struct SparseVec<T: Default + Clone + PartialEq> {
    entries: std::collections::HashMap<usize, T>,
    default_: T,
    logical_len: usize,
}
#[allow(dead_code)]
impl<T: Default + Clone + PartialEq> SparseVec<T> {
    /// Creates a new sparse vector with logical length `len`.
    pub fn new(len: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            default_: T::default(),
            logical_len: len,
        }
    }
    /// Sets element at `idx`.
    pub fn set(&mut self, idx: usize, val: T) {
        if val == self.default_ {
            self.entries.remove(&idx);
        } else {
            self.entries.insert(idx, val);
        }
    }
    /// Gets element at `idx`.
    pub fn get(&self, idx: usize) -> &T {
        self.entries.get(&idx).unwrap_or(&self.default_)
    }
    /// Returns the logical length.
    pub fn len(&self) -> usize {
        self.logical_len
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of non-default elements.
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }
}
/// A simple directed acyclic graph.
#[allow(dead_code)]
pub struct SimpleDag {
    /// `edges[i]` is the list of direct successors of node `i`.
    edges: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl SimpleDag {
    /// Creates a DAG with `n` nodes and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n],
        }
    }
    /// Adds an edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.edges.len() {
            self.edges[from].push(to);
        }
    }
    /// Returns the successors of `node`.
    pub fn successors(&self, node: usize) -> &[usize] {
        self.edges.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns `true` if `from` can reach `to` via DFS.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        let mut visited = vec![false; self.edges.len()];
        self.dfs(from, to, &mut visited)
    }
    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> bool {
        if cur == target {
            return true;
        }
        if cur >= visited.len() || visited[cur] {
            return false;
        }
        visited[cur] = true;
        for &next in self.successors(cur) {
            if self.dfs(next, target, visited) {
                return true;
            }
        }
        false
    }
    /// Returns the topological order of nodes, or `None` if a cycle is detected.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.edges.len();
        let mut in_degree = vec![0usize; n];
        for succs in &self.edges {
            for &s in succs {
                if s < n {
                    in_degree[s] += 1;
                }
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &s in self.successors(node) {
                if s < n {
                    in_degree[s] -= 1;
                    if in_degree[s] == 0 {
                        queue.push_back(s);
                    }
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }
    /// Returns the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.edges.len()
    }
}
/// A polymorphic instantiation: maps universe parameter names to levels.
#[derive(Debug, Clone, Default)]
pub struct UniverseInstantiation {
    /// The substitutions (parameter name → level).
    pub subst: std::collections::HashMap<Name, Level>,
}
impl UniverseInstantiation {
    /// Create an empty instantiation.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a substitution.
    pub fn add(&mut self, param: Name, level: Level) {
        self.subst.insert(param, level);
    }
    /// Apply the instantiation to a level.
    pub fn apply(&self, l: &Level) -> Level {
        let mut result = l.clone();
        for (p, replacement) in &self.subst {
            result = substitute_level_param(&result, p, replacement);
        }
        result
    }
    /// Number of substitutions.
    pub fn len(&self) -> usize {
        self.subst.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.subst.is_empty()
    }
    /// Compose with another instantiation (other is applied first).
    pub fn compose(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for (p, l) in &other.subst {
            result.add(p.clone(), self.apply(l));
        }
        for (p, l) in &self.subst {
            result.subst.entry(p.clone()).or_insert_with(|| l.clone());
        }
        result
    }
}
/// A generic counter that tracks min/max/sum for statistical summaries.
#[allow(dead_code)]
pub struct StatSummary {
    count: u64,
    sum: f64,
    min: f64,
    max: f64,
}
#[allow(dead_code)]
impl StatSummary {
    /// Creates an empty summary.
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    /// Records a sample.
    pub fn record(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }
    /// Returns the mean, or `None` if no samples.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the minimum, or `None` if no samples.
    pub fn min(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.min)
        }
    }
    /// Returns the maximum, or `None` if no samples.
    pub fn max(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.max)
        }
    }
    /// Returns the count of recorded samples.
    pub fn count(&self) -> u64 {
        self.count
    }
}
/// A fixed-size sliding window that computes a running sum.
#[allow(dead_code)]
pub struct SlidingSum {
    window: Vec<f64>,
    capacity: usize,
    pos: usize,
    sum: f64,
    count: usize,
}
#[allow(dead_code)]
impl SlidingSum {
    /// Creates a sliding sum with the given window size.
    pub fn new(capacity: usize) -> Self {
        Self {
            window: vec![0.0; capacity],
            capacity,
            pos: 0,
            sum: 0.0,
            count: 0,
        }
    }
    /// Adds a value to the window, removing the oldest if full.
    pub fn push(&mut self, val: f64) {
        let oldest = self.window[self.pos];
        self.sum -= oldest;
        self.sum += val;
        self.window[self.pos] = val;
        self.pos = (self.pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }
    /// Returns the current window sum.
    pub fn sum(&self) -> f64 {
        self.sum
    }
    /// Returns the window mean, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the current window size (number of valid elements).
    pub fn count(&self) -> usize {
        self.count
    }
}
/// A dependency closure builder (transitive closure via BFS).
#[allow(dead_code)]
pub struct TransitiveClosure {
    adj: Vec<Vec<usize>>,
    n: usize,
}
#[allow(dead_code)]
impl TransitiveClosure {
    /// Creates a transitive closure builder for `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }
    /// Adds a direct edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n {
            self.adj[from].push(to);
        }
    }
    /// Computes all nodes reachable from `start` (including `start`).
    pub fn reachable_from(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node >= self.n || visited[node] {
                continue;
            }
            visited[node] = true;
            for &next in &self.adj[node] {
                queue.push_back(next);
            }
        }
        (0..self.n).filter(|&i| visited[i]).collect()
    }
    /// Returns `true` if `from` can transitively reach `to`.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        self.reachable_from(from).contains(&to)
    }
}
/// A type-erased function pointer with arity tracking.
#[allow(dead_code)]
pub struct RawFnPtr {
    /// The raw function pointer (stored as usize for type erasure).
    ptr: usize,
    arity: usize,
    name: String,
}
#[allow(dead_code)]
impl RawFnPtr {
    /// Creates a new raw function pointer descriptor.
    pub fn new(ptr: usize, arity: usize, name: impl Into<String>) -> Self {
        Self {
            ptr,
            arity,
            name: name.into(),
        }
    }
    /// Returns the arity.
    pub fn arity(&self) -> usize {
        self.arity
    }
    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the raw pointer value.
    pub fn raw(&self) -> usize {
        self.ptr
    }
}
/// A non-empty list (at least one element guaranteed).
#[allow(dead_code)]
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}
#[allow(dead_code)]
impl<T> NonEmptyVec<T> {
    /// Creates a non-empty vec with a single element.
    pub fn singleton(val: T) -> Self {
        Self {
            head: val,
            tail: Vec::new(),
        }
    }
    /// Pushes an element.
    pub fn push(&mut self, val: T) {
        self.tail.push(val);
    }
    /// Returns a reference to the first element.
    pub fn first(&self) -> &T {
        &self.head
    }
    /// Returns a reference to the last element.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns all elements as a Vec.
    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![&self.head];
        v.extend(self.tail.iter());
        v
    }
}
/// A reusable scratch buffer for path computations.
#[allow(dead_code)]
pub struct PathBuf {
    components: Vec<String>,
}
#[allow(dead_code)]
impl PathBuf {
    /// Creates a new empty path buffer.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    /// Pushes a component.
    pub fn push(&mut self, comp: impl Into<String>) {
        self.components.push(comp.into());
    }
    /// Pops the last component.
    pub fn pop(&mut self) {
        self.components.pop();
    }
    /// Returns the current path as a `/`-separated string.
    pub fn as_str(&self) -> String {
        self.components.join("/")
    }
    /// Returns the depth of the path.
    pub fn depth(&self) -> usize {
        self.components.len()
    }
    /// Clears the path.
    pub fn clear(&mut self) {
        self.components.clear();
    }
}
/// Universe level constraint set with satisfiability checking.
#[derive(Debug, Clone, Default)]
pub struct UnivConstraintSet {
    constraints: Vec<UnivConstraint>,
}
impl UnivConstraintSet {
    /// Create empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint.
    pub fn add(&mut self, c: UnivConstraint) {
        if !self.constraints.contains(&c) {
            self.constraints.push(c);
        }
    }
    /// Merge another set.
    pub fn merge(&mut self, other: &Self) {
        for c in &other.constraints {
            self.add(c.clone());
        }
    }
    /// Number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Check all constraints against a checker.
    pub fn check_all(&self, checker: &UnivChecker) -> Result<(), String> {
        for c in &self.constraints {
            match c {
                UnivConstraint::Lt(u, v) => {
                    if !checker.is_gt(v, u) {
                        return Err(format!(
                            "constraint {} < {} violated",
                            format_level(u),
                            format_level(v)
                        ));
                    }
                }
                UnivConstraint::Le(u, v) => {
                    if !checker.is_geq(v, u) {
                        return Err(format!(
                            "constraint {} <= {} violated",
                            format_level(u),
                            format_level(v)
                        ));
                    }
                }
                UnivConstraint::Eq(u, v) => {
                    if !checker.is_level_def_eq(u, v) {
                        return Err(format!(
                            "constraint {} = {} violated",
                            format_level(u),
                            format_level(v)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    /// Deduplicate constraints.
    pub fn dedup(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.constraints.retain(|c| seen.insert(c.clone()));
    }
}
/// Universe constraint checker with real constraint solving.
pub struct UnivChecker {
    /// Accumulated constraints
    constraints: Vec<UnivConstraint>,
    /// Universe variables (parameters)
    univ_vars: HashSet<Name>,
    /// Level metavariable assignments
    mvar_assignments: HashMap<LevelMVarId, Level>,
    /// Next fresh metavar id
    next_mvar_id: u64,
}
impl UnivChecker {
    /// Create a new universe checker.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            univ_vars: HashSet::new(),
            mvar_assignments: HashMap::new(),
            next_mvar_id: 0,
        }
    }
    /// Add a universe variable.
    pub fn add_univ_var(&mut self, name: Name) {
        self.univ_vars.insert(name);
    }
    /// Add a constraint.
    pub fn add_constraint(&mut self, constraint: UnivConstraint) {
        self.constraints.push(constraint);
    }
    /// Create a fresh level metavariable.
    pub fn fresh_level_mvar(&mut self) -> Level {
        let id = LevelMVarId(self.next_mvar_id);
        self.next_mvar_id += 1;
        Level::MVar(id)
    }
    /// Assign a level metavariable.
    pub fn assign_mvar(&mut self, id: LevelMVarId, level: Level) {
        self.mvar_assignments.insert(id, level);
    }
    /// Get a metavariable assignment.
    pub fn get_mvar_assignment(&self, id: &LevelMVarId) -> Option<&Level> {
        self.mvar_assignments.get(id)
    }
    /// Instantiate all assigned metavariables in a level.
    pub fn instantiate_mvars(&self, l: &Level) -> Level {
        level::instantiate_level_mvars(l, &|id| self.mvar_assignments.get(&id).cloned())
    }
    /// Check if two levels are definitionally equal.
    ///
    /// Uses normalization: two levels are equal iff their normal forms are equal.
    pub fn is_level_def_eq(&self, l1: &Level, l2: &Level) -> bool {
        let l1_inst = self.instantiate_mvars(l1);
        let l2_inst = self.instantiate_mvars(l2);
        level::is_equivalent(&l1_inst, &l2_inst)
    }
    /// Check if l1 >= l2 under all possible assignments.
    pub fn is_geq(&self, l1: &Level, l2: &Level) -> bool {
        let l1_inst = self.instantiate_mvars(l1);
        let l2_inst = self.instantiate_mvars(l2);
        level::is_geq(&l1_inst, &l2_inst)
    }
    /// Check if l1 > l2 under all possible assignments.
    ///
    /// l1 > l2 iff l1 >= succ(l2)
    pub fn is_gt(&self, l1: &Level, l2: &Level) -> bool {
        let l1_inst = self.instantiate_mvars(l1);
        let l2_inst = self.instantiate_mvars(l2);
        level::is_geq(&l1_inst, &Level::succ(l2_inst))
    }
    /// Check if all accumulated constraints are satisfiable.
    pub fn check(&self) -> Result<(), String> {
        for constraint in &self.constraints {
            match constraint {
                UnivConstraint::Lt(u, v) => {
                    let u_inst = self.instantiate_mvars(u);
                    let v_inst = self.instantiate_mvars(v);
                    if !self.check_lt(&u_inst, &v_inst) {
                        return Err(format!(
                            "Universe constraint violated: {} < {}",
                            u_inst, v_inst
                        ));
                    }
                }
                UnivConstraint::Le(u, v) => {
                    let u_inst = self.instantiate_mvars(u);
                    let v_inst = self.instantiate_mvars(v);
                    if !level::is_geq(&v_inst, &u_inst) {
                        return Err(format!(
                            "Universe constraint violated: {} <= {}",
                            u_inst, v_inst
                        ));
                    }
                }
                UnivConstraint::Eq(u, v) => {
                    let u_inst = self.instantiate_mvars(u);
                    let v_inst = self.instantiate_mvars(v);
                    if !level::is_equivalent(&u_inst, &v_inst) {
                        return Err(format!(
                            "Universe constraint violated: {} = {}",
                            u_inst, v_inst
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    /// Check if u < v for concrete levels.
    fn check_lt(&self, u: &Level, v: &Level) -> bool {
        level::is_geq(v, &Level::succ(u.clone()))
    }
    /// Try to solve level metavariables from equality constraints.
    ///
    /// This is a simple first-order solver: if we have ?m = l where l
    /// has no metavariables, assign ?m := l.
    pub fn solve_simple(&mut self) -> bool {
        let mut changed = true;
        let mut any_solved = false;
        while changed {
            changed = false;
            let constraints = self.constraints.clone();
            for constraint in &constraints {
                if let UnivConstraint::Eq(l, r) = constraint {
                    let l_inst = self.instantiate_mvars(l);
                    let r_inst = self.instantiate_mvars(r);
                    if let Level::MVar(id) = &l_inst {
                        if !r_inst.has_mvar() {
                            self.mvar_assignments.insert(*id, r_inst);
                            changed = true;
                            any_solved = true;
                            continue;
                        }
                    }
                    if let Level::MVar(id) = &r_inst {
                        if !l_inst.has_mvar() {
                            self.mvar_assignments.insert(*id, l_inst);
                            changed = true;
                            any_solved = true;
                        }
                    }
                }
            }
        }
        any_solved
    }
    /// Get all constraints.
    pub fn all_constraints(&self) -> &[UnivConstraint] {
        &self.constraints
    }
    /// Get all universe variables.
    pub fn all_univ_vars(&self) -> &HashSet<Name> {
        &self.univ_vars
    }
    /// Clear all constraints and assignments.
    pub fn clear(&mut self) {
        self.constraints.clear();
        self.mvar_assignments.clear();
    }
    /// Check if a level has unassigned metavariables.
    pub fn has_unassigned_mvars(&self, l: &Level) -> bool {
        let inst = self.instantiate_mvars(l);
        inst.has_mvar()
    }
}
/// A mutable reference stack for tracking the current "focus" in a tree traversal.
#[allow(dead_code)]
pub struct FocusStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> FocusStack<T> {
    /// Creates an empty focus stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Focuses on `item`.
    pub fn focus(&mut self, item: T) {
        self.items.push(item);
    }
    /// Blurs (pops) the current focus.
    pub fn blur(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns the current focus, or `None`.
    pub fn current(&self) -> Option<&T> {
        self.items.last()
    }
    /// Returns the focus depth.
    pub fn depth(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if there is no current focus.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A pool of reusable string buffers.
#[allow(dead_code)]
pub struct StringPool {
    free: Vec<String>,
}
#[allow(dead_code)]
impl StringPool {
    /// Creates a new empty string pool.
    pub fn new() -> Self {
        Self { free: Vec::new() }
    }
    /// Takes a string from the pool (may be empty).
    pub fn take(&mut self) -> String {
        self.free.pop().unwrap_or_default()
    }
    /// Returns a string to the pool.
    pub fn give(&mut self, mut s: String) {
        s.clear();
        self.free.push(s);
    }
    /// Returns the number of free strings in the pool.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
}
/// A counter that can measure elapsed time between snapshots.
#[allow(dead_code)]
pub struct Stopwatch {
    start: std::time::Instant,
    splits: Vec<f64>,
}
#[allow(dead_code)]
impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            splits: Vec::new(),
        }
    }
    /// Records a split time (elapsed since start).
    pub fn split(&mut self) {
        self.splits.push(self.elapsed_ms());
    }
    /// Returns total elapsed milliseconds since start.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
    /// Returns all recorded split times.
    pub fn splits(&self) -> &[f64] {
        &self.splits
    }
    /// Returns the number of splits.
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }
}
/// A simple stack-based calculator for arithmetic expressions.
#[allow(dead_code)]
pub struct StackCalc {
    stack: Vec<i64>,
}
#[allow(dead_code)]
impl StackCalc {
    /// Creates a new empty calculator.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    /// Pushes an integer literal.
    pub fn push(&mut self, n: i64) {
        self.stack.push(n);
    }
    /// Adds the top two values.  Panics if fewer than two values.
    pub fn add(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        self.stack.push(a + b);
    }
    /// Subtracts top from second.
    pub fn sub(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        self.stack.push(a - b);
    }
    /// Multiplies the top two values.
    pub fn mul(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        self.stack.push(a * b);
    }
    /// Peeks the top value.
    pub fn peek(&self) -> Option<i64> {
        self.stack.last().copied()
    }
    /// Returns the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// A flat list of substitution pairs `(from, to)`.
#[allow(dead_code)]
pub struct FlatSubstitution {
    pairs: Vec<(String, String)>,
}
#[allow(dead_code)]
impl FlatSubstitution {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }
    /// Adds a pair.
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.pairs.push((from.into(), to.into()));
    }
    /// Applies all substitutions to `s` (leftmost-first order).
    pub fn apply(&self, s: &str) -> String {
        let mut result = s.to_string();
        for (from, to) in &self.pairs {
            result = result.replace(from.as_str(), to.as_str());
        }
        result
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// A hierarchical configuration tree.
#[allow(dead_code)]
pub struct ConfigNode {
    key: String,
    value: Option<String>,
    children: Vec<ConfigNode>,
}
#[allow(dead_code)]
impl ConfigNode {
    /// Creates a leaf config node with a value.
    pub fn leaf(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
            children: Vec::new(),
        }
    }
    /// Creates a section node with children.
    pub fn section(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
            children: Vec::new(),
        }
    }
    /// Adds a child node.
    pub fn add_child(&mut self, child: ConfigNode) {
        self.children.push(child);
    }
    /// Returns the key.
    pub fn key(&self) -> &str {
        &self.key
    }
    /// Returns the value, or `None` for section nodes.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Returns the number of children.
    pub fn num_children(&self) -> usize {
        self.children.len()
    }
    /// Looks up a dot-separated path.
    pub fn lookup(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
    fn lookup_relative(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
}
/// A simple decision tree node for rule dispatching.
#[allow(dead_code)]
#[allow(missing_docs)]
pub enum DecisionNode {
    /// A leaf with an action string.
    Leaf(String),
    /// An interior node: check `key` equals `val` → `yes_branch`, else `no_branch`.
    Branch {
        key: String,
        val: String,
        yes_branch: Box<DecisionNode>,
        no_branch: Box<DecisionNode>,
    },
}
#[allow(dead_code)]
impl DecisionNode {
    /// Evaluates the decision tree with the given context.
    pub fn evaluate(&self, ctx: &std::collections::HashMap<String, String>) -> &str {
        match self {
            DecisionNode::Leaf(action) => action.as_str(),
            DecisionNode::Branch {
                key,
                val,
                yes_branch,
                no_branch,
            } => {
                let actual = ctx.get(key).map(|s| s.as_str()).unwrap_or("");
                if actual == val.as_str() {
                    yes_branch.evaluate(ctx)
                } else {
                    no_branch.evaluate(ctx)
                }
            }
        }
    }
    /// Returns the depth of the decision tree.
    pub fn depth(&self) -> usize {
        match self {
            DecisionNode::Leaf(_) => 0,
            DecisionNode::Branch {
                yes_branch,
                no_branch,
                ..
            } => 1 + yes_branch.depth().max(no_branch.depth()),
        }
    }
}
/// A label set for a graph node.
#[allow(dead_code)]
pub struct LabelSet {
    labels: Vec<String>,
}
#[allow(dead_code)]
impl LabelSet {
    /// Creates a new empty label set.
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }
    /// Adds a label (deduplicates).
    pub fn add(&mut self, label: impl Into<String>) {
        let s = label.into();
        if !self.labels.contains(&s) {
            self.labels.push(s);
        }
    }
    /// Returns `true` if `label` is present.
    pub fn has(&self, label: &str) -> bool {
        self.labels.iter().any(|l| l == label)
    }
    /// Returns the count of labels.
    pub fn count(&self) -> usize {
        self.labels.len()
    }
    /// Returns all labels.
    pub fn all(&self) -> &[String] {
        &self.labels
    }
}
