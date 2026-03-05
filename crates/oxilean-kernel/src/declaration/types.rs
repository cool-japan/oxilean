//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::ReducibilityHint;
use crate::{Expr, Level, Name};

use std::collections::{HashMap, HashSet, VecDeque};

/// A simple LIFO work queue.
#[allow(dead_code)]
pub struct WorkStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> WorkStack<T> {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Pushes a work item.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    /// Pops the next work item.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending work items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A memoized computation slot that stores a cached value.
#[allow(dead_code)]
pub struct MemoSlot<T: Clone> {
    cached: Option<T>,
}
#[allow(dead_code)]
impl<T: Clone> MemoSlot<T> {
    /// Creates an uncomputed memo slot.
    pub fn new() -> Self {
        Self { cached: None }
    }
    /// Returns the cached value, computing it with `f` if absent.
    pub fn get_or_compute(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.cached.is_none() {
            self.cached = Some(f());
        }
        self.cached
            .as_ref()
            .expect("cached value must be initialized before access")
    }
    /// Invalidates the cached value.
    pub fn invalidate(&mut self) {
        self.cached = None;
    }
    /// Returns `true` if the value has been computed.
    pub fn is_cached(&self) -> bool {
        self.cached.is_some()
    }
}
/// Base constant value shared by all declaration types.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantVal {
    /// Declaration name.
    pub name: Name,
    /// Universe level parameters.
    pub level_params: Vec<Name>,
    /// Type of the constant.
    pub ty: Expr,
}
/// An index that maps declaration names to their positions.
#[allow(dead_code)]
pub struct DeclIndex {
    map: std::collections::HashMap<String, usize>,
}
#[allow(dead_code)]
impl DeclIndex {
    /// Creates an empty declaration index.
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    /// Inserts a name-position pair.
    pub fn insert(&mut self, name: impl Into<String>, pos: usize) {
        self.map.insert(name.into(), pos);
    }
    /// Returns the position of `name`, or `None`.
    pub fn get(&self, name: &str) -> Option<usize> {
        self.map.get(name).copied()
    }
    /// Returns the number of indexed declarations.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Returns all names.
    pub fn names(&self) -> Vec<&str> {
        self.map.keys().map(|s| s.as_str()).collect()
    }
}
/// Axiom declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct AxiomVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Whether this axiom is unsafe.
    pub is_unsafe: bool,
}
/// Which quotient component a declaration represents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuotKind {
    /// `Quot` type itself.
    Type,
    /// `Quot.mk` constructor.
    Mk,
    /// `Quot.lift` eliminator.
    Lift,
    /// `Quot.ind` induction principle.
    Ind,
}
/// Constructor declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructorVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Name of parent inductive type.
    pub induct: Name,
    /// Constructor index (position in declaration order).
    pub cidx: u32,
    /// Number of inductive parameters.
    pub num_params: u32,
    /// Number of fields (arity - nparams).
    pub num_fields: u32,
    /// Whether this is unsafe.
    pub is_unsafe: bool,
}
/// A simple event counter with named events.
#[allow(dead_code)]
pub struct EventCounter {
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl EventCounter {
    /// Creates a new empty counter.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Increments the counter for `event`.
    pub fn inc(&mut self, event: &str) {
        *self.counts.entry(event.to_string()).or_insert(0) += 1;
    }
    /// Adds `n` to the counter for `event`.
    pub fn add(&mut self, event: &str, n: u64) {
        *self.counts.entry(event.to_string()).or_insert(0) += n;
    }
    /// Returns the count for `event`.
    pub fn get(&self, event: &str) -> u64 {
        self.counts.get(event).copied().unwrap_or(0)
    }
    /// Returns the total count across all events.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Resets all counters.
    pub fn reset(&mut self) {
        self.counts.clear();
    }
    /// Returns all event names.
    pub fn event_names(&self) -> Vec<&str> {
        self.counts.keys().map(|s| s.as_str()).collect()
    }
}
/// Theorem declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct TheoremVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Proof term.
    pub value: Expr,
    /// Names in mutual declaration group.
    pub all: Vec<Name>,
}
/// Recursor (eliminator) declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct RecursorVal {
    /// Common fields.
    pub common: ConstantVal,
    /// All inductive types in mutual declaration.
    pub all: Vec<Name>,
    /// Number of parameters.
    pub num_params: u32,
    /// Number of indices.
    pub num_indices: u32,
    /// Number of motive arguments.
    pub num_motives: u32,
    /// Number of minor premises (one per constructor).
    pub num_minors: u32,
    /// Reduction rules (one per constructor).
    pub rules: Vec<RecursorRule>,
    /// Whether this supports K-like reduction.
    pub k: bool,
    /// Whether this is unsafe.
    pub is_unsafe: bool,
}
impl RecursorVal {
    /// Get the index of the major premise in the recursor's arguments.
    ///
    /// The major premise is the argument being eliminated.
    /// Position: nparams + nmotives + nminors + nindices
    pub fn get_major_idx(&self) -> u32 {
        self.num_params + self.num_motives + self.num_minors + self.num_indices
    }
    /// Get the total number of arguments before the major premise.
    pub fn get_first_index_idx(&self) -> u32 {
        self.num_params + self.num_motives + self.num_minors
    }
    /// Find the reduction rule for a given constructor.
    pub fn get_rule(&self, ctor: &Name) -> Option<&RecursorRule> {
        self.rules.iter().find(|r| &r.ctor == ctor)
    }
}
/// A simple stack-based scope tracker.
#[allow(dead_code)]
pub struct ScopeStack {
    names: Vec<String>,
}
#[allow(dead_code)]
impl ScopeStack {
    /// Creates a new empty scope stack.
    pub fn new() -> Self {
        Self { names: Vec::new() }
    }
    /// Pushes a scope name.
    pub fn push(&mut self, name: impl Into<String>) {
        self.names.push(name.into());
    }
    /// Pops the current scope.
    pub fn pop(&mut self) -> Option<String> {
        self.names.pop()
    }
    /// Returns the current (innermost) scope name, or `None`.
    pub fn current(&self) -> Option<&str> {
        self.names.last().map(|s| s.as_str())
    }
    /// Returns the depth of the scope stack.
    pub fn depth(&self) -> usize {
        self.names.len()
    }
    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Returns the full path as a dot-separated string.
    pub fn path(&self) -> String {
        self.names.join(".")
    }
}
/// A key-value annotation table for arbitrary metadata.
#[allow(dead_code)]
pub struct AnnotationTable {
    map: std::collections::HashMap<String, Vec<String>>,
}
#[allow(dead_code)]
impl AnnotationTable {
    /// Creates an empty annotation table.
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    /// Adds an annotation value for the given key.
    pub fn annotate(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.map.entry(key.into()).or_default().push(val.into());
    }
    /// Returns all annotations for `key`.
    pub fn get_all(&self, key: &str) -> &[String] {
        self.map.get(key).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns the number of distinct annotation keys.
    pub fn num_keys(&self) -> usize {
        self.map.len()
    }
    /// Returns `true` if the table has any annotations for `key`.
    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}
/// A clock that measures elapsed time in a loop.
#[allow(dead_code)]
pub struct LoopClock {
    start: std::time::Instant,
    iters: u64,
}
#[allow(dead_code)]
impl LoopClock {
    /// Starts the clock.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            iters: 0,
        }
    }
    /// Records one iteration.
    pub fn tick(&mut self) {
        self.iters += 1;
    }
    /// Returns the elapsed time in microseconds.
    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1e6
    }
    /// Returns the average microseconds per iteration.
    pub fn avg_us_per_iter(&self) -> f64 {
        if self.iters == 0 {
            return 0.0;
        }
        self.elapsed_us() / self.iters as f64
    }
    /// Returns the number of iterations.
    pub fn iters(&self) -> u64 {
        self.iters
    }
}
/// A simple sparse bit set.
#[allow(dead_code)]
pub struct SparseBitSet {
    words: Vec<u64>,
}
#[allow(dead_code)]
impl SparseBitSet {
    /// Creates a new bit set that can hold at least `capacity` bits.
    pub fn new(capacity: usize) -> Self {
        let words = (capacity + 63) / 64;
        Self {
            words: vec![0u64; words],
        }
    }
    /// Sets bit `i`.
    pub fn set(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] |= 1u64 << bit;
        }
    }
    /// Clears bit `i`.
    pub fn clear(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] &= !(1u64 << bit);
        }
    }
    /// Returns `true` if bit `i` is set.
    pub fn get(&self, i: usize) -> bool {
        let word = i / 64;
        let bit = i % 64;
        self.words.get(word).is_some_and(|w| w & (1u64 << bit) != 0)
    }
    /// Returns the number of set bits.
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }
    /// Returns the union with another bit set.
    pub fn union(&self, other: &SparseBitSet) -> SparseBitSet {
        let len = self.words.len().max(other.words.len());
        let mut result = SparseBitSet {
            words: vec![0u64; len],
        };
        for i in 0..self.words.len() {
            result.words[i] |= self.words[i];
        }
        for i in 0..other.words.len() {
            result.words[i] |= other.words[i];
        }
        result
    }
}
/// A counted-access cache that tracks hit and miss statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StatCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    /// The inner LRU cache.
    pub inner: SimpleLruCache<K, V>,
    /// Number of cache hits.
    pub hits: u64,
    /// Number of cache misses.
    pub misses: u64,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> StatCache<K, V> {
    /// Creates a new stat cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: SimpleLruCache::new(capacity),
            hits: 0,
            misses: 0,
        }
    }
    /// Performs a lookup, tracking hit/miss.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let result = self.inner.get(key);
        if result.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        None
    }
    /// Inserts a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        self.inner.put(key, val);
    }
    /// Returns the hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}
/// A slot that can hold a value, with lazy initialization.
#[allow(dead_code)]
pub struct Slot<T> {
    inner: Option<T>,
}
#[allow(dead_code)]
impl<T> Slot<T> {
    /// Creates an empty slot.
    pub fn empty() -> Self {
        Self { inner: None }
    }
    /// Fills the slot with `val`.  Panics if already filled.
    pub fn fill(&mut self, val: T) {
        assert!(self.inner.is_none(), "Slot: already filled");
        self.inner = Some(val);
    }
    /// Returns the slot's value, or `None`.
    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref()
    }
    /// Returns `true` if the slot is filled.
    pub fn is_filled(&self) -> bool {
        self.inner.is_some()
    }
    /// Takes the value out of the slot.
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }
    /// Fills the slot if empty, returning a reference to the value.
    pub fn get_or_fill_with(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.inner.is_none() {
            self.inner = Some(f());
        }
        self.inner
            .as_ref()
            .expect("inner value must be initialized before access")
    }
}
/// A pair of values useful for before/after comparisons.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BeforeAfter<T> {
    /// Value before the transformation.
    pub before: T,
    /// Value after the transformation.
    pub after: T,
}
#[allow(dead_code)]
impl<T: PartialEq> BeforeAfter<T> {
    /// Creates a new before/after pair.
    pub fn new(before: T, after: T) -> Self {
        Self { before, after }
    }
    /// Returns `true` if before equals after (no change).
    pub fn unchanged(&self) -> bool {
        self.before == self.after
    }
}
/// A single recursor reduction rule.
///
/// When the recursor's major premise is headed by constructor `ctor`,
/// the recursor reduces using the `rhs` expression.
#[derive(Clone, Debug, PartialEq)]
pub struct RecursorRule {
    /// Constructor this rule applies to.
    pub ctor: Name,
    /// Number of fields for this constructor.
    pub nfields: u32,
    /// Right-hand side of the reduction rule.
    pub rhs: Expr,
}
/// A monotonic timestamp in microseconds.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);
#[allow(dead_code)]
impl Timestamp {
    /// Creates a timestamp from microseconds.
    pub const fn from_us(us: u64) -> Self {
        Self(us)
    }
    /// Returns the timestamp in microseconds.
    pub fn as_us(self) -> u64 {
        self.0
    }
    /// Returns the duration between two timestamps.
    pub fn elapsed_since(self, earlier: Timestamp) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}
/// An attribute that can be attached to a declaration.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeclAttr {
    /// Mark as inline (always unfold).
    Inline,
    /// Mark as `@[simp]` (add to simp set).
    Simp,
    /// Mark as `@[ext]` (register as an extensionality lemma).
    Ext,
    /// Mark as reducible.
    Reducible,
    /// An unknown attribute with a name.
    Unknown(String),
}
#[allow(dead_code)]
impl DeclAttr {
    /// Returns the string name of the attribute.
    pub fn name(&self) -> &str {
        match self {
            DeclAttr::Inline => "inline",
            DeclAttr::Simp => "simp",
            DeclAttr::Ext => "ext",
            DeclAttr::Reducible => "reducible",
            DeclAttr::Unknown(s) => s.as_str(),
        }
    }
}
/// Unified constant info enum (mirrors LEAN 4's `constant_info`).
///
/// Every declaration in the environment is stored as a `ConstantInfo`.
/// The type checker dispatches on this enum to determine reduction behavior.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstantInfo {
    /// Axiom declaration.
    Axiom(AxiomVal),
    /// Definition with body.
    Definition(DefinitionVal),
    /// Theorem with proof.
    Theorem(TheoremVal),
    /// Opaque definition.
    Opaque(OpaqueVal),
    /// Inductive type.
    Inductive(InductiveVal),
    /// Constructor of inductive type.
    Constructor(ConstructorVal),
    /// Recursor (eliminator).
    Recursor(RecursorVal),
    /// Quotient type component.
    Quotient(QuotVal),
}
impl ConstantInfo {
    /// Get the name of this constant.
    pub fn name(&self) -> &Name {
        &self.common().name
    }
    /// Get the type of this constant.
    pub fn ty(&self) -> &Expr {
        &self.common().ty
    }
    /// Get the universe level parameters.
    pub fn level_params(&self) -> &[Name] {
        &self.common().level_params
    }
    /// Get the common constant value.
    pub fn common(&self) -> &ConstantVal {
        match self {
            ConstantInfo::Axiom(v) => &v.common,
            ConstantInfo::Definition(v) => &v.common,
            ConstantInfo::Theorem(v) => &v.common,
            ConstantInfo::Opaque(v) => &v.common,
            ConstantInfo::Inductive(v) => &v.common,
            ConstantInfo::Constructor(v) => &v.common,
            ConstantInfo::Recursor(v) => &v.common,
            ConstantInfo::Quotient(v) => &v.common,
        }
    }
    /// Check if this is an axiom.
    pub fn is_axiom(&self) -> bool {
        matches!(self, ConstantInfo::Axiom(_))
    }
    /// Check if this is a definition.
    pub fn is_definition(&self) -> bool {
        matches!(self, ConstantInfo::Definition(_))
    }
    /// Check if this is a theorem.
    pub fn is_theorem(&self) -> bool {
        matches!(self, ConstantInfo::Theorem(_))
    }
    /// Check if this is opaque.
    pub fn is_opaque(&self) -> bool {
        matches!(self, ConstantInfo::Opaque(_))
    }
    /// Check if this is an inductive type.
    pub fn is_inductive(&self) -> bool {
        matches!(self, ConstantInfo::Inductive(_))
    }
    /// Check if this is a constructor.
    pub fn is_constructor(&self) -> bool {
        matches!(self, ConstantInfo::Constructor(_))
    }
    /// Check if this is a recursor.
    pub fn is_recursor(&self) -> bool {
        matches!(self, ConstantInfo::Recursor(_))
    }
    /// Check if this is a quotient component.
    pub fn is_quotient(&self) -> bool {
        matches!(self, ConstantInfo::Quotient(_))
    }
    /// Get the definition value if this is a Definition.
    pub fn to_definition_val(&self) -> Option<&DefinitionVal> {
        match self {
            ConstantInfo::Definition(v) => Some(v),
            _ => None,
        }
    }
    /// Get the inductive value if this is an Inductive.
    pub fn to_inductive_val(&self) -> Option<&InductiveVal> {
        match self {
            ConstantInfo::Inductive(v) => Some(v),
            _ => None,
        }
    }
    /// Get the constructor value if this is a Constructor.
    pub fn to_constructor_val(&self) -> Option<&ConstructorVal> {
        match self {
            ConstantInfo::Constructor(v) => Some(v),
            _ => None,
        }
    }
    /// Get the recursor value if this is a Recursor.
    pub fn to_recursor_val(&self) -> Option<&RecursorVal> {
        match self {
            ConstantInfo::Recursor(v) => Some(v),
            _ => None,
        }
    }
    /// Get the quotient value if this is a Quotient.
    pub fn to_quotient_val(&self) -> Option<&QuotVal> {
        match self {
            ConstantInfo::Quotient(v) => Some(v),
            _ => None,
        }
    }
    /// Get the axiom value if this is an Axiom.
    pub fn to_axiom_val(&self) -> Option<&AxiomVal> {
        match self {
            ConstantInfo::Axiom(v) => Some(v),
            _ => None,
        }
    }
    /// Get the theorem value if this is a Theorem.
    pub fn to_theorem_val(&self) -> Option<&TheoremVal> {
        match self {
            ConstantInfo::Theorem(v) => Some(v),
            _ => None,
        }
    }
    /// Get the opaque value if this is an Opaque.
    pub fn to_opaque_val(&self) -> Option<&OpaqueVal> {
        match self {
            ConstantInfo::Opaque(v) => Some(v),
            _ => None,
        }
    }
    /// Check if this constant has a computable value.
    ///
    /// `allow_opaque` controls whether opaque definitions are considered.
    pub fn has_value(&self, allow_opaque: bool) -> bool {
        match self {
            ConstantInfo::Definition(_) | ConstantInfo::Theorem(_) => true,
            ConstantInfo::Opaque(_) => allow_opaque,
            _ => false,
        }
    }
    /// Get the value (body/proof) if available.
    pub fn value(&self) -> Option<&Expr> {
        match self {
            ConstantInfo::Definition(v) => Some(&v.value),
            ConstantInfo::Theorem(v) => Some(&v.value),
            ConstantInfo::Opaque(v) => Some(&v.value),
            _ => None,
        }
    }
    /// Get the reducibility hint.
    pub fn reducibility_hint(&self) -> ReducibilityHint {
        match self {
            ConstantInfo::Definition(v) => v.hints,
            ConstantInfo::Theorem(_) | ConstantInfo::Opaque(_) | ConstantInfo::Axiom(_) => {
                ReducibilityHint::Opaque
            }
            _ => ReducibilityHint::Opaque,
        }
    }
    /// Check if this declaration is unsafe.
    pub fn is_unsafe(&self) -> bool {
        match self {
            ConstantInfo::Axiom(v) => v.is_unsafe,
            ConstantInfo::Definition(v) => v.safety == DefinitionSafety::Unsafe,
            ConstantInfo::Opaque(v) => v.is_unsafe,
            ConstantInfo::Inductive(v) => v.is_unsafe,
            ConstantInfo::Constructor(v) => v.is_unsafe,
            ConstantInfo::Recursor(v) => v.is_unsafe,
            ConstantInfo::Theorem(_) | ConstantInfo::Quotient(_) => false,
        }
    }
    /// Check if this is a structure-like inductive (single constructor, not recursive).
    pub fn is_structure_like(&self) -> bool {
        match self {
            ConstantInfo::Inductive(v) => v.ctors.len() == 1 && !v.is_rec && v.num_indices == 0,
            _ => false,
        }
    }
}
impl ConstantInfo {
    /// Return the kind of this constant.
    pub fn kind(&self) -> ConstantKind {
        match self {
            ConstantInfo::Axiom(_) => ConstantKind::Axiom,
            ConstantInfo::Definition(_) => ConstantKind::Definition,
            ConstantInfo::Theorem(_) => ConstantKind::Theorem,
            ConstantInfo::Opaque(_) => ConstantKind::Opaque,
            ConstantInfo::Inductive(_) => ConstantKind::Inductive,
            ConstantInfo::Constructor(_) => ConstantKind::Constructor,
            ConstantInfo::Recursor(_) => ConstantKind::Recursor,
            ConstantInfo::Quotient(_) => ConstantKind::Quotient,
        }
    }
    /// Build a summary of this constant.
    pub fn summarize(&self) -> ConstantSummary {
        ConstantSummary {
            name: self.name().clone(),
            kind: self.kind(),
            num_level_params: self.level_params().len(),
            is_polymorphic: !self.level_params().is_empty(),
        }
    }
    /// Return the number of universe-level parameters.
    pub fn num_level_params(&self) -> usize {
        self.level_params().len()
    }
    /// Whether this constant is universe-polymorphic.
    pub fn is_polymorphic(&self) -> bool {
        !self.level_params().is_empty()
    }
    /// For constructors, return the parent inductive name.
    pub fn parent_inductive(&self) -> Option<&Name> {
        match self {
            ConstantInfo::Constructor(cv) => Some(&cv.induct),
            _ => None,
        }
    }
    /// For inductives, return the number of parameters.
    pub fn inductive_num_params(&self) -> Option<usize> {
        match self {
            ConstantInfo::Inductive(iv) => Some(iv.num_params as usize),
            _ => None,
        }
    }
    /// For inductives, return the number of indices.
    pub fn inductive_num_indices(&self) -> Option<usize> {
        match self {
            ConstantInfo::Inductive(iv) => Some(iv.num_indices as usize),
            _ => None,
        }
    }
    /// For inductives, return the list of constructor names.
    pub fn inductive_constructors(&self) -> Option<&[Name]> {
        match self {
            ConstantInfo::Inductive(iv) => Some(&iv.ctors),
            _ => None,
        }
    }
    /// For constructors, return the number of fields.
    pub fn ctor_num_fields(&self) -> Option<usize> {
        match self {
            ConstantInfo::Constructor(cv) => Some(cv.num_fields as usize),
            _ => None,
        }
    }
    /// For recursors, return the recursor rules.
    pub fn recursor_rules(&self) -> Option<&[RecursorRule]> {
        match self {
            ConstantInfo::Recursor(rv) => Some(&rv.rules),
            _ => None,
        }
    }
    /// Return a display string for this constant.
    pub fn display_string(&self) -> String {
        let kind = self.kind().as_str();
        if self.level_params().is_empty() {
            format!("{} {}", kind, self.name())
        } else {
            let params: Vec<String> = self
                .level_params()
                .iter()
                .map(|p| format!("{}", p))
                .collect();
            format!("{} {}.{{{}}} ", kind, self.name(), params.join(", "))
        }
    }
}
/// Safety classification for definitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DefinitionSafety {
    /// Safe: pure and terminating.
    Safe,
    /// Unsafe: may use unsafe declarations.
    Unsafe,
    /// Partial: may not terminate.
    Partial,
}
impl DefinitionSafety {
    /// Whether this safety level is safe (not partial or unsafe).
    pub fn is_safe(&self) -> bool {
        matches!(self, DefinitionSafety::Safe)
    }
    /// Whether this definition is partial (allows general recursion).
    pub fn is_partial(&self) -> bool {
        matches!(self, DefinitionSafety::Partial)
    }
    /// Return a human-readable string for the safety level.
    pub fn as_str(&self) -> &'static str {
        match self {
            DefinitionSafety::Safe => "safe",
            DefinitionSafety::Unsafe => "unsafe",
            DefinitionSafety::Partial => "partial",
        }
    }
}
/// A type-safe wrapper around a `u32` identifier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedId<T> {
    pub(super) id: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> TypedId<T> {
    /// Creates a new typed ID.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Returns the raw `u32` ID.
    pub fn raw(&self) -> u32 {
        self.id
    }
}
/// The kind of a declaration.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeclKind {
    /// A theorem with a proof body.
    Theorem,
    /// A definition with a definitional body.
    Definition,
    /// An opaque definition (body not unfolded during type-checking).
    Opaque,
    /// An axiom (no body).
    Axiom,
    /// A structure (record type).
    Structure,
    /// An inductive type.
    Inductive,
    /// A recursive definition.
    Recursive,
    /// An instance of a typeclass.
    Instance,
}
#[allow(dead_code)]
impl DeclKind {
    /// Returns `true` if the declaration has a body.
    pub fn has_body(self) -> bool {
        !matches!(self, DeclKind::Axiom)
    }
    /// Returns `true` if this is a type-defining declaration.
    pub fn is_type_decl(self) -> bool {
        matches!(self, DeclKind::Structure | DeclKind::Inductive)
    }
    /// Returns a short string label.
    pub fn label(self) -> &'static str {
        match self {
            DeclKind::Theorem => "theorem",
            DeclKind::Definition => "def",
            DeclKind::Opaque => "opaque",
            DeclKind::Axiom => "axiom",
            DeclKind::Structure => "structure",
            DeclKind::Inductive => "inductive",
            DeclKind::Recursive => "recdef",
            DeclKind::Instance => "instance",
        }
    }
}
/// Interns strings to save memory (each unique string stored once).
#[allow(dead_code)]
pub struct StringInterner {
    strings: Vec<String>,
    map: std::collections::HashMap<String, u32>,
}
#[allow(dead_code)]
impl StringInterner {
    /// Creates a new string interner.
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }
    /// Interns `s` and returns its ID.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), id);
        id
    }
    /// Returns the string for `id`.
    pub fn get(&self, id: u32) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }
    /// Returns the total number of interned strings.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}
/// A growable ring buffer with fixed maximum capacity.
#[allow(dead_code)]
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    count: usize,
    capacity: usize,
}
#[allow(dead_code)]
impl<T> RingBuffer<T> {
    /// Creates a new ring buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }
        Self {
            data,
            head: 0,
            tail: 0,
            count: 0,
            capacity,
        }
    }
    /// Pushes a value, overwriting the oldest if full.
    pub fn push(&mut self, val: T) {
        if self.count == self.capacity {
            self.data[self.head] = Some(val);
            self.head = (self.head + 1) % self.capacity;
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            self.data[self.tail] = Some(val);
            self.tail = (self.tail + 1) % self.capacity;
            self.count += 1;
        }
    }
    /// Pops the oldest value.
    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        val
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Returns `true` if at capacity.
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }
}
/// Inductive type declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct InductiveVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Number of parameters (uniform across constructors).
    pub num_params: u32,
    /// Number of indices (vary per constructor).
    pub num_indices: u32,
    /// All inductive types in mutual declaration.
    pub all: Vec<Name>,
    /// Constructor names.
    pub ctors: Vec<Name>,
    /// Number of nested inductive uses.
    pub num_nested: u32,
    /// Whether this type is recursively defined.
    pub is_rec: bool,
    /// Whether this is unsafe.
    pub is_unsafe: bool,
    /// Whether this is reflexive (all arguments are params).
    pub is_reflexive: bool,
    /// Whether this type lives in Prop.
    pub is_prop: bool,
}
/// A sequence number that can be compared for ordering.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeqNum(u64);
#[allow(dead_code)]
impl SeqNum {
    /// Creates sequence number zero.
    pub const ZERO: SeqNum = SeqNum(0);
    /// Advances the sequence number by one.
    pub fn next(self) -> SeqNum {
        SeqNum(self.0 + 1)
    }
    /// Returns the raw value.
    pub fn value(self) -> u64 {
        self.0
    }
}
/// A bidirectional map between two types.
#[allow(dead_code)]
pub struct BiMap<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> {
    forward: std::collections::HashMap<A, B>,
    backward: std::collections::HashMap<B, A>,
}
#[allow(dead_code)]
impl<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> BiMap<A, B> {
    /// Creates an empty bidirectional map.
    pub fn new() -> Self {
        Self {
            forward: std::collections::HashMap::new(),
            backward: std::collections::HashMap::new(),
        }
    }
    /// Inserts a pair `(a, b)`.
    pub fn insert(&mut self, a: A, b: B) {
        self.forward.insert(a.clone(), b.clone());
        self.backward.insert(b, a);
    }
    /// Looks up `b` for a given `a`.
    pub fn get_b(&self, a: &A) -> Option<&B> {
        self.forward.get(a)
    }
    /// Looks up `a` for a given `b`.
    pub fn get_a(&self, b: &B) -> Option<&A> {
        self.backward.get(b)
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
}
/// Opaque declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct OpaqueVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Hidden body.
    pub value: Expr,
    /// Whether this is unsafe.
    pub is_unsafe: bool,
    /// Names in mutual declaration group.
    pub all: Vec<Name>,
}
/// Visibility of a declaration.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclVisibility {
    /// Visible everywhere.
    Public,
    /// Visible only within the current namespace.
    Protected,
    /// Visible only within the current file.
    Private,
}
#[allow(dead_code)]
impl DeclVisibility {
    /// Returns `true` if the declaration is visible from outside its module.
    pub fn is_externally_visible(self) -> bool {
        matches!(self, DeclVisibility::Public)
    }
}
/// A counter that dispenses monotonically increasing `TypedId` values.
#[allow(dead_code)]
pub struct IdDispenser<T> {
    next: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> IdDispenser<T> {
    /// Creates a new dispenser starting from zero.
    pub fn new() -> Self {
        Self {
            next: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Dispenses the next ID.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> TypedId<T> {
        let id = TypedId::new(self.next);
        self.next += 1;
        id
    }
    /// Returns the number of IDs dispensed.
    pub fn count(&self) -> u32 {
        self.next
    }
}
/// Tracks the dependency graph between declarations.
#[allow(dead_code)]
pub struct DeclDependencies {
    /// `deps[i]` is the set of declaration indices that `i` depends on.
    deps: Vec<std::collections::HashSet<usize>>,
}
#[allow(dead_code)]
impl DeclDependencies {
    /// Creates a dependency table for `n` declarations.
    pub fn new(n: usize) -> Self {
        Self {
            deps: vec![std::collections::HashSet::new(); n],
        }
    }
    /// Adds a dependency: `dependent` depends on `dependency`.
    pub fn add(&mut self, dependent: usize, dependency: usize) {
        if dependent < self.deps.len() {
            self.deps[dependent].insert(dependency);
        }
    }
    /// Returns the dependencies of `idx`.
    pub fn deps_of(&self, idx: usize) -> &std::collections::HashSet<usize> {
        static EMPTY: std::sync::OnceLock<std::collections::HashSet<usize>> =
            std::sync::OnceLock::new();
        self.deps
            .get(idx)
            .unwrap_or_else(|| EMPTY.get_or_init(std::collections::HashSet::new))
    }
    /// Returns `true` if `dependent` directly depends on `dependency`.
    pub fn directly_depends(&self, dependent: usize, dependency: usize) -> bool {
        self.deps_of(dependent).contains(&dependency)
    }
    /// Returns the total number of dependency edges.
    pub fn total_edges(&self) -> usize {
        self.deps.iter().map(|s| s.len()).sum()
    }
}
/// A set of non-overlapping integer intervals.
#[allow(dead_code)]
pub struct IntervalSet {
    intervals: Vec<(i64, i64)>,
}
#[allow(dead_code)]
impl IntervalSet {
    /// Creates an empty interval set.
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }
    /// Adds the interval `[lo, hi]` to the set.
    pub fn add(&mut self, lo: i64, hi: i64) {
        if lo > hi {
            return;
        }
        let mut new_lo = lo;
        let mut new_hi = hi;
        let mut i = 0;
        while i < self.intervals.len() {
            let (il, ih) = self.intervals[i];
            if ih < new_lo - 1 {
                i += 1;
                continue;
            }
            if il > new_hi + 1 {
                break;
            }
            new_lo = new_lo.min(il);
            new_hi = new_hi.max(ih);
            self.intervals.remove(i);
        }
        self.intervals.insert(i, (new_lo, new_hi));
    }
    /// Returns `true` if `x` is in the set.
    pub fn contains(&self, x: i64) -> bool {
        self.intervals.iter().any(|&(lo, hi)| x >= lo && x <= hi)
    }
    /// Returns the number of intervals.
    pub fn num_intervals(&self) -> usize {
        self.intervals.len()
    }
    /// Returns the total count of integers covered.
    pub fn cardinality(&self) -> i64 {
        self.intervals.iter().map(|&(lo, hi)| hi - lo + 1).sum()
    }
}
/// A generation counter for validity tracking.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Generation(u32);
#[allow(dead_code)]
impl Generation {
    /// The initial generation.
    pub const INITIAL: Generation = Generation(0);
    /// Advances to the next generation.
    pub fn advance(self) -> Generation {
        Generation(self.0 + 1)
    }
    /// Returns the raw generation number.
    pub fn number(self) -> u32 {
        self.0
    }
}
/// A FIFO work queue.
#[allow(dead_code)]
pub struct WorkQueue<T> {
    items: std::collections::VecDeque<T>,
}
#[allow(dead_code)]
impl<T> WorkQueue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            items: std::collections::VecDeque::new(),
        }
    }
    /// Enqueues a work item.
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }
    /// Dequeues the next work item.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// Definition declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct DefinitionVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Definition body.
    pub value: Expr,
    /// Reducibility hints for unfolding strategy.
    pub hints: ReducibilityHint,
    /// Safety classification.
    pub safety: DefinitionSafety,
    /// Names in mutual definition group.
    pub all: Vec<Name>,
}
/// Quotient declaration value.
#[derive(Clone, Debug, PartialEq)]
pub struct QuotVal {
    /// Common fields.
    pub common: ConstantVal,
    /// Which quotient component.
    pub kind: QuotKind,
}
/// A summary of a declaration's key properties.
#[derive(Debug, Clone)]
pub struct ConstantSummary {
    /// The name of the constant.
    pub name: Name,
    /// The kind of constant.
    pub kind: ConstantKind,
    /// Number of universe level parameters.
    pub num_level_params: usize,
    /// Whether this is universe-polymorphic.
    pub is_polymorphic: bool,
}
/// Tracks the frequency of items.
#[allow(dead_code)]
pub struct FrequencyTable<T: std::hash::Hash + Eq + Clone> {
    counts: std::collections::HashMap<T, u64>,
}
#[allow(dead_code)]
impl<T: std::hash::Hash + Eq + Clone> FrequencyTable<T> {
    /// Creates a new empty frequency table.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Records one occurrence of `item`.
    pub fn record(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }
    /// Returns the frequency of `item`.
    pub fn freq(&self, item: &T) -> u64 {
        self.counts.get(item).copied().unwrap_or(0)
    }
    /// Returns the item with the highest frequency.
    pub fn most_frequent(&self) -> Option<(&T, u64)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    /// Returns the total number of recordings.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Returns the number of distinct items.
    pub fn distinct(&self) -> usize {
        self.counts.len()
    }
}
/// The kind of constant declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantKind {
    /// An axiom declaration.
    Axiom,
    /// A definition declaration.
    Definition,
    /// A theorem declaration.
    Theorem,
    /// An opaque definition.
    Opaque,
    /// An inductive type declaration.
    Inductive,
    /// A constructor of an inductive type.
    Constructor,
    /// A recursor (eliminator) for an inductive type.
    Recursor,
    /// A quotient type declaration.
    Quotient,
}
impl ConstantKind {
    /// Return a human-readable name for the kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            ConstantKind::Axiom => "axiom",
            ConstantKind::Definition => "definition",
            ConstantKind::Theorem => "theorem",
            ConstantKind::Opaque => "opaque",
            ConstantKind::Inductive => "inductive",
            ConstantKind::Constructor => "constructor",
            ConstantKind::Recursor => "recursor",
            ConstantKind::Quotient => "quotient",
        }
    }
    /// Whether this kind can have a proof term (body).
    pub fn has_body(&self) -> bool {
        matches!(
            self,
            ConstantKind::Definition | ConstantKind::Theorem | ConstantKind::Opaque
        )
    }
    /// Whether this kind is a structural element of an inductive type.
    pub fn is_inductive_family(&self) -> bool {
        matches!(
            self,
            ConstantKind::Inductive | ConstantKind::Constructor | ConstantKind::Recursor
        )
    }
}
/// Filters declarations by kind, namespace, or attributes.
#[allow(dead_code)]
pub struct DeclFilter {
    allowed_kinds: Option<Vec<DeclKind>>,
    ns_prefix: Option<String>,
    required_attrs: Vec<DeclAttr>,
}
#[allow(dead_code)]
impl DeclFilter {
    /// Creates a filter that accepts everything.
    pub fn accept_all() -> Self {
        Self {
            allowed_kinds: None,
            ns_prefix: None,
            required_attrs: Vec::new(),
        }
    }
    /// Restricts to specific kinds.
    pub fn with_kinds(mut self, kinds: Vec<DeclKind>) -> Self {
        self.allowed_kinds = Some(kinds);
        self
    }
    /// Restricts to declarations in the given namespace prefix.
    pub fn in_namespace(mut self, prefix: impl Into<String>) -> Self {
        self.ns_prefix = Some(prefix.into());
        self
    }
    /// Requires a specific attribute.
    pub fn with_attr(mut self, attr: DeclAttr) -> Self {
        self.required_attrs.push(attr);
        self
    }
    /// Tests whether a signature passes the filter.
    pub fn accepts(&self, sig: &DeclSignature, kind: DeclKind, attrs: &[DeclAttr]) -> bool {
        if let Some(ref kinds) = self.allowed_kinds {
            if !kinds.contains(&kind) {
                return false;
            }
        }
        if let Some(ref prefix) = self.ns_prefix {
            if !sig.name.starts_with(prefix.as_str()) {
                return false;
            }
        }
        for req in &self.required_attrs {
            if !attrs.contains(req) {
                return false;
            }
        }
        true
    }
}
/// A simple LRU cache backed by a linked list + hash map.
#[allow(dead_code)]
pub struct SimpleLruCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: std::collections::HashMap<K, usize>,
    keys: Vec<K>,
    vals: Vec<V>,
    order: Vec<usize>,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> SimpleLruCache<K, V> {
    /// Creates a new LRU cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            keys: Vec::new(),
            vals: Vec::new(),
            order: Vec::new(),
        }
    }
    /// Inserts or updates a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.vals[idx] = val;
            self.order.retain(|&x| x != idx);
            self.order.insert(0, idx);
            return;
        }
        if self.keys.len() >= self.capacity {
            let evict_idx = *self
                .order
                .last()
                .expect("order list must be non-empty before eviction");
            self.map.remove(&self.keys[evict_idx]);
            self.order.pop();
            self.keys[evict_idx] = key.clone();
            self.vals[evict_idx] = val;
            self.map.insert(key, evict_idx);
            self.order.insert(0, evict_idx);
        } else {
            let idx = self.keys.len();
            self.keys.push(key.clone());
            self.vals.push(val);
            self.map.insert(key, idx);
            self.order.insert(0, idx);
        }
    }
    /// Returns a reference to the value for `key`, promoting it.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let idx = *self.map.get(key)?;
        self.order.retain(|&x| x != idx);
        self.order.insert(0, idx);
        Some(&self.vals[idx])
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
/// The type signature of a declaration (name + type, no body).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DeclSignature {
    /// The fully-qualified name.
    pub name: String,
    /// String representation of the type.
    pub ty: String,
    /// Universe parameters.
    pub uparams: Vec<String>,
}
#[allow(dead_code)]
impl DeclSignature {
    /// Creates a new declaration signature.
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
            uparams: Vec::new(),
        }
    }
    /// Adds a universe parameter.
    pub fn with_uparam(mut self, u: impl Into<String>) -> Self {
        self.uparams.push(u.into());
        self
    }
    /// Returns `true` if this is a universe-polymorphic signature.
    pub fn is_universe_poly(&self) -> bool {
        !self.uparams.is_empty()
    }
}
/// A key-value store for diagnostic metadata.
#[allow(dead_code)]
pub struct DiagMeta {
    pub(super) entries: Vec<(String, String)>,
}
#[allow(dead_code)]
impl DiagMeta {
    /// Creates an empty metadata store.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Adds a key-value pair.
    pub fn add(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.entries.push((key.into(), val.into()));
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
