//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};
use std::collections::{HashMap, HashSet};

use super::functions::{mk_congr_theorem, TermIdx};

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
/// A congruence closure augmented with statistics collection.
pub struct InstrumentedCC {
    /// Inner closure.
    pub cc: CongruenceClosure,
    /// Collected statistics.
    pub stats: CongrClosureStats,
}
impl InstrumentedCC {
    /// Create a new instrumented congruence closure.
    pub fn new() -> Self {
        Self {
            cc: CongruenceClosure::new(),
            stats: CongrClosureStats::new(),
        }
    }
    /// Add an equality and record the operation.
    pub fn add_equality(&mut self, e1: Expr, e2: Expr) {
        let before = self.cc.num_classes();
        self.cc.add_equality(e1, e2);
        let after = self.cc.num_classes();
        if after < before {
            self.stats.unions += 1;
        }
        self.stats.equalities_added += 1;
        self.stats.apps_tracked = self.cc.apps.len();
    }
    /// Check equality (delegates to inner CC).
    pub fn are_equal(&mut self, e1: &Expr, e2: &Expr) -> bool {
        self.cc.are_equal(e1, e2)
    }
    /// Reset everything.
    pub fn reset(&mut self) {
        self.cc.clear();
        self.stats = CongrClosureStats::new();
    }
    /// Number of equivalence classes.
    pub fn num_classes(&mut self) -> usize {
        self.cc.num_classes()
    }
}
/// Extended equality graph for congruence closure with E-node tracking.
///
/// Provides richer structure than `CongruenceClosure` for proof reconstruction.
pub struct EGraph {
    /// E-nodes, each representing one equivalence class.
    nodes: Vec<ENode>,
}
impl EGraph {
    /// Create an empty E-graph.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    /// Add an expression to the E-graph (creating a new singleton class if not present).
    pub fn add_expr(&mut self, expr: Expr) -> usize {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.contains(&expr) {
                return i;
            }
        }
        self.nodes.push(ENode::singleton(expr));
        self.nodes.len() - 1
    }
    /// Get the class ID for an expression, if present.
    pub fn find_class(&self, expr: &Expr) -> Option<usize> {
        self.nodes.iter().position(|n| n.contains(expr))
    }
    /// Merge two equivalence classes.
    pub fn merge_classes(&mut self, id1: usize, id2: usize, proof: Option<Expr>) {
        if id1 == id2 {
            return;
        }
        let (small_id, large_id) = if id1 < id2 { (id2, id1) } else { (id1, id2) };
        let small = self.nodes.remove(small_id);
        for (m, p) in small.members.into_iter().zip(small.proofs) {
            let proof_for = proof.clone().or(p);
            self.nodes[large_id].add_member(m, proof_for);
        }
    }
    /// Check if two expressions are in the same class.
    pub fn are_equal(&self, e1: &Expr, e2: &Expr) -> bool {
        match (self.find_class(e1), self.find_class(e2)) {
            (Some(c1), Some(c2)) => c1 == c2,
            _ => false,
        }
    }
    /// Add an equality e1 = e2 with optional proof.
    pub fn add_equality(&mut self, e1: Expr, e2: Expr, proof: Option<Expr>) {
        let c1 = self.add_expr(e1);
        let c2 = self.add_expr(e2);
        if c1 != c2 {
            self.merge_classes(c1, c2, proof);
        }
    }
    /// Get the number of equivalence classes.
    pub fn num_classes(&self) -> usize {
        self.nodes.len()
    }
    /// Get an E-node by class ID.
    pub fn get_class(&self, id: usize) -> Option<&ENode> {
        self.nodes.get(id)
    }
    /// Get the representative of the class containing `expr`.
    pub fn representative(&self, expr: &Expr) -> Option<&Expr> {
        self.find_class(expr)
            .and_then(|id| self.nodes.get(id))
            .map(|n| &n.repr)
    }
    /// Clear the E-graph.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}
/// A proof of an equality in the congruence closure.
#[derive(Debug, Clone)]
pub enum CongrProof {
    /// `refl a`: a = a.
    Refl(Expr),
    /// `symm p`: p proves a = b, so symm proves b = a.
    Symm(Box<CongrProof>),
    /// `trans p q`: p proves a = b and q proves b = c.
    Trans(Box<CongrProof>, Box<CongrProof>),
    /// `congr p q`: p proves f = g and q proves a = b, so (f a) = (g b).
    Congr(Box<CongrProof>, Box<CongrProof>),
    /// `hyp name`: an equality hypothesis.
    Hyp(Name),
}
impl CongrProof {
    /// Compute the depth of the proof term.
    pub fn depth(&self) -> usize {
        match self {
            CongrProof::Refl(_) | CongrProof::Hyp(_) => 0,
            CongrProof::Symm(p) => 1 + p.depth(),
            CongrProof::Trans(p, q) | CongrProof::Congr(p, q) => 1 + p.depth().max(q.depth()),
        }
    }
    /// Check if the proof is a trivial reflexivity.
    pub fn is_refl(&self) -> bool {
        matches!(self, CongrProof::Refl(_))
    }
    /// Count the number of hypothesis uses.
    pub fn hypothesis_count(&self) -> usize {
        match self {
            CongrProof::Hyp(_) => 1,
            CongrProof::Refl(_) => 0,
            CongrProof::Symm(p) => p.hypothesis_count(),
            CongrProof::Trans(p, q) | CongrProof::Congr(p, q) => {
                p.hypothesis_count() + q.hypothesis_count()
            }
        }
    }
    /// Simplify double negations (`symm (symm p)` → `p`).
    pub fn simplify(self) -> Self {
        match self {
            CongrProof::Symm(inner) => match *inner {
                CongrProof::Symm(p) => p.simplify(),
                other => CongrProof::Symm(Box::new(other.simplify())),
            },
            CongrProof::Trans(p, q) => {
                CongrProof::Trans(Box::new(p.simplify()), Box::new(q.simplify()))
            }
            CongrProof::Congr(p, q) => {
                CongrProof::Congr(Box::new(p.simplify()), Box::new(q.simplify()))
            }
            other => other,
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
/// Cache for congruence lemmas to avoid regenerating them.
#[derive(Debug, Default)]
pub struct CongrLemmaCache {
    entries: std::collections::HashMap<(Name, usize), CongruenceTheorem>,
}
impl CongrLemmaCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Look up a cached lemma.
    pub fn get(&self, name: &Name, num_args: usize) -> Option<&CongruenceTheorem> {
        self.entries.get(&(name.clone(), num_args))
    }
    /// Insert a lemma into the cache.
    pub fn insert(&mut self, thm: CongruenceTheorem) {
        let key = (thm.fn_name.clone(), thm.num_args);
        self.entries.insert(key, thm);
    }
    /// Get or compute a basic congruence theorem.
    pub fn get_or_compute(&mut self, name: Name, num_args: usize) -> &CongruenceTheorem {
        let key = (name.clone(), num_args);
        self.entries
            .entry(key)
            .or_insert_with(|| mk_congr_theorem(name, num_args))
    }
    /// Number of cached lemmas.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
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
/// Kind of congruence argument.
///
/// When generating a congruence lemma for `f a₁ a₂ ... aₙ`,
/// each argument `aᵢ` falls into one of these categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongrArgKind {
    /// Fixed: this argument is the same on both sides (e.g., type parameters).
    Fixed,
    /// Eq: this argument requires an equality proof `aᵢ = bᵢ`.
    Eq,
    /// HEq: this argument requires a heterogeneous equality `HEq aᵢ bᵢ`.
    HEq,
    /// Cast: this argument is obtained by casting via a previous equality.
    Cast,
    /// Subsingletonality: this argument is in a subsingleton type (at most one element),
    /// so any two values are equal.
    Subsingle,
}
/// Congruence closure for tracking equalities.
///
/// Uses union-find with path compression for efficient equivalence
/// class management, plus congruence propagation for f(a) = f(b)
/// when a = b.
pub struct CongruenceClosure {
    /// Parent pointers for union-find
    parent: HashMap<Expr, Expr>,
    /// Rank for union by rank
    rank: HashMap<Expr, u32>,
    /// Pending equalities to process
    pending: Vec<(Expr, Expr)>,
    /// All App expressions we've seen, as (fn, arg) pairs
    apps: HashSet<(Expr, Expr)>,
    /// Proof terms for equalities (for proof reconstruction)
    proofs: HashMap<(Expr, Expr), Expr>,
}
impl CongruenceClosure {
    /// Create a new congruence closure.
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
            pending: Vec::new(),
            apps: HashSet::new(),
            proofs: HashMap::new(),
        }
    }
    /// Find the representative of an expression (with path compression).
    pub fn find(&mut self, expr: &Expr) -> Expr {
        if !self.parent.contains_key(expr) {
            self.parent.insert(expr.clone(), expr.clone());
            self.rank.insert(expr.clone(), 0);
            return expr.clone();
        }
        let parent = self
            .parent
            .get(expr)
            .cloned()
            .expect("expr must have a parent in the union-find structure");
        if &parent == expr {
            return expr.clone();
        }
        let root = self.find(&parent);
        self.parent.insert(expr.clone(), root.clone());
        root
    }
    /// Merge two equivalence classes (union by rank).
    fn union(&mut self, e1: &Expr, e2: &Expr) {
        let r1 = self.find(e1);
        let r2 = self.find(e2);
        if r1 == r2 {
            return;
        }
        let rank1 = *self.rank.get(&r1).unwrap_or(&0);
        let rank2 = *self.rank.get(&r2).unwrap_or(&0);
        if rank1 < rank2 {
            self.parent.insert(r1, r2);
        } else if rank1 > rank2 {
            self.parent.insert(r2, r1);
        } else {
            self.parent.insert(r2, r1.clone());
            self.rank.insert(r1, rank1 + 1);
        }
    }
    /// Register an expression and its subexpressions.
    fn register(&mut self, expr: &Expr) {
        if !self.parent.contains_key(expr) {
            self.parent.insert(expr.clone(), expr.clone());
            self.rank.insert(expr.clone(), 0);
        }
        if let Expr::App(f, a) = expr {
            self.apps.insert(((**f).clone(), (**a).clone()));
            self.register(f);
            self.register(a);
        }
    }
    /// Add an equality with an optional proof term.
    pub fn add_equality(&mut self, e1: Expr, e2: Expr) {
        self.register(&e1);
        self.register(&e2);
        self.pending.push((e1, e2));
        self.process_pending();
    }
    /// Add an equality with a proof term.
    pub fn add_equality_with_proof(&mut self, e1: Expr, e2: Expr, proof: Expr) {
        self.proofs.insert((e1.clone(), e2.clone()), proof);
        self.add_equality(e1, e2);
    }
    /// Process pending equalities and propagate congruences.
    fn process_pending(&mut self) {
        while let Some((e1, e2)) = self.pending.pop() {
            let r1 = self.find(&e1);
            let r2 = self.find(&e2);
            if r1 == r2 {
                continue;
            }
            self.union(&e1, &e2);
            let apps_vec: Vec<_> = self.apps.iter().cloned().collect();
            for i in 0..apps_vec.len() {
                for j in (i + 1)..apps_vec.len() {
                    let (f1, a1) = &apps_vec[i];
                    let (f2, a2) = &apps_vec[j];
                    let rf1 = self.find(f1);
                    let rf2 = self.find(f2);
                    let ra1 = self.find(a1);
                    let ra2 = self.find(a2);
                    if rf1 == rf2 && ra1 == ra2 {
                        let app1 = Expr::App(Box::new(f1.clone()), Box::new(a1.clone()));
                        let app2 = Expr::App(Box::new(f2.clone()), Box::new(a2.clone()));
                        if self.find(&app1) != self.find(&app2) {
                            self.pending.push((app1, app2));
                        }
                    }
                }
            }
        }
    }
    /// Check if two expressions are equal.
    pub fn are_equal(&mut self, e1: &Expr, e2: &Expr) -> bool {
        if self.find(e1) == self.find(e2) {
            return true;
        }
        match (e1, e2) {
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                self.are_equal(f1, f2) && self.are_equal(a1, a2)
            }
            _ => false,
        }
    }
    /// Get all expressions in the same equivalence class as `expr`.
    pub fn get_class(&mut self, expr: &Expr) -> Vec<Expr> {
        let root = self.find(expr);
        let keys: Vec<Expr> = self.parent.keys().cloned().collect();
        keys.into_iter().filter(|e| self.find(e) == root).collect()
    }
    /// Get the number of equivalence classes.
    pub fn num_classes(&mut self) -> usize {
        let keys: Vec<Expr> = self.parent.keys().cloned().collect();
        let mut roots = HashSet::new();
        for k in &keys {
            roots.insert(self.find(k));
        }
        roots.len()
    }
    /// Get a proof that two expressions are equal (if available).
    pub fn get_proof(&self, e1: &Expr, e2: &Expr) -> Option<&Expr> {
        self.proofs
            .get(&(e1.clone(), e2.clone()))
            .or_else(|| self.proofs.get(&(e2.clone(), e1.clone())))
    }
    /// Clear all state.
    pub fn clear(&mut self) {
        self.parent.clear();
        self.rank.clear();
        self.pending.clear();
        self.apps.clear();
        self.proofs.clear();
    }
}
/// A flat application: `(fn_idx, arg_idx) → result_idx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlatApp {
    /// Index of the function term.
    pub fn_idx: TermIdx,
    /// Index of the argument term.
    pub arg_idx: TermIdx,
    /// Index of the result term (the node representing the application).
    /// `None` if not tracked (result congruence propagation is skipped).
    pub result_idx: Option<TermIdx>,
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
/// A congruence theorem for a specific function.
#[derive(Debug, Clone)]
pub struct CongruenceTheorem {
    /// The function this theorem is about
    pub fn_name: Name,
    /// The number of arguments
    pub num_args: usize,
    /// The kind of each argument
    pub arg_kinds: Vec<CongrArgKind>,
    /// The proof term (if generated)
    pub proof: Option<Expr>,
    /// The type of the congruence lemma
    pub ty: Option<Expr>,
}
impl CongruenceTheorem {
    /// Create a new congruence theorem description.
    pub fn new(fn_name: Name, arg_kinds: Vec<CongrArgKind>) -> Self {
        let num_args = arg_kinds.len();
        Self {
            fn_name,
            num_args,
            arg_kinds,
            proof: None,
            ty: None,
        }
    }
    /// Check if any argument needs an equality proof.
    pub fn has_eq_args(&self) -> bool {
        self.arg_kinds
            .iter()
            .any(|k| matches!(k, CongrArgKind::Eq | CongrArgKind::HEq))
    }
    /// Get the number of equality hypotheses needed.
    pub fn num_eq_hypotheses(&self) -> usize {
        self.arg_kinds
            .iter()
            .filter(|k| matches!(k, CongrArgKind::Eq | CongrArgKind::HEq))
            .count()
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
/// A hypothesis in a congruence lemma.
///
/// Records the type of an equality hypothesis that must be provided.
#[derive(Debug, Clone)]
pub struct CongrHypothesis {
    /// Left-hand side of the equality.
    pub lhs: Expr,
    /// Right-hand side of the equality.
    pub rhs: Expr,
    /// Whether this is a heterogeneous equality.
    pub is_heq: bool,
}
impl CongrHypothesis {
    /// Create a homogeneous equality hypothesis `lhs = rhs`.
    pub fn eq(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            is_heq: false,
        }
    }
    /// Create a heterogeneous equality hypothesis `HEq lhs rhs`.
    pub fn heq(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            is_heq: true,
        }
    }
    /// Check if this is a trivial hypothesis (`lhs = lhs`).
    pub fn is_trivial(&self) -> bool {
        self.lhs == self.rhs
    }
}
/// Flat union-find for congruence closure on term indices.
///
/// This is a simpler, more cache-friendly representation than
/// the `HashMap`-based approach above.
pub struct FlatCC {
    /// Parent pointers (by index).
    parent: Vec<TermIdx>,
    /// Rank for union by rank.
    rank: Vec<u32>,
    /// All registered applications.
    apps: Vec<FlatApp>,
}
impl FlatCC {
    /// Create a new flat CC with `n` initial singleton classes.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            apps: Vec::new(),
        }
    }
    /// Find the root of the equivalence class containing `i`.
    pub fn find(&mut self, i: TermIdx) -> TermIdx {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }
    /// Merge the classes of `i` and `j`.
    pub fn union(&mut self, i: TermIdx, j: TermIdx) {
        let ri = self.find(i);
        let rj = self.find(j);
        if ri == rj {
            return;
        }
        if self.rank[ri] < self.rank[rj] {
            self.parent[ri] = rj;
        } else if self.rank[ri] > self.rank[rj] {
            self.parent[rj] = ri;
        } else {
            self.parent[rj] = ri;
            self.rank[ri] += 1;
        }
    }
    /// Add a new term node, returning its index.
    pub fn add_node(&mut self) -> TermIdx {
        let idx = self.parent.len();
        self.parent.push(idx);
        self.rank.push(0);
        idx
    }
    /// Register an application.
    pub fn add_app(&mut self, app: FlatApp) {
        self.apps.push(app);
    }
    /// Check if `i` and `j` are in the same class.
    pub fn are_equal(&mut self, i: TermIdx, j: TermIdx) -> bool {
        self.find(i) == self.find(j)
    }
    /// Get the total number of term nodes.
    pub fn num_nodes(&self) -> usize {
        self.parent.len()
    }
    /// Get the number of registered applications.
    pub fn num_apps(&self) -> usize {
        self.apps.len()
    }
    /// Propagate congruences: if fn(i) == fn(j) and arg(i) == arg(j),
    /// then result(i) == result(j).
    ///
    /// Merges the result classes of congruent applications so that downstream
    /// `are_equal` queries reflect these equalities.
    pub fn propagate_congruences(&mut self) {
        let len = self.apps.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let a = self.apps[i];
                let b = self.apps[j];
                if self.are_equal(a.fn_idx, b.fn_idx) && self.are_equal(a.arg_idx, b.arg_idx) {
                    if let (Some(ra), Some(rb)) = (a.result_idx, b.result_idx) {
                        self.union(ra, rb);
                    }
                }
            }
        }
    }
}
/// A node in the E-graph, representing an equivalence class.
#[derive(Debug, Clone)]
pub struct ENode {
    /// Representative of this equivalence class.
    pub repr: Expr,
    /// All members of this equivalence class.
    pub members: Vec<Expr>,
    /// Proof terms linking members to the representative (if available).
    pub proofs: Vec<Option<Expr>>,
}
impl ENode {
    /// Create a new E-node with a single expression.
    pub fn singleton(expr: Expr) -> Self {
        Self {
            repr: expr.clone(),
            members: vec![expr],
            proofs: vec![None],
        }
    }
    /// Add an expression to this equivalence class.
    pub fn add_member(&mut self, expr: Expr, proof: Option<Expr>) {
        if !self.members.contains(&expr) {
            self.members.push(expr);
            self.proofs.push(proof);
        }
    }
    /// Check if an expression is in this class.
    pub fn contains(&self, expr: &Expr) -> bool {
        self.members.contains(expr)
    }
    /// Size of the equivalence class.
    pub fn size(&self) -> usize {
        self.members.len()
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
/// Statistics gathered by the congruence closure algorithm.
#[derive(Debug, Clone, Default)]
pub struct CongrClosureStats {
    /// Number of equalities added.
    pub equalities_added: usize,
    /// Number of congruence steps propagated.
    pub congruences_propagated: usize,
    /// Number of union operations performed.
    pub unions: usize,
    /// Number of app expressions tracked.
    pub apps_tracked: usize,
}
impl CongrClosureStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
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
