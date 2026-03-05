//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::marker::PhantomData;

use std::collections::{HashMap, VecDeque};

/// A chained-block arena that grows by adding new blocks on overflow.
#[allow(dead_code)]
pub struct ChainedArena {
    blocks: Vec<ArenaBlock>,
    block_size: usize,
    total_alloc: usize,
}
#[allow(dead_code)]
impl ChainedArena {
    /// Creates a chained arena with the given initial block size.
    pub fn new(block_size: usize) -> Self {
        let first = ArenaBlock::new(block_size);
        Self {
            blocks: vec![first],
            block_size,
            total_alloc: 0,
        }
    }
    /// Allocates `bytes` bytes with `align` alignment.
    pub fn alloc(&mut self, bytes: usize, align: usize) -> usize {
        let last = self.blocks.len() - 1;
        if let Some(offset) = self.blocks[last].try_alloc(bytes, align) {
            self.total_alloc += bytes;
            return offset + last * self.block_size;
        }
        let new_block_size = self.block_size.max(bytes + align);
        let mut block = ArenaBlock::new(new_block_size);
        let offset = block
            .try_alloc(bytes, align)
            .expect("arena block allocation must succeed");
        let block_idx = self.blocks.len();
        self.blocks.push(block);
        self.total_alloc += bytes;
        offset + block_idx * self.block_size
    }
    /// Returns the total number of blocks.
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }
    /// Returns total bytes allocated (excluding padding).
    pub fn total_allocated(&self) -> usize {
        self.total_alloc
    }
}
/// A dense map from arena indices to values.
///
/// Backed by a `Vec<Option<V>>` aligned with an arena, allowing O(1) lookup
/// by index.
#[derive(Debug, Clone)]
pub struct ArenaMap<T, V> {
    data: Vec<Option<V>>,
    _marker: PhantomData<T>,
}
impl<T, V> ArenaMap<T, V> {
    /// Create an empty `ArenaMap`.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }
    /// Insert a value for `idx`, growing the map if necessary.
    pub fn insert(&mut self, idx: Idx<T>, value: V) {
        let i = idx.to_usize();
        if i >= self.data.len() {
            self.data.resize_with(i + 1, || None);
        }
        self.data[i] = Some(value);
    }
    /// Get a reference to the value for `idx`.
    pub fn get(&self, idx: Idx<T>) -> Option<&V> {
        self.data.get(idx.to_usize())?.as_ref()
    }
    /// Get a mutable reference to the value for `idx`.
    pub fn get_mut(&mut self, idx: Idx<T>) -> Option<&mut V> {
        self.data.get_mut(idx.to_usize())?.as_mut()
    }
    /// Remove the value at `idx`.
    pub fn remove(&mut self, idx: Idx<T>) -> Option<V> {
        self.data.get_mut(idx.to_usize())?.take()
    }
    /// Check whether `idx` has a value.
    pub fn contains(&self, idx: Idx<T>) -> bool {
        self.get(idx).is_some()
    }
    /// Number of non-empty entries.
    pub fn count(&self) -> usize {
        self.data.iter().filter(|e| e.is_some()).count()
    }
    /// Iterate over all (index, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (Idx<T>, &V)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|val| (Idx::new(i as u32), val)))
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
/// A two-generation arena (nursery + stable) for generational allocation.
#[allow(dead_code)]
pub struct TwoGenerationArena {
    pub(crate) nursery: GrowableArena,
    pub(crate) stable: GrowableArena,
    promotions: usize,
}
#[allow(dead_code)]
impl TwoGenerationArena {
    /// Creates a new two-generation arena.
    pub fn new(nursery_cap: usize, stable_cap: usize) -> Self {
        Self {
            nursery: GrowableArena::new(nursery_cap),
            stable: GrowableArena::new(stable_cap),
            promotions: 0,
        }
    }
    /// Allocates in the nursery generation.
    pub fn alloc_nursery(&mut self, size: usize, align: usize) -> usize {
        self.nursery.alloc(size, align)
    }
    /// Promotes an allocation from nursery to stable generation.
    pub fn promote(&mut self, size: usize, align: usize) -> usize {
        self.promotions += 1;
        self.stable.alloc(size, align)
    }
    /// Clears the nursery (minor GC).
    pub fn minor_gc(&mut self) {
        self.nursery.reset();
    }
    /// Returns the number of promotions performed.
    pub fn num_promotions(&self) -> usize {
        self.promotions
    }
}
/// A savepoint that can be used to roll back an arena to a prior state.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ArenaCheckpoint {
    /// The watermark position at checkpoint time.
    pub watermark: usize,
}
#[allow(dead_code)]
impl ArenaCheckpoint {
    /// Creates a checkpoint at the current position in `arena`.
    pub fn create(arena: &LinearArena) -> Self {
        Self {
            watermark: arena.used(),
        }
    }
    /// Rolls back `arena` to this checkpoint.
    pub fn restore(self, arena: &mut LinearArena) {
        arena.top = self.watermark;
    }
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
/// An arena that supports checkpointing and rollback.
///
/// By saving a checkpoint (the current length), the arena can be rolled back
/// to discard all allocations made after that point.
#[derive(Debug)]
pub struct ScopedArena<T> {
    inner: Arena<T>,
}
impl<T> ScopedArena<T> {
    /// Create a new empty scoped arena.
    pub fn new() -> Self {
        Self {
            inner: Arena::new(),
        }
    }
    /// Allocate a value.
    pub fn alloc(&mut self, value: T) -> Idx<T> {
        self.inner.alloc(value)
    }
    /// Get a reference to the value at `idx`.
    pub fn get(&self, idx: Idx<T>) -> &T {
        self.inner.get(idx)
    }
    /// Create a checkpoint (the current length).
    pub fn checkpoint(&self) -> u32 {
        self.inner.len() as u32
    }
    /// Rollback to a previously saved checkpoint.
    ///
    /// All allocations after `checkpoint` are dropped.
    pub fn rollback(&mut self, checkpoint: u32) {
        self.inner.data.truncate(checkpoint as usize);
    }
    /// Current number of allocated values.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
/// A simple linear (bump) arena allocator over a fixed backing buffer.
#[allow(dead_code)]
pub struct LinearArena {
    pub(crate) buf: Vec<u8>,
    top: usize,
    stats: ArenaStatsExt,
}
#[allow(dead_code)]
impl LinearArena {
    /// Creates a new arena with `capacity` bytes.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0u8; capacity],
            top: 0,
            stats: ArenaStatsExt::new(),
        }
    }
    /// Allocates `size` bytes with `align` alignment.
    /// Returns the offset into the backing buffer, or `None` if full.
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        let aligned = (self.top + align - 1) & !(align - 1);
        let end = aligned.checked_add(size)?;
        if end > self.buf.len() {
            return None;
        }
        let waste = aligned - self.top;
        self.top = end;
        self.stats.bytes_allocated += size;
        self.stats.alloc_count += 1;
        self.stats.wasted_bytes += waste;
        Some(aligned)
    }
    /// Resets the arena without releasing the backing buffer.
    pub fn reset(&mut self) {
        self.top = 0;
        self.stats = ArenaStatsExt::new();
    }
    /// Returns the number of used bytes.
    pub fn used(&self) -> usize {
        self.top
    }
    /// Returns the total capacity.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }
    /// Returns a reference to the collected statistics.
    pub fn stats(&self) -> &ArenaStatsExt {
        &self.stats
    }
}
/// Statistics about an arena's memory usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArenaStats {
    /// Number of allocated values.
    pub len: usize,
    /// Current capacity (number of values that fit without reallocation).
    pub capacity: usize,
}
impl ArenaStats {
    /// Fraction of capacity used (0.0 to 1.0).
    pub fn utilisation(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.len as f64 / self.capacity as f64
        }
    }
    /// Number of free slots remaining before reallocation.
    pub fn free(&self) -> usize {
        self.capacity.saturating_sub(self.len)
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
/// A simple bump allocator for byte slices.
///
/// Memory is allocated sequentially; deallocation of individual items is not
/// supported. The entire arena can be reset at once.
#[derive(Debug)]
pub struct BumpArena {
    buf: Vec<u8>,
    pos: usize,
}
impl BumpArena {
    /// Create a bump arena with the given capacity in bytes.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: vec![0u8; cap],
            pos: 0,
        }
    }
    /// Allocate `size` bytes, returning the start offset.
    ///
    /// Returns `None` if there is not enough space.
    pub fn alloc_bytes(&mut self, size: usize) -> Option<usize> {
        if self.pos + size > self.buf.len() {
            None
        } else {
            let offset = self.pos;
            self.pos += size;
            Some(offset)
        }
    }
    /// Write bytes at the given offset.
    pub fn write_at(&mut self, offset: usize, data: &[u8]) {
        self.buf[offset..offset + data.len()].copy_from_slice(data);
    }
    /// Read bytes from the given offset.
    pub fn read_at(&self, offset: usize, len: usize) -> &[u8] {
        &self.buf[offset..offset + len]
    }
    /// Reset the bump pointer without zeroing memory.
    pub fn reset(&mut self) {
        self.pos = 0;
    }
    /// Bytes used.
    pub fn used(&self) -> usize {
        self.pos
    }
    /// Total capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }
    /// Remaining bytes.
    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }
}
/// A pool of recycled arenas.
///
/// Allows arena instances to be reused across phases, reducing allocations.
#[derive(Debug)]
pub struct ArenaPool<T> {
    pub(super) pool: Vec<Arena<T>>,
}
impl<T> ArenaPool<T> {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self { pool: Vec::new() }
    }
    /// Retrieve an arena from the pool (creating one if empty).
    pub fn acquire(&mut self) -> Arena<T> {
        self.pool.pop().unwrap_or_default()
    }
    /// Return an arena to the pool after clearing it.
    pub fn release(&mut self, mut arena: Arena<T>) {
        arena.clear();
        self.pool.push(arena);
    }
    /// Number of arenas currently in the pool.
    pub fn pool_size(&self) -> usize {
        self.pool.len()
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
/// A growable arena that reallocates its backing buffer when full.
#[allow(dead_code)]
pub struct GrowableArena {
    pub(crate) data: Vec<u8>,
    top: usize,
    count: usize,
}
#[allow(dead_code)]
impl GrowableArena {
    /// Creates a new growable arena with `initial` bytes.
    pub fn new(initial: usize) -> Self {
        Self {
            data: vec![0u8; initial.max(16)],
            top: 0,
            count: 0,
        }
    }
    /// Allocates `size` bytes, growing the backing buffer if needed.
    pub fn alloc(&mut self, size: usize, align: usize) -> usize {
        let aligned = (self.top + align - 1) & !(align - 1);
        let end = aligned + size;
        if end > self.data.len() {
            let new_cap = (self.data.len() * 2).max(end);
            self.data.resize(new_cap, 0);
        }
        self.top = end;
        self.count += 1;
        aligned
    }
    /// Returns total bytes used.
    pub fn used(&self) -> usize {
        self.top
    }
    /// Returns total allocation count.
    pub fn count(&self) -> usize {
        self.count
    }
    /// Resets the arena.
    pub fn reset(&mut self) {
        self.top = 0;
        self.count = 0;
    }
}
/// A type-parameterised arena allocator that stores `T` values.
#[allow(dead_code)]
pub struct TypedArena<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> TypedArena<T> {
    /// Creates a new typed arena.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Allocates `val` and returns a reference with the arena's lifetime.
    pub fn alloc(&mut self, val: T) -> &T {
        self.items.push(val);
        self.items.last().expect("items list must be non-empty")
    }
    /// Returns the number of allocated items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if no items have been allocated.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Clears all items.
    pub fn clear(&mut self) {
        self.items.clear();
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
/// A contiguous range of typed indices `[start, end)`.
///
/// Used to represent a batch of values allocated together in an arena.
#[derive(Clone, Copy)]
pub struct IdxRange<T> {
    /// Inclusive start index.
    pub start: Idx<T>,
    /// Exclusive end index.
    pub end: Idx<T>,
}
impl<T> IdxRange<T> {
    /// Create a new range.
    pub fn new(start: Idx<T>, end: Idx<T>) -> Self {
        Self { start, end }
    }
    /// Create an empty range starting at `idx`.
    pub fn empty_at(idx: Idx<T>) -> Self {
        let r = idx.raw;
        Self {
            start: Idx::new(r),
            end: Idx::new(r),
        }
    }
    /// Length of the range.
    pub fn len(self) -> usize {
        (self.end.raw as usize).saturating_sub(self.start.raw as usize)
    }
    /// Returns `true` if the range is empty.
    pub fn is_empty(self) -> bool {
        self.start.raw >= self.end.raw
    }
    /// Check whether `idx` is in this range.
    pub fn contains(self, idx: Idx<T>) -> bool {
        idx.raw >= self.start.raw && idx.raw < self.end.raw
    }
    /// Iterate over all indices in this range.
    pub fn iter(self) -> impl Iterator<Item = Idx<T>> {
        (self.start.raw..self.end.raw).map(Idx::new)
    }
}
/// A fixed-size block in a chained-block arena.
#[allow(dead_code)]
pub struct ArenaBlock {
    data: Vec<u8>,
    used: usize,
}
#[allow(dead_code)]
impl ArenaBlock {
    /// Creates a block with `size` bytes capacity.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            used: 0,
        }
    }
    /// Tries to allocate `bytes` with alignment `align`.
    pub fn try_alloc(&mut self, bytes: usize, align: usize) -> Option<usize> {
        let base = (self.used + align - 1) & !(align - 1);
        let end = base.checked_add(bytes)?;
        if end > self.data.len() {
            return None;
        }
        self.used = end;
        Some(base)
    }
    /// Returns remaining capacity.
    pub fn remaining(&self) -> usize {
        self.data.len() - self.used
    }
    /// Returns the size of the block.
    pub fn block_size(&self) -> usize {
        self.data.len()
    }
}
/// An arena that interns values, returning the same index for equal values.
///
/// If a value was previously allocated, its existing index is returned
/// instead of allocating a new copy.
#[derive(Debug)]
pub struct InterningArena<T: PartialEq + Clone> {
    inner: Arena<T>,
}
impl<T: PartialEq + Clone> InterningArena<T> {
    /// Create a new empty interning arena.
    pub fn new() -> Self {
        Self {
            inner: Arena::new(),
        }
    }
    /// Allocate or retrieve an existing value.
    ///
    /// If `value` is already present, returns its existing index.
    /// Otherwise allocates a new copy.
    pub fn intern(&mut self, value: T) -> Idx<T> {
        for (i, v) in self.inner.data.iter().enumerate() {
            if *v == value {
                return Idx::new(i as u32);
            }
        }
        self.inner.alloc(value)
    }
    /// Get a reference to the interned value at `idx`.
    pub fn get(&self, idx: Idx<T>) -> &T {
        self.inner.get(idx)
    }
    /// Number of distinct values stored.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if the arena is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Check whether `value` is already interned.
    pub fn contains(&self, value: &T) -> bool {
        self.inner.data.iter().any(|v| v == value)
    }
    /// Collect statistics.
    pub fn stats(&self) -> ArenaStats {
        self.inner.stats()
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
/// A typed arena for storing values contiguously.
///
/// Values are never moved once allocated, so references remain valid
/// for the lifetime of the arena.
#[derive(Debug)]
pub struct Arena<T> {
    data: Vec<T>,
}
impl<T> Arena<T> {
    /// Create a new empty arena.
    #[inline]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Create a new arena with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    /// Allocate a value in the arena and return its index.
    #[inline]
    pub fn alloc(&mut self, value: T) -> Idx<T> {
        let idx = self.data.len();
        assert!(idx < u32::MAX as usize, "arena overflow");
        self.data.push(value);
        Idx::new(idx as u32)
    }
    /// Allocate multiple values at once, returning the index range.
    pub fn alloc_many(&mut self, values: impl IntoIterator<Item = T>) -> IdxRange<T> {
        let start = Idx::new(self.data.len() as u32);
        self.data.extend(values);
        let end = Idx::new(self.data.len() as u32);
        IdxRange::new(start, end)
    }
    /// Get a reference to the value at the given index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds (in debug mode).
    #[inline]
    pub fn get(&self, idx: Idx<T>) -> &T {
        &self.data[idx.raw as usize]
    }
    /// Get a mutable reference to the value at the given index.
    #[inline]
    pub fn get_mut(&mut self, idx: Idx<T>) -> &mut T {
        &mut self.data[idx.raw as usize]
    }
    /// Get a slice of values for the given range.
    pub fn get_range(&self, range: IdxRange<T>) -> &[T] {
        &self.data[range.start.to_usize()..range.end.to_usize()]
    }
    /// Get the number of values in the arena.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Check if the arena is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Get the index that would be assigned to the next allocation.
    pub fn next_idx(&self) -> Idx<T> {
        Idx::new(self.data.len() as u32)
    }
    /// Iterate over all (index, value) pairs.
    pub fn iter_indexed(&self) -> impl Iterator<Item = (Idx<T>, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, v)| (Idx::new(i as u32), v))
    }
    /// Iterate over all values.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    /// Collect the current size statistics.
    pub fn stats(&self) -> ArenaStats {
        ArenaStats {
            len: self.data.len(),
            capacity: self.data.capacity(),
        }
    }
    /// Shrink the arena's backing storage to fit.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    /// Reset the arena, dropping all values.
    pub fn clear(&mut self) {
        self.data.clear();
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
/// A string slice stored in an arena (UTF-8, null-terminated).
#[allow(dead_code)]
pub struct ArenaString {
    pub(crate) offset: usize,
    len: usize,
}
#[allow(dead_code)]
impl ArenaString {
    /// Stores `s` in `arena` and returns a handle.
    pub fn store(arena: &mut LinearArena, s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        let offset = arena.alloc(bytes.len() + 1, 1)?;
        let start = offset;
        for (i, &b) in bytes.iter().enumerate() {
            arena.buf[start + i] = b;
        }
        arena.buf[start + bytes.len()] = 0;
        Some(Self {
            offset,
            len: bytes.len(),
        })
    }
    /// Returns the length of the string in bytes.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
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
/// A registry of memory regions.
#[allow(dead_code)]
pub struct MemoryRegionRegistry {
    regions: Vec<MemoryRegion>,
}
#[allow(dead_code)]
impl MemoryRegionRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }
    /// Adds a region.
    pub fn add(&mut self, region: MemoryRegion) {
        self.regions.push(region);
    }
    /// Returns the region containing `addr`, or `None`.
    pub fn find(&self, addr: usize) -> Option<&MemoryRegion> {
        self.regions.iter().find(|r| r.contains(addr))
    }
    /// Returns the total number of regions.
    pub fn len(&self) -> usize {
        self.regions.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}
/// Two arenas paired together, useful for storing interrelated values.
///
/// Allocations in each arena are independent, but the two are kept together
/// for lifetime management.
#[derive(Debug, Default)]
pub struct DoubleArena<A, B> {
    /// The first arena.
    pub first: Arena<A>,
    /// The second arena.
    pub second: Arena<B>,
}
impl<A, B> DoubleArena<A, B> {
    /// Create an empty `DoubleArena`.
    pub fn new() -> Self {
        Self {
            first: Arena::new(),
            second: Arena::new(),
        }
    }
    /// Allocate a pair of related values atomically.
    ///
    /// Returns `(idx_a, idx_b)` where `idx_a.raw() == idx_b.raw()` is NOT
    /// guaranteed — the arenas may have different lengths.
    pub fn alloc_pair(&mut self, a: A, b: B) -> (Idx<A>, Idx<B>) {
        let ia = self.first.alloc(a);
        let ib = self.second.alloc(b);
        (ia, ib)
    }
    /// Total allocations across both arenas.
    pub fn total_len(&self) -> usize {
        self.first.len() + self.second.len()
    }
}
/// Represents a named memory region used by the arena system.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MemoryRegion {
    /// Human-readable label.
    pub label: String,
    /// Base offset.
    pub base: usize,
    /// Size in bytes.
    pub size: usize,
    /// Whether the region is currently in use.
    pub active: bool,
}
#[allow(dead_code)]
impl MemoryRegion {
    /// Creates a new memory region.
    pub fn new(label: impl Into<String>, base: usize, size: usize) -> Self {
        Self {
            label: label.into(),
            base,
            size,
            active: false,
        }
    }
    /// Activates the region.
    pub fn activate(&mut self) {
        self.active = true;
    }
    /// Deactivates the region.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    /// Returns the exclusive end of the region.
    pub fn end(&self) -> usize {
        self.base + self.size
    }
    /// Returns `true` if `addr` is within the region.
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.base && addr < self.end()
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
/// A scoped arena that tracks a watermark for bulk deallocation.
#[allow(dead_code)]
pub struct ScopedArenaExt {
    pub(crate) inner: LinearArena,
    watermarks: Vec<usize>,
}
#[allow(dead_code)]
impl ScopedArenaExt {
    /// Creates a scoped arena with `capacity` bytes.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: LinearArena::new(capacity),
            watermarks: Vec::new(),
        }
    }
    /// Pushes a scope: records the current allocation watermark.
    pub fn push_scope(&mut self) {
        self.watermarks.push(self.inner.used());
    }
    /// Pops a scope: resets allocation to the saved watermark.
    /// Panics if no scope is active.
    pub fn pop_scope(&mut self) {
        let wm = self.watermarks.pop().expect("ScopedArena: no active scope");
        self.inner.top = wm;
    }
    /// Allocates `size` bytes.
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        self.inner.alloc(size, align)
    }
    /// Returns the current depth of nested scopes.
    pub fn scope_depth(&self) -> usize {
        self.watermarks.len()
    }
}
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
/// A growable array stored entirely within a `LinearArena`.
///
/// Does NOT support drop (elements must be `Copy`).
#[allow(dead_code)]
pub struct ArenaVec<T: Copy> {
    base: usize,
    length: usize,
    _t: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T: Copy> ArenaVec<T> {
    /// Creates a new `ArenaVec` with `capacity` elements in `arena`.
    pub fn new(arena: &mut LinearArena, capacity: usize) -> Option<Self> {
        let base = arena.alloc(
            capacity * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        )?;
        Some(Self {
            base,
            length: 0,
            _t: std::marker::PhantomData,
        })
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.length
    }
    /// Returns `true` if the vec is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
/// Statistics collected from an arena allocator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ArenaStatsExt {
    /// Total bytes allocated.
    pub bytes_allocated: usize,
    /// Total number of allocations performed.
    pub alloc_count: usize,
    /// Number of chunks allocated.
    pub chunk_count: usize,
    /// Wasted bytes due to alignment padding.
    pub wasted_bytes: usize,
}
#[allow(dead_code)]
impl ArenaStatsExt {
    /// Creates a zeroed stats record.
    pub fn new() -> Self {
        Self {
            bytes_allocated: 0,
            alloc_count: 0,
            chunk_count: 0,
            wasted_bytes: 0,
        }
    }
    /// Returns the average allocation size in bytes.
    pub fn avg_alloc_size(&self) -> f64 {
        if self.alloc_count == 0 {
            return 0.0;
        }
        self.bytes_allocated as f64 / self.alloc_count as f64
    }
    /// Returns the fragmentation ratio (wasted / total).
    pub fn fragmentation(&self) -> f64 {
        let total = self.bytes_allocated + self.wasted_bytes;
        if total == 0 {
            return 0.0;
        }
        self.wasted_bytes as f64 / total as f64
    }
}
/// A fixed-size object pool backed by a free-list.
#[allow(dead_code)]
pub struct PoolArena {
    slot_size: usize,
    capacity: usize,
    free_list: Vec<usize>,
    data: Vec<u8>,
}
#[allow(dead_code)]
impl PoolArena {
    /// Creates a pool that holds `capacity` objects of `slot_size` bytes each.
    pub fn new(slot_size: usize, capacity: usize) -> Self {
        let data = vec![0u8; slot_size * capacity];
        let free_list: Vec<usize> = (0..capacity).collect();
        Self {
            slot_size,
            capacity,
            free_list,
            data,
        }
    }
    /// Allocates one slot.  Returns the slot index or `None` if exhausted.
    pub fn alloc_slot(&mut self) -> Option<usize> {
        self.free_list.pop()
    }
    /// Frees a slot by returning it to the free list.
    pub fn free_slot(&mut self, idx: usize) {
        if idx < self.capacity {
            self.free_list.push(idx);
        }
    }
    /// Returns the number of available (free) slots.
    pub fn available(&self) -> usize {
        self.free_list.len()
    }
    /// Returns the total pool capacity in slots.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Returns the slot size in bytes.
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
}
/// An arena with O(1) deallocation via a free list.
///
/// Freed slots are tracked and reused on the next allocation.
#[derive(Debug)]
pub struct SlabArena<T> {
    data: Vec<Option<T>>,
    free_list: Vec<u32>,
}
impl<T> SlabArena<T> {
    /// Create a new empty slab arena.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            free_list: Vec::new(),
        }
    }
    /// Create a slab arena with the given initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            free_list: Vec::new(),
        }
    }
    /// Allocate a value, reusing a freed slot if available.
    pub fn alloc(&mut self, value: T) -> Idx<T> {
        if let Some(idx) = self.free_list.pop() {
            self.data[idx as usize] = Some(value);
            Idx::new(idx)
        } else {
            let idx = self.data.len() as u32;
            assert!(idx < u32::MAX, "slab arena overflow");
            self.data.push(Some(value));
            Idx::new(idx)
        }
    }
    /// Free the value at `idx`, returning it.
    ///
    /// Returns `None` if the slot was already free.
    pub fn free(&mut self, idx: Idx<T>) -> Option<T> {
        let slot = self.data.get_mut(idx.raw as usize)?;
        let value = slot.take()?;
        self.free_list.push(idx.raw);
        Some(value)
    }
    /// Get a reference to the value at `idx`.
    ///
    /// Returns `None` if the slot is free.
    pub fn get(&self, idx: Idx<T>) -> Option<&T> {
        self.data.get(idx.raw as usize)?.as_ref()
    }
    /// Get a mutable reference to the value at `idx`.
    pub fn get_mut(&mut self, idx: Idx<T>) -> Option<&mut T> {
        self.data.get_mut(idx.raw as usize)?.as_mut()
    }
    /// Number of live (non-free) slots.
    pub fn len(&self) -> usize {
        self.data.iter().filter(|s| s.is_some()).count()
    }
    /// Total number of slots (including free ones).
    pub fn capacity_slots(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if there are no live values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Number of slots on the free list.
    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }
    /// Iterate over all live (non-free) (index, value) pairs.
    pub fn iter_live(&self) -> impl Iterator<Item = (Idx<T>, &T)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|v| (Idx::new(i as u32), v)))
    }
}
/// A typed index into an arena.
///
/// This is a zero-cost abstraction over `u32` that provides type safety.
/// Equality checks are O(1) pointer comparisons.
#[derive(Clone, Copy)]
pub struct Idx<T> {
    pub(super) raw: u32,
    _marker: PhantomData<T>,
}
impl<T> Idx<T> {
    /// Create a new index from a raw u32 value.
    ///
    /// # Safety
    /// The caller must ensure the index is valid for the target arena.
    #[inline]
    pub(crate) fn new(raw: u32) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }
    /// Get the raw index value.
    #[inline]
    pub fn raw(&self) -> u32 {
        self.raw
    }
    /// Cast to a different type without changing the raw index.
    ///
    /// # Safety
    /// The caller must ensure the reinterpreted index is valid.
    pub fn cast<U>(self) -> Idx<U> {
        Idx::new(self.raw)
    }
    /// Check whether this index is the first (index 0).
    pub fn is_first(self) -> bool {
        self.raw == 0
    }
    /// Increment the index by 1 (without bounds checking).
    pub fn next(self) -> Self {
        Self::new(self.raw + 1)
    }
    /// Convert to `usize` for slice indexing.
    pub fn to_usize(self) -> usize {
        self.raw as usize
    }
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
