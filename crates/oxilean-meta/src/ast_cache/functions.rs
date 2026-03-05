//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AstCacheAnalysisPass, AstCacheConfig, AstCacheConfigValue, AstCacheDiagnostics, AstCacheDiff,
    AstCacheExtConfig400, AstCacheExtConfig401, AstCacheExtConfigVal400, AstCacheExtConfigVal401,
    AstCacheExtDiag400, AstCacheExtDiag401, AstCacheExtDiff400, AstCacheExtDiff401,
    AstCacheExtPass400, AstCacheExtPass401, AstCacheExtPipeline400, AstCacheExtPipeline401,
    AstCacheExtResult400, AstCacheExtResult401, AstCachePipeline, AstCacheResult,
    AstTransformCache, BatchTransformer, CacheStatsExt, ExprHash, HashConsCache, InternTable,
    LazyExprCache, LruCacheExt, MemoTransformExt, SubstCache, TieredCache, TtlCache, TwoLevelCache,
    VersionedCache, WarmableCache, WarmingStrategy, WhnfCache,
};
use oxilean_kernel::{Expr, Level};
use std::hash::{Hash, Hasher};

pub(super) fn make_hasher() -> std::collections::hash_map::DefaultHasher {
    std::collections::hash_map::DefaultHasher::new()
}
/// Hash an `Expr` into a `u64` using its structural discriminant and key fields.
pub fn hash_expr(expr: &Expr) -> u64 {
    let mut h = make_hasher();
    hash_expr_into(expr, &mut h);
    h.finish()
}
pub(super) fn hash_level(level: &Level, h: &mut std::collections::hash_map::DefaultHasher) {
    std::mem::discriminant(level).hash(h);
    match level {
        Level::Zero => {}
        Level::Succ(inner) => hash_level(inner, h),
        Level::Max(a, b) | Level::IMax(a, b) => {
            hash_level(a, h);
            hash_level(b, h);
        }
        Level::Param(name) => name.hash(h),
        Level::MVar(id) => id.hash(h),
    }
}
pub(super) fn hash_expr_into(expr: &Expr, h: &mut std::collections::hash_map::DefaultHasher) {
    std::mem::discriminant(expr).hash(h);
    match expr {
        Expr::BVar(idx) => idx.hash(h),
        Expr::FVar(id) => id.0.hash(h),
        Expr::Sort(level) => hash_level(level, h),
        Expr::Const(name, levels) => {
            name.hash(h);
            for lvl in levels {
                hash_level(lvl, h);
            }
        }
        Expr::App(f, a) => {
            hash_expr_into(f, h);
            hash_expr_into(a, h);
        }
        Expr::Lam(bi, name, ty, body) => {
            bi.hash(h);
            name.hash(h);
            hash_expr_into(ty, h);
            hash_expr_into(body, h);
        }
        Expr::Pi(bi, name, ty, body) => {
            bi.hash(h);
            name.hash(h);
            hash_expr_into(ty, h);
            hash_expr_into(body, h);
        }
        Expr::Let(name, ty, val, body) => {
            name.hash(h);
            hash_expr_into(ty, h);
            hash_expr_into(val, h);
            hash_expr_into(body, h);
        }
        Expr::Lit(lit) => lit.hash(h),
        Expr::Proj(name, idx, inner) => {
            name.hash(h);
            idx.hash(h);
            hash_expr_into(inner, h);
        }
    }
}
/// Substitute `BVar(target_depth)` with `replacement` throughout `expr`.
/// This is a best-effort helper; the kernel's `subst` module is authoritative.
pub(super) fn substitute_bvar(expr: &Expr, target_depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx == target_depth {
                replacement.clone()
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(substitute_bvar(f, target_depth, replacement)),
            Box::new(substitute_bvar(a, target_depth, replacement)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(substitute_bvar(ty, target_depth, replacement)),
            Box::new(substitute_bvar(body, target_depth + 1, replacement)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(substitute_bvar(ty, target_depth, replacement)),
            Box::new(substitute_bvar(body, target_depth + 1, replacement)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(substitute_bvar(ty, target_depth, replacement)),
            Box::new(substitute_bvar(val, target_depth, replacement)),
            Box::new(substitute_bvar(body, target_depth + 1, replacement)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(substitute_bvar(inner, target_depth, replacement)),
        ),
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
    }
}
/// One-step top-level beta reduction: `(λ x. body) arg` → `body[arg/BVar(0)]`.
pub(super) fn beta_reduce_top(expr: &Expr) -> Expr {
    if let Expr::App(f, arg) = expr {
        if let Expr::Lam(_, _, _, body) = f.as_ref() {
            return substitute_bvar(body, 0, arg);
        }
    }
    expr.clone()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_cache::*;
    use oxilean_kernel::{BinderInfo, Expr, Level, Name};
    fn name(s: &str) -> Name {
        Name::str(s)
    }
    fn sort0() -> Expr {
        Expr::Sort(Level::Zero)
    }
    fn bvar(n: u32) -> Expr {
        Expr::BVar(n)
    }
    fn const_expr(s: &str) -> Expr {
        Expr::Const(name(s), vec![])
    }
    fn app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn lam(body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            name("x"),
            Box::new(sort0()),
            Box::new(body),
        )
    }
    #[test]
    fn test_expr_hash_consistent() {
        let e = app(const_expr("f"), bvar(0));
        let h1 = ExprHash::of(&e);
        let h2 = ExprHash::of(&e);
        assert_eq!(h1, h2, "Same expression must yield the same hash");
    }
    #[test]
    fn test_expr_hash_distinct() {
        let e1 = bvar(0);
        let e2 = bvar(1);
        assert_ne!(
            ExprHash::of(&e1),
            ExprHash::of(&e2),
            "BVar(0) and BVar(1) should hash differently"
        );
    }
    #[test]
    fn test_ast_cache_hit_miss() {
        let mut cache = AstTransformCache::new(10);
        let key = ExprHash(42);
        let val = sort0();
        let result = cache.get_or_compute(key, || val.clone());
        assert_eq!(result, val);
        assert_eq!(cache.miss_count, 1);
        assert_eq!(cache.hit_count, 0);
        let result2 = cache.get_or_compute(key, || bvar(99));
        assert_eq!(
            result2, val,
            "Should return cached value, not recomputed one"
        );
        assert_eq!(cache.hit_count, 1);
    }
    #[test]
    fn test_ast_cache_evict_lru() {
        let mut cache = AstTransformCache::new(2);
        let k1 = ExprHash(1);
        let k2 = ExprHash(2);
        let k3 = ExprHash(3);
        cache.insert(k1, bvar(0));
        cache.insert(k2, bvar(1));
        cache.insert(k3, bvar(2));
        assert!(cache.get(k1).is_none(), "k1 should have been evicted");
        assert!(cache.get(k2).is_some(), "k2 should still be present");
        assert!(cache.get(k3).is_some(), "k3 should be present");
    }
    #[test]
    fn test_ast_cache_hit_rate() {
        let mut cache = AstTransformCache::new(10);
        let key = ExprHash(7);
        cache.get_or_compute(key, sort0);
        cache.get_or_compute(key, sort0);
        cache.get_or_compute(key, sort0);
        let rate = cache.hit_rate();
        assert!(
            (rate - 2.0 / 3.0).abs() < 1e-9,
            "Expected hit rate 2/3, got {}",
            rate
        );
    }
    #[test]
    fn test_subst_cache_store_lookup() {
        let mut sc = SubstCache::new();
        let eh = ExprHash(100);
        let rh = ExprHash(200);
        let result = bvar(5);
        assert!(sc.lookup(eh, 0, rh).is_none());
        sc.store(eh, 0, rh, result.clone());
        assert_eq!(sc.lookup(eh, 0, rh), Some(&result));
        assert_eq!(sc.size(), 1);
        sc.clear();
        assert_eq!(sc.size(), 0);
    }
    #[test]
    fn test_whnf_cache_hot_entries() {
        let mut wc = WhnfCache::new(100);
        let k1 = ExprHash(1);
        let k2 = ExprHash(2);
        wc.insert(k1, sort0());
        wc.insert(k2, bvar(0));
        wc.get(k1);
        wc.get(k1);
        wc.get(k1);
        wc.get(k2);
        let hot = wc.hot_entries();
        assert!(hot.contains(&k1), "k1 should be hot");
        assert!(!hot.contains(&k2), "k2 should not be hot");
    }
    #[test]
    fn test_batch_transformer_beta_reduce() {
        let mut bt = BatchTransformer::new();
        let arg = const_expr("Nat");
        let body = bvar(0);
        let redex = app(lam(body), arg.clone());
        bt.enqueue(redex);
        let results = bt.batch_beta_reduce();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0], arg,
            "Beta reduction should produce Const(\"Nat\")"
        );
    }
}
#[cfg(test)]
mod ast_cache_ext_tests {
    use super::*;
    use crate::ast_cache::*;
    use oxilean_kernel::Name;
    #[test]
    fn test_lru_ext_basic() {
        let mut cache: LruCacheExt<String, i32> = LruCacheExt::new(3);
        cache.put("a".to_string(), 1);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
    }
    #[test]
    fn test_lru_ext_capacity() {
        let cache: LruCacheExt<i32, i32> = LruCacheExt::new(5);
        assert_eq!(cache.capacity(), 5);
        assert!(cache.is_empty());
    }
    #[test]
    fn test_lru_ext_eviction() {
        let mut cache: LruCacheExt<i32, i32> = LruCacheExt::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert!(cache.len() <= 2);
    }
    #[test]
    fn test_lru_ext_clear() {
        let mut cache: LruCacheExt<i32, i32> = LruCacheExt::new(5);
        cache.put(1, 1);
        cache.clear_all();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_tiered_cache_put_get() {
        let mut tc = TieredCache::new(2, 10);
        let e = Expr::Const(Name::str("Nat"), vec![]);
        tc.put(42, e.clone());
        let r = tc.get(42);
        assert!(r.is_some());
    }
    #[test]
    fn test_tiered_cache_miss() {
        let mut tc = TieredCache::new(2, 5);
        assert!(tc.get(99999).is_none());
        assert_eq!(tc.misses, 1);
    }
    #[test]
    fn test_tiered_cache_hit_rate() {
        let mut tc = TieredCache::new(5, 20);
        let e = Expr::BVar(0);
        tc.put(1, e);
        tc.get(1);
        tc.get(2);
        assert!(tc.hit_rate() > 0.0);
    }
    #[test]
    fn test_memo_transform_ext() {
        let mut memo = MemoTransformExt::new();
        let e = Expr::BVar(0);
        let r1 = memo.apply(&e, |x| x.clone());
        let r2 = memo.apply(&e, |x| x.clone());
        assert_eq!(memo.cache_hits, 1);
        let _ = r1;
        let _ = r2;
    }
    #[test]
    fn test_cache_stats_ext() {
        let mut stats = CacheStatsExt::new();
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();
        assert!((stats.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
        assert!(stats.is_warm());
    }
    #[test]
    fn test_cache_stats_merge() {
        let mut a = CacheStatsExt::new();
        let mut b = CacheStatsExt::new();
        a.record_hit();
        b.record_miss();
        a.merge(&b);
        assert_eq!(a.total_queries, 2);
    }
    #[test]
    fn test_lazy_expr_cache_ready() {
        let mut cache = LazyExprCache::new();
        cache.store_result(1, Expr::BVar(0));
        assert!(cache.get(1).map(|v| v.is_ready()).unwrap_or(false));
    }
    #[test]
    fn test_lazy_expr_cache_pending() {
        let mut cache = LazyExprCache::new();
        cache.mark_pending(2);
        assert!(cache.get(2).map(|v| v.is_pending()).unwrap_or(false));
    }
    #[test]
    fn test_lazy_expr_cache_error() {
        let mut cache = LazyExprCache::new();
        cache.store_error(3, "timeout".to_string());
        assert!(cache.get(3).map(|v| v.is_failed()).unwrap_or(false));
        assert_eq!(cache.get(3).and_then(|v| v.error_msg()), Some("timeout"));
    }
    #[test]
    fn test_lazy_expr_cache_counts() {
        let mut cache = LazyExprCache::new();
        cache.store_result(1, Expr::BVar(0));
        cache.store_error(2, "err".to_string());
        cache.mark_pending(3);
        assert_eq!(cache.num_ready(), 1);
        assert_eq!(cache.num_failed(), 1);
    }
    #[test]
    fn test_memo_transform_clear() {
        let mut memo = MemoTransformExt::new();
        let e = Expr::BVar(0);
        memo.apply(&e, |x| x.clone());
        assert_eq!(memo.cache_size(), 1);
        memo.clear_cache();
        assert_eq!(memo.cache_size(), 0);
    }
}
#[cfg(test)]
mod ast_cache_ext2_tests {
    use super::*;
    use crate::ast_cache::*;
    #[test]
    fn test_versioned_cache_basic() {
        let mut cache: VersionedCache<String, i32> = VersionedCache::new();
        cache.insert("a".to_string(), 1);
        assert_eq!(cache.get(&"a".to_string()), Some(&1));
    }
    #[test]
    fn test_versioned_cache_checkpoint_rollback() {
        let mut cache: VersionedCache<String, i32> = VersionedCache::new();
        cache.insert("a".to_string(), 1);
        cache.checkpoint();
        cache.insert("b".to_string(), 2);
        assert_eq!(cache.current_size(), 2);
        cache.rollback();
        assert_eq!(cache.current_size(), 1);
    }
    #[test]
    fn test_versioned_cache_num_versions() {
        let mut cache: VersionedCache<i32, i32> = VersionedCache::new();
        assert_eq!(cache.num_versions(), 1);
        cache.checkpoint();
        assert_eq!(cache.num_versions(), 2);
        cache.rollback();
        assert_eq!(cache.num_versions(), 1);
    }
    #[test]
    fn test_intern_table_basic() {
        let mut table = InternTable::new();
        let id1 = table.intern(42);
        let id2 = table.intern(42);
        assert_eq!(id1, id2);
        let id3 = table.intern(43);
        assert_ne!(id1, id3);
    }
    #[test]
    fn test_intern_table_size() {
        let mut table = InternTable::new();
        table.intern(1);
        table.intern(2);
        table.intern(1);
        assert_eq!(table.size(), 2);
    }
    #[test]
    fn test_ttl_cache_basic() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(3);
        cache.insert("x".to_string(), 42);
        assert_eq!(cache.get(&"x".to_string()), Some(42));
    }
    #[test]
    fn test_ttl_cache_expiry() {
        let mut cache: TtlCache<String, i32> = TtlCache::new(1);
        cache.insert("x".to_string(), 42);
        cache.get(&"x".to_string());
        assert_eq!(cache.get(&"x".to_string()), None);
    }
    #[test]
    fn test_two_level_cache_insert_lookup() {
        let mut cache = TwoLevelCache::new(5, 20);
        let e = Expr::BVar(0);
        cache.insert(1, e.clone());
        let r = cache.lookup(1);
        assert!(r.is_some());
    }
    #[test]
    fn test_two_level_cache_total_size() {
        let mut cache = TwoLevelCache::new(2, 5);
        cache.insert(1, Expr::BVar(0));
        cache.insert(2, Expr::BVar(1));
        assert_eq!(cache.total_size(), 2);
    }
    #[test]
    fn test_hash_cons_basic() {
        let mut hcc = HashConsCache::new();
        let e1 = Expr::BVar(0);
        let e2 = Expr::BVar(0);
        let r1 = hcc.intern(e1.clone());
        let r2 = hcc.intern(e2.clone());
        assert_eq!(r1, r2);
        assert_eq!(hcc.size(), 1);
    }
    #[test]
    fn test_hash_cons_hit_count() {
        let mut hcc = HashConsCache::new();
        let e = Expr::BVar(0);
        hcc.intern(e.clone());
        hcc.intern(e.clone());
        assert_eq!(hcc.hit_count(), 1);
    }
    #[test]
    fn test_warmable_cache_eager() {
        let mut cache = WarmableCache::new(10, WarmingStrategy::Eager);
        let entries = vec![(1u64, Expr::BVar(0)), (2u64, Expr::BVar(1))];
        cache.warm_with(entries);
        assert!(cache.is_warm());
        assert!(cache.get(1).is_some());
    }
    #[test]
    fn test_warmable_cache_fill_ratio() {
        let mut cache = WarmableCache::new(10, WarmingStrategy::Lazy);
        cache.put(1, Expr::BVar(0));
        cache.put(2, Expr::BVar(1));
        assert!((cache.fill_ratio() - 0.2).abs() < 1e-10);
    }
    #[test]
    fn test_lru_cache_ext_basic() {
        let mut cache: LruCacheExt<String, i32> = LruCacheExt::new(3);
        cache.put("a".to_string(), 1);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
    }
    #[test]
    fn test_lru_cache_ext_eviction() {
        let mut cache: LruCacheExt<i32, i32> = LruCacheExt::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert!(cache.len() <= 2);
    }
    #[test]
    fn test_tiered_cache_hit_rate() {
        let mut tc = TieredCache::new(5, 20);
        let e = Expr::BVar(0);
        tc.put(1, e);
        tc.get(1);
        tc.get(2);
        assert!(tc.hit_rate() > 0.0);
    }
    #[test]
    fn test_lazy_expr_cache_ext_basic() {
        let mut cache = LazyExprCache::new();
        cache.store_result(1, Expr::BVar(0));
        assert!(cache.get(1).map(|v| v.is_ready()).unwrap_or(false));
    }
    #[test]
    fn test_cache_stats_ext_merge() {
        let mut a = CacheStatsExt::new();
        let mut b = CacheStatsExt::new();
        a.record_hit();
        b.record_miss();
        a.merge(&b);
        assert_eq!(a.total_queries, 2);
    }
    #[test]
    fn test_versioned_cache_no_rollback_past_zero() {
        let mut cache: VersionedCache<i32, i32> = VersionedCache::new();
        assert!(!cache.rollback());
    }
    #[test]
    fn test_intern_table_lookup() {
        let mut table = InternTable::new();
        let id = table.intern(100);
        assert_eq!(table.lookup(100), Some(id));
        assert_eq!(table.lookup(999), None);
    }
}
#[cfg(test)]
mod astcache_analysis_tests {
    use super::*;
    use crate::ast_cache::*;
    #[test]
    fn test_astcache_result_ok() {
        let r = AstCacheResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_astcache_result_err() {
        let r = AstCacheResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_astcache_result_partial() {
        let r = AstCacheResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_astcache_result_skipped() {
        let r = AstCacheResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_astcache_analysis_pass_run() {
        let mut p = AstCacheAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_astcache_analysis_pass_empty_input() {
        let mut p = AstCacheAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_astcache_analysis_pass_success_rate() {
        let mut p = AstCacheAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_astcache_analysis_pass_disable() {
        let mut p = AstCacheAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_astcache_pipeline_basic() {
        let mut pipeline = AstCachePipeline::new("main_pipeline");
        pipeline.add_pass(AstCacheAnalysisPass::new("pass1"));
        pipeline.add_pass(AstCacheAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_astcache_pipeline_disabled_pass() {
        let mut pipeline = AstCachePipeline::new("partial");
        let mut p = AstCacheAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(AstCacheAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_astcache_diff_basic() {
        let mut d = AstCacheDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_astcache_diff_summary() {
        let mut d = AstCacheDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_astcache_config_set_get() {
        let mut cfg = AstCacheConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_astcache_config_read_only() {
        let mut cfg = AstCacheConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_astcache_config_remove() {
        let mut cfg = AstCacheConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_astcache_diagnostics_basic() {
        let mut diag = AstCacheDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_astcache_diagnostics_max_errors() {
        let mut diag = AstCacheDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_astcache_diagnostics_clear() {
        let mut diag = AstCacheDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_astcache_config_value_types() {
        let b = AstCacheConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = AstCacheConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = AstCacheConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = AstCacheConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = AstCacheConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod ast_cache_ext_tests_400 {
    use super::*;
    use crate::ast_cache::*;
    #[test]
    fn test_ast_cache_ext_result_ok_400() {
        let r = AstCacheExtResult400::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_err_400() {
        let r = AstCacheExtResult400::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_partial_400() {
        let r = AstCacheExtResult400::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_skipped_400() {
        let r = AstCacheExtResult400::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_ast_cache_ext_pass_run_400() {
        let mut p = AstCacheExtPass400::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_ast_cache_ext_pass_empty_400() {
        let mut p = AstCacheExtPass400::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_ast_cache_ext_pass_rate_400() {
        let mut p = AstCacheExtPass400::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_ast_cache_ext_pass_disable_400() {
        let mut p = AstCacheExtPass400::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_ast_cache_ext_pipeline_basic_400() {
        let mut pipeline = AstCacheExtPipeline400::new("main_pipeline");
        pipeline.add_pass(AstCacheExtPass400::new("pass1"));
        pipeline.add_pass(AstCacheExtPass400::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_ast_cache_ext_pipeline_disabled_400() {
        let mut pipeline = AstCacheExtPipeline400::new("partial");
        let mut p = AstCacheExtPass400::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(AstCacheExtPass400::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_ast_cache_ext_diff_basic_400() {
        let mut d = AstCacheExtDiff400::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_ast_cache_ext_config_set_get_400() {
        let mut cfg = AstCacheExtConfig400::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_ast_cache_ext_config_read_only_400() {
        let mut cfg = AstCacheExtConfig400::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_ast_cache_ext_config_remove_400() {
        let mut cfg = AstCacheExtConfig400::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_basic_400() {
        let mut diag = AstCacheExtDiag400::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_max_errors_400() {
        let mut diag = AstCacheExtDiag400::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_clear_400() {
        let mut diag = AstCacheExtDiag400::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_ast_cache_ext_config_value_types_400() {
        let b = AstCacheExtConfigVal400::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = AstCacheExtConfigVal400::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = AstCacheExtConfigVal400::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = AstCacheExtConfigVal400::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = AstCacheExtConfigVal400::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod ast_cache_ext_tests_401 {
    use super::*;
    use crate::ast_cache::*;
    #[test]
    fn test_ast_cache_ext_result_ok_400() {
        let r = AstCacheExtResult401::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_err_400() {
        let r = AstCacheExtResult401::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_partial_400() {
        let r = AstCacheExtResult401::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_ast_cache_ext_result_skipped_400() {
        let r = AstCacheExtResult401::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_ast_cache_ext_pass_run_400() {
        let mut p = AstCacheExtPass401::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_ast_cache_ext_pass_empty_400() {
        let mut p = AstCacheExtPass401::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_ast_cache_ext_pass_rate_400() {
        let mut p = AstCacheExtPass401::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_ast_cache_ext_pass_disable_400() {
        let mut p = AstCacheExtPass401::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_ast_cache_ext_pipeline_basic_400() {
        let mut pipeline = AstCacheExtPipeline401::new("main_pipeline");
        pipeline.add_pass(AstCacheExtPass401::new("pass1"));
        pipeline.add_pass(AstCacheExtPass401::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_ast_cache_ext_pipeline_disabled_400() {
        let mut pipeline = AstCacheExtPipeline401::new("partial");
        let mut p = AstCacheExtPass401::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(AstCacheExtPass401::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_ast_cache_ext_diff_basic_400() {
        let mut d = AstCacheExtDiff401::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_ast_cache_ext_config_set_get_400() {
        let mut cfg = AstCacheExtConfig401::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_ast_cache_ext_config_read_only_400() {
        let mut cfg = AstCacheExtConfig401::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_ast_cache_ext_config_remove_400() {
        let mut cfg = AstCacheExtConfig401::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_basic_400() {
        let mut diag = AstCacheExtDiag401::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_max_errors_400() {
        let mut diag = AstCacheExtDiag401::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_ast_cache_ext_diagnostics_clear_400() {
        let mut diag = AstCacheExtDiag401::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_ast_cache_ext_config_value_types_400() {
        let b = AstCacheExtConfigVal401::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = AstCacheExtConfigVal401::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = AstCacheExtConfigVal401::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = AstCacheExtConfigVal401::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = AstCacheExtConfigVal401::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
