//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    BloomFilterApprox, CacheManager, ConfigNode, DecisionNode, DefEqCache, Either2,
    FlatSubstitution, FocusStack, InferCache, LabelSet, LruCache, MultiLevelCache, NonEmptyVec,
    PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SimplifiedExpr, SlidingSum, SmallMap,
    SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, TtlCache, VersionedRecord, WhnfCache, WindowIterator, WriteOnce,
};

/// FNV-1a hash function (no external dependencies)
/// Inline implementation for u64 hash generation
pub fn fnv1a_hash<T: AsRef<[u8]>>(data: T) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let bytes = data.as_ref();
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fnv1a_hash_basic() {
        let hash1 = fnv1a_hash("test");
        let hash2 = fnv1a_hash("test");
        assert_eq!(hash1, hash2, "Same input should produce same hash");
    }
    #[test]
    fn test_fnv1a_hash_different() {
        let hash1 = fnv1a_hash("test1");
        let hash2 = fnv1a_hash("test2");
        assert_ne!(
            hash1, hash2,
            "Different inputs should produce different hashes"
        );
    }
    #[test]
    fn test_fnv1a_hash_consistency() {
        let data = b"consistency_test";
        let hash1 = fnv1a_hash(data);
        let hash2 = fnv1a_hash(data);
        assert_eq!(hash1, hash2);
    }
    #[test]
    fn test_lru_cache_new() {
        let cache: LruCache<String, i32> = LruCache::new(10);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.capacity(), 10);
        assert!(cache.is_empty());
    }
    #[test]
    fn test_lru_cache_insert_and_get() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_lru_cache_get_nonexistent() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        assert_eq!(cache.get(&"nonexistent".to_string()), None);
    }
    #[test]
    fn test_lru_cache_contains_key() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("key1".to_string(), 42);
        assert!(cache.contains_key(&"key1".to_string()));
        assert!(!cache.contains_key(&"key2".to_string()));
    }
    #[test]
    fn test_lru_cache_eviction() {
        let mut cache: LruCache<String, i32> = LruCache::new(3);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3);
        assert_eq!(cache.len(), 3);
        cache.insert("d".to_string(), 4);
        assert_eq!(cache.len(), 3);
        assert!(!cache.contains_key(&"a".to_string()));
        assert!(cache.contains_key(&"d".to_string()));
    }
    #[test]
    fn test_lru_cache_update_moves_to_head() {
        let mut cache: LruCache<String, i32> = LruCache::new(3);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.insert("c".to_string(), 3);
        cache.get(&"a".to_string());
        cache.insert("d".to_string(), 4);
        assert!(cache.contains_key(&"a".to_string()));
        assert!(!cache.contains_key(&"b".to_string()));
    }
    #[test]
    fn test_lru_cache_remove() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.remove(&"key1".to_string()), Some(42));
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
    }
    #[test]
    fn test_lru_cache_clear() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
    #[test]
    fn test_lru_cache_stats() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("a".to_string(), 1);
        cache.get(&"a".to_string());
        cache.get(&"b".to_string());
        let (hits, misses) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }
    #[test]
    fn test_lru_cache_hit_rate() {
        let mut cache: LruCache<String, i32> = LruCache::new(10);
        cache.insert("a".to_string(), 1);
        cache.get(&"a".to_string());
        cache.get(&"a".to_string());
        cache.get(&"b".to_string());
        let hit_rate = cache.hit_rate();
        assert!((hit_rate - 66.666666).abs() < 0.01);
    }
    #[test]
    fn test_expr_hash_consistency() {
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("x".to_string());
        assert_eq!(expr1.hash(), expr2.hash());
    }
    #[test]
    fn test_expr_hash_different() {
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("y".to_string());
        assert_ne!(expr1.hash(), expr2.hash());
    }
    #[test]
    fn test_expr_hash_complex() {
        let expr1 = SimplifiedExpr::App(
            Box::new(SimplifiedExpr::Var("f".to_string())),
            Box::new(SimplifiedExpr::Var("x".to_string())),
        );
        let expr2 = SimplifiedExpr::App(
            Box::new(SimplifiedExpr::Var("f".to_string())),
            Box::new(SimplifiedExpr::Var("x".to_string())),
        );
        assert_eq!(expr1.hash(), expr2.hash());
    }
    #[test]
    fn test_whnf_cache_new() {
        let cache = WhnfCache::new(10, false);
        assert!(!cache.is_transparent());
    }
    #[test]
    fn test_whnf_cache_lookup_store() {
        let mut cache = WhnfCache::new(10, false);
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        cache.store(&expr, whnf.clone());
        let result = cache.lookup(&expr);
        assert_eq!(result, Some(whnf));
    }
    #[test]
    fn test_whnf_cache_transparency_mode() {
        let mut cache = WhnfCache::new(10, false);
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        cache.store(&expr, whnf);
        cache.set_transparency(true);
        let result = cache.lookup(&expr);
        assert_eq!(
            result, None,
            "Transparent mode should not return cached values"
        );
    }
    #[test]
    fn test_whnf_cache_stats() {
        let mut cache = WhnfCache::new(10, false);
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        cache.store(&expr, whnf);
        cache.lookup(&expr);
        cache.lookup(&expr);
        let (hits, misses) = cache.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 0);
    }
    #[test]
    fn test_whnf_cache_clear() {
        let mut cache = WhnfCache::new(10, false);
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        cache.store(&expr, whnf);
        cache.clear();
        let result = cache.lookup(&expr);
        assert_eq!(result, None);
    }
    #[test]
    fn test_defeq_cache_new() {
        let cache = DefEqCache::new(10);
        let (hits, misses) = cache.stats();
        assert_eq!(hits, 0);
        assert_eq!(misses, 0);
    }
    #[test]
    fn test_defeq_cache_store_and_check() {
        let mut cache = DefEqCache::new(10);
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("y".to_string());
        cache.store_result(&expr1, &expr2, true);
        let result = cache.check_cache(&expr1, &expr2);
        assert_eq!(result, Some(true));
    }
    #[test]
    fn test_defeq_cache_symmetry() {
        let mut cache = DefEqCache::new(10);
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("y".to_string());
        cache.store_result(&expr1, &expr2, true);
        let result = cache.check_cache(&expr2, &expr1);
        assert_eq!(result, Some(true), "DefEq cache should be symmetry-aware");
    }
    #[test]
    fn test_defeq_cache_false_result() {
        let mut cache = DefEqCache::new(10);
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("y".to_string());
        cache.store_result(&expr1, &expr2, false);
        let result = cache.check_cache(&expr1, &expr2);
        assert_eq!(result, Some(false));
    }
    #[test]
    fn test_defeq_cache_clear() {
        let mut cache = DefEqCache::new(10);
        let expr1 = SimplifiedExpr::Var("x".to_string());
        let expr2 = SimplifiedExpr::Var("y".to_string());
        cache.store_result(&expr1, &expr2, true);
        cache.clear();
        let result = cache.check_cache(&expr1, &expr2);
        assert_eq!(result, None);
    }
    #[test]
    fn test_infer_cache_new() {
        let cache = InferCache::new(10);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_infer_cache_store_and_lookup() {
        let mut cache = InferCache::new(10);
        let expr = SimplifiedExpr::Var("x".to_string());
        let inferred_type = SimplifiedExpr::Var("Type".to_string());
        cache.store(&expr, inferred_type.clone());
        let result = cache.lookup(&expr);
        assert_eq!(result, Some(inferred_type));
    }
    #[test]
    fn test_infer_cache_stats() {
        let mut cache = InferCache::new(10);
        let expr = SimplifiedExpr::Var("x".to_string());
        let inferred_type = SimplifiedExpr::Var("Type".to_string());
        cache.store(&expr, inferred_type);
        cache.lookup(&expr);
        cache.lookup(&expr);
        let (hits, misses) = cache.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 0);
    }
    #[test]
    fn test_infer_cache_clear() {
        let mut cache = InferCache::new(10);
        let expr = SimplifiedExpr::Var("x".to_string());
        let inferred_type = SimplifiedExpr::Var("Type".to_string());
        cache.store(&expr, inferred_type);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_cache_manager_new() {
        let manager = CacheManager::new();
        let stats = manager.statistics();
        assert_eq!(stats.total_hits(), 0);
        assert_eq!(stats.total_misses(), 0);
    }
    #[test]
    fn test_cache_manager_with_capacities() {
        let manager = CacheManager::with_capacities(512, 256, 128);
        assert!(manager.whnf.cache.capacity() >= 512);
        assert!(manager.defeq.cache.capacity() >= 256);
        assert!(manager.infer.cache.capacity() >= 128);
    }
    #[test]
    fn test_cache_manager_clear_all() {
        let mut manager = CacheManager::new();
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        manager.whnf_mut().store(&expr, whnf);
        manager.clear_all();
        let result = manager.whnf_mut().lookup(&expr);
        assert_eq!(result, None);
    }
    #[test]
    fn test_cache_manager_statistics() {
        let mut manager = CacheManager::new();
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        manager.whnf_mut().store(&expr, whnf);
        manager.whnf_mut().lookup(&expr);
        let stats = manager.statistics();
        assert_eq!(stats.whnf_hits, 1);
    }
    #[test]
    fn test_cache_statistics_overall_hit_rate() {
        let mut manager = CacheManager::new();
        let expr = SimplifiedExpr::Var("x".to_string());
        let whnf = SimplifiedExpr::Var("y".to_string());
        manager.whnf_mut().store(&expr, whnf.clone());
        manager.whnf_mut().lookup(&expr);
        manager.whnf_mut().lookup(&expr);
        let stats = manager.statistics();
        assert!(stats.overall_hit_rate() > 50.0);
    }
    #[test]
    fn test_cache_manager_default() {
        let _manager = CacheManager::default();
    }
}
#[cfg(test)]
mod cache_extra_tests {
    use super::*;
    #[test]
    fn test_ttl_cache_insert_get() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(5);
        cache.insert("a".to_string(), 42);
        assert_eq!(cache.get(&"a".to_string()), Some(42));
    }
    #[test]
    fn test_ttl_cache_expiry() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(3);
        cache.insert("a".to_string(), 1);
        cache.tick_n(3);
        assert_eq!(cache.get(&"a".to_string()), None);
    }
    #[test]
    fn test_ttl_cache_not_expired_yet() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(5);
        cache.insert("b".to_string(), 99);
        cache.tick_n(4);
        assert_eq!(cache.get(&"b".to_string()), Some(99));
    }
    #[test]
    fn test_ttl_cache_custom_ttl() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(10);
        cache.insert_with_ttl("x".to_string(), 7, 1);
        cache.tick();
        assert_eq!(cache.get(&"x".to_string()), None);
    }
    #[test]
    fn test_ttl_cache_purge_expired() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(2);
        cache.insert("a".to_string(), 1);
        cache.insert("b".to_string(), 2);
        cache.tick_n(3);
        cache.purge_expired();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_ttl_cache_clear() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(10);
        cache.insert("a".to_string(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_multi_level_cache_l1_hit() {
        let mut cache: MultiLevelCache<String, i32> = MultiLevelCache::new(4, 16);
        cache.insert("k".to_string(), 42);
        let v = cache.get(&"k".to_string());
        assert_eq!(v, Some(42));
        assert_eq!(cache.l1_hits(), 1);
    }
    #[test]
    fn test_multi_level_cache_l2_promotion() {
        let mut cache: MultiLevelCache<String, i32> = MultiLevelCache::new(2, 16);
        cache.insert_l2_only("k".to_string(), 99);
        let v = cache.get(&"k".to_string());
        assert_eq!(v, Some(99));
        assert_eq!(cache.l2_hits(), 1);
        let v2 = cache.get(&"k".to_string());
        assert_eq!(v2, Some(99));
        assert_eq!(cache.l1_hits(), 1);
    }
    #[test]
    fn test_multi_level_cache_miss() {
        let mut cache: MultiLevelCache<String, i32> = MultiLevelCache::new(4, 16);
        let v = cache.get(&"absent".to_string());
        assert!(v.is_none());
        assert_eq!(cache.misses(), 1);
    }
    #[test]
    fn test_multi_level_cache_clear_all() {
        let mut cache: MultiLevelCache<String, i32> = MultiLevelCache::new(4, 16);
        cache.insert("k".to_string(), 1);
        cache.get(&"k".to_string());
        cache.clear_all();
        assert_eq!(cache.total_requests(), 0);
        assert!(cache.get(&"k".to_string()).is_none());
    }
    #[test]
    fn test_bloom_filter_insert_might_contain() {
        let mut bf = BloomFilterApprox::new(256);
        bf.insert(b"hello");
        assert!(bf.might_contain(b"hello"));
    }
    #[test]
    fn test_bloom_filter_clear() {
        let mut bf = BloomFilterApprox::new(256);
        bf.insert(b"world");
        bf.clear();
        assert!(!bf.might_contain(b"world"));
    }
    #[test]
    fn test_bloom_filter_set_bit_count() {
        let mut bf = BloomFilterApprox::new(256);
        let before = bf.set_bit_count();
        bf.insert(b"test");
        let after = bf.set_bit_count();
        assert!(after >= before);
    }
    #[test]
    fn test_bloom_filter_size() {
        let bf = BloomFilterApprox::new(512);
        assert_eq!(bf.size(), 512);
    }
    #[test]
    fn test_multi_level_cache_hit_rate() {
        let mut cache: MultiLevelCache<String, i32> = MultiLevelCache::new(4, 16);
        cache.insert("a".to_string(), 1);
        cache.get(&"a".to_string());
        cache.get(&"z".to_string());
        let rate = cache.hit_rate();
        assert!((rate - 50.0).abs() < 0.01);
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
