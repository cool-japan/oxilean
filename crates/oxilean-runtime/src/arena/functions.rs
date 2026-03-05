//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AdaptiveArena, ArenaAllocHistory, ArenaBenchResult, ArenaCheckpoint, ArenaChunkPool,
    ArenaExtStats, ArenaPageManager, ArenaPool, ArenaSnapshot, ArenaWatermark, BumpArena,
    GenerationalArena, LinearAllocator, MarkArena, Region, RegionManager, RingArena, ScopedArena,
    SlabArena, ThreadLocalArena, TypedArena,
};

/// Default chunk size (64 KB).
pub(super) const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;
/// Minimum chunk size (4 KB).
pub(super) const MIN_CHUNK_SIZE: usize = 4 * 1024;
/// Maximum chunk size (16 MB).
pub(super) const MAX_CHUNK_SIZE: usize = 16 * 1024 * 1024;
/// Alignment for arena allocations (8 bytes).
pub(super) const ARENA_ALIGN: usize = 8;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bump_arena_basic() {
        let mut arena = BumpArena::new();
        let loc1 = arena.alloc(16);
        let loc2 = arena.alloc(32);
        assert_ne!(loc1, loc2);
        assert_eq!(arena.bytes_used(), 48);
    }
    #[test]
    fn test_bump_arena_reset() {
        let mut arena = BumpArena::new();
        arena.alloc(100);
        arena.alloc(200);
        assert!(arena.bytes_used() > 0);
        arena.reset();
        assert_eq!(arena.bytes_used(), 0);
    }
    #[test]
    fn test_bump_arena_get_bytes() {
        let mut arena = BumpArena::new();
        let loc = arena.alloc(4);
        let bytes = arena
            .get_bytes_mut(&loc, 4)
            .expect("test operation should succeed");
        bytes[0] = 0xDE;
        bytes[1] = 0xAD;
        bytes[2] = 0xBE;
        bytes[3] = 0xEF;
        let bytes = arena
            .get_bytes(&loc, 4)
            .expect("test operation should succeed");
        assert_eq!(bytes, &[0xDE, 0xAD, 0xBE, 0xEF]);
    }
    #[test]
    fn test_bump_arena_large_alloc() {
        let mut arena = BumpArena::with_chunk_size(MIN_CHUNK_SIZE);
        let _loc1 = arena.alloc(MIN_CHUNK_SIZE - 8);
        let _loc2 = arena.alloc(MIN_CHUNK_SIZE - 8);
        assert!(arena.num_chunks() >= 2);
    }
    #[test]
    fn test_region_basic() {
        let mut region = Region::new(0);
        assert!(region.is_active());
        let loc = region.alloc(32).expect("allocation should succeed");
        assert!(region.get_bytes(&loc, 32).is_some());
    }
    #[test]
    fn test_region_deactivate() {
        let mut region = Region::new(0);
        region.deactivate();
        assert!(region.alloc(16).is_none());
        region.reactivate();
        assert!(region.alloc(16).is_some());
    }
    #[test]
    fn test_region_manager() {
        let mut mgr = RegionManager::new();
        assert_eq!(mgr.current_region_id(), 0);
        let r1 = mgr.push_region();
        assert_eq!(mgr.current_region_id(), r1);
        let r2 = mgr.push_region();
        assert_eq!(mgr.current_region_id(), r2);
        assert_eq!(mgr.scope_depth(), 3);
        mgr.pop_region();
        assert_eq!(mgr.current_region_id(), r1);
        mgr.pop_region();
        assert_eq!(mgr.current_region_id(), 0);
    }
    #[test]
    fn test_region_manager_alloc() {
        let mut mgr = RegionManager::new();
        let (region_id, _offset) = mgr.alloc(64).expect("allocation should succeed");
        assert_eq!(region_id, 0);
        let r1 = mgr.push_region();
        let (region_id2, _offset2) = mgr.alloc(32).expect("allocation should succeed");
        assert_eq!(region_id2, r1);
    }
    #[test]
    fn test_typed_arena() {
        let mut arena = TypedArena::<i32>::new();
        let idx1 = arena.alloc(42);
        let idx2 = arena.alloc(100);
        assert_eq!(*arena.get(idx1).expect("key should exist in map"), 42);
        assert_eq!(*arena.get(idx2).expect("key should exist in map"), 100);
        assert_eq!(arena.len(), 2);
    }
    #[test]
    fn test_typed_arena_iter() {
        let mut arena = TypedArena::<&str>::new();
        arena.alloc("hello");
        arena.alloc("world");
        let items: Vec<_> = arena.iter().map(|(_, v)| *v).collect();
        assert_eq!(items, vec!["hello", "world"]);
    }
    #[test]
    fn test_arena_pool() {
        let mut pool = ArenaPool::new();
        let arena = pool.acquire();
        assert_eq!(pool.available_count(), 0);
        pool.release(arena);
        assert_eq!(pool.available_count(), 1);
        let _arena2 = pool.acquire();
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.stats().acquired, 2);
    }
    #[test]
    fn test_generational_arena() {
        let mut arena = GenerationalArena::new();
        let idx1 = arena.insert(42);
        let idx2 = arena.insert(100);
        assert_eq!(*arena.get(idx1).expect("key should exist in map"), 42);
        assert_eq!(*arena.get(idx2).expect("key should exist in map"), 100);
        assert_eq!(arena.len(), 2);
        let removed = arena.remove(idx1);
        assert_eq!(removed, Some(42));
        assert_eq!(arena.len(), 1);
        assert!(arena.get(idx1).is_none());
    }
    #[test]
    fn test_generational_arena_reuse() {
        let mut arena = GenerationalArena::new();
        let idx1 = arena.insert(1);
        arena.remove(idx1);
        let idx2 = arena.insert(2);
        assert_ne!(idx1.generation, idx2.generation);
        assert!(arena.get(idx1).is_none());
        assert_eq!(*arena.get(idx2).expect("key should exist in map"), 2);
    }
    #[test]
    fn test_thread_local_arena() {
        let mut tl = ThreadLocalArena::new();
        let _loc = tl.alloc(64);
        assert!(tl.bytes_used() > 0);
        tl.reset();
        assert_eq!(tl.bytes_used(), 0);
    }
    #[test]
    fn test_arena_stats() {
        let mut arena = BumpArena::new();
        arena.alloc(16);
        arena.alloc(32);
        assert_eq!(arena.stats().total_allocations, 2);
        assert_eq!(arena.stats().total_bytes_allocated, 48);
    }
    #[test]
    fn test_scoped_arena() {
        let mut pool = ArenaPool::new();
        {
            let mut scoped = ScopedArena::new(&mut pool);
            let _loc = scoped.alloc(64);
            assert!(scoped.arena().bytes_used() > 0);
        }
        assert_eq!(pool.available_count(), 1);
    }
    #[test]
    fn test_arena_shrink() {
        let mut arena = BumpArena::with_chunk_size(MIN_CHUNK_SIZE);
        for _ in 0..100 {
            arena.alloc(64);
        }
        let chunks_before = arena.num_chunks();
        arena.reset();
        arena.shrink();
        assert!(arena.num_chunks() <= chunks_before);
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_arena_snapshot_capture() {
        let mut arena = BumpArena::new();
        arena.alloc(128);
        let snap = ArenaSnapshot::capture(&arena);
        assert!(snap.allocated_bytes >= 128);
    }
    #[test]
    fn test_arena_snapshot_bytes_since() {
        let mut arena = BumpArena::new();
        let snap1 = ArenaSnapshot::capture(&arena);
        arena.alloc(256);
        let snap2 = ArenaSnapshot::capture(&arena);
        assert!(snap1.bytes_since(&snap2) >= 256);
    }
    #[test]
    fn test_linear_alloc_basic() {
        let mut alloc = LinearAllocator::new(256);
        let off = alloc
            .alloc_offset(16, 8)
            .expect("allocation should succeed");
        assert_eq!(off % 8, 0);
        assert_eq!(alloc.alloc_count(), 1);
    }
    #[test]
    fn test_linear_alloc_overflow() {
        let mut alloc = LinearAllocator::new(32);
        assert!(alloc.alloc_offset(64, 1).is_none());
        assert_eq!(alloc.overflow_count(), 1);
    }
    #[test]
    fn test_linear_alloc_reset() {
        let mut alloc = LinearAllocator::new(256);
        alloc.alloc_offset(100, 1);
        alloc.reset();
        assert_eq!(alloc.top(), 0);
        assert!(alloc.alloc_offset(100, 1).is_some());
    }
    #[test]
    fn test_linear_alloc_utilization() {
        let mut alloc = LinearAllocator::new(100);
        alloc.alloc_offset(50, 1);
        assert!((alloc.utilization() - 0.5).abs() < 0.01);
    }
    #[test]
    fn test_linear_alloc_get_bytes() {
        let mut alloc = LinearAllocator::new(256);
        let off = alloc.alloc_offset(8, 1).expect("allocation should succeed");
        let bytes = alloc
            .get_bytes_mut(off, 8)
            .expect("allocation should succeed");
        bytes[0] = 42;
        assert_eq!(
            alloc.get_bytes(off, 8).expect("allocation should succeed")[0],
            42
        );
    }
    #[test]
    fn test_mark_arena_basic() {
        let mut arena = MarkArena::new(256);
        arena.alloc(64);
        let mark = arena.mark();
        arena.alloc(64);
        assert_eq!(arena.top(), 128);
        arena.release();
        assert_eq!(arena.top(), mark);
    }
    #[test]
    fn test_mark_arena_nested_marks() {
        let mut arena = MarkArena::new(512);
        let m1 = arena.mark();
        arena.alloc(32);
        let _m2 = arena.mark();
        arena.alloc(32);
        arena.release();
        arena.release();
        assert_eq!(arena.top(), m1);
        assert_eq!(arena.mark_depth(), 0);
    }
    #[test]
    fn test_mark_arena_reset() {
        let mut arena = MarkArena::new(256);
        arena.alloc(64);
        arena.mark();
        arena.reset();
        assert_eq!(arena.top(), 0);
        assert_eq!(arena.mark_depth(), 0);
    }
    #[test]
    fn test_mark_arena_overflow() {
        let mut arena = MarkArena::new(64);
        arena.alloc(64);
        assert!(arena.alloc(1).is_none());
    }
    #[test]
    fn test_arena_history_basic() {
        let mut arena = ArenaAllocHistory::new(1024, 100);
        arena.alloc_labeled(64, 8, "test_alloc");
        arena.alloc_labeled(128, 8, "big_alloc");
        assert_eq!(arena.history().len(), 2);
        assert_eq!(arena.largest_alloc().map(|r| r.size), Some(128));
    }
    #[test]
    fn test_arena_history_overflow() {
        let mut arena = ArenaAllocHistory::new(128, 2);
        arena.alloc_labeled(16, 1, "a");
        arena.alloc_labeled(16, 1, "b");
        arena.alloc_labeled(16, 1, "c");
        assert_eq!(arena.history().len(), 2);
    }
    #[test]
    fn test_arena_history_reset() {
        let mut arena = ArenaAllocHistory::new(256, 100);
        arena.alloc_labeled(32, 1, "foo");
        arena.reset();
        assert!(arena.history().is_empty());
        assert_eq!(arena.top(), 0);
    }
}
pub(super) const PAGE_SIZE: usize = 4096;
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_page_manager_alloc_free() {
        let mut mgr = ArenaPageManager::new();
        let idx = mgr.alloc_page();
        assert_eq!(mgr.live_pages(), 1);
        mgr.free_page(idx);
        assert_eq!(mgr.live_pages(), 0);
        assert_eq!(mgr.free_pages(), 1);
    }
    #[test]
    fn test_page_manager_reuse() {
        let mut mgr = ArenaPageManager::new();
        let idx1 = mgr.alloc_page();
        mgr.free_page(idx1);
        let idx2 = mgr.alloc_page();
        assert_eq!(idx1, idx2);
    }
    #[test]
    fn test_page_manager_write_read() {
        let mut mgr = ArenaPageManager::new();
        let idx = mgr.alloc_page();
        let page = mgr.page_mut(idx).expect("test operation should succeed");
        page[0] = 42;
        assert_eq!(mgr.page(idx).expect("test operation should succeed")[0], 42);
    }
    #[test]
    fn test_page_manager_total_bytes() {
        let mut mgr = ArenaPageManager::new();
        mgr.alloc_page();
        mgr.alloc_page();
        assert_eq!(mgr.total_bytes(), 2 * PAGE_SIZE);
    }
    #[test]
    fn test_slab_arena_alloc_free() {
        let mut slab = SlabArena::new(16, 4);
        let off = slab.alloc().expect("allocation should succeed");
        assert_eq!(slab.live_count(), 1);
        slab.free(off);
        assert_eq!(slab.live_count(), 0);
    }
    #[test]
    fn test_slab_arena_grow() {
        let mut slab = SlabArena::new(8, 2);
        slab.alloc();
        slab.alloc();
        slab.alloc();
        assert_eq!(slab.total_slots(), 3);
    }
    #[test]
    fn test_slab_arena_slot_reuse() {
        let mut slab = SlabArena::new(8, 4);
        let off = slab.alloc().expect("allocation should succeed");
        slab.free(off);
        let off2 = slab.alloc().expect("allocation should succeed");
        assert_eq!(off, off2);
    }
    #[test]
    fn test_adaptive_arena_alloc() {
        let mut arena = AdaptiveArena::new(0.75, 5);
        for _ in 0..10 {
            arena.alloc(64);
        }
        assert!(arena.allocated_bytes() > 0);
    }
    #[test]
    fn test_adaptive_arena_pressure() {
        let mut arena = AdaptiveArena::new(0.75, 5);
        assert!(!arena.is_over_utilized());
    }
    #[test]
    fn test_adaptive_arena_reset() {
        let mut arena = AdaptiveArena::new(0.75, 5);
        arena.alloc(128);
        arena.reset();
        assert_eq!(arena.allocated_bytes(), 0);
        assert_eq!(arena.avg_pressure(), 0.0);
    }
}
/// Run a simple arena benchmark: `iterations` rounds of `alloc_sizes` allocations.
#[allow(dead_code)]
pub fn bench_arena_allocs(alloc_sizes: &[usize], iterations: u64) -> ArenaBenchResult {
    let mut arena = BumpArena::new();
    let mut total_bytes: u64 = 0;
    for _ in 0..iterations {
        for &size in alloc_sizes {
            arena.alloc(size);
            total_bytes += size as u64;
        }
        arena.reset();
    }
    ArenaBenchResult::new(
        iterations,
        total_bytes,
        alloc_sizes.len(),
        "bump arena sequential alloc+reset",
    )
}
#[cfg(test)]
mod tests_arena_extended3 {
    use super::*;
    #[test]
    fn test_arena_ext_stats_merge() {
        let mut a = ArenaExtStats::new();
        a.record_alloc(100);
        let mut b = ArenaExtStats::new();
        b.record_alloc(200);
        a.merge(&b);
        assert_eq!(a.alloc_calls, 2);
        assert_eq!(a.total_bytes_allocated, 300);
    }
    #[test]
    fn test_arena_ext_stats_avg() {
        let mut s = ArenaExtStats::new();
        s.record_alloc(50);
        s.record_alloc(150);
        assert!((s.avg_alloc_size() - 100.0).abs() < 0.1);
    }
    #[test]
    fn test_arena_chunk_pool_acquire_release() {
        let mut pool = ArenaChunkPool::new(1024, 4);
        let chunk = pool.acquire();
        assert_eq!(chunk.len(), 1024);
        pool.release(chunk);
        assert_eq!(pool.pooled_count(), 1);
    }
    #[test]
    fn test_arena_chunk_pool_reuse() {
        let mut pool = ArenaChunkPool::new(64, 4);
        let chunk = pool.acquire();
        pool.release(chunk);
        let _ = pool.acquire();
        assert_eq!(pool.reused_count(), 1);
        assert_eq!(pool.created_count(), 1);
    }
    #[test]
    fn test_arena_chunk_pool_discard_oversized() {
        let mut pool = ArenaChunkPool::new(64, 1);
        let big = vec![0u8; 128];
        pool.release(big);
        assert_eq!(pool.pooled_count(), 0);
    }
    #[test]
    fn test_arena_checkpoint() {
        let mut arena = BumpArena::new();
        let cp = ArenaCheckpoint::capture(&arena);
        arena.alloc(256);
        let later = arena.bytes_used();
        assert!(cp.bytes_since(later) >= 256);
    }
    #[test]
    fn test_bench_arena_allocs() {
        let result = bench_arena_allocs(&[64, 128, 256], 10);
        assert_eq!(result.iterations, 10);
        assert_eq!(result.allocs_per_iter, 3);
        assert!(result.bytes_per_iter() > 0.0);
    }
    #[test]
    fn test_arena_watermark_from_arena() {
        let mut wm = ArenaWatermark::new();
        wm.record_alloc(500);
        wm.record_alloc(500);
        wm.record_free(200);
        assert_eq!(wm.peak(), 1000);
        assert_eq!(wm.current(), 800);
    }
}
#[cfg(test)]
mod tests_arena_extended4 {
    use super::*;
    #[test]
    fn test_ring_arena_alloc() {
        let mut r = RingArena::new(128);
        let off = r.alloc(32);
        assert_eq!(off, 0);
        assert_eq!(r.head(), 32);
    }
    #[test]
    fn test_ring_arena_wrap() {
        let mut r = RingArena::new(64);
        r.alloc(64);
        r.alloc(16);
        assert_eq!(r.wrap_count(), 1);
    }
    #[test]
    fn test_ring_arena_get() {
        let mut r = RingArena::new(64);
        let off = r.alloc(8);
        assert!(r.get(off, 8).is_some());
    }
}
