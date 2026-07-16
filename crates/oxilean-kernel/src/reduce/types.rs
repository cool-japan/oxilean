//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::expr_util::{get_app_args, get_app_fn, mk_app};
use crate::instantiate::instantiate_type_lparams;
use crate::subst::instantiate;
use crate::Node;
use crate::{Environment, Expr, Literal, Name};
use std::collections::HashMap;
use std::rc::Rc;

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
/// A token bucket rate limiter.
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_per_ms: u64,
    last_refill: crate::wall_clock::Instant,
}
#[allow(dead_code)]
impl TokenBucket {
    /// Creates a new token bucket.
    pub fn new(capacity: u64, refill_per_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_ms,
            last_refill: crate::wall_clock::Instant::now(),
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
        let now = crate::wall_clock::Instant::now();
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
/// Named reduction rules for tracing.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReductionRule {
    /// Beta reduction: (λx.body) arg → body[arg/x].
    Beta,
    /// Delta reduction: unfold a definition.
    Delta,
    /// Zeta reduction: let x := v in body → body[v/x].
    Zeta,
    /// Iota reduction: recursor application.
    Iota,
    /// Projection reduction.
    Proj,
    /// Quotient reduction.
    Quot,
    /// Nat literal operation.
    NatLit,
    /// String literal operation.
    StrLit,
    /// No reduction was possible.
    None,
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
/// Statistics about reduction performance.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ReducerStats {
    /// Number of WHNF calls.
    pub whnf_calls: u64,
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of beta reductions performed.
    pub beta_count: u64,
    /// Number of delta reductions performed.
    pub delta_count: u64,
    /// Number of iota reductions performed.
    pub iota_count: u64,
    /// Number of zeta reductions performed.
    pub zeta_count: u64,
    /// Number of nat literal reductions.
    pub nat_lit_count: u64,
}
impl ReducerStats {
    /// Compute the total reduction count.
    #[allow(dead_code)]
    pub fn total_reductions(&self) -> u64 {
        self.beta_count + self.delta_count + self.iota_count + self.zeta_count + self.nat_lit_count
    }
    /// Cache hit rate as a fraction.
    #[allow(dead_code)]
    pub fn cache_hit_rate(&self) -> f64 {
        if self.whnf_calls == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.whnf_calls as f64
        }
    }
    /// Pretty-print the stats.
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!(
            "whnf:{} hits:{} β:{} δ:{} ι:{} ζ:{} lit:{}",
            self.whnf_calls,
            self.cache_hits,
            self.beta_count,
            self.delta_count,
            self.iota_count,
            self.zeta_count,
            self.nat_lit_count,
        )
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
/// Maximum AST size (in nodes) of a key or value retained in the WHNF
/// cache. Entries beyond this are recomputed instead of retained; results
/// are unchanged. See [`crate::expr_util::expr_size_within`].
pub(crate) const WHNF_CACHE_MAX_NODES: usize = 4096;
/// The outcome of one head-reduction step (see `Reducer::whnf_step`).
enum StepOutcome {
    /// The term is in weak head normal form.
    Done(Expr),
    /// One step of progress; the caller's loop continues on this term.
    Continue(Expr),
}
/// Reduction context with caching.
pub struct Reducer {
    /// WHNF cache: expr -> whnf(expr)
    pub(crate) cache: HashMap<Expr, Expr>,
    /// Maximum reduction depth (prevents infinite loops)
    max_depth: u32,
    /// Current transparency mode.
    transparency: TransparencyMode,
    /// Types of the free variables of the surrounding checker's local
    /// context (mirrored via [`Reducer::record_fvar_type`]). K-like iota
    /// reduction must *type* the recursor's major premise; a major that
    /// mentions local FVars (e.g. `Eq.symm … (h : ctorIdx t = 0)` inside
    /// `Std.IterStep.noConfusionType`) is only typeable with this context —
    /// Lean's kernel reads the same information from its local context.
    local_fvar_types: HashMap<u64, Expr>,
}
impl Reducer {
    /// Create a new reducer with default settings.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_depth: 10000,
            transparency: TransparencyMode::Default,
            local_fvar_types: HashMap::new(),
        }
    }
    /// Create a reducer with custom max depth.
    pub fn with_max_depth(max_depth: u32) -> Self {
        Self {
            cache: HashMap::new(),
            max_depth,
            transparency: TransparencyMode::Default,
            local_fvar_types: HashMap::new(),
        }
    }
    /// Record the type of a free variable opened by the surrounding checker
    /// so K-like iota reduction can type majors that mention it.
    ///
    /// `FVarId`s are dispensed monotonically and never reused, so recorded
    /// types are stable and cached WHNF results stay sound. Defensively, a
    /// conflicting re-bind invalidates the cache.
    pub fn record_fvar_type(&mut self, id: crate::FVarId, ty: Expr) {
        if let Some(prev) = self.local_fvar_types.get(&id.0) {
            if *prev != ty {
                self.cache.clear();
            }
        }
        self.local_fvar_types.insert(id.0, ty);
    }
    /// Set the transparency mode.
    pub fn set_transparency(&mut self, mode: TransparencyMode) {
        if self.transparency != mode {
            self.transparency = mode;
            self.cache.clear();
        }
    }
    /// Clear the cache (call between declarations).
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Reduce an expression to Weak Head Normal Form.
    ///
    /// This implements all reduction rules except delta (which needs environment).
    pub fn whnf(&mut self, expr: &Expr) -> Expr {
        self.whnf_with_depth(expr, 0)
    }
    fn whnf_with_depth(&mut self, expr: &Expr, depth: u32) -> Expr {
        if depth > self.max_depth {
            return expr.clone();
        }
        if let Some(cached) = self.cache.get(expr) {
            return cached.clone();
        }
        let result = match expr {
            Expr::Sort(_)
            | Expr::BVar(_)
            | Expr::FVar(_)
            | Expr::Const(_, _)
            | Expr::Lam(_, _, _, _)
            | Expr::Pi(_, _, _, _)
            | Expr::Lit(_) => expr.clone(),
            Expr::App(f, a) => {
                let f_whnf = self.whnf_with_depth(f, depth + 1);
                match f_whnf {
                    Expr::Lam(_, _, _, body) => {
                        let reduced = instantiate(&body, a);
                        self.whnf_with_depth(&reduced, depth + 1)
                    }
                    _ => Expr::App(Node::new(f_whnf), a.clone()),
                }
            }
            Expr::Let(_, _, val, body) => {
                let reduced = instantiate(body, val);
                self.whnf_with_depth(&reduced, depth + 1)
            }
            Expr::Proj(struct_name, idx, struct_expr) => {
                let struct_whnf = self.whnf_with_depth(struct_expr, depth + 1);
                Expr::Proj(struct_name.clone(), *idx, Node::new(struct_whnf))
            }
        };
        if crate::expr_util::expr_size_within(expr, WHNF_CACHE_MAX_NODES)
            && crate::expr_util::expr_size_within(&result, WHNF_CACHE_MAX_NODES)
        {
            self.cache.insert(expr.clone(), result.clone());
        }
        result
    }
    /// Reduce to WHNF with delta-reduction (constant unfolding).
    ///
    /// This requires environment access to look up definitions.
    pub fn whnf_delta<F>(&mut self, expr: &Expr, lookup: &F) -> Expr
    where
        F: Fn(&crate::Name) -> Option<(Expr, ReducibilityHint)>,
    {
        self.whnf_delta_with_depth(expr, lookup, 0)
    }
    fn whnf_delta_with_depth<F>(&mut self, expr: &Expr, lookup: &F, depth: u32) -> Expr
    where
        F: Fn(&crate::Name) -> Option<(Expr, ReducibilityHint)>,
    {
        if depth > self.max_depth {
            return expr.clone();
        }
        match expr {
            Expr::Sort(_)
            | Expr::BVar(_)
            | Expr::FVar(_)
            | Expr::Lam(_, _, _, _)
            | Expr::Pi(_, _, _, _)
            | Expr::Lit(_) => expr.clone(),
            Expr::Const(name, _levels) => {
                if let Some((defn, hint)) = lookup(name) {
                    if self.should_unfold_hint(hint) {
                        return self.whnf_delta_with_depth(&defn, lookup, depth + 1);
                    }
                }
                expr.clone()
            }
            Expr::App(f, a) => {
                let f_whnf = self.whnf_delta_with_depth(f, lookup, depth + 1);
                match f_whnf {
                    Expr::Lam(_, _, _, body) => {
                        let reduced = instantiate(&body, a);
                        self.whnf_delta_with_depth(&reduced, lookup, depth + 1)
                    }
                    _ => Expr::App(Node::new(f_whnf), a.clone()),
                }
            }
            Expr::Let(_, _, val, body) => {
                let reduced = instantiate(body, val);
                self.whnf_delta_with_depth(&reduced, lookup, depth + 1)
            }
            Expr::Proj(struct_name, idx, struct_expr) => {
                let struct_whnf = self.whnf_delta_with_depth(struct_expr, lookup, depth + 1);
                Expr::Proj(struct_name.clone(), *idx, Node::new(struct_whnf))
            }
        }
    }
    /// Check if a hint should be unfolded under current transparency.
    pub(crate) fn should_unfold_hint(&self, hint: ReducibilityHint) -> bool {
        match self.transparency {
            TransparencyMode::All => true,
            TransparencyMode::Default => hint.should_unfold(),
            TransparencyMode::Reducible => matches!(hint, ReducibilityHint::Abbrev),
            TransparencyMode::Instances => false,
            TransparencyMode::None => false,
        }
    }
    /// Full WHNF with environment-aware reductions.
    ///
    /// Performs: beta, zeta, delta, iota (recursor), projection, nat/string ops.
    pub fn whnf_env(&mut self, expr: &Expr, env: &Environment) -> Expr {
        self.whnf_env_depth(expr, env, 0)
    }
    fn whnf_env_depth(&mut self, expr: &Expr, env: &Environment, depth: u32) -> Expr {
        if depth > self.max_depth {
            return expr.clone();
        }
        // Resource-fuel degradation (C16): once the per-declaration budget
        // is exhausted, reduction stops making progress — a stuck term is
        // always sound, and it halts the term growth that exhausted the
        // budget. The caller reports the failure as a named resource limit.
        if crate::fuel::is_exhausted() {
            return expr.clone();
        }
        if let Some(cached) = self.cache.get(expr) {
            return cached.clone();
        }
        // The head-reduction chain runs as a LOOP, not recursion: each step
        // *moves* ownership of the intermediate term into the next step, so
        // exactly one intermediate is alive at a time. The old tail
        // recursion kept every intermediate of a long delta/iota chain
        // alive on the call stack simultaneously — the multi-GiB peak of
        // Lean core's `Int.add_mul_ediv_right` (C16) and a large share of
        // the deep-stack requirement (C17).
        let mut current: Option<Expr> = None;
        let mut steps = depth;
        let result = loop {
            let cur: &Expr = current.as_ref().unwrap_or(expr);
            if steps > self.max_depth || crate::fuel::is_exhausted() {
                break cur.clone();
            }
            match self.whnf_step(cur, env, steps) {
                StepOutcome::Done(done) => break done,
                StepOutcome::Continue(next) => {
                    steps += 1;
                    // Drop the previous intermediate before the next step.
                    current = Some(next);
                }
            }
        };
        // Retain only reasonably-sized entries: caching every multi-million
        // node intermediate of a long reduction retains O(steps × term) heap.
        // Skipping retention never changes a result — only memoisation.
        if crate::expr_util::expr_size_within(expr, WHNF_CACHE_MAX_NODES)
            && crate::expr_util::expr_size_within(&result, WHNF_CACHE_MAX_NODES)
        {
            self.cache.insert(expr.clone(), result.clone());
        }
        result
    }
    /// Perform ONE head-reduction step (zeta, delta, beta, literal folding,
    /// iota, quotient iota, projection). `StepOutcome::Continue` hands the
    /// reduct back to the caller's loop ([`Self::whnf_env_depth`]);
    /// `StepOutcome::Done` is the WHNF. Sub-terms (application heads,
    /// recursor majors, projection structs) are still reduced by recursive
    /// calls — only the head-reduction *chain* is iterative.
    fn whnf_step(&mut self, expr: &Expr, env: &Environment, depth: u32) -> StepOutcome {
        match expr {
            Expr::Sort(_)
            | Expr::BVar(_)
            | Expr::FVar(_)
            | Expr::Lam(_, _, _, _)
            | Expr::Pi(_, _, _, _)
            | Expr::Lit(_) => StepOutcome::Done(expr.clone()),
            Expr::Let(_, _, val, body) => StepOutcome::Continue(instantiate(body, val)),
            Expr::Const(name, levels) => {
                if let Some(ci) = env.find(name) {
                    if let Some(val) = ci.value() {
                        let hint = ci.reducibility_hint();
                        if self.should_unfold_hint(hint) {
                            let unfolded = if ci.level_params().is_empty() || levels.is_empty() {
                                val.clone()
                            } else {
                                instantiate_type_lparams(val, ci.level_params(), levels)
                            };
                            return StepOutcome::Continue(unfolded);
                        }
                    }
                }
                StepOutcome::Done(expr.clone())
            }
            Expr::App(_, _) => {
                // Keep the spine arguments BORROWED until a reduction rule
                // actually needs owned copies: cloning every argument up
                // front (and again in the final `mk_app` rebuild) doubled
                // the clone volume of every whnf step on large stuck spines
                // (C16, Lean core's `Int.add_mul_ediv_right`).
                let head = get_app_fn(expr);
                let args: Vec<&Expr> = get_app_args(expr);
                // The head is reduced ONE STEP AT A TIME while the
                // application context is kept, exactly like Lean's whnf loop
                // (whnf_core, then the literal extension, then
                // `unfold_definition` — which unfolds the head constant of
                // the WHOLE application and re-wraps the arguments). Every
                // iteration therefore re-checks the Nat/String literal
                // extension and iota on the current head, so a definition
                // chain that lands on `Nat.mod lit lit` (e.g. through
                // `HMod.hMod → instHMod → Mod.mod → Nat.instMod`) folds via
                // the extension instead of delta-unfolding `Nat.mod` into
                // its structural-recursion body. Reducing the bare head in
                // isolation (the old code) unfolded accelerated operations
                // before the extension could see the arguments — the
                // wrong-REJECT class of `UInt64.toUInt32_mul`,
                // `Int64.toInt_minValue` and the Omega constraint lemmas on
                // the Init corpus. The extension itself fires only at the
                // operation's exact arity, so an over-applied op is left
                // stuck and no argument is ever dropped; real exports
                // declare `Nat.ble`/`Nat.add`/… as ordinary definitions
                // (structural recursion over `Nat`), and the extension
                // computes exactly what those definitions compute.
                let mut head_owned: Option<Expr> = None;
                let mut head_steps = depth;
                loop {
                    if head_steps > self.max_depth || crate::fuel::is_exhausted() {
                        break;
                    }
                    head_steps += 1;
                    let head_ref: &Expr = head_owned.as_ref().unwrap_or(head);
                    match head_ref {
                        Expr::Lam(_, _, _, _) => {
                            // β: consume as many leading lambdas as args.
                            let mut result = head_ref.clone();
                            for arg in &args {
                                match result {
                                    Expr::Lam(_, _, _, body) => {
                                        result = instantiate(&body, arg);
                                    }
                                    _ => {
                                        result =
                                            Expr::App(Node::new(result), Node::new((*arg).clone()));
                                    }
                                }
                            }
                            return StepOutcome::Continue(result);
                        }
                        Expr::Const(name, levels) => {
                            // 1. Literal folding (Nat/String kernel
                            //    extension), before delta — the Lean order.
                            if let Some(arity) = lit_op_arity(&name.to_string()) {
                                if args.len() == arity {
                                    let args_whnf: Vec<Expr> = args
                                        .iter()
                                        .map(|a| self.whnf_env_depth(a, env, depth + 1))
                                        .collect();
                                    if let Some(reduced) = try_reduce_nat_app(head_ref, &args_whnf)
                                    {
                                        let reduced = respell_bool_const_for_env(reduced, env);
                                        return StepOutcome::Continue(reduced);
                                    }
                                }
                            }
                            // 2. Iota (recursor) and quotient reduction.
                            if let Some(reduced) =
                                self.try_reduce_recursor(name, levels, &args, env, depth)
                            {
                                return StepOutcome::Continue(reduced);
                            }
                            if let Some(reduced) = self.try_reduce_quot(name, &args, env, depth) {
                                return StepOutcome::Continue(reduced);
                            }
                            // 3. Delta: unfold the head IN CONTEXT and keep
                            //    stepping (the next iteration β-reduces).
                            if let Some(ci) = env.find(name) {
                                if let Some(val) = ci.value() {
                                    if self.should_unfold_hint(ci.reducibility_hint()) {
                                        let unfolded = if ci.level_params().is_empty()
                                            || levels.is_empty()
                                        {
                                            val.clone()
                                        } else {
                                            instantiate_type_lparams(val, ci.level_params(), levels)
                                        };
                                        head_owned = Some(unfolded);
                                        continue;
                                    }
                                }
                            }
                            break; // stuck constant head
                        }
                        // Let / Proj / FVar / Sort / Pi / Lit heads: step the
                        // head itself; `Done` means the head is in WHNF and
                        // the application is stuck.
                        _ => match self.whnf_step(head_ref, env, depth + 1) {
                            StepOutcome::Continue(h2) => {
                                head_owned = Some(h2);
                            }
                            StepOutcome::Done(h2) => {
                                head_owned = Some(h2);
                                break;
                            }
                        },
                    }
                }
                match head_owned {
                    Some(h) => StepOutcome::Done(crate::expr_util::mk_app_refs(h, &args)),
                    None => StepOutcome::Done(expr.clone()),
                }
            }
            Expr::Proj(struct_name, idx, struct_expr) => {
                let mut struct_whnf = self.whnf_env_depth(struct_expr, env, depth + 1);
                // Lean v4.32 `reduce_proj_core`: a string-literal struct is
                // expanded (`String.ofList l`, then WHNF'd into constructor
                // form) before the field is projected — e.g.
                // `Proj String 0 (StrLit s)` must reduce, or Lean core's
                // `String.toByteArray ""` lemmas go stuck and are wrongly
                // rejected.
                if let Expr::Lit(Literal::Str(s)) = &struct_whnf {
                    if let Some(e) = super::iota::str_lit_expansion(s, env) {
                        struct_whnf = self.whnf_env_depth(&e, env, depth + 1);
                    }
                }
                if let Some(reduced) = try_reduce_proj(struct_name, *idx, &struct_whnf, env) {
                    return StepOutcome::Continue(reduced);
                }
                StepOutcome::Done(Expr::Proj(
                    struct_name.clone(),
                    *idx,
                    Node::new(struct_whnf),
                ))
            }
        }
    }
    /// Try to reduce a recursor application (iota-reduction), following the
    /// Lean 4 kernel: K-like reduction first (when the recursor carries the
    /// K flag), then WHNF of the major premise, then `Nat`/`String` literal
    /// expansion, and finally rule application. Rule right-hand sides are
    /// closed lambdas applied to `params ++ motives ++ minors ++ fields`;
    /// arguments beyond the major premise are re-applied (over-application
    /// never drops arguments). See [`super::iota`].
    fn try_reduce_recursor(
        &mut self,
        rec_name: &Name,
        rec_levels: &[crate::Level],
        args: &[&Expr],
        env: &Environment,
        depth: u32,
    ) -> Option<Expr> {
        let rec_val = env.get_recursor_val(rec_name)?;
        let major_idx = rec_val.get_major_idx() as usize;
        if args.len() <= major_idx {
            return None;
        }
        // K-like reduction: replace the major premise by the canonical
        // constructor when its *type* has the right shape (Lean does this
        // before reducing the major itself).
        let mut major = if rec_val.k {
            super::iota::to_ctor_when_k(rec_val, args[major_idx], env, &self.local_fvar_types)
                .unwrap_or_else(|| args[major_idx].clone())
        } else {
            args[major_idx].clone()
        };
        major = self.whnf_env_depth(&major, env, depth + 1);
        // Literal-to-constructor expansion (one layer at a time for Nat), and
        // structure-eta expansion of stuck majors of structure-like inductives
        // (Lean's `toCtorWhenStruct`) on the fallthrough arm — same order as
        // lean4lean's `inductiveReduceRec`.
        match &major {
            Expr::Lit(Literal::Nat(n)) => {
                if let Some(e) = super::iota::nat_lit_to_ctor(n, rec_val) {
                    major = e;
                }
            }
            Expr::Lit(Literal::Str(s)) => {
                // Lean v4.32: `major = whnf(string_lit_to_constructor(major))`
                // — the expansion goes through the `String.ofList` *function*
                // (the constructor is `ofByteArray` since the UTF-8 String
                // refactor), so it must be WHNF'd into constructor form.
                if let Some(e) = super::iota::str_lit_expansion(s, env) {
                    major = self.whnf_env_depth(&e, env, depth + 1);
                }
            }
            _ => {
                if let Some(e) =
                    super::iota::to_ctor_when_struct(rec_val, &major, env, &self.local_fvar_types)
                {
                    major = e;
                }
            }
        }
        super::iota::apply_recursor_rule(rec_val, rec_levels, args, major_idx, &major, env)
    }
    /// Try to reduce a `Quot.lift` / `Quot.ind` application (quotient iota).
    ///
    /// This mirrors Lean's `quot.cpp` `quot_reduce_rec`:
    ///
    /// - `Quot.lift {α} {r} {β} (f : α → β) (h) (q : Quot r)` has `mk_pos = 5`
    ///   with the applied function `f` at position `3`. When the major premise
    ///   `q` reduces to `Quot.mk α r a`, the result is `f a`.
    /// - `Quot.ind {α} {r} {β} (m : ∀ a, β (Quot.mk r a)) (q : Quot r)` has
    ///   `mk_pos = 4` with the minor premise `m` at position `3`. When `q`
    ///   reduces to `Quot.mk α r a`, the result is `m a`.
    ///
    /// The quotiented element is always `mk_args[2]` of the fully-applied
    /// `Quot.mk` (`Quot.mk {α} (r) (a)` — three explicit-position arguments).
    ///
    /// Unlike a naive implementation, this:
    /// 1. WHNFs the major premise before inspecting its head, so a `q` that
    ///    only reduces to `Quot.mk` (e.g. `id (Quot.mk r a)`) still fires; and
    /// 2. re-applies any over-application arguments `args[mk_pos + 1 ..]` to
    ///    the result, so no argument is ever silently dropped (which would be
    ///    unsound on the def-eq path).
    fn try_reduce_quot(
        &mut self,
        name: &Name,
        args: &[&Expr],
        env: &Environment,
        depth: u32,
    ) -> Option<Expr> {
        use crate::declaration::QuotKind;
        let qv = env.get_quotient_val(name)?;
        // (arg_pos, mk_pos): position of the applied function / minor premise,
        // and position of the major premise (the `Quot.mk` value).
        let (arg_pos, mk_pos) = match qv.kind {
            QuotKind::Lift => (3usize, 5usize),
            QuotKind::Ind => (3usize, 4usize),
            QuotKind::Type | QuotKind::Mk => return None,
        };
        // The major premise must be present.
        if args.len() <= mk_pos {
            return None;
        }
        // WHNF the major premise so terms that only *reduce* to `Quot.mk`
        // still fire the computation rule.
        let major_whnf = self.whnf_env_depth(args[mk_pos], env, depth + 1);
        let mk_head = get_app_fn(&major_whnf);
        let mk_name = if let Expr::Const(n, _) = mk_head {
            n
        } else {
            return None;
        };
        // The head must be a registered `Quot.mk`.
        let mk_qv = env.get_quotient_val(mk_name)?;
        if mk_qv.kind != QuotKind::Mk {
            return None;
        }
        // `Quot.mk {α} (r) (a)` — the quotiented element is `mk_args[2]`.
        let mk_args = get_app_args(&major_whnf);
        if mk_args.len() < 3 {
            return None;
        }
        let a = mk_args[2];
        let f = args[arg_pos];
        let reduced = Expr::App(Node::new(f.clone()), Node::new(a.clone()));
        // Re-apply any over-application arguments so they are never dropped.
        Some(crate::expr_util::mk_app_refs(reduced, &args[mk_pos + 1..]))
    }
    /// Check if two expressions are alpha-equivalent (syntactically equal).
    pub fn is_alpha_equiv(e1: &Expr, e2: &Expr) -> bool {
        e1 == e2
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
/// A reduction trace, collecting applied reduction steps.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReductionTrace {
    steps: Vec<ReductionStep>,
    enabled: bool,
}
impl ReductionTrace {
    /// Create an enabled trace.
    #[allow(dead_code)]
    pub fn enabled() -> Self {
        Self {
            steps: Vec::new(),
            enabled: true,
        }
    }
    /// Create a disabled (no-op) trace.
    #[allow(dead_code)]
    pub fn disabled() -> Self {
        Self {
            steps: Vec::new(),
            enabled: false,
        }
    }
    /// Record a step if tracing is enabled.
    #[allow(dead_code)]
    pub fn record(&mut self, rule: ReductionRule, before: Expr, after: Expr) {
        if self.enabled {
            self.steps.push(ReductionStep {
                rule,
                before,
                after,
            });
        }
    }
    /// Return the number of recorded steps.
    #[allow(dead_code)]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
    /// Return all recorded steps.
    #[allow(dead_code)]
    pub fn steps(&self) -> &[ReductionStep] {
        &self.steps
    }
    /// Clear all recorded steps.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.steps.clear();
    }
    /// Count steps by rule kind.
    #[allow(dead_code)]
    pub fn count_rule(&self, rule: &ReductionRule) -> usize {
        self.steps.iter().filter(|s| &s.rule == rule).count()
    }
}
/// A counter that can measure elapsed time between snapshots.
#[allow(dead_code)]
pub struct Stopwatch {
    start: crate::wall_clock::Instant,
    splits: Vec<f64>,
}
#[allow(dead_code)]
impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn start() -> Self {
        Self {
            start: crate::wall_clock::Instant::now(),
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
/// Transparency mode controls which definitions can be unfolded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransparencyMode {
    /// All definitions can be unfolded.
    All,
    /// Default: unfold non-opaque definitions.
    Default,
    /// Only unfold definitions marked as reducible/abbrev.
    Reducible,
    /// Only unfold instance definitions.
    Instances,
    /// Never unfold anything.
    None,
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
/// A reduction step record used for debugging and tracing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReductionStep {
    /// Which reduction rule was applied.
    pub rule: ReductionRule,
    /// The expression before reduction.
    pub before: Expr,
    /// The expression after reduction.
    pub after: Expr,
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
/// Reducibility hint controls when definitions are unfolded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReducibilityHint {
    /// Never unfold (like theorem proofs)
    Opaque,
    /// Always unfold first (like notation)
    Abbrev,
    /// Unfold based on height (lower = unfold first)
    Regular(u32),
}
impl ReducibilityHint {
    /// Get the unfolding height (lower = unfold first)
    pub fn height(&self) -> u32 {
        match self {
            ReducibilityHint::Opaque => u32::MAX,
            ReducibilityHint::Abbrev => 0,
            ReducibilityHint::Regular(h) => *h,
        }
    }
    /// Should this definition be unfolded?
    pub fn should_unfold(&self) -> bool {
        !matches!(self, ReducibilityHint::Opaque)
    }
}
