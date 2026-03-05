//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Name};

use std::collections::HashMap;

/// Records the outcome of an eta-reduction attempt.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtaOutcome {
    /// The expression was already in eta-normal form.
    AlreadyNormal,
    /// The expression was reduced successfully.
    Reduced,
    /// The expression could not be reduced (not an eta-redex).
    NotApplicable,
    /// Reduction was aborted due to exceeding depth limit.
    DepthLimitExceeded,
}
#[allow(dead_code)]
impl EtaOutcome {
    /// Returns `true` if this represents a successful reduction.
    pub fn is_success(self) -> bool {
        matches!(self, EtaOutcome::Reduced | EtaOutcome::AlreadyNormal)
    }
    /// Returns a short human-readable string.
    pub fn label(self) -> &'static str {
        match self {
            EtaOutcome::AlreadyNormal => "already_normal",
            EtaOutcome::Reduced => "reduced",
            EtaOutcome::NotApplicable => "not_applicable",
            EtaOutcome::DepthLimitExceeded => "depth_limit",
        }
    }
}
/// Describes the structure of an expression for eta-expansion purposes.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EtaStructure {
    /// An expression that is not a lambda or application.
    Atomic,
    /// A lambda abstraction with arity `n`.
    Lambda(usize),
    /// An application spine with `n` arguments.
    App(usize),
    /// An expression suitable for eta-expansion.
    EtaExpandable,
}
#[allow(dead_code)]
impl EtaStructure {
    /// Returns the arity if this is a lambda or app.
    pub fn arity(&self) -> usize {
        match self {
            EtaStructure::Lambda(n) | EtaStructure::App(n) => *n,
            _ => 0,
        }
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
/// A rewrite rule: `lhs → rhs` (both closed expressions).
#[derive(Clone, Debug)]
pub struct RewriteRule {
    /// Left-hand side pattern.
    pub lhs: Expr,
    /// Right-hand side replacement.
    pub rhs: Expr,
    /// Human-readable name for the rule.
    pub name: String,
}
impl RewriteRule {
    /// Create a new rewrite rule.
    pub fn new(name: impl Into<String>, lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            name: name.into(),
        }
    }
    /// Apply the rule to `expr` if the top-level matches.
    pub fn apply_top(&self, expr: &Expr) -> Option<Expr> {
        if expr == &self.lhs {
            Some(self.rhs.clone())
        } else {
            None
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
/// A simple memoised eta-normal-form checker.
#[allow(dead_code)]
pub struct EtaChecker {
    cache: EtaNormalCache,
    log: EtaLog,
}
#[allow(dead_code)]
impl EtaChecker {
    /// Creates a new eta checker.
    pub fn new() -> Self {
        Self {
            cache: EtaNormalCache::new(),
            log: EtaLog::new(),
        }
    }
    /// Checks whether `hash` is eta-normal, using the cache.
    pub fn is_eta_normal(&mut self, hash: u64) -> EtaOutcome {
        if let Some(is_normal) = self.cache.query(hash) {
            let outcome = if is_normal {
                EtaOutcome::AlreadyNormal
            } else {
                EtaOutcome::NotApplicable
            };
            self.log.record(outcome);
            return outcome;
        }
        self.cache.insert(hash, false);
        self.log.record(EtaOutcome::NotApplicable);
        EtaOutcome::NotApplicable
    }
    /// Returns the success rate from the log.
    pub fn success_rate(&self) -> f64 {
        self.log.success_rate()
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
/// Represents a pending eta-reduction job.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EtaJob {
    /// Unique job ID.
    pub id: u64,
    /// Expression hash to reduce.
    pub hash: u64,
    /// Priority (higher = process first).
    pub prio: u32,
}
#[allow(dead_code)]
impl EtaJob {
    /// Creates a new eta reduction job.
    pub fn new(id: u64, hash: u64, prio: u32) -> Self {
        Self { id, hash, prio }
    }
}
/// Represents a rewrite rule `lhs → rhs`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EtaRewriteRule {
    /// The left-hand side pattern.
    pub lhs: String,
    /// The right-hand side template.
    pub rhs: String,
}
#[allow(dead_code)]
impl EtaRewriteRule {
    /// Creates a new rewrite rule.
    pub fn new(lhs: impl Into<String>, rhs: impl Into<String>) -> Self {
        Self {
            lhs: lhs.into(),
            rhs: rhs.into(),
        }
    }
}
/// Statistics about an expression's lambda structure.
#[derive(Debug, Default, Clone)]
pub struct LambdaStats {
    /// Total number of lambda binders.
    pub lambda_count: usize,
    /// Total number of pi binders.
    pub pi_count: usize,
    /// Total number of application nodes.
    pub app_count: usize,
    /// Total number of let binders.
    pub let_count: usize,
    /// Maximum nesting depth encountered.
    pub max_depth: usize,
}
impl LambdaStats {
    /// Compute statistics for an expression.
    pub fn compute(expr: &Expr) -> Self {
        let mut stats = Self::default();
        Self::walk(expr, &mut stats, 0);
        stats
    }
    fn walk(expr: &Expr, stats: &mut Self, depth: usize) {
        if depth > stats.max_depth {
            stats.max_depth = depth;
        }
        match expr {
            Expr::Lam(_, _, ty, body) => {
                stats.lambda_count += 1;
                Self::walk(ty, stats, depth + 1);
                Self::walk(body, stats, depth + 1);
            }
            Expr::Pi(_, _, ty, body) => {
                stats.pi_count += 1;
                Self::walk(ty, stats, depth + 1);
                Self::walk(body, stats, depth + 1);
            }
            Expr::App(f, a) => {
                stats.app_count += 1;
                Self::walk(f, stats, depth + 1);
                Self::walk(a, stats, depth + 1);
            }
            Expr::Let(_, ty, val, body) => {
                stats.let_count += 1;
                Self::walk(ty, stats, depth + 1);
                Self::walk(val, stats, depth + 1);
                Self::walk(body, stats, depth + 1);
            }
            _ => {}
        }
    }
    /// Total number of binders (lambda + pi + let).
    pub fn total_binders(&self) -> usize {
        self.lambda_count + self.pi_count + self.let_count
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
/// A cache mapping expression hashes to whether they are eta-normal.
#[allow(dead_code)]
pub struct EtaNormalCache {
    cache: std::collections::HashMap<u64, bool>,
}
#[allow(dead_code)]
impl EtaNormalCache {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Records that `hash` maps to `is_normal`.
    pub fn insert(&mut self, hash: u64, is_normal: bool) {
        self.cache.insert(hash, is_normal);
    }
    /// Returns `Some(is_normal)` if cached, `None` otherwise.
    pub fn query(&self, hash: u64) -> Option<bool> {
        self.cache.get(&hash).copied()
    }
    /// Invalidates all cached entries.
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Returns the number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
/// A type-erased function pointer with arity tracking.
#[allow(dead_code)]
pub struct RawFnPtr {
    /// The raw function pointer (stored as usize for type erasure).
    ptr: usize,
    pub(super) arity: usize,
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
/// Information about the eta-normal form of an expression.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EtaNormInfo {
    /// The eta-normalized expression.
    pub normalized: Expr,
    /// Number of eta contractions performed.
    pub contractions: usize,
    /// Whether the expression was already in eta-normal form.
    pub already_normal: bool,
}
impl EtaNormInfo {
    /// Create an info record for an expression that was already normal.
    pub fn already_normal(expr: Expr) -> Self {
        Self {
            normalized: expr,
            contractions: 0,
            already_normal: true,
        }
    }
    /// Create an info record for an expression that required contractions.
    pub fn contracted(expr: Expr, n: usize) -> Self {
        Self {
            normalized: expr,
            contractions: n,
            already_normal: false,
        }
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
/// Statistics tracking eta-reduction performance.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EtaReductionStats {
    /// Number of successful eta-reductions.
    pub reductions: u64,
    /// Number of expressions examined.
    pub examined: u64,
    /// Number of eta-expanded forms produced.
    pub expansions: u64,
    /// Maximum nesting depth encountered.
    pub max_depth: usize,
}
#[allow(dead_code)]
impl EtaReductionStats {
    /// Creates zeroed statistics.
    pub fn new() -> Self {
        Self {
            reductions: 0,
            examined: 0,
            expansions: 0,
            max_depth: 0,
        }
    }
    /// Returns the reduction ratio (reductions / examined).
    pub fn ratio(&self) -> f64 {
        if self.examined == 0 {
            return 0.0;
        }
        self.reductions as f64 / self.examined as f64
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
/// A priority queue for eta reduction jobs.
#[allow(dead_code)]
pub struct EtaJobQueue {
    jobs: Vec<EtaJob>,
}
#[allow(dead_code)]
impl EtaJobQueue {
    /// Creates an empty job queue.
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }
    /// Enqueues a job.
    pub fn enqueue(&mut self, job: EtaJob) {
        let pos = self.jobs.partition_point(|j| j.prio >= job.prio);
        self.jobs.insert(pos, job);
    }
    /// Dequeues the highest-priority job.
    pub fn dequeue(&mut self) -> Option<EtaJob> {
        if self.jobs.is_empty() {
            None
        } else {
            Some(self.jobs.remove(0))
        }
    }
    /// Returns the number of pending jobs.
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
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
/// A log of eta-reduction outcomes with timestamps.
#[allow(dead_code)]
pub struct EtaLog {
    entries: Vec<(std::time::Instant, EtaOutcome)>,
}
#[allow(dead_code)]
impl EtaLog {
    /// Creates an empty log.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Records an outcome.
    pub fn record(&mut self, outcome: EtaOutcome) {
        self.entries.push((std::time::Instant::now(), outcome));
    }
    /// Returns the number of recorded outcomes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the count of a specific outcome.
    pub fn count(&self, target: EtaOutcome) -> usize {
        self.entries.iter().filter(|(_, o)| *o == target).count()
    }
    /// Returns the success rate (0.0–1.0).
    pub fn success_rate(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        let successes = self.entries.iter().filter(|(_, o)| o.is_success()).count();
        successes as f64 / self.entries.len() as f64
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
/// A counter tracking min/max/sum for eta pass statistics.
#[allow(dead_code)]
pub struct EtaStatCounter {
    count: u64,
    sum: i64,
    min: i64,
    max: i64,
}
#[allow(dead_code)]
impl EtaStatCounter {
    /// Creates a new counter.
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0,
            min: i64::MAX,
            max: i64::MIN,
        }
    }
    /// Records a value.
    pub fn record(&mut self, v: i64) {
        self.count += 1;
        self.sum += v;
        if v < self.min {
            self.min = v;
        }
        if v > self.max {
            self.max = v;
        }
    }
    /// Returns the mean, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum as f64 / self.count as f64)
        }
    }
    /// Returns the count.
    pub fn count(&self) -> u64 {
        self.count
    }
}
/// A simple counter for tracking eta-related operations.
#[allow(dead_code)]
pub struct EtaOpCounter {
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl EtaOpCounter {
    /// Creates a new counter.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Increments the counter for `op`.
    pub fn inc(&mut self, op: &str) {
        *self.counts.entry(op.to_string()).or_insert(0) += 1;
    }
    /// Returns the count for `op`.
    pub fn get(&self, op: &str) -> u64 {
        self.counts.get(op).copied().unwrap_or(0)
    }
    /// Returns the total count.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
}
