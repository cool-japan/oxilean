//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AnnotationTable, Arena, ArenaCheckpoint, ArenaMap, ArenaPool, ArenaStats, ArenaStatsExt,
    ArenaString, BiMap, BumpArena, ChainedArena, DiagMeta, DoubleArena, EventCounter,
    FrequencyTable, GrowableArena, IdDispenser, Idx, IdxRange, InterningArena, IntervalSet,
    LinearArena, LoopClock, MemoSlot, MemoryRegion, MemoryRegionRegistry, PoolArena, ScopeStack,
    ScopedArena, ScopedArenaExt, SimpleLruCache, SlabArena, Slot, SparseBitSet, StringInterner,
    Timestamp, TwoGenerationArena, TypedArena, TypedId, WorkQueue, WorkStack,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arena_alloc_and_get() {
        let mut arena = Arena::new();
        let idx1 = arena.alloc(42);
        let idx2 = arena.alloc(100);
        assert_eq!(*arena.get(idx1), 42);
        assert_eq!(*arena.get(idx2), 100);
    }
    #[test]
    fn test_idx_equality() {
        let mut arena = Arena::new();
        let idx1 = arena.alloc("hello");
        let idx2 = arena.alloc("world");
        let idx1_copy = Idx::new(idx1.raw());
        assert_eq!(idx1, idx1_copy);
        assert_ne!(idx1, idx2);
    }
    #[test]
    fn test_idx_ordering() {
        let idx0: Idx<u32> = Idx::new(0);
        let idx1: Idx<u32> = Idx::new(1);
        assert!(idx0 < idx1);
        assert!(idx1 > idx0);
    }
    #[test]
    fn test_idx_range() {
        let start: Idx<u32> = Idx::new(0);
        let end: Idx<u32> = Idx::new(5);
        let range = IdxRange::new(start, end);
        assert_eq!(range.len(), 5);
        assert!(range.contains(Idx::new(3)));
        assert!(!range.contains(Idx::new(5)));
        let indices: Vec<_> = range.iter().collect();
        assert_eq!(indices.len(), 5);
    }
    #[test]
    fn test_arena_alloc_many() {
        let mut arena: Arena<u32> = Arena::new();
        let range = arena.alloc_many([1, 2, 3, 4, 5]);
        assert_eq!(range.len(), 5);
        assert_eq!(arena.get_range(range), &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_arena_iter_indexed() {
        let mut arena: Arena<u32> = Arena::new();
        arena.alloc(10);
        arena.alloc(20);
        let pairs: Vec<_> = arena.iter_indexed().collect();
        assert_eq!(pairs.len(), 2);
        assert_eq!(*pairs[0].1, 10);
    }
    #[test]
    fn test_arena_stats() {
        let mut arena: Arena<u32> = Arena::with_capacity(16);
        arena.alloc(1);
        arena.alloc(2);
        let stats = arena.stats();
        assert_eq!(stats.len, 2);
        assert!(stats.capacity >= 2);
    }
    #[test]
    fn test_interning_arena() {
        let mut arena: InterningArena<String> = InterningArena::new();
        let idx1 = arena.intern("hello".to_string());
        let idx2 = arena.intern("hello".to_string());
        let idx3 = arena.intern("world".to_string());
        assert_eq!(idx1, idx2);
        assert_ne!(idx1, idx3);
        assert_eq!(arena.len(), 2);
    }
    #[test]
    fn test_slab_arena_alloc_free() {
        let mut arena: SlabArena<u32> = SlabArena::new();
        let idx1 = arena.alloc(10);
        let _idx2 = arena.alloc(20);
        assert_eq!(arena.len(), 2);
        let val = arena.free(idx1);
        assert_eq!(val, Some(10));
        assert_eq!(arena.len(), 1);
        assert_eq!(arena.free_count(), 1);
        let idx3 = arena.alloc(30);
        assert_eq!(idx3, idx1);
    }
    #[test]
    fn test_slab_arena_get_free_returns_none() {
        let mut arena: SlabArena<u32> = SlabArena::new();
        let idx = arena.alloc(5);
        arena.free(idx);
        assert_eq!(arena.get(idx), None);
    }
    #[test]
    fn test_bump_arena() {
        let mut bump = BumpArena::with_capacity(64);
        let off = bump.alloc_bytes(4).expect("off should be present");
        bump.write_at(off, &[1, 2, 3, 4]);
        assert_eq!(bump.read_at(off, 4), &[1, 2, 3, 4]);
        assert_eq!(bump.used(), 4);
    }
    #[test]
    fn test_bump_arena_overflow() {
        let mut bump = BumpArena::with_capacity(4);
        bump.alloc_bytes(4).expect("value should be present");
        let r = bump.alloc_bytes(1);
        assert!(r.is_none());
    }
    #[test]
    fn test_bump_arena_reset() {
        let mut bump = BumpArena::with_capacity(16);
        bump.alloc_bytes(8).expect("value should be present");
        assert_eq!(bump.used(), 8);
        bump.reset();
        assert_eq!(bump.used(), 0);
    }
    #[test]
    fn test_arena_pool() {
        let mut pool: ArenaPool<u32> = ArenaPool::new();
        let mut arena = pool.acquire();
        arena.alloc(42u32);
        assert_eq!(arena.len(), 1);
        pool.release(arena);
        assert_eq!(pool.pool_size(), 1);
        let arena2 = pool.acquire();
        assert_eq!(arena2.len(), 0);
    }
    #[test]
    fn test_idx_cast() {
        let idx: Idx<u32> = Idx::new(5);
        let idx2: Idx<u64> = idx.cast();
        assert_eq!(idx2.raw(), 5);
    }
    #[test]
    fn test_arena_stats_utilisation() {
        let stats = ArenaStats {
            len: 3,
            capacity: 4,
        };
        assert!((stats.utilisation() - 0.75).abs() < 1e-9);
        assert_eq!(stats.free(), 1);
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_scoped_arena_rollback() {
        let mut arena: ScopedArena<u32> = ScopedArena::new();
        arena.alloc(1);
        arena.alloc(2);
        let cp = arena.checkpoint();
        arena.alloc(3);
        arena.alloc(4);
        assert_eq!(arena.len(), 4);
        arena.rollback(cp);
        assert_eq!(arena.len(), 2);
    }
    #[test]
    fn test_scoped_arena_multiple_checkpoints() {
        let mut arena: ScopedArena<u32> = ScopedArena::new();
        arena.alloc(10);
        let cp1 = arena.checkpoint();
        arena.alloc(20);
        let cp2 = arena.checkpoint();
        arena.alloc(30);
        arena.rollback(cp2);
        assert_eq!(arena.len(), 2);
        arena.rollback(cp1);
        assert_eq!(arena.len(), 1);
    }
    #[test]
    fn test_double_arena() {
        let mut da: DoubleArena<u32, &str> = DoubleArena::new();
        let (ia, ib) = da.alloc_pair(42, "hello");
        assert_eq!(*da.first.get(ia), 42);
        assert_eq!(*da.second.get(ib), "hello");
        assert_eq!(da.total_len(), 2);
    }
    #[test]
    fn test_arena_map() {
        let mut arena: Arena<u32> = Arena::new();
        let idx1 = arena.alloc(10);
        let idx2 = arena.alloc(20);
        let mut map: ArenaMap<u32, String> = ArenaMap::new();
        map.insert(idx1, "ten".to_string());
        map.insert(idx2, "twenty".to_string());
        assert_eq!(map.get(idx1), Some(&"ten".to_string()));
        assert_eq!(map.get(idx2), Some(&"twenty".to_string()));
        assert_eq!(map.count(), 2);
    }
    #[test]
    fn test_arena_map_remove() {
        let mut arena: Arena<u32> = Arena::new();
        let idx = arena.alloc(5);
        let mut map: ArenaMap<u32, String> = ArenaMap::new();
        map.insert(idx, "five".to_string());
        let removed = map.remove(idx);
        assert_eq!(removed, Some("five".to_string()));
        assert!(!map.contains(idx));
    }
    #[test]
    fn test_arena_index_operator() {
        let mut arena: Arena<u32> = Arena::new();
        let idx = arena.alloc(99);
        assert_eq!(arena[idx], 99);
    }
    #[test]
    fn test_interning_arena_contains() {
        let mut arena: InterningArena<u32> = InterningArena::new();
        arena.intern(42);
        assert!(arena.contains(&42));
        assert!(!arena.contains(&99));
    }
    #[test]
    fn test_slab_live_iter() {
        let mut slab: SlabArena<u32> = SlabArena::new();
        let i1 = slab.alloc(1);
        let i2 = slab.alloc(2);
        let _i3 = slab.alloc(3);
        slab.free(i2);
        let live: Vec<_> = slab.iter_live().collect();
        assert_eq!(live.len(), 2);
        assert!(live.iter().all(|(_, &v)| v != 2));
        let _ = i1;
    }
}
#[cfg(test)]
mod tests_arena_extra {
    use super::*;
    #[test]
    fn test_linear_arena() {
        let mut arena = LinearArena::new(1024);
        let a = arena.alloc(16, 8).expect("a should be present");
        let b = arena.alloc(32, 8).expect("b should be present");
        assert!(b >= a + 16);
        assert_eq!(arena.stats().alloc_count, 2);
        arena.reset();
        assert_eq!(arena.used(), 0);
    }
    #[test]
    fn test_linear_arena_oom() {
        let mut arena = LinearArena::new(16);
        assert!(arena.alloc(17, 1).is_none());
    }
    #[test]
    fn test_chained_arena() {
        let mut arena = ChainedArena::new(64);
        for _ in 0..10 {
            arena.alloc(10, 1);
        }
        assert!(arena.num_blocks() >= 2);
        assert!(arena.total_allocated() >= 100);
    }
    #[test]
    fn test_pool_arena() {
        let mut pool = PoolArena::new(8, 4);
        assert_eq!(pool.available(), 4);
        let idx = pool.alloc_slot().expect("idx should be present");
        assert_eq!(pool.available(), 3);
        pool.free_slot(idx);
        assert_eq!(pool.available(), 4);
    }
    #[test]
    fn test_typed_arena() {
        let mut ta: TypedArena<u64> = TypedArena::new();
        let r = ta.alloc(42u64);
        assert_eq!(*r, 42);
        assert_eq!(ta.len(), 1);
        ta.clear();
        assert!(ta.is_empty());
    }
    #[test]
    fn test_scoped_arena() {
        let mut sa = ScopedArenaExt::new(256);
        sa.push_scope();
        sa.alloc(10, 1);
        assert_eq!(sa.scope_depth(), 1);
        sa.pop_scope();
        assert_eq!(sa.inner.used(), 0);
    }
    #[test]
    fn test_arena_checkpoint() {
        let mut arena = LinearArena::new(256);
        arena.alloc(16, 1);
        let cp = ArenaCheckpoint::create(&arena);
        arena.alloc(32, 1);
        assert_eq!(arena.used(), 48);
        cp.restore(&mut arena);
        assert_eq!(arena.used(), 16);
    }
    #[test]
    fn test_arena_string() {
        let mut arena = LinearArena::new(256);
        let s = ArenaString::store(&mut arena, "hello").expect("s should be present");
        assert_eq!(s.len(), 5);
        assert_eq!(arena.buf[s.offset + 5], 0);
    }
    #[test]
    fn test_arena_stats_fragmentation() {
        let mut stats = ArenaStatsExt::new();
        stats.bytes_allocated = 80;
        stats.wasted_bytes = 20;
        assert!((stats.fragmentation() - 0.2).abs() < 1e-9);
    }
}
#[cfg(test)]
mod tests_arena_extra2 {
    use super::*;
    #[test]
    fn test_growable_arena() {
        let mut ga = GrowableArena::new(8);
        ga.alloc(100, 1);
        assert!(ga.data.len() >= 100);
        assert_eq!(ga.count(), 1);
        ga.reset();
        assert_eq!(ga.used(), 0);
    }
    #[test]
    fn test_two_generation_arena() {
        let mut tga = TwoGenerationArena::new(64, 256);
        tga.alloc_nursery(10, 1);
        tga.alloc_nursery(10, 1);
        tga.promote(10, 1);
        assert_eq!(tga.num_promotions(), 1);
        tga.minor_gc();
        assert_eq!(tga.nursery.used(), 0);
        assert!(tga.stable.used() > 0);
    }
}
/// A trait for types that provide arena-style allocation.
#[allow(dead_code)]
pub trait ArenaAllocator {
    /// Allocates `bytes` bytes with `align` alignment.
    /// Returns `Some(offset)` on success or `None` on failure.
    fn alloc_raw(&mut self, bytes: usize, align: usize) -> Option<usize>;
    /// Returns the number of bytes currently used.
    fn used_bytes(&self) -> usize;
}
/// Allocates `count` objects of type `T` in any arena.
#[allow(dead_code)]
pub fn arena_alloc_array<A: ArenaAllocator>(arena: &mut A, count: usize) -> Option<usize> {
    let size = std::mem::size_of::<u64>() * count;
    let align = std::mem::align_of::<u64>();
    arena.alloc_raw(size, align)
}
#[cfg(test)]
mod tests_arena_trait {
    use super::*;
    #[test]
    fn test_arena_allocator_trait() {
        let mut la = LinearArena::new(256);
        let offset = arena_alloc_array(&mut la, 4).expect("offset should be present");
        assert!(offset + 32 <= la.used());
        let mut ga = GrowableArena::new(16);
        let _offset2 = arena_alloc_array(&mut ga, 8).expect("_offset2 should be present");
        assert!(ga.used() >= 64);
    }
}
#[cfg(test)]
mod tests_memory_region {
    use super::*;
    #[test]
    fn test_memory_region() {
        let mut r = MemoryRegion::new("heap", 0x1000, 0x4000);
        assert!(!r.active);
        r.activate();
        assert!(r.active);
        assert!(r.contains(0x2000));
        assert!(!r.contains(0x5000));
        assert_eq!(r.end(), 0x5000);
    }
    #[test]
    fn test_memory_region_registry() {
        let mut reg = MemoryRegionRegistry::new();
        reg.add(MemoryRegion::new("stack", 0x0000, 0x1000));
        reg.add(MemoryRegion::new("heap", 0x1000, 0x4000));
        assert_eq!(reg.len(), 2);
        assert!(reg.find(0x2000).is_some());
        assert_eq!(
            reg.find(0x2000).expect("value should be present").label,
            "heap"
        );
    }
}
#[cfg(test)]
mod tests_common_infra {
    use super::*;
    #[test]
    fn test_event_counter() {
        let mut ec = EventCounter::new();
        ec.inc("hit");
        ec.inc("hit");
        ec.inc("miss");
        assert_eq!(ec.get("hit"), 2);
        assert_eq!(ec.get("miss"), 1);
        assert_eq!(ec.total(), 3);
        ec.reset();
        assert_eq!(ec.total(), 0);
    }
    #[test]
    fn test_diag_meta() {
        let mut m = DiagMeta::new();
        m.add("os", "linux");
        m.add("arch", "x86_64");
        assert_eq!(m.get("os"), Some("linux"));
        assert_eq!(m.len(), 2);
        let s = m.to_string();
        assert!(s.contains("os=linux"));
    }
    #[test]
    fn test_scope_stack() {
        let mut ss = ScopeStack::new();
        ss.push("Nat");
        ss.push("succ");
        assert_eq!(ss.current(), Some("succ"));
        assert_eq!(ss.depth(), 2);
        assert_eq!(ss.path(), "Nat.succ");
        ss.pop();
        assert_eq!(ss.current(), Some("Nat"));
    }
    #[test]
    fn test_annotation_table() {
        let mut tbl = AnnotationTable::new();
        tbl.annotate("doc", "first line");
        tbl.annotate("doc", "second line");
        assert_eq!(tbl.get_all("doc").len(), 2);
        assert!(tbl.has("doc"));
        assert!(!tbl.has("other"));
    }
    #[test]
    fn test_work_stack() {
        let mut ws = WorkStack::new();
        ws.push(1u32);
        ws.push(2u32);
        assert_eq!(ws.pop(), Some(2));
        assert_eq!(ws.len(), 1);
    }
    #[test]
    fn test_work_queue() {
        let mut wq = WorkQueue::new();
        wq.enqueue(1u32);
        wq.enqueue(2u32);
        assert_eq!(wq.dequeue(), Some(1));
        assert_eq!(wq.len(), 1);
    }
    #[test]
    fn test_sparse_bit_set() {
        let mut bs = SparseBitSet::new(128);
        bs.set(5);
        bs.set(63);
        bs.set(64);
        assert!(bs.get(5));
        assert!(bs.get(63));
        assert!(bs.get(64));
        assert!(!bs.get(0));
        assert_eq!(bs.count_ones(), 3);
        bs.clear(5);
        assert!(!bs.get(5));
    }
    #[test]
    fn test_loop_clock() {
        let mut clk = LoopClock::start();
        for _ in 0..10 {
            clk.tick();
        }
        assert_eq!(clk.iters(), 10);
        assert!(clk.elapsed_us() >= 0.0);
    }
}
#[cfg(test)]
mod tests_extra_data_structures {
    use super::*;
    #[test]
    fn test_simple_lru_cache() {
        let mut cache: SimpleLruCache<&str, u32> = SimpleLruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("d", 4);
        assert!(cache.len() <= 3);
    }
    #[test]
    fn test_string_interner() {
        let mut si = StringInterner::new();
        let id1 = si.intern("hello");
        let id2 = si.intern("hello");
        assert_eq!(id1, id2);
        let id3 = si.intern("world");
        assert_ne!(id1, id3);
        assert_eq!(si.get(id1), Some("hello"));
        assert_eq!(si.len(), 2);
    }
    #[test]
    fn test_frequency_table() {
        let mut ft = FrequencyTable::new();
        ft.record("a");
        ft.record("b");
        ft.record("a");
        ft.record("a");
        assert_eq!(ft.freq(&"a"), 3);
        assert_eq!(ft.freq(&"b"), 1);
        assert_eq!(ft.most_frequent(), Some((&"a", 3)));
        assert_eq!(ft.total(), 4);
        assert_eq!(ft.distinct(), 2);
    }
    #[test]
    fn test_bimap() {
        let mut bm: BiMap<u32, &str> = BiMap::new();
        bm.insert(1, "one");
        bm.insert(2, "two");
        assert_eq!(bm.get_b(&1), Some(&"one"));
        assert_eq!(bm.get_a(&"two"), Some(&2));
        assert_eq!(bm.len(), 2);
    }
}
#[cfg(test)]
mod tests_interval_set {
    use super::*;
    #[test]
    fn test_interval_set() {
        let mut s = IntervalSet::new();
        s.add(1, 5);
        s.add(3, 8);
        assert_eq!(s.num_intervals(), 1);
        assert_eq!(s.cardinality(), 8);
        assert!(s.contains(4));
        assert!(!s.contains(9));
        s.add(10, 15);
        assert_eq!(s.num_intervals(), 2);
    }
}
/// Returns the current timestamp.
#[allow(dead_code)]
pub fn now_us() -> Timestamp {
    let us = std::time::SystemTime::UNIX_EPOCH
        .elapsed()
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0);
    Timestamp::from_us(us)
}
#[cfg(test)]
mod tests_typed_utilities {
    use super::*;
    #[test]
    fn test_timestamp() {
        let t1 = Timestamp::from_us(1000);
        let t2 = Timestamp::from_us(1500);
        assert_eq!(t2.elapsed_since(t1), 500);
        assert!(t1 < t2);
    }
    #[test]
    fn test_typed_id() {
        struct Foo;
        let id: TypedId<Foo> = TypedId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(format!("{id}"), "#42");
    }
    #[test]
    fn test_id_dispenser() {
        struct Bar;
        let mut disp: IdDispenser<Bar> = IdDispenser::new();
        let a = disp.next();
        let b = disp.next();
        assert_eq!(a.raw(), 0);
        assert_eq!(b.raw(), 1);
        assert_eq!(disp.count(), 2);
    }
    #[test]
    fn test_slot() {
        let mut slot: Slot<u32> = Slot::empty();
        assert!(!slot.is_filled());
        slot.fill(99);
        assert!(slot.is_filled());
        assert_eq!(slot.get(), Some(&99));
        let v = slot.take();
        assert_eq!(v, Some(99));
        assert!(!slot.is_filled());
    }
    #[test]
    #[should_panic]
    fn test_slot_double_fill() {
        let mut slot: Slot<u32> = Slot::empty();
        slot.fill(1);
        slot.fill(2);
    }
    #[test]
    fn test_memo_slot() {
        let mut ms: MemoSlot<u32> = MemoSlot::new();
        assert!(!ms.is_cached());
        let val = ms.get_or_compute(|| 42);
        assert_eq!(*val, 42);
        assert!(ms.is_cached());
        ms.invalidate();
        assert!(!ms.is_cached());
    }
}
