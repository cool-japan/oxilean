//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Level, Literal, Name};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::types::{
    BitSet64, BucketCounter, CacheSessionStats, ConfigNode, DecisionNode, Either2, EvictionTracker,
    ExprHashcons, ExprId, ExprPool, Fixture, FlatSubstitution, FocusStack, InvalidationSet,
    LabelSet, MemoTable, MinHeap, NonEmptyVec, PathBuf, PathCache, PrefixCounter, RcExprPool,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    TwoLevelCache, VersionedCache, VersionedRecord, WindowIterator, WriteOnce,
};

/// Helper: write a discriminant tag into a hasher.
pub(super) fn hash_tag(state: &mut impl Hasher, tag: u8) {
    state.write_u8(tag);
}
/// Helper: hash a `Level` into a hasher (Level already derives Hash).
pub(super) fn hash_level(level: &Level, state: &mut impl Hasher) {
    level.hash(state);
}
/// Helper: hash a `Name` into a hasher (Name already derives Hash).
pub(super) fn hash_name(name: &Name, state: &mut impl Hasher) {
    name.hash(state);
}
/// Helper: hash a `BinderInfo` into a hasher.
pub(super) fn hash_binder_info(bi: &BinderInfo, state: &mut impl Hasher) {
    bi.hash(state);
}
/// Helper: hash a `Literal` into a hasher.
pub(super) fn hash_literal(lit: &Literal, state: &mut impl Hasher) {
    lit.hash(state);
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Level, Name};
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop_expr() -> Expr {
        Expr::Sort(Level::Zero)
    }
    fn type1_expr() -> Expr {
        Expr::Sort(Level::succ(Level::Zero))
    }
    fn bvar0() -> Expr {
        Expr::BVar(0)
    }
    #[test]
    fn test_intern_same_expr_twice() {
        let mut hc = ExprHashcons::new();
        let (id1, was_new1) = hc.intern(nat_expr());
        let (id2, was_new2) = hc.intern(nat_expr());
        assert!(was_new1, "first intern should be new");
        assert!(!was_new2, "second intern of same expr should be a hit");
        assert_eq!(id1, id2, "same expr must yield same ExprId");
    }
    #[test]
    fn test_intern_different_exprs() {
        let mut hc = ExprHashcons::new();
        let (id1, _) = hc.intern(nat_expr());
        let (id2, _) = hc.intern(prop_expr());
        let (id3, _) = hc.intern(bvar0());
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
        assert_eq!(hc.size(), 3);
    }
    #[test]
    fn test_hit_rate_empty() {
        let hc = ExprHashcons::new();
        assert_eq!(hc.hit_rate(), 0.0, "empty table hit rate should be 0.0");
    }
    #[test]
    fn test_hit_rate_with_duplicates() {
        let mut hc = ExprHashcons::new();
        hc.intern(nat_expr());
        hc.intern(nat_expr());
        hc.intern(nat_expr());
        hc.intern(prop_expr());
        let rate = hc.hit_rate();
        assert!(
            (rate - 0.5).abs() < 1e-9,
            "expected hit rate 0.5, got {}",
            rate
        );
    }
    #[test]
    fn test_pool_add_root() {
        let mut pool = ExprPool::new();
        let id = pool.add_root(nat_expr());
        assert_eq!(pool.live_count(), 1);
        assert_eq!(pool.total_count(), 1);
        assert_eq!(pool.get(id), Some(&nat_expr()));
    }
    #[test]
    fn test_pool_live_count() {
        let mut pool = ExprPool::new();
        let id1 = pool.add_root(nat_expr());
        let _id2 = pool.add(prop_expr());
        let id3 = pool.add_root(type1_expr());
        assert_eq!(pool.live_count(), 2);
        assert_eq!(pool.total_count(), 3);
        let prop_id = pool
            .get_id(&prop_expr())
            .expect("prop_id should be present");
        pool.mark_root(prop_id);
        assert_eq!(pool.live_count(), 3);
        pool.mark_root(id1);
        pool.mark_root(id3);
        assert_eq!(pool.live_count(), 3);
    }
    #[test]
    fn test_dedup_ratio() {
        let mut pool = ExprPool::new();
        pool.add(nat_expr());
        pool.add(nat_expr());
        pool.add(nat_expr());
        let ratio = pool.dedup_ratio();
        assert!(
            (ratio - 2.0 / 3.0).abs() < 1e-9,
            "expected dedup_ratio ~0.666, got {}",
            ratio
        );
    }
    #[test]
    fn test_get_by_id() {
        let mut hc = ExprHashcons::new();
        let e = Expr::BVar(42);
        let (id, _) = hc.intern(e.clone());
        let retrieved = hc.get(id);
        assert_eq!(retrieved, Some(&e));
        let bad_id = ExprId(9999);
        assert_eq!(hc.get(bad_id), None);
    }
    #[test]
    fn test_get_id_lookup() {
        let mut hc = ExprHashcons::new();
        let (id, _) = hc.intern(nat_expr());
        let found = hc.get_id(&nat_expr());
        assert_eq!(found, Some(id));
        let unknown = hc.get_id(&prop_expr());
        assert_eq!(unknown, None);
    }
}
#[cfg(test)]
mod tests_cache_extended {
    use super::*;
    #[test]
    fn test_eviction_tracker_lru_mru() {
        let mut t = EvictionTracker::new(3);
        t.access(1);
        t.access(2);
        t.access(3);
        assert_eq!(t.lru(), Some(1));
        assert_eq!(t.mru(), Some(3));
        t.access(1);
        assert_eq!(t.lru(), Some(2));
        assert_eq!(t.mru(), Some(1));
    }
    #[test]
    fn test_eviction_tracker_capacity() {
        let mut t = EvictionTracker::new(2);
        t.access(10);
        t.access(20);
        t.access(30);
        assert_eq!(t.len(), 2);
        assert_eq!(t.lru(), Some(20));
    }
    #[test]
    fn test_memo_table_insert_get_remove() {
        let mut m = MemoTable::new();
        m.insert(100, 999);
        assert_eq!(m.get(100), Some(999));
        m.insert(100, 1000);
        assert_eq!(m.get(100), Some(1000));
        let old = m.remove(100);
        assert_eq!(old, Some(1000));
        assert_eq!(m.get(100), None);
    }
    #[test]
    fn test_cache_session_stats() {
        let mut s = CacheSessionStats::new();
        s.hits = 80;
        s.misses = 20;
        assert!((s.hit_rate() - 0.8).abs() < 1e-9);
        let summary = s.summary();
        assert!(summary.contains("hit_rate=80.0%"));
    }
    #[test]
    fn test_two_level_cache() {
        let mut cache = TwoLevelCache::new(2);
        cache.insert(1, 100);
        cache.insert(2, 200);
        assert_eq!(cache.get(1), Some(100));
        assert_eq!(cache.get(2), Some(200));
        assert_eq!(cache.get(99), None);
        cache.insert(3, 300);
        assert_eq!(cache.total_len(), 3);
    }
    #[test]
    fn test_path_cache() {
        let mut pc = PathCache::new();
        pc.insert(&[1, 2, 3], 42);
        pc.insert(&[1, 2, 4], 43);
        pc.insert(&[5], 99);
        assert_eq!(pc.get(&[1, 2, 3]), Some(42));
        assert_eq!(pc.get(&[1, 2, 4]), Some(43));
        assert_eq!(pc.get(&[5]), Some(99));
        assert_eq!(pc.get(&[1, 2]), None);
        assert_eq!(pc.get(&[6]), None);
    }
    #[test]
    fn test_versioned_cache() {
        let mut vc = VersionedCache::new();
        vc.insert(10, 100);
        assert_eq!(vc.get(10), Some(100));
        vc.bump_version();
        assert_eq!(vc.get(10), None);
        vc.insert(10, 200);
        assert_eq!(vc.get(10), Some(200));
        vc.evict_stale();
        assert_eq!(vc.valid_count(), 1);
    }
    #[test]
    fn test_rc_expr_pool_gc() {
        let mut pool = RcExprPool::new();
        let i1 = pool.alloc(111);
        let _i2 = pool.alloc(222);
        pool.inc_ref(i1);
        pool.dec_ref(i1);
        pool.dec_ref(i1);
        let dead = pool.collect_garbage();
        assert!(dead.contains(&i1));
        assert_eq!(pool.live_count(), 1);
    }
    #[test]
    fn test_invalidation_set() {
        let mut inv = InvalidationSet::new();
        inv.add(1);
        inv.add(2);
        inv.add(1);
        assert_eq!(inv.len(), 2);
        assert!(inv.contains(1));
        assert!(!inv.contains(99));
        inv.clear();
        assert!(inv.is_empty());
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
#[cfg(test)]
mod tests_tiny_padding {
    use super::*;
    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::new();
        bs.insert(0);
        bs.insert(63);
        assert!(bs.contains(0));
        assert!(bs.contains(63));
        assert!(!bs.contains(1));
        assert_eq!(bs.len(), 2);
        bs.remove(0);
        assert!(!bs.contains(0));
    }
    #[test]
    fn test_bucket_counter() {
        let mut bc: BucketCounter<4> = BucketCounter::new();
        bc.inc(0);
        bc.inc(0);
        bc.inc(1);
        assert_eq!(bc.get(0), 2);
        assert_eq!(bc.total(), 3);
        assert_eq!(bc.argmax(), 0);
    }
}
