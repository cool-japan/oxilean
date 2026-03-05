//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::{BinderInfo, Expr, FVarId, Name};
use std::collections::HashMap;

/// A saved snapshot of the context for backtracking.
#[derive(Debug, Clone)]
pub struct ContextSnapshot {
    /// Number of locals at snapshot time
    num_locals: usize,
    /// Next fvar id at snapshot time
    next_fvar: u64,
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
/// Local variable declaration in context.
#[derive(Debug, Clone)]
pub struct LocalVar {
    /// Variable name
    pub name: Name,
    /// Binder info (default, implicit, etc.)
    pub binder_info: BinderInfo,
    /// Variable type
    pub ty: Expr,
    /// Optional value (for let-bindings)
    pub val: Option<Expr>,
    /// Free variable ID
    pub fvar: FVarId,
    /// De Bruijn index (position in context, 0 = innermost)
    pub index: usize,
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
/// A multi-scope context that supports pushing and popping entire scopes.
///
/// Each scope contains a set of local variables; popping a scope removes all
/// locals introduced since the last push.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScopedContext {
    /// The base context.
    inner: Context,
    /// Stack of snapshots marking scope boundaries.
    scope_stack: Vec<ContextSnapshot>,
}
impl ScopedContext {
    /// Create a new empty scoped context.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: Context::new(),
            scope_stack: Vec::new(),
        }
    }
    /// Push a new scope.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scope_stack.push(self.inner.save());
    }
    /// Pop the current scope, removing all locals introduced since the last push.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        if let Some(snap) = self.scope_stack.pop() {
            self.inner.restore(&snap);
        }
    }
    /// Add a local in the current scope.
    #[allow(dead_code)]
    pub fn add_local(&mut self, name: Name, ty: Expr) -> FVarId {
        self.inner.push_local(name, ty, None)
    }
    /// Get a local by fvar id.
    #[allow(dead_code)]
    pub fn get_local(&self, fvar: FVarId) -> Option<&LocalVar> {
        self.inner.get_local(fvar)
    }
    /// Depth of the scope stack.
    #[allow(dead_code)]
    pub fn scope_depth(&self) -> usize {
        self.scope_stack.len()
    }
    /// Number of locals in the inner context.
    #[allow(dead_code)]
    pub fn num_locals(&self) -> usize {
        self.inner.num_locals()
    }
    /// Access the inner context.
    #[allow(dead_code)]
    pub fn inner(&self) -> &Context {
        &self.inner
    }
    /// Get all free variable expressions.
    #[allow(dead_code)]
    pub fn get_fvars(&self) -> Vec<Expr> {
        self.inner.get_fvars()
    }
}
/// Utility for generating sequences of fresh variable names in a context.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FreshNameSeq {
    base: String,
    counter: u64,
    used: Vec<String>,
}
impl FreshNameSeq {
    /// Create a fresh name sequence with the given base.
    #[allow(dead_code)]
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            counter: 0,
            used: Vec::new(),
        }
    }
    /// Generate the next fresh name.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Name {
        loop {
            let candidate = if self.counter == 0 {
                self.base.clone()
            } else {
                format!("{}_{}", self.base, self.counter)
            };
            self.counter += 1;
            if !self.used.contains(&candidate) {
                self.used.push(candidate.clone());
                return Name::str(&candidate);
            }
        }
    }
    /// Mark a name as used (won't be generated).
    #[allow(dead_code)]
    pub fn reserve(&mut self, name: &str) {
        if !self.used.contains(&name.to_string()) {
            self.used.push(name.to_string());
        }
    }
    /// Number of names generated so far.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.used.len()
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
/// A context that supports multiple named local hypotheses (proof state).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HypContext {
    inner: Context,
    /// Named hypotheses (name → fvar id).
    hyps: Vec<(Name, FVarId)>,
}
impl HypContext {
    /// Create an empty hypothesis context.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: Context::new(),
            hyps: Vec::new(),
        }
    }
    /// Add a hypothesis with the given name and type.
    #[allow(dead_code)]
    pub fn add_hyp(&mut self, name: Name, ty: Expr) -> FVarId {
        let fvar = self.inner.push_local(name.clone(), ty, None);
        self.hyps.push((name, fvar));
        fvar
    }
    /// Look up a hypothesis by name (searches from innermost).
    #[allow(dead_code)]
    pub fn find_hyp(&self, name: &Name) -> Option<FVarId> {
        self.hyps
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
    }
    /// Get the type of a hypothesis.
    #[allow(dead_code)]
    pub fn hyp_type(&self, fvar: FVarId) -> Option<&Expr> {
        self.inner.get_type(fvar)
    }
    /// Number of hypotheses.
    #[allow(dead_code)]
    pub fn num_hyps(&self) -> usize {
        self.hyps.len()
    }
    /// All hypothesis names.
    #[allow(dead_code)]
    pub fn hyp_names(&self) -> Vec<&Name> {
        self.hyps.iter().map(|(n, _)| n).collect()
    }
    /// Remove the last added hypothesis.
    #[allow(dead_code)]
    pub fn remove_last_hyp(&mut self) {
        if let Some((_, _)) = self.hyps.pop() {
            self.inner.pop_local();
        }
    }
    /// Clear all hypotheses.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.hyps.clear();
        self.inner.clear();
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
/// Type checking context.
#[derive(Debug, Clone)]
pub struct Context {
    /// Local variables (ordered from outermost to innermost)
    locals: Vec<LocalVar>,
    /// Mapping from free variable IDs to local variable indices
    fvar_map: HashMap<FVarId, usize>,
    /// Next fresh free variable ID
    next_fvar: u64,
}
impl Context {
    /// Create a new empty context.
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            fvar_map: HashMap::new(),
            next_fvar: 0,
        }
    }
    /// Create with a starting fvar id (to avoid conflicts).
    pub fn with_start_fvar(start: u64) -> Self {
        Self {
            locals: Vec::new(),
            fvar_map: HashMap::new(),
            next_fvar: start,
        }
    }
    /// Generate a fresh free variable ID.
    pub fn fresh_fvar(&mut self) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        fvar
    }
    /// Push a local variable declaration (without value).
    pub fn push_local(&mut self, name: Name, ty: Expr, val: Option<Expr>) -> FVarId {
        self.push_local_with_binder(name, BinderInfo::Default, ty, val)
    }
    /// Push a local variable declaration with binder info.
    pub fn push_local_with_binder(
        &mut self,
        name: Name,
        binder_info: BinderInfo,
        ty: Expr,
        val: Option<Expr>,
    ) -> FVarId {
        let fvar = self.fresh_fvar();
        let index = self.locals.len();
        self.locals.push(LocalVar {
            name,
            binder_info,
            ty,
            val,
            fvar,
            index,
        });
        self.fvar_map.insert(fvar, index);
        fvar
    }
    /// Create a local declaration and return the free variable expression.
    pub fn mk_local_decl(&mut self, name: Name, binder_info: BinderInfo, ty: Expr) -> Expr {
        let fvar = self.push_local_with_binder(name, binder_info, ty, None);
        Expr::FVar(fvar)
    }
    /// Create a let declaration and return the free variable expression.
    pub fn mk_let_decl(&mut self, name: Name, ty: Expr, val: Expr) -> Expr {
        let fvar = self.push_local(name, ty, Some(val));
        Expr::FVar(fvar)
    }
    /// Pop the last local variable from the context.
    pub fn pop_local(&mut self) -> Option<LocalVar> {
        if let Some(local) = self.locals.pop() {
            self.fvar_map.remove(&local.fvar);
            Some(local)
        } else {
            None
        }
    }
    /// Save the current state as a snapshot.
    pub fn save(&self) -> ContextSnapshot {
        ContextSnapshot {
            num_locals: self.locals.len(),
            next_fvar: self.next_fvar,
        }
    }
    /// Restore to a previous snapshot.
    pub fn restore(&mut self, snapshot: &ContextSnapshot) {
        while self.locals.len() > snapshot.num_locals {
            self.pop_local();
        }
        self.next_fvar = snapshot.next_fvar;
    }
    /// Get a local variable by free variable ID.
    pub fn get_local(&self, fvar: FVarId) -> Option<&LocalVar> {
        self.fvar_map
            .get(&fvar)
            .and_then(|&idx| self.locals.get(idx))
    }
    /// Get the type of a free variable.
    pub fn get_type(&self, fvar: FVarId) -> Option<&Expr> {
        self.get_local(fvar).map(|l| &l.ty)
    }
    /// Get the value of a let-bound free variable.
    pub fn get_value(&self, fvar: FVarId) -> Option<&Expr> {
        self.get_local(fvar).and_then(|l| l.val.as_ref())
    }
    /// Check if a free variable is a let-binding.
    pub fn is_let(&self, fvar: FVarId) -> bool {
        self.get_local(fvar).is_some_and(|l| l.val.is_some())
    }
    /// Get a local variable by name (searches from innermost to outermost).
    pub fn find_local(&self, name: &Name) -> Option<&LocalVar> {
        self.locals.iter().rev().find(|local| &local.name == name)
    }
    /// Get the number of local variables.
    pub fn num_locals(&self) -> usize {
        self.locals.len()
    }
    /// Check if the context is empty.
    pub fn is_empty(&self) -> bool {
        self.locals.is_empty()
    }
    /// Get all local variables.
    pub fn all_locals(&self) -> &[LocalVar] {
        &self.locals
    }
    /// Get all free variable expressions in the context.
    pub fn get_fvars(&self) -> Vec<Expr> {
        self.locals.iter().map(|l| Expr::FVar(l.fvar)).collect()
    }
    /// Build a lambda abstraction from the context's free variables.
    ///
    /// Given free variables `[x₁, x₂, ..., xₙ]` in the context,
    /// builds `λ (x₁ : τ₁) (x₂ : τ₂) ... (xₙ : τₙ), body`
    /// where body has all free variables replaced by bound variables.
    pub fn mk_lambda(&self, fvars: &[FVarId], body: Expr) -> Expr {
        let mut result = body;
        for &fvar in fvars.iter().rev() {
            if let Some(local) = self.get_local(fvar) {
                result = abstract_fvar(result, fvar);
                result = Expr::Lam(
                    local.binder_info,
                    local.name.clone(),
                    Box::new(abstract_fvars_in_type(local.ty.clone(), fvars, fvar)),
                    Box::new(result),
                );
            }
        }
        result
    }
    /// Build a Pi/forall type from the context's free variables.
    ///
    /// Given free variables `[x₁, x₂, ..., xₙ]`,
    /// builds `∀ (x₁ : τ₁) (x₂ : τ₂) ... (xₙ : τₙ), body`
    pub fn mk_pi(&self, fvars: &[FVarId], body: Expr) -> Expr {
        let mut result = body;
        for &fvar in fvars.iter().rev() {
            if let Some(local) = self.get_local(fvar) {
                result = abstract_fvar(result, fvar);
                result = Expr::Pi(
                    local.binder_info,
                    local.name.clone(),
                    Box::new(abstract_fvars_in_type(local.ty.clone(), fvars, fvar)),
                    Box::new(result),
                );
            }
        }
        result
    }
    /// Clear the context.
    pub fn clear(&mut self) {
        self.locals.clear();
        self.fvar_map.clear();
    }
    /// Execute a function with a temporary local declaration.
    ///
    /// The local is automatically popped when the function returns.
    pub fn with_local<F, R>(&mut self, name: Name, ty: Expr, f: F) -> R
    where
        F: FnOnce(&mut Self, FVarId) -> R,
    {
        let fvar = self.push_local(name, ty, None);
        let result = f(self, fvar);
        self.pop_local();
        result
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
/// A scoped context entry with type and optional value.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextEntry {
    /// Variable name.
    pub name: Name,
    /// Variable type.
    pub ty: Expr,
    /// Optional value (for let-bindings).
    pub val: Option<Expr>,
    /// Binder information.
    pub binder_info: BinderInfo,
}
impl ContextEntry {
    /// Create a simple local (no value).
    #[allow(dead_code)]
    pub fn local(name: Name, ty: Expr) -> Self {
        Self {
            name,
            ty,
            val: None,
            binder_info: BinderInfo::Default,
        }
    }
    /// Create an implicit local.
    #[allow(dead_code)]
    pub fn implicit(name: Name, ty: Expr) -> Self {
        Self {
            name,
            ty,
            val: None,
            binder_info: BinderInfo::Implicit,
        }
    }
    /// Create a let-binding.
    #[allow(dead_code)]
    pub fn let_binding(name: Name, ty: Expr, val: Expr) -> Self {
        Self {
            name,
            ty,
            val: Some(val),
            binder_info: BinderInfo::Default,
        }
    }
    /// Check if this is a let-binding.
    #[allow(dead_code)]
    pub fn is_let(&self) -> bool {
        self.val.is_some()
    }
    /// Check if this is implicit.
    #[allow(dead_code)]
    pub fn is_implicit(&self) -> bool {
        matches!(self.binder_info, BinderInfo::Implicit)
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
/// A flat sequence of context entries for serialization or display.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ContextChain {
    entries: Vec<ContextEntry>,
}
impl ContextChain {
    /// Create an empty chain.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push an entry.
    #[allow(dead_code)]
    pub fn push(&mut self, entry: ContextEntry) {
        self.entries.push(entry);
    }
    /// Pop the last entry.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<ContextEntry> {
        self.entries.pop()
    }
    /// Get the number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Get all entries.
    #[allow(dead_code)]
    pub fn entries(&self) -> &[ContextEntry] {
        &self.entries
    }
    /// Count let-bindings.
    #[allow(dead_code)]
    pub fn num_lets(&self) -> usize {
        self.entries.iter().filter(|e| e.is_let()).count()
    }
    /// Count implicit entries.
    #[allow(dead_code)]
    pub fn num_implicit(&self) -> usize {
        self.entries.iter().filter(|e| e.is_implicit()).count()
    }
    /// Find an entry by name (searches from innermost).
    #[allow(dead_code)]
    pub fn find(&self, name: &Name) -> Option<&ContextEntry> {
        self.entries.iter().rev().find(|e| &e.name == name)
    }
    /// Build from a `Context`.
    #[allow(dead_code)]
    pub fn from_context(ctx: &Context) -> Self {
        let mut chain = Self::new();
        for local in ctx.all_locals() {
            chain.push(ContextEntry {
                name: local.name.clone(),
                ty: local.ty.clone(),
                val: local.val.clone(),
                binder_info: local.binder_info,
            });
        }
        chain
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
/// A diff between two contexts.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ContextDiff {
    /// Entries present in the new context but not the old.
    pub added: Vec<Name>,
    /// Entries present in the old context but not the new.
    pub removed: Vec<Name>,
}
impl ContextDiff {
    /// Compute the diff between two contexts.
    #[allow(dead_code)]
    pub fn compute(old: &Context, new: &Context) -> Self {
        let old_names: std::collections::HashSet<&Name> =
            old.all_locals().iter().map(|l| &l.name).collect();
        let new_names: std::collections::HashSet<&Name> =
            new.all_locals().iter().map(|l| &l.name).collect();
        let added = new_names
            .difference(&old_names)
            .map(|&n| n.clone())
            .collect();
        let removed = old_names
            .difference(&new_names)
            .map(|&n| n.clone())
            .collect();
        Self { added, removed }
    }
    /// Check if the diff is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
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
/// A name generator for producing fresh variable names.
#[derive(Debug, Clone)]
pub struct NameGenerator {
    /// Prefix for generated names
    prefix: String,
    /// Next index
    next: u64,
}
impl NameGenerator {
    /// Create a new name generator with a given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            next: 0,
        }
    }
    /// Generate the next fresh name.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Name {
        let n = self.next;
        self.next += 1;
        Name::str(format!("{}_{}", self.prefix, n))
    }
    /// Generate the next fresh FVarId.
    pub fn next_fvar_id(&mut self) -> FVarId {
        let n = self.next;
        self.next += 1;
        FVarId(n)
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
/// Statistics for a context.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ContextStats {
    /// Total number of locals.
    pub num_locals: usize,
    /// Number of let-bindings.
    pub num_lets: usize,
    /// Number of implicit binders.
    pub num_implicit: usize,
    /// Maximum nesting depth reached.
    pub max_depth: usize,
}
impl ContextStats {
    /// Compute stats from a context.
    #[allow(dead_code)]
    pub fn from_context(ctx: &Context) -> Self {
        let locals = ctx.all_locals();
        let num_lets = locals.iter().filter(|l| l.val.is_some()).count();
        let num_implicit = locals
            .iter()
            .filter(|l| matches!(l.binder_info, BinderInfo::Implicit))
            .count();
        Self {
            num_locals: locals.len(),
            num_lets,
            num_implicit,
            max_depth: locals.len(),
        }
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
