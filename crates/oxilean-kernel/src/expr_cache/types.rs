//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Level, Literal, Name};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
/// Unique identifier for an interned expression.
///
/// Two expressions with the same `ExprId` are guaranteed to be structurally
/// identical (hash-consing invariant).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExprId(pub(crate) u32);
impl ExprId {
    /// Return the underlying u32 value.
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
/// Hash-consing table for `Expr` values.
///
/// Every unique expression (by structural equality) is assigned a stable
/// `ExprId`. Subsequent `intern` calls for the same expression return the
/// same `ExprId` and increment `hit_count`.
pub struct ExprHashcons {
    /// All interned expressions, indexed by `ExprId.0`.
    id_to_expr: Vec<Expr>,
    /// Mapping from structural expression key to its assigned `ExprId`.
    expr_to_id: HashMap<ExprKey, ExprId>,
    /// Number of times `intern` returned an existing ID.
    hit_count: u64,
    /// Number of times `intern` created a new entry.
    miss_count: u64,
}
impl ExprHashcons {
    /// Create a new, empty hash-consing table.
    pub fn new() -> Self {
        ExprHashcons {
            id_to_expr: Vec::new(),
            expr_to_id: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
        }
    }
    /// Intern an expression.
    ///
    /// Returns `(id, was_new)` where `was_new` is `true` if this expression
    /// had never been interned before.
    pub fn intern(&mut self, expr: Expr) -> (ExprId, bool) {
        let key = ExprKey(expr.clone());
        if let Some(&id) = self.expr_to_id.get(&key) {
            self.hit_count += 1;
            (id, false)
        } else {
            let id = ExprId(self.id_to_expr.len() as u32);
            self.id_to_expr.push(expr);
            self.expr_to_id.insert(key, id);
            self.miss_count += 1;
            (id, true)
        }
    }
    /// Look up an expression by `ExprId`.
    ///
    /// Returns `None` if the ID is out of range (should not happen for
    /// IDs produced by this table).
    pub fn get(&self, id: ExprId) -> Option<&Expr> {
        self.id_to_expr.get(id.0 as usize)
    }
    /// Find the `ExprId` for a structurally equal expression, if it exists.
    ///
    /// Does **not** update hit/miss counters.
    pub fn get_id(&self, expr: &Expr) -> Option<ExprId> {
        let key = ExprKey(expr.clone());
        self.expr_to_id.get(&key).copied()
    }
    /// Return the number of distinct expressions currently interned.
    pub fn size(&self) -> usize {
        self.id_to_expr.len()
    }
    /// Compute the cache hit rate as a value in [0.0, 1.0].
    ///
    /// Returns `0.0` if no `intern` calls have been made.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    /// Return a formatted statistics string.
    pub fn stats(&self) -> String {
        format!(
            "ExprHashcons {{ size: {}, hits: {}, misses: {}, hit_rate: {:.2}% }}",
            self.size(),
            self.hit_count,
            self.miss_count,
            self.hit_rate() * 100.0,
        )
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
/// A small fixed-size set implemented as a bit array.
#[allow(dead_code)]
pub struct BitSet64 {
    bits: u64,
}
#[allow(dead_code)]
impl BitSet64 {
    /// Creates an empty set.
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    /// Inserts element `i` (0–63).
    pub fn insert(&mut self, i: u32) {
        if i < 64 {
            self.bits |= 1u64 << i;
        }
    }
    /// Removes element `i`.
    pub fn remove(&mut self, i: u32) {
        if i < 64 {
            self.bits &= !(1u64 << i);
        }
    }
    /// Returns `true` if `i` is in the set.
    pub fn contains(&self, i: u32) -> bool {
        i < 64 && (self.bits >> i) & 1 != 0
    }
    /// Returns the cardinality.
    pub fn len(&self) -> u32 {
        self.bits.count_ones()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }
    /// Returns the union with `other`.
    pub fn union(self, other: BitSet64) -> BitSet64 {
        BitSet64 {
            bits: self.bits | other.bits,
        }
    }
    /// Returns the intersection with `other`.
    pub fn intersect(self, other: BitSet64) -> BitSet64 {
        BitSet64 {
            bits: self.bits & other.bits,
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
/// Statistics about a cache lookup session.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CacheSessionStats {
    pub hits: u64,
    pub misses: u64,
    pub insertions: u64,
    pub evictions: u64,
}
#[allow(dead_code)]
impl CacheSessionStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        CacheSessionStats::default()
    }
    /// Return the hit rate as a fraction in \[0,1\].
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            1.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Merge another stats struct into this one.
    pub fn merge(&mut self, other: &CacheSessionStats) {
        self.hits += other.hits;
        self.misses += other.misses;
        self.insertions += other.insertions;
        self.evictions += other.evictions;
    }
    /// Reset to zero.
    pub fn reset(&mut self) {
        *self = CacheSessionStats::default();
    }
    /// Format a one-line summary.
    pub fn summary(&self) -> String {
        format!(
            "hits={} misses={} insertions={} evictions={} hit_rate={:.1}%",
            self.hits,
            self.misses,
            self.insertions,
            self.evictions,
            self.hit_rate() * 100.0
        )
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
/// A min-heap implemented as a binary heap.
#[allow(dead_code)]
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}
#[allow(dead_code)]
impl<T: Ord> MinHeap<T> {
    /// Creates a new empty min-heap.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Inserts an element.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }
    /// Removes and returns the minimum element.
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }
    /// Returns a reference to the minimum element.
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.data.swap(i, smallest);
            i = smallest;
        }
    }
}
/// A trie-based prefix counter.
#[allow(dead_code)]
pub struct PrefixCounter {
    children: std::collections::HashMap<char, PrefixCounter>,
    count: usize,
}
#[allow(dead_code)]
impl PrefixCounter {
    /// Creates an empty prefix counter.
    pub fn new() -> Self {
        Self {
            children: std::collections::HashMap::new(),
            count: 0,
        }
    }
    /// Records a string.
    pub fn record(&mut self, s: &str) {
        self.count += 1;
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
            node.count += 1;
        }
    }
    /// Returns how many strings have been recorded that start with `prefix`.
    pub fn count_with_prefix(&self, prefix: &str) -> usize {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return 0,
            }
        }
        node.count
    }
}
/// A two-level cache: a small hot cache backed by a larger cold cache.
#[allow(dead_code)]
pub struct TwoLevelCache {
    hot: MemoTable,
    cold: MemoTable,
    hot_capacity: usize,
    stats: CacheSessionStats,
}
#[allow(dead_code)]
impl TwoLevelCache {
    /// Create a two-level cache with the given hot capacity.
    pub fn new(hot_capacity: usize) -> Self {
        TwoLevelCache {
            hot: MemoTable::new(),
            cold: MemoTable::new(),
            hot_capacity,
            stats: CacheSessionStats::new(),
        }
    }
    /// Look up a key; checks hot first, then cold (and promotes on hit).
    pub fn get(&mut self, key: u64) -> Option<u64> {
        if let Some(v) = self.hot.get(key) {
            self.stats.hits += 1;
            return Some(v);
        }
        if let Some(v) = self.cold.get(key) {
            self.stats.hits += 1;
            self.promote(key, v);
            return Some(v);
        }
        self.stats.misses += 1;
        None
    }
    /// Insert a value; placed in hot cache (evicts to cold if full).
    pub fn insert(&mut self, key: u64, val: u64) {
        if self.hot.len() >= self.hot_capacity {
            if let Some((k, v)) = self.hot.entries.first().copied() {
                self.hot.remove(k);
                self.cold.insert(k, v);
                self.stats.evictions += 1;
            }
        }
        self.hot.insert(key, val);
        self.stats.insertions += 1;
    }
    fn promote(&mut self, key: u64, val: u64) {
        self.cold.remove(key);
        self.insert(key, val);
    }
    /// Return a reference to the current session stats.
    pub fn stats(&self) -> &CacheSessionStats {
        &self.stats
    }
    /// Return total number of entries (hot + cold).
    pub fn total_len(&self) -> usize {
        self.hot.len() + self.cold.len()
    }
    /// Clear both levels.
    pub fn clear(&mut self) {
        self.hot.clear();
        self.cold.clear();
        self.stats.reset();
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
#[allow(dead_code)]
struct PathNode {
    key: u32,
    value: Option<u64>,
    children: Vec<usize>,
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
/// A simple mutable key-value store for test fixtures.
#[allow(dead_code)]
pub struct Fixture {
    data: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl Fixture {
    /// Creates an empty fixture.
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    /// Sets a key.
    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.data.insert(key.into(), val.into());
    }
    /// Gets a value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
/// A cache invalidation set tracking which expression ids to evict.
#[allow(dead_code)]
pub struct InvalidationSet {
    ids: Vec<u64>,
}
#[allow(dead_code)]
impl InvalidationSet {
    /// Create an empty set.
    pub fn new() -> Self {
        InvalidationSet { ids: Vec::new() }
    }
    /// Add an id to invalidate.
    pub fn add(&mut self, id: u64) {
        if !self.ids.contains(&id) {
            self.ids.push(id);
        }
    }
    /// Return whether the set contains the given id.
    pub fn contains(&self, id: u64) -> bool {
        self.ids.contains(&id)
    }
    /// Return all ids to invalidate.
    pub fn ids(&self) -> &[u64] {
        &self.ids
    }
    /// Clear the set.
    pub fn clear(&mut self) {
        self.ids.clear();
    }
    /// Return the number of ids.
    pub fn len(&self) -> usize {
        self.ids.len()
    }
    /// Return whether empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}
/// A reference-counted expression pool that garbage-collects dead entries.
#[allow(dead_code)]
pub struct RcExprPool {
    entries: Vec<SharedCacheEntry>,
}
#[allow(dead_code)]
impl RcExprPool {
    /// Create an empty pool.
    pub fn new() -> Self {
        RcExprPool {
            entries: Vec::new(),
        }
    }
    /// Allocate a new entry and return its pool index.
    pub fn alloc(&mut self, hash: u64) -> usize {
        let id = self.entries.len() as u64;
        self.entries.push(SharedCacheEntry::new(id, hash));
        self.entries.len() - 1
    }
    /// Increment ref count for the entry at `idx`.
    pub fn inc_ref(&mut self, idx: usize) {
        if let Some(e) = self.entries.get_mut(idx) {
            e.inc_ref();
        }
    }
    /// Decrement ref count; returns true if still alive.
    pub fn dec_ref(&mut self, idx: usize) -> bool {
        if let Some(e) = self.entries.get_mut(idx) {
            e.dec_ref()
        } else {
            false
        }
    }
    /// Collect all dead entries and return their indices.
    pub fn collect_garbage(&mut self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_dead())
            .map(|(i, _)| i)
            .collect()
    }
    /// Return the total number of entries (including dead).
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return number of live entries.
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.is_dead()).count()
    }
}
/// A simple LRU-like eviction policy tracker for the expression cache.
#[allow(dead_code)]
pub struct EvictionTracker {
    capacity: usize,
    order: Vec<u64>,
}
#[allow(dead_code)]
impl EvictionTracker {
    /// Create an eviction tracker with a given capacity.
    pub fn new(capacity: usize) -> Self {
        EvictionTracker {
            capacity,
            order: Vec::with_capacity(capacity),
        }
    }
    /// Record an access for the given id; promotes it to most-recent.
    pub fn access(&mut self, id: u64) {
        if let Some(pos) = self.order.iter().position(|&x| x == id) {
            self.order.remove(pos);
        }
        self.order.push(id);
        while self.order.len() > self.capacity {
            self.order.remove(0);
        }
    }
    /// Return the least-recently-used id, if any.
    pub fn lru(&self) -> Option<u64> {
        self.order.first().copied()
    }
    /// Return the most-recently-used id, if any.
    pub fn mru(&self) -> Option<u64> {
        self.order.last().copied()
    }
    /// Return how many entries are tracked.
    pub fn len(&self) -> usize {
        self.order.len()
    }
    /// Return whether tracker is empty.
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    /// Evict and return the LRU entry.
    pub fn evict_lru(&mut self) -> Option<u64> {
        if self.order.is_empty() {
            None
        } else {
            Some(self.order.remove(0))
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
/// A versioned cache that associates a monotonic version number with each entry.
#[allow(dead_code)]
pub struct VersionedCache {
    entries: Vec<(u64, u64, u64)>,
    current_version: u64,
}
#[allow(dead_code)]
impl VersionedCache {
    /// Create a new versioned cache.
    pub fn new() -> Self {
        VersionedCache {
            entries: Vec::new(),
            current_version: 0,
        }
    }
    /// Bump the version (e.g., after a global environment change).
    pub fn bump_version(&mut self) {
        self.current_version += 1;
    }
    /// Insert with the current version.
    pub fn insert(&mut self, key: u64, val: u64) {
        let ver = self.current_version;
        if let Some(e) = self.entries.iter_mut().find(|(k, _, _)| *k == key) {
            e.1 = val;
            e.2 = ver;
        } else {
            self.entries.push((key, val, ver));
        }
    }
    /// Look up a value; returns None if the entry's version is stale.
    pub fn get(&self, key: u64) -> Option<u64> {
        self.entries
            .iter()
            .find(|(k, _, v)| *k == key && *v == self.current_version)
            .map(|(_, val, _)| *val)
    }
    /// Return the current version number.
    pub fn version(&self) -> u64 {
        self.current_version
    }
    /// Evict all entries from a previous version.
    pub fn evict_stale(&mut self) {
        let cur = self.current_version;
        self.entries.retain(|(_, _, v)| *v == cur);
    }
    /// Return the number of valid (current-version) entries.
    pub fn valid_count(&self) -> usize {
        let cur = self.current_version;
        self.entries.iter().filter(|(_, _, v)| *v == cur).count()
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
/// A counter for tracking how many items are in each of `N` buckets.
#[allow(dead_code)]
pub struct BucketCounter<const N: usize> {
    buckets: [u64; N],
}
#[allow(dead_code)]
impl<const N: usize> BucketCounter<N> {
    /// Creates a zeroed bucket counter.
    pub const fn new() -> Self {
        Self { buckets: [0u64; N] }
    }
    /// Increments bucket `i`.
    pub fn inc(&mut self, i: usize) {
        if i < N {
            self.buckets[i] += 1;
        }
    }
    /// Returns the count for bucket `i`.
    pub fn get(&self, i: usize) -> u64 {
        if i < N {
            self.buckets[i]
        } else {
            0
        }
    }
    /// Returns the total count across all buckets.
    pub fn total(&self) -> u64 {
        self.buckets.iter().sum()
    }
    /// Returns the index of the bucket with the highest count.
    pub fn argmax(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0)
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
/// A trie-style path cache that maps expression path hashes to node ids.
#[allow(dead_code)]
pub struct PathCache {
    nodes: Vec<PathNode>,
}
#[allow(dead_code)]
impl PathCache {
    /// Create an empty path cache.
    pub fn new() -> Self {
        PathCache {
            nodes: vec![PathNode {
                key: 0,
                value: None,
                children: Vec::new(),
            }],
        }
    }
    /// Insert a path (sequence of u32 keys) mapping to a u64 id.
    pub fn insert(&mut self, path: &[u32], id: u64) {
        let mut node_idx = 0;
        for &step in path {
            let child_idx = self.nodes[node_idx]
                .children
                .iter()
                .copied()
                .find(|&c| self.nodes[c].key == step);
            match child_idx {
                Some(ci) => node_idx = ci,
                None => {
                    let new_idx = self.nodes.len();
                    self.nodes.push(PathNode {
                        key: step,
                        value: None,
                        children: Vec::new(),
                    });
                    self.nodes[node_idx].children.push(new_idx);
                    node_idx = new_idx;
                }
            }
        }
        self.nodes[node_idx].value = Some(id);
    }
    /// Look up a path.
    pub fn get(&self, path: &[u32]) -> Option<u64> {
        let mut node_idx = 0;
        for &step in path {
            let child_idx = self.nodes[node_idx]
                .children
                .iter()
                .copied()
                .find(|&c| self.nodes[c].key == step)?;
            node_idx = child_idx;
        }
        self.nodes[node_idx].value
    }
    /// Return total number of nodes in the trie.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
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
/// A shared reference-counted expression cache entry.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SharedCacheEntry {
    pub id: u64,
    pub hash: u64,
    pub ref_count: u32,
}
#[allow(dead_code)]
impl SharedCacheEntry {
    /// Create a new entry with ref count 1.
    pub fn new(id: u64, hash: u64) -> Self {
        SharedCacheEntry {
            id,
            hash,
            ref_count: 1,
        }
    }
    /// Increment the reference count.
    pub fn inc_ref(&mut self) {
        self.ref_count = self.ref_count.saturating_add(1);
    }
    /// Decrement the reference count. Returns true if still alive.
    pub fn dec_ref(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count > 0
    }
    /// Return true if this entry has no live references.
    pub fn is_dead(&self) -> bool {
        self.ref_count == 0
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
/// Memoization table mapping u64 keys to u64 values.
#[allow(dead_code)]
pub struct MemoTable {
    entries: Vec<(u64, u64)>,
}
#[allow(dead_code)]
impl MemoTable {
    /// Create a new empty memo table.
    pub fn new() -> Self {
        MemoTable {
            entries: Vec::new(),
        }
    }
    /// Insert a key→value pair; replaces if key already present.
    pub fn insert(&mut self, key: u64, val: u64) {
        if let Some(e) = self.entries.iter_mut().find(|(k, _)| *k == key) {
            e.1 = val;
        } else {
            self.entries.push((key, val));
        }
    }
    /// Look up a value by key.
    pub fn get(&self, key: u64) -> Option<u64> {
        self.entries
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
    /// Remove a key; returns the old value if present.
    pub fn remove(&mut self, key: u64) -> Option<u64> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| *k == key) {
            Some(self.entries.remove(pos).1)
        } else {
            None
        }
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Drain all entries into a vec.
    pub fn drain_all(&mut self) -> Vec<(u64, u64)> {
        let v = self.entries.clone();
        self.entries.clear();
        v
    }
}
/// Wrapper around `Expr` that implements `Hash` and `PartialEq` by structure.
///
/// This is used as the key in `ExprHashcons::expr_to_id` so that structurally
/// identical expressions map to the same slot.
#[derive(Clone, Debug)]
pub(crate) struct ExprKey(pub(crate) Expr);
/// Arena-like pool for `Expr` values backed by a hash-consing table.
///
/// `ExprPool` layers GC-root tracking on top of `ExprHashcons`.  Any
/// expression added via `add_root` (or subsequently marked with
/// `mark_root`) is considered live.
pub struct ExprPool {
    /// The underlying hash-consing table.
    hashcons: ExprHashcons,
    /// Root expression IDs (GC roots — considered permanently live).
    roots: Vec<ExprId>,
}
impl ExprPool {
    /// Create a new, empty pool.
    pub fn new() -> Self {
        ExprPool {
            hashcons: ExprHashcons::new(),
            roots: Vec::new(),
        }
    }
    /// Intern an expression without marking it as a root.
    pub fn add(&mut self, expr: Expr) -> ExprId {
        let (id, _) = self.hashcons.intern(expr);
        id
    }
    /// Intern an expression and mark it as a GC root.
    pub fn add_root(&mut self, expr: Expr) -> ExprId {
        let (id, _) = self.hashcons.intern(expr);
        self.roots.push(id);
        id
    }
    /// Look up an expression by ID.
    pub fn get(&self, id: ExprId) -> Option<&Expr> {
        self.hashcons.get(id)
    }
    /// Mark an existing interned expression as a GC root.
    ///
    /// If `id` is already a root it will appear twice in `roots`;
    /// `live_count` uses a deduplicated count so this is harmless.
    pub fn mark_root(&mut self, id: ExprId) {
        self.roots.push(id);
    }
    /// Return the number of distinct live (root) expressions.
    ///
    /// Duplicates in `roots` are ignored.
    pub fn live_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for &id in &self.roots {
            seen.insert(id);
        }
        seen.len()
    }
    /// Return the total number of distinct expressions in the pool.
    pub fn total_count(&self) -> usize {
        self.hashcons.size()
    }
    /// Compute the deduplication ratio: hits / total intern calls.
    ///
    /// A value close to 1.0 means most `add`/`add_root` calls were
    /// satisfied from cache.
    pub fn dedup_ratio(&self) -> f64 {
        self.hashcons.hit_rate()
    }
    /// Look up the  for a structurally equal expression, if interned.
    pub fn get_id(&self, expr: &Expr) -> Option<ExprId> {
        self.hashcons.get_id(expr)
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
