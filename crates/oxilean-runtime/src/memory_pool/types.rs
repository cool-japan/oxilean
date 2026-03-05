//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use super::functions::{BUCKET_SIZES, CACHE_LINE_SIZE, NUM_BUCKETS, SIZE_CLASSES};

use std::collections::HashMap;

/// Statistics for a pool allocator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PoolStats {
    /// Number of currently allocated (in-use) objects.
    pub allocated: usize,
    /// Number of free (available) objects in the pool.
    pub free: usize,
    /// Total capacity (allocated + free).
    pub total: usize,
    /// Number of slabs allocated from the system.
    pub slab_count: usize,
}
impl PoolStats {
    /// Utilization ratio (0.0 to 1.0). Returns 0.0 if total is 0.
    pub fn utilization(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.allocated as f64 / self.total as f64
        }
    }
}
/// GC color used by tri-color marking.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GcColor {
    White,
    Gray,
    Black,
}
/// A virtual page → physical frame mapping.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PageTable {
    entries: std::collections::HashMap<u64, u64>,
    page_size: usize,
    fault_count: u64,
    hit_count: u64,
}
#[allow(dead_code)]
impl PageTable {
    pub fn new(page_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            page_size,
            fault_count: 0,
            hit_count: 0,
        }
    }
    /// Map virtual page `vpn` to physical frame `pfn`.
    pub fn map(&mut self, vpn: u64, pfn: u64) {
        self.entries.insert(vpn, pfn);
    }
    /// Unmap a virtual page.
    pub fn unmap(&mut self, vpn: u64) -> bool {
        self.entries.remove(&vpn).is_some()
    }
    /// Translate a virtual address to a physical address.
    pub fn translate(&mut self, vaddr: u64) -> Option<u64> {
        let vpn = vaddr / self.page_size as u64;
        let offset = vaddr % self.page_size as u64;
        if let Some(&pfn) = self.entries.get(&vpn) {
            self.hit_count += 1;
            Some(pfn * self.page_size as u64 + offset)
        } else {
            self.fault_count += 1;
            None
        }
    }
    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn mapped_pages(&self) -> usize {
        self.entries.len()
    }
    pub fn fault_count(&self) -> u64 {
        self.fault_count
    }
    pub fn hit_count(&self) -> u64 {
        self.hit_count
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.fault_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
}
/// A typed memory pool allocator.
///
/// Pre-allocates slabs of contiguous memory and maintains a free list for
/// fast allocation and deallocation of objects of type `T`.
pub struct PoolAllocator<T> {
    /// Configuration.
    config: PoolConfig,
    /// Allocated slabs.
    slabs: Vec<Slab>,
    /// Free list: indices encoded as `(slab_index, slot_index)`.
    free_list: Vec<(usize, usize)>,
    /// Count of currently allocated objects.
    allocated_count: usize,
    /// Total capacity across all slabs.
    total_capacity: usize,
    /// Size of the next slab to allocate.
    next_slab_size: usize,
    /// Phantom type marker.
    _marker: PhantomData<T>,
}
impl<T> PoolAllocator<T> {
    /// Create a new pool with default configuration.
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    }
    /// Create a new pool with the given configuration.
    pub fn with_config(config: PoolConfig) -> Self {
        let block_size = config.block_size.max(1);
        Self {
            config: PoolConfig {
                block_size,
                ..config
            },
            slabs: Vec::new(),
            free_list: Vec::new(),
            allocated_count: 0,
            total_capacity: 0,
            next_slab_size: block_size,
            _marker: PhantomData,
        }
    }
    /// Allocate a slot and write `value` into it. Returns a raw pointer to the object.
    ///
    /// Returns `None` if the pool is exhausted (max slabs reached and all slots used).
    pub fn allocate(&mut self, value: T) -> Option<NonNull<T>> {
        if self.free_list.is_empty() {
            self.grow()?;
        }
        let (slab_idx, slot_idx) = self.free_list.pop()?;
        let slot_size = std::mem::size_of::<T>().max(1);
        let slot_ptr = self.slabs[slab_idx].slot_ptr(slot_idx, slot_size) as *mut T;
        unsafe {
            ptr::write(slot_ptr, value);
        }
        self.allocated_count += 1;
        NonNull::new(slot_ptr)
    }
    /// Deallocate an object previously allocated from this pool.
    ///
    /// # Safety
    /// The pointer must have been returned by `allocate` on this pool and must
    /// not have been deallocated already.
    pub unsafe fn deallocate(&mut self, ptr: NonNull<T>) {
        ptr::drop_in_place(ptr.as_ptr());
        let slot_size = std::mem::size_of::<T>().max(1);
        let addr = ptr.as_ptr() as usize;
        for (slab_idx, slab) in self.slabs.iter().enumerate() {
            let base = slab.ptr.as_ptr() as usize;
            let end = base + slab.capacity * slot_size;
            if addr >= base && addr < end {
                let offset = addr - base;
                let slot_idx = offset / slot_size;
                self.free_list.push((slab_idx, slot_idx));
                self.allocated_count = self.allocated_count.saturating_sub(1);
                return;
            }
        }
        debug_assert!(false, "deallocate: pointer does not belong to this pool");
    }
    /// Grow the pool by allocating a new slab.
    fn grow(&mut self) -> Option<()> {
        if self.config.max_blocks > 0 && self.slabs.len() >= self.config.max_blocks {
            return None;
        }
        let slot_layout = Layout::new::<T>();
        let slab_cap = self.next_slab_size;
        let slab = Slab::new(slot_layout, slab_cap)?;
        let slab_idx = self.slabs.len();
        for slot_idx in (0..slab_cap).rev() {
            self.free_list.push((slab_idx, slot_idx));
        }
        self.slabs.push(slab);
        self.total_capacity += slab_cap;
        self.next_slab_size =
            ((self.next_slab_size as f64 * self.config.growth_factor) as usize).max(1);
        Some(())
    }
    /// Get pool statistics.
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            allocated: self.allocated_count,
            free: self.free_list.len(),
            total: self.total_capacity,
            slab_count: self.slabs.len(),
        }
    }
    /// Reset the pool: drop all allocated objects and return all slots to the free list.
    ///
    /// # Safety
    /// All previously returned pointers become invalid after this call. The caller
    /// must ensure no references to pool-allocated objects are held.
    pub unsafe fn reset(&mut self) {
        self.free_list.clear();
        let slot_size = std::mem::size_of::<T>().max(1);
        for (slab_idx, slab) in self.slabs.iter().enumerate() {
            for slot_idx in (0..slab.capacity).rev() {
                let p = slab.slot_ptr(slot_idx, slot_size);
                ptr::write_bytes(p, 0, slot_size);
                self.free_list.push((slab_idx, slot_idx));
            }
        }
        self.allocated_count = 0;
    }
    /// Clear the pool: deallocate all slabs and reset to empty state.
    pub fn clear(&mut self) {
        self.slabs.clear();
        self.free_list.clear();
        self.allocated_count = 0;
        self.total_capacity = 0;
        self.next_slab_size = self.config.block_size.max(1);
    }
    /// Number of currently allocated objects.
    pub fn allocated(&self) -> usize {
        self.allocated_count
    }
    /// Number of free slots available without growing.
    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }
    /// Total capacity across all slabs.
    pub fn capacity(&self) -> usize {
        self.total_capacity
    }
}
/// A free-list backed pool that returns stable indices.
#[allow(dead_code)]
pub struct FreeListPool<T> {
    storage: Vec<Option<T>>,
    free: Vec<u32>,
}
#[allow(dead_code)]
impl<T> FreeListPool<T> {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            free: Vec::new(),
        }
    }
    /// Insert a value and return its stable index.
    pub fn insert(&mut self, value: T) -> PoolIndex {
        if let Some(idx) = self.free.pop() {
            self.storage[idx as usize] = Some(value);
            PoolIndex(idx)
        } else {
            let idx = self.storage.len() as u32;
            self.storage.push(Some(value));
            PoolIndex(idx)
        }
    }
    /// Remove a value by index, returning it if it exists.
    pub fn remove(&mut self, idx: PoolIndex) -> Option<T> {
        let slot = self.storage.get_mut(idx.0 as usize)?;
        let val = slot.take();
        if val.is_some() {
            self.free.push(idx.0);
        }
        val
    }
    /// Get a reference to the value at `idx`.
    pub fn get(&self, idx: PoolIndex) -> Option<&T> {
        self.storage.get(idx.0 as usize)?.as_ref()
    }
    /// Get a mutable reference to the value at `idx`.
    pub fn get_mut(&mut self, idx: PoolIndex) -> Option<&mut T> {
        self.storage.get_mut(idx.0 as usize)?.as_mut()
    }
    /// Number of live (non-freed) values.
    pub fn len(&self) -> usize {
        self.storage.iter().filter(|slot| slot.is_some()).count()
    }
    /// Whether the pool has no live values.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Total capacity (live + freed slots).
    pub fn capacity(&self) -> usize {
        self.storage.len()
    }
    /// Number of free slots.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
    /// Iterate over all live values with their indices.
    pub fn iter(&self) -> impl Iterator<Item = (PoolIndex, &T)> {
        self.storage
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|v| (PoolIndex(i as u32), v)))
    }
    /// Clear the pool.
    pub fn clear(&mut self) {
        self.storage.clear();
        self.free.clear();
    }
}
/// A heap-allocated buffer with guaranteed alignment.
#[allow(dead_code)]
pub struct AlignedBuffer {
    data: Vec<u8>,
    align: usize,
}
#[allow(dead_code)]
impl AlignedBuffer {
    /// Allocate a buffer of `size` bytes with the given alignment.
    pub fn new(size: usize, align: usize) -> Self {
        let align = align.next_power_of_two().max(1);
        let extra = align - 1;
        let mut data = vec![0u8; size + extra];
        let addr = data.as_ptr() as usize;
        let offset = (align - (addr % align)) % align;
        data.drain(..offset);
        data.truncate(size);
        Self { data, align }
    }
    /// Get the buffer's byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    /// Get a mutable byte slice.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
    /// Buffer length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// The alignment guarantee.
    pub fn alignment(&self) -> usize {
        self.align
    }
    /// Fill the buffer with a specific byte value.
    pub fn fill(&mut self, val: u8) {
        for b in self.data.iter_mut() {
            *b = val;
        }
    }
}
/// A contiguous allocation of `capacity` slots of size `slot_size` bytes.
pub(super) struct Slab {
    /// Pointer to the raw allocation.
    pub(super) ptr: NonNull<u8>,
    /// Layout used for the allocation.
    pub(super) layout: Layout,
    /// Number of slots in this slab.
    capacity: usize,
}
impl Slab {
    /// Allocate a new slab with `capacity` slots of the given `slot_layout`.
    fn new(slot_layout: Layout, capacity: usize) -> Option<Self> {
        if capacity == 0 || slot_layout.size() == 0 {
            return None;
        }
        let total_size = slot_layout.size().checked_mul(capacity)?;
        let layout = Layout::from_size_align(total_size, slot_layout.align()).ok()?;
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = NonNull::new(ptr)?;
        Some(Self {
            ptr,
            layout,
            capacity,
        })
    }
    /// Get a pointer to the `index`-th slot.
    fn slot_ptr(&self, index: usize, slot_size: usize) -> *mut u8 {
        debug_assert!(index < self.capacity);
        unsafe { self.ptr.as_ptr().add(index * slot_size) }
    }
}
/// A sequence of pool snapshots for trend analysis.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PoolTimeline {
    snapshots: Vec<PoolSnapshot>,
}
#[allow(dead_code)]
impl PoolTimeline {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, snap: PoolSnapshot) {
        self.snapshots.push(snap);
    }
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
    pub fn latest(&self) -> Option<&PoolSnapshot> {
        self.snapshots.last()
    }
    pub fn avg_utilization(&self) -> f64 {
        if self.snapshots.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.snapshots.iter().map(|s| s.utilization()).sum();
        sum / self.snapshots.len() as f64
    }
    pub fn peak_allocated(&self) -> usize {
        self.snapshots
            .iter()
            .map(|s| s.allocated)
            .max()
            .unwrap_or(0)
    }
}
/// A copy-on-write buffer — shares the source until a write is needed.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CowBuffer {
    data: Vec<u8>,
    shared: bool,
    write_count: usize,
}
#[allow(dead_code)]
impl CowBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            shared: true,
            write_count: 0,
        }
    }
    pub fn from_slice(s: &[u8]) -> Self {
        Self::new(s.to_vec())
    }
    /// Get a read-only view.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    /// Get a mutable view, forcing a copy if needed.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.shared {
            self.data = self.data.clone();
            self.shared = false;
        }
        self.write_count += 1;
        &mut self.data
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn is_shared(&self) -> bool {
        self.shared
    }
    pub fn write_count(&self) -> usize {
        self.write_count
    }
}
/// A diagnostic report for a pool.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PoolReport {
    pub name: String,
    pub allocated: usize,
    pub capacity: usize,
    pub live: usize,
    pub reuse_count: u64,
    pub utilization: f64,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl PoolReport {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    pub fn with_utilization(mut self, util: f64) -> Self {
        self.utilization = util;
        if util > 0.9 {
            self.warnings.push("High utilization (>90%)".to_string());
        }
        self
    }
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    pub fn summary(&self) -> String {
        format!(
            "Pool[{}]: allocated={} capacity={} live={} util={:.1}%",
            self.name,
            self.allocated,
            self.capacity,
            self.live,
            self.utilization * 100.0
        )
    }
}
/// A pool of `MemoryBlock` objects with basic tracking.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct BlockPool {
    blocks: Vec<MemoryBlock>,
    next_id: u64,
    block_capacity: usize,
}
#[allow(dead_code)]
impl BlockPool {
    pub fn new(block_capacity: usize) -> Self {
        Self {
            blocks: Vec::new(),
            next_id: 0,
            block_capacity,
        }
    }
    /// Allocate a new block and return its index.
    pub fn alloc_block(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.blocks.push(MemoryBlock::new(self.block_capacity, id));
        self.blocks.len() - 1
    }
    /// Get a reference to a block.
    pub fn get(&self, idx: usize) -> Option<&MemoryBlock> {
        self.blocks.get(idx)
    }
    /// Get a mutable reference to a block.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut MemoryBlock> {
        self.blocks.get_mut(idx)
    }
    /// Total number of allocated blocks.
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
    /// Total bytes across all blocks.
    pub fn total_bytes(&self) -> usize {
        self.blocks.len() * self.block_capacity
    }
    /// Total used bytes across all blocks.
    pub fn total_used(&self) -> usize {
        self.blocks.iter().map(|b| b.used()).sum()
    }
    /// Average utilization across blocks.
    pub fn avg_utilization(&self) -> f64 {
        if self.blocks.is_empty() {
            return 0.0;
        }
        self.blocks.iter().map(|b| b.utilization()).sum::<f64>() / self.blocks.len() as f64
    }
    /// Clear all blocks (reuse storage).
    pub fn clear_all(&mut self) {
        for b in self.blocks.iter_mut() {
            b.clear();
        }
    }
}
/// A pool that ensures each slot is cache-line aligned (64-byte aligned).
#[allow(dead_code)]
#[derive(Debug)]
pub struct CacheLineAlignedPool {
    storage: Vec<u8>,
    slot_size: usize,
    capacity: usize,
    free_list: Vec<usize>,
    live: usize,
}
#[allow(dead_code)]
impl CacheLineAlignedPool {
    pub fn new(slot_size: usize, capacity: usize) -> Self {
        let aligned = ((slot_size + CACHE_LINE_SIZE - 1) / CACHE_LINE_SIZE) * CACHE_LINE_SIZE;
        let storage = vec![0u8; aligned * capacity];
        let free_list = (0..capacity).collect();
        Self {
            storage,
            slot_size: aligned,
            capacity,
            free_list,
            live: 0,
        }
    }
    /// Allocate a slot. Returns slot index or None if full.
    pub fn alloc(&mut self) -> Option<usize> {
        let idx = self.free_list.pop()?;
        self.live += 1;
        Some(idx)
    }
    /// Free a slot by index.
    pub fn free(&mut self, idx: usize) {
        if idx < self.capacity && self.live > 0 {
            let start = idx * self.slot_size;
            for b in self.storage[start..start + self.slot_size].iter_mut() {
                *b = 0;
            }
            self.free_list.push(idx);
            self.live -= 1;
        }
    }
    /// Get a byte slice for a slot.
    pub fn slot(&self, idx: usize) -> Option<&[u8]> {
        if idx < self.capacity {
            let start = idx * self.slot_size;
            Some(&self.storage[start..start + self.slot_size])
        } else {
            None
        }
    }
    /// Get a mutable byte slice for a slot.
    pub fn slot_mut(&mut self, idx: usize) -> Option<&mut [u8]> {
        if idx < self.capacity {
            let start = idx * self.slot_size;
            Some(&mut self.storage[start..start + self.slot_size])
        } else {
            None
        }
    }
    pub fn live(&self) -> usize {
        self.live
    }
    pub fn available(&self) -> usize {
        self.free_list.len()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    pub fn is_cache_aligned(&self) -> bool {
        self.slot_size % CACHE_LINE_SIZE == 0
    }
}
/// Snapshot of a pool's state at a point in time.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PoolSnapshot {
    pub timestamp_us: u64,
    pub allocated: usize,
    pub capacity: usize,
    pub live_count: usize,
}
#[allow(dead_code)]
impl PoolSnapshot {
    pub fn capture<T>(pool: &PoolAllocator<T>, timestamp_us: u64) -> Self {
        Self {
            timestamp_us,
            allocated: pool.allocated(),
            capacity: pool.capacity(),
            live_count: pool.allocated(),
        }
    }
    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.allocated as f64 / self.capacity as f64
        }
    }
}
/// A header prepended to every heap object for GC support.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    pub color: GcColor,
    pub size_class: u8,
    pub generation: u8,
    pub pinned: bool,
    pub finalizable: bool,
}
#[allow(dead_code)]
impl ObjectHeader {
    pub fn new(size_class: u8, generation: u8) -> Self {
        Self {
            color: GcColor::White,
            size_class,
            generation,
            pinned: false,
            finalizable: false,
        }
    }
    pub fn mark_gray(&mut self) {
        self.color = GcColor::Gray;
    }
    pub fn mark_black(&mut self) {
        self.color = GcColor::Black;
    }
    pub fn reset_to_white(&mut self) {
        self.color = GcColor::White;
    }
    pub fn is_white(&self) -> bool {
        self.color == GcColor::White
    }
    pub fn is_gray(&self) -> bool {
        self.color == GcColor::Gray
    }
    pub fn is_black(&self) -> bool {
        self.color == GcColor::Black
    }
    pub fn pin(&mut self) {
        self.pinned = true;
    }
    pub fn unpin(&mut self) {
        self.pinned = false;
    }
}
/// A double-buffered pool for read/write separation.
#[allow(dead_code)]
pub struct PoolMirror<T: Clone> {
    front: Vec<Option<T>>,
    back: Vec<Option<T>>,
}
#[allow(dead_code)]
impl<T: Clone> PoolMirror<T> {
    /// Create an empty mirror.
    pub fn new() -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
        }
    }
    /// Write a value to the back buffer at the given index.
    pub fn write(&mut self, idx: usize, value: T) {
        while self.back.len() <= idx {
            self.back.push(None);
        }
        self.back[idx] = Some(value);
    }
    /// Flip the buffers (swap front and back).
    pub fn flip(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
    /// Read from the front buffer.
    pub fn read(&self, idx: usize) -> Option<&T> {
        self.front.get(idx)?.as_ref()
    }
    /// Front buffer length.
    pub fn front_len(&self) -> usize {
        self.front.iter().filter(|s| s.is_some()).count()
    }
    /// Back buffer length.
    pub fn back_len(&self) -> usize {
        self.back.iter().filter(|s| s.is_some()).count()
    }
}
/// A pool that doubles its block size each time it runs out.
#[allow(dead_code)]
#[derive(Debug)]
pub struct GrowablePool {
    chunks: Vec<Vec<u8>>,
    current: usize,
    initial_size: usize,
    total_allocated: usize,
}
#[allow(dead_code)]
impl GrowablePool {
    pub fn new(initial_size: usize) -> Self {
        Self {
            chunks: vec![vec![0u8; initial_size]],
            current: 0,
            initial_size,
            total_allocated: 0,
        }
    }
    /// Allocate `size` bytes. Grows automatically.
    pub fn alloc(&mut self, size: usize) -> &mut [u8] {
        let chunk_len = self.chunks[self.current].len();
        if self.total_allocated + size > chunk_len {
            let new_size = (chunk_len * 2).max(size);
            self.chunks.push(vec![0u8; new_size]);
            self.current += 1;
            self.total_allocated = 0;
        }
        let start = self.total_allocated;
        self.total_allocated += size;
        &mut self.chunks[self.current][start..start + size]
    }
    /// Reset the pool (keeps the first chunk).
    pub fn reset(&mut self) {
        self.chunks.truncate(1);
        self.current = 0;
        self.total_allocated = 0;
    }
    pub fn num_chunks(&self) -> usize {
        self.chunks.len()
    }
    pub fn total_capacity(&self) -> usize {
        self.chunks.iter().map(|c| c.len()).sum()
    }
}
/// A bump allocator for raw byte sequences.
///
/// Allocations are extremely fast (pointer increment) but individual
/// deallocations are not supported. The entire arena can be reset at once.
pub struct ArenaAllocator {
    /// Backing storage chunks.
    chunks: Vec<Vec<u8>>,
    /// Current write offset within the active (last) chunk.
    offset: usize,
    /// Size of the next chunk to allocate.
    chunk_size: usize,
    /// Total bytes allocated by the user.
    total_allocated: usize,
    /// Total bytes reserved (sum of chunk capacities).
    total_reserved: usize,
    /// Growth factor for chunk sizes.
    growth_factor: f64,
}
impl ArenaAllocator {
    /// Create a new arena with the given initial chunk size.
    pub fn new(initial_chunk_size: usize) -> Self {
        let cs = initial_chunk_size.max(64);
        Self {
            chunks: Vec::new(),
            offset: 0,
            chunk_size: cs,
            total_allocated: 0,
            total_reserved: 0,
            growth_factor: 2.0,
        }
    }
    /// Allocate `size` bytes with alignment `align`. Returns a pointer to the
    /// allocated region, or `None` if allocation fails.
    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        if size == 0 {
            return NonNull::new(align as *mut u8);
        }
        let align = align.max(1);
        if let Some(ptr) = self.try_alloc_from_current(size, align) {
            self.total_allocated += size;
            return Some(ptr);
        }
        let needed = size + align - 1;
        let new_size = self.chunk_size.max(needed);
        let mut chunk = vec![0u8; new_size];
        self.total_reserved += new_size;
        let base = chunk.as_mut_ptr() as usize;
        let aligned_offset = (base.wrapping_add(align - 1)) & !(align - 1);
        let padding = aligned_offset - base;
        debug_assert!(padding + size <= new_size);
        let ptr = unsafe { chunk.as_mut_ptr().add(padding) };
        self.offset = padding + size;
        self.chunks.push(chunk);
        self.chunk_size = ((self.chunk_size as f64 * self.growth_factor) as usize).max(64);
        self.total_allocated += size;
        NonNull::new(ptr)
    }
    /// Try to allocate from the current (last) chunk.
    fn try_alloc_from_current(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let chunk = self.chunks.last_mut()?;
        let base = chunk.as_mut_ptr() as usize;
        let current = base + self.offset;
        let aligned = (current.wrapping_add(align - 1)) & !(align - 1);
        let padding = aligned - current;
        let new_offset = self.offset + padding + size;
        if new_offset > chunk.len() {
            return None;
        }
        let ptr = unsafe { chunk.as_mut_ptr().add(self.offset + padding) };
        self.offset = new_offset;
        NonNull::new(ptr)
    }
    /// Reset the arena, invalidating all previous allocations.
    /// Keeps the allocated chunks for reuse.
    pub fn reset(&mut self) {
        if self.chunks.len() > 1 {
            let total: usize = self.chunks.iter().map(|c| c.len()).sum();
            self.chunks.clear();
            self.chunks.push(vec![0u8; total]);
            self.total_reserved = total;
        }
        self.offset = 0;
        self.total_allocated = 0;
    }
    /// Total bytes allocated by user requests.
    pub fn bytes_allocated(&self) -> usize {
        self.total_allocated
    }
    /// Total bytes reserved from the system.
    pub fn bytes_reserved(&self) -> usize {
        self.total_reserved
    }
    /// Number of chunks allocated.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
    /// Fragmentation ratio: 1.0 - (allocated / reserved). Returns 0.0 if nothing reserved.
    pub fn fragmentation(&self) -> f64 {
        if self.total_reserved == 0 {
            0.0
        } else {
            1.0 - (self.total_allocated as f64 / self.total_reserved as f64)
        }
    }
    /// Allocate space for a value of type `T` and write it. Returns a pointer.
    pub fn alloc_value<T>(&mut self, value: T) -> Option<NonNull<T>> {
        let layout = Layout::new::<T>();
        let ptr = self.alloc_bytes(layout.size(), layout.align())?;
        let typed = ptr.as_ptr() as *mut T;
        unsafe {
            ptr::write(typed, value);
        }
        NonNull::new(typed)
    }
}
/// Configuration for a pool allocator.
#[derive(Clone, Debug)]
pub struct PoolConfig {
    /// Number of objects per slab (initial allocation unit).
    pub block_size: usize,
    /// Maximum number of slabs (0 = unlimited).
    pub max_blocks: usize,
    /// When a new slab is needed, multiply the previous slab size by this factor.
    /// Clamped to >= 1.0.
    pub growth_factor: f64,
}
impl PoolConfig {
    /// Create a config with the given initial block size.
    pub fn with_block_size(mut self, n: usize) -> Self {
        self.block_size = n.max(1);
        self
    }
    /// Set the maximum number of slabs.
    pub fn with_max_blocks(mut self, n: usize) -> Self {
        self.max_blocks = n;
        self
    }
    /// Set the growth factor (clamped to >= 1.0).
    pub fn with_growth_factor(mut self, f: f64) -> Self {
        self.growth_factor = if f < 1.0 { 1.0 } else { f };
        self
    }
}
/// A bump-pointer arena that allocates values of type `T`.
/// Faster than `PoolAllocator<T>` for append-only workloads.
#[allow(dead_code)]
pub struct TypedArena<T> {
    chunks: Vec<Vec<T>>,
    chunk_size: usize,
}
#[allow(dead_code)]
impl<T> TypedArena<T> {
    /// Create a new typed arena with the given chunk size.
    pub fn new(chunk_size: usize) -> Self {
        let cs = chunk_size.max(8);
        Self {
            chunks: Vec::new(),
            chunk_size: cs,
        }
    }
    /// Allocate a value in the arena.
    pub fn alloc(&mut self, value: T) -> &mut T {
        let need_new_chunk = self
            .chunks
            .last()
            .map(|c| c.len() >= c.capacity())
            .unwrap_or(true);
        if need_new_chunk {
            let new_chunk = Vec::with_capacity(self.chunk_size);
            self.chunks.push(new_chunk);
            self.chunk_size = self.chunk_size.saturating_mul(2).max(8);
        }
        let chunk = self.chunks.last_mut().unwrap_or_else(|| {
            unreachable!("chunks is always non-empty: a new chunk was pushed above if needed")
        });
        chunk.push(value);
        chunk.last_mut().unwrap_or_else(|| {
            unreachable!("chunk is always non-empty: value was just pushed above")
        })
    }
    /// Total number of allocated values.
    pub fn len(&self) -> usize {
        self.chunks.iter().map(|c| c.len()).sum()
    }
    /// Whether the arena is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Number of chunks.
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
    /// Iterate over all allocated values.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.chunks.iter().flat_map(|c| c.iter())
    }
    /// Iterate mutably over all allocated values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.chunks.iter_mut().flat_map(|c| c.iter_mut())
    }
    /// Clear the arena, dropping all values.
    pub fn clear(&mut self) {
        self.chunks.clear();
    }
}
/// A region-based allocator: multiple named regions that can be freed independently.
#[allow(dead_code)]
pub struct RegionAllocator {
    regions: Vec<Option<Vec<u8>>>,
    free_ids: Vec<u32>,
}
#[allow(dead_code)]
impl RegionAllocator {
    /// Create a new region allocator.
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            free_ids: Vec::new(),
        }
    }
    /// Allocate a new region with at least `size` bytes.
    pub fn alloc_region(&mut self, size: usize) -> RegionId {
        let buf = vec![0u8; size];
        if let Some(id) = self.free_ids.pop() {
            self.regions[id as usize] = Some(buf);
            RegionId(id)
        } else {
            let id = self.regions.len() as u32;
            self.regions.push(Some(buf));
            RegionId(id)
        }
    }
    /// Free a region by ID.
    pub fn free_region(&mut self, id: RegionId) {
        if let Some(slot) = self.regions.get_mut(id.0 as usize) {
            if slot.take().is_some() {
                self.free_ids.push(id.0);
            }
        }
    }
    /// Get a reference to a region's bytes.
    pub fn region_bytes(&self, id: RegionId) -> Option<&[u8]> {
        self.regions.get(id.0 as usize)?.as_deref()
    }
    /// Get a mutable reference to a region's bytes.
    pub fn region_bytes_mut(&mut self, id: RegionId) -> Option<&mut [u8]> {
        self.regions.get_mut(id.0 as usize)?.as_deref_mut()
    }
    /// Number of live regions.
    pub fn live_count(&self) -> usize {
        self.regions.iter().filter(|r| r.is_some()).count()
    }
    /// Total bytes across all live regions.
    pub fn total_bytes(&self) -> usize {
        self.regions
            .iter()
            .filter_map(|r| r.as_ref())
            .map(|b| b.len())
            .sum()
    }
}
/// An index into a `FreeListPool<T>`.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PoolIndex(pub u32);
/// Tracks free gaps in a byte range and allocates from them.
#[allow(dead_code)]
#[derive(Debug)]
pub struct GapAllocator {
    total: usize,
    gaps: Vec<(usize, usize)>,
}
#[allow(dead_code)]
impl GapAllocator {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            gaps: vec![(0, total)],
        }
    }
    /// Allocate `size` bytes. Returns start offset or None.
    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        for i in 0..self.gaps.len() {
            let (start, end) = self.gaps[i];
            if end - start >= size {
                let alloc_start = start;
                let new_start = start + size;
                if new_start >= end {
                    self.gaps.remove(i);
                } else {
                    self.gaps[i] = (new_start, end);
                }
                return Some(alloc_start);
            }
        }
        None
    }
    /// Free a range back to the gap list (no merging for simplicity).
    pub fn free(&mut self, start: usize, size: usize) {
        self.gaps.push((start, start + size));
        self.gaps.sort_unstable();
    }
    pub fn gap_count(&self) -> usize {
        self.gaps.len()
    }
    pub fn total_free(&self) -> usize {
        self.gaps.iter().map(|(s, e)| e - s).sum()
    }
    pub fn total(&self) -> usize {
        self.total
    }
}
/// A region identifier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RegionId(pub u32);
/// A stack-based pool of pre-allocated T slots.
///
/// Slots are pushed back when released; popped when acquired.
#[allow(dead_code)]
#[derive(Debug)]
pub struct StackPool<T> {
    stack: Vec<T>,
    capacity: usize,
    acquired: usize,
    released: usize,
}
#[allow(dead_code)]
impl<T: Default + Clone> StackPool<T> {
    /// Create a pool with `capacity` pre-allocated slots.
    pub fn new(capacity: usize) -> Self {
        let stack = (0..capacity).map(|_| T::default()).collect();
        Self {
            stack,
            capacity,
            acquired: 0,
            released: 0,
        }
    }
    /// Acquire a slot from the pool. Returns `None` if exhausted.
    pub fn acquire(&mut self) -> Option<T> {
        if let Some(item) = self.stack.pop() {
            self.acquired += 1;
            Some(item)
        } else {
            None
        }
    }
    /// Return a slot to the pool. Discards if over capacity.
    pub fn release(&mut self, item: T) {
        self.released += 1;
        if self.stack.len() < self.capacity {
            self.stack.push(item);
        }
    }
    /// How many slots are currently available.
    pub fn available(&self) -> usize {
        self.stack.len()
    }
    /// Total acquisitions so far.
    pub fn total_acquired(&self) -> usize {
        self.acquired
    }
    /// Total releases so far.
    pub fn total_released(&self) -> usize {
        self.released
    }
    /// Whether the pool is empty (all slots in use).
    pub fn is_exhausted(&self) -> bool {
        self.stack.is_empty()
    }
    /// Reset the pool back to full capacity.
    pub fn reset(&mut self) {
        self.stack.clear();
        for _ in 0..self.capacity {
            self.stack.push(T::default());
        }
    }
    /// Maximum capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
/// A collection of pools, one per size class.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SizeClassPool {
    buckets: Vec<Vec<Vec<u8>>>,
    alloc_count: [u64; 6],
    free_count: [u64; 6],
}
#[allow(dead_code)]
impl SizeClassPool {
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 6],
            alloc_count: [0u64; 6],
            free_count: [0u64; 6],
        }
    }
    /// Find the size class index for a given allocation size.
    pub fn size_class_for(size: usize) -> Option<usize> {
        SIZE_CLASSES.iter().position(|&sc| sc >= size)
    }
    /// Allocate a buffer of the appropriate size class.
    pub fn alloc(&mut self, size: usize) -> Option<Vec<u8>> {
        let idx = Self::size_class_for(size)?;
        self.alloc_count[idx] += 1;
        if let Some(buf) = self.buckets[idx].pop() {
            Some(buf)
        } else {
            Some(vec![0u8; SIZE_CLASSES[idx]])
        }
    }
    /// Return a buffer back to its size class pool.
    pub fn free(&mut self, mut buf: Vec<u8>) {
        if let Some(idx) = SIZE_CLASSES.iter().position(|&sc| sc == buf.len()) {
            self.free_count[idx] += 1;
            for b in buf.iter_mut() {
                *b = 0;
            }
            self.buckets[idx].push(buf);
        }
    }
    /// Number of free slots for a given size class index.
    pub fn free_slots(&self, class_idx: usize) -> usize {
        self.buckets.get(class_idx).map(|v| v.len()).unwrap_or(0)
    }
    /// Total allocations across all size classes.
    pub fn total_allocs(&self) -> u64 {
        self.alloc_count.iter().sum()
    }
    /// Total frees across all size classes.
    pub fn total_frees(&self) -> u64 {
        self.free_count.iter().sum()
    }
}
/// A Vec wrapper that automatically compacts (removes tombstones) when a threshold is reached.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CompactVec<T> {
    data: Vec<Option<T>>,
    live: usize,
    dead: usize,
    compact_threshold: usize,
}
#[allow(dead_code)]
impl<T: Clone> CompactVec<T> {
    pub fn new(compact_threshold: usize) -> Self {
        Self {
            data: Vec::new(),
            live: 0,
            dead: 0,
            compact_threshold,
        }
    }
    pub fn push(&mut self, item: T) {
        self.data.push(Some(item));
        self.live += 1;
    }
    pub fn remove(&mut self, idx: usize) -> bool {
        if idx < self.data.len() && self.data[idx].is_some() {
            self.data[idx] = None;
            self.live -= 1;
            self.dead += 1;
            if self.dead >= self.compact_threshold {
                self.compact();
            }
            true
        } else {
            false
        }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)?.as_ref()
    }
    fn compact(&mut self) {
        self.data.retain(|slot| slot.is_some());
        self.dead = 0;
    }
    pub fn live_count(&self) -> usize {
        self.live
    }
    pub fn dead_count(&self) -> usize {
        self.dead
    }
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
    pub fn iter_live(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|s| s.as_ref())
    }
}
/// A multi-bucket memory pool that routes allocation requests to the
/// smallest bucket that can satisfy the request.
#[allow(dead_code)]
pub struct BucketPool {
    buckets: [Vec<Vec<u8>>; NUM_BUCKETS],
    alloc_stats: [usize; NUM_BUCKETS],
    miss_count: usize,
}
#[allow(dead_code)]
impl BucketPool {
    /// Create a new bucket pool.
    pub fn new() -> Self {
        Self {
            buckets: Default::default(),
            alloc_stats: [0; NUM_BUCKETS],
            miss_count: 0,
        }
    }
    /// Allocate a buffer of at least `size` bytes.
    /// Returns `None` if no bucket can fit the request (size > 1024).
    pub fn alloc(&mut self, size: usize) -> Option<Vec<u8>> {
        let bucket = BUCKET_SIZES.iter().position(|&b| b >= size)?;
        self.alloc_stats[bucket] += 1;
        if let Some(buf) = self.buckets[bucket].pop() {
            Some(buf)
        } else {
            Some(vec![0u8; BUCKET_SIZES[bucket]])
        }
    }
    /// Return a buffer to the appropriate bucket.
    pub fn free(&mut self, mut buf: Vec<u8>) {
        let size = buf.len();
        if let Some(bucket) = BUCKET_SIZES.iter().position(|&b| b == size) {
            for b in buf.iter_mut() {
                *b = 0;
            }
            self.buckets[bucket].push(buf);
        } else {
            self.miss_count += 1;
        }
    }
    /// Get the allocation count for each bucket.
    pub fn alloc_stats(&self) -> &[usize; NUM_BUCKETS] {
        &self.alloc_stats
    }
    /// Number of buffers currently in the pool (across all buckets).
    pub fn pool_size(&self) -> usize {
        self.buckets.iter().map(|b| b.len()).sum()
    }
    /// Total bytes held in the pool.
    pub fn pool_bytes(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .map(|(i, b)| b.len() * BUCKET_SIZES[i])
            .sum()
    }
}
/// A memory budget tracker. Allocations fail when the budget is exceeded.
#[allow(dead_code)]
pub struct MemoryBudget {
    pub(super) limit: usize,
    pub(super) used: usize,
    pub(super) peak: usize,
    alloc_count: u64,
    dealloc_count: u64,
}
#[allow(dead_code)]
impl MemoryBudget {
    /// Create a budget with the given byte limit.
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            used: 0,
            peak: 0,
            alloc_count: 0,
            dealloc_count: 0,
        }
    }
    /// Try to allocate `size` bytes. Returns `true` if within budget.
    pub fn try_alloc(&mut self, size: usize) -> bool {
        if self
            .used
            .checked_add(size)
            .map(|u| u <= self.limit)
            .unwrap_or(false)
        {
            self.used += size;
            if self.used > self.peak {
                self.peak = self.used;
            }
            self.alloc_count += 1;
            true
        } else {
            false
        }
    }
    /// Release `size` bytes.
    pub fn release(&mut self, size: usize) {
        self.used = self.used.saturating_sub(size);
        self.dealloc_count += 1;
    }
    /// Current usage in bytes.
    pub fn used(&self) -> usize {
        self.used
    }
    /// Peak usage ever recorded.
    pub fn peak(&self) -> usize {
        self.peak
    }
    /// Budget limit.
    pub fn limit(&self) -> usize {
        self.limit
    }
    /// Remaining bytes before the limit.
    pub fn remaining(&self) -> usize {
        self.limit.saturating_sub(self.used)
    }
    /// Utilization fraction (0.0 to 1.0).
    pub fn utilization(&self) -> f64 {
        if self.limit == 0 {
            0.0
        } else {
            self.used as f64 / self.limit as f64
        }
    }
    /// Number of successful allocations.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
    /// Number of deallocations.
    pub fn dealloc_count(&self) -> u64 {
        self.dealloc_count
    }
}
/// A raw memory block of fixed size.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryBlock {
    data: Vec<u8>,
    used: usize,
    id: u64,
    tag: u8,
}
#[allow(dead_code)]
impl MemoryBlock {
    pub fn new(capacity: usize, id: u64) -> Self {
        Self {
            data: vec![0u8; capacity],
            used: 0,
            id,
            tag: 0,
        }
    }
    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> bool {
        if offset + bytes.len() > self.data.len() {
            return false;
        }
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes);
        if offset + bytes.len() > self.used {
            self.used = offset + bytes.len();
        }
        true
    }
    pub fn read_bytes(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len > self.data.len() {
            return None;
        }
        Some(&self.data[offset..offset + len])
    }
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
    pub fn used(&self) -> usize {
        self.used
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn tag(&self) -> u8 {
        self.tag
    }
    pub fn set_tag(&mut self, t: u8) {
        self.tag = t;
    }
    pub fn is_full(&self) -> bool {
        self.used >= self.data.len()
    }
    pub fn utilization(&self) -> f64 {
        if self.data.is_empty() {
            0.0
        } else {
            self.used as f64 / self.data.len() as f64
        }
    }
    pub fn clear(&mut self) {
        self.used = 0;
        for b in self.data.iter_mut() {
            *b = 0;
        }
    }
}
