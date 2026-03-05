//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::Cell;
use std::collections::HashMap;

/// An observer that logs RC events.
#[allow(dead_code)]
pub struct RcObserver {
    events: Vec<RcEvent>,
    max_events: usize,
}
#[allow(dead_code)]
impl RcObserver {
    /// Create a new observer.
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }
    /// Record an event.
    pub fn record(&mut self, id: u64, kind: RcEventKind, count_after: u64) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(RcEvent {
            id,
            kind,
            count_after,
        });
    }
    /// Get all events.
    pub fn events(&self) -> &[RcEvent] {
        &self.events
    }
    /// Count events of a specific kind.
    pub fn count_kind(&self, kind: &RcEventKind) -> usize {
        self.events.iter().filter(|e| &e.kind == kind).count()
    }
    /// Number of Drop events.
    pub fn drop_count(&self) -> usize {
        self.count_kind(&RcEventKind::Drop)
    }
    /// Number of Alloc events.
    pub fn alloc_count(&self) -> usize {
        self.count_kind(&RcEventKind::Alloc)
    }
    /// Clear the event log.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// Results of RC elision analysis for a function.
#[derive(Clone, Debug)]
pub struct RcElisionAnalysis {
    /// Hints for each variable in the function.
    pub variable_hints: HashMap<String, RcElisionHint>,
    /// Total number of RC operations that can be elided.
    pub elided_ops: u32,
    /// Total number of RC operations remaining.
    pub remaining_ops: u32,
    /// Whether the function is fully linear (no RC needed).
    pub fully_linear: bool,
}
impl RcElisionAnalysis {
    /// Create a new empty analysis.
    pub fn new() -> Self {
        RcElisionAnalysis {
            variable_hints: HashMap::new(),
            elided_ops: 0,
            remaining_ops: 0,
            fully_linear: true,
        }
    }
    /// Add a hint for a variable.
    pub fn add_hint(&mut self, var: String, hint: RcElisionHint) {
        if hint == RcElisionHint::None {
            self.fully_linear = false;
        }
        self.variable_hints.insert(var, hint);
    }
    /// Get the hint for a variable.
    pub fn get_hint(&self, var: &str) -> RcElisionHint {
        self.variable_hints
            .get(var)
            .cloned()
            .unwrap_or(RcElisionHint::None)
    }
    /// Calculate the elision ratio.
    pub fn elision_ratio(&self) -> f64 {
        let total = self.elided_ops + self.remaining_ops;
        if total == 0 {
            return 1.0;
        }
        self.elided_ops as f64 / total as f64
    }
    /// Merge with another analysis (for nested scopes).
    pub fn merge(&mut self, other: &RcElisionAnalysis) {
        for (var, hint) in &other.variable_hints {
            let existing = self.get_hint(var);
            let combined = existing.combine(hint);
            self.add_hint(var.clone(), combined);
        }
        self.elided_ops += other.elided_ops;
        self.remaining_ops += other.remaining_ops;
        self.fully_linear = self.fully_linear && other.fully_linear;
    }
}
/// A map where values are reference counted; removing all refs drops the value.
#[allow(dead_code)]
pub struct RefcountedMap<K: Eq + std::hash::Hash + Clone, V: Clone> {
    map: HashMap<K, (V, u32)>,
}
#[allow(dead_code)]
impl<K: Eq + std::hash::Hash + Clone, V: Clone> RefcountedMap<K, V> {
    /// Create an empty map.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// Insert a value with refcount 1.
    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, (value, 1));
    }
    /// Increment the refcount of a key.
    pub fn inc_ref(&mut self, key: &K) {
        if let Some((_, rc)) = self.map.get_mut(key) {
            *rc = rc.saturating_add(1);
        }
    }
    /// Decrement the refcount. Returns true if the value was dropped.
    pub fn dec_ref(&mut self, key: &K) -> bool {
        if let Some((_, rc)) = self.map.get_mut(key) {
            if *rc > 0 {
                *rc -= 1;
                if *rc == 0 {
                    self.map.remove(key);
                    return true;
                }
            }
        }
        false
    }
    /// Get a reference to the value.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|(v, _)| v)
    }
    /// Get the refcount of a key.
    pub fn refcount(&self, key: &K) -> u32 {
        self.map.get(key).map(|(_, rc)| *rc).unwrap_or(0)
    }
    /// Number of live entries.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RcEventKind {
    Inc,
    Dec,
    Drop,
    Alloc,
}
/// A reference counter that saturates at `MAX` (immortal objects).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StickyRc {
    pub(super) count: u32,
    pub(super) max: u32,
}
#[allow(dead_code)]
impl StickyRc {
    /// Create a counter with `initial` count and given `max`.
    pub fn new(initial: u32, max: u32) -> Self {
        Self {
            count: initial.min(max),
            max,
        }
    }
    /// Increment (saturates at `max`).
    pub fn inc(&mut self) {
        if self.count < self.max {
            self.count += 1;
        }
    }
    /// Decrement (saturates at 0). Returns true if now zero.
    pub fn dec(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
        }
        self.count == 0
    }
    /// Whether at max (immortal).
    pub fn is_immortal(&self) -> bool {
        self.count >= self.max
    }
    /// Current count.
    pub fn count(&self) -> u32 {
        self.count
    }
    /// Whether the count is zero.
    pub fn is_zero(&self) -> bool {
        self.count == 0
    }
}
/// The inner data shared between all `Rc` references.
pub(super) struct RcInner<T> {
    /// The actual value.
    pub(super) value: T,
    /// Weak reference count (strong count is tracked by the outer std::rc::Rc).
    weak_count: Cell<u32>,
}
impl<T> RcInner<T> {
    fn new(value: T) -> Self {
        RcInner {
            value,
            weak_count: Cell::new(0),
        }
    }
}
/// An atomic reference-counted pointer for shared data.
///
/// This is similar to `std::sync::Arc` but with additional features
/// for the OxiLean runtime.
pub struct RtArc<T> {
    /// Shared inner data; uses std::sync::Arc for proper reference-count sharing.
    pub(super) inner: std::sync::Arc<ArcInner<T>>,
}
impl<T> RtArc<T> {
    /// Create a new `RtArc` wrapping the given value.
    pub fn new(value: T) -> Self {
        RtArc {
            inner: std::sync::Arc::new(ArcInner::new(value)),
        }
    }
    /// Get the current strong reference count.
    pub fn strong_count(&self) -> u32 {
        std::sync::Arc::strong_count(&self.inner) as u32
    }
    /// Get the current weak reference count.
    pub fn weak_count(&self) -> u32 {
        self.inner
            .weak_count
            .load(std::sync::atomic::Ordering::Acquire)
    }
    /// Check if this is the only strong reference.
    pub fn is_unique(&self) -> bool {
        self.strong_count() == 1
    }
    /// Get a reference to the inner value.
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        &self.inner.value
    }
    /// Try to get a mutable reference if this is the only strong reference.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.weak_count() == 0 {
            std::sync::Arc::get_mut(&mut self.inner).map(|r| &mut r.value)
        } else {
            None
        }
    }
    /// Try to unwrap the value if this is the only reference.
    pub fn try_unwrap(self) -> Result<T, Self> {
        if self.weak_count() == 0 {
            match std::sync::Arc::try_unwrap(self.inner) {
                Ok(inner) => Ok(inner.value),
                Err(inner) => Err(RtArc { inner }),
            }
        } else {
            Err(self)
        }
    }
    /// Create a clone sharing the same allocation (increments strong count).
    pub fn clone_arc(&self) -> Self {
        RtArc {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}
/// The inner data shared between all `RtArc` references.
pub(super) struct ArcInner<T> {
    /// The actual value.
    pub(super) value: T,
    /// Weak reference count (strong count is tracked by the outer std::sync::Arc).
    weak_count: std::sync::atomic::AtomicU32,
}
impl<T> ArcInner<T> {
    fn new(value: T) -> Self {
        ArcInner {
            value,
            weak_count: std::sync::atomic::AtomicU32::new(0),
        }
    }
}
/// Compact reference count tracking using a bitmask.
/// Supports up to 64 objects, each with a 1-bit "alive" flag.
#[allow(dead_code)]
pub struct RcBitmask {
    pub(super) mask: u64,
}
#[allow(dead_code)]
impl RcBitmask {
    /// Create an empty bitmask (all dead).
    pub fn new() -> Self {
        Self { mask: 0 }
    }
    /// Mark slot `i` as alive (0 <= i < 64).
    pub fn set_alive(&mut self, i: u32) {
        debug_assert!(i < 64);
        self.mask |= 1u64 << i;
    }
    /// Mark slot `i` as dead.
    pub fn set_dead(&mut self, i: u32) {
        debug_assert!(i < 64);
        self.mask &= !(1u64 << i);
    }
    /// Check if slot `i` is alive.
    pub fn is_alive(&self, i: u32) -> bool {
        (self.mask >> i) & 1 == 1
    }
    /// Count alive slots.
    pub fn alive_count(&self) -> u32 {
        self.mask.count_ones()
    }
    /// Count dead slots.
    pub fn dead_count(&self) -> u32 {
        self.mask.count_zeros()
    }
    /// Find the first dead slot (for allocation).
    pub fn first_dead(&self) -> Option<u32> {
        let inv = !self.mask;
        if inv == 0 {
            None
        } else {
            Some(inv.trailing_zeros())
        }
    }
    /// Find the first alive slot.
    pub fn first_alive(&self) -> Option<u32> {
        if self.mask == 0 {
            None
        } else {
            Some(self.mask.trailing_zeros())
        }
    }
    /// Raw bitmask value.
    pub fn raw(&self) -> u64 {
        self.mask
    }
}
/// Borrow state for runtime borrow checking.
///
/// This tracks whether a value is currently borrowed mutably or immutably,
/// similar to `RefCell` but for the runtime system.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorrowState {
    /// Value is not borrowed.
    Unborrowed,
    /// Value is borrowed immutably N times.
    ImmutableBorrow(u32),
    /// Value is borrowed mutably.
    MutableBorrow,
}
/// A node in the RC graph.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RcGraphNode {
    pub id: u32,
    pub data: u64,
    pub out_edges: Vec<u32>,
}
/// Index into an `RcPool`.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RcPoolIdx(pub usize);
/// A copy-on-write smart pointer.
///
/// `CowBox<T>` wraps a value that is shared (read-only) until a mutation
/// is requested, at which point it copies the value to create a unique owner.
pub struct CowBox<T> {
    /// The inner RC.
    pub(super) inner: Rc<T>,
    /// Whether a COW copy has been made.
    pub(super) copied: Cell<bool>,
}
impl<T: Clone> CowBox<T> {
    /// Create a new CowBox.
    pub fn new(value: T) -> Self {
        CowBox {
            inner: Rc::new(value),
            copied: Cell::new(false),
        }
    }
    /// Get a read-only reference.
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
    /// Get a mutable reference, copying if necessary.
    #[allow(clippy::should_implement_trait)]
    pub fn as_mut(&mut self) -> &mut T {
        if !self.inner.is_unique() {
            let new_value = self.inner.as_ref().clone();
            self.inner = Rc::new(new_value);
            self.copied.set(true);
        }
        self.inner
            .get_mut()
            .unwrap_or_else(|| unreachable!("COW clone guarantees unique ownership before get_mut"))
    }
    /// Check if a COW copy has been made.
    pub fn was_copied(&self) -> bool {
        self.copied.get()
    }
    /// Check if this is the unique owner.
    pub fn is_unique(&self) -> bool {
        self.inner.is_unique()
    }
    /// Unwrap the value, cloning if not unique.
    pub fn into_owned(self) -> T {
        match self.inner.try_unwrap() {
            Ok(v) => v,
            Err(rc) => rc.as_ref().clone(),
        }
    }
}
/// Records an ownership transfer event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct OwnershipEvent {
    pub object_id: u64,
    pub from_owner: String,
    pub to_owner: String,
    pub timestamp: u64,
}
/// A log of ownership transfer events for debugging.
#[allow(dead_code)]
pub struct OwnershipLog {
    events: Vec<OwnershipEvent>,
    max_events: usize,
}
#[allow(dead_code)]
impl OwnershipLog {
    /// Create a new log.
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }
    /// Record a transfer.
    pub fn record_transfer(&mut self, object_id: u64, from: &str, to: &str, ts: u64) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(OwnershipEvent {
            object_id,
            from_owner: from.to_string(),
            to_owner: to.to_string(),
            timestamp: ts,
        });
    }
    /// Events for a specific object.
    pub fn events_for(&self, object_id: u64) -> Vec<&OwnershipEvent> {
        self.events
            .iter()
            .filter(|e| e.object_id == object_id)
            .collect()
    }
    /// Total events.
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
/// Policy for how reference counting behaves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RcPolicy {
    /// Standard reference counting (increment on share, decrement on drop).
    Standard,
    /// Deferred reference counting (batch decrements at scope boundaries).
    Deferred,
    /// Aggressive elision (skip RC for provably linear values).
    AggressiveElision,
    /// No reference counting (for debugging, everything leaks).
    Disabled,
}
impl RcPolicy {
    /// Check if this policy uses deferred decrements.
    pub fn is_deferred(&self) -> bool {
        matches!(self, RcPolicy::Deferred)
    }
    /// Check if this policy allows elision.
    pub fn allows_elision(&self) -> bool {
        matches!(self, RcPolicy::AggressiveElision | RcPolicy::Deferred)
    }
    /// Check if RC is enabled.
    pub fn is_enabled(&self) -> bool {
        !matches!(self, RcPolicy::Disabled)
    }
}
/// A node in the GC graph.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct GcNode {
    /// Direct references to other nodes.
    pub refs: Vec<u32>,
    /// Whether this node is a GC root.
    pub is_root: bool,
    /// Whether the node has been marked (reachable from roots).
    pub marked: bool,
}
/// A log entry for RC changes.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RcEvent {
    pub id: u64,
    pub kind: RcEventKind,
    pub count_after: u64,
}
/// A weak reference for atomic reference counting.
pub struct ArcWeak<T> {
    /// Whether the value is still alive.
    pub(super) alive: std::sync::atomic::AtomicBool,
    /// A copy of the value for upgrade attempts.
    value: Option<T>,
}
impl<T: Clone + Send + Sync> ArcWeak<T> {
    /// Create a new weak reference from a strong reference.
    pub fn from_arc(arc: &RtArc<T>) -> Self {
        arc.inner
            .weak_count
            .fetch_add(1, std::sync::atomic::Ordering::Release);
        ArcWeak {
            alive: std::sync::atomic::AtomicBool::new(true),
            value: Some(arc.inner.value.clone()),
        }
    }
    /// Try to upgrade to a strong reference.
    pub fn upgrade(&self) -> Option<RtArc<T>> {
        if self.alive.load(std::sync::atomic::Ordering::Acquire) {
            self.value.as_ref().map(|v| RtArc::new(v.clone()))
        } else {
            None
        }
    }
    /// Check if the referenced value is still alive.
    pub fn is_alive(&self) -> bool {
        self.alive.load(std::sync::atomic::Ordering::Acquire)
    }
    /// Mark as dead.
    pub fn invalidate(&self) {
        self.alive
            .store(false, std::sync::atomic::Ordering::Release);
    }
}
/// A borrow flag for tracking runtime borrows.
pub struct BorrowFlag {
    /// Current borrow state.
    state: Cell<BorrowState>,
}
impl BorrowFlag {
    /// Create a new unborrowed flag.
    pub fn new() -> Self {
        BorrowFlag {
            state: Cell::new(BorrowState::Unborrowed),
        }
    }
    /// Try to acquire an immutable borrow.
    pub fn try_borrow(&self) -> bool {
        match self.state.get() {
            BorrowState::Unborrowed => {
                self.state.set(BorrowState::ImmutableBorrow(1));
                true
            }
            BorrowState::ImmutableBorrow(n) => {
                self.state.set(BorrowState::ImmutableBorrow(n + 1));
                true
            }
            BorrowState::MutableBorrow => false,
        }
    }
    /// Release an immutable borrow.
    pub fn release_borrow(&self) {
        match self.state.get() {
            BorrowState::ImmutableBorrow(1) => {
                self.state.set(BorrowState::Unborrowed);
            }
            BorrowState::ImmutableBorrow(n) if n > 1 => {
                self.state.set(BorrowState::ImmutableBorrow(n - 1));
            }
            _ => {}
        }
    }
    /// Try to acquire a mutable borrow.
    pub fn try_borrow_mut(&self) -> bool {
        match self.state.get() {
            BorrowState::Unborrowed => {
                self.state.set(BorrowState::MutableBorrow);
                true
            }
            _ => false,
        }
    }
    /// Release a mutable borrow.
    pub fn release_borrow_mut(&self) {
        if self.state.get() == BorrowState::MutableBorrow {
            self.state.set(BorrowState::Unborrowed);
        }
    }
    /// Get the current borrow state.
    pub fn state(&self) -> BorrowState {
        self.state.get()
    }
    /// Check if the value is currently borrowed.
    pub fn is_borrowed(&self) -> bool {
        self.state.get() != BorrowState::Unborrowed
    }
    /// Check if the value is mutably borrowed.
    pub fn is_mutably_borrowed(&self) -> bool {
        self.state.get() == BorrowState::MutableBorrow
    }
}
/// A pool of reference-counted slot values, indexed by integer ID.
#[allow(dead_code)]
pub struct RcPool<T: Clone> {
    slots: Vec<Option<T>>,
    refcounts: Vec<u32>,
    free: Vec<usize>,
    alloc_count: u64,
}
#[allow(dead_code)]
impl<T: Clone> RcPool<T> {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            refcounts: Vec::new(),
            free: Vec::new(),
            alloc_count: 0,
        }
    }
    /// Insert a value, returning an index with refcount 1.
    pub fn insert(&mut self, value: T) -> RcPoolIdx {
        self.alloc_count += 1;
        if let Some(idx) = self.free.pop() {
            self.slots[idx] = Some(value);
            self.refcounts[idx] = 1;
            RcPoolIdx(idx)
        } else {
            let idx = self.slots.len();
            self.slots.push(Some(value));
            self.refcounts.push(1);
            RcPoolIdx(idx)
        }
    }
    /// Increment the refcount of an index.
    pub fn inc_ref(&mut self, idx: RcPoolIdx) {
        if let Some(rc) = self.refcounts.get_mut(idx.0) {
            *rc = rc.saturating_add(1);
        }
    }
    /// Decrement the refcount. Returns `true` if the object was freed.
    pub fn dec_ref(&mut self, idx: RcPoolIdx) -> bool {
        if let Some(rc) = self.refcounts.get_mut(idx.0) {
            if *rc == 0 {
                return false;
            }
            *rc -= 1;
            if *rc == 0 {
                self.slots[idx.0] = None;
                self.free.push(idx.0);
                return true;
            }
        }
        false
    }
    /// Get the refcount of an index.
    pub fn refcount(&self, idx: RcPoolIdx) -> u32 {
        self.refcounts.get(idx.0).copied().unwrap_or(0)
    }
    /// Get a reference to the value.
    pub fn get(&self, idx: RcPoolIdx) -> Option<&T> {
        self.slots.get(idx.0)?.as_ref()
    }
    /// Get a mutable reference (unique access).
    pub fn get_mut(&mut self, idx: RcPoolIdx) -> Option<&mut T> {
        if self.refcount(idx) != 1 {
            return None;
        }
        self.slots.get_mut(idx.0)?.as_mut()
    }
    /// Clone-on-write: if refcount > 1, clone the value into a new slot.
    pub fn cow(&mut self, idx: RcPoolIdx) -> Option<RcPoolIdx> {
        let rc = self.refcount(idx);
        if rc <= 1 {
            return Some(idx);
        }
        let value = self.get(idx)?.clone();
        self.dec_ref(idx);
        Some(self.insert(value))
    }
    /// Number of live slots.
    pub fn live_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
    /// Total allocation count.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
    /// Capacity.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
}
/// A non-atomic reference-counted pointer.
///
/// This is similar to `std::rc::Rc` but with additional features:
/// - Elision hints from the compiler
/// - Statistics tracking
/// - Unique-owner optimization
///
/// ## Safety
///
/// This type is `!Send` and `!Sync` — it must only be used on a single thread.
pub struct Rc<T> {
    /// Shared inner data; uses std::rc::Rc for proper reference-count sharing.
    pub(super) inner: std::rc::Rc<RcInner<T>>,
}
impl<T> Rc<T> {
    /// Create a new `Rc` wrapping the given value.
    pub fn new(value: T) -> Self {
        Rc {
            inner: std::rc::Rc::new(RcInner::new(value)),
        }
    }
    /// Get the current strong reference count.
    pub fn strong_count(&self) -> u32 {
        std::rc::Rc::strong_count(&self.inner) as u32
    }
    /// Get the current weak reference count.
    pub fn weak_count(&self) -> u32 {
        self.inner.weak_count.get()
    }
    /// Check if this is the only strong reference.
    pub fn is_unique(&self) -> bool {
        self.strong_count() == 1
    }
    /// Try to get a mutable reference if this is the only strong reference.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.weak_count() == 0 {
            std::rc::Rc::get_mut(&mut self.inner).map(|r| &mut r.value)
        } else {
            None
        }
    }
    /// Get a reference to the inner value.
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        &self.inner.value
    }
    /// Try to unwrap the value if this is the only reference.
    pub fn try_unwrap(self) -> Result<T, Self> {
        if self.weak_count() == 0 {
            match std::rc::Rc::try_unwrap(self.inner) {
                Ok(inner) => Ok(inner.value),
                Err(inner) => Err(Rc { inner }),
            }
        } else {
            Err(self)
        }
    }
    /// Create a clone sharing the same allocation (increments strong count).
    pub fn clone_rc(&self) -> Self {
        Rc {
            inner: std::rc::Rc::clone(&self.inner),
        }
    }
    /// Increment the weak count.
    fn inc_weak(&self) {
        let c = self.inner.weak_count.get();
        self.inner.weak_count.set(c.saturating_add(1));
    }
    /// Decrement the weak count.
    fn dec_weak(&self) {
        let c = self.inner.weak_count.get();
        if c > 0 {
            self.inner.weak_count.set(c - 1);
        }
    }
}
/// Hints from the compiler about when RC operations can be elided.
///
/// During compilation, the OxiLean compiler analyzes usage patterns
/// to determine when reference counting operations can be skipped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RcElisionHint {
    /// No elision possible — standard RC behavior.
    None,
    /// Value is consumed exactly once (linear use).
    /// RC increment/decrement can both be elided.
    LinearUse,
    /// Value is created and immediately consumed (ephemeral).
    /// No RC needed at all.
    Ephemeral,
    /// Value is borrowed (read-only reference).
    /// Only the borrow needs tracking, not the object itself.
    Borrowed,
    /// Value is owned uniquely by the current scope.
    /// Mutations can be done in-place.
    UniqueOwner,
    /// Value is shared but immutable.
    /// RC operations needed but no copy-on-write.
    SharedImmutable,
    /// Value is in a tail position (will be returned).
    /// The caller's reference can be transferred.
    TailPosition,
    /// Value is a function argument that is not captured.
    /// RC can be deferred to call site.
    UncapturedArg,
    /// Value is stored in a data structure and then forgotten.
    /// RC can be merged with the structure's RC.
    StructField,
    /// Value is part of a known-dead path (e.g., unreachable branch).
    /// All RC operations can be elided.
    DeadPath,
}
impl RcElisionHint {
    /// Check if this hint allows eliding the RC increment.
    pub fn can_elide_inc(&self) -> bool {
        matches!(
            self,
            RcElisionHint::LinearUse
                | RcElisionHint::Ephemeral
                | RcElisionHint::TailPosition
                | RcElisionHint::DeadPath
        )
    }
    /// Check if this hint allows eliding the RC decrement.
    pub fn can_elide_dec(&self) -> bool {
        matches!(
            self,
            RcElisionHint::LinearUse | RcElisionHint::Ephemeral | RcElisionHint::DeadPath
        )
    }
    /// Check if this hint allows in-place mutation.
    pub fn can_mutate_inplace(&self) -> bool {
        matches!(self, RcElisionHint::UniqueOwner | RcElisionHint::LinearUse)
    }
    /// Combine two hints (conservative: take the least optimistic).
    pub fn combine(&self, other: &RcElisionHint) -> RcElisionHint {
        if self == other {
            return self.clone();
        }
        if *self == RcElisionHint::DeadPath {
            return other.clone();
        }
        if *other == RcElisionHint::DeadPath {
            return self.clone();
        }
        RcElisionHint::None
    }
}
/// A simple mark-and-sweep GC tracer over an abstract object graph.
#[allow(dead_code)]
pub struct GcTracer {
    nodes: Vec<GcNode>,
}
#[allow(dead_code)]
impl GcTracer {
    /// Create an empty tracer.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    /// Add a node and return its ID.
    pub fn add_node(&mut self, is_root: bool) -> u32 {
        let id = self.nodes.len() as u32;
        self.nodes.push(GcNode {
            refs: Vec::new(),
            is_root,
            marked: false,
        });
        id
    }
    /// Add a reference from `from` to `to`.
    pub fn add_ref(&mut self, from: u32, to: u32) {
        if let Some(node) = self.nodes.get_mut(from as usize) {
            if !node.refs.contains(&to) {
                node.refs.push(to);
            }
        }
    }
    /// Mark all nodes reachable from roots.
    pub fn mark(&mut self) {
        let roots: Vec<u32> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.is_root)
            .map(|(i, _)| i as u32)
            .collect();
        let mut worklist = roots;
        while let Some(id) = worklist.pop() {
            if let Some(node) = self.nodes.get_mut(id as usize) {
                if node.marked {
                    continue;
                }
                node.marked = true;
                let refs = node.refs.clone();
                for next in refs {
                    worklist.push(next);
                }
            }
        }
    }
    /// Sweep: return IDs of unreachable (non-marked) nodes.
    pub fn sweep(&self) -> Vec<u32> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| !n.marked)
            .map(|(i, _)| i as u32)
            .collect()
    }
    /// Full collection: mark then sweep.
    pub fn collect(&mut self) -> Vec<u32> {
        for node in self.nodes.iter_mut() {
            node.marked = false;
        }
        self.mark();
        self.sweep()
    }
    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Number of marked (live) nodes.
    pub fn live_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.marked).count()
    }
}
/// A simple atomic reference counter (non-owning, just counts).
#[allow(dead_code)]
pub struct AtomicRefCounter {
    count: std::sync::atomic::AtomicU64,
}
#[allow(dead_code)]
impl AtomicRefCounter {
    /// Create a new counter starting at `initial`.
    pub fn new(initial: u64) -> Self {
        Self {
            count: std::sync::atomic::AtomicU64::new(initial),
        }
    }
    /// Increment and return the new value.
    pub fn inc(&self) -> u64 {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
    }
    /// Decrement and return the new value (saturating at 0).
    pub fn dec(&self) -> u64 {
        match self.count.fetch_update(
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
            |v| if v > 0 { Some(v - 1) } else { None },
        ) {
            Ok(prev) => prev - 1,
            Err(_) => 0,
        }
    }
    /// Load the current count.
    pub fn load(&self) -> u64 {
        self.count.load(std::sync::atomic::Ordering::SeqCst)
    }
    /// Reset to a new value.
    pub fn reset(&self, val: u64) {
        self.count.store(val, std::sync::atomic::Ordering::SeqCst);
    }
    /// Whether the count is zero.
    pub fn is_zero(&self) -> bool {
        self.load() == 0
    }
}
/// A table of weak references to values.
#[allow(dead_code)]
pub struct WeakTable<T: Clone> {
    entries: HashMap<u64, std::sync::Weak<T>>,
    next_key: u64,
    miss_count: u64,
}
#[allow(dead_code)]
impl<T: Clone> WeakTable<T> {
    /// Create an empty weak table.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_key: 0,
            miss_count: 0,
        }
    }
    /// Register a strong reference, returning its key.
    pub fn register(&mut self, value: std::sync::Arc<T>) -> u64 {
        let key = self.next_key;
        self.next_key += 1;
        self.entries.insert(key, std::sync::Arc::downgrade(&value));
        key
    }
    /// Try to upgrade a weak reference.
    pub fn get(&mut self, key: u64) -> Option<std::sync::Arc<T>> {
        if let Some(weak) = self.entries.get(&key) {
            if let Some(strong) = weak.upgrade() {
                return Some(strong);
            } else {
                self.entries.remove(&key);
                self.miss_count += 1;
            }
        }
        None
    }
    /// Prune all expired (dead) weak references.
    pub fn prune(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, weak| weak.upgrade().is_some());
        before - self.entries.len()
    }
    /// Number of registered entries (including dead ones).
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Number of times a lookup found a dead reference.
    pub fn miss_count(&self) -> u64 {
        self.miss_count
    }
}
/// A directed graph where nodes are reference-counted.
#[allow(dead_code)]
pub struct RcGraph {
    nodes: HashMap<u32, (RcGraphNode, u32)>,
    next_id: u32,
    edge_count: usize,
}
#[allow(dead_code)]
impl RcGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: 0,
            edge_count: 0,
        }
    }
    /// Add a node with the given data.
    pub fn add_node(&mut self, data: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(
            id,
            (
                RcGraphNode {
                    id,
                    data,
                    out_edges: Vec::new(),
                },
                1,
            ),
        );
        id
    }
    /// Add a directed edge from `src` to `dst`.
    pub fn add_edge(&mut self, src: u32, dst: u32) {
        if let Some((node, _)) = self.nodes.get_mut(&src) {
            if !node.out_edges.contains(&dst) {
                node.out_edges.push(dst);
                self.edge_count += 1;
            }
        }
        if let Some((_, rc)) = self.nodes.get_mut(&dst) {
            *rc = rc.saturating_add(1);
        }
    }
    /// Remove an edge and decrement destination refcount.
    pub fn remove_edge(&mut self, src: u32, dst: u32) -> bool {
        let removed = if let Some((node, _)) = self.nodes.get_mut(&src) {
            let before = node.out_edges.len();
            node.out_edges.retain(|&e| e != dst);
            node.out_edges.len() < before
        } else {
            false
        };
        if removed {
            self.edge_count = self.edge_count.saturating_sub(1);
            if let Some((_, rc)) = self.nodes.get_mut(&dst) {
                *rc = rc.saturating_sub(1);
            }
        }
        removed
    }
    /// Get node data.
    pub fn node_data(&self, id: u32) -> Option<u64> {
        self.nodes.get(&id).map(|(n, _)| n.data)
    }
    /// Get refcount.
    pub fn refcount(&self, id: u32) -> u32 {
        self.nodes.get(&id).map(|(_, rc)| *rc).unwrap_or(0)
    }
    /// Get out-edges.
    pub fn out_edges(&self, id: u32) -> Vec<u32> {
        self.nodes
            .get(&id)
            .map(|(n, _)| n.out_edges.clone())
            .unwrap_or_default()
    }
    /// Remove a node (and all edges from it).
    pub fn remove_node(&mut self, id: u32) {
        if let Some((node, _)) = self.nodes.remove(&id) {
            for dst in node.out_edges {
                if let Some((_, rc)) = self.nodes.get_mut(&dst) {
                    *rc = rc.saturating_sub(1);
                }
                self.edge_count = self.edge_count.saturating_sub(1);
            }
        }
    }
    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
    /// Nodes with zero refcount (potential garbage).
    pub fn zero_refcount_nodes(&self) -> Vec<u32> {
        self.nodes
            .iter()
            .filter(|(_, (_, rc))| *rc == 0)
            .map(|(id, _)| *id)
            .collect()
    }
}
/// Statistics about reference counting operations.
#[derive(Clone, Debug, Default)]
pub struct RcStats {
    /// Total number of RC increments.
    pub increments: u64,
    /// Total number of RC decrements.
    pub decrements: u64,
    /// Total number of deallocations (rc reached 0).
    pub deallocations: u64,
    /// Total number of elided increments.
    pub elided_increments: u64,
    /// Total number of elided decrements.
    pub elided_decrements: u64,
    /// Total number of in-place mutations (unique owner).
    pub inplace_mutations: u64,
    /// Total number of copy-on-write operations.
    pub copy_on_write: u64,
    /// Peak reference count observed.
    pub peak_rc: u32,
}
impl RcStats {
    /// Create new empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an RC increment.
    pub fn record_inc(&mut self) {
        self.increments += 1;
    }
    /// Record an RC decrement.
    pub fn record_dec(&mut self) {
        self.decrements += 1;
    }
    /// Record a deallocation.
    pub fn record_dealloc(&mut self) {
        self.deallocations += 1;
    }
    /// Record an elided increment.
    pub fn record_elided_inc(&mut self) {
        self.elided_increments += 1;
    }
    /// Record an elided decrement.
    pub fn record_elided_dec(&mut self) {
        self.elided_decrements += 1;
    }
    /// Record an in-place mutation.
    pub fn record_inplace_mutation(&mut self) {
        self.inplace_mutations += 1;
    }
    /// Record a copy-on-write operation.
    pub fn record_cow(&mut self) {
        self.copy_on_write += 1;
    }
    /// Update the peak RC if necessary.
    pub fn update_peak(&mut self, rc: u32) {
        if rc > self.peak_rc {
            self.peak_rc = rc;
        }
    }
    /// Total RC operations (not elided).
    pub fn total_ops(&self) -> u64 {
        self.increments + self.decrements
    }
    /// Total elided operations.
    pub fn total_elided(&self) -> u64 {
        self.elided_increments + self.elided_decrements
    }
    /// Elision ratio.
    pub fn elision_ratio(&self) -> f64 {
        let total = self.total_ops() + self.total_elided();
        if total == 0 {
            return 1.0;
        }
        self.total_elided() as f64 / total as f64
    }
    /// Merge with another stats instance.
    pub fn merge(&mut self, other: &RcStats) {
        self.increments += other.increments;
        self.decrements += other.decrements;
        self.deallocations += other.deallocations;
        self.elided_increments += other.elided_increments;
        self.elided_decrements += other.elided_decrements;
        self.inplace_mutations += other.inplace_mutations;
        self.copy_on_write += other.copy_on_write;
        if other.peak_rc > self.peak_rc {
            self.peak_rc = other.peak_rc;
        }
    }
    /// Reset all statistics.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// A typed retain/release counter attached to a value.
#[allow(dead_code)]
pub struct RetainRelease<T> {
    pub(super) value: T,
    retain_count: u64,
    release_count: u64,
}
#[allow(dead_code)]
impl<T> RetainRelease<T> {
    /// Create a new retained value.
    pub fn new(value: T) -> Self {
        Self {
            value,
            retain_count: 1,
            release_count: 0,
        }
    }
    /// Retain (increment refcount).
    pub fn retain(&mut self) {
        self.retain_count += 1;
    }
    /// Release (decrement refcount). Returns true if the object should be dropped.
    pub fn release(&mut self) -> bool {
        self.release_count += 1;
        self.retain_count <= self.release_count
    }
    /// Current live refcount.
    pub fn live_count(&self) -> u64 {
        self.retain_count.saturating_sub(self.release_count)
    }
    /// Access the inner value.
    pub fn get(&self) -> &T {
        &self.value
    }
    /// Mutably access the inner value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// Total retains.
    pub fn retain_count(&self) -> u64 {
        self.retain_count
    }
    /// Total releases.
    pub fn release_count(&self) -> u64 {
        self.release_count
    }
}
/// Manages reference counting within a scope.
///
/// The `RcManager` tracks all live references and their counts,
/// applies elision hints, and collects statistics.
pub struct RcManager {
    /// Elision analysis for the current scope.
    analysis: RcElisionAnalysis,
    /// Statistics for the current scope.
    stats: RcStats,
    /// Whether RC tracking is enabled (can be disabled for benchmarking).
    enabled: bool,
    /// Maximum reference count before triggering a warning.
    max_rc_threshold: u32,
    /// Pending decrements (batched for efficiency).
    pending_decrements: Vec<String>,
}
impl RcManager {
    /// Create a new RC manager.
    pub fn new() -> Self {
        RcManager {
            analysis: RcElisionAnalysis::new(),
            stats: RcStats::new(),
            enabled: true,
            max_rc_threshold: 1_000_000,
            pending_decrements: Vec::new(),
        }
    }
    /// Create a new RC manager with elision analysis.
    pub fn with_analysis(analysis: RcElisionAnalysis) -> Self {
        RcManager {
            analysis,
            stats: RcStats::new(),
            enabled: true,
            max_rc_threshold: 1_000_000,
            pending_decrements: Vec::new(),
        }
    }
    /// Record an RC increment for a variable.
    pub fn inc(&mut self, var: &str) {
        if !self.enabled {
            return;
        }
        let hint = self.analysis.get_hint(var);
        if hint.can_elide_inc() {
            self.stats.record_elided_inc();
        } else {
            self.stats.record_inc();
        }
    }
    /// Record an RC decrement for a variable.
    pub fn dec(&mut self, var: &str) {
        if !self.enabled {
            return;
        }
        let hint = self.analysis.get_hint(var);
        if hint.can_elide_dec() {
            self.stats.record_elided_dec();
        } else {
            self.stats.record_dec();
        }
    }
    /// Schedule a decrement for later (batch processing).
    pub fn schedule_dec(&mut self, var: String) {
        self.pending_decrements.push(var);
    }
    /// Process all pending decrements.
    pub fn flush_pending(&mut self) {
        let pending = std::mem::take(&mut self.pending_decrements);
        for var in &pending {
            self.dec(var);
        }
    }
    /// Check if a variable can be mutated in-place.
    pub fn can_mutate_inplace(&self, var: &str) -> bool {
        self.analysis.get_hint(var).can_mutate_inplace()
    }
    /// Record an in-place mutation.
    pub fn record_inplace_mutation(&mut self) {
        self.stats.record_inplace_mutation();
    }
    /// Record a copy-on-write operation.
    pub fn record_cow(&mut self) {
        self.stats.record_cow();
    }
    /// Get the current statistics.
    pub fn stats(&self) -> &RcStats {
        &self.stats
    }
    /// Get the elision analysis.
    pub fn analysis(&self) -> &RcElisionAnalysis {
        &self.analysis
    }
    /// Enable or disable RC tracking.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Set the maximum RC threshold.
    pub fn set_max_rc_threshold(&mut self, threshold: u32) {
        self.max_rc_threshold = threshold;
    }
    /// Get the max RC threshold.
    pub fn max_rc_threshold(&self) -> u32 {
        self.max_rc_threshold
    }
    /// Reset statistics.
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }
}
/// A weak reference that does not prevent deallocation.
///
/// Weak references can be upgraded to strong references if the value
/// is still alive.
pub struct Weak<T> {
    /// The value (kept alive by the strong count on the original Rc).
    _marker: std::marker::PhantomData<T>,
    /// Whether the value is still alive.
    pub(super) alive: Cell<bool>,
    /// A copy of the value for upgrade attempts.
    value: Option<T>,
}
impl<T: Clone> Weak<T> {
    /// Create a new weak reference from a strong reference.
    pub fn from_rc(rc: &Rc<T>) -> Self {
        rc.inc_weak();
        Weak {
            _marker: std::marker::PhantomData,
            alive: Cell::new(true),
            value: Some(rc.inner.value.clone()),
        }
    }
    /// Try to upgrade this weak reference to a strong reference.
    pub fn upgrade(&self) -> Option<Rc<T>> {
        if self.alive.get() {
            self.value.as_ref().map(|v| Rc::new(v.clone()))
        } else {
            None
        }
    }
    /// Check if the referenced value is still alive.
    pub fn is_alive(&self) -> bool {
        self.alive.get()
    }
    /// Mark this weak reference as dead (the strong references are all gone).
    pub fn invalidate(&self) {
        self.alive.set(false);
    }
}
