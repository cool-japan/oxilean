//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Level, Name};

use std::collections::HashMap;
use std::rc::Rc;

use super::functions::{
    collect_consts, collect_fvars, count_bvar_occ, has_loose_bvar, max_loose_bvar_impl,
};

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
/// Binder display annotation.
///
/// Determines how arguments are displayed and inferred.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BinderInfo {
    /// Default explicit argument: (x : α)
    Default,
    /// Implicit argument: {x : α}
    Implicit,
    /// Strict implicit: ⦃x : α⦄
    StrictImplicit,
    /// Instance implicit (type class): [x : α]
    InstImplicit,
}
impl BinderInfo {
    /// Check if this is an explicit (default) binder.
    pub fn is_explicit(self) -> bool {
        matches!(self, BinderInfo::Default)
    }
    /// Check if this is an implicit binder.
    pub fn is_implicit(self) -> bool {
        matches!(self, BinderInfo::Implicit | BinderInfo::StrictImplicit)
    }
    /// Check if this is an instance-implicit binder.
    pub fn is_inst_implicit(self) -> bool {
        matches!(self, BinderInfo::InstImplicit)
    }
    /// Return the opening delimiter for this binder.
    pub fn open_delim(self) -> &'static str {
        match self {
            BinderInfo::Default => "(",
            BinderInfo::Implicit => "{",
            BinderInfo::StrictImplicit => "⦃",
            BinderInfo::InstImplicit => "[",
        }
    }
    /// Return the closing delimiter for this binder.
    pub fn close_delim(self) -> &'static str {
        match self {
            BinderInfo::Default => ")",
            BinderInfo::Implicit => "}",
            BinderInfo::StrictImplicit => "⦄",
            BinderInfo::InstImplicit => "]",
        }
    }
}
/// Unique identifier for free variables.
///
/// Free variables are used during elaboration and type checking to represent
/// local hypotheses.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FVarId(pub u64);
impl FVarId {
    /// Create a new free variable ID.
    pub fn new(id: u64) -> Self {
        FVarId(id)
    }
}
impl FVarId {
    /// Return the underlying integer ID.
    pub fn raw(self) -> u64 {
        self.0
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
/// Literal values supported natively.
///
/// Exactly the two literal kinds of the Lean 4 kernel: arbitrary-precision
/// natural numbers (`natVal`) and UTF-8 strings (`strVal`). There is
/// deliberately **no** integer literal — Lean encodes integers as
/// `Int.ofNat` / `Int.negSucc` over Nat literals, and a signed fixed-width
/// literal inside the TCB previously carried wrapping arithmetic.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Literal {
    /// Natural number literal (arbitrary precision).
    Nat(crate::bignat::BigNat),
    /// String literal.
    Str(String),
}
impl Literal {
    /// Build a Nat literal from a small value (ergonomic constructor).
    pub fn nat(n: u64) -> Self {
        Literal::Nat(crate::bignat::BigNat::from(n))
    }
    /// Build a String literal.
    pub fn string(s: impl Into<String>) -> Self {
        Literal::Str(s.into())
    }
    /// Return true if this is a natural number literal.
    pub fn is_nat(&self) -> bool {
        matches!(self, Literal::Nat(_))
    }
    /// Return true if this is a string literal.
    pub fn is_str(&self) -> bool {
        matches!(self, Literal::Str(_))
    }
    /// Extract the natural number value if it fits in `u64` (best effort;
    /// returns `None` for larger values — use [`Literal::as_bignat`] for the
    /// exact value).
    pub fn as_nat(&self) -> Option<u64> {
        if let Literal::Nat(n) = self {
            n.to_u64()
        } else {
            None
        }
    }
    /// Extract the exact natural number value, or `None`.
    pub fn as_bignat(&self) -> Option<&crate::bignat::BigNat> {
        if let Literal::Nat(n) = self {
            Some(n)
        } else {
            None
        }
    }
    /// Extract the string value, or `None`.
    pub fn as_str(&self) -> Option<&str> {
        if let Literal::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    /// Return the Lean type name for this literal.
    pub fn type_name(&self) -> &'static str {
        match self {
            Literal::Nat(_) => "Nat",
            Literal::Str(_) => "String",
        }
    }
}
impl From<crate::bignat::BigNat> for Literal {
    fn from(n: crate::bignat::BigNat) -> Self {
        Literal::Nat(n)
    }
}
impl From<u64> for Literal {
    fn from(n: u64) -> Self {
        Literal::nat(n)
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
    /// Replaces the top two values, `b` on top of `a`, with `op(a, b)` and
    /// returns it.  Returns `None`, leaving the stack unchanged, if it holds
    /// fewer than two values.
    fn apply_binary(&mut self, op: impl FnOnce(i64, i64) -> i64) -> Option<i64> {
        let (rest_len, a, b) = match self.stack[..] {
            [ref rest @ .., a, b] => (rest.len(), a, b),
            _ => return None,
        };
        let value = op(a, b);
        self.stack.truncate(rest_len);
        self.stack.push(value);
        Some(value)
    }
    /// Adds the top two values and returns the sum, which replaces them.
    /// Returns `None`, leaving the stack unchanged, if it holds fewer than
    /// two values.
    pub fn add(&mut self) -> Option<i64> {
        self.apply_binary(|a, b| a + b)
    }
    /// Subtracts top from second and returns the difference, which replaces
    /// them.  Returns `None`, leaving the stack unchanged, if it holds fewer
    /// than two values.
    pub fn sub(&mut self) -> Option<i64> {
        self.apply_binary(|a, b| a - b)
    }
    /// Multiplies the top two values and returns the product, which replaces
    /// them.  Returns `None`, leaving the stack unchanged, if it holds fewer
    /// than two values.
    pub fn mul(&mut self) -> Option<i64> {
        self.apply_binary(|a, b| a * b)
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
/// A child edge in an [`Expr`]: a reference-counted subterm together with
/// the facts about it that the kernel needs in O(1) (structural sharing,
/// wave5 Stage E; the hash and the three flags added for 0.1.4).
///
/// * `range` — the subterm's `looseBVarRange`: `1 + the largest loose de Bruijn
///   index in the subterm` (0 = closed). It lets a substitution or lift decide
///   in O(1) whether a subtree is affected — `range <= depth` means "no loose
///   bvar can be touched, share it unchanged" — without traversing the subtree.
/// * `cost` — the size of the subterm counted as a TREE (a shared subterm once
///   per path to it), saturating at [`REBUILD_COST_MAX`]: one unit per node
///   plus one more per leaf, so `cost(leaf) = 2` and
///   `cost(internal) = 1 + Σ cost(children)`. The reduction and def-eq caches
///   read it to keep giant terms out (`expr_util::expr_size_within`). Through
///   0.1.4's first performance pass this was also what a substitution charged
///   to the per-declaration budget for a subtree it shared instead of
///   rebuilding; sharing a subtree now charges one unit (see [`crate::fuel`]).
/// * `hash` — the hash of the subterm's structure (the kernel-private
///   `structural_hash` module documents it). `Hash` writes it instead of
///   walking the subterm, and `==` uses it as a fast negative test.
/// * three flags — whether the subterm contains a free variable, a universe
///   parameter, a universe metavariable. They let the operations that only
///   concern those (abstracting a free variable, instantiating universe
///   parameters, the universe hygiene check of a declaration) pass over
///   everything else without visiting it.
///
/// All of them are computed once, bottom-up, in [`Node::new`] from the
/// subterm's own immediate children (also `Node`s), so construction stays
/// O(nodes) and none of them ever needs a traversal.
///
/// # Layout
///
/// The facts live in the shared allocation, next to the subterm; a `Node` is
/// one pointer. (Through 0.1.3 `range` and `cost` sat on the edge beside the
/// pointer, and the allocation held the bare `Expr`.) An allocation is the
/// two reference counts, 16 bytes of facts and a 40-byte `Expr` — 72 bytes
/// where the old layout used 80, with the hash and the flags added (the
/// flags share a word with `cost`) — and an `Expr` held by value, as every
/// cache key is, is 40 bytes where it was 64. Reading a fact dereferences the
/// edge, which costs nothing extra where it matters: a substitution that
/// skips a subtree shares it by bumping the reference count stored right
/// beside the facts, and one that does not skip it walks into it.
///
/// `Node` is a transparent wrapper: it `Deref`s to the inner [`Expr`], and its
/// `PartialEq`/`Eq`/`Hash` are those of the subterm (so the instances on
/// `Expr` keep exactly their structural meaning). The cached facts are
/// functions of the subterm and so never change what is equal to what.
#[derive(Clone)]
pub struct Node {
    rc: Rc<Shared>,
}

/// The shared allocation behind a [`Node`]: a subterm and the facts cached
/// about it.
///
/// Invariant: `range` and `hash` are `expr.range()` and
/// `expr.structural_hash()`, and `meta` is `expr.rebuild_cost()` in its low
/// 29 bits with `expr.flags()` above them. They are set in [`Node::new`], the
/// only place a `Shared` is built, and never changed — a `Node` gives no
/// mutable access to its allocation.
struct Shared {
    range: u32,
    /// The tree size (`cost`, at most [`REBUILD_COST_MAX`]) and the three
    /// content flags, in one word.
    meta: u32,
    hash: u64,
    /// `expr.depth()`, in a build that tracks the depth of terms
    /// ([`crate::fuel::TERM_DEPTH_TRACKED`]).
    #[cfg(feature = "term-depth")]
    depth: u32,
    expr: Expr,
}

/// The largest value [`Expr::rebuild_cost`] and [`Node::rebuild_cost`]
/// report: the count saturates here. (It shares a 32-bit word with the three
/// content flags, which take the bits above it — see the `node_facts` module
/// of this directory; no cache keeps a term anywhere near this size.)
pub const REBUILD_COST_MAX: u32 = (1 << 29) - 1;

impl Node {
    /// Wrap an [`Expr`] as a child edge, caching its `looseBVarRange`, its
    /// tree size, its content flags and the hash of its structure. O(1): each
    /// is read off the cached facts of `e`'s immediate children (for a `Sort`
    /// or a `Const`, off its universe levels).
    ///
    /// In a build that tracks the depth of terms
    /// ([`crate::fuel::TERM_DEPTH_TRACKED`]) the depth is cached too, and a
    /// subterm deeper than the limit of
    /// [`crate::fuel::set_term_depth_limit`] spends the budget of the
    /// declaration being checked.
    #[inline]
    pub fn new(e: Expr) -> Node {
        #[cfg(feature = "term-depth")]
        let depth = {
            let depth = e.depth();
            crate::fuel::note_term_depth(depth);
            depth
        };
        Node {
            rc: Rc::new(Shared {
                range: e.range(),
                meta: e.rebuild_cost() | e.flags(),
                hash: e.structural_hash(),
                #[cfg(feature = "term-depth")]
                depth,
                expr: e,
            }),
        }
    }

    /// The cached depth of this subterm (O(1)); see [`Expr::depth`].
    #[cfg(feature = "term-depth")]
    #[inline]
    #[must_use]
    pub(crate) fn depth(&self) -> u32 {
        self.rc.depth
    }

    /// The cached `looseBVarRange` of this subterm (O(1)).
    #[inline]
    #[must_use]
    pub fn range(&self) -> u32 {
        self.rc.range
    }

    /// The cached size of this subterm counted as a tree (O(1)); see
    /// [`Expr::rebuild_cost`].
    #[inline]
    #[must_use]
    pub fn rebuild_cost(&self) -> u32 {
        self.rc.meta & REBUILD_COST_MAX
    }

    /// The cached content flags of this subterm (O(1)): the bits of the
    /// shared word above the tree size.
    #[inline]
    #[must_use]
    pub(super) fn flags(&self) -> u32 {
        self.rc.meta & !REBUILD_COST_MAX
    }

    /// Whether this allocation has more than one owner — another parent in
    /// the same term, a cache, another term. A subterm that is reached twice
    /// while one term is walked always has; the walks that rebuild a term
    /// keep the result for such a subterm and reuse it the second time, so a
    /// shared subterm stays shared in what they build.
    #[inline]
    #[must_use]
    pub(crate) fn is_shared(&self) -> bool {
        Rc::strong_count(&self.rc) > 1
    }

    /// The cached hash of this subterm's structure (O(1)): the value
    /// [`Expr::structural_hash`] computes for the subterm.
    #[inline]
    #[must_use]
    pub(crate) fn structural_hash(&self) -> u64 {
        self.rc.hash
    }

    /// The address of the shared subterm. Two nodes return the same address
    /// exactly when they share one allocation, for as long as either is
    /// alive — so the address can key a memo table over a term whose subterms
    /// are shared, visiting each shared subterm once.
    ///
    /// The pointer is for comparison only. (This replaces `rc()`, which
    /// exposed the `Rc<Expr>` the edge used to hold.)
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const Expr {
        &self.rc.expr
    }

    /// Whether two nodes share one allocation (which implies they are equal;
    /// equal nodes need not share one).
    #[inline]
    #[must_use]
    pub fn ptr_eq(this: &Node, other: &Node) -> bool {
        Rc::ptr_eq(&this.rc, &other.rc)
    }
}

impl std::ops::Deref for Node {
    type Target = Expr;
    #[inline]
    fn deref(&self) -> &Expr {
        &self.rc.expr
    }
}

impl PartialEq for Node {
    /// Structural equality of the two subterms, decided without comparing
    /// them when it can be: one allocation is equal to itself, and two
    /// subterms whose cached hashes differ are different (equal terms always
    /// hash equal). Only when the hashes agree are the subterms compared.
    #[inline]
    fn eq(&self, other: &Node) -> bool {
        Rc::ptr_eq(&self.rc, &other.rc)
            || (self.rc.hash == other.rc.hash && self.rc.expr == other.rc.expr)
    }
}
impl Eq for Node {}
impl std::hash::Hash for Node {
    /// Writes the cached structural hash: what `Expr`'s `Hash` writes for the
    /// subterm (as `Borrow<Expr>` requires), without walking it.
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.rc.hash);
    }
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.rc.expr, f)
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.rc.expr, f)
    }
}
impl AsRef<Expr> for Node {
    #[inline]
    fn as_ref(&self) -> &Expr {
        &self.rc.expr
    }
}
impl std::borrow::Borrow<Expr> for Node {
    #[inline]
    fn borrow(&self) -> &Expr {
        &self.rc.expr
    }
}

/// Core expression type.
///
/// This is the heart of the kernel. All terms in the type theory are
/// represented as `Expr` values.
///
/// Sub-expressions are held behind [`Node`]s (structural sharing, wave5): an
/// export DAG id referenced by many parents materializes to a single shared
/// node, and `clone` is an O(1) refcount bump rather than a deep copy. The
/// kernel is single-threaded (the verify pipeline runs on one spawned thread
/// and the wasm target is single-threaded), so the sharing is by `Rc` — not
/// `Arc`.
///
/// `Clone` is implemented by hand (not derived) so each clone charges one unit
/// of the deterministic per-declaration resource fuel — see [`crate::fuel`].
/// A clone copies this one node and shares its children (the sub-`clone`s are
/// refcount bumps), so the unit is for the call, not for the size of the
/// term; the walks that build new terms charge for the subterms they reach.
///
/// `Hash` is implemented by hand too: it writes the term's structural hash, one
/// 64-bit value computed from the hashes the children cache, where the derived
/// implementation walked the whole term. `PartialEq` is derived; the children
/// compare through [`Node`]'s equality, which consults the cached hashes
/// first.
#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    /// Sort u — universe.
    ///
    /// `Prop` is `Sort(Level::Zero)`.
    /// `Type` is `Sort(Level::Succ(Level::Zero))`.
    Sort(Level),
    /// Bound variable (de Bruijn index, 0-based).
    ///
    /// `BVar(0)` refers to the innermost binder.
    /// `BVar(n)` refers to the n-th enclosing binder.
    BVar(u32),
    /// Free variable (locally nameless).
    ///
    /// Used during type checking to represent local hypotheses.
    FVar(FVarId),
    /// Named constant with universe level instantiation.
    ///
    /// `Const(name, levels)` represents a global constant with its
    /// universe parameters instantiated.
    Const(Name, Vec<Level>),
    /// Function application: `f a`.
    ///
    /// Application is always binary. `f a b` is `App(App(f, a), b)`.
    App(Node, Node),
    /// Lambda abstraction: `λ (x : type), body`.
    Lam(BinderInfo, Name, Node, Node),
    /// Dependent function type: `Π (x : type), body`.
    ///
    /// Non-dependent arrows `A → B` are represented as
    /// `Pi(Default, _, A, B)` where the body doesn't use `BVar(0)`.
    Pi(BinderInfo, Name, Node, Node),
    /// Let binding: `let x : type := val in body`.
    Let(Name, Node, Node, Node),
    /// Literal value (Nat or String).
    Lit(Literal),
    /// Structure projection: `projName.idx struct`.
    Proj(Name, u32, Node),
}

impl Expr {
    /// This node's `looseBVarRange` — `1 + the largest loose de Bruijn index`,
    /// or 0 if closed. O(1): it reads the immediate children's cached ranges
    /// (see [`Node`]) and combines them, accounting for the binders each child
    /// sits under.
    #[inline]
    #[must_use]
    pub fn range(&self) -> u32 {
        match self {
            Expr::BVar(n) => n.saturating_add(1),
            Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
            Expr::App(f, a) => f.range().max(a.range()),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                ty.range().max(body.range().saturating_sub(1))
            }
            Expr::Let(_, ty, val, body) => ty
                .range()
                .max(val.range())
                .max(body.range().saturating_sub(1)),
            Expr::Proj(_, _, inner) => inner.range(),
        }
    }

    /// The size of this term counted as a TREE — a shared subterm once per
    /// path to it — O(1) from the children's cached sizes: one unit per node
    /// and one more per leaf, so `cost(leaf) = 2` and
    /// `cost(internal) = 1 + Σ cost(children)`, saturating at
    /// [`REBUILD_COST_MAX`]. It is what rebuilding the term node by node,
    /// with no sharing, would cost; the caches read it to keep giant terms
    /// out.
    #[inline]
    #[must_use]
    pub fn rebuild_cost(&self) -> u32 {
        // Each child cost is at most `REBUILD_COST_MAX` (2^29 - 1), so the sum
        // of up to three of them plus one cannot overflow a `u32`.
        let total = match self {
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 2,
            Expr::App(f, a) => 1 + f.rebuild_cost() + a.rebuild_cost(),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                1 + ty.rebuild_cost() + body.rebuild_cost()
            }
            Expr::Let(_, ty, val, body) => {
                1 + ty.rebuild_cost() + val.rebuild_cost() + body.rebuild_cost()
            }
            Expr::Proj(_, _, inner) => 1 + inner.rebuild_cost(),
        };
        total.min(REBUILD_COST_MAX)
    }

    /// The hash of this term's structure (see [`crate::structural_hash`]).
    ///
    /// O(1) in the size of the term: it combines this node's own fields with
    /// the hashes its children cache ([`Node`], `Name` and `Level` each store
    /// theirs), so nothing below the immediate children is visited. A literal
    /// is the one exception — its limbs or bytes are hashed in full, as there
    /// is nothing beneath a literal to cache them on.
    ///
    /// Every field `==` compares takes part — binder names and binder
    /// annotations included, since `Expr` equality distinguishes them — so two
    /// equal terms always have equal hashes, and the hash separates what the
    /// derived `Hash` separated.
    #[must_use]
    pub(crate) fn structural_hash(&self) -> u64 {
        use crate::structural_hash::{
            mix, mix_bytes, mix_words, EXPR_APP, EXPR_BVAR, EXPR_CONST, EXPR_FVAR, EXPR_LAM,
            EXPR_LET, EXPR_LIT_NAT, EXPR_LIT_STR, EXPR_PI, EXPR_PROJ, EXPR_SORT,
        };
        match self {
            Expr::Sort(level) => mix(EXPR_SORT, level.structural_hash()),
            Expr::BVar(index) => mix(EXPR_BVAR, u64::from(*index)),
            Expr::FVar(id) => mix(EXPR_FVAR, id.0),
            Expr::Const(name, levels) => levels
                .iter()
                .fold(mix(EXPR_CONST, name.structural_hash()), |hash, level| {
                    mix(hash, level.structural_hash())
                }),
            Expr::App(f, a) => mix(mix(EXPR_APP, f.structural_hash()), a.structural_hash()),
            Expr::Lam(bi, name, ty, body) => mix(
                mix(
                    mix(mix(EXPR_LAM, binder_info_code(*bi)), name.structural_hash()),
                    ty.structural_hash(),
                ),
                body.structural_hash(),
            ),
            Expr::Pi(bi, name, ty, body) => mix(
                mix(
                    mix(mix(EXPR_PI, binder_info_code(*bi)), name.structural_hash()),
                    ty.structural_hash(),
                ),
                body.structural_hash(),
            ),
            Expr::Let(name, ty, val, body) => mix(
                mix(
                    mix(mix(EXPR_LET, name.structural_hash()), ty.structural_hash()),
                    val.structural_hash(),
                ),
                body.structural_hash(),
            ),
            Expr::Lit(Literal::Nat(n)) => mix_words(EXPR_LIT_NAT, n.as_limbs()),
            Expr::Lit(Literal::Str(s)) => mix_bytes(EXPR_LIT_STR, s.as_bytes()),
            Expr::Proj(name, index, inner) => mix(
                mix(mix(EXPR_PROJ, name.structural_hash()), u64::from(*index)),
                inner.structural_hash(),
            ),
        }
    }
}
/// A distinct number for each binder annotation, for
/// [`Expr::structural_hash`].
const fn binder_info_code(bi: BinderInfo) -> u64 {
    match bi {
        BinderInfo::Default => 0,
        BinderInfo::Implicit => 1,
        BinderInfo::StrictImplicit => 2,
        BinderInfo::InstImplicit => 3,
    }
}
impl std::hash::Hash for Expr {
    /// Writes the term's structural hash — one 64-bit value, O(1) in the size
    /// of the term. Consistent with the derived `PartialEq`: the hash covers
    /// exactly the fields equality compares.
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.structural_hash());
    }
}
impl Clone for Expr {
    fn clone(&self) -> Self {
        // One unit of resource fuel per clone: this node is copied, its
        // children are shared.
        crate::fuel::charge(1);
        match self {
            Expr::Sort(l) => Expr::Sort(l.clone()),
            Expr::BVar(n) => Expr::BVar(*n),
            Expr::FVar(id) => Expr::FVar(*id),
            Expr::Const(name, levels) => Expr::Const(name.clone(), levels.clone()),
            Expr::App(f, a) => Expr::App(f.clone(), a.clone()),
            Expr::Lam(bi, name, ty, body) => Expr::Lam(*bi, name.clone(), ty.clone(), body.clone()),
            Expr::Pi(bi, name, ty, body) => Expr::Pi(*bi, name.clone(), ty.clone(), body.clone()),
            Expr::Let(name, ty, val, body) => {
                Expr::Let(name.clone(), ty.clone(), val.clone(), body.clone())
            }
            Expr::Lit(l) => Expr::Lit(l.clone()),
            Expr::Proj(name, idx, inner) => Expr::Proj(name.clone(), *idx, inner.clone()),
        }
    }
}
impl Expr {
    /// Check if this is a Sort.
    pub fn is_sort(&self) -> bool {
        matches!(self, Expr::Sort(_))
    }
    /// Check if this is Prop.
    pub fn is_prop(&self) -> bool {
        matches!(self, Expr::Sort(l) if l.is_zero())
    }
    /// Check if this is a bound variable.
    pub fn is_bvar(&self) -> bool {
        matches!(self, Expr::BVar(_))
    }
    /// Check if this is a free variable.
    pub fn is_fvar(&self) -> bool {
        matches!(self, Expr::FVar(_))
    }
    /// Check if this is an application.
    pub fn is_app(&self) -> bool {
        matches!(self, Expr::App(_, _))
    }
    /// Check if this is a lambda.
    pub fn is_lambda(&self) -> bool {
        matches!(self, Expr::Lam(_, _, _, _))
    }
    /// Check if this is a Pi type.
    pub fn is_pi(&self) -> bool {
        matches!(self, Expr::Pi(_, _, _, _))
    }
}
impl Expr {
    /// Check if this expression is a let binding.
    pub fn is_let(&self) -> bool {
        matches!(self, Expr::Let(_, _, _, _))
    }
    /// Check if this expression is a literal.
    pub fn is_lit(&self) -> bool {
        matches!(self, Expr::Lit(_))
    }
    /// Check if this expression is a named constant.
    pub fn is_const(&self) -> bool {
        matches!(self, Expr::Const(_, _))
    }
    /// Check if this expression is a projection.
    pub fn is_proj(&self) -> bool {
        matches!(self, Expr::Proj(_, _, _))
    }
    /// Check if this expression is an "atom" (cannot be further reduced
    /// without context): Sort, BVar, FVar, Const, Lit.
    pub fn is_atom(&self) -> bool {
        matches!(
            self,
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_)
        )
    }
    /// Extract the bound variable index, or `None`.
    pub fn as_bvar(&self) -> Option<u32> {
        if let Expr::BVar(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    /// Extract the free variable ID, or `None`.
    pub fn as_fvar(&self) -> Option<FVarId> {
        if let Expr::FVar(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    /// Extract the constant name, or `None`.
    pub fn as_const_name(&self) -> Option<&Name> {
        if let Expr::Const(n, _) = self {
            Some(n)
        } else {
            None
        }
    }
    /// Extract the sort level, or `None`.
    pub fn as_sort_level(&self) -> Option<&Level> {
        if let Expr::Sort(l) = self {
            Some(l)
        } else {
            None
        }
    }
    /// Return the head and argument list of an application spine.
    ///
    /// For `f a b c`, returns `(&f, [&a, &b, &c])`.
    pub fn app_head_args(&self) -> (&Expr, Vec<&Expr>) {
        let mut args = Vec::new();
        let mut cur = self;
        while let Expr::App(f, a) = cur {
            args.push(a.as_ref());
            cur = f;
        }
        args.reverse();
        (cur, args)
    }
    /// Return the number of arguments in an application spine.
    pub fn app_arity(&self) -> usize {
        let mut n = 0;
        let mut cur = self;
        while let Expr::App(f, _) = cur {
            n += 1;
            cur = f;
        }
        n
    }
    /// Count the number of leading Pi binders.
    pub fn pi_arity(&self) -> usize {
        let mut n = 0;
        let mut cur = self;
        while let Expr::Pi(_, _, _, body) = cur {
            n += 1;
            cur = body;
        }
        n
    }
    /// Count the number of leading Lambda binders.
    pub fn lam_arity(&self) -> usize {
        let mut n = 0;
        let mut cur = self;
        while let Expr::Lam(_, _, _, body) = cur {
            n += 1;
            cur = body;
        }
        n
    }
    /// Compute the AST size (total number of nodes).
    pub fn size(&self) -> usize {
        match self {
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
            Expr::App(f, a) => 1 + f.size() + a.size(),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + ty.size() + body.size(),
            Expr::Let(_, ty, val, body) => 1 + ty.size() + val.size() + body.size(),
            Expr::Proj(_, _, s) => 1 + s.size(),
        }
    }
    /// Compute the AST depth (longest path to a leaf).
    pub fn ast_depth(&self) -> usize {
        match self {
            Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
            Expr::App(f, a) => 1 + f.ast_depth().max(a.ast_depth()),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                1 + ty.ast_depth().max(body.ast_depth())
            }
            Expr::Let(_, ty, val, body) => {
                1 + ty.ast_depth().max(val.ast_depth()).max(body.ast_depth())
            }
            Expr::Proj(_, _, s) => 1 + s.ast_depth(),
        }
    }
    /// Apply this expression to a list of arguments.
    ///
    /// `e.mk_app_many(&[a, b, c])` returns `((e a) b) c`.
    pub fn mk_app_many(self, args: &[Expr]) -> Expr {
        args.iter().fold(self, |acc, a| {
            Expr::App(Node::new(acc), Node::new(a.clone()))
        })
    }
    /// Check if this expression has any loose bound variables at depth `d`.
    pub fn has_loose_bvar_at(&self, d: u32) -> bool {
        has_loose_bvar(self, d)
    }
    /// Count occurrences of bound variable `BVar(idx)` in this expression.
    pub fn count_bvar_occurrences(&self, idx: u32) -> usize {
        count_bvar_occ(self, idx)
    }
    /// Check if this expression is closed (no loose bound variables).
    pub fn is_closed(&self) -> bool {
        !has_loose_bvar(self, 0) && self.max_loose_bvar().is_none()
    }
    /// Find the maximum loose bound variable index, if any.
    fn max_loose_bvar(&self) -> Option<u32> {
        max_loose_bvar_impl(self, 0)
    }
    /// Collect all free variable IDs occurring in this expression.
    pub fn free_vars(&self) -> std::collections::HashSet<FVarId> {
        let mut set = std::collections::HashSet::new();
        collect_fvars(self, &mut set);
        set
    }
    /// Collect all constant names occurring in this expression.
    pub fn constants(&self) -> std::collections::HashSet<Name> {
        let mut set = std::collections::HashSet::new();
        collect_consts(self, &mut set);
        set
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
/// Generator for fresh free variable IDs.
///
/// Produces a strictly increasing sequence of unique IDs.
#[derive(Debug, Default)]
pub struct FVarIdGen {
    next: u64,
}
impl FVarIdGen {
    /// Create a new generator starting at 0.
    pub fn new() -> Self {
        Self::default()
    }
    /// Produce the next fresh ID.
    pub fn fresh(&mut self) -> FVarId {
        let id = FVarId::new(self.next);
        self.next += 1;
        id
    }
    /// Peek at the next ID without consuming it.
    pub fn peek(&self) -> FVarId {
        FVarId::new(self.next)
    }
    /// Reset the generator.
    pub fn reset(&mut self) {
        self.next = 0;
    }
    /// Return the current counter value.
    pub fn current(&self) -> u64 {
        self.next
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
/// A versioned record that stores a history of values.
#[allow(dead_code)]
pub struct VersionedRecord<T: Clone> {
    /// Every earlier value, oldest first: version `n` is `earlier[n]`.
    earlier: Vec<T>,
    /// The latest value (version `earlier.len()`), declared last so it drops last.
    current: T,
}
#[allow(dead_code)]
impl<T: Clone> VersionedRecord<T> {
    /// Creates a new record with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            earlier: Vec::new(),
            current: initial,
        }
    }
    /// Updates the record with a new version.
    pub fn update(&mut self, val: T) {
        let previous = std::mem::replace(&mut self.current, val);
        self.earlier.push(previous);
    }
    /// Returns the current (latest) value.
    pub fn current(&self) -> &T {
        &self.current
    }
    /// Returns the value at version `n` (0-indexed), or `None`.
    pub fn at_version(&self, n: usize) -> Option<&T> {
        match self.earlier.get(n) {
            Some(value) => Some(value),
            None if n == self.earlier.len() => Some(&self.current),
            None => None,
        }
    }
    /// Returns the version number of the current value.
    pub fn version(&self) -> usize {
        self.earlier.len()
    }
    /// Returns `true` if more than one version exists.
    pub fn has_history(&self) -> bool {
        !self.earlier.is_empty()
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
