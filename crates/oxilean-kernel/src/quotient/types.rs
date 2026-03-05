//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::{Expr, KernelError, Name};
use std::collections::HashMap;

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
/// Kernel-level record for quotient operations.
#[derive(Debug, Clone)]
pub struct QuotientKernel {
    quot_types: Vec<QuotientType>,
}
impl QuotientKernel {
    /// Create a new empty quotient kernel.
    pub fn new() -> Self {
        Self {
            quot_types: Vec::new(),
        }
    }
    /// Register a quotient type.
    pub fn register(&mut self, qt: QuotientType) {
        self.quot_types.push(qt);
    }
    /// Find by base type.
    pub fn find_by_base(&self, base: &Expr) -> Option<&QuotientType> {
        self.quot_types.iter().find(|qt| &qt.base_type == base)
    }
    /// Check if an expression is a quotient type.
    ///
    /// Returns `true` if `expr` is either:
    /// 1. Registered in this kernel (exact match against `qt.quot_type`), or
    /// 2. Structurally a `Quot α r` application (`App(App(Quot, _), _)`).
    pub fn is_quot_type(&self, expr: &Expr) -> bool {
        if is_quot_type_expr(expr) {
            return true;
        }
        self.quot_types.iter().any(|qt| &qt.quot_type == expr)
    }
    /// Attempt to reduce a quotient expression.
    pub fn reduce(&self, head: &Expr, args: &[Expr]) -> Option<Expr> {
        try_reduce_quot(head, args)
    }
    /// Count registered quotient types.
    pub fn count(&self) -> usize {
        self.quot_types.len()
    }
}
/// A normalizer that reduces quotient expressions to normal form.
///
/// Repeatedly applies quotient reduction rules until no more apply.
#[allow(dead_code)]
pub struct QuotientNormalizer {
    /// Maximum number of normalization steps.
    max_steps: usize,
    /// Accumulated reduction steps.
    steps_taken: usize,
}
impl QuotientNormalizer {
    /// Create a new normalizer.
    #[allow(dead_code)]
    pub fn new(max_steps: usize) -> Self {
        Self {
            max_steps,
            steps_taken: 0,
        }
    }
    /// Normalize an expression, returning the result and whether any change occurred.
    #[allow(dead_code)]
    pub fn normalize(&mut self, expr: Expr) -> (Expr, bool) {
        let mut current = expr;
        let mut changed = false;
        while self.steps_taken < self.max_steps {
            let (next, did_change) = self.step(&current);
            if !did_change {
                break;
            }
            current = next;
            changed = true;
            self.steps_taken += 1;
        }
        (current, changed)
    }
    fn step(&self, expr: &Expr) -> (Expr, bool) {
        match expr {
            Expr::App(f, _) => {
                let args = collect_args_norm(expr);
                if let Expr::Const(name, _) = &args[0] {
                    if *name == Name::str("Quot.lift") && args.len() >= 4 {
                        if let Some(r) = reduce_quot_lift(&args[1..]) {
                            return (r, true);
                        }
                    }
                    if *name == Name::str("Quot.ind") && args.len() >= 3 {
                        if let Some(r) = reduce_quot_ind(&args[1..]) {
                            return (r, true);
                        }
                    }
                }
                let (f2, cf) = self.step(f);
                let arg = args.last().cloned().unwrap_or(Expr::BVar(0));
                let (a2, ca) = self.step(&arg);
                if cf || ca {
                    return (Expr::App(Box::new(f2), Box::new(a2)), true);
                }
                (expr.clone(), false)
            }
            _ => (expr.clone(), false),
        }
    }
    /// Number of steps taken.
    #[allow(dead_code)]
    pub fn steps_taken(&self) -> usize {
        self.steps_taken
    }
    /// Reset the step counter.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.steps_taken = 0;
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
/// Description of a quotient type for display purposes.
pub struct QuotientDescription {
    /// Display name of the base type.
    pub base_name: String,
    /// Display name of the relation.
    pub relation_name: String,
    /// Number of elements in a finite model (if known).
    pub finite_model_size: Option<usize>,
}
impl QuotientDescription {
    /// Create a description.
    pub fn new(base_name: impl Into<String>, relation_name: impl Into<String>) -> Self {
        Self {
            base_name: base_name.into(),
            relation_name: relation_name.into(),
            finite_model_size: None,
        }
    }
    /// Set the finite model size.
    pub fn with_model_size(mut self, n: usize) -> Self {
        self.finite_model_size = Some(n);
        self
    }
    /// Return a human-readable summary.
    pub fn display(&self) -> String {
        match self.finite_model_size {
            Some(n) => {
                format!(
                    "Quot {} {} ({} elements)",
                    self.base_name, self.relation_name, n
                )
            }
            None => format!("Quot {} {}", self.base_name, self.relation_name),
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
/// A stack-based quotient reduction engine.
pub struct QuotientReducer {
    steps: Vec<QuotReductionStep>,
    #[allow(dead_code)]
    cache: QuotLiftCache,
    pub(crate) max_steps: usize,
}
impl QuotientReducer {
    /// Create a new reducer with the given step limit.
    pub fn new(max_steps: usize) -> Self {
        Self {
            steps: Vec::new(),
            cache: QuotLiftCache::new(),
            max_steps,
        }
    }
    /// Try to reduce an expression. Returns the reduced expression and whether any reduction happened.
    pub fn reduce(&mut self, expr: &Expr) -> (Expr, bool) {
        match expr {
            Expr::App(f, _arg) => {
                if let Expr::Const(name, _) = f.as_ref() {
                    if *name == Name::str("Quot.lift") {
                        let args = collect_args(expr);
                        if let Some((reduced, kind)) = try_reduce_quot_full(&args[0], &args[1..]) {
                            let step = QuotReductionStep::new(kind, expr.clone(), reduced.clone());
                            if self.steps.len() < self.max_steps {
                                self.steps.push(step);
                            }
                            return (reduced, true);
                        }
                    }
                }
                (expr.clone(), false)
            }
            _ => (expr.clone(), false),
        }
    }
    /// Get all recorded reduction steps.
    pub fn steps(&self) -> &[QuotReductionStep] {
        &self.steps
    }
    /// Clear the step history.
    pub fn clear_steps(&mut self) {
        self.steps.clear();
    }
    /// Number of recorded steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}
/// A quotient type declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct QuotientType {
    /// Type being quotiented.
    pub base_type: Expr,
    /// Equivalence relation.
    pub relation: Expr,
    /// The quotient type itself.
    pub quot_type: Expr,
}
impl QuotientType {
    /// Create a new quotient type.
    pub fn new(base_type: Expr, relation: Expr) -> Self {
        let quot_type = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Quot"), vec![])),
                Box::new(base_type.clone()),
            )),
            Box::new(relation.clone()),
        );
        Self {
            base_type,
            relation,
            quot_type,
        }
    }
    /// Get the mk constant.
    pub fn mk_const(&self) -> Expr {
        Expr::Const(Name::str("Quot.mk"), vec![])
    }
    /// Apply Quot.mk to an element.
    pub fn mk_apply(&self, elem: Expr) -> Expr {
        Expr::App(Box::new(self.mk_const()), Box::new(elem))
    }
    /// Get the lift constant.
    pub fn lift_const(&self) -> Expr {
        Expr::Const(Name::str("Quot.lift"), vec![])
    }
    /// Get the ind constant.
    pub fn ind_const(&self) -> Expr {
        Expr::Const(Name::str("Quot.ind"), vec![])
    }
    /// Get the sound constant.
    pub fn sound_const(&self) -> Expr {
        Expr::Const(Name::str("Quot.sound"), vec![])
    }
    /// Apply Quot.lift f h q.
    pub fn lift_apply(&self, f: Expr, h: Expr, q: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(self.lift_const()), Box::new(f))),
                Box::new(h),
            )),
            Box::new(q),
        )
    }
}
/// A simple cache for Quot.lift reductions.
#[derive(Clone, Debug, Default)]
pub struct QuotLiftCache {
    cache: std::collections::HashMap<String, Expr>,
}
impl QuotLiftCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Look up a cached reduction.
    pub fn get(&self, f: &Expr, a: &Expr) -> Option<&Expr> {
        let key = format!("{:?}-{:?}", f, a);
        self.cache.get(&key)
    }
    /// Store a reduction result.
    pub fn put(&mut self, f: &Expr, a: &Expr, result: Expr) {
        let key = format!("{:?}-{:?}", f, a);
        self.cache.insert(key, result);
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
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
/// Equivalence class representative system.
///
/// Given an equivalence relation, this tracks canonical representatives for
/// each equivalence class using a union-find style approach.
#[derive(Clone, Debug, Default)]
pub struct EquivClassSystem {
    /// Map from expression debug string to the canonical representative.
    reps: std::collections::HashMap<String, Expr>,
}
impl EquivClassSystem {
    /// Create an empty system.
    pub fn new() -> Self {
        Self {
            reps: std::collections::HashMap::new(),
        }
    }
    /// Register `expr` as its own representative (singleton class).
    pub fn insert(&mut self, expr: Expr) {
        let key = format!("{:?}", expr);
        self.reps.entry(key).or_insert(expr);
    }
    /// Merge the classes of `a` and `b` (choosing `a` as canonical).
    pub fn merge(&mut self, a: &Expr, b: &Expr) {
        let key_b = format!("{:?}", b);
        let rep_a = self.rep_of(a).unwrap_or_else(|| a.clone());
        self.reps.insert(key_b, rep_a);
    }
    /// Look up the representative of `expr`.
    pub fn rep_of(&self, expr: &Expr) -> Option<Expr> {
        let key = format!("{:?}", expr);
        self.reps.get(&key).cloned()
    }
    /// Check whether `a` and `b` have the same representative.
    pub fn same_class(&self, a: &Expr, b: &Expr) -> bool {
        match (self.rep_of(a), self.rep_of(b)) {
            (Some(ra), Some(rb)) => ra == rb,
            _ => a == b,
        }
    }
    /// Number of registered expressions.
    pub fn len(&self) -> usize {
        self.reps.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.reps.is_empty()
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
/// Builder for constructing quotient type structures step by step.
#[derive(Debug, Default)]
pub struct QuotientBuilder {
    base_type: Option<Expr>,
    relation: Option<Expr>,
}
impl QuotientBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            base_type: None,
            relation: None,
        }
    }
    /// Set the base type.
    pub fn base(mut self, ty: Expr) -> Self {
        self.base_type = Some(ty);
        self
    }
    /// Set the equivalence relation.
    pub fn relation(mut self, rel: Expr) -> Self {
        self.relation = Some(rel);
        self
    }
    /// Build the `QuotientType`, returning `None` if either field is missing.
    pub fn build(self) -> Option<QuotientType> {
        Some(QuotientType::new(self.base_type?, self.relation?))
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
/// Kind of a quotient reduction rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotReductionKind {
    /// Quot.lift f h (Quot.mk a) → f a.
    Lift,
    /// Quot.ind h (Quot.mk a) → h a.
    Ind,
    /// Quot.sound h → Quot.mk a = Quot.mk b (when r a b).
    Sound,
}
impl QuotReductionKind {
    /// Human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            QuotReductionKind::Lift => "Quot.lift reduction",
            QuotReductionKind::Ind => "Quot.ind reduction",
            QuotReductionKind::Sound => "Quot.sound",
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
/// Statistics about quotient usage in an expression.
#[derive(Debug, Default, Clone)]
pub struct QuotStats {
    /// Total `Quot.mk` applications found.
    pub mk_count: usize,
    /// Total `Quot.lift` applications found.
    pub lift_count: usize,
    /// Total `Quot.ind` applications found.
    pub ind_count: usize,
    /// Total `Quot.sound` applications found.
    pub sound_count: usize,
}
impl QuotStats {
    /// Compute statistics for an expression.
    pub fn compute(expr: &Expr) -> Self {
        let mut stats = Self::default();
        Self::walk(expr, &mut stats);
        stats
    }
    fn walk(expr: &Expr, stats: &mut Self) {
        match expr {
            Expr::Const(name, _) => {
                if *name == Name::str("Quot.mk") {
                    stats.mk_count += 1;
                } else if *name == Name::str("Quot.lift") {
                    stats.lift_count += 1;
                } else if *name == Name::str("Quot.ind") {
                    stats.ind_count += 1;
                } else if *name == Name::str("Quot.sound") {
                    stats.sound_count += 1;
                }
            }
            Expr::App(f, a) => {
                Self::walk(f, stats);
                Self::walk(a, stats);
            }
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                Self::walk(ty, stats);
                Self::walk(body, stats);
            }
            Expr::Let(_, ty, val, body) => {
                Self::walk(ty, stats);
                Self::walk(val, stats);
                Self::walk(body, stats);
            }
            _ => {}
        }
    }
    /// Total number of quotient-related subterms.
    pub fn total(&self) -> usize {
        self.mk_count + self.lift_count + self.ind_count + self.sound_count
    }
    /// Check whether any quotient terms appear.
    pub fn has_quot(&self) -> bool {
        self.total() > 0
    }
}
/// Properties of an equivalence relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquivProperty {
    /// Reflexivity: ∀ a, r a a.
    Reflexive,
    /// Symmetry: ∀ a b, r a b → r b a.
    Symmetric,
    /// Transitivity: ∀ a b c, r a b → r b c → r a c.
    Transitive,
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
/// Which quotient elimination form is being checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuotUsageKind {
    /// `Quot.lift` — eliminator; valid for any motive sort.
    Lift,
    /// `Quot.ind` — induction principle; motive must be propositional.
    Ind,
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
/// A single quotient reduction step (for tracing).
#[derive(Clone, Debug)]
pub struct QuotReductionStep {
    /// Which rule was applied.
    pub kind: QuotReductionKind,
    /// The expression before reduction.
    pub before: Expr,
    /// The expression after reduction.
    pub after: Expr,
}
impl QuotReductionStep {
    /// Create a new reduction step.
    pub fn new(kind: QuotReductionKind, before: Expr, after: Expr) -> Self {
        Self {
            kind,
            before,
            after,
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
