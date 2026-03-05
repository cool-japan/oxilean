//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AlignedBuffer, ArenaAllocator, BlockPool, BucketPool, CacheLineAlignedPool, CompactVec,
    CowBuffer, FreeListPool, GapAllocator, GrowablePool, MemoryBlock, MemoryBudget, ObjectHeader,
    PageTable, PoolAllocator, PoolConfig, PoolMirror, PoolReport, PoolSnapshot, PoolStats,
    PoolTimeline, RegionAllocator, SizeClassPool, StackPool, TypedArena,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pool_config_default() {
        let cfg = PoolConfig::default();
        assert_eq!(cfg.block_size, 64);
        assert_eq!(cfg.max_blocks, 0);
        assert!((cfg.growth_factor - 2.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_config_builder() {
        let cfg = PoolConfig::default()
            .with_block_size(128)
            .with_max_blocks(10)
            .with_growth_factor(1.5);
        assert_eq!(cfg.block_size, 128);
        assert_eq!(cfg.max_blocks, 10);
        assert!((cfg.growth_factor - 1.5).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_config_growth_factor_clamped() {
        let cfg = PoolConfig::default().with_growth_factor(0.5);
        assert!((cfg.growth_factor - 1.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_allocate_and_read() {
        let mut pool: PoolAllocator<u64> = PoolAllocator::new();
        let ptr = pool.allocate(42u64).expect("allocation should succeed");
        assert_eq!(unsafe { *ptr.as_ptr() }, 42u64);
        assert_eq!(pool.allocated(), 1);
    }
    #[test]
    fn test_pool_allocate_deallocate() {
        let mut pool: PoolAllocator<i32> = PoolAllocator::new();
        let ptr = pool.allocate(100).expect("allocation should succeed");
        assert_eq!(pool.allocated(), 1);
        unsafe { pool.deallocate(ptr) };
        assert_eq!(pool.allocated(), 0);
    }
    #[test]
    fn test_pool_stats() {
        let cfg = PoolConfig::default().with_block_size(4);
        let mut pool: PoolAllocator<u32> = PoolAllocator::with_config(cfg);
        let _ = pool.allocate(1);
        let _ = pool.allocate(2);
        let stats = pool.stats();
        assert_eq!(stats.allocated, 2);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.free, 2);
        assert_eq!(stats.slab_count, 1);
    }
    #[test]
    fn test_pool_stats_utilization() {
        let stats = PoolStats {
            allocated: 3,
            free: 1,
            total: 4,
            slab_count: 1,
        };
        assert!((stats.utilization() - 0.75).abs() < f64::EPSILON);
    }
    #[test]
    fn test_pool_growth() {
        let cfg = PoolConfig::default()
            .with_block_size(2)
            .with_growth_factor(2.0);
        let mut pool: PoolAllocator<u8> = PoolAllocator::with_config(cfg);
        let _ = pool.allocate(1);
        let _ = pool.allocate(2);
        let _ = pool.allocate(3);
        let stats = pool.stats();
        assert_eq!(stats.slab_count, 2);
        assert_eq!(stats.allocated, 3);
        assert_eq!(stats.total, 6);
    }
    #[test]
    fn test_pool_max_blocks_limit() {
        let cfg = PoolConfig::default()
            .with_block_size(1)
            .with_max_blocks(2)
            .with_growth_factor(1.0);
        let mut pool: PoolAllocator<u64> = PoolAllocator::with_config(cfg);
        let _ = pool.allocate(1);
        let _ = pool.allocate(2);
        let result = pool.allocate(3);
        assert!(result.is_none());
    }
    #[test]
    fn test_pool_clear() {
        let mut pool: PoolAllocator<u32> = PoolAllocator::new();
        let _ = pool.allocate(10);
        let _ = pool.allocate(20);
        pool.clear();
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.capacity(), 0);
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_pool_reset() {
        let cfg = PoolConfig::default().with_block_size(8);
        let mut pool: PoolAllocator<u64> = PoolAllocator::with_config(cfg);
        let _ = pool.allocate(1);
        let _ = pool.allocate(2);
        let _ = pool.allocate(3);
        assert_eq!(pool.allocated(), 3);
        unsafe { pool.reset() };
        assert_eq!(pool.allocated(), 0);
        assert_eq!(pool.capacity(), 8);
        assert_eq!(pool.free_count(), 8);
    }
    #[test]
    fn test_pool_reuse_after_dealloc() {
        let cfg = PoolConfig::default().with_block_size(2);
        let mut pool: PoolAllocator<u64> = PoolAllocator::with_config(cfg);
        let p1 = pool.allocate(10).expect("allocation should succeed");
        unsafe { pool.deallocate(p1) };
        let p2 = pool.allocate(20).expect("allocation should succeed");
        assert_eq!(unsafe { *p2.as_ptr() }, 20);
        assert_eq!(pool.stats().slab_count, 1);
    }
    #[test]
    fn test_pool_stats_display() {
        let stats = PoolStats {
            allocated: 5,
            free: 3,
            total: 8,
            slab_count: 1,
        };
        let s = format!("{}", stats);
        assert!(s.contains("allocated: 5"));
        assert!(s.contains("free: 3"));
    }
    #[test]
    fn test_arena_alloc_bytes() {
        let mut arena = ArenaAllocator::new(256);
        let ptr = arena.alloc_bytes(16, 8);
        assert!(ptr.is_some());
        assert_eq!(arena.bytes_allocated(), 16);
    }
    #[test]
    fn test_arena_alloc_value() {
        let mut arena = ArenaAllocator::new(256);
        let ptr = arena.alloc_value(42u64).expect("allocation should succeed");
        assert_eq!(unsafe { *ptr.as_ptr() }, 42u64);
    }
    #[test]
    fn test_arena_multiple_allocs() {
        let mut arena = ArenaAllocator::new(128);
        for i in 0u32..20 {
            let ptr = arena.alloc_value(i).expect("allocation should succeed");
            assert_eq!(unsafe { *ptr.as_ptr() }, i);
        }
        assert_eq!(arena.bytes_allocated(), 20 * std::mem::size_of::<u32>());
    }
    #[test]
    fn test_arena_reset() {
        let mut arena = ArenaAllocator::new(128);
        for i in 0u64..10 {
            arena.alloc_value(i);
        }
        let reserved_before = arena.bytes_reserved();
        arena.reset();
        assert_eq!(arena.bytes_allocated(), 0);
        assert!(arena.bytes_reserved() > 0 || reserved_before == 0);
    }
    #[test]
    fn test_arena_chunk_growth() {
        let mut arena = ArenaAllocator::new(64);
        for _ in 0..100 {
            arena.alloc_bytes(32, 1);
        }
        assert!(arena.chunk_count() > 1);
    }
    #[test]
    fn test_arena_alignment() {
        let mut arena = ArenaAllocator::new(256);
        let ptr = arena.alloc_bytes(8, 16).expect("allocation should succeed");
        assert_eq!(ptr.as_ptr() as usize % 16, 0);
    }
    #[test]
    fn test_arena_fragmentation() {
        let arena = ArenaAllocator::new(256);
        assert_eq!(arena.fragmentation(), 0.0);
    }
    #[test]
    fn test_arena_zero_size_alloc() {
        let mut arena = ArenaAllocator::new(128);
        let ptr = arena.alloc_bytes(0, 1);
        assert!(ptr.is_some());
        assert_eq!(arena.bytes_allocated(), 0);
    }
}
pub(super) const NUM_BUCKETS: usize = 8;
pub(super) const BUCKET_SIZES: [usize; NUM_BUCKETS] = [8, 16, 32, 64, 128, 256, 512, 1024];
/// Statistics summary for pool health monitoring.
#[allow(dead_code)]
pub fn pool_health_report<T>(pool: &PoolAllocator<T>) -> String {
    let stats = pool.stats();
    format!(
        "PoolHealth {{ allocated: {}, free: {}, total: {}, slabs: {}, utilization: {:.1}% }}",
        stats.allocated,
        stats.free,
        stats.total,
        stats.slab_count,
        stats.utilization() * 100.0,
    )
}
/// Determine if a pool should grow (utilization >= threshold).
#[allow(dead_code)]
pub fn should_grow<T>(pool: &PoolAllocator<T>, threshold: f64) -> bool {
    pool.stats().utilization() >= threshold
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_typed_arena_alloc() {
        let mut arena: TypedArena<u64> = TypedArena::new(4);
        let r1 = arena.alloc(10u64);
        assert_eq!(*r1, 10);
        arena.alloc(20u64);
        arena.alloc(30u64);
        arena.alloc(40u64);
        arena.alloc(50u64);
        assert_eq!(arena.len(), 5);
        assert!(arena.chunk_count() >= 1);
    }
    #[test]
    fn test_typed_arena_iter() {
        let mut arena: TypedArena<u32> = TypedArena::new(4);
        for i in 0u32..10 {
            arena.alloc(i);
        }
        let sum: u32 = arena.iter().sum();
        assert_eq!(sum, 45);
    }
    #[test]
    fn test_typed_arena_clear() {
        let mut arena: TypedArena<String> = TypedArena::new(4);
        arena.alloc("hello".to_string());
        arena.alloc("world".to_string());
        arena.clear();
        assert!(arena.is_empty());
    }
    #[test]
    fn test_free_list_pool_insert_get() {
        let mut pool: FreeListPool<u32> = FreeListPool::new();
        let idx = pool.insert(42);
        assert_eq!(pool.get(idx), Some(&42));
    }
    #[test]
    fn test_free_list_pool_remove() {
        let mut pool: FreeListPool<String> = FreeListPool::new();
        let idx = pool.insert("hello".to_string());
        let val = pool.remove(idx);
        assert_eq!(val, Some("hello".to_string()));
        assert_eq!(pool.get(idx), None);
    }
    #[test]
    fn test_free_list_pool_reuse_slot() {
        let mut pool: FreeListPool<u32> = FreeListPool::new();
        let idx1 = pool.insert(1);
        pool.remove(idx1);
        let idx2 = pool.insert(2);
        assert_eq!(idx1, idx2);
        assert_eq!(pool.get(idx2), Some(&2));
    }
    #[test]
    fn test_free_list_pool_len() {
        let mut pool: FreeListPool<u64> = FreeListPool::new();
        pool.insert(1);
        pool.insert(2);
        let idx = pool.insert(3);
        pool.remove(idx);
        assert_eq!(pool.len(), 2);
    }
    #[test]
    fn test_free_list_pool_iter() {
        let mut pool: FreeListPool<u32> = FreeListPool::new();
        pool.insert(10);
        pool.insert(20);
        pool.insert(30);
        let sum: u32 = pool.iter().map(|(_, v)| *v).sum();
        assert_eq!(sum, 60);
    }
    #[test]
    fn test_bucket_pool_alloc_free() {
        let mut pool = BucketPool::new();
        let buf = pool.alloc(10).expect("allocation should succeed");
        assert!(buf.len() >= 10);
        pool.free(buf);
        assert_eq!(pool.pool_size(), 1);
    }
    #[test]
    fn test_bucket_pool_reuse() {
        let mut pool = BucketPool::new();
        let buf1 = pool.alloc(8).expect("allocation should succeed");
        pool.free(buf1);
        let _buf2 = pool.alloc(8).expect("allocation should succeed");
        assert_eq!(pool.pool_size(), 0);
    }
    #[test]
    fn test_bucket_pool_too_large() {
        let mut pool = BucketPool::new();
        let result = pool.alloc(2000);
        assert!(result.is_none());
    }
    #[test]
    fn test_bucket_pool_bytes() {
        let mut pool = BucketPool::new();
        let buf = pool.alloc(8).expect("allocation should succeed");
        pool.free(buf);
        assert_eq!(pool.pool_bytes(), 8);
    }
    #[test]
    fn test_region_alloc_and_free() {
        let mut alloc = RegionAllocator::new();
        let id = alloc.alloc_region(64);
        let bytes = alloc.region_bytes(id).expect("allocation should succeed");
        assert_eq!(bytes.len(), 64);
        alloc.free_region(id);
        assert!(alloc.region_bytes(id).is_none());
    }
    #[test]
    fn test_region_reuse() {
        let mut alloc = RegionAllocator::new();
        let id1 = alloc.alloc_region(32);
        alloc.free_region(id1);
        let id2 = alloc.alloc_region(32);
        assert_eq!(id1, id2);
        assert_eq!(alloc.live_count(), 1);
    }
    #[test]
    fn test_region_total_bytes() {
        let mut alloc = RegionAllocator::new();
        alloc.alloc_region(100);
        alloc.alloc_region(200);
        assert_eq!(alloc.total_bytes(), 300);
    }
    #[test]
    fn test_region_bytes_mut() {
        let mut alloc = RegionAllocator::new();
        let id = alloc.alloc_region(8);
        let bytes = alloc
            .region_bytes_mut(id)
            .expect("allocation should succeed");
        bytes[0] = 42;
        assert_eq!(
            alloc.region_bytes(id).expect("allocation should succeed")[0],
            42
        );
    }
    #[test]
    fn test_memory_budget_basic() {
        let mut budget = MemoryBudget::new(1000);
        assert!(budget.try_alloc(500));
        assert_eq!(budget.used(), 500);
        assert_eq!(budget.remaining(), 500);
    }
    #[test]
    fn test_memory_budget_exceeded() {
        let mut budget = MemoryBudget::new(100);
        assert!(budget.try_alloc(80));
        assert!(!budget.try_alloc(30));
        assert_eq!(budget.used(), 80);
    }
    #[test]
    fn test_memory_budget_release() {
        let mut budget = MemoryBudget::new(100);
        budget.try_alloc(60);
        budget.release(20);
        assert_eq!(budget.used(), 40);
    }
    #[test]
    fn test_memory_budget_peak() {
        let mut budget = MemoryBudget::new(1000);
        budget.try_alloc(300);
        budget.try_alloc(400);
        budget.release(200);
        assert_eq!(budget.peak(), 700);
    }
    #[test]
    fn test_memory_budget_display() {
        let mut budget = MemoryBudget::new(1000);
        budget.try_alloc(250);
        let s = format!("{}", budget);
        assert!(s.contains("250/1000"));
    }
    #[test]
    fn test_pool_mirror_write_flip_read() {
        let mut mirror: PoolMirror<u32> = PoolMirror::new();
        mirror.write(0, 10u32);
        mirror.write(1, 20u32);
        mirror.flip();
        assert_eq!(mirror.read(0), Some(&10));
        assert_eq!(mirror.read(1), Some(&20));
    }
    #[test]
    fn test_pool_mirror_front_empty_before_flip() {
        let mut mirror: PoolMirror<u32> = PoolMirror::new();
        mirror.write(0, 5u32);
        assert_eq!(mirror.read(0), None);
    }
    #[test]
    fn test_pool_mirror_double_flip() {
        let mut mirror: PoolMirror<u32> = PoolMirror::new();
        mirror.write(0, 100u32);
        mirror.flip();
        mirror.write(0, 200u32);
        mirror.flip();
        assert_eq!(mirror.read(0), Some(&200));
    }
    #[test]
    fn test_aligned_buffer_len() {
        let buf = AlignedBuffer::new(64, 16);
        assert_eq!(buf.len(), 64);
    }
    #[test]
    fn test_aligned_buffer_fill() {
        let mut buf = AlignedBuffer::new(8, 8);
        buf.fill(0xFF);
        for b in buf.as_slice() {
            assert_eq!(*b, 0xFF);
        }
    }
    #[test]
    fn test_aligned_buffer_alignment() {
        let buf = AlignedBuffer::new(32, 16);
        assert_eq!(buf.alignment(), 16);
    }
    #[test]
    fn test_pool_health_report() {
        let mut pool: PoolAllocator<u32> = PoolAllocator::new();
        pool.allocate(1);
        let report = pool_health_report(&pool);
        assert!(report.contains("allocated: 1"));
    }
    #[test]
    fn test_should_grow() {
        let cfg = PoolConfig::default().with_block_size(4);
        let mut pool: PoolAllocator<u32> = PoolAllocator::with_config(cfg);
        pool.allocate(1);
        pool.allocate(2);
        pool.allocate(3);
        assert!(should_grow(&pool, 0.7));
        assert!(!should_grow(&pool, 0.9));
    }
}
pub(super) const SIZE_CLASSES: [usize; 6] = [8, 16, 32, 64, 128, 256];
#[cfg(test)]
mod tests_memory_extended {
    use super::*;
    #[test]
    fn test_stack_pool_acquire_release() {
        let mut pool: StackPool<u32> = StackPool::new(4);
        let v = pool.acquire().expect("test operation should succeed");
        assert_eq!(pool.available(), 3);
        pool.release(v);
        assert_eq!(pool.available(), 4);
    }
    #[test]
    fn test_stack_pool_exhaustion() {
        let mut pool: StackPool<u32> = StackPool::new(2);
        pool.acquire();
        pool.acquire();
        assert!(pool.acquire().is_none());
        assert!(pool.is_exhausted());
    }
    #[test]
    fn test_stack_pool_reset() {
        let mut pool: StackPool<u32> = StackPool::new(3);
        pool.acquire();
        pool.acquire();
        pool.reset();
        assert_eq!(pool.available(), 3);
    }
    #[test]
    fn test_object_header_colors() {
        let mut h = ObjectHeader::new(1, 0);
        assert!(h.is_white());
        h.mark_gray();
        assert!(h.is_gray());
        h.mark_black();
        assert!(h.is_black());
        h.reset_to_white();
        assert!(h.is_white());
    }
    #[test]
    fn test_size_class_pool_alloc_free() {
        let mut pool = SizeClassPool::new();
        let buf = pool.alloc(10).expect("allocation should succeed");
        assert_eq!(buf.len(), 16);
        pool.free(buf);
        assert_eq!(pool.free_slots(1), 1);
    }
    #[test]
    fn test_size_class_pool_unknown_size() {
        let _pool = SizeClassPool::new();
        assert!(SizeClassPool::size_class_for(512).is_none());
    }
    #[test]
    fn test_memory_block_write_read() {
        let mut block = MemoryBlock::new(64, 0);
        assert!(block.write_bytes(0, &[1, 2, 3, 4]));
        assert_eq!(
            block
                .read_bytes(0, 4)
                .expect("test operation should succeed"),
            &[1, 2, 3, 4]
        );
        assert_eq!(block.used(), 4);
    }
    #[test]
    fn test_memory_block_overflow() {
        let mut block = MemoryBlock::new(4, 0);
        assert!(!block.write_bytes(3, &[1, 2]));
    }
    #[test]
    fn test_memory_block_clear() {
        let mut block = MemoryBlock::new(16, 0);
        block.write_bytes(0, &[42; 8]);
        block.clear();
        assert_eq!(block.used(), 0);
        assert_eq!(
            block
                .read_bytes(0, 1)
                .expect("test operation should succeed")[0],
            0
        );
    }
    #[test]
    fn test_block_pool_alloc() {
        let mut pool = BlockPool::new(128);
        let idx = pool.alloc_block();
        assert_eq!(pool.block_count(), 1);
        assert_eq!(
            pool.get(idx).expect("key should exist in map").capacity(),
            128
        );
    }
    #[test]
    fn test_block_pool_avg_utilization() {
        let mut pool = BlockPool::new(100);
        let i0 = pool.alloc_block();
        let i1 = pool.alloc_block();
        pool.get_mut(i0)
            .expect("test operation should succeed")
            .write_bytes(0, &[0u8; 50]);
        pool.get_mut(i1)
            .expect("test operation should succeed")
            .write_bytes(0, &[0u8; 25]);
        let avg = pool.avg_utilization();
        assert!((avg - 0.375).abs() < 0.01);
    }
    #[test]
    fn test_page_table_translate() {
        let mut pt = PageTable::new(4096);
        pt.map(0, 5);
        let paddr = pt.translate(100).expect("test operation should succeed");
        assert_eq!(paddr, 5 * 4096 + 100);
    }
    #[test]
    fn test_page_table_fault() {
        let mut pt = PageTable::new(4096);
        assert!(pt.translate(9999).is_none());
        assert_eq!(pt.fault_count(), 1);
    }
    #[test]
    fn test_compact_vec_push_remove() {
        let mut cv: CompactVec<i32> = CompactVec::new(10);
        cv.push(1);
        cv.push(2);
        cv.push(3);
        cv.remove(1);
        assert_eq!(cv.live_count(), 2);
        assert_eq!(cv.dead_count(), 1);
    }
    #[test]
    fn test_compact_vec_auto_compact() {
        let mut cv: CompactVec<i32> = CompactVec::new(2);
        cv.push(1);
        cv.push(2);
        cv.push(3);
        cv.remove(0);
        cv.remove(1);
        assert_eq!(cv.dead_count(), 0);
        assert_eq!(cv.live_count(), 1);
    }
    #[test]
    fn test_pool_snapshot_utilization() {
        let mut pool: PoolAllocator<u32> = PoolAllocator::new();
        pool.allocate(1);
        let snap = PoolSnapshot::capture(&pool, 1000);
        assert!(snap.utilization() > 0.0);
        assert!(snap.live_count > 0);
    }
    #[test]
    fn test_pool_timeline() {
        let mut timeline = PoolTimeline::new();
        let mut pool: PoolAllocator<u32> = PoolAllocator::new();
        pool.allocate(10);
        timeline.record(PoolSnapshot::capture(&pool, 0));
        pool.allocate(20);
        timeline.record(PoolSnapshot::capture(&pool, 1));
        assert_eq!(timeline.len(), 2);
        assert!(timeline.peak_allocated() >= 2);
    }
}
pub(super) const CACHE_LINE_SIZE: usize = 64;
#[cfg(test)]
mod tests_memory_extended2 {
    use super::*;
    #[test]
    fn test_cow_buffer_read() {
        let buf = CowBuffer::from_slice(&[1, 2, 3]);
        assert_eq!(buf.as_slice(), &[1, 2, 3]);
        assert!(buf.is_shared());
    }
    #[test]
    fn test_cow_buffer_write() {
        let mut buf = CowBuffer::from_slice(&[1, 2, 3]);
        buf.as_mut_slice()[0] = 99;
        assert_eq!(buf.as_slice()[0], 99);
        assert!(!buf.is_shared());
        assert_eq!(buf.write_count(), 1);
    }
    #[test]
    fn test_growable_pool_alloc() {
        let mut pool = GrowablePool::new(64);
        let s = pool.alloc(32);
        s[0] = 7;
        assert_eq!(pool.num_chunks(), 1);
    }
    #[test]
    fn test_growable_pool_grow() {
        let mut pool = GrowablePool::new(16);
        pool.alloc(16);
        pool.alloc(1);
        assert_eq!(pool.num_chunks(), 2);
    }
    #[test]
    fn test_growable_pool_reset() {
        let mut pool = GrowablePool::new(32);
        pool.alloc(32);
        pool.alloc(1);
        pool.reset();
        assert_eq!(pool.num_chunks(), 1);
    }
    #[test]
    fn test_cache_aligned_pool_alloc_free() {
        let mut pool = CacheLineAlignedPool::new(10, 4);
        assert!(pool.is_cache_aligned());
        let idx = pool.alloc().expect("allocation should succeed");
        assert_eq!(pool.live(), 1);
        pool.free(idx);
        assert_eq!(pool.live(), 0);
    }
    #[test]
    fn test_cache_aligned_pool_slot_access() {
        let mut pool = CacheLineAlignedPool::new(16, 2);
        let idx = pool.alloc().expect("allocation should succeed");
        let slot = pool.slot_mut(idx).expect("test operation should succeed");
        slot[0] = 42;
        assert_eq!(
            pool.slot(idx).expect("test operation should succeed")[0],
            42
        );
    }
    #[test]
    fn test_cache_aligned_pool_exhaustion() {
        let mut pool = CacheLineAlignedPool::new(8, 1);
        let _ = pool.alloc().expect("allocation should succeed");
        assert!(pool.alloc().is_none());
    }
    #[test]
    fn test_cache_aligned_slot_size() {
        let pool = CacheLineAlignedPool::new(10, 2);
        assert_eq!(pool.slot_size(), CACHE_LINE_SIZE);
    }
}
/// Compute a PoolReport from a PoolAllocator.
#[allow(dead_code)]
pub fn build_report<T>(name: &str, pool: &PoolAllocator<T>) -> PoolReport {
    let util = if pool.capacity() == 0 {
        0.0
    } else {
        pool.allocated() as f64 / pool.capacity() as f64
    };
    PoolReport::new(name).with_utilization(util)
}
#[cfg(test)]
mod tests_memory_extended3 {
    use super::*;
    #[test]
    fn test_pool_report_summary() {
        let mut pool: PoolAllocator<u32> = PoolAllocator::new();
        pool.allocate(5);
        let report = build_report("test", &pool);
        let s = report.summary();
        assert!(s.contains("Pool[test]"));
    }
    #[test]
    fn test_pool_report_high_util_warning() {
        let report = PoolReport::new("r").with_utilization(0.95);
        assert!(report.has_warnings());
    }
    #[test]
    fn test_gap_allocator_basic() {
        let mut alloc = GapAllocator::new(100);
        let off = alloc.alloc(30).expect("allocation should succeed");
        assert_eq!(off, 0);
        assert_eq!(alloc.total_free(), 70);
    }
    #[test]
    fn test_gap_allocator_exhaustion() {
        let mut alloc = GapAllocator::new(16);
        alloc.alloc(16).expect("allocation should succeed");
        assert!(alloc.alloc(1).is_none());
    }
    #[test]
    fn test_gap_allocator_free() {
        let mut alloc = GapAllocator::new(100);
        let off = alloc.alloc(50).expect("allocation should succeed");
        alloc.free(off, 50);
        assert_eq!(alloc.gap_count(), 2);
    }
}
