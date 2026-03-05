//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::{Expr, Name};
use std::collections::HashMap;

/// A decision tree node.
#[derive(Debug, Clone)]
pub enum DecisionTree {
    /// Leaf: execute this arm's RHS
    Leaf {
        /// Index of the original arm
        arm_idx: usize,
        /// Bindings from pattern variables
        bindings: Vec<(Name, Expr)>,
    },
    /// Switch on a variable by constructor
    Switch {
        /// The scrutinee variable
        scrutinee: Expr,
        /// One branch per constructor
        branches: Vec<(Name, Vec<Name>, DecisionTree)>,
        /// Default branch (if not all constructors are listed)
        default: Option<Box<DecisionTree>>,
    },
    /// Failure: no match (should not occur in well-typed code)
    Failure,
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
/// Statistics about a compiled match expression.
#[derive(Debug, Clone)]
pub struct MatchStats {
    /// Number of arms in the original match.
    pub num_arms: usize,
    /// Indices of arms that are actually reachable.
    pub reachable_arms: Vec<usize>,
    /// Indices of arms that are unreachable (redundant).
    pub unreachable_arms: Vec<usize>,
    /// Whether the match is exhaustive (covers all cases).
    pub is_exhaustive: bool,
    /// Depth of the generated decision tree.
    pub tree_depth: usize,
}
impl MatchStats {
    /// Returns true if all arms are reachable.
    pub fn is_exhaustive(&self) -> bool {
        self.is_exhaustive
    }
    /// Returns true if there are any redundant (unreachable) arms.
    pub fn has_redundant_arms(&self) -> bool {
        !self.unreachable_arms.is_empty()
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
/// A match arm: pattern(s) → body.
#[derive(Debug, Clone)]
pub struct MatchArm {
    /// Patterns (one per scrutinee in multi-way match)
    pub patterns: Vec<Pattern>,
    /// Right-hand side
    pub rhs: Expr,
    /// Guard condition (if any)
    pub guard: Option<Expr>,
}
/// Result of compilation.
#[derive(Debug, Clone)]
pub struct CompileResult {
    /// The decision tree
    pub tree: DecisionTree,
    /// Indices of unreachable arms (redundancy)
    pub unreachable_arms: Vec<usize>,
    /// Missing constructor patterns (for exhaustiveness errors)
    pub missing_patterns: Vec<Vec<Pattern>>,
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
/// A pattern in a match expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard: matches anything
    Wildcard,
    /// Variable binding: matches anything, binds the value
    Var(Name),
    /// Constructor application: `C p1 p2 ...`
    Constructor(Name, Vec<Pattern>),
    /// Literal: matches a specific literal
    Literal(crate::Literal),
    /// As-pattern: `x @ p` (bind and match)
    As(Name, Box<Pattern>),
    /// Or-pattern: `p1 | p2`
    Or(Vec<Pattern>),
    /// Inaccessible pattern: `.( expr )`
    Inaccessible(Expr),
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
/// Constructor info needed for exhaustiveness checking.
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    /// Constructor name
    pub name: Name,
    /// Number of fields
    pub num_fields: u32,
    /// Parent inductive type
    pub inductive: Name,
}
/// Statistics about the compiled pattern match.
#[derive(Clone, Debug, Default)]
pub struct PatternStats {
    /// Total number of patterns in the match.
    pub total_patterns: usize,
    /// Number of wildcard patterns.
    pub wildcards: usize,
    /// Number of constructor patterns.
    pub constructors: usize,
    /// Number of literal patterns.
    pub literals: usize,
    /// Number of or-patterns.
    pub or_patterns: usize,
    /// Number of redundant arms detected.
    pub redundant_arms: usize,
}
impl PatternStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Collect stats from a list of patterns.
    pub fn from_patterns(patterns: &[Pattern]) -> Self {
        let mut stats = Self::new();
        for p in patterns {
            stats.total_patterns += 1;
            stats.count_pattern(p);
        }
        stats
    }
    fn count_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Wildcard => self.wildcards += 1,
            Pattern::Var(_) => self.wildcards += 1,
            Pattern::Constructor(_, sub) => {
                self.constructors += 1;
                for p in sub {
                    self.count_pattern(p);
                }
            }
            Pattern::Literal(_) => self.literals += 1,
            Pattern::As(_, inner) => self.count_pattern(inner),
            Pattern::Or(pats) => {
                self.or_patterns += 1;
                for p in pats {
                    self.count_pattern(p);
                }
            }
            Pattern::Inaccessible(_) => {}
        }
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
/// Pattern match compiler.
pub struct MatchCompiler {
    /// Counter for generating fresh variables
    next_var: u32,
    /// Known constructors for each inductive type
    constructors: HashMap<Name, Vec<ConstructorInfo>>,
}
impl MatchCompiler {
    /// Create a new match compiler.
    pub fn new() -> Self {
        Self {
            next_var: 0,
            constructors: HashMap::new(),
        }
    }
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> Name {
        let n = self.next_var;
        self.next_var += 1;
        Name::str(format!("_match{}", n))
    }
    /// Register constructors for an inductive type.
    pub fn register_constructors(&mut self, ind_name: Name, ctors: Vec<ConstructorInfo>) {
        self.constructors.insert(ind_name, ctors);
    }
    /// Get constructors for an inductive type.
    pub fn get_constructors(&self, ind_name: &Name) -> Option<&Vec<ConstructorInfo>> {
        self.constructors.get(ind_name)
    }
    /// Compile a match expression.
    ///
    /// Takes a list of scrutinees and match arms, produces a decision tree.
    pub fn compile_match(
        &mut self,
        scrutinees: &[Expr],
        arms: &[MatchArm],
    ) -> Result<CompileResult, String> {
        if arms.is_empty() {
            return Err("Match expression has no arms".to_string());
        }
        for (i, arm) in arms.iter().enumerate() {
            if arm.patterns.len() != scrutinees.len() {
                return Err(format!(
                    "Arm {} has {} patterns but expected {}",
                    i,
                    arm.patterns.len(),
                    scrutinees.len()
                ));
            }
        }
        let matrix: Vec<(Vec<&Pattern>, usize)> = arms
            .iter()
            .enumerate()
            .map(|(i, arm)| (arm.patterns.iter().collect(), i))
            .collect();
        let tree = self.compile_matrix(scrutinees, &matrix)?;
        let mut reachable = vec![false; arms.len()];
        self.mark_reachable(&tree, &mut reachable);
        let unreachable_arms: Vec<usize> = reachable
            .iter()
            .enumerate()
            .filter(|(_, &r)| !r)
            .map(|(i, _)| i)
            .collect();
        let missing_patterns = self.check_exhaustiveness_tree(&tree);
        Ok(CompileResult {
            tree,
            unreachable_arms,
            missing_patterns,
        })
    }
    /// Compile a pattern matrix into a decision tree.
    fn compile_matrix(
        &mut self,
        scrutinees: &[Expr],
        matrix: &[(Vec<&Pattern>, usize)],
    ) -> Result<DecisionTree, String> {
        if scrutinees.is_empty() {
            if let Some((_, arm_idx)) = matrix.first() {
                return Ok(DecisionTree::Leaf {
                    arm_idx: *arm_idx,
                    bindings: Vec::new(),
                });
            }
            return Ok(DecisionTree::Failure);
        }
        if matrix.is_empty() {
            return Ok(DecisionTree::Failure);
        }
        let col = self.find_best_column(matrix);
        let all_wildcards = matrix
            .iter()
            .all(|(pats, _)| matches!(pats[col], Pattern::Wildcard | Pattern::Var(_)));
        if all_wildcards {
            let (_, arm_idx) = &matrix[0];
            return Ok(DecisionTree::Leaf {
                arm_idx: *arm_idx,
                bindings: Vec::new(),
            });
        }
        let ctors_used: Vec<Name> = matrix
            .iter()
            .filter_map(|(pats, _)| {
                if let Pattern::Constructor(name, _) = &pats[col] {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        if ctors_used.is_empty() {
            let has_literals = matrix
                .iter()
                .any(|(pats, _)| matches!(pats[col], Pattern::Literal(_)));
            if has_literals {
                return self.compile_literal_column(scrutinees, matrix, col);
            }
            let (_, arm_idx) = &matrix[0];
            return Ok(DecisionTree::Leaf {
                arm_idx: *arm_idx,
                bindings: Vec::new(),
            });
        }
        let mut branches = Vec::new();
        let mut has_default = false;
        for ctor_name in &ctors_used {
            let ctor_arity = self.get_ctor_arity(ctor_name);
            let field_names: Vec<Name> = (0..ctor_arity).map(|_| self.fresh_var()).collect();
            let sub_matrix: Vec<(Vec<&Pattern>, usize)> = matrix
                .iter()
                .filter_map(|(pats, arm_idx)| match &pats[col] {
                    Pattern::Constructor(name, sub_pats) if name == ctor_name => {
                        let mut new_pats = Vec::new();
                        for (i, p) in pats.iter().enumerate() {
                            if i == col {
                                for sp in sub_pats {
                                    new_pats.push(sp);
                                }
                            } else {
                                new_pats.push(p);
                            }
                        }
                        Some((new_pats, *arm_idx))
                    }
                    Pattern::Wildcard | Pattern::Var(_) => {
                        let mut new_pats = Vec::new();
                        for (i, p) in pats.iter().enumerate() {
                            if i == col {
                                for _ in 0..ctor_arity {
                                    new_pats.push(&Pattern::Wildcard);
                                }
                            } else {
                                new_pats.push(p);
                            }
                        }
                        Some((new_pats, *arm_idx))
                    }
                    _ => None,
                })
                .collect();
            let mut sub_scrutinees = Vec::new();
            for (i, s) in scrutinees.iter().enumerate() {
                if i == col {
                    for (j, fname) in field_names.iter().enumerate() {
                        sub_scrutinees.push(Expr::Proj(
                            ctor_name.clone(),
                            j as u32,
                            Box::new(s.clone()),
                        ));
                        let _ = fname;
                    }
                } else {
                    sub_scrutinees.push(s.clone());
                }
            }
            let sub_tree = self.compile_matrix(&sub_scrutinees, &sub_matrix)?;
            branches.push((ctor_name.clone(), field_names, sub_tree));
        }
        let default_matrix: Vec<(Vec<&Pattern>, usize)> = matrix
            .iter()
            .filter_map(|(pats, arm_idx)| match &pats[col] {
                Pattern::Wildcard | Pattern::Var(_) => {
                    let mut new_pats: Vec<&Pattern> = pats
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != col)
                        .map(|(_, p)| *p)
                        .collect();
                    if new_pats.is_empty() && !pats.is_empty() {
                        new_pats.push(&Pattern::Wildcard);
                    }
                    Some((new_pats, *arm_idx))
                }
                _ => None,
            })
            .collect();
        let default = if !default_matrix.is_empty() {
            has_default = true;
            let remaining: Vec<Expr> = scrutinees
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != col)
                .map(|(_, s)| s.clone())
                .collect();
            Some(Box::new(self.compile_matrix(&remaining, &default_matrix)?))
        } else {
            None
        };
        let _ = has_default;
        Ok(DecisionTree::Switch {
            scrutinee: scrutinees[col].clone(),
            branches,
            default,
        })
    }
    /// Find the best column to split on.
    fn find_best_column(&self, matrix: &[(Vec<&Pattern>, usize)]) -> usize {
        if matrix.is_empty() || matrix[0].0.is_empty() {
            return 0;
        }
        let ncols = matrix[0].0.len();
        let mut best_col = 0;
        let mut best_score = 0;
        for col in 0..ncols {
            let mut score = 0;
            for (pats, _) in matrix {
                match pats[col] {
                    Pattern::Constructor(_, _) => score += 2,
                    Pattern::Literal(_) => score += 1,
                    _ => {}
                }
            }
            if score > best_score {
                best_score = score;
                best_col = col;
            }
        }
        best_col
    }
    /// Get constructor arity from registered info.
    fn get_ctor_arity(&self, ctor_name: &Name) -> u32 {
        for ctors in self.constructors.values() {
            for ctor in ctors {
                if &ctor.name == ctor_name {
                    return ctor.num_fields;
                }
            }
        }
        0
    }
    /// Compile a column with literal patterns.
    ///
    /// Builds a `Switch` node where each branch corresponds to a unique literal
    /// value found in `col`. Rows with wildcard/var patterns in `col` contribute
    /// to the default branch.
    fn compile_literal_column(
        &mut self,
        scrutinees: &[Expr],
        matrix: &[(Vec<&Pattern>, usize)],
        col: usize,
    ) -> Result<DecisionTree, String> {
        if matrix.is_empty() {
            return Ok(DecisionTree::Failure);
        }
        let mut seen_lits: Vec<crate::Literal> = Vec::new();
        for (pats, _) in matrix {
            if let Pattern::Literal(lit) = &pats[col] {
                if !seen_lits.contains(lit) {
                    seen_lits.push(lit.clone());
                }
            }
        }
        let sub_scrutinees: Vec<Expr> = scrutinees
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != col)
            .map(|(_, s)| s.clone())
            .collect();
        let mut branches = Vec::new();
        for lit in &seen_lits {
            let sub_matrix: Vec<(Vec<&Pattern>, usize)> = matrix
                .iter()
                .filter_map(|(pats, arm_idx)| match &pats[col] {
                    Pattern::Literal(l) if l == lit => {
                        let new_pats: Vec<&Pattern> = pats
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != col)
                            .map(|(_, p)| *p)
                            .collect();
                        Some((new_pats, *arm_idx))
                    }
                    Pattern::Wildcard | Pattern::Var(_) => {
                        let new_pats: Vec<&Pattern> = pats
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != col)
                            .map(|(_, p)| *p)
                            .collect();
                        Some((new_pats, *arm_idx))
                    }
                    _ => None,
                })
                .collect();
            let sub_tree = self.compile_matrix(&sub_scrutinees, &sub_matrix)?;
            let lit_name = Name::str(format!("{:?}", lit));
            branches.push((lit_name, Vec::new(), sub_tree));
        }
        let default_matrix: Vec<(Vec<&Pattern>, usize)> = matrix
            .iter()
            .filter_map(|(pats, arm_idx)| match &pats[col] {
                Pattern::Wildcard | Pattern::Var(_) => {
                    let new_pats: Vec<&Pattern> = pats
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != col)
                        .map(|(_, p)| *p)
                        .collect();
                    Some((new_pats, *arm_idx))
                }
                _ => None,
            })
            .collect();
        let default = if !default_matrix.is_empty() {
            Some(Box::new(
                self.compile_matrix(&sub_scrutinees, &default_matrix)?,
            ))
        } else {
            None
        };
        Ok(DecisionTree::Switch {
            scrutinee: scrutinees[col].clone(),
            branches,
            default,
        })
    }
    /// Mark reachable arms in a decision tree.
    fn mark_reachable(&self, tree: &DecisionTree, reachable: &mut [bool]) {
        match tree {
            DecisionTree::Leaf { arm_idx, .. } => {
                if *arm_idx < reachable.len() {
                    reachable[*arm_idx] = true;
                }
            }
            DecisionTree::Switch {
                branches, default, ..
            } => {
                for (_, _, sub_tree) in branches {
                    self.mark_reachable(sub_tree, reachable);
                }
                if let Some(d) = default {
                    self.mark_reachable(d, reachable);
                }
            }
            DecisionTree::Failure => {}
        }
    }
    /// Check exhaustiveness from the decision tree.
    fn check_exhaustiveness_tree(&self, tree: &DecisionTree) -> Vec<Vec<Pattern>> {
        match tree {
            DecisionTree::Failure => vec![vec![Pattern::Wildcard]],
            DecisionTree::Leaf { .. } => Vec::new(),
            DecisionTree::Switch {
                branches, default, ..
            } => {
                let mut missing = Vec::new();
                for (_, _, sub_tree) in branches {
                    missing.extend(self.check_exhaustiveness_tree(sub_tree));
                }
                if let Some(d) = default {
                    missing.extend(self.check_exhaustiveness_tree(d));
                }
                missing
            }
        }
    }
    /// Check if patterns are exhaustive.
    ///
    /// Handles Or-patterns (flattened), As-patterns (inner checked),
    /// Var/Wildcard (irrefutable), and constructor coverage.
    pub fn check_exhaustive(&self, patterns: &[Pattern], ind_name: &Name) -> Result<(), String> {
        let mut flat: Vec<&Pattern> = Vec::new();
        for pat in patterns {
            Self::collect_top_patterns(pat, &mut flat);
        }
        if flat.iter().any(|p| Self::is_irrefutable(p)) {
            return Ok(());
        }
        if let Some(ctors) = self.constructors.get(ind_name) {
            let mut covered: std::collections::HashSet<Name> = std::collections::HashSet::new();
            for pat in &flat {
                if let Pattern::Constructor(name, _) = pat {
                    covered.insert(name.clone());
                }
            }
            let missing: Vec<&Name> = ctors
                .iter()
                .filter(|c| !covered.contains(&c.name))
                .map(|c| &c.name)
                .collect();
            if missing.is_empty() {
                Ok(())
            } else {
                Err(format!(
                    "Non-exhaustive patterns: missing constructors {:?}",
                    missing
                ))
            }
        } else {
            Ok(())
        }
    }
    /// Check if any patterns are redundant.
    ///
    /// A pattern is redundant if it is preceded by a wildcard/var, or if
    /// a constructor or literal it matches was already covered earlier
    /// (including via Or-patterns).
    pub fn check_redundant(&self, patterns: &[Pattern]) -> Vec<usize> {
        let mut seen_wildcard = false;
        let mut seen_ctors: std::collections::HashSet<Name> = std::collections::HashSet::new();
        let mut seen_lits: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut redundant = Vec::new();
        for (i, pat) in patterns.iter().enumerate() {
            if seen_wildcard {
                redundant.push(i);
                continue;
            }
            if Self::is_covered_by(pat, &seen_ctors, &seen_lits) {
                redundant.push(i);
            }
            Self::record_pattern(pat, &mut seen_wildcard, &mut seen_ctors, &mut seen_lits);
        }
        redundant
    }
    /// Flatten Or/As wrappers, collecting concrete patterns.
    fn collect_top_patterns<'a>(pat: &'a Pattern, out: &mut Vec<&'a Pattern>) {
        match pat {
            Pattern::Or(pats) => {
                for p in pats {
                    Self::collect_top_patterns(p, out);
                }
            }
            Pattern::As(_, inner) => Self::collect_top_patterns(inner, out),
            _ => out.push(pat),
        }
    }
    /// Return `true` if the pattern is irrefutable (matches everything).
    fn is_irrefutable(pat: &Pattern) -> bool {
        match pat {
            Pattern::Wildcard | Pattern::Var(_) => true,
            Pattern::As(_, inner) => Self::is_irrefutable(inner),
            Pattern::Or(pats) => pats.iter().any(Self::is_irrefutable),
            _ => false,
        }
    }
    /// Check if `pat` is completely covered by what we've seen so far.
    fn is_covered_by(
        pat: &Pattern,
        seen_ctors: &std::collections::HashSet<Name>,
        seen_lits: &std::collections::HashSet<String>,
    ) -> bool {
        match pat {
            Pattern::Constructor(name, _) => seen_ctors.contains(name),
            Pattern::Literal(lit) => seen_lits.contains(&format!("{:?}", lit)),
            Pattern::As(_, inner) => Self::is_covered_by(inner, seen_ctors, seen_lits),
            Pattern::Or(pats) => pats
                .iter()
                .all(|p| Self::is_covered_by(p, seen_ctors, seen_lits)),
            Pattern::Wildcard | Pattern::Var(_) | Pattern::Inaccessible(_) => false,
        }
    }
    /// Record the patterns introduced by `pat` into the tracking sets.
    fn record_pattern(
        pat: &Pattern,
        seen_wildcard: &mut bool,
        seen_ctors: &mut std::collections::HashSet<Name>,
        seen_lits: &mut std::collections::HashSet<String>,
    ) {
        match pat {
            Pattern::Wildcard | Pattern::Var(_) => *seen_wildcard = true,
            Pattern::As(_, inner) => {
                Self::record_pattern(inner, seen_wildcard, seen_ctors, seen_lits)
            }
            Pattern::Constructor(name, _) => {
                seen_ctors.insert(name.clone());
            }
            Pattern::Literal(lit) => {
                seen_lits.insert(format!("{:?}", lit));
            }
            Pattern::Or(pats) => {
                for p in pats {
                    Self::record_pattern(p, seen_wildcard, seen_ctors, seen_lits);
                }
            }
            Pattern::Inaccessible(_) => {}
        }
    }
}
