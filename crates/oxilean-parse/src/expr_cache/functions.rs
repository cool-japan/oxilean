//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheTier;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_interner_new() {
        let interner = StringInterner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.len(), 0);
    }
    #[test]
    fn test_intern_dedup() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern("Nat");
        let id2 = interner.intern("Nat");
        let id3 = interner.intern("Type");
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_eq!(interner.len(), 2);
    }
    #[test]
    fn test_intern_lookup() {
        let mut interner = StringInterner::new();
        let id = interner.intern("Prop");
        assert_eq!(interner.get(id), Some("Prop"));
        assert!(interner.contains("Prop"));
        assert!(!interner.contains("Bool"));
    }
    #[test]
    fn test_decl_hash_compute() {
        let h = DeclHash::compute("def foo : Nat := 0");
        assert_ne!(h.value(), 0);
    }
    #[test]
    fn test_decl_hash_same() {
        let h1 = DeclHash::compute("theorem bar : True := trivial");
        let h2 = DeclHash::compute("theorem bar : True := trivial");
        let h3 = DeclHash::compute("theorem baz : True := trivial");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_parse_cache_lookup() {
        let mut cache = ParseCache::new(10);
        cache.insert("def foo : Nat := 0", Some("foo".to_string()));
        assert!(cache.lookup("def foo : Nat := 0").is_some());
        assert!(cache.lookup("def bar : Nat := 1").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_parse_cache_insert_evict() {
        let mut cache = ParseCache::new(2);
        cache.insert("def a := 1", Some("a".to_string()));
        cache.insert("def b := 2", Some("b".to_string()));
        assert_eq!(cache.len(), 2);
        cache.insert("def c := 3", Some("c".to_string()));
        assert_eq!(cache.len(), 2);
    }
    #[test]
    fn test_hit_rate() {
        let mut cache = ParseCache::new(10);
        cache.insert("def x := 0", None);
        cache.lookup("def x := 0");
        cache.lookup("def x := 0");
        cache.lookup("def y := 1");
        let rate = cache.hit_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }
}
/// Deterministic FNV-1a hash.
#[allow(dead_code)]
pub fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for &b in data {
        h = h.wrapping_mul(1099511628211) ^ b as u64;
    }
    h
}
/// Mix two hashes.
#[allow(dead_code)]
pub fn mix_hashes(a: u64, b: u64) -> u64 {
    let mut x = a ^ b.rotate_left(17);
    x = x.wrapping_add(b);
    x ^= x >> 31;
    x = x.wrapping_mul(0x9e3779b97f4a7c15);
    x ^= x >> 27;
    x
}
/// Compute checksum.
#[allow(dead_code)]
pub fn compute_checksum(data: &str) -> u32 {
    let mut sum: u32 = 0;
    for (i, b) in data.bytes().enumerate() {
        sum = sum.wrapping_add((b as u32).wrapping_mul(i as u32 + 1));
    }
    sum
}
/// Validate cache integrity.
#[allow(dead_code)]
pub fn validate_cache_integrity(data: &str, stored: u32) -> bool {
    compute_checksum(data) == stored
}
/// Classify a cache entry into a tier based on access frequency.
#[allow(dead_code)]
pub fn classify_cache_entry(access_count: u64, age_ticks: u64) -> CacheTier {
    let score = if age_ticks == 0 {
        access_count as f64
    } else {
        access_count as f64 / age_ticks as f64
    };
    if score >= 1.0 {
        CacheTier::Hot
    } else if score >= 0.1 {
        CacheTier::Warm
    } else if score >= 0.01 {
        CacheTier::Cold
    } else {
        CacheTier::Dead
    }
}
#[cfg(test)]
mod extended_expr_cache_tests {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_fnv1a_hash_stable() {
        let h1 = fnv1a_hash(b"hello");
        let h2 = fnv1a_hash(b"hello");
        assert_eq!(h1, h2);
        assert_ne!(fnv1a_hash(b"hello"), fnv1a_hash(b"world"));
    }
    #[test]
    fn test_mix_hashes() {
        let a = fnv1a_hash(b"alpha");
        let b = fnv1a_hash(b"beta");
        let m = mix_hashes(a, b);
        assert_ne!(m, a);
        assert_ne!(m, b);
    }
    #[test]
    fn test_parse_result_cache() {
        let mut cache = ParseResultCache::new(10);
        cache.store("x + y", "App".to_string(), 42);
        assert!(cache.lookup("x + y").is_some());
        assert!(cache.lookup("unknown").is_none());
        assert_eq!(cache.stats().0, 1);
    }
    #[test]
    fn test_segment_table() {
        let src = "fun x -> x";
        let mut table = SegmentTable::new();
        let seg = ExprSegment::from_slice(src, 0, 5, SegmentKind::Lambda);
        table.add(seg);
        assert_eq!(table.count(), 1);
        table.invalidate_range(0, 5);
        assert_eq!(table.count(), 0);
    }
    #[test]
    fn test_subexpr_frequency_map() {
        let mut map = SubexprFrequencyMap::new();
        map.record(42);
        map.record(42);
        map.record(7);
        assert_eq!(map.frequency(42), 2);
        assert_eq!(map.total_unique(), 2);
    }
    #[test]
    fn test_alpha_eq_cache() {
        let mut c = AlphaEqCache::new();
        c.mark_equal(1, 2);
        assert_eq!(c.query(2, 1), Some(true));
        c.mark_inequal(3, 4);
        assert_eq!(c.query(3, 4), Some(false));
    }
    #[test]
    fn test_cache_pressure_monitor() {
        let mut m = CachePressureMonitor::new();
        m.record_insert(10);
        m.record_lookup(true);
        m.record_lookup(false);
        m.record_eviction();
        assert!((m.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_string_pool() {
        let mut p = StringPool::new();
        p.intern("hello");
        p.intern("hello");
        p.intern("world");
        assert_eq!(p.count(), 2);
        assert_eq!(p.saved_bytes(), 5);
    }
    #[test]
    fn test_window_cache() {
        let mut c: WindowCache<i32, &str> = WindowCache::new(2);
        c.insert(1, "a");
        c.insert(2, "b");
        c.insert(3, "c");
        assert_eq!(c.get(&1), None);
        assert_eq!(c.get(&3), Some(&"c"));
    }
    #[test]
    fn test_cache_integrity() {
        let d = "theorem foo";
        let cs = compute_checksum(d);
        assert!(validate_cache_integrity(d, cs));
        assert!(!validate_cache_integrity(d, cs.wrapping_add(1)));
    }
    #[test]
    fn test_classify_entry() {
        assert_eq!(classify_cache_entry(10, 1), CacheTier::Hot);
        assert_eq!(classify_cache_entry(0, 1000), CacheTier::Dead);
    }
    #[test]
    fn test_versioned_cache() {
        let mut c: VersionedCache<&str, i32> = VersionedCache::new();
        c.insert("x", 10);
        assert_eq!(c.get(&"x"), Some(&10));
        c.bump_version();
        assert_eq!(c.get(&"x"), None);
    }
    #[test]
    fn test_bloom_filter() {
        let mut bf = BloomFilter::new(1024, 3);
        bf.insert(42);
        assert!(bf.may_contain(42));
        bf.clear();
    }
    #[test]
    fn test_nesting_depth() {
        let mut t = NestingDepthTracker::new(3);
        assert!(t.enter().is_ok());
        assert!(t.enter().is_ok());
        assert!(t.enter().is_ok());
        assert!(t.enter().is_err());
        t.exit();
        assert!(t.enter().is_ok());
    }
    #[test]
    fn test_rolling_hash() {
        let mut rh = RollingHash::new(3);
        rh.push(b'a');
        rh.push(b'b');
        rh.push(b'c');
        assert!(rh.window_full());
        let h1 = rh.current_hash();
        rh.push(b'd');
        assert_ne!(rh.current_hash(), h1);
    }
    #[test]
    fn test_windowed_metrics() {
        let mut m = WindowedCacheMetrics::new(100);
        m.record_hit();
        m.record_miss();
        assert!((m.hit_rate() - 0.5).abs() < 1e-9);
        m.reset();
        assert_eq!(m.window_hits, 0);
    }
    #[test]
    fn test_cache_health_report() {
        let r = CacheHealthReport {
            total_entries: 100,
            hot_entries: 60,
            warm_entries: 20,
            cold_entries: 10,
            dead_entries: 10,
            estimated_waste_pct: 10.0,
        };
        assert!(r.is_healthy());
        assert!(r.summary().contains("total=100"));
    }
}
#[cfg(test)]
mod extended_expr_cache_tests_2 {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_expr_location_index() {
        let mut idx = ExprLocationIndex::new();
        idx.record(42, 10, 20);
        idx.record(42, 30, 40);
        assert_eq!(idx.count_occurrences(42), 2);
        assert_eq!(idx.total_tracked(), 2);
    }
    #[test]
    fn test_cache_coverage_report() {
        let mut r = CacheCoverageReport::new();
        r.record_cached(1000);
        r.record_uncached(500);
        assert!((r.coverage_pct() - 66.666).abs() < 0.01);
    }
    #[test]
    fn test_namespaced_cache() {
        let mut c: NamespacedCache<&str, i32> = NamespacedCache::new();
        c.insert("math", "pi", 314);
        assert_eq!(c.get("math", &"pi"), Some(&314));
        c.invalidate_namespace("math");
        assert_eq!(c.get("math", &"pi"), None);
    }
    #[test]
    fn test_type_check_cache() {
        let mut tc = TypeCheckCache::new(5);
        let r = TypeCheckResult {
            expr_hash: 99,
            inferred_type: "Nat".into(),
            is_valid: true,
            check_time_us: 10,
        };
        tc.store(r);
        assert!(tc.lookup(99).is_some());
        tc.invalidate(99);
        assert!(tc.lookup(99).is_none());
    }
    #[test]
    fn test_cache_prewarmer() {
        let sources = vec!["x".into(), "y".into()];
        let mut w = CachePrewarmer::new(sources);
        let mut c = ParseResultCache::new(100);
        let n = w.prewarm_all(&mut c);
        assert_eq!(n, 2);
        let n2 = w.prewarm_all(&mut c);
        assert_eq!(n2, 0);
    }
    #[test]
    fn test_hash_set_64() {
        let mut hs = HashSet64::new();
        assert!(hs.insert(42));
        assert!(!hs.insert(42));
        assert!(hs.contains(42));
        hs.clear();
        assert!(hs.is_empty());
    }
    #[test]
    fn test_two_queue_cache() {
        let mut c: TwoQueueCache<String, i32> = TwoQueueCache::new(4);
        c.insert("a".into(), 1);
        c.insert("b".into(), 2);
        assert_eq!(c.get(&"a".into()), Some(&1));
        assert_eq!(c.get(&"z".into()), None);
    }
}
#[cfg(test)]
mod extended_expr_cache_tests_3 {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_lfu_eviction() {
        let p = LfuEviction::new(1, 0.0);
        assert!(!p.should_evict(2, 0, 100));
        assert!(p.should_evict(0, 0, 100));
        assert_eq!(p.policy_name(), "LFU-Age");
    }
    #[test]
    fn test_ttl_eviction() {
        let p = TtlEviction::new(5);
        assert!(!p.should_evict(0, 10, 14));
        assert!(p.should_evict(0, 10, 16));
        assert_eq!(p.policy_name(), "TTL");
    }
    #[test]
    fn test_macro_expansion_cache() {
        let mut c = MacroExpansionCache::new(10);
        c.store(MacroExpansionEntry {
            macro_hash: 1,
            arg_hash: 2,
            expansion: "exp".into(),
            expansion_depth: 1,
            use_count: 0,
        });
        assert!(c.lookup(1, 2).is_some());
        assert_eq!(c.total_stored(), 1);
    }
    #[test]
    fn test_lru_cache() {
        let mut c: LruCache<&str> = LruCache::new(3);
        c.insert(1, "a");
        c.insert(2, "b");
        c.insert(3, "c");
        c.insert(4, "d");
        assert!(!c.contains(1));
        assert_eq!(c.get(4), Some(&"d"));
    }
    #[test]
    fn test_expr_pool() {
        let mut p = ExprPool::new();
        let h = p.intern("Nat".to_string());
        let h2 = p.intern("Nat".to_string());
        assert_eq!(h, h2);
        assert_eq!(p.total_refs(), 2);
        p.release(h);
        assert_eq!(p.total_refs(), 1);
        p.release(h);
        assert_eq!(p.total_exprs(), 0);
    }
    #[test]
    fn test_memo_table() {
        let mut t = MemoTable::new();
        t.store(
            0,
            "expr",
            MemoEntry {
                end_pos: 5,
                result: "x".into(),
                success: true,
            },
        );
        let found = t.lookup(0, "expr");
        assert!(found.is_some());
        assert_eq!(found.expect("test operation should succeed").end_pos, 5);
    }
    #[test]
    fn test_global_expr_table() {
        let mut t = GlobalExprTable::new();
        let id1 = t.intern("Nat");
        let id2 = t.intern("Nat");
        assert_eq!(id1, id2);
        let id3 = t.intern("Bool");
        assert_ne!(id1, id3);
        assert_eq!(t.lookup_repr(id1), Some("Nat"));
        assert_eq!(t.table_size(), 2);
    }
    #[test]
    fn test_symbol_interner() {
        let mut si = SymbolInterner::new();
        let id1 = si.intern("foo");
        let id2 = si.intern("foo");
        assert_eq!(id1, id2);
        let id3 = si.intern("bar");
        assert_ne!(id1, id3);
        assert_eq!(si.lookup(id1), Some("foo"));
        assert_eq!(si.size(), 2);
        assert!(si.contains("foo"));
        assert!(!si.contains("baz"));
    }
}
/// DJB2 hash for expressions.
#[allow(dead_code)]
pub fn djb2_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    hash
}
/// Combined hash: mixes FNV-1a and DJB2.
#[allow(dead_code)]
pub fn combined_hash(s: &str) -> u64 {
    let fnv = fnv1a_hash(s.as_bytes());
    let djb = djb2_hash(s);
    mix_hashes(fnv, djb)
}
/// Checks if two expression strings are alpha-equivalent by their hashes.
#[allow(dead_code)]
pub fn hash_alpha_equiv(a: &str, b: &str) -> bool {
    combined_hash(a) == combined_hash(b)
}
/// Estimate memory usage of a cached string in bytes.
#[allow(dead_code)]
pub fn estimate_string_memory(s: &str) -> usize {
    s.len() + std::mem::size_of::<String>() + 8
}
/// Build a simple cache key from multiple components.
#[allow(dead_code)]
pub fn build_cache_key(parts: &[&str]) -> u64 {
    let mut h: u64 = 0;
    for part in parts {
        h = mix_hashes(h, fnv1a_hash(part.as_bytes()));
    }
    h
}
#[cfg(test)]
mod extended_expr_cache_tests_4 {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_bump_allocator() {
        let mut alloc = BumpAllocator::new(100);
        let pos = alloc
            .alloc_str("hello")
            .expect("test operation should succeed");
        assert_eq!(alloc.get_str(pos, 5), Some("hello"));
        assert_eq!(alloc.used(), 5);
        alloc.reset();
        assert_eq!(alloc.used(), 0);
    }
    #[test]
    fn test_token_frequency_table() {
        let mut t = TokenFrequencyTable::new();
        t.record("def");
        t.record("def");
        t.record("fun");
        assert_eq!(t.count("def"), 2);
        assert_eq!(t.unique_tokens(), 2);
        assert_eq!(t.total_tokens(), 3);
        let top = t.top_n(1);
        assert_eq!(top[0].0, "def");
    }
    #[test]
    fn test_cache_warmup() {
        let w = CacheWarmup::new(vec!["x".into(), "y".into()]).with_priority(CachePriority::High);
        assert_eq!(w.source_count(), 2);
        assert_eq!(w.priority, CachePriority::High);
    }
    #[test]
    fn test_cache_report() {
        let r = CacheReport::new(100, 80, 20, 5, 1024);
        assert!((r.hit_rate() - 0.8).abs() < 1e-9);
        let s = r.summary();
        assert!(s.contains("hit_rate=80.0%"));
    }
    #[test]
    fn test_token_window() {
        let mut w = TokenWindow::new(3);
        w.push("a");
        w.push("b");
        w.push("c");
        assert!(w.is_full());
        w.push("d");
        assert!(!w.contains("a"));
        assert!(w.contains("d"));
    }
    #[test]
    fn test_djb2_hash() {
        let h1 = djb2_hash("hello");
        let h2 = djb2_hash("hello");
        assert_eq!(h1, h2);
        assert_ne!(djb2_hash("hello"), djb2_hash("world"));
    }
    #[test]
    fn test_combined_hash() {
        let h = combined_hash("theorem foo : Nat");
        assert!(h > 0);
        assert_eq!(combined_hash("x"), combined_hash("x"));
        assert_ne!(combined_hash("x"), combined_hash("y"));
    }
    #[test]
    fn test_build_cache_key() {
        let k1 = build_cache_key(&["rule1", "pos:5"]);
        let k2 = build_cache_key(&["rule1", "pos:5"]);
        assert_eq!(k1, k2);
        assert_ne!(build_cache_key(&["a"]), build_cache_key(&["b"]));
    }
    #[test]
    fn test_estimate_string_memory() {
        let m = estimate_string_memory("hello");
        assert!(m >= 5);
    }
    #[test]
    fn test_interning_stats() {
        let mut stats = InterningStats::new();
        stats.record_new();
        stats.record_new();
        stats.record_hit(10);
        assert_eq!(stats.unique_strings, 2);
        assert_eq!(stats.bytes_saved, 10);
        assert!((stats.dedup_ratio() - 1.5).abs() < 1e-9);
    }
    #[test]
    fn test_hash_alpha_equiv() {
        assert!(hash_alpha_equiv("hello", "hello"));
        assert!(!hash_alpha_equiv("foo", "bar"));
    }
}
#[cfg(test)]
mod extended_expr_cache_tests_5 {
    use super::*;
    use crate::expr_cache::*;
    #[test]
    fn test_cache_key_builder() {
        let k1 = CacheKeyBuilder::new()
            .with_str("expr")
            .with_usize(5)
            .build();
        let k2 = CacheKeyBuilder::new()
            .with_str("expr")
            .with_usize(5)
            .build();
        assert_eq!(k1, k2);
        let k3 = CacheKeyBuilder::new().with_str("ty").with_usize(5).build();
        assert_ne!(k1, k3);
    }
    #[test]
    fn test_adaptive_lru_cache() {
        let mut c: AdaptiveLruCache<i32> = AdaptiveLruCache::new(10, 5, 100);
        c.insert(1, 42);
        assert_eq!(c.get(1), Some(&42));
        assert_eq!(c.get(99), None);
        assert_eq!(c.hit_rate(), 0.5);
    }
    #[test]
    fn test_expr_diff_cache() {
        let mut c = ExprDiffCache::new(10);
        c.store(1, 2, "diff text");
        assert_eq!(c.lookup(1, 2), Some("diff text"));
        assert_eq!(c.lookup(2, 1), Some("diff text"));
        assert_eq!(c.size(), 1);
    }
    #[test]
    fn test_multi_level_cache() {
        let mut c: MultiLevelCache<String> = MultiLevelCache::new(2, 10);
        c.insert(1, "a".to_string());
        c.insert(2, "b".to_string());
        assert_eq!(c.get(&1), Some("a".to_string()));
        assert_eq!(c.get(&99), None);
    }
    #[test]
    fn test_persistent_cache() {
        let mut c = PersistentCache::new();
        c.insert(42, "hello");
        c.insert(99, "world");
        let s = c.serialize();
        let c2 = PersistentCache::deserialize(&s);
        assert_eq!(c2.lookup(42), Some("hello"));
        assert_eq!(c2.lookup(99), Some("world"));
        assert_eq!(c2.entry_count(), 2);
    }
}
