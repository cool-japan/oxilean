//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ArcWeak, AtomicRefCounter, BorrowFlag, CowBox, GcTracer, OwnershipLog, Rc, RcBitmask,
    RcElisionAnalysis, RcElisionHint, RcEventKind, RcGraph, RcManager, RcObserver, RcPolicy,
    RcPool, RcStats, RefcountedMap, RetainRelease, RtArc, StickyRc, Weak, WeakTable,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rc_basic() {
        let rc = Rc::new(42);
        assert_eq!(*rc.as_ref(), 42);
        assert!(rc.is_unique());
        assert_eq!(rc.strong_count(), 1);
    }
    #[test]
    fn test_rc_get_mut() {
        let mut rc = Rc::new(42);
        if let Some(v) = rc.get_mut() {
            *v = 100;
        }
        assert_eq!(*rc.as_ref(), 100);
    }
    #[test]
    fn test_rc_try_unwrap() {
        let rc = Rc::new(42);
        let v = rc.try_unwrap().expect("try operation should succeed");
        assert_eq!(v, 42);
    }
    #[test]
    fn test_weak_reference() {
        let rc = Rc::new(42);
        let weak = Weak::from_rc(&rc);
        assert!(weak.is_alive());
        let upgraded = weak.upgrade().expect("test operation should succeed");
        assert_eq!(*upgraded.as_ref(), 42);
    }
    #[test]
    fn test_weak_invalidate() {
        let rc = Rc::new(42);
        let weak = Weak::from_rc(&rc);
        weak.invalidate();
        assert!(!weak.is_alive());
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn test_arc_basic() {
        let arc = RtArc::new(42);
        assert_eq!(*arc.as_ref(), 42);
        assert!(arc.is_unique());
    }
    #[test]
    fn test_arc_try_unwrap() {
        let arc = RtArc::new(42);
        let v = arc.try_unwrap().expect("try operation should succeed");
        assert_eq!(v, 42);
    }
    #[test]
    fn test_elision_hint() {
        let linear = RcElisionHint::LinearUse;
        assert!(linear.can_elide_inc());
        assert!(linear.can_elide_dec());
        assert!(linear.can_mutate_inplace());
        let none = RcElisionHint::None;
        assert!(!none.can_elide_inc());
        assert!(!none.can_elide_dec());
        assert!(!none.can_mutate_inplace());
    }
    #[test]
    fn test_elision_analysis() {
        let mut analysis = RcElisionAnalysis::new();
        analysis.add_hint("x".to_string(), RcElisionHint::LinearUse);
        analysis.add_hint("y".to_string(), RcElisionHint::SharedImmutable);
        assert_eq!(analysis.get_hint("x"), RcElisionHint::LinearUse);
        assert_eq!(analysis.get_hint("z"), RcElisionHint::None);
    }
    #[test]
    fn test_borrow_flag() {
        let flag = BorrowFlag::new();
        assert!(!flag.is_borrowed());
        assert!(flag.try_borrow());
        assert!(flag.is_borrowed());
        assert!(!flag.is_mutably_borrowed());
        assert!(flag.try_borrow());
        assert!(!flag.try_borrow_mut());
        flag.release_borrow();
        flag.release_borrow();
        assert!(!flag.is_borrowed());
        assert!(flag.try_borrow_mut());
        assert!(flag.is_mutably_borrowed());
        assert!(!flag.try_borrow());
        flag.release_borrow_mut();
        assert!(!flag.is_borrowed());
    }
    #[test]
    fn test_rc_stats() {
        let mut stats = RcStats::new();
        stats.record_inc();
        stats.record_inc();
        stats.record_dec();
        stats.record_elided_inc();
        assert_eq!(stats.total_ops(), 3);
        assert_eq!(stats.total_elided(), 1);
    }
    #[test]
    fn test_rc_manager() {
        let mut analysis = RcElisionAnalysis::new();
        analysis.add_hint("x".to_string(), RcElisionHint::LinearUse);
        analysis.add_hint("y".to_string(), RcElisionHint::None);
        let mut manager = RcManager::with_analysis(analysis);
        manager.inc("x");
        manager.inc("y");
        manager.dec("x");
        manager.dec("y");
        assert_eq!(manager.stats().elided_increments, 1);
        assert_eq!(manager.stats().increments, 1);
        assert_eq!(manager.stats().elided_decrements, 1);
        assert_eq!(manager.stats().decrements, 1);
    }
    #[test]
    fn test_cow_box() {
        let mut cow = CowBox::new(42);
        assert!(!cow.was_copied());
        assert_eq!(*cow.as_ref(), 42);
        *cow.as_mut() = 100;
        assert_eq!(*cow.as_ref(), 100);
    }
    #[test]
    fn test_cow_box_into_owned() {
        let cow = CowBox::new(42);
        let v = cow.into_owned();
        assert_eq!(v, 42);
    }
    #[test]
    fn test_rc_policy() {
        assert!(RcPolicy::Standard.is_enabled());
        assert!(!RcPolicy::Disabled.is_enabled());
        assert!(RcPolicy::Deferred.is_deferred());
        assert!(RcPolicy::AggressiveElision.allows_elision());
    }
    #[test]
    fn test_arc_weak() {
        let arc = RtArc::new(42);
        let weak = ArcWeak::from_arc(&arc);
        assert!(weak.is_alive());
        let upgraded = weak.upgrade().expect("test operation should succeed");
        assert_eq!(*upgraded.as_ref(), 42);
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_rc_pool_insert_get() {
        let mut pool: RcPool<u32> = RcPool::new();
        let idx = pool.insert(42);
        assert_eq!(pool.get(idx), Some(&42));
        assert_eq!(pool.refcount(idx), 1);
    }
    #[test]
    fn test_rc_pool_inc_dec_ref() {
        let mut pool: RcPool<u32> = RcPool::new();
        let idx = pool.insert(10);
        pool.inc_ref(idx);
        assert_eq!(pool.refcount(idx), 2);
        pool.dec_ref(idx);
        assert_eq!(pool.refcount(idx), 1);
        let freed = pool.dec_ref(idx);
        assert!(freed);
        assert_eq!(pool.get(idx), None);
    }
    #[test]
    fn test_rc_pool_cow() {
        let mut pool: RcPool<String> = RcPool::new();
        let idx = pool.insert("hello".to_string());
        pool.inc_ref(idx);
        assert_eq!(pool.refcount(idx), 2);
        let new_idx = pool.cow(idx).expect("creation should succeed");
        assert_ne!(idx, new_idx);
        assert_eq!(pool.refcount(idx), 1);
    }
    #[test]
    fn test_rc_pool_slot_reuse() {
        let mut pool: RcPool<u32> = RcPool::new();
        let idx1 = pool.insert(1);
        pool.dec_ref(idx1);
        let idx2 = pool.insert(2);
        assert_eq!(idx1, idx2);
    }
    #[test]
    fn test_rc_pool_get_mut_unique_only() {
        let mut pool: RcPool<u32> = RcPool::new();
        let idx = pool.insert(100);
        pool.inc_ref(idx);
        assert!(pool.get_mut(idx).is_none());
        pool.dec_ref(idx);
        let r = pool.get_mut(idx);
        assert!(r.is_some());
    }
    #[test]
    fn test_gc_tracer_basic_collect() {
        let mut gc = GcTracer::new();
        let root = gc.add_node(true);
        let live = gc.add_node(false);
        let dead = gc.add_node(false);
        gc.add_ref(root, live);
        let collected = gc.collect();
        assert!(collected.contains(&dead));
        assert!(!collected.contains(&live));
        assert!(!collected.contains(&root));
    }
    #[test]
    fn test_gc_tracer_all_roots() {
        let mut gc = GcTracer::new();
        gc.add_node(true);
        gc.add_node(true);
        let collected = gc.collect();
        assert!(collected.is_empty());
    }
    #[test]
    fn test_gc_tracer_live_count() {
        let mut gc = GcTracer::new();
        let root = gc.add_node(true);
        let child = gc.add_node(false);
        gc.add_node(false);
        gc.add_ref(root, child);
        gc.mark();
        assert_eq!(gc.live_count(), 2);
    }
    #[test]
    fn test_rc_observer_record() {
        let mut obs = RcObserver::new(100);
        obs.record(1, RcEventKind::Alloc, 1);
        obs.record(1, RcEventKind::Inc, 2);
        obs.record(1, RcEventKind::Dec, 1);
        obs.record(1, RcEventKind::Drop, 0);
        assert_eq!(obs.drop_count(), 1);
        assert_eq!(obs.alloc_count(), 1);
    }
    #[test]
    fn test_rc_observer_overflow() {
        let mut obs = RcObserver::new(3);
        for i in 0..5u64 {
            obs.record(i, RcEventKind::Inc, i);
        }
        assert_eq!(obs.events().len(), 3);
    }
    #[test]
    fn test_rc_observer_clear() {
        let mut obs = RcObserver::new(100);
        obs.record(0, RcEventKind::Alloc, 1);
        obs.clear();
        assert!(obs.events().is_empty());
    }
    #[test]
    fn test_rcmap_insert_get() {
        let mut map: RefcountedMap<String, u32> = RefcountedMap::new();
        map.insert("key".to_string(), 42);
        assert_eq!(map.get(&"key".to_string()), Some(&42));
        assert_eq!(map.refcount(&"key".to_string()), 1);
    }
    #[test]
    fn test_rcmap_inc_dec() {
        let mut map: RefcountedMap<u32, u32> = RefcountedMap::new();
        map.insert(1, 100);
        map.inc_ref(&1);
        assert_eq!(map.refcount(&1), 2);
        map.dec_ref(&1);
        assert_eq!(map.refcount(&1), 1);
        let dropped = map.dec_ref(&1);
        assert!(dropped);
        assert!(map.get(&1).is_none());
    }
    #[test]
    fn test_rcmap_len() {
        let mut map: RefcountedMap<u32, u32> = RefcountedMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        assert_eq!(map.len(), 2);
        map.dec_ref(&1);
        assert_eq!(map.len(), 1);
    }
}
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_rc_graph_add_node_edge() {
        let mut g = RcGraph::new();
        let a = g.add_node(1);
        let b = g.add_node(2);
        g.add_edge(a, b);
        assert_eq!(g.edge_count(), 1);
        assert_eq!(g.refcount(b), 2);
    }
    #[test]
    fn test_rc_graph_remove_edge() {
        let mut g = RcGraph::new();
        let a = g.add_node(0);
        let b = g.add_node(0);
        g.add_edge(a, b);
        let removed = g.remove_edge(a, b);
        assert!(removed);
        assert_eq!(g.refcount(b), 1);
        assert_eq!(g.edge_count(), 0);
    }
    #[test]
    fn test_rc_graph_remove_node() {
        let mut g = RcGraph::new();
        let a = g.add_node(0);
        let b = g.add_node(0);
        g.add_edge(a, b);
        g.remove_node(a);
        assert_eq!(g.node_count(), 1);
        assert_eq!(g.refcount(b), 1);
    }
    #[test]
    fn test_rc_graph_zero_refcount() {
        let mut g = RcGraph::new();
        let a = g.add_node(0);
        let b = g.add_node(0);
        g.add_edge(a, b);
        g.remove_node(a);
        assert!(g.zero_refcount_nodes().is_empty());
    }
    #[test]
    fn test_rc_graph_out_edges() {
        let mut g = RcGraph::new();
        let a = g.add_node(0);
        let b = g.add_node(0);
        let c = g.add_node(0);
        g.add_edge(a, b);
        g.add_edge(a, c);
        let edges = g.out_edges(a);
        assert_eq!(edges.len(), 2);
    }
    #[test]
    fn test_atomic_ref_counter_inc_dec() {
        let counter = AtomicRefCounter::new(1);
        assert_eq!(counter.inc(), 2);
        assert_eq!(counter.dec(), 1);
        assert_eq!(counter.dec(), 0);
    }
    #[test]
    fn test_atomic_ref_counter_saturating() {
        let counter = AtomicRefCounter::new(0);
        assert_eq!(counter.dec(), 0);
        assert!(counter.is_zero());
    }
    #[test]
    fn test_atomic_ref_counter_reset() {
        let counter = AtomicRefCounter::new(5);
        counter.reset(10);
        assert_eq!(counter.load(), 10);
    }
    #[test]
    fn test_atomic_ref_counter_display() {
        let counter = AtomicRefCounter::new(7);
        let s = format!("{}", counter);
        assert!(s.contains('7'));
    }
}
#[cfg(test)]
mod tests_extended3 {
    use super::*;
    #[test]
    fn test_weak_table_register_get() {
        let mut table: WeakTable<u32> = WeakTable::new();
        let arc = std::sync::Arc::new(42u32);
        let key = table.register(arc.clone());
        let retrieved = table.get(key);
        assert!(retrieved.is_some());
        assert_eq!(*retrieved.expect("test operation should succeed"), 42);
    }
    #[test]
    fn test_weak_table_expired() {
        let mut table: WeakTable<u32> = WeakTable::new();
        let arc = std::sync::Arc::new(99u32);
        let key = table.register(arc.clone());
        drop(arc);
        let result = table.get(key);
        assert!(result.is_none());
        assert_eq!(table.miss_count(), 1);
    }
    #[test]
    fn test_weak_table_prune() {
        let mut table: WeakTable<u32> = WeakTable::new();
        let arc1 = std::sync::Arc::new(1u32);
        let arc2 = std::sync::Arc::new(2u32);
        let _k1 = table.register(arc1.clone());
        let _k2 = table.register(arc2.clone());
        drop(arc2);
        let pruned = table.prune();
        assert_eq!(pruned, 1);
        assert_eq!(table.len(), 1);
    }
    #[test]
    fn test_rc_bitmask_basic() {
        let mut bm = RcBitmask::new();
        bm.set_alive(0);
        bm.set_alive(3);
        assert!(bm.is_alive(0));
        assert!(bm.is_alive(3));
        assert!(!bm.is_alive(1));
        assert_eq!(bm.alive_count(), 2);
    }
    #[test]
    fn test_rc_bitmask_first_dead() {
        let mut bm = RcBitmask::new();
        bm.set_alive(0);
        bm.set_alive(1);
        assert_eq!(bm.first_dead(), Some(2));
    }
    #[test]
    fn test_rc_bitmask_first_alive() {
        let mut bm = RcBitmask::new();
        bm.set_alive(5);
        assert_eq!(bm.first_alive(), Some(5));
    }
    #[test]
    fn test_rc_bitmask_set_dead() {
        let mut bm = RcBitmask::new();
        bm.set_alive(2);
        bm.set_dead(2);
        assert!(!bm.is_alive(2));
    }
    #[test]
    fn test_rc_bitmask_display() {
        let bm = RcBitmask::new();
        let s = format!("{}", bm);
        assert!(s.contains("0x"));
    }
    #[test]
    fn test_retain_release_basic() {
        let mut rr = RetainRelease::new(42u32);
        assert_eq!(rr.live_count(), 1);
        rr.retain();
        assert_eq!(rr.live_count(), 2);
        rr.release();
        assert_eq!(rr.live_count(), 1);
        let should_drop = rr.release();
        assert!(should_drop);
        assert_eq!(rr.live_count(), 0);
    }
    #[test]
    fn test_retain_release_get_mut() {
        let mut rr = RetainRelease::new(10u32);
        *rr.get_mut() = 20;
        assert_eq!(*rr.get(), 20);
    }
    #[test]
    fn test_retain_release_counts() {
        let mut rr = RetainRelease::new(0u32);
        rr.retain();
        rr.retain();
        rr.release();
        assert_eq!(rr.retain_count(), 3);
        assert_eq!(rr.release_count(), 1);
    }
}
/// Compute the RC statistics for a list of counts.
#[allow(dead_code)]
pub fn rc_statistics(counts: &[u64]) -> (u64, u64, f64) {
    if counts.is_empty() {
        return (0, 0, 0.0);
    }
    let min = *counts.iter().min().expect("test operation should succeed");
    let max = *counts.iter().max().expect("test operation should succeed");
    let avg = counts.iter().sum::<u64>() as f64 / counts.len() as f64;
    (min, max, avg)
}
#[cfg(test)]
mod tests_ownership {
    use super::*;
    #[test]
    fn test_ownership_log() {
        let mut log = OwnershipLog::new(100);
        log.record_transfer(1, "main", "worker", 0);
        log.record_transfer(1, "worker", "main", 100);
        let events = log.events_for(1);
        assert_eq!(events.len(), 2);
    }
    #[test]
    fn test_rc_statistics() {
        let counts = vec![1, 2, 3, 4, 5];
        let (min, max, avg) = rc_statistics(&counts);
        assert_eq!(min, 1);
        assert_eq!(max, 5);
        assert!((avg - 3.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_rc_statistics_empty() {
        let (min, max, avg) = rc_statistics(&[]);
        assert_eq!(min, 0);
        assert_eq!(max, 0);
        assert_eq!(avg, 0.0);
    }
}
#[cfg(test)]
mod tests_sticky {
    use super::*;
    #[test]
    fn test_sticky_rc_saturate() {
        let mut rc = StickyRc::new(1, 3);
        rc.inc();
        rc.inc();
        rc.inc();
        assert!(rc.is_immortal());
        assert_eq!(rc.count(), 3);
    }
    #[test]
    fn test_sticky_rc_dec_to_zero() {
        let mut rc = StickyRc::new(1, 10);
        let dropped = rc.dec();
        assert!(dropped);
        assert!(rc.is_zero());
    }
    #[test]
    fn test_sticky_rc_display() {
        let rc = StickyRc::new(2, 10);
        let s = format!("{}", rc);
        assert!(s.contains("2/10"));
    }
}
