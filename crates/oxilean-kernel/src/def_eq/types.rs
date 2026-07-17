//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::equiv_manager::EquivManager;
use crate::expr_util::{
    get_app_args, get_app_fn, get_app_fn_args, has_loose_bvar, has_loose_bvars, mk_app,
};
use crate::instantiate::instantiate_type_lparams;
use crate::level;
use crate::reduce::{Reducer, ReducibilityHint, TransparencyMode};
use crate::subst::instantiate;
use crate::Node;
use crate::{Environment, Expr, Level};
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
/// A simple ordered index mapping names to integer IDs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NameIndex {
    names: Vec<String>,
    index: std::collections::HashMap<String, usize>,
}
impl NameIndex {
    /// Create a new empty name index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a name, returning its ID.
    /// If the name is already present, returns the existing ID.
    #[allow(dead_code)]
    pub fn insert(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        if let Some(&id) = self.index.get(&name) {
            return id;
        }
        let id = self.names.len();
        self.index.insert(name.clone(), id);
        self.names.push(name);
        id
    }
    /// Get the ID for a name, if it exists.
    #[allow(dead_code)]
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }
    /// Get the name for an ID.
    #[allow(dead_code)]
    pub fn get_name(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(|s| s.as_str())
    }
    /// Get the number of names in the index.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Check if the index is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Get all names in insertion order.
    #[allow(dead_code)]
    pub fn all_names(&self) -> &[String] {
        &self.names
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
/// A batch checker that can check multiple pairs efficiently.
#[allow(dead_code)]
pub struct BatchDefEqChecker<'env> {
    checker: DefEqChecker<'env>,
}
impl<'env> BatchDefEqChecker<'env> {
    /// Create a new batch checker.
    #[allow(dead_code)]
    pub fn new(env: &'env Environment) -> Self {
        Self {
            checker: DefEqChecker::new(env),
        }
    }
    /// Check definitional equality for a single pair.
    #[allow(dead_code)]
    pub fn check(&mut self, t: &Expr, s: &Expr) -> bool {
        self.checker.is_def_eq(t, s)
    }
    /// Check all pairs in a list; returns true iff all pairs are def-equal.
    #[allow(dead_code)]
    pub fn check_all(&mut self, pairs: &[(Expr, Expr)]) -> bool {
        pairs.iter().all(|(t, s)| self.checker.is_def_eq(t, s))
    }
    /// Check any pair in a list; returns true iff at least one pair is def-equal.
    #[allow(dead_code)]
    pub fn check_any(&mut self, pairs: &[(Expr, Expr)]) -> bool {
        pairs.iter().any(|(t, s)| self.checker.is_def_eq(t, s))
    }
    /// Count how many pairs in the list are definitionally equal.
    #[allow(dead_code)]
    pub fn count_equal(&mut self, pairs: &[(Expr, Expr)]) -> usize {
        pairs
            .iter()
            .filter(|(t, s)| self.checker.is_def_eq(t, s))
            .count()
    }
    /// Reset the checker (clears caches).
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.checker.cache.clear();
        self.checker.equiv_manager.clear();
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
/// Statistics for a DefEq check run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DefEqStats {
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of cache misses.
    pub cache_misses: u64,
    /// Number of reduction steps taken.
    pub reduction_steps: u64,
    /// Number of successful delta reductions.
    pub delta_reductions: u64,
    /// Number of beta reductions.
    pub beta_reductions: u64,
    /// Number of eta expansions tried.
    pub eta_attempts: u64,
    /// Number of equiv-manager hits.
    pub equiv_hits: u64,
}
impl DefEqStats {
    /// Compute the cache hit rate in [0, 1].
    #[allow(dead_code)]
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            1.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    /// Compute total number of cache accesses.
    #[allow(dead_code)]
    pub fn total_cache_accesses(&self) -> u64 {
        self.cache_hits + self.cache_misses
    }
    /// Compute total number of reductions across all kinds.
    #[allow(dead_code)]
    pub fn total_reductions(&self) -> u64 {
        self.reduction_steps + self.delta_reductions + self.beta_reductions
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
/// Status of a lazy delta reduction step.
#[derive(Debug, PartialEq, Eq)]
pub enum ReductionStatus {
    /// Continue: reduced one or both sides.
    Continue(Expr, Expr),
    /// Definitely equal.
    Equal,
    /// Definitely not equal (both stuck).
    Stuck,
    /// Unknown: need more reduction.
    Unknown,
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
/// Configuration for the definitional equality checker.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DefEqConfig {
    /// Maximum number of reduction steps before giving up.
    pub max_steps: u32,
    /// Whether to use proof irrelevance (Props are equal).
    pub proof_irrelevance: bool,
    /// Whether to check eta-expansion.
    pub eta: bool,
    /// Whether to try lazy delta reduction.
    pub lazy_delta: bool,
    /// Transparency mode for unfolding.
    pub transparency: TransparencyMode,
}
impl DefEqConfig {
    /// Create a config for checking definitional equality with full unfolding.
    #[allow(dead_code)]
    pub fn full_transparency() -> Self {
        Self {
            transparency: TransparencyMode::All,
            ..Self::default()
        }
    }
    /// Create a config for opaque checking (no unfolding).
    #[allow(dead_code)]
    pub fn opaque() -> Self {
        Self {
            lazy_delta: false,
            transparency: TransparencyMode::None,
            ..Self::default()
        }
    }
    /// Create a config with proof irrelevance disabled.
    #[allow(dead_code)]
    pub fn no_proof_irrelevance() -> Self {
        Self {
            proof_irrelevance: false,
            ..Self::default()
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
/// Simple trie for efficient string prefix lookup.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StringTrie {
    pub(super) children: std::collections::HashMap<char, StringTrie>,
    is_end: bool,
    pub(super) value: Option<String>,
}
impl StringTrie {
    /// Create a new empty trie.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a string into the trie.
    #[allow(dead_code)]
    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
        node.value = Some(s.to_string());
    }
    /// Check if a string is in the trie.
    #[allow(dead_code)]
    pub fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }
    /// Find all strings with a given prefix.
    #[allow(dead_code)]
    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return vec![],
            }
        }
        let mut results = Vec::new();
        collect_strings(node, &mut results);
        results
    }
    /// Get the number of strings in the trie.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut count = if self.is_end { 1 } else { 0 };
        for child in self.children.values() {
            count += child.len();
        }
        count
    }
    /// Check if the trie is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
/// Definitional equality checker with full optimizations.
pub struct DefEqChecker<'env> {
    env: &'env Environment,
    reducer: Reducer,
    cache: HashMap<(Expr, Expr), bool>,
    equiv_manager: EquivManager,
    proof_irrelevance: bool,
    /// Domain types of the binders the checker has descended under
    /// (outermost first). Entry `i` is expressed in the context of entries
    /// `0..i`, exactly like a de Bruijn telescope. This lets
    /// `Self::quick_infer_type` type loose bound variables, which the
    /// type-*dependent* rules (structure eta, unit-like equality, proof
    /// irrelevance) need when comparing open `Pi`/`Lam` bodies — Lean's
    /// kernel gets the same information from its local context.
    binder_types: Vec<Expr>,
    /// Types of the free variables the surrounding `TypeChecker` has opened
    /// (its local context), keyed by `FVarId`. This lets
    /// `Self::quick_infer_type` type FVars, which one-sided eta expansion
    /// (`eta_expand_one`) needs when a Pi-bound function is compared against
    /// a lambda — e.g. Lean core's `funext`, where `f` must eta-expand to
    /// match `fun x => extfunApp (Quot.mk eqv f) x`. Lean's kernel reads the
    /// same information from its local context.
    fvar_types: HashMap<u64, Expr>,
    /// The next id for a local free variable minted by
    /// [`Self::is_def_eq_under`] when opening a binder. Dispensed from a
    /// dedicated high range ([`DEF_EQ_OPENED_FVAR_BASE`]) so ids never
    /// collide with the surrounding `TypeChecker`'s (which are dispensed
    /// sequentially from 0 — reaching this range would take 2^62 locals).
    next_opened_fvar: u64,
}
/// Base id for binder-opening free variables minted by the def-eq checker.
const DEF_EQ_OPENED_FVAR_BASE: u64 = 1 << 62;
/// Maximum AST size (in nodes) of either side of a pair retained in the
/// def-eq cache / equivalence manager. Oversized pairs are recomputed
/// instead of retained; verdicts are unchanged.
const DEF_EQ_CACHE_MAX_NODES: usize = 4096;
impl<'env> DefEqChecker<'env> {
    /// Create a new definitional equality checker.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            env,
            reducer: Reducer::new(),
            cache: HashMap::new(),
            equiv_manager: EquivManager::new(),
            proof_irrelevance: true,
            binder_types: Vec::new(),
            fvar_types: HashMap::new(),
            next_opened_fvar: DEF_EQ_OPENED_FVAR_BASE,
        }
    }
    /// Record the type of a free variable opened by the surrounding
    /// type checker, so `Self::quick_infer_type` can type it.
    ///
    /// `FVarId`s are dispensed monotonically by the `TypeChecker` and never
    /// reused, so recorded types are stable for the checker's lifetime and
    /// cached def-eq verdicts stay sound. Defensively, if a caller *does*
    /// re-bind an id to a different type (only possible through the public
    /// `TypeChecker::push_local`), the caches are invalidated.
    pub fn record_fvar_type(&mut self, id: crate::FVarId, ty: Expr) {
        if let Some(prev) = self.fvar_types.get(&id.0) {
            if *prev != ty {
                self.cache.clear();
                self.equiv_manager.clear();
            }
        }
        // Mirror into the reducer so K-like iota reduction can type recursor
        // majors mentioning this free variable during WHNF.
        self.reducer.record_fvar_type(id, ty.clone());
        self.fvar_types.insert(id.0, ty);
    }
    /// Disable proof irrelevance for this checker.
    pub fn set_proof_irrelevance(&mut self, enabled: bool) {
        self.proof_irrelevance = enabled;
    }
    /// Set the transparency mode for this checker.
    pub fn set_transparency(&mut self, mode: TransparencyMode) {
        self.reducer.set_transparency(mode);
        self.cache.clear();
        self.equiv_manager.clear();
    }
    /// Check if two expressions are definitionally equal.
    pub fn is_def_eq(&mut self, t: &Expr, s: &Expr) -> bool {
        if t == s {
            return true;
        }
        // Resource-fuel degradation (C16): with the budget exhausted only
        // syntactic equality (above) is decided — `false` here is always
        // conservative (never a wrong accept), and it breaks the
        // unfold/whnf/compare loops that would otherwise keep growing
        // terms. The caller reports the failure as a named resource limit.
        if crate::fuel::is_exhausted() {
            return false;
        }
        // Context-dependence guard: for OPEN terms under binders the verdict
        // can depend on the binder-type stack (structure eta / proof
        // irrelevance type loose BVars via `quick_infer_type`), so only pairs
        // that are closed — or compared at the top level, where loose BVars
        // cannot be typed and the rules degrade to structural ones — may
        // participate in the cache and the equivalence manager.
        //
        // Size guard: giant intermediates (multi-million-node reduction
        // steps) are recomputed rather than retained — retaining every such
        // pair is what turned Lean core's `Int.add_mul_ediv_right` into a
        // multi-GiB peak (Init corpus, C16). Never changes a verdict.
        let cacheable = (self.binder_types.is_empty()
            || (!has_loose_bvars(t) && !has_loose_bvars(s)))
            && crate::expr_util::expr_size_within(t, DEF_EQ_CACHE_MAX_NODES)
            && crate::expr_util::expr_size_within(s, DEF_EQ_CACHE_MAX_NODES);
        if cacheable {
            if self.equiv_manager.is_equiv(t, s) {
                return true;
            }
            if self.equiv_manager.is_failure(t, s) {
                return false;
            }
            let key = (t.clone(), s.clone());
            if let Some(&result) = self.cache.get(&key) {
                return result;
            }
        }
        let result = self.is_def_eq_core(t, s);
        if cacheable {
            self.cache.insert((t.clone(), s.clone()), result);
            if result {
                self.equiv_manager.add_equiv(t, s);
            } else {
                self.equiv_manager.add_failure(t, s);
            }
        }
        result
    }
    /// Compare two binder bodies under a binder of domain type `dom` —
    /// Lean's `isDefEqBinding`: the binder is *opened* with a fresh, typed
    /// local free variable and the instantiated bodies are compared.
    ///
    /// Opening (rather than comparing the raw bodies with loose BVars under
    /// a binder-type stack) keeps every subterm locally closed, so every
    /// reduction step downstream can type what it needs *wherever the redex
    /// sits*. The decisive case is K-like iota during WHNF: in Lean core's
    /// `Std.IterStep.noConfusion` (new `ctorIdx`-based scheme, Lean ≥ 4.32)
    /// the term `(Eq.rec … (Eq.symm … : 0 = ctorIdx t) …).PULift.0 it out`
    /// is stuck unless the `Eq.rec` major — which mentions the Pi-bound
    /// `it`/`out` — can be typed; with loose BVars it cannot be (at any
    /// depth), and the declaration was wrongly rejected.
    ///
    /// The minted ids come from a dedicated high range and are never reused,
    /// so recorded types are stable and the def-eq/whnf caches stay sound.
    fn is_def_eq_under(&mut self, dom: &Expr, b1: &Expr, b2: &Expr) -> bool {
        let id = crate::FVarId(self.next_opened_fvar);
        self.next_opened_fvar += 1;
        self.record_fvar_type(id, dom.clone());
        let fv = Expr::FVar(id);
        let b1_open = instantiate(b1, &fv);
        let b2_open = instantiate(b2, &fv);
        self.is_def_eq(&b1_open, &b2_open)
    }
    fn is_def_eq_core(&mut self, t: &Expr, s: &Expr) -> bool {
        let t_whnf = self.reducer.whnf_env(t, self.env);
        let s_whnf = self.reducer.whnf_env(s, self.env);
        if t_whnf == s_whnf {
            return true;
        }
        if self.is_proof_irrelevant_eq(&t_whnf, &s_whnf) {
            return true;
        }
        let matched = match (&t_whnf, &s_whnf) {
            (Expr::Sort(l1), Expr::Sort(l2)) => level::is_equivalent(l1, l2),
            (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
            (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
            (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
                n1 == n2
                    && ls1.len() == ls2.len()
                    && ls1
                        .iter()
                        .zip(ls2.iter())
                        .all(|(l1, l2)| level::is_equivalent(l1, l2))
            }
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                if self.is_def_eq_app(&t_whnf, &s_whnf) {
                    return true;
                }
                self.is_def_eq(f1, f2) && self.is_def_eq(a1, a2)
            }
            (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
                self.is_def_eq(ty1, ty2) && self.is_def_eq_under(ty1, b1, b2)
            }
            (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
                self.is_def_eq(ty1, ty2) && self.is_def_eq_under(ty1, b1, b2)
            }
            (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
                self.is_def_eq(ty1, ty2)
                    && self.is_def_eq(v1, v2)
                    && self.is_def_eq_under(ty1, b1, b2)
            }
            (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
            (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
                n1 == n2 && i1 == i2 && self.is_def_eq(e1, e2)
            }
            (Expr::Lam(_, _, _, _), _) => self.try_eta_lhs(&t_whnf, &s_whnf),
            (_, Expr::Lam(_, _, _, _)) => self.try_eta_rhs(&t_whnf, &s_whnf),
            // Literal ↔ constructor-form bridge (Lean's `natLitExt` /
            // `strLitExt`): a `Nat`/`String` literal is defeq to its
            // constructor-form denotation (`Nat.zero`/`Nat.succ …`,
            // `String.mk (List Char)`). Only one side is a literal here (both
            // were handled above). If the bridge applies it decides; otherwise
            // fall through to lazy delta so a constant that *unfolds* to a
            // constructor form is still reached.
            _ => match self.try_lit_ext(&t_whnf, &s_whnf) {
                Some(b) => b,
                None => self.try_lazy_delta(&t_whnf, &s_whnf),
            },
        };
        if matched {
            return true;
        }
        // K-like reduction that needs the *local* binder context to type the
        // major premise: the Reducer's own K path (`to_ctor_when_k`) uses a
        // fresh `TypeChecker` and cannot type loose bound variables, so e.g.
        // `Eq.rec ... h` with `h` a Pi-bound hypothesis stays stuck there.
        // Here the binder-type stack can type it (Lean's kernel has the same
        // information via its local context).
        if let Some(t_red) = self.try_k_whnf(&t_whnf) {
            return self.is_def_eq(&t_red, &s_whnf);
        }
        if let Some(s_red) = self.try_k_whnf(&s_whnf) {
            return self.is_def_eq(&t_whnf, &s_red);
        }
        // Lean kernel ordering (`isDefEqCore`): once the structural and
        // lazy-delta paths have failed, try definitional eta for structures
        // (`isDefEqEtaStruct`) in both orientations, then unit-like equality
        // (`isDefEqUnitLike`). Proof irrelevance was already tried above,
        // mirroring Lean's order, so Prop-valued structures are handled
        // there first.
        if self.try_eta_struct(&t_whnf, &s_whnf) || self.try_eta_struct(&s_whnf, &t_whnf) {
            return true;
        }
        self.is_def_eq_unit_like(&t_whnf, &s_whnf)
    }
    /// K-like iota reduction driven by the *def-eq-local* typing context
    /// (Lean `to_cnstr_when_K` executed with the kernel's local context).
    ///
    /// If `e` is a stuck application of a recursor with the K flag, type its
    /// major premise via `Self::quick_infer_type` (which can see the binder
    /// stack, unlike the Reducer's fresh `TypeChecker`), verify the major's
    /// type is the K-inductive applied to its parameters/indices, and replace
    /// the major with the canonical constructor application so iota can fire.
    /// Returns the WHNF of the rewritten application, or `None` when anything
    /// does not line up or no progress is made (never a wrong reduction).
    fn try_k_whnf(&mut self, e: &Expr) -> Option<Expr> {
        let (head, args) = get_app_fn_args(e);
        let Expr::Const(rec_name, rec_levels) = head else {
            return None;
        };
        let rec_val = self.env.get_recursor_val(rec_name)?.clone();
        if !rec_val.k {
            return None;
        }
        let major_idx = rec_val.get_major_idx() as usize;
        if args.len() <= major_idx {
            return None;
        }
        let major = args[major_idx].clone();
        let major_ty = self.quick_infer_type(&major)?;
        let major_ty = self.reducer.whnf_env(&major_ty, self.env);
        let (ty_head, ty_args) = get_app_fn_args(&major_ty);
        let Expr::Const(ind_name, ind_levels) = ty_head else {
            return None;
        };
        if ind_name != rec_val.all.first()? {
            return None;
        }
        let iv = self.env.get_inductive_val(ind_name)?.clone();
        if ty_args.len() != (iv.num_params + iv.num_indices) as usize {
            return None;
        }
        let ctor_name = iv.ctors.first()?.clone();
        let params: Vec<Expr> = ty_args[..iv.num_params as usize]
            .iter()
            .map(|e| (*e).clone())
            .collect();
        let new_ctor = mk_app(Expr::Const(ctor_name, ind_levels.clone()), &params);
        if new_ctor == major {
            // Already canonical: the Reducer was stuck for a different
            // reason; nothing to gain (and rewriting would not terminate).
            return None;
        }
        let new_ty = self.quick_infer_type(&new_ctor)?;
        if !self.is_def_eq(&major_ty, &new_ty) {
            return None;
        }
        let mut new_args: Vec<Expr> = args.iter().map(|a| (*a).clone()).collect();
        new_args[major_idx] = new_ctor;
        let rebuilt = mk_app(Expr::Const(rec_name.clone(), rec_levels.clone()), &new_args);
        let reduced = self.reducer.whnf_env(&rebuilt, self.env);
        if reduced == *e {
            return None;
        }
        Some(reduced)
    }
    /// Try lazy delta reduction: unfold one side at a time.
    ///
    /// When comparing `f a1 ... an` with `g b1 ... bm` where f and g
    /// are both definitions, unfold the one with lower height first.
    fn try_lazy_delta(&mut self, t: &Expr, s: &Expr) -> bool {
        let t_head = get_app_fn(t);
        let s_head = get_app_fn(s);
        let t_hint = self.get_hint(t_head);
        let s_hint = self.get_hint(s_head);
        match (t_hint, s_hint) {
            (Some(th), Some(sh)) => {
                if th.height() <= sh.height() {
                    if let Some(t_unfolded) = self.unfold_definition(t) {
                        let t_whnf = self.reducer.whnf_env(&t_unfolded, self.env);
                        return self.is_def_eq(&t_whnf, s);
                    }
                }
                if let Some(s_unfolded) = self.unfold_definition(s) {
                    let s_whnf = self.reducer.whnf_env(&s_unfolded, self.env);
                    return self.is_def_eq(t, &s_whnf);
                }
                false
            }
            (Some(_), None) => {
                if let Some(t_unfolded) = self.unfold_definition(t) {
                    let t_whnf = self.reducer.whnf_env(&t_unfolded, self.env);
                    return self.is_def_eq(&t_whnf, s);
                }
                false
            }
            (None, Some(_)) => {
                if let Some(s_unfolded) = self.unfold_definition(s) {
                    let s_whnf = self.reducer.whnf_env(&s_unfolded, self.env);
                    return self.is_def_eq(t, &s_whnf);
                }
                false
            }
            (None, None) => false,
        }
    }
    /// Literal ↔ constructor-form definitional-equality bridge — Lean's
    /// `natLitExt?` / `strLitExt?`.
    ///
    /// Exactly one of `t` / `s` is a `Lit` (the both-literal case is decided
    /// structurally before this is reached). A `Nat`/`String` literal is
    /// definitionally equal to its constructor-form denotation:
    ///
    /// * `NatLit 0`  ≡ `Nat.zero`;
    /// * `NatLit (n+1)` ≡ `Nat.succ e`  when `NatLit n ≡ e`;
    /// * `StrLit s` ≡ `String.mk l`  when `l` is the `List Char` of `s`.
    ///
    /// Returns `Some(true)` / `Some(false)` when the bridge conclusively
    /// decides, and `None` when it does not apply (the other side is neither
    /// `Nat.zero`/`Nat.succ`/`String.mk`-headed nor a literal) — the caller
    /// then falls through to lazy delta, so a *constant* that only unfolds to a
    /// constructor form is still reached.
    ///
    /// Termination / no blow-up: the literal is **never** materialised into a
    /// succ-tower. Instead the *constructor-form* side is peeled one layer at a
    /// time and the literal is decremented (`pred`) each step, so the number of
    /// steps is bounded by `min(literal, actual constructor depth)`. A large
    /// literal against a shallow succ-tower (e.g. `10^9` vs `succ (FVar)`)
    /// therefore stops after one layer with a fast mismatch, never eagerly
    /// expanding the literal.
    fn try_lit_ext(&mut self, t: &Expr, s: &Expr) -> Option<bool> {
        match (t, s) {
            (Expr::Lit(crate::Literal::Nat(n)), other)
            | (other, Expr::Lit(crate::Literal::Nat(n))) => self.nat_lit_eq_term(n, other),
            (Expr::Lit(crate::Literal::Str(str_lit)), other)
            | (other, Expr::Lit(crate::Literal::Str(str_lit))) => {
                self.str_lit_eq_term(str_lit, other)
            }
            _ => None,
        }
    }
    /// Decide `NatLit n ≡ term` via Lean's `natLitExt?`. `term` is already in
    /// WHNF and is *not* a `Nat` literal (that case is decided structurally).
    fn nat_lit_eq_term(&mut self, n: &crate::bignat::BigNat, term: &Expr) -> Option<bool> {
        let head = get_app_fn(term);
        let head_name = match head {
            Expr::Const(name, _) => name.to_string(),
            // Non-constructor head (FVar, Sort, …): the bridge cannot decide;
            // defer to lazy delta (which ultimately fails / stays stuck) so a
            // mismatch is never reported as equal.
            _ => return None,
        };
        if n.is_zero() {
            // `NatLit 0`: equal iff the term is `Nat.zero`; a `Nat.succ …`
            // head is a definite mismatch.
            if head_name == "Nat.zero" {
                Some(true)
            } else if head_name == "Nat.succ" {
                Some(false)
            } else {
                None
            }
        } else {
            // `NatLit (n>0)`: equal iff the term is `Nat.succ e` with
            // `NatLit (n-1) ≡ e`; a `Nat.zero` head is a definite mismatch.
            match head_name.as_str() {
                "Nat.succ" => {
                    let args = get_app_args(term);
                    // `Nat.succ` must be applied to exactly one argument.
                    if args.len() != 1 {
                        return None;
                    }
                    let pred_lit = Expr::Lit(crate::Literal::Nat(n.pred()));
                    // Recurse through `is_def_eq`: the argument is re-WHNF'd
                    // (idempotent, cheap) and the both-literal fast path or a
                    // further succ-peel applies. Each step strictly decrements
                    // the literal, bounding the recursion by the succ depth.
                    Some(self.is_def_eq(&pred_lit, args[0]))
                }
                "Nat.zero" => Some(false),
                _ => None,
            }
        }
    }
    /// Decide `StrLit s ≡ term` via Lean's `try_string_lit_expansion`
    /// (type_checker.cpp, v4.32). `term` is already in WHNF-core form and is
    /// *not* a `String` literal (that case is decided structurally).
    ///
    /// * **v4.32 model** (`String`'s constructor is `ofByteArray`): the
    ///   extension fires exactly when `term` is an application headed by the
    ///   `String.ofList` *function* — Lean checks `app_fn(s) == String.ofList`
    ///   syntactically, before lazy delta unfolds it. The literal expands to
    ///   `String.ofList <chars>` (`string_lit_to_constructor`), is WHNF'd,
    ///   and the two sides go through full definitional equality.
    /// * **Old model** (a one-field `String.mk : List Char → String`, e.g.
    ///   the builtin env): the constructor-form list is peeled one
    ///   `List.cons` layer at a time against `s.chars()` — bounded by the
    ///   string length, never eagerly materialising the whole list on a
    ///   length mismatch. Gated on `String.mk` really being the sole
    ///   one-field constructor of `String` in this environment, so an
    ///   unrelated definition of that name can never bridge.
    fn str_lit_eq_term(&mut self, s: &str, term: &Expr) -> Option<bool> {
        let (head, args) = crate::expr_util::get_app_fn_args(term);
        let head_name = match head {
            Expr::Const(name, _) => name.to_string(),
            _ => return None,
        };
        if head_name == "String.ofList" && args.len() == 1 {
            let expansion = crate::reduce::iota::str_lit_expansion(s, self.env)?;
            let expansion_whnf = self.reducer.whnf_env(&expansion, self.env);
            // Re-entry guard: with the real `String.ofList` the WHNF is a
            // constructor application (or a stuck `ofList` application, when
            // `ofList` is opaque in a synthetic env), never a literal. If an
            // adversarial environment defines `ofList` so the expansion
            // WHNF-folds back to a string literal, deciding here would
            // recurse into this same bridge forever — decline instead and
            // let lazy delta take over (safe incompleteness).
            if matches!(expansion_whnf, Expr::Lit(crate::Literal::Str(_))) {
                return None;
            }
            return Some(self.is_def_eq(&expansion_whnf, term));
        }
        if head_name == "String.mk" && args.len() == 1 {
            let cv = self.env.get_constructor_val(match head {
                Expr::Const(name, _) => name,
                _ => return None,
            })?;
            if cv.induct.to_string() != "String" || cv.num_fields != 1 || cv.num_params != 0 {
                return None;
            }
            return Some(self.char_list_eq_str(&s.chars().collect::<Vec<char>>(), args[0]));
        }
        None
    }
    /// Decide whether the `List Char` expression `list` denotes exactly the
    /// characters `chars` (in order). Peels one `List.cons` per character; a
    /// `List.nil` head must coincide with the end of `chars`. Bounded by
    /// `chars.len()`.
    ///
    /// Constructor names are matched by their full dotted string
    /// (`List.nil` / `List.cons` / `Char.ofNat`), which is identical for both
    /// the flat and hierarchical naming styles a `String` literal can expand to
    /// (see `reduce::iota::str_lit_to_ctor`), so the bridge is naming-style
    /// agnostic without risking a collision with an unrelated `Foo.cons`.
    fn char_list_eq_str(&mut self, chars: &[char], list: &Expr) -> bool {
        let list_whnf = self.reducer.whnf_env(list, self.env);
        let (head, args) = crate::expr_util::get_app_fn_args(&list_whnf);
        let head_name = match head {
            Expr::Const(name, _) => name.to_string(),
            _ => return false,
        };
        match head_name.as_str() {
            // `List.nil Char`: matches iff there are no characters left.
            "List.nil" => chars.is_empty(),
            // `List.cons Char c rest`: `Char` type arg, head char, tail list.
            "List.cons" => {
                if chars.is_empty() || args.len() != 3 {
                    return false;
                }
                self.char_eq(chars[0], args[1]) && self.char_list_eq_str(&chars[1..], args[2])
            }
            _ => false,
        }
    }
    /// Decide whether the WHNF'd expression `ch` denotes the character `c`,
    /// i.e. `ch` is `Char.ofNat <code>` with `<code>` the Unicode scalar value
    /// of `c`. The code is compared as a `Nat` literal via `is_def_eq`, so any
    /// def-eq encoding of the same scalar value is accepted.
    fn char_eq(&mut self, c: char, ch: &Expr) -> bool {
        let ch_whnf = self.reducer.whnf_env(ch, self.env);
        let (head, args) = crate::expr_util::get_app_fn_args(&ch_whnf);
        match head {
            Expr::Const(name, _) if name.to_string() == "Char.ofNat" && args.len() == 1 => {
                let code = Expr::Lit(crate::Literal::Nat(crate::bignat::BigNat::from(c as u32)));
                self.is_def_eq(&code, args[0])
            }
            _ => false,
        }
    }
    /// Get the reducibility hint for an expression head.
    fn get_hint(&self, head: &Expr) -> Option<ReducibilityHint> {
        if let Expr::Const(name, _) = head {
            if let Some(ci) = self.env.find(name) {
                let hint = ci.reducibility_hint();
                if hint.should_unfold() {
                    return Some(hint);
                }
            }
        }
        None
    }
    /// Try to unfold a definition at the head of an expression.
    fn unfold_definition(&self, expr: &Expr) -> Option<Expr> {
        let head = get_app_fn(expr);
        if let Expr::Const(name, levels) = head {
            if let Some(ci) = self.env.find(name) {
                if let Some(val) = ci.value() {
                    let val_inst = if ci.level_params().is_empty() || levels.is_empty() {
                        val.clone()
                    } else {
                        crate::instantiate::instantiate_type_lparams(val, ci.level_params(), levels)
                    };
                    let args: Vec<Expr> = get_app_args(expr).into_iter().cloned().collect();
                    return Some(crate::expr_util::mk_app(val_inst, &args));
                }
            }
        }
        None
    }
    /// Try structural application equality.
    ///
    /// Compare `f a1 ... an` with `g b1 ... bn` by comparing
    /// the heads and all arguments.
    fn is_def_eq_app(&mut self, t: &Expr, s: &Expr) -> bool {
        let t_head = get_app_fn(t);
        let s_head = get_app_fn(s);
        let t_args = get_app_args(t);
        let s_args = get_app_args(s);
        if t_args.len() != s_args.len() {
            return false;
        }
        if !self.is_def_eq(t_head, s_head) {
            return false;
        }
        t_args
            .iter()
            .zip(s_args.iter())
            .all(|(a, b)| self.is_def_eq(a, b))
    }
    /// Quickly infer the type of a term without full elaboration.
    ///
    /// Handles the common structural cases needed for proof irrelevance:
    /// constants, applications of Pi-typed functions, Sort, and literals.
    /// Returns `None` for cases that require a full type checker (e.g. BVar, FVar).
    fn quick_infer_type(&mut self, expr: &Expr) -> Option<Expr> {
        match expr {
            Expr::Sort(l) => Some(Expr::Sort(crate::level::Level::succ(l.clone()))),
            Expr::Lit(crate::Literal::Nat(_)) => Some(Expr::Const(crate::Name::str("Nat"), vec![])),
            Expr::Lit(crate::Literal::Str(_)) => {
                Some(Expr::Const(crate::Name::str("String"), vec![]))
            }
            Expr::Const(name, levels) => {
                let ci = self.env.find(name)?;
                let raw_ty = ci.ty().clone();
                let params = ci.level_params().to_vec();
                if params.is_empty() || levels.is_empty() {
                    Some(raw_ty)
                } else {
                    Some(crate::instantiate::instantiate_type_lparams(
                        &raw_ty, &params, levels,
                    ))
                }
            }
            Expr::App(f, a) => {
                let f_ty = self.quick_infer_type(f)?;
                let f_ty_whnf = self.reducer.whnf_env(&f_ty, self.env);
                if let Expr::Pi(_, _, _, body) = f_ty_whnf {
                    Some(crate::subst::instantiate(&body, a))
                } else {
                    None
                }
            }
            // A loose bound variable is typed from the binder-type stack the
            // checker maintains while descending under binders. Entry types
            // are expressed *outside* their binder, so the result is lifted
            // past the binders in between (idx + 1 of them).
            Expr::BVar(idx) => {
                let i = *idx as usize;
                let len = self.binder_types.len();
                if i < len {
                    let ty = self.binder_types[len - 1 - i].clone();
                    Some(crate::expr_util::lift_loose_bvars(&ty, *idx + 1, 0))
                } else {
                    None
                }
            }
            // A free variable is typed from the surrounding type checker's
            // local context (recorded via [`Self::record_fvar_type`]). FVar
            // types are closed with respect to the binder stack, so no
            // lifting is needed.
            Expr::FVar(id) => self.fvar_types.get(&id.0).cloned(),
            Expr::Lam(bi, name, dom, body) => {
                self.binder_types.push((**dom).clone());
                let body_ty = self.quick_infer_type(body);
                self.binder_types.pop();
                Some(Expr::Pi(
                    *bi,
                    name.clone(),
                    dom.clone(),
                    Node::new(body_ty?),
                ))
            }
            Expr::Pi(_, _, dom, cod) => {
                let dom_ty = self.quick_infer_type(dom)?;
                self.binder_types.push((**dom).clone());
                let cod_ty = self.quick_infer_type(cod);
                self.binder_types.pop();
                let cod_ty = cod_ty?;
                let dom_whnf = self.reducer.whnf_env(&dom_ty, self.env);
                let cod_whnf = self.reducer.whnf_env(&cod_ty, self.env);
                match (dom_whnf, cod_whnf) {
                    (Expr::Sort(l1), Expr::Sort(l2)) => {
                        Some(Expr::Sort(crate::level::Level::imax(l1, l2)))
                    }
                    _ => None,
                }
            }
            Expr::Let(_, _ty, val, body) => {
                let body_subst = crate::subst::instantiate(body, val);
                self.quick_infer_type(&body_subst)
            }
            Expr::Proj(struct_name, idx, inner) => {
                let inner_ty = self.quick_infer_type(inner)?;
                self.quick_proj_type(struct_name, *idx, inner, &inner_ty)
            }
        }
    }
    /// Infer the type of `Proj(struct_name, idx, inner)` given `inner`'s type.
    ///
    /// Mirrors `TypeChecker::infer_proj_field_type`: instantiates the
    /// constructor's type with the universe levels and inductive parameters
    /// read off `inner`'s (whnf'd) type, substitutes earlier fields with
    /// projections of `inner`, and returns the `idx`-th Pi domain. Returns
    /// `None` whenever anything does not line up — this is a best-effort
    /// helper; the full `TypeChecker` reports typed errors instead.
    fn quick_proj_type(
        &mut self,
        struct_name: &crate::Name,
        idx: u32,
        inner: &Expr,
        inner_ty: &Expr,
    ) -> Option<Expr> {
        if !self.env.is_structure_like(struct_name) {
            return None;
        }
        let ind_val = self.env.get_inductive_val(struct_name)?.clone();
        let ctor_name = ind_val.ctors.first()?.clone();
        let ctor_val = self.env.get_constructor_val(&ctor_name)?.clone();
        if idx >= ctor_val.num_fields {
            return None;
        }
        let inner_ty_whnf = self.reducer.whnf_env(inner_ty, self.env);
        let levels: Vec<Level> = match get_app_fn(&inner_ty_whnf) {
            Expr::Const(name, lvls) if name == struct_name => lvls.clone(),
            _ => return None,
        };
        let ty_args: Vec<Expr> = get_app_args(&inner_ty_whnf).into_iter().cloned().collect();
        if ty_args.len() != ind_val.num_params as usize {
            return None;
        }
        let level_params = &ind_val.common.level_params;
        let mut cur_ty = if level_params.is_empty() || levels.is_empty() {
            ctor_val.common.ty.clone()
        } else {
            instantiate_type_lparams(&ctor_val.common.ty, level_params, &levels)
        };
        for param in &ty_args {
            match cur_ty {
                Expr::Pi(_, _, _, body) => cur_ty = instantiate(&body, param),
                _ => return None,
            }
        }
        for j in 0..idx {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let field_val = Expr::Proj(struct_name.clone(), j, Node::new(inner.clone()));
                    cur_ty = instantiate(&body, &field_val);
                }
                _ => return None,
            }
        }
        match cur_ty {
            Expr::Pi(_, _, dom, _) => Some((*dom).clone()),
            _ => None,
        }
    }
    /// Check proof irrelevance: two proofs of the same Prop are definitionally equal.
    ///
    /// Two terms `t` and `s` are proof-irrelevantly equal when:
    /// - Both have types that reduce to `Sort 0` (i.e., they live in `Prop`)
    /// - Their types are definitionally equal (they prove the same proposition)
    fn is_proof_irrelevant_eq(&mut self, t: &Expr, s: &Expr) -> bool {
        if !self.proof_irrelevance {
            return false;
        }
        let ty_t = match self.quick_infer_type(t) {
            Some(ty) => ty,
            None => return false,
        };
        let ty_ty_t = match self.quick_infer_type(&ty_t) {
            Some(ty) => ty,
            None => return false,
        };
        let ty_ty_t_whnf = self.reducer.whnf_env(&ty_ty_t, self.env);
        // Prop test must be up to universe equivalence, not syntactic: a proof
        // whose sort level is e.g. `imax(u, 0)` normalizes to `0` (Prop) but is
        // not the literal `Zero`. Use the complete `is_equivalent` check.
        if !matches!(&ty_ty_t_whnf, Expr::Sort(l) if level::is_equivalent(l, &Level::zero())) {
            return false;
        }
        let ty_s = match self.quick_infer_type(s) {
            Some(ty) => ty,
            None => return false,
        };
        let ty_ty_s = match self.quick_infer_type(&ty_s) {
            Some(ty) => ty,
            None => return false,
        };
        let ty_ty_s_whnf = self.reducer.whnf_env(&ty_ty_s, self.env);
        if !matches!(&ty_ty_s_whnf, Expr::Sort(l) if level::is_equivalent(l, &Level::zero())) {
            return false;
        }
        let ty_t_whnf = self.reducer.whnf_env(&ty_t, self.env);
        let ty_s_whnf = self.reducer.whnf_env(&ty_s, self.env);
        self.is_def_eq(&ty_t_whnf, &ty_s_whnf)
    }
    /// Try function eta when the left side is a lambda and the right is not.
    ///
    /// Fast path (contraction): `λ x. f x` =?= `g` when `f` doesn't use `x`,
    /// then `f =?= g`. Fallback (Lean `tryEtaExpansionCore`): eta-expand the
    /// non-lambda side by one binder from its Pi type and recurse, which
    /// handles multi-binder telescopes such as `(fun x y => g x y) =?= g`.
    fn try_eta_lhs(&mut self, t: &Expr, s: &Expr) -> bool {
        if let Expr::Lam(_, _, _, body) = t {
            if let Expr::App(f, a) = body.as_ref() {
                if let Expr::BVar(0) = **a {
                    if !has_loose_bvar(f, 0) {
                        let f_shifted =
                            crate::subst::instantiate(f, &Expr::FVar(crate::FVarId(u64::MAX)));
                        if self.is_def_eq(&f_shifted, s) {
                            return true;
                        }
                    }
                }
            }
        }
        if let Some(s_expanded) = self.eta_expand_one(s) {
            return self.is_def_eq(t, &s_expanded);
        }
        false
    }
    /// Try function eta when the right side is a lambda and the left is not.
    ///
    /// Symmetric to [`Self::try_eta_lhs`].
    fn try_eta_rhs(&mut self, t: &Expr, s: &Expr) -> bool {
        if let Expr::Lam(_, _, _, body) = s {
            if let Expr::App(f, a) = body.as_ref() {
                if let Expr::BVar(0) = **a {
                    if !has_loose_bvar(f, 0) {
                        let f_shifted =
                            crate::subst::instantiate(f, &Expr::FVar(crate::FVarId(u64::MAX)));
                        if self.is_def_eq(t, &f_shifted) {
                            return true;
                        }
                    }
                }
            }
        }
        if let Some(t_expanded) = self.eta_expand_one(t) {
            return self.is_def_eq(&t_expanded, s);
        }
        false
    }
    /// Eta-expand a non-lambda expression by one binder (one step of Lean's
    /// `etaExpand`).
    ///
    /// Given `s` whose inferred type whnf's to `Pi (a : d), b`, returns
    /// `fun (a : d) => s a`, lifting loose bound variables in `s` under the
    /// new binder. Returns `None` if `s` is a lambda or its type cannot be
    /// determined to be a Pi.
    fn eta_expand_one(&mut self, s: &Expr) -> Option<Expr> {
        if matches!(s, Expr::Lam(_, _, _, _)) {
            return None;
        }
        let s_ty = self.quick_infer_type(s)?;
        let s_ty_whnf = self.reducer.whnf_env(&s_ty, self.env);
        if let Expr::Pi(bi, name, dom, _) = s_ty_whnf {
            let s_lifted = crate::expr_util::lift_loose_bvars(s, 1, 0);
            Some(Expr::Lam(
                bi,
                name,
                dom,
                Node::new(Expr::App(Node::new(s_lifted), Node::new(Expr::BVar(0)))),
            ))
        } else {
            None
        }
    }
    /// Definitional eta for structures (Lean `isDefEqEtaStruct`).
    ///
    /// If `s` is a fully applied constructor application `S.mk ps fs` of a
    /// structure-like inductive `S` (exactly one constructor, no indices,
    /// not recursive) and `t` is not itself a constructor application, then
    /// eta-expand `t` to `S.mk ps' t.0 ... t.(n-1)` — with parameters `ps'`
    /// and universe levels read off `t`'s (whnf'd) type — and recurse.
    /// Application congruence then compares the constructor levels, the
    /// parameters, and each field `f_i` against `Proj(S, i, t)`, which
    /// subsumes Lean's `isDefEq (inferType t) (inferType s)` gate.
    ///
    /// Only fires when `t`'s type whnf's to the structure type `S ps'`;
    /// otherwise the pair is left for other rules (never a wrong result).
    fn try_eta_struct(&mut self, t: &Expr, s: &Expr) -> bool {
        let (ctor_name, ctor_val) = match get_app_fn(s) {
            Expr::Const(name, _) => match self.env.get_constructor_val(name) {
                Some(cv) => (name.clone(), cv.clone()),
                None => return false,
            },
            _ => return false,
        };
        if get_app_args(s).len() != (ctor_val.num_params + ctor_val.num_fields) as usize {
            return false;
        }
        if !self.env.is_structure_like(&ctor_val.induct) {
            return false;
        }
        // Constructor-vs-constructor pairs are already handled by
        // application congruence; expanding here would be redundant.
        if let Expr::Const(t_head, _) = get_app_fn(t) {
            if self.env.is_constructor(t_head) {
                return false;
            }
        }
        let t_ty = match self.quick_infer_type(t) {
            Some(ty) => ty,
            None => return false,
        };
        let t_ty_whnf = self.reducer.whnf_env(&t_ty, self.env);
        let levels: Vec<Level> = match get_app_fn(&t_ty_whnf) {
            Expr::Const(ind_name, lvls) if *ind_name == ctor_val.induct => lvls.clone(),
            _ => return false,
        };
        let params: Vec<Expr> = get_app_args(&t_ty_whnf).into_iter().cloned().collect();
        if params.len() != ctor_val.num_params as usize {
            return false;
        }
        let mut expansion = Expr::Const(ctor_name, levels);
        for param in params {
            expansion = Expr::App(Node::new(expansion), Node::new(param));
        }
        for i in 0..ctor_val.num_fields {
            let field = Expr::Proj(ctor_val.induct.clone(), i, Node::new(t.clone()));
            expansion = Expr::App(Node::new(expansion), Node::new(field));
        }
        self.is_def_eq(s, &expansion)
    }
    /// Unit-like definitional equality (Lean `isDefEqUnitLike`).
    ///
    /// If `t`'s type whnf's to `S ps` where `S` is structure-like and its
    /// single constructor has no fields beyond the parameters, then any two
    /// elements of that type are definitionally equal: it suffices that the
    /// types agree, i.e. `t : S ps` and `s`'s type is def-eq to `S ps`.
    fn is_def_eq_unit_like(&mut self, t: &Expr, s: &Expr) -> bool {
        let t_ty = match self.quick_infer_type(t) {
            Some(ty) => ty,
            None => return false,
        };
        let t_ty_whnf = self.reducer.whnf_env(&t_ty, self.env);
        let ind_name = match get_app_fn(&t_ty_whnf) {
            Expr::Const(name, _) => name.clone(),
            _ => return false,
        };
        if !self.env.is_structure_like(&ind_name) {
            return false;
        }
        let ctor_name = match self
            .env
            .get_inductive_val(&ind_name)
            .and_then(|iv| iv.ctors.first())
        {
            Some(name) => name.clone(),
            None => return false,
        };
        match self.env.get_constructor_val(&ctor_name) {
            Some(cv) if cv.num_fields == 0 => {}
            _ => return false,
        }
        let s_ty = match self.quick_infer_type(s) {
            Some(ty) => ty,
            None => return false,
        };
        self.is_def_eq(&t_ty_whnf, &s_ty)
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
