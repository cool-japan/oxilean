//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ArtifactManifest, BandwidthStats, CacheBackendKind, CacheCapacityBudget, CacheEntry,
    CacheHealthChecker, CacheIndex, CacheKey, CacheKeyHasher, CacheKeyNamespace, CacheLease,
    CacheLeaseManager, CacheMetrics, CachePolicy, CachePrefetchList, CacheQuota, CacheReadPolicy,
    CacheSegment, CacheSegmentManager, CacheSessionId, CacheStatsSnapshot, CacheWarmingConfig,
    CacheWritePolicy, CompressedArtifact, ContentAddressedStore, EvictionCandidate, EvictionPolicy,
    GcsCacheConfig, HttpCacheConfig, LocalMirrorCache, MultiBackendCache, RemoteCacheClient,
    RemoteCacheConfig, RemoteCachePool, RetryConfig, S3CacheConfig,
};

#[cfg(test)]
mod tests {
    use super::*;
    fn make_key(module: &str, hash: &str) -> CacheKey {
        CacheKey::new(module, hash)
    }
    #[test]
    fn cache_key_path_component_no_spaces() {
        let key = make_key("Mathlib/Algebra", "abc123");
        let path = key.to_path_component();
        assert!(!path.contains('/'));
        assert!(path.contains("abc123"));
    }
    #[test]
    fn cache_entry_is_stale() {
        let entry = CacheEntry {
            key: make_key("Mod", "h"),
            artifact_size: 100,
            timestamp: 1000,
            hit_count: 0,
        };
        assert!(entry.is_stale(500, 2000));
        assert!(!entry.is_stale(5000, 2000));
    }
    #[test]
    fn remote_cache_config_builder() {
        let cfg = RemoteCacheConfig::new("https://cache.example.com").with_api_key("secret");
        assert_eq!(cfg.api_key.as_deref(), Some("secret"));
        assert_eq!(cfg.timeout_secs, 30);
    }
    #[test]
    fn client_try_fetch_returns_none() {
        let cfg = RemoteCacheConfig::new("https://cache.example.com");
        let mut client = RemoteCacheClient::new(cfg);
        let key = make_key("Mod", "deadbeef");
        assert!(client.try_fetch(&key).is_none());
        assert_eq!(client.miss_count, 1);
    }
    #[test]
    fn client_try_upload_returns_true() {
        let cfg = RemoteCacheConfig::new("https://cache.example.com");
        let mut client = RemoteCacheClient::new(cfg);
        let key = make_key("Mod", "deadbeef");
        assert!(client.try_upload(&key, b"artifact-data"));
        assert_eq!(client.upload_count, 1);
    }
    #[test]
    fn client_hit_rate_zero_when_no_requests() {
        let cfg = RemoteCacheConfig::new("https://cache.example.com");
        let client = RemoteCacheClient::new(cfg);
        assert!((client.hit_rate() - 0.0).abs() < f64::EPSILON);
    }
    #[test]
    fn local_mirror_register_and_lookup() {
        let mut cache = LocalMirrorCache::new("/tmp/oxilean_cache_test");
        let key = make_key("Data.Nat", "ff00");
        cache.register(CacheKey::new("Data.Nat", "ff00"), 4096);
        assert!(cache.lookup(&key).is_some());
        assert_eq!(cache.entry_count(), 1);
    }
    #[test]
    fn local_mirror_evict_stale() {
        let mut cache = LocalMirrorCache::new("/tmp/oxilean_cache_test2");
        cache.register(CacheKey::new("ModA", "h1"), 100);
        cache.register(CacheKey::new("ModB", "h2"), 200);
        let evicted = cache.evict_stale(500, 10_000);
        assert_eq!(evicted, 2);
        assert_eq!(cache.entry_count(), 0);
        assert_eq!(cache.total_size_bytes(), 0);
    }
}
/// Compute LRU eviction candidates from a `CacheIndex`.
pub fn lru_candidates(index: &CacheIndex, now_secs: u64) -> Vec<EvictionCandidate> {
    index
        .entries
        .iter()
        .map(|(k, e)| {
            let age = now_secs.saturating_sub(e.last_access) as f64;
            EvictionCandidate::new(k, age, e.size_bytes)
        })
        .collect()
}
/// Compute LFU eviction candidates from a `CacheIndex`.
pub fn lfu_candidates(index: &CacheIndex) -> Vec<EvictionCandidate> {
    index
        .entries
        .iter()
        .map(|(k, e)| {
            let score = 1.0 / (e.hit_count as f64 + 1.0);
            EvictionCandidate::new(k, score, e.size_bytes)
        })
        .collect()
}
/// Sort candidates descending by score (highest score → evict first).
pub fn sort_candidates(candidates: &mut Vec<EvictionCandidate>) {
    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn retry_config_delay_for_attempt() {
        let r = RetryConfig::default_remote();
        assert_eq!(r.delay_for_attempt(0), 200);
        assert_eq!(r.delay_for_attempt(1), 400);
        assert_eq!(r.delay_for_attempt(2), 800);
    }
    #[test]
    fn retry_config_capped_at_max() {
        let r = RetryConfig {
            max_attempts: 10,
            initial_delay_ms: 1_000,
            backoff_factor: 10.0,
            max_delay_ms: 3_000,
        };
        assert!(r.delay_for_attempt(3) <= 3_000);
    }
    #[test]
    fn retry_config_should_retry() {
        let r = RetryConfig::default_remote();
        assert!(r.should_retry(0));
        assert!(r.should_retry(1));
        assert!(!r.should_retry(2));
    }
    #[test]
    fn bandwidth_stats_avg_speeds() {
        let mut stats = BandwidthStats::new();
        stats.record_download(1_000, 1_000);
        stats.record_upload(2_000, 500);
        assert!((stats.avg_download_bps() - 1_000.0).abs() < 1.0);
        assert!((stats.avg_upload_bps() - 4_000.0).abs() < 1.0);
    }
    #[test]
    fn bandwidth_stats_summary_not_empty() {
        let stats = BandwidthStats::new();
        assert!(!stats.summary().is_empty());
    }
    #[test]
    fn cas_insert_and_get() {
        let mut cas = ContentAddressedStore::with_capacity(10);
        let data = b"hello oxilean".to_vec();
        let hash = cas.insert(data.clone());
        assert_eq!(cas.get(hash).cloned(), Some(data));
    }
    #[test]
    fn cas_evicts_at_capacity() {
        let mut cas = ContentAddressedStore::with_capacity(2);
        cas.insert(b"a".to_vec());
        cas.insert(b"b".to_vec());
        cas.insert(b"c".to_vec());
        assert!(cas.len() <= 2);
    }
    #[test]
    fn cas_hash_bytes_deterministic() {
        let h1 = ContentAddressedStore::hash_bytes(b"test");
        let h2 = ContentAddressedStore::hash_bytes(b"test");
        assert_eq!(h1, h2);
    }
    #[test]
    fn cas_remove_entry() {
        let mut cas = ContentAddressedStore::with_capacity(10);
        let hash = cas.insert(b"data".to_vec());
        assert!(cas.remove(hash));
        assert!(cas.get(hash).is_none());
    }
    #[test]
    fn cas_total_bytes() {
        let mut cas = ContentAddressedStore::with_capacity(10);
        cas.insert(vec![0u8; 100]);
        cas.insert(vec![0u8; 200]);
        assert!(cas.total_bytes() >= 300);
    }
    #[test]
    fn cache_index_upsert_and_get() {
        let mut idx = CacheIndex::new();
        idx.upsert(
            "mod-v1-abc",
            0xdeadbeef,
            1024,
            1000,
            CacheBackendKind::Local,
        );
        let entry = idx.get("mod-v1-abc").expect("key should exist");
        assert_eq!(entry.content_hash, 0xdeadbeef);
    }
    #[test]
    fn cache_index_evict_stale() {
        let mut idx = CacheIndex::new();
        idx.upsert("old-entry", 1, 512, 100, CacheBackendKind::Local);
        idx.upsert("new-entry", 2, 512, 9_800, CacheBackendKind::Local);
        let evicted = idx.evict_stale(500, 10_000);
        assert_eq!(evicted, 1);
        assert!(idx.get("new-entry").is_some());
    }
    #[test]
    fn cache_index_record_hit() {
        let mut idx = CacheIndex::new();
        idx.upsert("m", 0, 0, 0, CacheBackendKind::Http);
        idx.record_hit("m", 100);
        assert_eq!(idx.get("m").expect("key should exist").hit_count, 1);
    }
    #[test]
    fn cache_index_total_bytes() {
        let mut idx = CacheIndex::new();
        idx.upsert("a", 0, 1_000, 0, CacheBackendKind::Local);
        idx.upsert("b", 1, 2_000, 0, CacheBackendKind::Local);
        assert_eq!(idx.total_bytes(), 3_000);
    }
    #[test]
    fn multi_backend_cache_store_and_fetch() {
        let mut cache = MultiBackendCache::new(100);
        let key = CacheKey::new("Mod.A", "aabbccdd");
        cache.store(&key, b"compiled-artifact".to_vec(), 0);
        let result = cache.fetch(&key);
        assert!(result.is_some());
        assert_eq!(cache.hit_rate(), 1.0);
    }
    #[test]
    fn multi_backend_cache_miss() {
        let mut cache = MultiBackendCache::new(100);
        let key = CacheKey::new("Mod.B", "00000000");
        assert!(cache.fetch(&key).is_none());
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn multi_backend_cache_has_remote_backend() {
        let mut cache = MultiBackendCache::new(10);
        assert!(!cache.has_remote_backend());
        cache.add_backend(CacheBackendKind::S3, true);
        assert!(cache.has_remote_backend());
    }
    #[test]
    fn cache_key_hasher_deterministic() {
        let mut h1 = CacheKeyHasher::new();
        h1.mix_str("ModA").mix_u64(42);
        let mut h2 = CacheKeyHasher::new();
        h2.mix_str("ModA").mix_u64(42);
        assert_eq!(h1.finish_hex(), h2.finish_hex());
    }
    #[test]
    fn cache_key_hasher_into_cache_key() {
        let mut h = CacheKeyHasher::new();
        h.mix_str("Mathlib.Algebra");
        let key = h.into_cache_key("Mathlib.Algebra");
        assert_eq!(key.module_name, "Mathlib.Algebra");
        assert!(!key.hash.is_empty());
    }
    #[test]
    fn prefetch_list_ordering() {
        let mut list = CachePrefetchList::new();
        let k1 = CacheKey::new("A", "h1");
        let k2 = CacheKey::new("B", "h2");
        list.enqueue(&k1, 5);
        list.enqueue(&k2, 10);
        let (prio, _) = list.dequeue().expect("test operation should succeed");
        assert_eq!(prio, 10);
    }
    #[test]
    fn prefetch_list_empty_dequeue() {
        let mut list = CachePrefetchList::new();
        assert!(list.dequeue().is_none());
    }
    #[test]
    fn cache_metrics_hit_rate() {
        let mut m = CacheMetrics::new();
        m.record_hit(1024);
        m.record_hit(2048);
        m.record_miss();
        assert!((m.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn cache_metrics_summary_non_empty() {
        let m = CacheMetrics::new();
        assert!(!m.summary().is_empty());
    }
    #[test]
    fn artifact_manifest_round_trip() {
        let mut manifest = ArtifactManifest::new();
        let key = CacheKey::new("Mod.X", "deadcafe");
        manifest.add("Mod.X", &key, 4096);
        let text = manifest.to_text();
        let restored = ArtifactManifest::from_text(&text);
        assert_eq!(restored.len(), 1);
        assert_eq!(restored.total_size(), 4096);
    }
    #[test]
    fn artifact_manifest_empty_text() {
        let m = ArtifactManifest::from_text("");
        assert!(m.is_empty());
    }
    #[test]
    fn cache_pool_upload_and_count() {
        let mut pool = RemoteCachePool::new();
        pool.add("https://cache1.example.com");
        pool.add("https://cache2.example.com");
        assert_eq!(pool.client_count(), 2);
        let key = CacheKey::new("Mod", "ff");
        let n = pool.upload_all(&key, b"data");
        assert_eq!(n, 2);
    }
    #[test]
    fn cache_pool_fetch_any_all_miss() {
        let mut pool = RemoteCachePool::new();
        pool.add("https://cache.example.com");
        let key = CacheKey::new("Mod", "ff");
        assert!(pool.try_fetch_any(&key).is_none());
        assert_eq!(pool.total_misses(), 1);
    }
    #[test]
    fn cache_key_namespace_key_for() {
        let ns = CacheKeyNamespace::new("oxilean", "v0.1.0");
        let key = ns.key_for("Mathlib.Data.Nat", "abc");
        assert!(key.module_name.contains("oxilean"));
        assert!(key.module_name.contains("v0.1.0"));
    }
    #[test]
    fn lru_candidates_ordering() {
        let mut idx = CacheIndex::new();
        idx.upsert("old", 0, 100, 1_000, CacheBackendKind::Local);
        idx.upsert("new", 1, 200, 9_000, CacheBackendKind::Local);
        let mut candidates = lru_candidates(&idx, 10_000);
        sort_candidates(&mut candidates);
        assert_eq!(candidates[0].path_component, "old");
    }
    #[test]
    fn lfu_candidates_low_hit_count_first() {
        let mut idx = CacheIndex::new();
        idx.upsert("popular", 0, 100, 0, CacheBackendKind::Local);
        idx.upsert("unpopular", 1, 100, 0, CacheBackendKind::Local);
        for _ in 0..10 {
            idx.record_hit("popular", 0);
        }
        let mut candidates = lfu_candidates(&idx);
        sort_candidates(&mut candidates);
        assert_eq!(candidates[0].path_component, "unpopular");
    }
    #[test]
    fn capacity_budget_can_fit() {
        let mut budget = CacheCapacityBudget::new(1_000, 10);
        assert!(budget.can_fit(500));
        budget.add(600);
        assert!(!budget.can_fit(500));
    }
    #[test]
    fn capacity_budget_remove() {
        let mut budget = CacheCapacityBudget::new(1_000, 10);
        budget.add(400);
        budget.remove(400);
        assert_eq!(budget.used_bytes, 0);
        assert_eq!(budget.used_entries, 0);
    }
    #[test]
    fn capacity_budget_usage_pct() {
        let mut budget = CacheCapacityBudget::new(1_000, 10);
        budget.add(500);
        assert!((budget.usage_pct() - 50.0).abs() < 0.01);
    }
    #[test]
    fn cache_session_id_deterministic() {
        let id1 = CacheSessionId::from_seed("build-2026-01-01");
        let id2 = CacheSessionId::from_seed("build-2026-01-01");
        assert_eq!(id1, id2);
    }
    #[test]
    fn cache_session_id_display() {
        let id = CacheSessionId::from_seed("test");
        let s = format!("{}", id);
        assert!(s.starts_with("CacheSession("));
    }
    #[test]
    fn s3_config_object_key() {
        let cfg = S3CacheConfig::new("https://minio.local", "build-cache")
            .with_path_style()
            .with_region("eu-west-1");
        let key = cfg.object_key("Mod-v1_0-abc123");
        assert!(key.starts_with("oxilean-cache/"));
    }
    #[test]
    fn gcs_config_object_name() {
        let cfg = GcsCacheConfig::new("my-gcs-bucket")
            .with_service_account("/secrets/sa.json")
            .with_project("my-project");
        let name = cfg.object_name("Mod-v1_0-abc123");
        assert!(name.starts_with("oxilean/"));
    }
    #[test]
    fn http_config_artifact_url() {
        let cfg = HttpCacheConfig::new("https://cache.example.com/api")
            .with_token("my-token")
            .insecure();
        let url = cfg.artifact_url("Mod-v1_0-abc123");
        assert!(url.contains("/ac/Mod-v1_0-abc123"));
    }
    #[test]
    fn cache_backend_kind_display() {
        assert_eq!(format!("{}", CacheBackendKind::Http), "http");
        assert_eq!(format!("{}", CacheBackendKind::S3), "s3");
        assert_eq!(format!("{}", CacheBackendKind::Gcs), "gcs");
        assert_eq!(format!("{}", CacheBackendKind::Local), "local");
    }
    #[test]
    fn eviction_policy_display() {
        assert_eq!(format!("{}", EvictionPolicy::Lru), "lru");
        assert_eq!(format!("{}", EvictionPolicy::Lfu), "lfu");
        assert_eq!(format!("{}", EvictionPolicy::Fifo), "fifo");
        assert_eq!(format!("{}", EvictionPolicy::LargestFirst), "largest-first");
    }
    #[test]
    fn cache_warming_config_disabled() {
        let cfg = CacheWarmingConfig::disabled();
        assert!(!cfg.enabled);
        assert_eq!(cfg.module_count(), 0);
    }
    #[test]
    fn cache_warming_config_with_modules() {
        let cfg = CacheWarmingConfig::new(4)
            .with_module("Mathlib.Data.Nat")
            .with_module("Mathlib.Algebra.Ring");
        assert!(cfg.enabled);
        assert_eq!(cfg.module_count(), 2);
    }
}
#[cfg(test)]
mod extra2_tests {
    use super::*;
    #[test]
    fn health_checker_check_succeeds() {
        let mut hc = CacheHealthChecker::new("https://cache.example.com", 60);
        assert!(hc.check(1000));
        assert_eq!(hc.success_count, 1);
        assert!(hc.is_healthy);
    }
    #[test]
    fn health_checker_record_failure() {
        let mut hc = CacheHealthChecker::new("https://cache.example.com", 60);
        hc.record_failure(500);
        assert!(!hc.is_healthy);
        assert_eq!(hc.failure_count, 1);
    }
    #[test]
    fn health_checker_check_due() {
        let hc = CacheHealthChecker::new("https://cache.example.com", 60);
        assert!(hc.check_due(1000));
    }
    #[test]
    fn health_checker_reliability_pct() {
        let mut hc = CacheHealthChecker::new("https://cache.example.com", 60);
        hc.check(1);
        hc.check(2);
        hc.record_failure(3);
        let rel = hc.reliability_pct();
        assert!((rel - 200.0 / 3.0).abs() < 0.01);
    }
    #[test]
    fn cache_policy_read_write() {
        let p = CachePolicy::read_write();
        assert!(p.reads_enabled());
        assert!(p.writes_enabled());
    }
    #[test]
    fn cache_policy_read_only() {
        let p = CachePolicy::read_only();
        assert!(p.reads_enabled());
        assert!(!p.writes_enabled());
    }
    #[test]
    fn cache_policy_disabled() {
        let p = CachePolicy::disabled();
        assert!(!p.reads_enabled());
        assert!(!p.writes_enabled());
    }
    #[test]
    fn cache_write_policy_display() {
        assert_eq!(format!("{}", CacheWritePolicy::Synchronous), "synchronous");
        assert_eq!(format!("{}", CacheWritePolicy::ReadOnly), "read-only");
    }
    #[test]
    fn cache_read_policy_display() {
        assert_eq!(format!("{}", CacheReadPolicy::Enabled), "enabled");
        assert_eq!(format!("{}", CacheReadPolicy::Bypass), "bypass");
        assert_eq!(format!("{}", CacheReadPolicy::Verified), "verified");
    }
    #[test]
    fn compressed_artifact_uncompressed_ratio_is_one() {
        let artifact = CompressedArtifact::uncompressed(vec![0u8; 1000]);
        assert!((artifact.compression_ratio() - 1.0).abs() < 1e-9);
        assert!((artifact.space_saving_pct() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn compressed_artifact_zstd_tag() {
        let artifact = CompressedArtifact::compress_zstd(vec![0u8; 500]);
        assert_eq!(artifact.algorithm, "zstd");
        assert!(artifact.is_compressed);
    }
    #[test]
    fn cache_segment_record_add_remove() {
        let mut seg = CacheSegment::new("v0.1.0-x86_64");
        seg.record_add(1024);
        seg.record_add(2048);
        assert_eq!(seg.entry_count, 2);
        assert_eq!(seg.total_bytes, 3072);
        seg.record_remove(1024);
        assert_eq!(seg.entry_count, 1);
        assert_eq!(seg.total_bytes, 2048);
    }
    #[test]
    fn cache_segment_avg_artifact_size() {
        let mut seg = CacheSegment::new("seg");
        seg.record_add(1000);
        seg.record_add(3000);
        assert!((seg.avg_artifact_size() - 2000.0).abs() < 1e-9);
    }
    #[test]
    fn segment_manager_totals() {
        let mut mgr = CacheSegmentManager::new();
        mgr.record_add("v1", 500);
        mgr.record_add("v1", 500);
        mgr.record_add("v2", 1000);
        assert_eq!(mgr.total_bytes(), 2000);
        assert_eq!(mgr.total_entries(), 3);
        assert_eq!(mgr.segment_count(), 2);
    }
    #[test]
    fn segment_manager_deactivate() {
        let mut mgr = CacheSegmentManager::new();
        mgr.ensure_segment("v1");
        mgr.deactivate("v1");
        assert!(mgr.active_segment_ids().is_empty());
    }
    #[test]
    fn cache_quota_would_exceed() {
        let mut quota = CacheQuota::new("project-A", 1_000);
        quota.add_usage(800);
        assert!(quota.would_exceed(300));
        assert!(!quota.would_exceed(200));
    }
    #[test]
    fn cache_quota_release_usage() {
        let mut quota = CacheQuota::new("project-B", 1_000);
        quota.add_usage(500);
        quota.release_usage(500);
        assert_eq!(quota.used_bytes, 0);
    }
    #[test]
    fn cache_quota_usage_pct() {
        let mut quota = CacheQuota::new("project-C", 1_000);
        quota.add_usage(250);
        assert!((quota.usage_pct() - 25.0).abs() < 0.01);
    }
    #[test]
    fn cache_lease_is_valid() {
        let lease = CacheLease::new("mod-key", 1000, 300);
        assert!(lease.is_valid(1000));
        assert!(lease.is_valid(1299));
        assert!(!lease.is_valid(1300));
    }
    #[test]
    fn cache_lease_remaining_secs() {
        let lease = CacheLease::new("mod-key", 1000, 300);
        assert_eq!(lease.remaining_secs(1100), 200);
    }
    #[test]
    fn lease_manager_acquire_and_is_leased() {
        let mut mgr = CacheLeaseManager::new();
        mgr.acquire("key1", 1000, 300);
        assert!(mgr.is_leased("key1", 1100));
        assert!(!mgr.is_leased("key1", 1400));
    }
    #[test]
    fn lease_manager_release() {
        let mut mgr = CacheLeaseManager::new();
        mgr.acquire("key1", 1000, 300);
        mgr.release("key1");
        assert!(!mgr.is_leased("key1", 1000));
    }
    #[test]
    fn lease_manager_expire_old() {
        let mut mgr = CacheLeaseManager::new();
        mgr.acquire("a", 1000, 100);
        mgr.acquire("b", 1000, 500);
        let expired = mgr.expire_old(1200);
        assert_eq!(expired, 1);
        assert_eq!(mgr.active_count(), 1);
    }
}
#[cfg(test)]
mod snapshot_tests {
    use super::*;
    #[test]
    fn cache_stats_snapshot_fields() {
        let s = CacheStatsSnapshot::new(10, 4096, 0.75, 9999);
        assert_eq!(s.entry_count, 10);
        assert_eq!(s.total_bytes, 4096);
        assert!((s.hit_rate - 0.75).abs() < 1e-9);
        assert_eq!(s.timestamp, 9999);
    }
}
