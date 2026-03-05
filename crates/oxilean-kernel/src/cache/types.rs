//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::expr_util::lift_loose_bvars;
use std::collections::HashMap;

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
/// Global cache manager for all kernel caches
pub struct CacheManager {
    pub(crate) whnf: WhnfCache,
    pub(crate) defeq: DefEqCache,
    pub(crate) infer: InferCache,
}
impl CacheManager {
    /// Create a new cache manager with default capacities
    pub fn new() -> Self {
        CacheManager {
            whnf: WhnfCache::new(1024, false),
            defeq: DefEqCache::new(512),
            infer: InferCache::new(256),
        }
    }
    /// Create a cache manager with custom capacities
    pub fn with_capacities(whnf_cap: usize, defeq_cap: usize, infer_cap: usize) -> Self {
        CacheManager {
            whnf: WhnfCache::new(whnf_cap, false),
            defeq: DefEqCache::new(defeq_cap),
            infer: InferCache::new(infer_cap),
        }
    }
    /// Get mutable reference to WHNF cache
    pub fn whnf_mut(&mut self) -> &mut WhnfCache {
        &mut self.whnf
    }
    /// Get mutable reference to DefEq cache
    pub fn defeq_mut(&mut self) -> &mut DefEqCache {
        &mut self.defeq
    }
    /// Get mutable reference to Infer cache
    pub fn infer_mut(&mut self) -> &mut InferCache {
        &mut self.infer
    }
    /// Clear all caches
    pub fn clear_all(&mut self) {
        self.whnf.clear();
        self.defeq.clear();
        self.infer.clear();
    }
    /// Resize all caches to new capacities
    pub fn resize_all(&mut self, whnf_cap: usize, defeq_cap: usize, infer_cap: usize) {
        self.whnf = WhnfCache::new(whnf_cap, self.whnf.is_transparent());
        self.defeq = DefEqCache::new(defeq_cap);
        self.infer = InferCache::new(infer_cap);
    }
    /// Get comprehensive statistics for all caches
    pub fn statistics(&self) -> CacheStatistics {
        let (whnf_hits, whnf_misses) = self.whnf.stats();
        let (defeq_hits, defeq_misses) = self.defeq.stats();
        let (infer_hits, infer_misses) = self.infer.stats();
        CacheStatistics {
            whnf_hits,
            whnf_misses,
            whnf_hit_rate: self.whnf.hit_rate(),
            defeq_hits,
            defeq_misses,
            defeq_hit_rate: self.defeq.hit_rate(),
            infer_hits,
            infer_misses,
            infer_hit_rate: self.infer.hit_rate(),
        }
    }
}
/// Simplified Expr representation for hashing (mirrors actual Expr).
///
/// Used for caching purposes with a streamlined structure suitable for hashing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimplifiedExpr {
    /// A variable reference with the given name.
    Var(String),
    /// Function application: function and argument.
    App(Box<SimplifiedExpr>, Box<SimplifiedExpr>),
    /// Lambda abstraction: parameter name and body.
    Lambda(String, Box<SimplifiedExpr>),
    /// Pi type: parameter name, parameter type, and body type.
    Pi(String, Box<SimplifiedExpr>, Box<SimplifiedExpr>),
}
impl SimplifiedExpr {
    /// Compute FNV-1a hash for this expression
    pub fn hash(&self) -> u64 {
        match self {
            SimplifiedExpr::Var(name) => {
                let mut bytes = vec![0u8];
                bytes.extend_from_slice(name.as_bytes());
                fnv1a_hash(&bytes)
            }
            SimplifiedExpr::App(f, arg) => {
                let f_hash = f.hash();
                let arg_hash = arg.hash();
                let mut bytes = vec![1u8];
                bytes.extend_from_slice(&f_hash.to_le_bytes());
                bytes.extend_from_slice(&arg_hash.to_le_bytes());
                fnv1a_hash(&bytes)
            }
            SimplifiedExpr::Lambda(name, body) => {
                let body_hash = body.hash();
                let mut bytes = vec![2u8];
                bytes.extend_from_slice(name.as_bytes());
                bytes.extend_from_slice(&body_hash.to_le_bytes());
                fnv1a_hash(&bytes)
            }
            SimplifiedExpr::Pi(name, typ, body) => {
                let typ_hash = typ.hash();
                let body_hash = body.hash();
                let mut bytes = vec![3u8];
                bytes.extend_from_slice(name.as_bytes());
                bytes.extend_from_slice(&typ_hash.to_le_bytes());
                bytes.extend_from_slice(&body_hash.to_le_bytes());
                fnv1a_hash(&bytes)
            }
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
/// Generic LRU Cache with HashMap + Vec-based intrusive list
pub struct LruCache<K: Clone + Eq + std::hash::Hash, V: Clone> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: Option<usize>,
    tail: Option<usize>,
    hits: u64,
    misses: u64,
}
impl<K: Clone + Eq + std::hash::Hash, V: Clone> LruCache<K, V> {
    /// Create a new LRU cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LRU cache capacity must be > 0");
        LruCache {
            capacity,
            map: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
            hits: 0,
            misses: 0,
        }
    }
    /// Get a value by key and move it to the most recently used position
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(&idx) = self.map.get(key) {
            self.hits += 1;
            self.move_to_head(idx);
            Some(self.nodes[idx].value.clone())
        } else {
            self.misses += 1;
            None
        }
    }
    /// Insert a key-value pair into the cache
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].value = value;
            self.move_to_head(idx);
        } else {
            if self.nodes.len() >= self.capacity {
                self.evict_lru();
            }
            let new_idx = self.nodes.len();
            let node = Node {
                key: key.clone(),
                value,
                prev: None,
                next: self.head,
            };
            self.nodes.push(node);
            self.map.insert(key, new_idx);
            if let Some(old_head) = self.head {
                self.nodes[old_head].prev = Some(new_idx);
            }
            self.head = Some(new_idx);
            if self.tail.is_none() {
                self.tail = Some(new_idx);
            }
        }
    }
    /// Remove a key from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(&idx) = self.map.get(key) {
            let node = &self.nodes[idx];
            let prev = node.prev;
            let next = node.next;
            if let Some(p) = prev {
                self.nodes[p].next = next;
            } else {
                self.tail = next;
            }
            if let Some(n) = next {
                self.nodes[n].prev = prev;
            } else {
                self.head = prev;
            }
            self.map.remove(key);
            Some(self.nodes[idx].value.clone())
        } else {
            None
        }
    }
    /// Check if key exists in the cache
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Get the capacity of the cache
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Clear all entries from the cache
    pub fn clear(&mut self) {
        self.map.clear();
        self.nodes.clear();
        self.head = None;
        self.tail = None;
        self.hits = 0;
        self.misses = 0;
    }
    /// Get cache statistics
    pub fn stats(&self) -> (u64, u64) {
        (self.hits, self.misses)
    }
    /// Get hit rate as a percentage (0.0 to 100.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
    fn move_to_head(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        if let Some(p) = prev {
            self.nodes[p].next = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            self.tail = prev;
        }
        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;
        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }
        self.head = Some(idx);
    }
    fn evict_lru(&mut self) {
        if let Some(tail_idx) = self.tail {
            let key = self.nodes[tail_idx].key.clone();
            let prev = self.nodes[tail_idx].prev;
            if let Some(p) = prev {
                self.nodes[p].next = None;
                self.head = Some(p);
            } else {
                self.head = None;
            }
            self.tail = prev;
            self.map.remove(&key);
            self.nodes.remove(tail_idx);
            self.nodes.iter().enumerate().for_each(|(i, node)| {
                *self
                    .map
                    .get_mut(&node.key)
                    .expect("node key must exist in map") = i;
            });
        }
    }
}
/// A very simple approximate membership test using bit-hashing.
///
/// This is NOT a production-quality bloom filter; it uses only two hash
/// functions and a small bit array for illustration.
pub struct BloomFilterApprox {
    bits: Vec<bool>,
    size: usize,
}
impl BloomFilterApprox {
    /// Create a bloom filter with `size` bits.
    pub fn new(size: usize) -> Self {
        Self {
            bits: vec![false; size],
            size,
        }
    }
    fn hash1(data: &[u8], size: usize) -> usize {
        fnv1a_hash(data) as usize % size
    }
    fn hash2(data: &[u8], size: usize) -> usize {
        let h = fnv1a_hash(data).wrapping_mul(0x9e3779b9_7f4a7c15);
        h as usize % size
    }
    /// Insert a key.
    pub fn insert<T: AsRef<[u8]>>(&mut self, key: T) {
        let bytes = key.as_ref();
        let h1 = Self::hash1(bytes, self.size);
        let h2 = Self::hash2(bytes, self.size);
        self.bits[h1] = true;
        self.bits[h2] = true;
    }
    /// Check if a key might be present (may have false positives).
    pub fn might_contain<T: AsRef<[u8]>>(&self, key: T) -> bool {
        let bytes = key.as_ref();
        let h1 = Self::hash1(bytes, self.size);
        let h2 = Self::hash2(bytes, self.size);
        self.bits[h1] && self.bits[h2]
    }
    /// Clear all bits.
    pub fn clear(&mut self) {
        self.bits.iter_mut().for_each(|b| *b = false);
    }
    /// Number of set bits.
    pub fn set_bit_count(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }
    /// Filter size in bits.
    pub fn size(&self) -> usize {
        self.size
    }
}
/// A two-level cache hierarchy.
///
/// L1 is a small, fast LRU cache. L2 is a larger LRU cache. On a miss in L1,
/// L2 is checked and the result is promoted to L1.
pub struct MultiLevelCache<K: Clone + Eq + std::hash::Hash, V: Clone> {
    l1: LruCache<K, V>,
    l2: LruCache<K, V>,
    l1_hits: u64,
    l2_hits: u64,
    misses: u64,
}
impl<K: Clone + Eq + std::hash::Hash, V: Clone> MultiLevelCache<K, V> {
    /// Create a two-level cache.
    pub fn new(l1_cap: usize, l2_cap: usize) -> Self {
        Self {
            l1: LruCache::new(l1_cap),
            l2: LruCache::new(l2_cap),
            l1_hits: 0,
            l2_hits: 0,
            misses: 0,
        }
    }
    /// Get a value, checking L1 first, then L2.
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(v) = self.l1.get(key) {
            self.l1_hits += 1;
            return Some(v);
        }
        if let Some(v) = self.l2.get(key) {
            self.l2_hits += 1;
            self.l1.insert(key.clone(), v.clone());
            return Some(v);
        }
        self.misses += 1;
        None
    }
    /// Insert into both L1 and L2.
    pub fn insert(&mut self, key: K, value: V) {
        self.l1.insert(key.clone(), value.clone());
        self.l2.insert(key, value);
    }
    /// Insert only into L2 (for pre-warming the L2 cache).
    pub fn insert_l2_only(&mut self, key: K, value: V) {
        self.l2.insert(key, value);
    }
    /// Clear L1 only.
    pub fn clear_l1(&mut self) {
        self.l1.clear();
    }
    /// Clear both levels.
    pub fn clear_all(&mut self) {
        self.l1.clear();
        self.l2.clear();
        self.l1_hits = 0;
        self.l2_hits = 0;
        self.misses = 0;
    }
    /// L1 hit count.
    pub fn l1_hits(&self) -> u64 {
        self.l1_hits
    }
    /// L2 hit count.
    pub fn l2_hits(&self) -> u64 {
        self.l2_hits
    }
    /// Miss count.
    pub fn misses(&self) -> u64 {
        self.misses
    }
    /// Total requests served.
    pub fn total_requests(&self) -> u64 {
        self.l1_hits + self.l2_hits + self.misses
    }
    /// Overall hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_requests();
        if total == 0 {
            0.0
        } else {
            ((self.l1_hits + self.l2_hits) as f64 / total as f64) * 100.0
        }
    }
}
/// Symmetry-aware cache for definitional equality checks
pub struct DefEqCache {
    pub(crate) cache: LruCache<(u64, u64), bool>,
}
impl DefEqCache {
    /// Create a new definitional equality cache
    pub fn new(capacity: usize) -> Self {
        DefEqCache {
            cache: LruCache::new(capacity),
        }
    }
    /// Check if a definitional equality result is cached (symmetry-aware)
    pub fn check_cache(&mut self, expr1: &SimplifiedExpr, expr2: &SimplifiedExpr) -> Option<bool> {
        let hash1 = expr1.hash();
        let hash2 = expr2.hash();
        if let Some(result) = self.cache.get(&(hash1, hash2)) {
            return Some(result);
        }
        if let Some(result) = self.cache.get(&(hash2, hash1)) {
            return Some(result);
        }
        None
    }
    /// Store a definitional equality result (symmetry-aware)
    pub fn store_result(&mut self, expr1: &SimplifiedExpr, expr2: &SimplifiedExpr, result: bool) {
        let hash1 = expr1.hash();
        let hash2 = expr2.hash();
        self.cache.insert((hash1, hash2), result);
        self.cache.insert((hash2, hash1), result);
    }
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Get cache statistics
    pub fn stats(&self) -> (u64, u64) {
        self.cache.stats()
    }
    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        self.cache.hit_rate()
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
#[derive(Clone, Debug)]
struct TtlEntry<V> {
    value: V,
    expires_at: u64,
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
/// Statistics for all caches.
///
/// Provides comprehensive metrics for monitoring cache performance across all cache types.
#[derive(Clone, Debug)]
pub struct CacheStatistics {
    /// Number of hits in the WHNF cache.
    pub whnf_hits: u64,
    /// Number of misses in the WHNF cache.
    pub whnf_misses: u64,
    /// Hit rate percentage for the WHNF cache (0.0-100.0).
    pub whnf_hit_rate: f64,
    /// Number of hits in the DefEq cache.
    pub defeq_hits: u64,
    /// Number of misses in the DefEq cache.
    pub defeq_misses: u64,
    /// Hit rate percentage for the DefEq cache (0.0-100.0).
    pub defeq_hit_rate: f64,
    /// Number of hits in the Infer cache.
    pub infer_hits: u64,
    /// Number of misses in the Infer cache.
    pub infer_misses: u64,
    /// Hit rate percentage for the Infer cache (0.0-100.0).
    pub infer_hit_rate: f64,
}
impl CacheStatistics {
    /// Get total hits across all caches
    pub fn total_hits(&self) -> u64 {
        self.whnf_hits + self.defeq_hits + self.infer_hits
    }
    /// Get total misses across all caches
    pub fn total_misses(&self) -> u64 {
        self.whnf_misses + self.defeq_misses + self.infer_misses
    }
    /// Get overall hit rate across all caches
    pub fn overall_hit_rate(&self) -> f64 {
        let total = self.total_hits() + self.total_misses();
        if total == 0 {
            0.0
        } else {
            (self.total_hits() as f64 / total as f64) * 100.0
        }
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
/// Cache for type inference results
pub struct InferCache {
    pub(crate) cache: LruCache<u64, SimplifiedExpr>,
}
impl InferCache {
    /// Create a new type inference cache
    pub fn new(capacity: usize) -> Self {
        InferCache {
            cache: LruCache::new(capacity),
        }
    }
    /// Look up inferred type for an expression
    pub fn lookup(&mut self, expr: &SimplifiedExpr) -> Option<SimplifiedExpr> {
        let hash = expr.hash();
        self.cache.get(&hash)
    }
    /// Store inferred type for an expression
    pub fn store(&mut self, expr: &SimplifiedExpr, inferred_type: SimplifiedExpr) {
        let hash = expr.hash();
        self.cache.insert(hash, inferred_type);
    }
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Get cache statistics
    pub fn stats(&self) -> (u64, u64) {
        self.cache.stats()
    }
    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
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
/// Cache for weak head normal form (WHNF) reduction results
pub struct WhnfCache {
    pub(crate) cache: LruCache<u64, SimplifiedExpr>,
    transparency_mode: bool,
}
impl WhnfCache {
    /// Create a new WHNF cache
    pub fn new(capacity: usize, transparency_mode: bool) -> Self {
        WhnfCache {
            cache: LruCache::new(capacity),
            transparency_mode,
        }
    }
    /// Look up an expression's WHNF in the cache
    pub fn lookup(&mut self, expr: &SimplifiedExpr) -> Option<SimplifiedExpr> {
        if !self.transparency_mode {
            let hash = expr.hash();
            self.cache.get(&hash)
        } else {
            None
        }
    }
    /// Store an expression and its WHNF in the cache
    pub fn store(&mut self, expr: &SimplifiedExpr, whnf: SimplifiedExpr) {
        if !self.transparency_mode {
            let hash = expr.hash();
            self.cache.insert(hash, whnf);
        }
    }
    /// Clear the WHNF cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Get cache statistics
    pub fn stats(&self) -> (u64, u64) {
        self.cache.stats()
    }
    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
    /// Set transparency mode
    pub fn set_transparency(&mut self, mode: bool) {
        self.transparency_mode = mode;
        if mode {
            self.cache.clear();
        }
    }
    /// Get current transparency mode
    pub fn is_transparent(&self) -> bool {
        self.transparency_mode
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
/// A simple TTL-based cache using a step counter as a clock.
///
/// Each entry has a `ttl` expressed in "steps". The cache is checked on every
/// `get`; stale entries are lazily removed.
pub struct TtlCache<K: Clone + Eq + std::hash::Hash, V: Clone> {
    entries: HashMap<K, TtlEntry<V>>,
    /// Monotonic step counter (incremented on every `tick()`).
    step: u64,
    /// Default TTL for new entries.
    default_ttl: u64,
}
impl<K: Clone + Eq + std::hash::Hash, V: Clone> TtlCache<K, V> {
    /// Create a new TTL cache.
    pub fn new(default_ttl: u64) -> Self {
        Self {
            entries: HashMap::new(),
            step: 0,
            default_ttl,
        }
    }
    /// Advance the clock by one step.
    pub fn tick(&mut self) {
        self.step += 1;
    }
    /// Advance the clock by `n` steps.
    pub fn tick_n(&mut self, n: u64) {
        self.step += n;
    }
    /// Insert an entry with the default TTL.
    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(
            key,
            TtlEntry {
                value,
                expires_at: self.step + self.default_ttl,
            },
        );
    }
    /// Insert an entry with a custom TTL.
    pub fn insert_with_ttl(&mut self, key: K, value: V, ttl: u64) {
        self.entries.insert(
            key,
            TtlEntry {
                value,
                expires_at: self.step + ttl,
            },
        );
    }
    /// Get an entry if it hasn't expired.
    pub fn get(&self, key: &K) -> Option<V> {
        self.entries.get(key).and_then(|e| {
            if self.step < e.expires_at {
                Some(e.value.clone())
            } else {
                None
            }
        })
    }
    /// Remove expired entries.
    pub fn purge_expired(&mut self) {
        let step = self.step;
        self.entries.retain(|_, e| step < e.expires_at);
    }
    /// Number of entries (including possibly stale).
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Current step counter.
    pub fn current_step(&self) -> u64 {
        self.step
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
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
/// Node in the intrusive doubly-linked list for LRU tracking
#[derive(Clone, Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
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
