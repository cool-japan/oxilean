//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::ConstantInfo;
use crate::{
    BinderInfo, Declaration, Environment, Expr, FVarId, Level, LevelMVarId, Literal, Name,
};
use std::collections::HashMap;

use super::functions::import_module;

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
/// Result of a module integrity check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityCheckResult {
    /// All checks passed.
    Ok,
    /// The module is empty.
    EmptyModule,
    /// The module has duplicate declaration names.
    DuplicateNames(Vec<Name>),
    /// The magic number in the binary header is wrong.
    BadMagicNumber,
    /// The version is unsupported.
    UnsupportedVersion(u32),
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
/// A dependency graph between modules.
///
/// Allows topological ordering for deterministic import order.
#[derive(Debug, Default)]
pub struct ModuleDependencyGraph {
    /// Adjacency list: module → list of modules it depends on.
    edges: HashMap<String, Vec<String>>,
}
impl ModuleDependencyGraph {
    /// Create an empty dependency graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a module to the graph.
    pub fn add_module(&mut self, name: String) {
        self.edges.entry(name).or_default();
    }
    /// Add a dependency edge: `from` depends on `to`.
    pub fn add_dep(&mut self, from: String, to: String) {
        self.edges.entry(from).or_default().push(to);
    }
    /// Build a dependency graph from a module cache.
    pub fn from_cache(cache: &ModuleCache) -> Self {
        let mut g = Self::new();
        for name in cache.all_modules() {
            g.add_module(name.to_string());
            if let Some(module) = cache.get(name) {
                for dep in &module.dependencies {
                    g.add_dep(name.to_string(), dep.clone());
                }
            }
        }
        g
    }
    /// Compute a topological ordering of modules (Kahn's algorithm).
    ///
    /// Returns `Err` if there is a cycle.
    pub fn topological_order(&self) -> Result<Vec<String>, String> {
        use std::collections::VecDeque;
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for (node, deps) in &self.edges {
            in_degree.entry(node.clone()).or_insert(0);
            for dep in deps {
                in_degree.entry(dep.clone()).or_insert(0);
            }
        }
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 0;
            }
        }
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for node in self.edges.keys() {
            let deps = self.edges.get(node).map(|v| v.len()).unwrap_or(0);
            in_degree.insert(node.clone(), deps);
        }
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(k, _)| k.clone())
            .collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node.clone());
            for (other, deps) in &self.edges {
                if deps.contains(&node) {
                    let deg = in_degree
                        .get_mut(other)
                        .expect("node must exist in in_degree map");
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push_back(other.clone());
                    }
                }
            }
        }
        if order.len() != self.edges.len() {
            Err("Cycle detected in module dependency graph".to_string())
        } else {
            Ok(order)
        }
    }
    /// Check if module `a` depends (transitively) on module `b`.
    pub fn depends_on(&self, a: &str, b: &str) -> bool {
        if let Some(deps) = self.edges.get(a) {
            if deps.iter().any(|d| d == b) {
                return true;
            }
            deps.iter().any(|d| self.depends_on(d, b))
        } else {
            false
        }
    }
    /// Number of modules in the graph.
    pub fn num_modules(&self) -> usize {
        self.edges.len()
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
/// A registry that tracks which declarations come from which module.
///
/// Used during compilation to produce better error messages and to support
/// incremental builds.
#[derive(Debug, Default)]
pub struct ModuleRegistry {
    /// Maps declaration name to the module it was imported from.
    decl_to_module: HashMap<Name, String>,
    /// Maps module name to the set of declarations it provides.
    module_to_decls: HashMap<String, Vec<Name>>,
}
impl ModuleRegistry {
    /// Create a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a declaration as coming from a module.
    pub fn register(&mut self, decl_name: Name, module_name: String) {
        self.decl_to_module
            .insert(decl_name.clone(), module_name.clone());
        self.module_to_decls
            .entry(module_name)
            .or_default()
            .push(decl_name);
    }
    /// Register all declarations from a module.
    pub fn register_module(&mut self, module: &ExportedModule) {
        for name in module.declaration_names() {
            self.register(name.clone(), module.name.clone());
        }
    }
    /// Look up which module a declaration came from.
    pub fn module_for(&self, decl: &Name) -> Option<&str> {
        self.decl_to_module.get(decl).map(|s| s.as_str())
    }
    /// Get all declarations provided by a module.
    pub fn decls_for_module(&self, module: &str) -> &[Name] {
        self.module_to_decls
            .get(module)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Get all registered module names.
    pub fn all_module_names(&self) -> Vec<&str> {
        self.module_to_decls.keys().map(|s| s.as_str()).collect()
    }
    /// Check if a declaration is registered.
    pub fn contains_decl(&self, decl: &Name) -> bool {
        self.decl_to_module.contains_key(decl)
    }
    /// Number of registered declarations.
    pub fn num_decls(&self) -> usize {
        self.decl_to_module.len()
    }
    /// Number of registered modules.
    pub fn num_modules(&self) -> usize {
        self.module_to_decls.len()
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
/// Exported module containing declarations.
#[derive(Debug, Clone)]
pub struct ExportedModule {
    /// Module name
    pub name: String,
    /// Exported legacy declarations
    pub declarations: Vec<(Name, Declaration)>,
    /// Exported constant info entries
    pub constants: Vec<(Name, ConstantInfo)>,
    /// Module dependencies (other module names)
    pub dependencies: Vec<String>,
    /// Module version
    pub version: String,
    /// Metadata (arbitrary key-value pairs)
    pub metadata: HashMap<String, String>,
}
impl ExportedModule {
    /// Create a new exported module.
    pub fn new(name: String) -> Self {
        Self {
            name,
            declarations: Vec::new(),
            constants: Vec::new(),
            dependencies: Vec::new(),
            version: "0.1.0".to_string(),
            metadata: HashMap::new(),
        }
    }
    /// Add a legacy declaration to the module.
    pub fn add_declaration(&mut self, name: Name, decl: Declaration) {
        self.declarations.push((name, decl));
    }
    /// Add a ConstantInfo to the module.
    pub fn add_constant(&mut self, name: Name, ci: ConstantInfo) {
        self.constants.push((name, ci));
    }
    /// Add a dependency.
    pub fn add_dependency(&mut self, dep: String) {
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }
    /// Add metadata.
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    /// Get all declaration names (both legacy and constant info).
    pub fn declaration_names(&self) -> Vec<&Name> {
        let mut names: Vec<&Name> = self.declarations.iter().map(|(name, _)| name).collect();
        names.extend(self.constants.iter().map(|(name, _)| name));
        names
    }
    /// Get the total number of entries.
    pub fn num_entries(&self) -> usize {
        self.declarations.len() + self.constants.len()
    }
    /// Check if the module is empty.
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty() && self.constants.is_empty()
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
/// The difference between two module versions.
#[derive(Debug, Clone, Default)]
pub struct ModuleDiff {
    /// Names added in the new version.
    pub added: Vec<Name>,
    /// Names removed from the old version.
    pub removed: Vec<Name>,
    /// Names present in both but with changed content.
    pub changed: Vec<Name>,
}
impl ModuleDiff {
    /// Create an empty diff.
    pub fn new() -> Self {
        Self::default()
    }
    /// Check if the diff is empty (no changes).
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
    /// Total number of changed items.
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len() + self.changed.len()
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
/// A semantic version for a module.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleVersion {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
}
impl ModuleVersion {
    /// Create a new version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Parse a version string of the form "M.N.P".
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }
    /// Whether this version is backwards compatible with `other`.
    ///
    /// Compatible if same major version and this is at least as new.
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major && *self >= *other
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
/// Extended metadata for a module.
#[derive(Clone, Debug, Default)]
pub struct ModuleInfo {
    /// The module's semantic version.
    pub version: Option<ModuleVersion>,
    /// Author or maintainer.
    pub author: Option<String>,
    /// License identifier (e.g. "MIT").
    pub license: Option<String>,
    /// Short description.
    pub description: Option<String>,
}
impl ModuleInfo {
    /// Create an empty info struct.
    pub fn new() -> Self {
        Self::default()
    }
    /// Attach a version.
    pub fn with_version(mut self, v: ModuleVersion) -> Self {
        self.version = Some(v);
        self
    }
    /// Attach an author.
    pub fn with_author(mut self, a: impl Into<String>) -> Self {
        self.author = Some(a.into());
        self
    }
    /// Attach a license.
    pub fn with_license(mut self, l: impl Into<String>) -> Self {
        self.license = Some(l.into());
        self
    }
    /// Attach a description.
    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
}
/// Module cache for managing multiple modules.
pub struct ModuleCache {
    /// Cached modules indexed by name
    modules: HashMap<String, ExportedModule>,
    /// Import order (for deterministic behavior)
    import_order: Vec<String>,
}
impl ModuleCache {
    /// Create a new module cache.
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            import_order: Vec::new(),
        }
    }
    /// Add a module to the cache.
    pub fn add(&mut self, module: ExportedModule) {
        let name = module.name.clone();
        self.modules.insert(name.clone(), module);
        if !self.import_order.contains(&name) {
            self.import_order.push(name);
        }
    }
    /// Get a module from the cache.
    pub fn get(&self, name: &str) -> Option<&ExportedModule> {
        self.modules.get(name)
    }
    /// Check if a module is cached.
    pub fn contains(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }
    /// Get all cached module names.
    pub fn all_modules(&self) -> Vec<&str> {
        self.import_order.iter().map(|s| s.as_str()).collect()
    }
    /// Get the number of cached modules.
    pub fn num_modules(&self) -> usize {
        self.modules.len()
    }
    /// Import all cached modules into an environment (in order).
    pub fn import_all(&self, env: &mut Environment) -> Result<(), String> {
        for name in &self.import_order {
            if let Some(module) = self.modules.get(name) {
                import_module(env, module)?;
            }
        }
        Ok(())
    }
    /// Import a specific module and its transitive dependencies.
    pub fn import_with_deps(&self, env: &mut Environment, name: &str) -> Result<(), String> {
        let module = self
            .modules
            .get(name)
            .ok_or_else(|| format!("Module '{}' not found in cache", name))?;
        for dep in &module.dependencies {
            if self.contains(dep) {
                self.import_with_deps(env, dep)?;
            }
        }
        import_module(env, module)
    }
    /// Remove a module from the cache.
    pub fn remove(&mut self, name: &str) -> Option<ExportedModule> {
        self.import_order.retain(|n| n != name);
        self.modules.remove(name)
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.modules.clear();
        self.import_order.clear();
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
