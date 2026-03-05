//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

/// A history of GC cycles for analysis and debugging.
#[derive(Clone, Debug, Default)]
pub struct GcHistory {
    /// All recorded cycles.
    pub cycles: Vec<GcCycleRecord>,
    /// Maximum history length (0 = unlimited).
    pub max_cycles: usize,
}
impl GcHistory {
    /// Create an unlimited history.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a history with a maximum size.
    pub fn with_capacity(max_cycles: usize) -> Self {
        Self {
            cycles: Vec::new(),
            max_cycles,
        }
    }
    /// Add a cycle record, evicting the oldest if at capacity.
    pub fn record(&mut self, record: GcCycleRecord) {
        if self.max_cycles > 0 && self.cycles.len() >= self.max_cycles {
            self.cycles.remove(0);
        }
        self.cycles.push(record);
    }
    /// Total bytes freed across all recorded cycles.
    pub fn total_freed(&self) -> usize {
        self.cycles.iter().map(|c| c.bytes_freed).sum()
    }
    /// Average collection duration.
    pub fn avg_duration_ns(&self) -> f64 {
        if self.cycles.is_empty() {
            0.0
        } else {
            self.cycles.iter().map(|c| c.duration_ns).sum::<u64>() as f64 / self.cycles.len() as f64
        }
    }
    /// Average survival rate across all cycles.
    pub fn avg_survival_rate(&self) -> f64 {
        if self.cycles.is_empty() {
            1.0
        } else {
            self.cycles.iter().map(|c| c.survival_rate()).sum::<f64>() / self.cycles.len() as f64
        }
    }
    /// Number of major collections recorded.
    pub fn major_count(&self) -> usize {
        self.cycles.iter().filter(|c| c.is_major).count()
    }
    /// Number of minor collections recorded.
    pub fn minor_count(&self) -> usize {
        self.cycles.iter().filter(|c| !c.is_major).count()
    }
    /// The last `n` cycles.
    pub fn last_n(&self, n: usize) -> &[GcCycleRecord] {
        let start = self.cycles.len().saturating_sub(n);
        &self.cycles[start..]
    }
}
/// A write-barrier action to be taken when a pointer is updated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierAction {
    /// No action needed.
    None,
    /// Gray the source object (Dijkstra-style).
    GraySource,
    /// Gray the destination (destination-gray).
    GrayDest,
    /// Record the old value for snapshot-at-the-beginning.
    SnapshotOld,
}
/// A simple first-fit free list allocator.
#[derive(Clone, Debug)]
pub struct FreeList {
    /// List of free blocks as (offset, size) pairs.
    pub blocks: Vec<(usize, usize)>,
    /// Total heap capacity.
    pub capacity: usize,
    /// Total bytes currently allocated.
    pub allocated: usize,
}
impl FreeList {
    /// Create a new free list with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            blocks: vec![(0, capacity)],
            capacity,
            allocated: 0,
        }
    }
    /// Allocate a block of the given size. Returns the offset or `None`.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        for i in 0..self.blocks.len() {
            let (offset, block_size) = self.blocks[i];
            if block_size >= size {
                let remaining = block_size - size;
                if remaining > 0 {
                    self.blocks[i] = (offset + size, remaining);
                } else {
                    self.blocks.remove(i);
                }
                self.allocated += size;
                return Some(offset);
            }
        }
        None
    }
    /// Free a block at the given offset with the given size.
    pub fn free(&mut self, offset: usize, size: usize) {
        self.allocated = self.allocated.saturating_sub(size);
        self.blocks.push((offset, size));
        self.coalesce();
    }
    /// Merge adjacent free blocks.
    pub fn coalesce(&mut self) {
        self.blocks.sort_by_key(|&(off, _)| off);
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            let (off_a, size_a) = self.blocks[i];
            let (off_b, size_b) = self.blocks[i + 1];
            if off_a + size_a == off_b {
                self.blocks[i] = (off_a, size_a + size_b);
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
    /// Number of free blocks.
    pub fn free_block_count(&self) -> usize {
        self.blocks.len()
    }
    /// Total free bytes.
    pub fn free_bytes(&self) -> usize {
        self.blocks.iter().map(|&(_, s)| s).sum()
    }
    /// Utilization ratio (allocated / capacity).
    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.allocated as f64 / self.capacity as f64
        }
    }
    /// Largest free block size.
    pub fn largest_free(&self) -> usize {
        self.blocks.iter().map(|&(_, s)| s).max().unwrap_or(0)
    }
}
/// Results from a single GC benchmark run.
#[derive(Clone, Debug, Default)]
pub struct GcBenchmarkResult {
    /// Strategy name.
    pub strategy: String,
    /// Total allocated bytes.
    pub total_allocated: usize,
    /// Total freed bytes.
    pub total_freed: usize,
    /// Number of collections.
    pub collections: u64,
    /// Total allocation time in ns (simulated).
    pub alloc_time_ns: u64,
    /// Total GC time in ns (simulated).
    pub gc_time_ns: u64,
}
impl GcBenchmarkResult {
    /// Throughput: allocation rate in bytes/ns.
    pub fn alloc_throughput(&self) -> f64 {
        if self.alloc_time_ns == 0 {
            0.0
        } else {
            self.total_allocated as f64 / self.alloc_time_ns as f64
        }
    }
    /// Overhead ratio: GC time / (alloc time + GC time).
    pub fn gc_overhead(&self) -> f64 {
        let total = self.alloc_time_ns + self.gc_time_ns;
        if total == 0 {
            0.0
        } else {
            self.gc_time_ns as f64 / total as f64
        }
    }
}
/// Statistics recorded by a GC.
#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub collections: u64,
    pub bytes_collected: u64,
    pub pause_time_ns: u64,
    pub heap_size: usize,
}
impl GcStats {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_collection(&mut self, bytes: u64, pause_ns: u64) {
        self.collections += 1;
        self.bytes_collected += bytes;
        self.pause_time_ns += pause_ns;
    }
    /// Returns the percentage of time NOT spent pausing.
    pub fn throughput_pct(&self, total_ns: u64) -> f64 {
        if total_ns == 0 {
            return 100.0;
        }
        let pause = self.pause_time_ns.min(total_ns);
        (total_ns - pause) as f64 / total_ns as f64 * 100.0
    }
}
/// A detailed record of a single GC collection cycle.
#[derive(Clone, Debug)]
pub struct GcCycleRecord {
    /// Cycle sequence number.
    pub cycle_id: u64,
    /// Strategy used.
    pub strategy: String,
    /// Bytes allocated before collection.
    pub bytes_before: usize,
    /// Bytes alive after collection.
    pub bytes_after: usize,
    /// Bytes freed.
    pub bytes_freed: usize,
    /// Duration of the cycle in nanoseconds.
    pub duration_ns: u64,
    /// Whether this was a full (major) collection.
    pub is_major: bool,
}
impl GcCycleRecord {
    /// Create a new cycle record.
    pub fn new(cycle_id: u64, strategy: &str, bytes_before: usize) -> Self {
        Self {
            cycle_id,
            strategy: strategy.to_string(),
            bytes_before,
            bytes_after: 0,
            bytes_freed: 0,
            duration_ns: 0,
            is_major: false,
        }
    }
    /// Finalize this record after collection.
    pub fn finalize(&mut self, bytes_after: usize, duration_ns: u64) {
        self.bytes_after = bytes_after;
        self.bytes_freed = self.bytes_before.saturating_sub(bytes_after);
        self.duration_ns = duration_ns;
    }
    /// Survival rate (bytes_after / bytes_before).
    pub fn survival_rate(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    /// Reclaim rate (bytes_freed / bytes_before).
    pub fn reclaim_rate(&self) -> f64 {
        if self.bytes_before == 0 {
            0.0
        } else {
            self.bytes_freed as f64 / self.bytes_before as f64
        }
    }
}
/// A queue for objects that need finalization before being freed.
#[derive(Clone, Debug, Default)]
pub struct FinalizerQueue {
    /// Object IDs pending finalization.
    pub pending: Vec<u64>,
    /// Number of finalizers run.
    pub finalized_count: u64,
}
impl FinalizerQueue {
    /// Create a new empty finalizer queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue an object for finalization.
    pub fn enqueue(&mut self, id: u64) {
        self.pending.push(id);
    }
    /// Dequeue and "run" the next finalizer.
    /// Returns the object ID or `None` if the queue is empty.
    pub fn drain_one(&mut self) -> Option<u64> {
        if self.pending.is_empty() {
            None
        } else {
            let id = self.pending.remove(0);
            self.finalized_count += 1;
            Some(id)
        }
    }
    /// Drain all pending finalizers, returning how many were run.
    pub fn drain_all(&mut self) -> usize {
        let count = self.pending.len();
        self.pending.clear();
        self.finalized_count += count as u64;
        count
    }
    /// Number of objects pending finalization.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Whether any finalizers are pending.
    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }
}
/// Adaptively tunes GC parameters based on observed behavior.
#[derive(Clone, Debug)]
pub struct GcTuner {
    /// Recent pause times (ring buffer).
    recent_pauses: Vec<u64>,
    /// Target maximum pause time in nanoseconds.
    pub target_pause_ns: u64,
    /// Current collection threshold (fraction of heap used before triggering GC).
    pub collection_threshold: f64,
    /// Whether tuning is enabled.
    pub enabled: bool,
    /// Number of pause samples to track.
    pub window_size: usize,
}
impl GcTuner {
    /// Create a new tuner with default settings.
    pub fn new() -> Self {
        Self {
            recent_pauses: Vec::new(),
            target_pause_ns: 1_000_000,
            collection_threshold: 0.75,
            enabled: true,
            window_size: 20,
        }
    }
    /// Record an observed pause time.
    pub fn record_pause(&mut self, pause_ns: u64) {
        if self.recent_pauses.len() >= self.window_size {
            self.recent_pauses.remove(0);
        }
        self.recent_pauses.push(pause_ns);
        if self.enabled {
            self.tune();
        }
    }
    /// Adapt the collection threshold based on recent pauses.
    fn tune(&mut self) {
        if self.recent_pauses.is_empty() {
            return;
        }
        let avg: f64 =
            self.recent_pauses.iter().sum::<u64>() as f64 / self.recent_pauses.len() as f64;
        if avg > self.target_pause_ns as f64 * 1.5 {
            self.collection_threshold = (self.collection_threshold - 0.05).max(0.5);
        } else if avg < self.target_pause_ns as f64 * 0.5 {
            self.collection_threshold = (self.collection_threshold + 0.05).min(0.95);
        }
    }
    /// Current average pause time.
    pub fn avg_pause_ns(&self) -> f64 {
        if self.recent_pauses.is_empty() {
            0.0
        } else {
            self.recent_pauses.iter().sum::<u64>() as f64 / self.recent_pauses.len() as f64
        }
    }
    /// Whether the tuner has enough data.
    pub fn has_data(&self) -> bool {
        self.recent_pauses.len() >= 3
    }
}
/// Copying (semispace) garbage collector.
pub struct SemispaceGc {
    pub from_space: Vec<u8>,
    pub to_space: Vec<u8>,
    pub free_ptr: usize,
    pub stats: GcStats,
}
impl SemispaceGc {
    pub fn new(space_size: usize) -> Self {
        Self {
            from_space: vec![0u8; space_size],
            to_space: vec![0u8; space_size],
            free_ptr: 0,
            stats: GcStats::new(),
        }
    }
    /// Allocate `size` bytes in from-space; returns offset or `None`.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.free_ptr + size > self.from_space.len() {
            return None;
        }
        let offset = self.free_ptr;
        self.free_ptr += size;
        self.stats.heap_size = self.free_ptr;
        Some(offset)
    }
    /// Flip spaces; simulates copying live data (here: copies all of from_space).
    /// Returns the number of bytes that survived (= free_ptr before flip).
    pub fn flip(&mut self) -> usize {
        let surviving = self.free_ptr;
        let len = surviving.min(self.to_space.len());
        self.to_space[..len].copy_from_slice(&self.from_space[..len]);
        std::mem::swap(&mut self.from_space, &mut self.to_space);
        for b in self.to_space.iter_mut() {
            *b = 0;
        }
        let freed = (self.from_space.len().saturating_sub(surviving)) as u64;
        self.stats.record_collection(freed, 0);
        self.free_ptr = surviving;
        self.stats.heap_size = self.free_ptr;
        surviving
    }
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }
}
/// A managed set of GC roots.
///
/// Roots are heap addresses that are reachable from the mutator stack,
/// global variables, or external handles.
#[derive(Clone, Debug, Default)]
pub struct GcRootSet {
    /// Root addresses (indices into the heap).
    roots: Vec<usize>,
    /// Named roots (for debugging).
    named: Vec<(String, usize)>,
}
impl GcRootSet {
    /// Create an empty root set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an unnamed root.
    pub fn add(&mut self, addr: usize) {
        if !self.roots.contains(&addr) {
            self.roots.push(addr);
        }
    }
    /// Add a named root for debugging.
    pub fn add_named(&mut self, name: impl Into<String>, addr: usize) {
        let name = name.into();
        self.named.retain(|(n, _)| n != &name);
        self.named.push((name, addr));
        self.add(addr);
    }
    /// Remove a root.
    pub fn remove(&mut self, addr: usize) {
        self.roots.retain(|&r| r != addr);
        self.named.retain(|(_, a)| *a != addr);
    }
    /// Get all root addresses.
    pub fn all_roots(&self) -> &[usize] {
        &self.roots
    }
    /// Lookup a named root.
    pub fn lookup_named(&self, name: &str) -> Option<usize> {
        self.named.iter().find(|(n, _)| n == name).map(|(_, a)| *a)
    }
    /// Number of roots.
    pub fn len(&self) -> usize {
        self.roots.len()
    }
    /// Whether the root set is empty.
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
    /// Clear all roots.
    pub fn clear(&mut self) {
        self.roots.clear();
        self.named.clear();
    }
}
/// A region-based garbage collector (G1-inspired).
#[derive(Clone, Debug)]
pub struct RegionBasedGc {
    /// All regions.
    pub regions: Vec<GcRegion>,
    /// Size of each region.
    pub region_size: usize,
    /// Index of the current allocation region.
    pub current_region: usize,
    /// Statistics.
    pub stats: GcStats,
}
impl RegionBasedGc {
    /// Create a new region-based GC with `num_regions` regions.
    pub fn new(num_regions: usize, region_size: usize) -> Self {
        let regions = (0..num_regions)
            .map(|i| GcRegion::new(i, region_size))
            .collect();
        Self {
            regions,
            region_size,
            current_region: 0,
            stats: GcStats::new(),
        }
    }
    /// Allocate `size` bytes; finds a region with enough space.
    pub fn allocate(&mut self, size: usize) -> Option<(usize, usize)> {
        for start in self.current_region..self.regions.len() {
            if let Some(off) = self.regions[start].allocate(size) {
                self.stats.heap_size += size;
                return Some((start, off));
            }
        }
        None
    }
    /// Select collection-set: regions with lowest liveness.
    pub fn select_cset(&mut self, max_regions: usize) {
        for r in self.regions.iter_mut() {
            r.in_cset = false;
        }
        let mut candidates: Vec<usize> = (0..self.regions.len())
            .filter(|&i| !self.regions[i].is_empty())
            .collect();
        candidates.sort_by_key(|&i| self.regions[i].liveness_pct);
        for &idx in candidates.iter().take(max_regions) {
            self.regions[idx].in_cset = true;
        }
    }
    /// Collect the current collection set; returns total bytes freed.
    pub fn collect_cset(&mut self) -> usize {
        let mut freed = 0usize;
        for region in self.regions.iter_mut() {
            if region.in_cset {
                let dead = region.used * (100 - region.liveness_pct as usize) / 100;
                freed += dead;
                region.used = region.used.saturating_sub(dead);
                self.stats.heap_size = self.stats.heap_size.saturating_sub(dead);
                region.in_cset = false;
            }
        }
        self.stats.record_collection(freed as u64, 0);
        freed
    }
    /// Total used bytes across all regions.
    pub fn total_used(&self) -> usize {
        self.regions.iter().map(|r| r.used).sum()
    }
    /// Total free bytes.
    pub fn total_free(&self) -> usize {
        self.regions.iter().map(|r| r.free()).sum()
    }
    /// Number of empty regions.
    pub fn empty_region_count(&self) -> usize {
        self.regions.iter().filter(|r| r.is_empty()).count()
    }
    /// Number of full regions.
    pub fn full_region_count(&self) -> usize {
        self.regions.iter().filter(|r| r.is_nearly_full()).count()
    }
    /// Average liveness across all non-empty regions.
    pub fn avg_liveness(&self) -> f64 {
        let non_empty: Vec<&GcRegion> = self.regions.iter().filter(|r| !r.is_empty()).collect();
        if non_empty.is_empty() {
            return 100.0;
        }
        non_empty.iter().map(|r| r.liveness_pct as f64).sum::<f64>() / non_empty.len() as f64
    }
}
/// Incremental tri-color mark-and-sweep garbage collector.
///
/// This collector interleaves marking work with mutator work to reduce pause
/// times. Call `step_mark()` repeatedly between mutator steps, then call
/// `sweep()` when the gray set is empty.
pub struct IncrementalGc {
    /// All heap cells indexed by their ID.
    pub cells: Vec<HeapCell>,
    /// Indices of root objects (always alive).
    pub roots: Vec<usize>,
    /// The gray work-list.
    pub gray_set: Vec<usize>,
    /// Statistics.
    pub stats: GcStats,
    /// Whether a collection cycle is in progress.
    pub collecting: bool,
    /// Number of mark steps per cycle.
    pub steps_per_cycle: usize,
}
impl IncrementalGc {
    /// Create a new incremental GC.
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            roots: Vec::new(),
            gray_set: Vec::new(),
            stats: GcStats::new(),
            collecting: false,
            steps_per_cycle: 10,
        }
    }
    /// Allocate a new heap cell; returns its index.
    pub fn allocate(&mut self, size: usize) -> usize {
        let idx = self.cells.len();
        self.cells.push(HeapCell::new(size));
        self.stats.heap_size += size;
        idx
    }
    /// Add an object to the root set.
    pub fn add_root(&mut self, idx: usize) {
        if !self.roots.contains(&idx) {
            self.roots.push(idx);
        }
    }
    /// Remove an object from the root set.
    pub fn remove_root(&mut self, idx: usize) {
        self.roots.retain(|&r| r != idx);
    }
    /// Add a reference from `from` to `to` (write barrier).
    pub fn write_barrier(&mut self, from: usize, to: usize) {
        if from < self.cells.len() {
            self.cells[from].add_child(to);
            if self.cells[from].color == TriColor::Black
                && to < self.cells.len()
                && self.cells[to].color == TriColor::White
            {
                self.cells[from].color = TriColor::Gray;
                if !self.gray_set.contains(&from) {
                    self.gray_set.push(from);
                }
            }
        }
    }
    /// Begin a collection cycle by greying all roots.
    pub fn begin_collection(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.color = TriColor::White;
        }
        self.gray_set.clear();
        for &root in &self.roots.clone() {
            if root < self.cells.len() {
                self.cells[root].color = TriColor::Gray;
                self.gray_set.push(root);
            }
        }
        self.collecting = true;
    }
    /// Perform one incremental marking step. Returns `true` if the gray set is empty.
    pub fn step_mark(&mut self) -> bool {
        if let Some(idx) = self.gray_set.pop() {
            if idx < self.cells.len() {
                self.cells[idx].color = TriColor::Black;
                let children: Vec<usize> = self.cells[idx].children.clone();
                for child in children {
                    if child < self.cells.len() && self.cells[child].color == TriColor::White {
                        self.cells[child].color = TriColor::Gray;
                        self.gray_set.push(child);
                    }
                }
            }
        }
        self.gray_set.is_empty()
    }
    /// Run all pending mark steps at once.
    pub fn mark_all(&mut self) {
        while !self.step_mark() {}
    }
    /// Sweep phase: reclaim all white (unreachable) cells.
    pub fn sweep(&mut self) -> usize {
        let mut freed = 0usize;
        for cell in self.cells.iter_mut() {
            if cell.color == TriColor::White && cell.live {
                freed += cell.size;
                self.stats.heap_size = self.stats.heap_size.saturating_sub(cell.size);
                cell.live = false;
            }
        }
        self.stats.record_collection(freed as u64, 0);
        self.collecting = false;
        freed
    }
    /// Run a complete collection (begin + mark_all + sweep).
    pub fn collect(&mut self) -> usize {
        self.begin_collection();
        self.mark_all();
        self.sweep()
    }
    /// Count live cells.
    pub fn live_count(&self) -> usize {
        self.cells.iter().filter(|c| c.live).count()
    }
    /// Total bytes in live cells.
    pub fn live_bytes(&self) -> usize {
        self.cells.iter().filter(|c| c.live).map(|c| c.size).sum()
    }
    /// Number of cells in the gray work-list.
    pub fn gray_count(&self) -> usize {
        self.gray_set.len()
    }
}
/// A card table for tracking cross-generation pointers.
///
/// The heap is divided into cards (fixed-size blocks). When a card contains
/// a pointer from old to young generation, the card is marked "dirty".
#[derive(Clone, Debug)]
pub struct CardTable {
    /// Dirty bits for each card.
    pub cards: Vec<bool>,
    /// Size of each card in bytes.
    pub card_size: usize,
    /// Total heap size covered.
    pub heap_size: usize,
}
impl CardTable {
    /// Create a new card table for the given heap size and card size.
    pub fn new(heap_size: usize, card_size: usize) -> Self {
        let num_cards = (heap_size + card_size - 1) / card_size;
        Self {
            cards: vec![false; num_cards],
            card_size,
            heap_size,
        }
    }
    /// Mark the card containing `addr` as dirty.
    pub fn mark_dirty(&mut self, addr: usize) {
        let card = addr / self.card_size;
        if card < self.cards.len() {
            self.cards[card] = true;
        }
    }
    /// Clear all dirty bits.
    pub fn clear(&mut self) {
        for bit in self.cards.iter_mut() {
            *bit = false;
        }
    }
    /// Get all dirty card indices.
    pub fn dirty_cards(&self) -> Vec<usize> {
        self.cards
            .iter()
            .enumerate()
            .filter(|(_, &dirty)| dirty)
            .map(|(i, _)| i)
            .collect()
    }
    /// Number of dirty cards.
    pub fn dirty_count(&self) -> usize {
        self.cards.iter().filter(|&&b| b).count()
    }
    /// Number of cards total.
    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }
    /// Whether a given address is in a dirty card.
    pub fn is_dirty(&self, addr: usize) -> bool {
        let card = addr / self.card_size;
        card < self.cards.len() && self.cards[card]
    }
    /// The address range covered by card `idx`.
    pub fn card_range(&self, idx: usize) -> (usize, usize) {
        let start = idx * self.card_size;
        let end = (start + self.card_size).min(self.heap_size);
        (start, end)
    }
}
/// A simulated heap object for tri-color marking.
#[derive(Clone, Debug)]
pub struct HeapCell {
    /// Color of this cell.
    pub color: TriColor,
    /// Indices of objects this cell references (children).
    pub children: Vec<usize>,
    /// Whether this cell is live data (not freed).
    pub live: bool,
    /// Size of this cell in bytes.
    pub size: usize,
}
impl HeapCell {
    /// Create a new heap cell with no children.
    pub fn new(size: usize) -> Self {
        Self {
            color: TriColor::White,
            children: Vec::new(),
            live: true,
            size,
        }
    }
    /// Add a reference to another object.
    pub fn add_child(&mut self, idx: usize) {
        if !self.children.contains(&idx) {
            self.children.push(idx);
        }
    }
}
/// Sticky mark-bits collector variant.
///
/// In this scheme mark bits are NOT cleared after a collection.
/// Objects that survived the previous collection are assumed live and
/// their bits are left set ("sticky"). Only newly-allocated objects
/// start with clear bits and must survive a cycle to become sticky.
#[derive(Clone, Debug, Default)]
pub struct StickyMarkBitsGc {
    /// Mark bits: true = sticky (survived ≥1 collection).
    pub bits: Vec<bool>,
    /// Allocation watermark (bytes allocated since last collection).
    pub watermark: usize,
    /// Statistics.
    pub stats: GcStats,
    /// Total capacity.
    pub capacity: usize,
}
impl StickyMarkBitsGc {
    /// Create a new sticky-mark-bits GC.
    pub fn new(capacity: usize) -> Self {
        Self {
            bits: vec![false; capacity],
            watermark: 0,
            stats: GcStats::new(),
            capacity,
        }
    }
    /// Allocate `size` bytes; returns offset or `None` if out of space.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let offset = self.watermark;
        if offset + size > self.capacity {
            return None;
        }
        self.watermark += size;
        self.stats.heap_size = self.watermark;
        Some(offset)
    }
    /// Mark object at `addr` as sticky (survived).
    pub fn mark_sticky(&mut self, addr: usize) {
        if addr < self.bits.len() {
            self.bits[addr] = true;
        }
    }
    /// Collect: clear non-sticky bits, reset watermark to sticky region.
    pub fn collect(&mut self) -> usize {
        let mut last_sticky = 0usize;
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                last_sticky = i + 1;
            }
        }
        let freed = self.watermark.saturating_sub(last_sticky);
        self.watermark = last_sticky;
        self.stats.heap_size = self.watermark;
        self.stats.record_collection(freed as u64, 0);
        freed
    }
    /// Promote all marked objects (make all bits sticky).
    pub fn promote_all(&mut self) {
        for bit in self.bits.iter_mut() {
            *bit = true;
        }
    }
    /// Clear all sticky bits (full reset).
    pub fn reset_bits(&mut self) {
        for bit in self.bits.iter_mut() {
            *bit = false;
        }
    }
    /// Number of sticky bytes.
    pub fn sticky_bytes(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }
}
/// Strategy for heap compaction after a collection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompactionStrategy {
    /// Never compact the heap.
    Never,
    /// Compact after every collection.
    Always,
    /// Compact when fragmentation exceeds a threshold.
    OnFragmentation,
    /// Compact only on major collections.
    MajorOnly,
}
impl CompactionStrategy {
    /// Name of this strategy.
    pub fn name(self) -> &'static str {
        match self {
            CompactionStrategy::Never => "never",
            CompactionStrategy::Always => "always",
            CompactionStrategy::OnFragmentation => "on-fragmentation",
            CompactionStrategy::MajorOnly => "major-only",
        }
    }
    /// Whether compaction is enabled at all.
    pub fn is_enabled(self) -> bool {
        !matches!(self, CompactionStrategy::Never)
    }
}
/// Color of a GC object in the tri-color invariant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriColor {
    /// White: not yet discovered; will be reclaimed if still white at end.
    White,
    /// Gray: discovered but children not yet traced.
    Gray,
    /// Black: fully traced; children also reachable.
    Black,
}
/// Configuration for a garbage collector.
#[derive(Clone, Debug)]
pub struct GcConfig {
    /// Which GC algorithm to use.
    pub strategy: GcStrategy,
    /// Maximum heap size in bytes.
    pub heap_limit: usize,
    /// Trigger collection when heap usage exceeds this fraction.
    pub collection_threshold: f64,
    /// Number of incremental steps per GC cycle.
    pub incremental_steps: usize,
    /// Whether to enable write barriers.
    pub write_barriers: bool,
    /// Young generation size (for generational GC).
    pub young_gen_size: usize,
    /// Old generation size (for generational GC).
    pub old_gen_size: usize,
}
impl GcConfig {
    /// Create a new GC configuration with the given strategy.
    pub fn new(strategy: GcStrategy) -> Self {
        Self {
            strategy,
            ..Default::default()
        }
    }
    /// Set the heap limit.
    pub fn with_heap_limit(mut self, bytes: usize) -> Self {
        self.heap_limit = bytes;
        self
    }
    /// Set the collection threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.collection_threshold = threshold.clamp(0.1, 1.0);
        self
    }
    /// Set the number of incremental steps.
    pub fn with_incremental_steps(mut self, steps: usize) -> Self {
        self.incremental_steps = steps;
        self
    }
    /// Enable or disable write barriers.
    pub fn with_write_barriers(mut self, enabled: bool) -> Self {
        self.write_barriers = enabled;
        self
    }
    /// Validate the configuration, returning a list of issues.
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();
        if self.heap_limit == 0 {
            issues.push("heap_limit must be > 0".to_string());
        }
        if self.collection_threshold <= 0.0 || self.collection_threshold > 1.0 {
            issues.push("collection_threshold must be in (0.0, 1.0]".to_string());
        }
        if self.incremental_steps == 0 {
            issues.push("incremental_steps must be > 0".to_string());
        }
        if self.young_gen_size >= self.old_gen_size {
            issues.push("young_gen_size must be < old_gen_size".to_string());
        }
        issues
    }
}
/// Which GC is currently active.
pub enum ActiveGc {
    /// Mark-sweep GC.
    MarkSweep(MarkSweepGc),
    /// Semispace GC.
    Semispace(SemispaceGc),
    /// Generational GC.
    Generational(GenerationalGc),
    /// Incremental GC.
    Incremental(IncrementalGc),
}
/// Measures heap fragmentation.
#[derive(Clone, Debug, Default)]
pub struct HeapFragmentation {
    /// Total heap capacity in bytes.
    pub total_capacity: usize,
    /// Total live bytes.
    pub live_bytes: usize,
    /// Largest contiguous free block in bytes.
    pub largest_free_block: usize,
    /// Total number of free blocks.
    pub free_block_count: usize,
}
impl HeapFragmentation {
    /// Create a new fragmentation measurement.
    pub fn new(total_capacity: usize, live_bytes: usize) -> Self {
        Self {
            total_capacity,
            live_bytes,
            largest_free_block: 0,
            free_block_count: 0,
        }
    }
    /// Compute the fragmentation ratio (0.0 = no fragmentation, 1.0 = fully fragmented).
    pub fn ratio(&self) -> f64 {
        let free = self.total_capacity.saturating_sub(self.live_bytes);
        if free == 0 || self.largest_free_block == 0 {
            return 0.0;
        }
        1.0 - (self.largest_free_block as f64 / free as f64)
    }
    /// Whether fragmentation is high (ratio > 0.5).
    pub fn is_high(&self) -> bool {
        self.ratio() > 0.5
    }
    /// Free bytes.
    pub fn free_bytes(&self) -> usize {
        self.total_capacity.saturating_sub(self.live_bytes)
    }
    /// Utilization fraction (live / total).
    pub fn utilization(&self) -> f64 {
        if self.total_capacity == 0 {
            0.0
        } else {
            self.live_bytes as f64 / self.total_capacity as f64
        }
    }
}
/// A single region in a region-based GC.
#[derive(Clone, Debug)]
pub struct GcRegion {
    /// Region index.
    pub id: usize,
    /// Bytes allocated in this region.
    pub used: usize,
    /// Total capacity of this region.
    pub capacity: usize,
    /// Whether this region is in the collection set for the current cycle.
    pub in_cset: bool,
    /// Humongous flag: region holds a single large object.
    pub humongous: bool,
    /// Liveness percentage (0–100).
    pub liveness_pct: u8,
}
impl GcRegion {
    /// Create a new region.
    pub fn new(id: usize, capacity: usize) -> Self {
        Self {
            id,
            used: 0,
            capacity,
            in_cset: false,
            humongous: false,
            liveness_pct: 100,
        }
    }
    /// Free bytes remaining in this region.
    pub fn free(&self) -> usize {
        self.capacity.saturating_sub(self.used)
    }
    /// Try to allocate `size` bytes; returns offset or `None`.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.used + size > self.capacity {
            return None;
        }
        let off = self.used;
        self.used += size;
        Some(off)
    }
    /// Mark this region as a candidate for collection.
    pub fn add_to_cset(&mut self) {
        self.in_cset = true;
    }
    /// Update liveness estimate.
    pub fn set_liveness(&mut self, pct: u8) {
        self.liveness_pct = pct.min(100);
    }
    /// Whether this region is nearly full (>90% used).
    pub fn is_nearly_full(&self) -> bool {
        self.used * 10 >= self.capacity * 9
    }
    /// Whether this region is completely empty.
    pub fn is_empty(&self) -> bool {
        self.used == 0
    }
}
/// Tracks the age of objects (in GC cycles) for generational analysis.
#[derive(Clone, Debug, Default)]
pub struct ObjectAgeTable {
    /// Map from object ID to age (in collection cycles).
    pub ages: std::collections::HashMap<u64, u32>,
    /// Total objects promoted (age exceeded threshold).
    pub promoted_count: u64,
    /// Promotion age threshold.
    pub promotion_threshold: u32,
}
impl ObjectAgeTable {
    /// Create a new age table.
    pub fn new(promotion_threshold: u32) -> Self {
        Self {
            ages: std::collections::HashMap::new(),
            promoted_count: 0,
            promotion_threshold,
        }
    }
    /// Register a new object.
    pub fn register(&mut self, id: u64) {
        self.ages.insert(id, 0);
    }
    /// Increment age of an object. Returns `true` if it should be promoted.
    pub fn age_object(&mut self, id: u64) -> bool {
        if let Some(age) = self.ages.get_mut(&id) {
            *age += 1;
            if *age >= self.promotion_threshold {
                self.promoted_count += 1;
                return true;
            }
        }
        false
    }
    /// Remove an object from the table.
    pub fn remove(&mut self, id: u64) {
        self.ages.remove(&id);
    }
    /// Get the age of an object.
    pub fn age_of(&self, id: u64) -> Option<u32> {
        self.ages.get(&id).copied()
    }
    /// Number of tracked objects.
    pub fn len(&self) -> usize {
        self.ages.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.ages.is_empty()
    }
    /// Average age of tracked objects.
    pub fn avg_age(&self) -> f64 {
        if self.ages.is_empty() {
            0.0
        } else {
            self.ages.values().sum::<u32>() as f64 / self.ages.len() as f64
        }
    }
    /// Objects older than the promotion threshold.
    pub fn old_objects(&self) -> Vec<u64> {
        self.ages
            .iter()
            .filter(|(_, &age)| age >= self.promotion_threshold)
            .map(|(&id, _)| id)
            .collect()
    }
}
/// The GC algorithm variant in use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GcStrategy {
    RefCounting,
    MarkSweep,
    Semispace,
    Generational,
    Incremental,
}
impl GcStrategy {
    pub fn name(&self) -> &str {
        match self {
            GcStrategy::RefCounting => "RefCounting",
            GcStrategy::MarkSweep => "MarkSweep",
            GcStrategy::Semispace => "Semispace",
            GcStrategy::Generational => "Generational",
            GcStrategy::Incremental => "Incremental",
        }
    }
    pub fn description(&self) -> &str {
        match self {
            GcStrategy::RefCounting => "Reference counting with cycle detection",
            GcStrategy::MarkSweep => "Classic mark-and-sweep stop-the-world collector",
            GcStrategy::Semispace => "Copying collector using two semispaces",
            GcStrategy::Generational => "Generational collector with young/old generations",
            GcStrategy::Incremental => "Incremental tri-color mark-and-sweep collector",
        }
    }
    pub fn is_concurrent(&self) -> bool {
        matches!(self, GcStrategy::Incremental)
    }
}
/// Generational garbage collector combining a young (semispace) and old (mark-sweep) generation.
pub struct GenerationalGc {
    pub young: SemispaceGc,
    pub old: MarkSweepGc,
    pub promotion_threshold: usize,
}
impl GenerationalGc {
    pub fn new() -> Self {
        Self {
            young: SemispaceGc::new(64 * 1024),
            old: MarkSweepGc::new(512 * 1024),
            promotion_threshold: 2,
        }
    }
    /// Collect the young generation; returns bytes freed.
    pub fn minor_gc(&mut self) -> usize {
        let surviving = self.young.flip();
        if surviving > 0 {
            if let Some(off) = self.old.allocate(surviving) {
                let len = surviving.min(self.young.from_space.len());
                let src: Vec<u8> = self.young.from_space[..len].to_vec();
                let dst_len = self.old.heap.len();
                let copy_len = len.min(dst_len.saturating_sub(off));
                self.old.heap[off..off + copy_len].copy_from_slice(&src[..copy_len]);
            }
        }
        self.young.stats.bytes_collected as usize
    }
    /// Collect the old generation; returns bytes freed.
    pub fn major_gc(&mut self) -> usize {
        self.old.collect()
    }
    /// Human-readable statistics report.
    pub fn stats_report(&self) -> String {
        format!(
            "Young: {} collections, {} bytes collected | Old: {} collections, {} bytes collected",
            self.young.stats.collections,
            self.young.stats.bytes_collected,
            self.old.stats.collections,
            self.old.stats.bytes_collected,
        )
    }
}
/// The current phase of a garbage collection cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcPhase {
    /// Mutator is running; GC is idle.
    Idle,
    /// Initial mark phase (stop-the-world, brief).
    InitialMark,
    /// Concurrent mark phase (runs alongside mutator).
    ConcurrentMark,
    /// Final remark (stop-the-world, brief).
    Remark,
    /// Sweep phase.
    Sweep,
    /// Compaction phase.
    Compact,
    /// Finalization phase.
    Finalize,
}
impl GcPhase {
    /// Whether this phase requires stopping the mutator.
    pub fn is_stop_the_world(self) -> bool {
        matches!(
            self,
            GcPhase::InitialMark | GcPhase::Remark | GcPhase::Compact
        )
    }
    /// Name of this phase.
    pub fn name(self) -> &'static str {
        match self {
            GcPhase::Idle => "idle",
            GcPhase::InitialMark => "initial-mark",
            GcPhase::ConcurrentMark => "concurrent-mark",
            GcPhase::Remark => "remark",
            GcPhase::Sweep => "sweep",
            GcPhase::Compact => "compact",
            GcPhase::Finalize => "finalize",
        }
    }
}
/// A simple benchmark harness for comparing GC strategies.
#[derive(Clone, Debug, Default)]
pub struct GcBenchmark {
    /// Results indexed by strategy name.
    pub results: std::collections::HashMap<String, GcBenchmarkResult>,
}
impl GcBenchmark {
    /// Create a new benchmark harness.
    pub fn new() -> Self {
        Self::default()
    }
    /// Run a mark-sweep benchmark with the given number of allocations.
    pub fn run_mark_sweep(&mut self, num_allocs: usize, alloc_size: usize) {
        let mut gc = MarkSweepGc::new(num_allocs * alloc_size * 2);
        let mut total_freed = 0usize;
        for i in 0..num_allocs {
            if gc.allocate(alloc_size).is_none() {
                total_freed += gc.collect();
                let _ = gc.allocate(alloc_size);
            }
            if i % 2 == 0 {
                let start = i * alloc_size;
                if start < gc.mark_bits.len() {
                    gc.mark(start);
                }
            }
        }
        let result = GcBenchmarkResult {
            strategy: "MarkSweep".to_string(),
            total_allocated: num_allocs * alloc_size,
            total_freed,
            collections: gc.stats.collections,
            alloc_time_ns: 0,
            gc_time_ns: gc.stats.pause_time_ns,
        };
        self.results.insert("MarkSweep".to_string(), result);
    }
    /// Run a semispace benchmark.
    pub fn run_semispace(&mut self, num_allocs: usize, alloc_size: usize) {
        let mut gc = SemispaceGc::new(num_allocs * alloc_size);
        let mut total_freed = 0usize;
        for _ in 0..num_allocs {
            if gc.allocate(alloc_size).is_none() {
                let surviving = gc.flip();
                total_freed += gc.from_space.len().saturating_sub(surviving);
            }
        }
        let result = GcBenchmarkResult {
            strategy: "Semispace".to_string(),
            total_allocated: num_allocs * alloc_size,
            total_freed,
            collections: gc.stats.collections,
            alloc_time_ns: 0,
            gc_time_ns: gc.stats.pause_time_ns,
        };
        self.results.insert("Semispace".to_string(), result);
    }
    /// Print a comparison table.
    pub fn print_comparison(&self) -> String {
        let mut lines = vec!["=== GC Benchmark Comparison ===".to_string()];
        for (name, result) in &self.results {
            lines.push(format!(
                "{}: allocated={}, freed={}, collections={}",
                name, result.total_allocated, result.total_freed, result.collections
            ));
        }
        lines.join("\n")
    }
}
/// Simple mark-and-sweep garbage collector.
pub struct MarkSweepGc {
    pub heap: Vec<u8>,
    pub mark_bits: Vec<bool>,
    pub stats: GcStats,
    pub heap_limit: usize,
}
impl MarkSweepGc {
    pub fn new(heap_limit: usize) -> Self {
        Self {
            heap: Vec::new(),
            mark_bits: Vec::new(),
            stats: GcStats::new(),
            heap_limit,
        }
    }
    /// Allocate `size` bytes; returns the starting offset or `None` if out of memory.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let offset = self.heap.len();
        if offset + size > self.heap_limit {
            return None;
        }
        self.heap.extend(std::iter::repeat(0u8).take(size));
        self.mark_bits.extend(std::iter::repeat(false).take(size));
        self.stats.heap_size = self.heap.len();
        Some(offset)
    }
    /// Mark the byte at `addr` as live.
    pub fn mark(&mut self, addr: usize) {
        if addr < self.mark_bits.len() {
            self.mark_bits[addr] = true;
        }
    }
    /// Sweep unmarked bytes, returning the number of bytes freed.
    pub fn sweep(&mut self) -> usize {
        let mut freed = 0usize;
        for (bit, byte) in self.mark_bits.iter_mut().zip(self.heap.iter_mut()) {
            if !*bit {
                *byte = 0;
                freed += 1;
            }
            *bit = false;
        }
        self.stats.heap_size = self.heap.len();
        freed
    }
    /// Run a full collection cycle (mark roots, then sweep).
    pub fn collect(&mut self) -> usize {
        let freed = self.sweep() as u64;
        self.stats.record_collection(freed, 0);
        freed as usize
    }
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }
}
/// Records GC pause events.
#[derive(Clone, Debug, Default)]
pub struct GcPauseLog {
    /// List of `(start_ns, duration_ns)` pairs.
    pub pauses: Vec<(u64, u64)>,
}
impl GcPauseLog {
    /// Create an empty pause log.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a pause event.
    pub fn record(&mut self, start_ns: u64, duration_ns: u64) {
        self.pauses.push((start_ns, duration_ns));
    }
    /// Total pause time in nanoseconds.
    pub fn total_pause_ns(&self) -> u64 {
        self.pauses.iter().map(|(_, d)| d).sum()
    }
    /// Maximum pause time in nanoseconds.
    pub fn max_pause_ns(&self) -> u64 {
        self.pauses.iter().map(|(_, d)| *d).max().unwrap_or(0)
    }
    /// Average pause time in nanoseconds.
    pub fn avg_pause_ns(&self) -> f64 {
        if self.pauses.is_empty() {
            0.0
        } else {
            self.total_pause_ns() as f64 / self.pauses.len() as f64
        }
    }
    /// Number of pause events.
    pub fn count(&self) -> usize {
        self.pauses.len()
    }
    /// 99th percentile pause time (approximate).
    pub fn p99_pause_ns(&self) -> u64 {
        if self.pauses.is_empty() {
            return 0;
        }
        let mut durations: Vec<u64> = self.pauses.iter().map(|(_, d)| *d).collect();
        durations.sort_unstable();
        let idx = ((durations.len() as f64 * 0.99) as usize).min(durations.len() - 1);
        durations[idx]
    }
}
/// A handle to whichever GC strategy is configured.
pub struct GcHandle {
    /// The active GC implementation.
    pub gc: ActiveGc,
    /// The configuration.
    pub config: GcConfig,
}
impl GcHandle {
    /// Create a new GC handle based on the config.
    pub fn from_config(config: GcConfig) -> Self {
        let gc = match config.strategy {
            GcStrategy::RefCounting | GcStrategy::MarkSweep => {
                ActiveGc::MarkSweep(MarkSweepGc::new(config.heap_limit))
            }
            GcStrategy::Semispace => ActiveGc::Semispace(SemispaceGc::new(config.heap_limit / 2)),
            GcStrategy::Generational => ActiveGc::Generational(GenerationalGc::new()),
            GcStrategy::Incremental => ActiveGc::Incremental(IncrementalGc::new()),
        };
        Self { gc, config }
    }
    /// Get the current heap size.
    pub fn heap_size(&self) -> usize {
        match &self.gc {
            ActiveGc::MarkSweep(g) => g.stats.heap_size,
            ActiveGc::Semispace(g) => g.stats.heap_size,
            ActiveGc::Generational(g) => g.young.stats.heap_size + g.old.stats.heap_size,
            ActiveGc::Incremental(g) => g.stats.heap_size,
        }
    }
    /// Whether collection should be triggered now.
    pub fn should_collect(&self) -> bool {
        let used = self.heap_size();
        let limit = self.config.heap_limit;
        if limit == 0 {
            return false;
        }
        used as f64 / limit as f64 >= self.config.collection_threshold
    }
    /// Get the strategy name.
    pub fn strategy_name(&self) -> &str {
        self.config.strategy.name()
    }
    /// Get combined stats.
    pub fn stats(&self) -> GcStats {
        match &self.gc {
            ActiveGc::MarkSweep(g) => g.stats.clone(),
            ActiveGc::Semispace(g) => g.stats.clone(),
            ActiveGc::Generational(g) => {
                let mut s = g.young.stats.clone();
                s.collections += g.old.stats.collections;
                s.bytes_collected += g.old.stats.bytes_collected;
                s.pause_time_ns += g.old.stats.pause_time_ns;
                s
            }
            ActiveGc::Incremental(g) => g.stats.clone(),
        }
    }
}
/// Tracks write barrier actions for incremental collectors.
pub struct WriteBarrier {
    /// Log of pending barrier actions.
    pub log: Vec<(usize, usize, BarrierAction)>,
    /// Whether the barrier is active.
    pub active: bool,
}
impl WriteBarrier {
    /// Create a new write barrier.
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            active: false,
        }
    }
    /// Activate the barrier.
    pub fn activate(&mut self) {
        self.active = true;
    }
    /// Deactivate the barrier.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    /// Record a pointer update from `src` to `dst`.
    pub fn record(&mut self, src: usize, dst: usize, action: BarrierAction) {
        if self.active {
            self.log.push((src, dst, action));
        }
    }
    /// Drain and return all pending barrier actions.
    pub fn drain(&mut self) -> Vec<(usize, usize, BarrierAction)> {
        std::mem::take(&mut self.log)
    }
    /// Number of pending actions.
    pub fn pending_count(&self) -> usize {
        self.log.len()
    }
}
/// A cooperative GC safe-point mechanism.
///
/// Threads check in at safe-points to allow GC to run without
/// stopping threads at arbitrary points.
#[derive(Clone, Debug, Default)]
pub struct GcSafePoint {
    /// Whether a GC stop is requested.
    pub stop_requested: bool,
    /// Number of threads at a safe-point.
    pub threads_at_safepoint: u32,
    /// Total number of registered threads.
    pub total_threads: u32,
}
impl GcSafePoint {
    /// Create a new safe-point manager.
    pub fn new(total_threads: u32) -> Self {
        Self {
            stop_requested: false,
            threads_at_safepoint: 0,
            total_threads,
        }
    }
    /// Request all threads to stop at their next safe-point.
    pub fn request_stop(&mut self) {
        self.stop_requested = true;
    }
    /// A thread signals it is at a safe-point.
    pub fn thread_at_safepoint(&mut self) {
        if self.threads_at_safepoint < self.total_threads {
            self.threads_at_safepoint += 1;
        }
    }
    /// A thread exits a safe-point.
    pub fn thread_exit_safepoint(&mut self) {
        if self.threads_at_safepoint > 0 {
            self.threads_at_safepoint -= 1;
        }
    }
    /// Whether all threads are at a safe-point.
    pub fn all_stopped(&self) -> bool {
        self.stop_requested && self.threads_at_safepoint >= self.total_threads
    }
    /// Release all threads from the safe-point.
    pub fn release(&mut self) {
        self.stop_requested = false;
        self.threads_at_safepoint = 0;
    }
    /// Fraction of threads stopped.
    pub fn stopped_fraction(&self) -> f64 {
        if self.total_threads == 0 {
            0.0
        } else {
            self.threads_at_safepoint as f64 / self.total_threads as f64
        }
    }
}
/// Metadata stored alongside each heap object.
#[derive(Clone, Debug)]
pub struct GcObjectHeader {
    /// Unique object ID.
    pub id: u64,
    /// Size of the object in bytes.
    pub size: usize,
    /// Reference count (for ref-counted strategies).
    pub ref_count: u32,
    /// GC generation (0 = young, 1+ = old).
    pub generation: u8,
    /// Whether the object has been pinned (cannot be moved).
    pub pinned: bool,
    /// Whether this object has a finalizer.
    pub has_finalizer: bool,
    /// Mark bit for mark-sweep.
    pub marked: bool,
    /// Forwarding pointer (for copying GC), if any.
    pub forwarding: Option<u64>,
}
impl GcObjectHeader {
    /// Create a new object header.
    pub fn new(id: u64, size: usize) -> Self {
        Self {
            id,
            size,
            ref_count: 1,
            generation: 0,
            pinned: false,
            has_finalizer: false,
            marked: false,
            forwarding: None,
        }
    }
    /// Increment the reference count.
    pub fn inc_ref(&mut self) {
        self.ref_count = self.ref_count.saturating_add(1);
    }
    /// Decrement the reference count. Returns `true` if the count reaches zero.
    pub fn dec_ref(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }
    /// Mark this object as reachable.
    pub fn mark(&mut self) {
        self.marked = true;
    }
    /// Clear the mark bit.
    pub fn clear_mark(&mut self) {
        self.marked = false;
    }
    /// Promote this object to the next generation.
    pub fn promote(&mut self) {
        self.generation = self.generation.saturating_add(1);
    }
    /// Pin this object so it cannot be moved by a copying collector.
    pub fn pin(&mut self) {
        self.pinned = true;
    }
    /// Unpin this object.
    pub fn unpin(&mut self) {
        self.pinned = false;
    }
    /// Set a forwarding pointer (for copying GC).
    pub fn forward_to(&mut self, addr: u64) {
        self.forwarding = Some(addr);
    }
    /// Whether this object has been forwarded.
    pub fn is_forwarded(&self) -> bool {
        self.forwarding.is_some()
    }
}
