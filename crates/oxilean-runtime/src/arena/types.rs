//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::Cell;
use std::collections::HashMap;

use super::functions::{
    ARENA_ALIGN, DEFAULT_CHUNK_SIZE, MAX_CHUNK_SIZE, MIN_CHUNK_SIZE, PAGE_SIZE,
};

/// A location within a bump arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArenaOffset {
    /// Index of the chunk.
    pub chunk: usize,
    /// Byte offset within the chunk.
    pub offset: usize,
}
impl ArenaOffset {
    /// Create a new offset.
    pub fn new(chunk: usize, offset: usize) -> Self {
        ArenaOffset { chunk, offset }
    }
}
/// A linear (bump-pointer) allocator backed by a fixed-size buffer.
#[allow(dead_code)]
pub struct LinearAllocator {
    buf: Vec<u8>,
    top: usize,
    alloc_count: u64,
    overflow_count: u64,
}
#[allow(dead_code)]
impl LinearAllocator {
    /// Create a linear allocator with the given buffer size.
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0u8; size.max(16)],
            top: 0,
            alloc_count: 0,
            overflow_count: 0,
        }
    }
    /// Allocate `size` bytes (aligned to `align`). Returns an offset into the buffer.
    pub fn alloc_offset(&mut self, size: usize, align: usize) -> Option<usize> {
        let align = align.next_power_of_two().max(1);
        let aligned = (self.top + align - 1) & !(align - 1);
        if aligned + size > self.buf.len() {
            self.overflow_count += 1;
            return None;
        }
        self.top = aligned + size;
        self.alloc_count += 1;
        Some(aligned)
    }
    /// Get a reference to bytes at the given offset.
    pub fn get_bytes(&self, offset: usize, size: usize) -> Option<&[u8]> {
        self.buf.get(offset..offset + size)
    }
    /// Get a mutable reference to bytes at the given offset.
    pub fn get_bytes_mut(&mut self, offset: usize, size: usize) -> Option<&mut [u8]> {
        self.buf.get_mut(offset..offset + size)
    }
    /// Reset the allocator (reuse the buffer).
    pub fn reset(&mut self) {
        self.top = 0;
    }
    /// Current top-of-stack offset.
    pub fn top(&self) -> usize {
        self.top
    }
    /// Total buffer capacity.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }
    /// Free bytes remaining.
    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.top)
    }
    /// Utilization fraction.
    pub fn utilization(&self) -> f64 {
        if self.buf.is_empty() {
            0.0
        } else {
            self.top as f64 / self.buf.len() as f64
        }
    }
    /// Number of successful allocations.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
    /// Number of failed (overflow) allocations.
    pub fn overflow_count(&self) -> u64 {
        self.overflow_count
    }
}
/// Results from an arena benchmark run.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArenaBenchResult {
    pub iterations: u64,
    pub total_bytes: u64,
    pub allocs_per_iter: usize,
    pub description: String,
}
#[allow(dead_code)]
impl ArenaBenchResult {
    pub fn new(iterations: u64, total_bytes: u64, allocs_per_iter: usize, desc: &str) -> Self {
        Self {
            iterations,
            total_bytes,
            allocs_per_iter,
            description: desc.to_string(),
        }
    }
    pub fn bytes_per_iter(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            self.total_bytes as f64 / self.iterations as f64
        }
    }
}
/// Statistics for a region.
#[derive(Clone, Debug, Default)]
pub struct RegionStats {
    /// Number of allocations in this region.
    pub allocations: u64,
    /// Bytes allocated in this region.
    pub bytes_allocated: u64,
    /// Number of times this region was reset.
    pub resets: u64,
}
/// An arena that tracks generations for safe index validation.
///
/// When an element is removed, the slot's generation is incremented.
/// Indices store the generation they were created with, so stale
/// references can be detected.
pub struct GenerationalArena<T> {
    /// Values with generation counters.
    pub(super) entries: Vec<GenerationalEntry<T>>,
    /// Free list.
    free_list: Vec<usize>,
    /// Current generation for new allocations.
    pub(super) generation: u32,
}
impl<T> GenerationalArena<T> {
    /// Create a new generational arena.
    pub fn new() -> Self {
        GenerationalArena {
            entries: Vec::new(),
            free_list: Vec::new(),
            generation: 0,
        }
    }
    /// Create with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        GenerationalArena {
            entries: Vec::with_capacity(cap),
            free_list: Vec::new(),
            generation: 0,
        }
    }
    /// Insert a value and get its index.
    pub fn insert(&mut self, value: T) -> GenIdx {
        self.generation = self.generation.wrapping_add(1);
        if let Some(slot) = self.free_list.pop() {
            self.entries[slot] = GenerationalEntry {
                value: Some(value),
                generation: self.generation,
            };
            GenIdx {
                index: slot as u32,
                generation: self.generation,
            }
        } else {
            let index = self.entries.len() as u32;
            self.entries.push(GenerationalEntry {
                value: Some(value),
                generation: self.generation,
            });
            GenIdx {
                index,
                generation: self.generation,
            }
        }
    }
    /// Get a reference to a value by generational index.
    pub fn get(&self, idx: GenIdx) -> Option<&T> {
        let entry = self.entries.get(idx.index as usize)?;
        if entry.generation == idx.generation {
            entry.value.as_ref()
        } else {
            None
        }
    }
    /// Get a mutable reference to a value by generational index.
    pub fn get_mut(&mut self, idx: GenIdx) -> Option<&mut T> {
        let entry = self.entries.get_mut(idx.index as usize)?;
        if entry.generation == idx.generation {
            entry.value.as_mut()
        } else {
            None
        }
    }
    /// Remove a value by generational index.
    pub fn remove(&mut self, idx: GenIdx) -> Option<T> {
        let entry = self.entries.get_mut(idx.index as usize)?;
        if entry.generation == idx.generation {
            let value = entry.value.take();
            self.free_list.push(idx.index as usize);
            value
        } else {
            None
        }
    }
    /// Check if an index is valid.
    pub fn contains(&self, idx: GenIdx) -> bool {
        self.entries
            .get(idx.index as usize)
            .map(|e| e.generation == idx.generation && e.value.is_some())
            .unwrap_or(false)
    }
    /// Number of live entries.
    pub fn len(&self) -> usize {
        self.entries.len() - self.free_list.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.free_list.clear();
    }
    /// Iterate over live entries.
    pub fn iter(&self) -> impl Iterator<Item = (GenIdx, &T)> {
        self.entries.iter().enumerate().filter_map(|(i, e)| {
            e.value.as_ref().map(|v| {
                (
                    GenIdx {
                        index: i as u32,
                        generation: e.generation,
                    },
                    v,
                )
            })
        })
    }
}
/// An index into a generational arena that includes a generation counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GenIdx {
    /// The slot index.
    pub index: u32,
    /// The generation when this index was created.
    pub generation: u32,
}
/// A pool of reusable bump arenas.
///
/// When a temporary arena is needed (e.g., for evaluating a single expression),
/// it can be acquired from the pool and returned after use rather than creating
/// a new one each time.
pub struct ArenaPool {
    /// Available (idle) arenas.
    pub(super) available: Vec<BumpArena>,
    /// Maximum number of arenas to keep in the pool.
    pub(super) max_pool_size: usize,
    /// Default chunk size for new arenas.
    pub(super) chunk_size: usize,
    /// Statistics.
    stats: ArenaPoolStats,
}
impl ArenaPool {
    /// Create a new arena pool.
    pub fn new() -> Self {
        ArenaPool {
            available: Vec::new(),
            max_pool_size: 8,
            chunk_size: DEFAULT_CHUNK_SIZE,
            stats: ArenaPoolStats::default(),
        }
    }
    /// Create a new arena pool with custom parameters.
    pub fn with_config(max_pool_size: usize, chunk_size: usize) -> Self {
        ArenaPool {
            available: Vec::new(),
            max_pool_size,
            chunk_size,
            stats: ArenaPoolStats::default(),
        }
    }
    /// Acquire an arena from the pool (or create a new one).
    pub fn acquire(&mut self) -> BumpArena {
        self.stats.acquired += 1;
        if let Some(mut arena) = self.available.pop() {
            arena.reset();
            arena
        } else {
            self.stats.created += 1;
            BumpArena::with_chunk_size(self.chunk_size)
        }
    }
    /// Return an arena to the pool for reuse.
    pub fn release(&mut self, arena: BumpArena) {
        self.stats.returned += 1;
        if self.available.len() < self.max_pool_size {
            self.available.push(arena);
        } else {
            self.stats.discarded += 1;
        }
    }
    /// Number of available arenas in the pool.
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
    /// Get the pool statistics.
    pub fn stats(&self) -> &ArenaPoolStats {
        &self.stats
    }
    /// Set the maximum pool size.
    pub fn set_max_pool_size(&mut self, size: usize) {
        self.max_pool_size = size;
        while self.available.len() > self.max_pool_size {
            self.available.pop();
        }
    }
    /// Clear the pool.
    pub fn clear(&mut self) {
        self.available.clear();
    }
}
/// A page manager that allocates fixed-size 4096-byte pages.
#[allow(dead_code)]
pub struct ArenaPageManager {
    pages: Vec<Box<[u8; PAGE_SIZE]>>,
    free_list: Vec<usize>,
    alloc_count: u64,
    free_count: u64,
}
#[allow(dead_code)]
impl ArenaPageManager {
    /// Create an empty page manager.
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            free_list: Vec::new(),
            alloc_count: 0,
            free_count: 0,
        }
    }
    /// Allocate a page. Returns the page index.
    pub fn alloc_page(&mut self) -> usize {
        self.alloc_count += 1;
        if let Some(idx) = self.free_list.pop() {
            idx
        } else {
            let idx = self.pages.len();
            self.pages.push(Box::new([0u8; PAGE_SIZE]));
            idx
        }
    }
    /// Free a page by index.
    pub fn free_page(&mut self, idx: usize) {
        if idx < self.pages.len() {
            for b in self.pages[idx].iter_mut() {
                *b = 0;
            }
            self.free_list.push(idx);
            self.free_count += 1;
        }
    }
    /// Get a reference to a page.
    pub fn page(&self, idx: usize) -> Option<&[u8; PAGE_SIZE]> {
        self.pages.get(idx).map(|p| p.as_ref())
    }
    /// Get a mutable reference to a page.
    pub fn page_mut(&mut self, idx: usize) -> Option<&mut [u8; PAGE_SIZE]> {
        self.pages.get_mut(idx).map(|p| p.as_mut())
    }
    /// Total pages allocated from the system.
    pub fn total_pages(&self) -> usize {
        self.pages.len()
    }
    /// Free pages in the free list.
    pub fn free_pages(&self) -> usize {
        self.free_list.len()
    }
    /// Live pages (total - free).
    pub fn live_pages(&self) -> usize {
        self.pages.len().saturating_sub(self.free_list.len())
    }
    /// Total bytes managed.
    pub fn total_bytes(&self) -> usize {
        self.pages.len() * PAGE_SIZE
    }
    /// Total allocs.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
    /// Total frees.
    pub fn free_count(&self) -> u64 {
        self.free_count
    }
}
/// A bump allocator for fast, thread-local allocation.
///
/// Objects are allocated by advancing a pointer. Deallocation happens
/// all at once when `reset()` is called. This is ideal for temporary
/// allocations during evaluation.
pub struct BumpArena {
    /// The chunks of memory.
    pub(super) chunks: Vec<Chunk>,
    /// Index of the current chunk.
    current_chunk: usize,
    /// Default chunk size for new allocations.
    chunk_size: usize,
    /// Statistics.
    stats: ArenaStats,
}
impl BumpArena {
    /// Create a new bump arena with default chunk size (64 KB).
    pub fn new() -> Self {
        BumpArena {
            chunks: vec![Chunk::new(DEFAULT_CHUNK_SIZE)],
            current_chunk: 0,
            chunk_size: DEFAULT_CHUNK_SIZE,
            stats: ArenaStats::new(),
        }
    }
    /// Create a new bump arena with the specified chunk size.
    pub fn with_chunk_size(size: usize) -> Self {
        let size = size.clamp(MIN_CHUNK_SIZE, MAX_CHUNK_SIZE);
        BumpArena {
            chunks: vec![Chunk::new(size)],
            current_chunk: 0,
            chunk_size: size,
            stats: ArenaStats::new(),
        }
    }
    /// Allocate `size` bytes with default alignment.
    ///
    /// Returns the offset within the arena (chunk_index, byte_offset).
    pub fn alloc(&mut self, size: usize) -> ArenaOffset {
        self.alloc_aligned(size, ARENA_ALIGN)
    }
    /// Allocate `size` bytes with the specified alignment.
    pub fn alloc_aligned(&mut self, size: usize, align: usize) -> ArenaOffset {
        self.stats.total_allocations += 1;
        self.stats.total_bytes_allocated += size as u64;
        if let Some(offset) = self.chunks[self.current_chunk].try_alloc(size, align) {
            return ArenaOffset {
                chunk: self.current_chunk,
                offset,
            };
        }
        for i in (self.current_chunk + 1)..self.chunks.len() {
            if let Some(offset) = self.chunks[i].try_alloc(size, align) {
                self.current_chunk = i;
                return ArenaOffset {
                    chunk: self.current_chunk,
                    offset,
                };
            }
        }
        let new_chunk_size = if size > self.chunk_size {
            (size + align).max(self.chunk_size)
        } else {
            self.chunk_size.min(MAX_CHUNK_SIZE)
        };
        let mut chunk = Chunk::new(new_chunk_size);
        let offset = chunk
            .try_alloc(size, align)
            .expect("freshly allocated chunk must have enough space for the requested allocation");
        self.chunks.push(chunk);
        self.current_chunk = self.chunks.len() - 1;
        self.stats.total_chunks_allocated += 1;
        ArenaOffset {
            chunk: self.current_chunk,
            offset,
        }
    }
    /// Get a byte slice for a previously allocated region.
    pub fn get_bytes(&self, loc: &ArenaOffset, size: usize) -> Option<&[u8]> {
        let chunk = self.chunks.get(loc.chunk)?;
        if loc.offset + size > chunk.data.len() {
            return None;
        }
        Some(&chunk.data[loc.offset..loc.offset + size])
    }
    /// Get a mutable byte slice for a previously allocated region.
    pub fn get_bytes_mut(&mut self, loc: &ArenaOffset, size: usize) -> Option<&mut [u8]> {
        let chunk = self.chunks.get_mut(loc.chunk)?;
        if loc.offset + size > chunk.data.len() {
            return None;
        }
        Some(&mut chunk.data[loc.offset..loc.offset + size])
    }
    /// Reset the arena, freeing all allocations.
    ///
    /// This does not deallocate the underlying memory — chunks are reused.
    pub fn reset(&mut self) {
        for chunk in &mut self.chunks {
            chunk.reset();
        }
        self.current_chunk = 0;
        self.stats.total_resets += 1;
    }
    /// Total number of bytes currently allocated.
    pub fn bytes_used(&self) -> usize {
        self.chunks.iter().map(|c| c.used).sum()
    }
    /// Total capacity (including unused space).
    pub fn total_capacity(&self) -> usize {
        self.chunks.iter().map(|c| c.capacity()).sum()
    }
    /// Number of chunks.
    pub fn num_chunks(&self) -> usize {
        self.chunks.len()
    }
    /// Get the arena statistics.
    pub fn stats(&self) -> &ArenaStats {
        &self.stats
    }
    /// Shrink the arena, releasing unused chunks.
    pub fn shrink(&mut self) {
        let keep = self
            .chunks
            .iter()
            .position(|c| c.used == 0)
            .unwrap_or(self.chunks.len());
        self.chunks.truncate(keep.max(1));
        self.current_chunk = self.current_chunk.min(self.chunks.len() - 1);
    }
}
/// A bump arena with mark/release support for scoped allocation.
#[allow(dead_code)]
pub struct MarkArena {
    buf: Vec<u8>,
    top: usize,
    marks: Vec<usize>,
}
#[allow(dead_code)]
impl MarkArena {
    /// Create a mark arena with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0u8; capacity.max(64)],
            top: 0,
            marks: Vec::new(),
        }
    }
    /// Allocate `size` bytes. Returns an offset or None if full.
    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        if self.top + size > self.buf.len() {
            return None;
        }
        let offset = self.top;
        self.top += size;
        Some(offset)
    }
    /// Save the current top as a mark.
    pub fn mark(&mut self) -> usize {
        let mark = self.top;
        self.marks.push(mark);
        mark
    }
    /// Release back to the most recent mark.
    pub fn release(&mut self) {
        if let Some(mark) = self.marks.pop() {
            self.top = mark;
        }
    }
    /// Release back to a specific mark (and discard newer marks).
    pub fn release_to(&mut self, mark: usize) {
        self.marks.retain(|&m| m < mark);
        self.top = mark.min(self.top);
    }
    /// Reset to empty.
    pub fn reset(&mut self) {
        self.top = 0;
        self.marks.clear();
    }
    /// Current top.
    pub fn top(&self) -> usize {
        self.top
    }
    /// Current mark stack depth.
    pub fn mark_depth(&self) -> usize {
        self.marks.len()
    }
    /// Capacity.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }
}
/// An entry in a generational arena.
#[derive(Debug)]
pub(super) struct GenerationalEntry<T> {
    /// The stored value (None if free).
    value: Option<T>,
    /// The generation when this slot was last written.
    generation: u32,
}
/// Thread-local arena for temporary allocations.
///
/// Each thread gets its own bump arena that can be used for short-lived
/// allocations without synchronization.
pub struct ThreadLocalArena {
    /// The arena.
    arena: BumpArena,
    /// High-water mark for automatic resets.
    _high_water_mark: Cell<usize>,
    /// Number of allocations since last reset.
    _allocs_since_reset: Cell<u64>,
}
impl ThreadLocalArena {
    /// Create a new thread-local arena.
    pub fn new() -> Self {
        ThreadLocalArena {
            arena: BumpArena::new(),
            _high_water_mark: Cell::new(DEFAULT_CHUNK_SIZE),
            _allocs_since_reset: Cell::new(0),
        }
    }
    /// Allocate bytes.
    pub fn alloc(&mut self, size: usize) -> ArenaOffset {
        self._allocs_since_reset
            .set(self._allocs_since_reset.get() + 1);
        self.arena.alloc(size)
    }
    /// Reset the thread-local arena.
    pub fn reset(&mut self) {
        self.arena.reset();
        self._allocs_since_reset.set(0);
    }
    /// Get the underlying arena.
    pub fn arena(&self) -> &BumpArena {
        &self.arena
    }
    /// Bytes used.
    pub fn bytes_used(&self) -> usize {
        self.arena.bytes_used()
    }
}
/// An arena that tracks its allocation pressure and adapts chunk size.
#[allow(dead_code)]
pub struct AdaptiveArena {
    inner: BumpArena,
    pressure_samples: Vec<f64>,
    target_utilization: f64,
    sample_window: usize,
}
#[allow(dead_code)]
impl AdaptiveArena {
    /// Create an adaptive arena.
    pub fn new(target_utilization: f64, sample_window: usize) -> Self {
        Self {
            inner: BumpArena::new(),
            pressure_samples: Vec::new(),
            target_utilization: target_utilization.clamp(0.1, 0.99),
            sample_window: sample_window.max(3),
        }
    }
    /// Allocate bytes and record pressure.
    pub fn alloc(&mut self, size: usize) -> ArenaOffset {
        let result = self.inner.alloc(size);
        let pressure = self.inner.bytes_used() as f64
            / (self.inner.num_chunks() as f64 * DEFAULT_CHUNK_SIZE as f64 + 1.0);
        self.pressure_samples.push(pressure);
        if self.pressure_samples.len() > self.sample_window {
            self.pressure_samples.remove(0);
        }
        result
    }
    /// Average pressure over recent samples.
    pub fn avg_pressure(&self) -> f64 {
        if self.pressure_samples.is_empty() {
            return 0.0;
        }
        self.pressure_samples.iter().sum::<f64>() / self.pressure_samples.len() as f64
    }
    /// Whether the arena is over-utilized.
    pub fn is_over_utilized(&self) -> bool {
        self.avg_pressure() > self.target_utilization
    }
    /// Reset the arena.
    pub fn reset(&mut self) {
        self.inner.reset();
        self.pressure_samples.clear();
    }
    /// Allocated bytes.
    pub fn allocated_bytes(&self) -> usize {
        self.inner.bytes_used()
    }
}
/// Index into a typed arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArenaIdx(pub u32);
impl ArenaIdx {
    /// Create a new arena index.
    pub fn new(index: u32) -> Self {
        ArenaIdx(index)
    }
    /// Get the raw index value.
    pub fn raw(self) -> u32 {
        self.0
    }
}
/// Statistics for arena allocators.
#[derive(Clone, Debug, Default)]
pub struct ArenaStats {
    /// Total number of allocations.
    pub total_allocations: u64,
    /// Total bytes allocated.
    pub total_bytes_allocated: u64,
    /// Total number of arena resets.
    pub total_resets: u64,
    /// Total number of chunks allocated.
    pub total_chunks_allocated: u64,
}
impl ArenaStats {
    /// Create new empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Average allocation size.
    pub fn avg_alloc_size(&self) -> f64 {
        if self.total_allocations == 0 {
            return 0.0;
        }
        self.total_bytes_allocated as f64 / self.total_allocations as f64
    }
    /// Reset the statistics.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// Statistics for a typed arena.
#[derive(Clone, Debug, Default)]
pub struct TypedArenaStats {
    /// Total allocations.
    pub total_allocations: u64,
    /// Total deallocations.
    pub total_deallocations: u64,
    /// Current live count.
    pub live_count: u64,
    /// Peak live count.
    pub peak_count: u64,
}
/// A pool of pre-allocated byte chunks for arena reuse.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ArenaChunkPool {
    chunks: Vec<Vec<u8>>,
    chunk_size: usize,
    max_pooled: usize,
    reused: u64,
    created: u64,
}
#[allow(dead_code)]
impl ArenaChunkPool {
    pub fn new(chunk_size: usize, max_pooled: usize) -> Self {
        Self {
            chunks: Vec::new(),
            chunk_size,
            max_pooled,
            reused: 0,
            created: 0,
        }
    }
    /// Acquire a chunk (from pool or newly allocated).
    pub fn acquire(&mut self) -> Vec<u8> {
        if let Some(mut chunk) = self.chunks.pop() {
            for b in chunk.iter_mut() {
                *b = 0;
            }
            self.reused += 1;
            chunk
        } else {
            self.created += 1;
            vec![0u8; self.chunk_size]
        }
    }
    /// Return a chunk to the pool (or discard if at capacity).
    pub fn release(&mut self, chunk: Vec<u8>) {
        if chunk.len() == self.chunk_size && self.chunks.len() < self.max_pooled {
            self.chunks.push(chunk);
        }
    }
    pub fn pooled_count(&self) -> usize {
        self.chunks.len()
    }
    pub fn reused_count(&self) -> u64 {
        self.reused
    }
    pub fn created_count(&self) -> u64 {
        self.created
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.reused + self.created;
        if total == 0 {
            0.0
        } else {
            self.reused as f64 / total as f64
        }
    }
}
/// A slab allocator for a specific slot size.
#[allow(dead_code)]
pub struct SlabArena {
    slot_size: usize,
    buf: Vec<u8>,
    free_slots: Vec<usize>,
    alloc_count: u64,
}
#[allow(dead_code)]
impl SlabArena {
    /// Create a slab arena with the given slot size and initial capacity.
    pub fn new(slot_size: usize, initial_slots: usize) -> Self {
        let slot_size = slot_size.max(8);
        Self {
            slot_size,
            buf: vec![0u8; slot_size * initial_slots],
            free_slots: (0..initial_slots).rev().collect(),
            alloc_count: 0,
        }
    }
    /// Allocate a slot. Returns the slot offset in the buffer.
    pub fn alloc(&mut self) -> Option<usize> {
        if let Some(slot) = self.free_slots.pop() {
            self.alloc_count += 1;
            Some(slot * self.slot_size)
        } else {
            let new_slot = self.buf.len() / self.slot_size;
            self.buf.extend(vec![0u8; self.slot_size]);
            self.alloc_count += 1;
            Some(new_slot * self.slot_size)
        }
    }
    /// Free a slot by its offset.
    pub fn free(&mut self, offset: usize) {
        let slot = offset / self.slot_size;
        if !self.free_slots.contains(&slot) {
            self.free_slots.push(slot);
        }
    }
    /// Number of live (allocated) slots.
    pub fn live_count(&self) -> usize {
        let total = self.buf.len() / self.slot_size;
        total.saturating_sub(self.free_slots.len())
    }
    /// Total slots.
    pub fn total_slots(&self) -> usize {
        self.buf.len() / self.slot_size
    }
    /// Slot size.
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    /// Total allocation count.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
}
/// Detailed statistics for an arena session.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ArenaExtStats {
    pub alloc_calls: u64,
    pub total_bytes_allocated: u64,
    pub peak_bytes: u64,
    pub reset_count: u64,
    pub overflow_count: u64,
    pub chunk_alloc_count: u64,
}
#[allow(dead_code)]
impl ArenaExtStats {
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another stats object into this one.
    pub fn merge(&mut self, other: &ArenaExtStats) {
        self.alloc_calls += other.alloc_calls;
        self.total_bytes_allocated += other.total_bytes_allocated;
        self.peak_bytes = self.peak_bytes.max(other.peak_bytes);
        self.reset_count += other.reset_count;
        self.overflow_count += other.overflow_count;
        self.chunk_alloc_count += other.chunk_alloc_count;
    }
    pub fn record_alloc(&mut self, bytes: u64) {
        self.alloc_calls += 1;
        self.total_bytes_allocated += bytes;
    }
    pub fn record_reset(&mut self) {
        self.reset_count += 1;
    }
    pub fn record_overflow(&mut self) {
        self.overflow_count += 1;
    }
    pub fn record_chunk_alloc(&mut self) {
        self.chunk_alloc_count += 1;
    }
    pub fn update_peak(&mut self, current_bytes: u64) {
        if current_bytes > self.peak_bytes {
            self.peak_bytes = current_bytes;
        }
    }
    pub fn avg_alloc_size(&self) -> f64 {
        if self.alloc_calls == 0 {
            0.0
        } else {
            self.total_bytes_allocated as f64 / self.alloc_calls as f64
        }
    }
}
/// An arena that automatically returns to a pool when dropped.
pub struct ScopedArena<'pool> {
    /// The underlying arena.
    pub(super) arena: Option<BumpArena>,
    /// The pool to return the arena to.
    pub(super) pool: &'pool mut ArenaPool,
}
impl<'pool> ScopedArena<'pool> {
    /// Create a new scoped arena from a pool.
    pub fn new(pool: &'pool mut ArenaPool) -> Self {
        let arena = pool.acquire();
        ScopedArena {
            arena: Some(arena),
            pool,
        }
    }
    /// Allocate bytes in this arena.
    pub fn alloc(&mut self, size: usize) -> ArenaOffset {
        self.arena
            .as_mut()
            .expect("ScopedArena is valid during its lifetime; arena is always Some before drop")
            .alloc(size)
    }
    /// Get the underlying arena.
    pub fn arena(&self) -> &BumpArena {
        self.arena
            .as_ref()
            .expect("ScopedArena is valid during its lifetime; arena is always Some before drop")
    }
    /// Get the underlying arena mutably.
    pub fn arena_mut(&mut self) -> &mut BumpArena {
        self.arena
            .as_mut()
            .expect("ScopedArena is valid during its lifetime; arena is always Some before drop")
    }
}
/// An arena checkpoint (offset into a BumpArena).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArenaCheckpoint {
    bytes_used: usize,
    chunk_count: usize,
}
#[allow(dead_code)]
impl ArenaCheckpoint {
    pub fn capture(arena: &BumpArena) -> Self {
        Self {
            bytes_used: arena.bytes_used(),
            chunk_count: arena.num_chunks(),
        }
    }
    pub fn bytes_used(&self) -> usize {
        self.bytes_used
    }
    pub fn chunk_count(&self) -> usize {
        self.chunk_count
    }
    pub fn bytes_since(&self, later_bytes_used: usize) -> usize {
        later_bytes_used.saturating_sub(self.bytes_used)
    }
}
/// A snapshot of arena state (offset/byte count only, not actual data).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArenaSnapshot {
    /// Offset at snapshot time.
    pub offset: usize,
    /// Number of chunks at snapshot time.
    pub chunk_count: usize,
    /// Total allocated bytes at snapshot time.
    pub allocated_bytes: usize,
}
#[allow(dead_code)]
impl ArenaSnapshot {
    /// Create a snapshot from an arena.
    pub fn capture(arena: &BumpArena) -> Self {
        Self {
            offset: arena.bytes_used(),
            chunk_count: arena.num_chunks(),
            allocated_bytes: arena.bytes_used(),
        }
    }
    /// Check if `later` represents more allocation than `self`.
    pub fn bytes_since(&self, later: &ArenaSnapshot) -> usize {
        later.allocated_bytes.saturating_sub(self.allocated_bytes)
    }
    /// Whether extra chunks were allocated between `self` and `later`.
    pub fn new_chunks_since(&self, later: &ArenaSnapshot) -> usize {
        later.chunk_count.saturating_sub(self.chunk_count)
    }
}
/// A record of arena allocations for diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AllocRecord {
    pub size: usize,
    pub align: usize,
    pub offset: usize,
    pub label: String,
}
/// A chunk of memory used by the arena.
#[derive(Debug)]
pub(super) struct Chunk {
    /// The actual storage.
    data: Vec<u8>,
    /// How many bytes have been allocated in this chunk.
    used: usize,
}
impl Chunk {
    /// Create a new chunk with the given capacity.
    fn new(capacity: usize) -> Self {
        Chunk {
            data: vec![0u8; capacity],
            used: 0,
        }
    }
    /// Capacity of this chunk.
    fn capacity(&self) -> usize {
        self.data.len()
    }
    /// Remaining space in this chunk.
    fn remaining(&self) -> usize {
        self.data.len() - self.used
    }
    /// Try to allocate `size` bytes with the given alignment.
    fn try_alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        let aligned_used = (self.used + align - 1) & !(align - 1);
        let new_used = aligned_used + size;
        if new_used > self.data.len() {
            return None;
        }
        let offset = aligned_used;
        self.used = new_used;
        Some(offset)
    }
    /// Reset this chunk (mark all space as free).
    fn reset(&mut self) {
        self.used = 0;
    }
}
/// Manages a hierarchy of memory regions.
pub struct RegionManager {
    /// All regions, indexed by ID.
    pub(super) regions: HashMap<u64, Region>,
    /// The next region ID to assign.
    next_id: u64,
    /// Stack of active region IDs (current scope).
    pub(super) scope_stack: Vec<u64>,
}
impl RegionManager {
    /// Create a new region manager.
    pub fn new() -> Self {
        let root = Region::new(0);
        let mut regions = HashMap::new();
        regions.insert(0, root);
        RegionManager {
            regions,
            next_id: 1,
            scope_stack: vec![0],
        }
    }
    /// Get the current active region ID.
    pub fn current_region_id(&self) -> u64 {
        *self.scope_stack.last().unwrap_or(&0)
    }
    /// Push a new region scope.
    pub fn push_region(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let parent_id = self.current_region_id();
        let region = Region::child(id, parent_id);
        self.regions.insert(id, region);
        if let Some(parent) = self.regions.get_mut(&parent_id) {
            parent.add_child(id);
        }
        self.scope_stack.push(id);
        id
    }
    /// Push a region with a custom size.
    pub fn push_region_with_size(&mut self, chunk_size: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let parent_id = self.current_region_id();
        let mut region = Region::with_size(id, chunk_size);
        region.parent_id = Some(parent_id);
        self.regions.insert(id, region);
        if let Some(parent) = self.regions.get_mut(&parent_id) {
            parent.add_child(id);
        }
        self.scope_stack.push(id);
        id
    }
    /// Pop the current region scope (deactivating it).
    pub fn pop_region(&mut self) -> Option<u64> {
        if self.scope_stack.len() <= 1 {
            return None;
        }
        let id = self.scope_stack.pop()?;
        if let Some(region) = self.regions.get_mut(&id) {
            region.deactivate();
        }
        Some(id)
    }
    /// Allocate in the current region.
    pub fn alloc(&mut self, size: usize) -> Option<(u64, ArenaOffset)> {
        let id = self.current_region_id();
        let offset = self.regions.get_mut(&id)?.alloc(size)?;
        Some((id, offset))
    }
    /// Get bytes from a specific region.
    pub fn get_bytes(&self, region_id: u64, loc: &ArenaOffset, size: usize) -> Option<&[u8]> {
        self.regions.get(&region_id)?.get_bytes(loc, size)
    }
    /// Reset a region and all its children.
    pub fn reset_region(&mut self, region_id: u64) {
        let children: Vec<u64> = self
            .regions
            .get(&region_id)
            .map(|r| r.children().to_vec())
            .unwrap_or_default();
        for child_id in children {
            self.reset_region(child_id);
        }
        if let Some(region) = self.regions.get_mut(&region_id) {
            region.reset();
        }
    }
    /// Get a reference to a region.
    pub fn get_region(&self, id: u64) -> Option<&Region> {
        self.regions.get(&id)
    }
    /// Get a mutable reference to a region.
    pub fn get_region_mut(&mut self, id: u64) -> Option<&mut Region> {
        self.regions.get_mut(&id)
    }
    /// Number of regions.
    pub fn num_regions(&self) -> usize {
        self.regions.len()
    }
    /// Total bytes used across all regions.
    pub fn total_bytes_used(&self) -> usize {
        self.regions.values().map(|r| r.bytes_used()).sum()
    }
    /// Total capacity across all regions.
    pub fn total_capacity(&self) -> usize {
        self.regions.values().map(|r| r.total_capacity()).sum()
    }
    /// Depth of the current scope stack.
    pub fn scope_depth(&self) -> usize {
        self.scope_stack.len()
    }
    /// Remove a region and all its children.
    pub fn remove_region(&mut self, region_id: u64) {
        let children: Vec<u64> = self
            .regions
            .get(&region_id)
            .map(|r| r.children().to_vec())
            .unwrap_or_default();
        for child_id in children {
            self.remove_region(child_id);
        }
        self.regions.remove(&region_id);
    }
}
/// A region for bulk allocation and deallocation.
///
/// Regions provide scoped memory management: all allocations in a region
/// are freed when the region is dropped or reset. Regions can be nested.
pub struct Region {
    /// Region identifier.
    pub(super) id: u64,
    /// The underlying bump arena for this region.
    pub(super) arena: BumpArena,
    /// Parent region ID (for nesting).
    pub(super) parent_id: Option<u64>,
    /// Child region IDs.
    pub(super) children: Vec<u64>,
    /// Whether this region is active (can allocate).
    pub(super) active: bool,
    /// Statistics for this region.
    stats: RegionStats,
}
impl Region {
    /// Create a new root region.
    pub fn new(id: u64) -> Self {
        Region {
            id,
            arena: BumpArena::new(),
            parent_id: None,
            children: Vec::new(),
            active: true,
            stats: RegionStats::default(),
        }
    }
    /// Create a new region with a custom arena size.
    pub fn with_size(id: u64, chunk_size: usize) -> Self {
        Region {
            id,
            arena: BumpArena::with_chunk_size(chunk_size),
            parent_id: None,
            children: Vec::new(),
            active: true,
            stats: RegionStats::default(),
        }
    }
    /// Create a child region.
    pub fn child(id: u64, parent_id: u64) -> Self {
        Region {
            id,
            arena: BumpArena::new(),
            parent_id: Some(parent_id),
            children: Vec::new(),
            active: true,
            stats: RegionStats::default(),
        }
    }
    /// Get the region ID.
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Get the parent region ID.
    pub fn parent_id(&self) -> Option<u64> {
        self.parent_id
    }
    /// Check if this region is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
    /// Allocate bytes in this region.
    pub fn alloc(&mut self, size: usize) -> Option<ArenaOffset> {
        if !self.active {
            return None;
        }
        self.stats.allocations += 1;
        self.stats.bytes_allocated += size as u64;
        Some(self.arena.alloc(size))
    }
    /// Get bytes from this region.
    pub fn get_bytes(&self, loc: &ArenaOffset, size: usize) -> Option<&[u8]> {
        self.arena.get_bytes(loc, size)
    }
    /// Get mutable bytes from this region.
    pub fn get_bytes_mut(&mut self, loc: &ArenaOffset, size: usize) -> Option<&mut [u8]> {
        self.arena.get_bytes_mut(loc, size)
    }
    /// Reset this region (free all allocations).
    pub fn reset(&mut self) {
        self.arena.reset();
        self.stats.resets += 1;
    }
    /// Deactivate this region (prevent further allocations).
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    /// Reactivate this region.
    pub fn reactivate(&mut self) {
        self.active = true;
    }
    /// Add a child region ID.
    pub fn add_child(&mut self, child_id: u64) {
        self.children.push(child_id);
    }
    /// Get child region IDs.
    pub fn children(&self) -> &[u64] {
        &self.children
    }
    /// Get the region statistics.
    pub fn stats(&self) -> &RegionStats {
        &self.stats
    }
    /// Get the underlying arena.
    pub fn arena(&self) -> &BumpArena {
        &self.arena
    }
    /// Bytes used in this region.
    pub fn bytes_used(&self) -> usize {
        self.arena.bytes_used()
    }
    /// Total capacity of this region.
    pub fn total_capacity(&self) -> usize {
        self.arena.total_capacity()
    }
}
/// A typed arena for homogeneous allocation.
///
/// All values stored in a `TypedArena<T>` have the same type `T`.
/// Values can be referenced by index (analogous to kernel `Idx<T>`).
pub struct TypedArena<T> {
    /// The stored values.
    pub(super) values: Vec<T>,
    /// Free list for reuse (indices of removed values).
    free_list: Vec<usize>,
    /// Statistics.
    stats: TypedArenaStats,
}
impl<T> TypedArena<T> {
    /// Create a new empty typed arena.
    pub fn new() -> Self {
        TypedArena {
            values: Vec::new(),
            free_list: Vec::new(),
            stats: TypedArenaStats::default(),
        }
    }
    /// Create a new typed arena with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        TypedArena {
            values: Vec::with_capacity(cap),
            free_list: Vec::new(),
            stats: TypedArenaStats::default(),
        }
    }
    /// Allocate a new value and return its index.
    pub fn alloc(&mut self, value: T) -> ArenaIdx {
        self.stats.total_allocations += 1;
        self.stats.live_count += 1;
        if self.stats.live_count > self.stats.peak_count {
            self.stats.peak_count = self.stats.live_count;
        }
        let idx = self.values.len();
        self.values.push(value);
        ArenaIdx(idx as u32)
    }
    /// Get a reference to a value by index.
    pub fn get(&self, idx: ArenaIdx) -> Option<&T> {
        self.values.get(idx.0 as usize)
    }
    /// Get a mutable reference to a value by index.
    pub fn get_mut(&mut self, idx: ArenaIdx) -> Option<&mut T> {
        self.values.get_mut(idx.0 as usize)
    }
    /// Number of values in the arena.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Check if the arena is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    /// Iterate over all values.
    pub fn iter(&self) -> impl Iterator<Item = (ArenaIdx, &T)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, v)| (ArenaIdx(i as u32), v))
    }
    /// Iterate over all values mutably.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ArenaIdx, &mut T)> {
        self.values
            .iter_mut()
            .enumerate()
            .map(|(i, v)| (ArenaIdx(i as u32), v))
    }
    /// Get the arena statistics.
    pub fn stats(&self) -> &TypedArenaStats {
        &self.stats
    }
    /// Clear the arena.
    pub fn clear(&mut self) {
        self.values.clear();
        self.free_list.clear();
        self.stats.live_count = 0;
    }
    /// Capacity of the arena.
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }
}
/// Statistics for the arena pool.
#[derive(Clone, Debug, Default)]
pub struct ArenaPoolStats {
    /// Number of arenas acquired.
    pub acquired: u64,
    /// Number of arenas returned.
    pub returned: u64,
    /// Number of arenas created (not found in pool).
    pub created: u64,
    /// Number of arenas discarded (pool was full).
    pub discarded: u64,
}
/// An arena that records allocation history for debugging.
#[allow(dead_code)]
pub struct ArenaAllocHistory {
    inner: LinearAllocator,
    history: Vec<AllocRecord>,
    max_history: usize,
}
#[allow(dead_code)]
impl ArenaAllocHistory {
    /// Create an arena with history tracking.
    pub fn new(capacity: usize, max_history: usize) -> Self {
        Self {
            inner: LinearAllocator::new(capacity),
            history: Vec::new(),
            max_history,
        }
    }
    /// Allocate and record.
    pub fn alloc_labeled(&mut self, size: usize, align: usize, label: &str) -> Option<usize> {
        let offset = self.inner.alloc_offset(size, align)?;
        if self.history.len() < self.max_history {
            self.history.push(AllocRecord {
                size,
                align,
                offset,
                label: label.to_string(),
            });
        }
        Some(offset)
    }
    /// Get allocation history.
    pub fn history(&self) -> &[AllocRecord] {
        &self.history
    }
    /// Total bytes allocated.
    pub fn top(&self) -> usize {
        self.inner.top()
    }
    /// Reset and clear history.
    pub fn reset(&mut self) {
        self.inner.reset();
        self.history.clear();
    }
    /// Allocation count.
    pub fn alloc_count(&self) -> u64 {
        self.inner.alloc_count()
    }
    /// Largest single allocation.
    pub fn largest_alloc(&self) -> Option<&AllocRecord> {
        self.history.iter().max_by_key(|r| r.size)
    }
}
/// A circular/ring arena that overwrites oldest data when full.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RingArena {
    buf: Vec<u8>,
    head: usize,
    wrap_count: u64,
}
#[allow(dead_code)]
impl RingArena {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0u8; capacity],
            head: 0,
            wrap_count: 0,
        }
    }
    /// Allocate `size` bytes (wraps around, overwriting old data).
    pub fn alloc(&mut self, size: usize) -> usize {
        let start = self.head;
        let cap = self.buf.len();
        if cap == 0 {
            return 0;
        }
        if start + size > cap {
            self.wrap_count += 1;
            self.head = size % cap;
            0
        } else {
            self.head = (self.head + size) % cap;
            if self.head == 0 && size > 0 {
                self.wrap_count += 1;
            }
            start
        }
    }
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }
    pub fn head(&self) -> usize {
        self.head
    }
    pub fn wrap_count(&self) -> u64 {
        self.wrap_count
    }
    pub fn get(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len <= self.buf.len() {
            Some(&self.buf[offset..offset + len])
        } else {
            None
        }
    }
}
/// Tracks arena allocation watermarks (current and peak usage).
#[derive(Clone, Debug, Default)]
pub struct ArenaWatermark {
    /// Current allocated bytes.
    current: u64,
    /// Peak allocated bytes.
    peak: u64,
}
#[allow(dead_code)]
impl ArenaWatermark {
    /// Create a new watermark tracker with zero usage.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an allocation of `bytes`.
    pub fn record_alloc(&mut self, bytes: u64) {
        self.current += bytes;
        if self.current > self.peak {
            self.peak = self.current;
        }
    }
    /// Record a deallocation of `bytes`.
    pub fn record_free(&mut self, bytes: u64) {
        self.current = self.current.saturating_sub(bytes);
    }
    /// Current allocated bytes.
    pub fn current(&self) -> u64 {
        self.current
    }
    /// Peak allocated bytes observed.
    pub fn peak(&self) -> u64 {
        self.peak
    }
    /// Reset both current and peak to zero.
    pub fn reset(&mut self) {
        self.current = 0;
        self.peak = 0;
    }
}
