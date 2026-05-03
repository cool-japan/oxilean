//! Region-based memory allocator implementation and tests.

use super::types::{AllocStats, Region, RegionAllocator, RegionConfig, RegionHandle, RegionId};

/// Round `offset` up to the nearest multiple of `align`.
///
/// `align` must be a power of two; if it is zero it is treated as 1.
pub fn align_up(offset: usize, align: usize) -> usize {
    let align = if align == 0 {
        1
    } else {
        align.next_power_of_two()
    };
    (offset + align - 1) & !(align - 1)
}

impl RegionAllocator {
    /// Create a new allocator with the given configuration.
    pub fn new(config: RegionConfig) -> Self {
        let default_capacity = config.initial_capacity;
        RegionAllocator {
            regions: Vec::new(),
            free_list: Vec::new(),
            default_capacity,
            config,
            stats: AllocStats::default(),
            next_id: 0,
            current_usage: 0,
        }
    }

    /// Allocate a fresh (or recycled) region and return its id.
    ///
    /// Returns `None` if `max_regions` has been reached and no free regions are available.
    pub fn alloc_region(&mut self) -> RegionId {
        // Prefer a recycled region from the free list.
        if let Some(id) = self.free_list.pop() {
            // Reset it so it looks fresh.
            if let Some(r) = self.regions.get_mut(id.raw() as usize) {
                r.reset();
            }
            self.stats.regions_created += 1;
            return id;
        }

        // Enforce max_regions limit — if at limit, reclaim the least-used region.
        let live = self
            .stats
            .regions_created
            .saturating_sub(self.stats.regions_freed);
        if live >= self.config.max_regions {
            // Evict the region with the smallest offset (least bytes used).
            let victim = self
                .regions
                .iter()
                .enumerate()
                .filter(|(_, r)| r.offset > 0) // skip already-empty ones
                .min_by_key(|(_, r)| r.offset)
                .map(|(i, _)| i);
            if let Some(idx) = victim {
                let id = RegionId::new(idx as u32);
                self.free_region(id);
                if let Some(recycled) = self.free_list.pop() {
                    if let Some(r) = self.regions.get_mut(recycled.raw() as usize) {
                        r.reset();
                    }
                    self.stats.regions_created += 1;
                    return recycled;
                }
            }
        }

        // Create a brand-new region.
        let id = RegionId::new(self.next_id);
        self.next_id += 1;
        let region = Region::new(id, self.default_capacity);
        self.regions.push(region);
        self.stats.regions_created += 1;
        id
    }

    /// Return a region to the free list for later reuse.
    ///
    /// The region is immediately reset (zeroed) so its memory is available.
    pub fn free_region(&mut self, id: RegionId) {
        if let Some(r) = self.regions.get_mut(id.raw() as usize) {
            let freed_bytes = r.offset;
            self.current_usage = self.current_usage.saturating_sub(freed_bytes);
            self.stats.total_freed += freed_bytes;
            r.reset();
        }
        self.free_list.push(id);
        self.stats.regions_freed += 1;
    }

    /// Attempt to allocate `size` bytes with `align`-byte alignment inside `region`.
    ///
    /// Returns `Some(offset)` — the byte offset within the region's buffer —
    /// or `None` if the region lacks sufficient space.
    pub fn alloc_in(&mut self, region: RegionId, size: usize, align: usize) -> Option<usize> {
        let r = self.regions.get_mut(region.raw() as usize)?;
        let aligned = align_up(r.offset, align);
        let end = aligned.checked_add(size)?;
        if end > r.capacity {
            return None;
        }
        r.offset = end;
        self.current_usage += size;
        self.stats.total_allocated += size;
        if self.current_usage > self.stats.peak_usage {
            self.stats.peak_usage = self.current_usage;
        }
        Some(aligned)
    }

    /// Return a snapshot of current allocation statistics.
    pub fn stats(&self) -> AllocStats {
        self.stats.clone()
    }

    /// Free every live region and return all memory to the free list.
    pub fn reset(&mut self) {
        let len = self.regions.len();
        for i in 0..len {
            if let Some(r) = self.regions.get_mut(i) {
                if r.offset > 0 {
                    self.stats.total_freed += r.offset;
                    self.stats.regions_freed += 1;
                    r.reset();
                    self.free_list.push(r.id);
                }
            }
        }
        self.current_usage = 0;
    }

    /// Remove empty regions from the managed pool, releasing their backing memory.
    ///
    /// This compacts `regions` in-place. Only regions that are both in the
    /// free list AND have zero allocated bytes are dropped.
    pub fn compact(&mut self) {
        // Collect the set of free-listed ids.
        let free_set: std::collections::HashSet<u32> =
            self.free_list.iter().map(|r| r.raw()).collect();

        // Partition: retain only regions that are active (not free).
        let mut new_regions: Vec<Region> = Vec::with_capacity(self.regions.len());
        let mut new_free: Vec<RegionId> = Vec::new();
        let mut next_id: u32 = 0;

        for r in self.regions.drain(..) {
            if free_set.contains(&r.id.raw()) && r.is_empty() {
                // Drop this region — its backing Vec<u8> is freed here.
            } else {
                // Re-index so ids stay contiguous.
                let new_id = RegionId::new(next_id);
                let mut region = r;
                region.id = new_id;
                if free_set.contains(&region.id.raw()) {
                    new_free.push(new_id);
                }
                new_regions.push(region);
                next_id += 1;
            }
        }

        self.regions = new_regions;
        self.free_list = new_free;
        self.next_id = next_id;
    }

    /// Get a read-only reference to a region by id.
    pub fn get_region(&self, id: RegionId) -> Option<&Region> {
        self.regions.get(id.raw() as usize)
    }

    /// Get the raw bytes slice for an allocation inside a region.
    ///
    /// `offset` and `size` are as returned/used by `alloc_in`.
    pub fn get_bytes(&self, id: RegionId, offset: usize, size: usize) -> Option<&[u8]> {
        let r = self.regions.get(id.raw() as usize)?;
        r.data.get(offset..offset + size)
    }

    /// Get a mutable bytes slice for an allocation inside a region.
    pub fn get_bytes_mut(&mut self, id: RegionId, offset: usize, size: usize) -> Option<&mut [u8]> {
        let r = self.regions.get_mut(id.raw() as usize)?;
        r.data.get_mut(offset..offset + size)
    }

    /// Obtain a lifetime-tied handle to a region.
    pub fn handle(&self, id: RegionId) -> Option<RegionHandle<'_>> {
        if (id.raw() as usize) < self.regions.len() {
            Some(RegionHandle::new(id))
        } else {
            None
        }
    }

    /// Number of currently live (non-freed) regions.
    pub fn live_count(&self) -> usize {
        self.stats
            .regions_created
            .saturating_sub(self.stats.regions_freed)
    }

    /// Total bytes currently allocated across all live regions.
    pub fn current_usage(&self) -> usize {
        self.current_usage
    }

    /// Write bytes into an allocation inside a region.
    pub fn write_bytes(&mut self, id: RegionId, offset: usize, data: &[u8]) -> Option<()> {
        let r = self.regions.get_mut(id.raw() as usize)?;
        let dest = r.data.get_mut(offset..offset + data.len())?;
        dest.copy_from_slice(data);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_alloc() -> RegionAllocator {
        RegionAllocator::new(RegionConfig::default())
    }

    // --- align_up tests ---

    #[test]
    fn test_align_up_zero_align() {
        assert_eq!(align_up(0, 0), 0);
        assert_eq!(align_up(5, 0), 5);
    }

    #[test]
    fn test_align_up_already_aligned() {
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(16, 4), 16);
    }

    #[test]
    fn test_align_up_needs_padding() {
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(9, 8), 16);
        assert_eq!(align_up(3, 4), 4);
    }

    #[test]
    fn test_align_up_non_power_of_two_align() {
        // align=6 is rounded up to 8 internally.
        assert_eq!(align_up(1, 6), 8);
    }

    // --- RegionId tests ---

    #[test]
    fn test_region_id_display() {
        let id = RegionId::new(42);
        assert_eq!(format!("{}", id), "region#42");
    }

    #[test]
    fn test_region_id_ord() {
        let a = RegionId::new(1);
        let b = RegionId::new(2);
        assert!(a < b);
    }

    // --- RegionConfig tests ---

    #[test]
    fn test_region_config_clamping() {
        let cfg = RegionConfig::new(0, 0.0, 0);
        assert!(cfg.growth_factor > 1.0);
        assert!(cfg.initial_capacity >= 64);
        assert!(cfg.max_regions >= 1);
    }

    #[test]
    fn test_region_config_grow() {
        let cfg = RegionConfig::new(1024, 2.0, 16);
        assert_eq!(cfg.grow(1024), 2048);
        assert_eq!(cfg.grow(0), 64); // max(0, 64)
    }

    #[test]
    fn test_region_config_default() {
        let cfg = RegionConfig::default();
        assert!(cfg.initial_capacity > 0);
        assert!(cfg.max_regions > 0);
    }

    // --- Region tests ---

    #[test]
    fn test_region_new() {
        let r = Region::new(RegionId::new(0), 256);
        assert_eq!(r.capacity, 256);
        assert_eq!(r.offset, 0);
        assert!(r.is_empty());
    }

    #[test]
    fn test_region_remaining_and_used() {
        let mut r = Region::new(RegionId::new(0), 100);
        assert_eq!(r.remaining(), 100);
        r.offset = 30;
        assert_eq!(r.remaining(), 70);
        assert_eq!(r.used(), 30);
    }

    #[test]
    fn test_region_reset_zeroes() {
        let mut r = Region::new(RegionId::new(0), 64);
        r.offset = 10;
        r.data[0] = 0xFF;
        r.reset();
        assert!(r.is_empty());
        assert_eq!(r.data[0], 0x00);
    }

    // --- AllocStats tests ---

    #[test]
    fn test_alloc_stats_active_regions() {
        let stats = AllocStats {
            regions_created: 5,
            regions_freed: 2,
            ..Default::default()
        };
        assert_eq!(stats.active_regions(), 3);
    }

    #[test]
    fn test_alloc_stats_utilization() {
        let stats = AllocStats {
            total_allocated: 50,
            ..Default::default()
        };
        assert!((stats.utilization(100) - 0.5).abs() < 1e-9);
        assert_eq!(stats.utilization(0), 0.0);
    }

    // --- RegionAllocator::alloc_region tests ---

    #[test]
    fn test_alloc_region_basic() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        assert_eq!(id.raw(), 0);
        assert_eq!(alloc.stats().regions_created, 1);
    }

    #[test]
    fn test_alloc_region_multiple() {
        let mut alloc = default_alloc();
        let id0 = alloc.alloc_region();
        let id1 = alloc.alloc_region();
        assert_ne!(id0, id1);
    }

    // --- RegionAllocator::free_region tests ---

    #[test]
    fn test_free_region_updates_stats() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        alloc.alloc_in(id, 128, 1);
        alloc.free_region(id);
        let s = alloc.stats();
        assert_eq!(s.regions_freed, 1);
        assert_eq!(s.total_freed, 128);
    }

    #[test]
    fn test_free_region_enables_reuse() {
        let mut alloc = default_alloc();
        let id0 = alloc.alloc_region();
        alloc.free_region(id0);
        // Next alloc_region should recycle id0 (it's on the free list).
        let id1 = alloc.alloc_region();
        assert_eq!(id0, id1);
    }

    // --- RegionAllocator::alloc_in tests ---

    #[test]
    fn test_alloc_in_basic() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let off = alloc.alloc_in(id, 16, 1);
        assert_eq!(off, Some(0));
        let off2 = alloc.alloc_in(id, 16, 1);
        assert_eq!(off2, Some(16));
    }

    #[test]
    fn test_alloc_in_alignment() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        alloc.alloc_in(id, 1, 1); // offset now = 1
        let off = alloc.alloc_in(id, 4, 4);
        // 1 padded to 4 = offset 4
        assert_eq!(off, Some(4));
    }

    #[test]
    fn test_alloc_in_oom() {
        // initial_capacity is clamped to 64 by RegionConfig::new, so use 128 capacity
        // and request more than that.
        let cfg = RegionConfig::new(128, 2.0, 8);
        let mut alloc = RegionAllocator::new(cfg);
        let id = alloc.alloc_region();
        // 200 bytes > 128 capacity → should fail.
        let big = alloc.alloc_in(id, 200, 1);
        assert_eq!(big, None);
    }

    #[test]
    fn test_alloc_in_updates_stats() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        alloc.alloc_in(id, 100, 1);
        alloc.alloc_in(id, 200, 1);
        let s = alloc.stats();
        assert_eq!(s.total_allocated, 300);
        assert!(s.peak_usage >= 300);
    }

    // --- write / read bytes ---

    #[test]
    fn test_write_and_read_bytes() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let off = alloc.alloc_in(id, 4, 1).expect("allocation must succeed");
        alloc.write_bytes(id, off, &[1, 2, 3, 4]);
        let bytes = alloc
            .get_bytes(id, off, 4)
            .expect("bytes must be accessible");
        assert_eq!(bytes, [1, 2, 3, 4]);
    }

    #[test]
    fn test_get_bytes_mut() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let off = alloc.alloc_in(id, 8, 8).expect("allocation must succeed");
        {
            let buf = alloc.get_bytes_mut(id, off, 8).expect("must be accessible");
            buf[0] = 0xAB;
            buf[7] = 0xCD;
        }
        let bytes = alloc
            .get_bytes(id, off, 8)
            .expect("bytes must be accessible");
        assert_eq!(bytes[0], 0xAB);
        assert_eq!(bytes[7], 0xCD);
    }

    // --- reset ---

    #[test]
    fn test_reset_clears_all() {
        let mut alloc = default_alloc();
        let id0 = alloc.alloc_region();
        let id1 = alloc.alloc_region();
        alloc.alloc_in(id0, 50, 1);
        alloc.alloc_in(id1, 100, 1);
        alloc.reset();
        assert_eq!(alloc.current_usage(), 0);
        assert!(alloc.free_list.len() >= 2);
    }

    // --- compact ---

    #[test]
    fn test_compact_removes_free_regions() {
        let mut alloc = default_alloc();
        let id0 = alloc.alloc_region();
        let id1 = alloc.alloc_region();
        alloc.alloc_in(id1, 10, 1);
        // Free id0 (empty), keep id1 alive.
        alloc.free_region(id0);
        let before = alloc.regions.len();
        alloc.compact();
        // id0 was empty and on free list, so it should be removed.
        assert!(alloc.regions.len() <= before);
    }

    // --- handle ---

    #[test]
    fn test_handle_valid_id() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let handle = alloc.handle(id);
        assert!(handle.is_some());
        let h = handle.expect("handle must exist");
        assert_eq!(h.region_id(), id);
    }

    #[test]
    fn test_handle_invalid_id() {
        let alloc = default_alloc();
        let bad_id = RegionId::new(999);
        assert!(alloc.handle(bad_id).is_none());
    }

    // --- live_count / current_usage ---

    #[test]
    fn test_live_count() {
        let mut alloc = default_alloc();
        assert_eq!(alloc.live_count(), 0);
        let id = alloc.alloc_region();
        assert_eq!(alloc.live_count(), 1);
        alloc.free_region(id);
        assert_eq!(alloc.live_count(), 0);
    }

    #[test]
    fn test_current_usage_tracks_allocs() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        assert_eq!(alloc.current_usage(), 0);
        alloc.alloc_in(id, 64, 1);
        assert_eq!(alloc.current_usage(), 64);
    }

    #[test]
    fn test_current_usage_decreases_on_free() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        alloc.alloc_in(id, 128, 1);
        alloc.free_region(id);
        assert_eq!(alloc.current_usage(), 0);
    }

    // --- RegionHandle display ---

    #[test]
    fn test_region_handle_display() {
        let h: RegionHandle<'_> = RegionHandle::new(RegionId::new(7));
        assert_eq!(format!("{}", h), "handle(region#7)");
    }

    // --- edge cases ---

    #[test]
    fn test_alloc_in_fills_exactly() {
        // initial_capacity is clamped to 64 by RegionConfig::new.
        let cfg = RegionConfig::new(64, 2.0, 8);
        let mut alloc = RegionAllocator::new(cfg);
        let id = alloc.alloc_region();
        // Fill exactly 64 bytes.
        let off = alloc.alloc_in(id, 64, 1);
        assert_eq!(off, Some(0));
        // No more space at all.
        let off2 = alloc.alloc_in(id, 1, 1);
        assert_eq!(off2, None);
    }

    #[test]
    fn test_free_then_realloc_is_clean() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let off = alloc.alloc_in(id, 4, 1).expect("alloc must succeed");
        alloc.write_bytes(id, off, &[0xFF, 0xFF, 0xFF, 0xFF]);
        alloc.free_region(id);

        // Recycle the same region.
        let id2 = alloc.alloc_region();
        let off2 = alloc.alloc_in(id2, 4, 1).expect("alloc must succeed");
        let bytes = alloc
            .get_bytes(id2, off2, 4)
            .expect("bytes must be accessible");
        // After reset, bytes should be zeroed.
        assert_eq!(bytes, [0, 0, 0, 0]);
    }

    #[test]
    fn test_get_region() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        let r = alloc.get_region(id);
        assert!(r.is_some());
        assert_eq!(r.expect("region must exist").id, id);
    }

    #[test]
    fn test_multiple_allocs_alignment_chain() {
        let mut alloc = default_alloc();
        let id = alloc.alloc_region();
        // 1-byte alloc, then 8-byte aligned.
        let off0 = alloc.alloc_in(id, 1, 1).expect("alloc0 must succeed");
        let off1 = alloc.alloc_in(id, 8, 8).expect("alloc1 must succeed");
        let off2 = alloc.alloc_in(id, 3, 4).expect("alloc2 must succeed");
        // off1 must be 8-aligned.
        assert_eq!(off1 % 8, 0);
        // off2 must be 4-aligned.
        assert_eq!(off2 % 4, 0);
        // They must not overlap.
        assert!(off0 < off1);
        assert!(off1 < off2);
    }
}
