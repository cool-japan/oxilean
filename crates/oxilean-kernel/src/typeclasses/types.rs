//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::{Expr, Name};
use std::collections::HashMap;

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
/// Priority levels for instance selection.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum InstancePriority {
    /// Low priority (fallback).
    Low = 0,
    /// Normal priority.
    #[default]
    Normal = 100,
    /// High priority (preferred over Normal).
    High = 200,
    /// Forced priority (always chosen).
    Forced = 1000,
}
impl InstancePriority {
    /// Convert a numeric priority to an `InstancePriority`.
    #[allow(dead_code)]
    pub fn from_u32(n: u32) -> Self {
        match n {
            0..=50 => InstancePriority::Low,
            51..=150 => InstancePriority::Normal,
            151..=500 => InstancePriority::High,
            _ => InstancePriority::Forced,
        }
    }
    /// Numeric value.
    #[allow(dead_code)]
    pub fn value(self) -> u32 {
        self as u32
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
/// Statistics about typeclass resolution.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TypeClassStats {
    /// Number of successful instance lookups.
    pub cache_hits: u64,
    /// Number of failed instance lookups.
    pub cache_misses: u64,
    /// Total instance resolution calls.
    pub total_lookups: u64,
    /// Number of instances registered.
    pub instances_registered: u64,
    /// Number of classes registered.
    pub classes_registered: u64,
}
impl TypeClassStats {
    /// Create zeroed stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Hit rate for instance resolution (0.0–1.0).
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        if self.total_lookups == 0 {
            1.0
        } else {
            self.cache_hits as f64 / self.total_lookups as f64
        }
    }
    /// Merge another stats object.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &Self) {
        self.cache_hits += other.cache_hits;
        self.cache_misses += other.cache_misses;
        self.total_lookups += other.total_lookups;
        self.instances_registered += other.instances_registered;
        self.classes_registered += other.classes_registered;
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
/// A layered typeclass registry that supports scoped instance introduction.
///
/// Useful for local `instance` declarations inside tactics or proofs.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LayeredTypeClassRegistry {
    /// Stack of local registries (outermost first).
    layers: Vec<TypeClassRegistry>,
    /// Global base registry.
    global: TypeClassRegistry,
}
impl LayeredTypeClassRegistry {
    /// Create a new empty layered registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            global: TypeClassRegistry::new(),
        }
    }
    /// Push a new empty layer.
    #[allow(dead_code)]
    pub fn push_layer(&mut self) {
        self.layers.push(TypeClassRegistry::new());
    }
    /// Pop the topmost layer.
    #[allow(dead_code)]
    pub fn pop_layer(&mut self) {
        self.layers.pop();
    }
    /// Register an instance in the top layer (or global if no layers).
    #[allow(dead_code)]
    pub fn add_instance(&mut self, inst: Instance) {
        if let Some(top) = self.layers.last_mut() {
            top.register_instance(inst);
        } else {
            self.global.register_instance(inst);
        }
    }
    /// Register a class in the global registry.
    #[allow(dead_code)]
    pub fn add_class(&mut self, class: TypeClass) {
        self.global.register_class(class);
    }
    /// Find an instance by searching from the top layer to global.
    #[allow(dead_code)]
    pub fn find_instance(&self, class: &Name, ty: &Expr) -> Option<&Instance> {
        for layer in self.layers.iter().rev() {
            let found = layer.find_instances(class, ty);
            if let Some(inst) = found.into_iter().next() {
                return Some(inst);
            }
        }
        self.global.find_instances(class, ty).into_iter().next()
    }
    /// Current number of layers.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.layers.len()
    }
    /// Total instance count across all layers.
    #[allow(dead_code)]
    pub fn total_instances(&self) -> usize {
        let layer_total: usize = self.layers.iter().map(|l| l.instances.len()).sum();
        layer_total + self.global.instances.len()
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
/// The registry that holds all type class and instance declarations.
#[derive(Clone, Debug, Default)]
pub struct TypeClassRegistry {
    /// All registered classes, keyed by class name.
    pub(super) classes: HashMap<String, TypeClass>,
    /// All registered instances, in registration order.
    pub(super) instances: Vec<Instance>,
    /// Cache from (class, type-repr) → instance index.
    instance_cache: HashMap<(String, String), usize>,
}
impl TypeClassRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            instances: Vec::new(),
            instance_cache: HashMap::new(),
        }
    }
    /// Register a type class.
    pub fn register_class(&mut self, class: TypeClass) {
        self.classes.insert(class.name.to_string(), class);
    }
    /// Look up a class by name.
    pub fn get_class(&self, name: &Name) -> Option<&TypeClass> {
        self.classes.get(&name.to_string())
    }
    /// Check whether a name refers to a registered class.
    pub fn is_class(&self, name: &Name) -> bool {
        self.classes.contains_key(&name.to_string())
    }
    /// Number of registered classes.
    pub fn class_count(&self) -> usize {
        self.classes.len()
    }
    /// Iterate over all class names.
    pub fn class_names(&self) -> impl Iterator<Item = &String> {
        self.classes.keys()
    }
    /// Get all classes that extend the given super-class.
    pub fn subclasses_of(&self, super_name: &Name) -> Vec<&TypeClass> {
        self.classes
            .values()
            .filter(|c| c.has_super(super_name))
            .collect()
    }
    /// Register an instance.
    pub fn register_instance(&mut self, instance: Instance) {
        self.instance_cache
            .remove(&(instance.class.to_string(), format!("{:?}", instance.ty)));
        self.instances.push(instance);
    }
    /// Search for instances of the given class for the given type.
    pub fn find_instances(&self, class: &Name, ty: &Expr) -> Vec<&Instance> {
        self.instances
            .iter()
            .filter(|inst| &inst.class == class && instances_match(ty, &inst.ty))
            .collect()
    }
    /// Find the best (highest priority, lowest number) instance.
    pub fn find_best_instance(&self, class: &Name, ty: &Expr) -> InstanceSearchResult {
        let mut candidates: Vec<&Instance> = self.find_instances(class, ty);
        if candidates.is_empty() {
            return InstanceSearchResult::NotFound;
        }
        candidates.sort_by_key(|i| i.priority);
        let best_priority = candidates[0].priority;
        let top: Vec<&Instance> = candidates
            .iter()
            .filter(|i| i.priority == best_priority)
            .copied()
            .collect();
        if top.len() == 1 {
            InstanceSearchResult::Found(top[0].clone())
        } else {
            InstanceSearchResult::Ambiguous(top.into_iter().cloned().collect())
        }
    }
    /// Number of registered instances.
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }
    /// Remove all local instances.
    pub fn clear_local_instances(&mut self) {
        self.instances.retain(|i| !i.is_local);
        self.instance_cache.clear();
    }
    /// Get all instances for a given class, sorted by priority.
    pub fn instances_for_class(&self, class: &Name) -> Vec<&Instance> {
        let mut result: Vec<&Instance> = self
            .instances
            .iter()
            .filter(|i| &i.class == class)
            .collect();
        result.sort_by_key(|i| i.priority);
        result
    }
    /// Get instances that match a given type predicate.
    pub fn filter_instances<F>(&self, predicate: F) -> Vec<&Instance>
    where
        F: Fn(&Instance) -> bool,
    {
        self.instances.iter().filter(|i| predicate(i)).collect()
    }
    /// Get the projection expression for a method in a class (method index → projection).
    pub fn method_projection(&self, class: &Name, method: &Name) -> Option<Expr> {
        let cls = self.get_class(class)?;
        let m = cls.find_method(method)?;
        Some(build_method_projection(class, method, m.index))
    }
    /// Summarize the registry.
    pub fn summary(&self) -> String {
        format!(
            "TypeClassRegistry {{ classes: {}, instances: {} }}",
            self.classes.len(),
            self.instances.len()
        )
    }
}
impl TypeClassRegistry {
    /// Take a snapshot of the current instance count.
    pub fn snapshot(&self) -> RegistrySnapshot {
        RegistrySnapshot {
            instance_count: self.instances.len(),
        }
    }
    /// Restore the registry to a previously taken snapshot (removes instances added after).
    pub fn restore(&mut self, snapshot: RegistrySnapshot) {
        self.instances.truncate(snapshot.instance_count);
        self.instance_cache.clear();
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
/// A snapshot of a `TypeClassRegistry` for backtracking during elaboration.
#[derive(Debug)]
pub struct RegistrySnapshot {
    instance_count: usize,
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
/// An instance resolver that always returns `None`.
#[derive(Debug)]
pub struct NullResolver;
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
/// A complete set of method implementations for an instance.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct InstanceImpl {
    /// Map from method name to implementation.
    impls: Vec<MethodImpl>,
}
impl InstanceImpl {
    /// Create an empty implementation set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a method implementation.
    #[allow(dead_code)]
    pub fn add(&mut self, impl_: MethodImpl) {
        self.impls.push(impl_);
    }
    /// Get the implementation for a method.
    #[allow(dead_code)]
    pub fn get(&self, method: &Name) -> Option<&MethodImpl> {
        self.impls.iter().find(|m| &m.method_name == method)
    }
    /// Number of implemented methods.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.impls.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.impls.is_empty()
    }
    /// Count default implementations.
    #[allow(dead_code)]
    pub fn count_defaults(&self) -> usize {
        self.impls.iter().filter(|m| m.is_default).count()
    }
    /// All method names.
    #[allow(dead_code)]
    pub fn method_names(&self) -> Vec<&Name> {
        self.impls.iter().map(|m| &m.method_name).collect()
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
/// A type class declaration.
///
/// For example, `class Add (α : Type)` declares a type class named `Add` with
/// one type parameter and one method `add : α → α → α`.
#[derive(Clone, Debug)]
pub struct TypeClass {
    /// Name of the class (e.g., `Add`, `Mul`, `Inhabited`).
    pub name: Name,
    /// Names of the type parameters.
    pub params: Vec<Name>,
    /// The full type of the class (as a Pi-type).
    pub ty: Expr,
    /// Methods declared in the class, in declaration order.
    pub methods: Vec<Method>,
    /// Names of super-classes that instances must also implement.
    pub super_classes: Vec<Name>,
    /// Whether this class is in Prop (proof-irrelevant).
    pub is_prop: bool,
}
impl TypeClass {
    /// Create a new type class with no methods or super-classes.
    pub fn new(name: Name, params: Vec<Name>, ty: Expr) -> Self {
        Self {
            name,
            params,
            ty,
            methods: Vec::new(),
            super_classes: Vec::new(),
            is_prop: false,
        }
    }
    /// Add a method to the class.
    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }
    /// Add a super-class dependency.
    pub fn add_super(&mut self, super_name: Name) {
        self.super_classes.push(super_name);
    }
    /// Mark this class as living in Prop.
    pub fn mark_prop(mut self) -> Self {
        self.is_prop = true;
        self
    }
    /// Look up a method by name.
    pub fn find_method(&self, name: &Name) -> Option<&Method> {
        self.methods.iter().find(|m| &m.name == name)
    }
    /// Check whether the class has a super-class with the given name.
    pub fn has_super(&self, name: &Name) -> bool {
        self.super_classes.contains(name)
    }
    /// Count methods in the class.
    pub fn method_count(&self) -> usize {
        self.methods.len()
    }
    /// Check whether the class has any methods.
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }
    /// Iterate over all method names.
    pub fn method_names(&self) -> impl Iterator<Item = &Name> {
        self.methods.iter().map(|m| &m.name)
    }
    /// Check whether the class has any super-classes.
    pub fn has_super_classes(&self) -> bool {
        !self.super_classes.is_empty()
    }
    /// Number of type parameters.
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}
/// An instance of a type class.
///
/// For example, `instance : Add Nat` is an instance of `Add` for the type `Nat`.
#[derive(Clone, Debug)]
pub struct Instance {
    /// The class this is an instance of.
    pub class: Name,
    /// The type argument(s) for this instance.
    pub ty: Expr,
    /// Search priority (lower number = searched first).
    pub priority: i32,
    /// Implementations of each method (method name → expression).
    pub methods: HashMap<String, Expr>,
    /// Optional name for the instance declaration.
    pub instance_name: Option<Name>,
    /// Whether this is a local (hypothetical) instance.
    pub is_local: bool,
}
impl Instance {
    /// Create a new anonymous instance with default priority.
    pub fn new(class: Name, ty: Expr) -> Self {
        Self {
            class,
            ty,
            priority: 100,
            methods: HashMap::new(),
            instance_name: None,
            is_local: false,
        }
    }
    /// Create a named instance.
    pub fn named(class: Name, ty: Expr, name: Name) -> Self {
        Self {
            class,
            ty,
            priority: 100,
            methods: HashMap::new(),
            instance_name: Some(name),
            is_local: false,
        }
    }
    /// Set the priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    /// Mark as a local instance.
    pub fn as_local(mut self) -> Self {
        self.is_local = true;
        self
    }
    /// Add an implementation for a method.
    pub fn add_method_impl(&mut self, method_name: impl Into<String>, impl_expr: Expr) {
        self.methods.insert(method_name.into(), impl_expr);
    }
    /// Look up the implementation of a method.
    pub fn get_method_impl(&self, method_name: &str) -> Option<&Expr> {
        self.methods.get(method_name)
    }
    /// Check whether all methods of the given class are implemented.
    pub fn implements_all(&self, class: &TypeClass) -> bool {
        class
            .methods
            .iter()
            .all(|m| self.methods.contains_key(&m.name.to_string()))
    }
    /// Number of implemented methods.
    pub fn implemented_count(&self) -> usize {
        self.methods.len()
    }
    /// Check whether this instance has a name.
    pub fn is_named(&self) -> bool {
        self.instance_name.is_some()
    }
}
/// A single method belonging to a type class.
#[derive(Clone, Debug)]
pub struct Method {
    /// Name of the method.
    pub name: Name,
    /// Type signature of the method (as a kernel expression).
    pub ty: Expr,
    /// Whether the method has a default implementation.
    pub has_default: bool,
    /// Index of this method in the class declaration (0-based).
    pub index: usize,
}
impl Method {
    /// Create a new method without a default implementation.
    pub fn new(name: Name, ty: Expr, index: usize) -> Self {
        Self {
            name,
            ty,
            has_default: false,
            index,
        }
    }
    /// Create a method that has a default implementation.
    pub fn with_default(name: Name, ty: Expr, index: usize) -> Self {
        Self {
            name,
            ty,
            has_default: true,
            index,
        }
    }
    /// Return a copy marked as having a default.
    pub fn set_default(mut self) -> Self {
        self.has_default = true;
        self
    }
}
/// Represents a class hierarchy edge (parent → child relationship).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassEdge {
    /// The parent class.
    pub parent: Name,
    /// The child class that extends the parent.
    pub child: Name,
}
impl ClassEdge {
    /// Create a new class hierarchy edge.
    pub fn new(parent: Name, child: Name) -> Self {
        Self { parent, child }
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
/// A method implementation override for derived instances.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MethodImpl {
    /// Method name.
    pub method_name: Name,
    /// Implementation expression.
    pub impl_expr: Expr,
    /// Whether this uses a default implementation.
    pub is_default: bool,
}
impl MethodImpl {
    /// Create a custom method implementation.
    #[allow(dead_code)]
    pub fn custom(method: Name, expr: Expr) -> Self {
        Self {
            method_name: method,
            impl_expr: expr,
            is_default: false,
        }
    }
    /// Create a default method implementation.
    #[allow(dead_code)]
    pub fn default_impl(method: Name, expr: Expr) -> Self {
        Self {
            method_name: method,
            impl_expr: expr,
            is_default: true,
        }
    }
}
/// Result of a type class search.
#[derive(Clone, Debug)]
pub enum InstanceSearchResult {
    /// A unique instance was found.
    Found(Instance),
    /// Multiple instances matched (ambiguity).
    Ambiguous(Vec<Instance>),
    /// No instance was found.
    NotFound,
}
impl InstanceSearchResult {
    /// Returns `true` if a unique instance was found.
    pub fn is_found(&self) -> bool {
        matches!(self, InstanceSearchResult::Found(_))
    }
    /// Returns `true` if the search was ambiguous.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, InstanceSearchResult::Ambiguous(_))
    }
    /// Extract the unique instance, if any.
    pub fn into_instance(self) -> Option<Instance> {
        match self {
            InstanceSearchResult::Found(inst) => Some(inst),
            _ => None,
        }
    }
}
