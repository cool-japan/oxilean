//! Region-based memory allocator types for bulk-freeable proof-checking data.

use std::marker::PhantomData;

/// Unique region identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionId(pub u32);

impl RegionId {
    /// Create a new RegionId from a raw u32 value.
    pub fn new(id: u32) -> Self {
        RegionId(id)
    }

    /// Get the raw u32 identifier.
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for RegionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "region#{}", self.0)
    }
}

/// A contiguous memory region backed by a `Vec<u8>`.
///
/// The region acts as a bump allocator within a fixed-capacity buffer.
/// Allocation advances `offset` by the requested (aligned) size.
#[derive(Debug)]
pub struct Region {
    /// Unique identifier for this region.
    pub id: RegionId,
    /// Raw backing storage.
    pub data: Vec<u8>,
    /// Current allocation watermark (next free byte index).
    pub offset: usize,
    /// Total capacity of `data` in bytes.
    pub capacity: usize,
}

impl Region {
    /// Create a new region with the given id and capacity.
    pub fn new(id: RegionId, capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Region {
            id,
            data: vec![0u8; capacity],
            offset: 0,
            capacity,
        }
    }

    /// Bytes still available for allocation.
    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.offset)
    }

    /// Bytes already allocated.
    pub fn used(&self) -> usize {
        self.offset
    }

    /// Whether this region has no active allocations.
    pub fn is_empty(&self) -> bool {
        self.offset == 0
    }

    /// Reset the region so all memory becomes available again.
    pub fn reset(&mut self) {
        self.offset = 0;
        // Zero the backing buffer so stale data cannot leak.
        for b in &mut self.data {
            *b = 0;
        }
    }
}

/// A lifetime-tied handle to a specific region.
///
/// The `'a` lifetime ensures the handle cannot outlive the allocator
/// that owns the underlying `Region`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionHandle<'a> {
    /// The region this handle refers to.
    pub id: RegionId,
    /// Ties the handle lifetime to the allocator borrow.
    pub phantom: PhantomData<&'a ()>,
}

impl<'a> RegionHandle<'a> {
    /// Create a handle for the given region id with an explicit lifetime.
    pub fn new(id: RegionId) -> Self {
        RegionHandle {
            id,
            phantom: PhantomData,
        }
    }

    /// Expose the underlying region id.
    pub fn region_id(self) -> RegionId {
        self.id
    }
}

impl<'a> std::fmt::Display for RegionHandle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle({})", self.id)
    }
}

/// Allocation statistics snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AllocStats {
    /// Total number of regions ever created (including recycled).
    pub regions_created: usize,
    /// Total number of regions freed (returned to the free list).
    pub regions_freed: usize,
    /// Total bytes allocated across all active regions.
    pub total_allocated: usize,
    /// Total capacity freed (bytes in freed regions).
    pub total_freed: usize,
    /// Peak in-use byte count observed.
    pub peak_usage: usize,
}

impl AllocStats {
    /// Net active regions (created minus freed).
    pub fn active_regions(&self) -> usize {
        self.regions_created.saturating_sub(self.regions_freed)
    }

    /// Efficiency ratio: bytes allocated vs total capacity allocated so far.
    pub fn utilization(&self, total_capacity: usize) -> f64 {
        if total_capacity == 0 {
            return 0.0;
        }
        self.total_allocated as f64 / total_capacity as f64
    }
}

impl std::fmt::Display for AllocStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AllocStats {{ created: {}, freed: {}, allocated: {}, peak: {} }}",
            self.regions_created, self.regions_freed, self.total_allocated, self.peak_usage
        )
    }
}

/// Configuration for a `RegionAllocator`.
#[derive(Clone, Debug)]
pub struct RegionConfig {
    /// Initial capacity (bytes) of each newly created region.
    pub initial_capacity: usize,
    /// Multiplicative growth factor when a region must be expanded.
    /// Values outside `(1.0, 16.0]` are clamped.
    pub growth_factor: f64,
    /// Maximum number of live regions the allocator will manage.
    pub max_regions: usize,
}

impl RegionConfig {
    /// Construct a configuration, clamping `growth_factor` to `(1.0, 16.0]`.
    pub fn new(initial_capacity: usize, growth_factor: f64, max_regions: usize) -> Self {
        let growth_factor = growth_factor.clamp(1.001, 16.0);
        RegionConfig {
            initial_capacity: initial_capacity.max(64),
            growth_factor,
            max_regions: max_regions.max(1),
        }
    }

    /// Return next capacity after growing by the configured factor.
    pub fn grow(&self, current: usize) -> usize {
        ((current as f64 * self.growth_factor) as usize).max(current + 64)
    }
}

impl Default for RegionConfig {
    fn default() -> Self {
        RegionConfig::new(4096, 2.0, 256)
    }
}

/// A region-based (arena) allocator that manages a pool of `Region`s.
///
/// Regions can be bulk-freed by calling `free_region`, which returns
/// them to an internal free list for reuse without individual deallocation.
pub struct RegionAllocator {
    /// All managed regions, indexed by `RegionId::raw()`.
    pub regions: Vec<Region>,
    /// Ids of regions that are free for reuse.
    pub free_list: Vec<RegionId>,
    /// Default capacity for newly created regions.
    pub default_capacity: usize,
    /// Allocator configuration.
    pub(super) config: RegionConfig,
    /// Running statistics.
    pub(super) stats: AllocStats,
    /// Next fresh id to assign (monotonically increasing).
    pub(super) next_id: u32,
    /// Current total bytes allocated across all active (non-freed) regions.
    pub(super) current_usage: usize,
}
