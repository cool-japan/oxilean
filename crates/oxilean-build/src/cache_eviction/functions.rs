//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::{
    AdaptivePolicy, CacheEntry, CacheHitRateMonitor, CacheManager, CacheManagerWithStats,
    CachePressureDetector, CachePressureLevel, CacheSizeTracker, CacheUsageHistory,
    CacheUsageSample, CacheWarmupPlan, CombinedPolicy, CombinedWeights, EvictionAuditLog,
    EvictionHistogram, EvictionQuota, EvictionRunStats, EvictionStats, GroupedPolicy, LfuPolicy,
    LruPolicy, MultiTierCache, PriorityPolicy, RandomPolicy, SizeBasedPolicy, TtlPolicy,
    WarmupEntry, WorkloadHint,
};

/// A policy that selects which cache entries should be evicted.
///
/// Implementations receive a slice of entries and return the keys of entries
/// to evict, ordered from highest eviction priority to lowest.
pub trait EvictionPolicy {
    /// Given all current entries and the number of bytes that must be freed,
    /// return the keys of entries to evict (in eviction order).
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String>;
    /// Human-readable name of this policy.
    fn name(&self) -> &str;
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    /// Helper: create a CacheEntry with a given key and size.
    fn make_entry(key: &str, size: u64) -> CacheEntry {
        CacheEntry::new(key, size, vec![0u8; size as usize])
    }
    /// Helper: create a CacheEntry with custom access count and a small sleep
    /// to ensure distinct timestamps.
    fn make_entry_with_accesses(key: &str, size: u64, accesses: u64) -> CacheEntry {
        let mut entry = CacheEntry::new(key, size, vec![]);
        for _ in 0..accesses {
            entry.record_access();
        }
        entry
    }
    #[test]
    fn test_lru_eviction_order() {
        let now = Instant::now();
        let mut e1 = make_entry("old", 100);
        e1.last_access = now - Duration::from_secs(30);
        let mut e2 = make_entry("mid", 100);
        e2.last_access = now - Duration::from_secs(10);
        let mut e3 = make_entry("new", 100);
        e3.last_access = now;
        let policy = LruPolicy::new();
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let evicted = policy.select_evictions(&refs, 150);
        assert_eq!(evicted.len(), 2);
        assert_eq!(evicted[0], "old");
        assert_eq!(evicted[1], "mid");
    }
    #[test]
    fn test_lfu_eviction_order() {
        let mut e1 = make_entry("rarely", 100);
        e1.access_count = 1;
        let mut e2 = make_entry("sometimes", 100);
        e2.access_count = 5;
        let mut e3 = make_entry("often", 100);
        e3.access_count = 20;
        let policy = LfuPolicy::new();
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let evicted = policy.select_evictions(&refs, 100);
        assert_eq!(evicted.len(), 1);
        assert_eq!(evicted[0], "rarely");
    }
    #[test]
    fn test_size_based_eviction() {
        let e1 = make_entry("small", 10);
        let e2 = make_entry("medium", 50);
        let e3 = make_entry("large", 200);
        let policy = SizeBasedPolicy::new();
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let evicted = policy.select_evictions(&refs, 100);
        assert_eq!(evicted.len(), 1);
        assert_eq!(evicted[0], "large");
    }
    #[test]
    fn test_ttl_eviction() {
        let ttl = Duration::from_millis(50);
        let policy = TtlPolicy::new(ttl);
        let e1 = make_entry("expired", 100);
        thread::sleep(Duration::from_millis(60));
        let e2 = make_entry("fresh", 100);
        assert!(policy.is_expired(&e1));
        assert!(!policy.is_expired(&e2));
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 50);
        assert!(!evicted.is_empty());
        assert_eq!(evicted[0], "expired");
    }
    #[test]
    fn test_combined_policy() {
        let weights = CombinedWeights {
            recency: 0.0,
            frequency: 0.0,
            size: 10.0,
            age: 0.0,
        };
        let policy = CombinedPolicy::new(weights);
        let e1 = make_entry("small", 10);
        let e2 = make_entry("huge", 500);
        let e3 = make_entry("medium", 100);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let evicted = policy.select_evictions(&refs, 100);
        assert!(!evicted.is_empty());
        assert_eq!(evicted[0], "huge");
    }
    #[test]
    fn test_cache_manager_basic() {
        let mut mgr = CacheManager::new(1000, Box::new(LruPolicy::new()));
        let e1 = make_entry("a", 200);
        let e2 = make_entry("b", 300);
        mgr.insert(e1);
        mgr.insert(e2);
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.current_size(), 500);
        assert!(mgr.contains("a"));
        assert!(mgr.contains("b"));
        let entry = mgr.get("a");
        assert!(entry.is_some());
        assert_eq!(
            entry.expect("test operation should succeed").access_count,
            1
        );
        let removed = mgr.remove("b");
        assert!(removed.is_some());
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.current_size(), 200);
    }
    #[test]
    fn test_cache_manager_eviction_on_overflow() {
        let mut mgr = CacheManager::new(500, Box::new(LruPolicy::new()));
        mgr.insert(make_entry("first", 200));
        thread::sleep(Duration::from_millis(5));
        mgr.insert(make_entry("second", 200));
        thread::sleep(Duration::from_millis(5));
        mgr.get("first");
        thread::sleep(Duration::from_millis(5));
        let evicted = mgr.insert(make_entry("third", 300));
        assert!(evicted.contains(&"second".to_string()));
        assert!(!mgr.contains("second"));
        assert!(mgr.contains("first"));
        assert!(mgr.contains("third"));
        assert!(mgr.current_size() <= 500);
    }
    #[test]
    fn test_cache_manager_rejects_oversized() {
        let mut mgr = CacheManager::new(100, Box::new(LruPolicy::new()));
        let evicted = mgr.insert(make_entry("toobig", 200));
        assert!(evicted.is_empty());
        assert!(!mgr.contains("toobig"));
        assert_eq!(mgr.len(), 0);
    }
    #[test]
    fn test_cache_manager_replace_existing() {
        let mut mgr = CacheManager::new(500, Box::new(LruPolicy::new()));
        mgr.insert(make_entry("key", 100));
        assert_eq!(mgr.current_size(), 100);
        mgr.insert(make_entry("key", 200));
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.current_size(), 200);
    }
    #[test]
    fn test_cache_manager_evict_expired() {
        let mut mgr = CacheManager::new(1000, Box::new(LruPolicy::new()));
        mgr.insert(make_entry("old_entry", 100));
        thread::sleep(Duration::from_millis(60));
        mgr.insert(make_entry("new_entry", 100));
        let evicted = mgr.evict_expired(Duration::from_millis(50));
        assert!(evicted.contains(&"old_entry".to_string()));
        assert!(!evicted.contains(&"new_entry".to_string()));
        assert!(!mgr.contains("old_entry"));
        assert!(mgr.contains("new_entry"));
    }
    #[test]
    fn test_cache_manager_utilization() {
        let mut mgr = CacheManager::new(1000, Box::new(SizeBasedPolicy::new()));
        assert_eq!(mgr.policy_name(), "SizeBased");
        assert!((mgr.utilization() - 0.0).abs() < f64::EPSILON);
        mgr.insert(make_entry("half", 500));
        assert!((mgr.utilization() - 0.5).abs() < f64::EPSILON);
    }
    #[test]
    fn test_cache_manager_clear() {
        let mut mgr = CacheManager::new(1000, Box::new(LfuPolicy::new()));
        mgr.insert(make_entry("a", 100));
        mgr.insert(make_entry("b", 200));
        mgr.clear();
        assert!(mgr.is_empty());
        assert_eq!(mgr.current_size(), 0);
    }
    #[test]
    fn test_cache_entry_access_tracking() {
        let mut entry = make_entry("test", 64);
        assert_eq!(entry.access_count, 0);
        entry.record_access();
        entry.record_access();
        entry.record_access();
        assert_eq!(entry.access_count, 3);
    }
    #[test]
    fn test_lfu_tiebreak_by_recency() {
        let now = Instant::now();
        let mut e1 = make_entry("a", 100);
        e1.access_count = 5;
        e1.last_access = now - Duration::from_secs(20);
        let mut e2 = make_entry("b", 100);
        e2.access_count = 5;
        e2.last_access = now - Duration::from_secs(1);
        let policy = LfuPolicy::new();
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 100);
        assert_eq!(evicted.len(), 1);
        assert_eq!(evicted[0], "a");
    }
    #[test]
    fn test_combined_frequency_weight() {
        let weights = CombinedWeights {
            recency: 0.0,
            frequency: 10.0,
            size: 0.0,
            age: 0.0,
        };
        let policy = CombinedPolicy::new(weights);
        let e1 = make_entry_with_accesses("popular", 100, 50);
        let e2 = make_entry_with_accesses("moderate", 100, 10);
        let e3 = make_entry_with_accesses("unpopular", 100, 0);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let scores = policy.compute_scores(&refs);
        let score_map: HashMap<&str, f64> = scores.iter().map(|(k, s)| (k.as_str(), *s)).collect();
        assert!(
            score_map["unpopular"] > score_map["moderate"],
            "unpopular ({}) should score higher than moderate ({})",
            score_map["unpopular"],
            score_map["moderate"]
        );
        assert!(
            score_map["moderate"] > score_map["popular"],
            "moderate ({}) should score higher than popular ({})",
            score_map["moderate"],
            score_map["popular"]
        );
    }
    #[test]
    fn test_policy_names() {
        assert_eq!(LruPolicy::new().name(), "LRU");
        assert_eq!(LfuPolicy::new().name(), "LFU");
        assert_eq!(SizeBasedPolicy::new().name(), "SizeBased");
        assert_eq!(TtlPolicy::new(Duration::from_secs(60)).name(), "TTL");
        assert_eq!(
            CombinedPolicy::new(CombinedWeights::default()).name(),
            "Combined"
        );
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use std::time::{Duration, Instant};
    fn make_entry(key: &str, size: u64) -> CacheEntry {
        CacheEntry::new(key, size, vec![0u8; size as usize])
    }
    #[test]
    fn test_adaptive_policy_temporal() {
        let policy = AdaptivePolicy::new(WorkloadHint::TemporalLocality);
        assert_eq!(policy.name(), "Adaptive");
        let now = Instant::now();
        let mut e1 = make_entry("old", 100);
        e1.last_access = now - Duration::from_secs(60);
        let e2 = make_entry("new", 100);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 100);
        assert_eq!(evicted[0], "old");
    }
    #[test]
    fn test_adaptive_policy_large_blob() {
        let policy = AdaptivePolicy::new(WorkloadHint::LargeBlobHeavy);
        let e1 = make_entry("small", 10);
        let e2 = make_entry("big", 500);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 100);
        assert_eq!(evicted[0], "big");
    }
    #[test]
    fn test_random_policy_name() {
        let policy = RandomPolicy::new(42);
        assert_eq!(policy.name(), "Random");
    }
    #[test]
    fn test_random_policy_frees_enough() {
        let policy = RandomPolicy::new(123);
        let e1 = make_entry("a", 100);
        let e2 = make_entry("b", 100);
        let e3 = make_entry("c", 100);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2, &e3];
        let evicted = policy.select_evictions(&refs, 150);
        assert!(evicted.len() >= 2);
    }
    #[test]
    fn test_random_policy_empty() {
        let policy = RandomPolicy::new(0);
        let refs: Vec<&CacheEntry> = vec![];
        let evicted = policy.select_evictions(&refs, 100);
        assert!(evicted.is_empty());
    }
    #[test]
    fn test_priority_policy_evicts_lowest_priority() {
        let mut policy = PriorityPolicy::new(0);
        policy.set_priority("important", 100);
        policy.set_priority("disposable", 1);
        let e1 = make_entry("important", 200);
        let e2 = make_entry("disposable", 200);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 200);
        assert_eq!(evicted[0], "disposable");
    }
    #[test]
    fn test_priority_policy_default_priority() {
        let policy = PriorityPolicy::new(5);
        assert_eq!(policy.priority_of("unknown"), 5);
    }
    #[test]
    fn test_priority_policy_name() {
        let policy = PriorityPolicy::new(0);
        assert_eq!(policy.name(), "Priority");
    }
    #[test]
    fn test_grouped_policy_evicts_oldest_group_first() {
        let mut policy = GroupedPolicy::new();
        policy.assign_group("entry_a", "group_old");
        policy.assign_group("entry_b", "group_new");
        let e1 = make_entry("entry_a", 200);
        let e2 = make_entry("entry_b", 200);
        let refs: Vec<&CacheEntry> = vec![&e1, &e2];
        let evicted = policy.select_evictions(&refs, 200);
        assert_eq!(evicted[0], "entry_a");
    }
    #[test]
    fn test_grouped_policy_name() {
        let policy = GroupedPolicy::new();
        assert_eq!(policy.name(), "Grouped");
    }
    #[test]
    fn test_eviction_stats_record_round() {
        let mut stats = EvictionStats::new();
        stats.record_round(3, 300);
        stats.record_round(1, 100);
        assert_eq!(stats.eviction_rounds, 2);
        assert_eq!(stats.total_bytes_freed, 400);
        assert_eq!(stats.total_entries_evicted, 4);
        assert_eq!(stats.max_bytes_freed_per_round, 300);
        assert_eq!(stats.min_bytes_freed_per_round, 100);
    }
    #[test]
    fn test_eviction_stats_averages() {
        let mut stats = EvictionStats::new();
        stats.record_round(2, 200);
        stats.record_round(4, 400);
        assert!((stats.avg_bytes_freed_per_round() - 300.0).abs() < 1e-9);
        assert!((stats.avg_entries_per_round() - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_cache_manager_with_stats_insert() {
        let mut mgr = CacheManagerWithStats::new(200, Box::new(LruPolicy::new()));
        mgr.insert(make_entry("a", 100));
        mgr.insert(make_entry("b", 100));
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.stats().eviction_rounds, 0);
    }
    #[test]
    fn test_cache_manager_with_stats_eviction_stats() {
        let mut mgr = CacheManagerWithStats::new(150, Box::new(SizeBasedPolicy::new()));
        mgr.insert(make_entry("a", 100));
        mgr.insert(make_entry("b", 100));
        assert!(mgr.stats().eviction_rounds > 0);
    }
    #[test]
    fn test_cache_warmup_plan_execute() {
        let mut plan = CacheWarmupPlan::new();
        plan.add(WarmupEntry::new("mod_a", 100, 10));
        plan.add(WarmupEntry::new("mod_b", 100, 5));
        assert_eq!(plan.total_bytes(), 200);
        let mut cache = CacheManager::new(1000, Box::new(LruPolicy::new()));
        let inserted = plan.execute(&mut cache);
        assert_eq!(inserted, 2);
        assert!(cache.contains("mod_a"));
        assert!(cache.contains("mod_b"));
    }
    #[test]
    fn test_cache_warmup_plan_sort_priority() {
        let mut plan = CacheWarmupPlan::new();
        plan.add(WarmupEntry::new("low", 50, 1));
        plan.add(WarmupEntry::new("high", 50, 100));
        plan.sort_by_priority();
        assert_eq!(plan.entries[0].key, "high");
    }
    #[test]
    fn test_eviction_histogram_record() {
        let mut h = EvictionHistogram::default_buckets();
        h.record(512);
        h.record(2000);
        h.record(5_000_000);
        assert_eq!(h.total(), 3);
        assert_eq!(h.count_at(0), 1);
        assert_eq!(h.count_at(1), 1);
    }
    #[test]
    fn test_eviction_histogram_summary() {
        let mut h = EvictionHistogram::default_buckets();
        h.record(100);
        let s = h.to_summary();
        assert!(!s.is_empty());
        assert!(s.contains("B"));
    }
    #[test]
    fn test_cache_hit_rate_monitor() {
        let mut m = CacheHitRateMonitor::new(10);
        m.record(0, true);
        m.record(1, false);
        m.record(2, true);
        assert!((m.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
        assert_eq!(m.window_size(), 3);
    }
    #[test]
    fn test_cache_hit_rate_monitor_window_sliding() {
        let mut m = CacheHitRateMonitor::new(3);
        for i in 0u64..5 {
            m.record(i, false);
        }
        m.record(5, true);
        assert_eq!(m.window_size(), 3);
        assert!((m.hit_rate() - 1.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_cache_pressure_detector_eviction_rate() {
        let mut d = CachePressureDetector::new(0.9, 0.5);
        d.record_insert(0);
        d.record_insert(1);
        assert!((d.eviction_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_cache_pressure_detector_pressure() {
        let d = CachePressureDetector::new(0.8, 0.5);
        assert!(d.is_under_pressure(0.9));
        assert!(!d.is_under_pressure(0.5));
    }
    #[test]
    fn test_multi_tier_cache_basic() {
        let mut cache = MultiTierCache::new(
            100,
            500,
            Box::new(LruPolicy::new()),
            Box::new(LruPolicy::new()),
            3,
        );
        cache.insert(make_entry("mod_a", 50));
        assert!(cache.contains("mod_a"));
        assert_eq!(cache.total_entries(), 1);
    }
    #[test]
    fn test_multi_tier_cache_clear() {
        let mut cache = MultiTierCache::new(
            100,
            500,
            Box::new(LruPolicy::new()),
            Box::new(LruPolicy::new()),
            3,
        );
        cache.insert(make_entry("x", 50));
        cache.clear();
        assert_eq!(cache.total_entries(), 0);
    }
    #[test]
    fn test_multi_tier_cache_get_promotes() {
        let mut cache = MultiTierCache::new(
            500,
            1000,
            Box::new(LruPolicy::new()),
            Box::new(LruPolicy::new()),
            2,
        );
        let mut entry = make_entry("promo", 50);
        entry.access_count = 2;
        cache.l2.insert(entry);
        let found = cache.get("promo");
        assert!(found.is_some());
    }
}
#[cfg(test)]
mod eviction_extra_tests {
    use super::*;
    #[test]
    fn usage_history_avg_hit_rate() {
        let mut h = CacheUsageHistory::with_capacity(10);
        h.record(CacheUsageSample::new(0, 10, 1000, 0.8));
        h.record(CacheUsageSample::new(1, 12, 1200, 0.6));
        assert!((h.avg_hit_rate() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn usage_history_peak() {
        let mut h = CacheUsageHistory::with_capacity(5);
        h.record(CacheUsageSample::new(0, 5, 500, 0.9));
        h.record(CacheUsageSample::new(1, 20, 2000, 0.7));
        assert_eq!(h.peak_entry_count(), 20);
        assert_eq!(h.peak_bytes(), 2000);
    }
    #[test]
    fn usage_history_evicts_at_capacity() {
        let mut h = CacheUsageHistory::with_capacity(3);
        for i in 0u64..5 {
            h.record(CacheUsageSample::new(i, i as usize, i * 100, 0.5));
        }
        assert_eq!(h.len(), 3);
    }
    #[test]
    fn eviction_audit_log_record() {
        let mut log = EvictionAuditLog::new();
        log.record("key1");
        log.record("key2");
        assert_eq!(log.len(), 2);
        assert!(log.keys().contains(&"key1".to_string()));
    }
    #[test]
    fn cache_pressure_from_utilization() {
        assert_eq!(
            CachePressureLevel::from_utilization(0.3),
            CachePressureLevel::Low
        );
        assert_eq!(
            CachePressureLevel::from_utilization(0.6),
            CachePressureLevel::Medium
        );
        assert_eq!(
            CachePressureLevel::from_utilization(0.8),
            CachePressureLevel::High
        );
        assert_eq!(
            CachePressureLevel::from_utilization(0.95),
            CachePressureLevel::Critical
        );
    }
    #[test]
    fn cache_pressure_should_evict() {
        assert!(!CachePressureLevel::Low.should_evict());
        assert!(!CachePressureLevel::Medium.should_evict());
        assert!(CachePressureLevel::High.should_evict());
        assert!(CachePressureLevel::Critical.should_evict());
    }
    #[test]
    fn cache_pressure_display() {
        assert_eq!(format!("{}", CachePressureLevel::Low), "low");
        assert_eq!(format!("{}", CachePressureLevel::Critical), "critical");
    }
    #[test]
    fn eviction_quota_pressure() {
        let mut q = EvictionQuota::new("ns", 1000);
        q.add(850);
        assert_eq!(q.pressure(), CachePressureLevel::High);
    }
    #[test]
    fn eviction_quota_would_exceed() {
        let mut q = EvictionQuota::new("ns", 1000);
        q.add(900);
        assert!(q.would_exceed(200));
        assert!(!q.would_exceed(50));
    }
    #[test]
    fn cache_size_tracker_can_fit() {
        let mut t = CacheSizeTracker::new(1000, 10);
        t.add(600);
        assert!(t.can_fit(200));
        assert!(!t.can_fit(500));
    }
    #[test]
    fn cache_size_tracker_pressure() {
        let mut t = CacheSizeTracker::new(1000, 100);
        t.add(800);
        assert_eq!(t.pressure(), CachePressureLevel::High);
    }
}
/// Returns the default eviction threshold (fraction of max capacity).
pub fn default_eviction_threshold() -> f64 {
    0.80
}
#[cfg(test)]
mod eviction_threshold_test {
    use super::*;
    #[test]
    fn default_threshold_in_range() {
        let t = default_eviction_threshold();
        assert!(t > 0.0 && t <= 1.0);
    }
}
#[cfg(test)]
mod eviction_run_stats_tests {
    use super::*;
    #[test]
    fn eviction_run_stats_record_run() {
        let mut s = EvictionRunStats::new();
        s.record_run(10, 1024, 5);
        s.record_run(20, 2048, 10);
        assert_eq!(s.eviction_runs, 2);
        assert_eq!(s.entries_evicted, 30);
        assert!((s.avg_entries_per_run() - 15.0).abs() < 1e-9);
    }
    #[test]
    fn eviction_run_stats_summary() {
        let s = EvictionRunStats::new();
        assert!(!s.summary().is_empty());
    }
}
/// Returns the eviction subsystem version.
pub fn eviction_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[cfg(test)]
mod eviction_version_test {
    use super::*;
    #[test]
    fn eviction_version_nonempty() {
        assert!(!eviction_version().is_empty());
    }
}
/// Returns the maximum number of entries per eviction run by default.
pub fn default_max_evictions_per_run() -> usize {
    100
}
/// Returns whether adaptive eviction scheduling is supported.
pub fn adaptive_scheduling_supported() -> bool {
    true
}
#[cfg(test)]
mod eviction_helpers_tests {
    use super::*;
    #[test]
    fn max_evictions_positive() {
        assert!(default_max_evictions_per_run() > 0);
    }
    #[test]
    fn adaptive_scheduling_is_supported() {
        assert!(adaptive_scheduling_supported());
    }
}
/// Compute a simple eviction score for an entry (higher = evict sooner).
/// Uses a combination of age and inverse access frequency.
pub fn compute_eviction_score(age_secs: u64, access_count: u64) -> f64 {
    let age_f = age_secs as f64;
    let access_f = (access_count as f64 + 1.0).ln();
    age_f / access_f
}
#[cfg(test)]
mod eviction_score_test {
    use super::*;
    #[test]
    fn score_increases_with_age() {
        let old = compute_eviction_score(1000, 5);
        let young = compute_eviction_score(100, 5);
        assert!(old > young);
    }
    #[test]
    fn score_decreases_with_access() {
        let popular = compute_eviction_score(500, 100);
        let unused = compute_eviction_score(500, 0);
        assert!(unused > popular);
    }
}
/// Returns the default cache warmup batch size.
pub fn default_warmup_batch_size() -> usize {
    16
}
/// Returns the default minimum access count to retain an entry under LFU.
pub fn default_lfu_min_access() -> u64 {
    2
}
/// Returns the default LRU max idle time in seconds.
pub fn default_lru_max_idle_secs() -> u64 {
    3600
}
