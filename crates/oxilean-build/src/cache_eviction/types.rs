//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use super::functions::EvictionPolicy;

/// A cache manager that additionally tracks detailed eviction statistics.
pub struct CacheManagerWithStats {
    pub(super) inner: CacheManager,
    pub(super) stats: EvictionStats,
}
impl CacheManagerWithStats {
    /// Create a new tracked cache manager.
    pub fn new(max_capacity: u64, policy: Box<dyn EvictionPolicy>) -> Self {
        Self {
            inner: CacheManager::new(max_capacity, policy),
            stats: EvictionStats::new(),
        }
    }
    /// Insert an entry and record eviction stats.
    pub fn insert(&mut self, entry: CacheEntry) -> Vec<String> {
        let evicted = self.inner.insert(entry);
        if !evicted.is_empty() {
            let bytes_freed: u64 = evicted.len() as u64;
            self.stats.record_round(evicted.len() as u64, bytes_freed);
        }
        evicted
    }
    /// Delegate to inner get.
    pub fn get(&mut self, key: &str) -> Option<&CacheEntry> {
        self.inner.get(key)
    }
    /// Delegate to inner remove.
    pub fn remove(&mut self, key: &str) -> Option<CacheEntry> {
        self.inner.remove(key)
    }
    /// Delegate to inner contains.
    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains(key)
    }
    /// Return eviction statistics.
    pub fn stats(&self) -> &EvictionStats {
        &self.stats
    }
    /// Current size.
    pub fn current_size(&self) -> u64 {
        self.inner.current_size()
    }
    /// Max capacity.
    pub fn max_capacity(&self) -> u64 {
        self.inner.max_capacity()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Clear.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    /// Utilization.
    pub fn utilization(&self) -> f64 {
        self.inner.utilization()
    }
}
/// How much pressure the cache is under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CachePressureLevel {
    /// Below 50% capacity.
    Low,
    /// 50–75% capacity.
    Medium,
    /// 75–90% capacity.
    High,
    /// Above 90% capacity.
    Critical,
}
impl CachePressureLevel {
    /// Compute pressure from a utilization fraction (0.0–1.0).
    pub fn from_utilization(util: f64) -> Self {
        if util < 0.5 {
            CachePressureLevel::Low
        } else if util < 0.75 {
            CachePressureLevel::Medium
        } else if util < 0.90 {
            CachePressureLevel::High
        } else {
            CachePressureLevel::Critical
        }
    }
    /// Whether eviction should be triggered.
    pub fn should_evict(&self) -> bool {
        *self >= CachePressureLevel::High
    }
}
/// Aggregate eviction statistics for a cache session.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct EvictionRunStats {
    /// Total eviction runs.
    pub eviction_runs: u64,
    /// Total entries evicted.
    pub entries_evicted: u64,
    /// Total bytes freed.
    pub bytes_freed: u64,
    /// Total time spent evicting in milliseconds.
    pub eviction_ms: u64,
}
#[allow(dead_code)]
impl EvictionRunStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an eviction run.
    pub fn record_run(&mut self, entries: u64, bytes: u64, elapsed_ms: u64) {
        self.eviction_runs += 1;
        self.entries_evicted += entries;
        self.bytes_freed += bytes;
        self.eviction_ms += elapsed_ms;
    }
    /// Average entries evicted per run.
    pub fn avg_entries_per_run(&self) -> f64 {
        if self.eviction_runs == 0 {
            0.0
        } else {
            self.entries_evicted as f64 / self.eviction_runs as f64
        }
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "runs={} evicted={} freed={}B ms={}",
            self.eviction_runs, self.entries_evicted, self.bytes_freed, self.eviction_ms,
        )
    }
}
/// Least Frequently Used eviction: evicts entries with the lowest access count.
/// Ties are broken by oldest last_access.
#[derive(Clone, Debug, Default)]
pub struct LfuPolicy;
impl LfuPolicy {
    pub fn new() -> Self {
        Self
    }
}
/// Weighted scoring for the combined eviction strategy.
#[derive(Clone, Debug)]
pub struct CombinedWeights {
    /// Weight for recency (higher = prefer evicting older-accessed entries).
    pub recency: f64,
    /// Weight for frequency (higher = prefer evicting less-accessed entries).
    pub frequency: f64,
    /// Weight for size (higher = prefer evicting larger entries).
    pub size: f64,
    /// Weight for age (higher = prefer evicting older-created entries).
    pub age: f64,
}
/// Manages a set of cache entries and applies an eviction policy when the
/// total cache size exceeds the configured capacity.
pub struct CacheManager {
    /// All current cache entries, keyed by entry key.
    pub(super) entries: HashMap<String, CacheEntry>,
    /// Maximum total cache size in bytes.
    pub(super) max_capacity: u64,
    /// Current total size in bytes.
    pub(super) current_size: u64,
    /// The eviction policy to use.
    pub(super) policy: Box<dyn EvictionPolicy>,
    /// Running count of evictions performed.
    pub(super) eviction_count: u64,
}
impl CacheManager {
    /// Create a new cache manager with the given capacity and eviction policy.
    pub fn new(max_capacity: u64, policy: Box<dyn EvictionPolicy>) -> Self {
        Self {
            entries: HashMap::new(),
            max_capacity,
            current_size: 0,
            policy,
            eviction_count: 0,
        }
    }
    /// Insert an entry into the cache. If the cache would exceed capacity,
    /// the configured eviction policy is applied to make room.
    /// Returns the list of keys evicted to make room (empty if none).
    pub fn insert(&mut self, entry: CacheEntry) -> Vec<String> {
        let mut evicted_keys = Vec::new();
        if entry.size_bytes > self.max_capacity {
            return evicted_keys;
        }
        if let Some(old) = self.entries.remove(&entry.key) {
            self.current_size = self.current_size.saturating_sub(old.size_bytes);
        }
        while self.current_size + entry.size_bytes > self.max_capacity {
            let entry_refs: Vec<&CacheEntry> = self.entries.values().collect();
            if entry_refs.is_empty() {
                break;
            }
            let overflow = (self.current_size + entry.size_bytes) - self.max_capacity;
            let keys_to_evict = self.policy.select_evictions(&entry_refs, overflow);
            if keys_to_evict.is_empty() {
                break;
            }
            for k in &keys_to_evict {
                if let Some(removed) = self.entries.remove(k) {
                    self.current_size = self.current_size.saturating_sub(removed.size_bytes);
                    self.eviction_count += 1;
                    evicted_keys.push(k.clone());
                }
            }
        }
        self.current_size += entry.size_bytes;
        self.entries.insert(entry.key.clone(), entry);
        evicted_keys
    }
    /// Access (get) an entry, recording the access.
    pub fn get(&mut self, key: &str) -> Option<&CacheEntry> {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.record_access();
        }
        self.entries.get(key)
    }
    /// Remove an entry by key.
    pub fn remove(&mut self, key: &str) -> Option<CacheEntry> {
        if let Some(entry) = self.entries.remove(key) {
            self.current_size = self.current_size.saturating_sub(entry.size_bytes);
            Some(entry)
        } else {
            None
        }
    }
    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Current total cached size in bytes.
    pub fn current_size(&self) -> u64 {
        self.current_size
    }
    /// Maximum capacity in bytes.
    pub fn max_capacity(&self) -> u64 {
        self.max_capacity
    }
    /// Utilization ratio (0.0 to 1.0).
    pub fn utilization(&self) -> f64 {
        if self.max_capacity == 0 {
            return 0.0;
        }
        self.current_size as f64 / self.max_capacity as f64
    }
    /// Total number of evictions that have occurred.
    pub fn eviction_count(&self) -> u64 {
        self.eviction_count
    }
    /// Name of the active eviction policy.
    pub fn policy_name(&self) -> &str {
        self.policy.name()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_size = 0;
    }
    /// Run TTL-based cleanup: remove all entries older than the given max age.
    /// Returns the keys of evicted entries.
    pub fn evict_expired(&mut self, max_age: Duration) -> Vec<String> {
        let expired_keys: Vec<String> = self
            .entries
            .values()
            .filter(|e| e.age() >= max_age)
            .map(|e| e.key.clone())
            .collect();
        let mut evicted = Vec::new();
        for key in expired_keys {
            if let Some(entry) = self.entries.remove(&key) {
                self.current_size = self.current_size.saturating_sub(entry.size_bytes);
                self.eviction_count += 1;
                evicted.push(key);
            }
        }
        evicted
    }
    /// Check whether a key is present.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }
    /// Return all keys currently in the cache.
    pub fn keys(&self) -> Vec<&str> {
        self.entries.keys().map(|k| k.as_str()).collect()
    }
}
/// A two-tier cache where L1 is fast/small and L2 is slower/larger.
pub struct MultiTierCache {
    pub(super) l1: CacheManager,
    pub(super) l2: CacheManager,
    /// Promotion threshold: entries in L2 with this many hits are promoted to L1.
    promote_threshold: u64,
}
impl MultiTierCache {
    /// Create a two-tier cache.
    pub fn new(
        l1_capacity: u64,
        l2_capacity: u64,
        l1_policy: Box<dyn EvictionPolicy>,
        l2_policy: Box<dyn EvictionPolicy>,
        promote_threshold: u64,
    ) -> Self {
        Self {
            l1: CacheManager::new(l1_capacity, l1_policy),
            l2: CacheManager::new(l2_capacity, l2_policy),
            promote_threshold,
        }
    }
    /// Insert an entry into L2 (new entries start cold).
    pub fn insert(&mut self, entry: CacheEntry) {
        self.l2.insert(entry);
    }
    /// Get an entry, checking L1 first then L2.
    /// Promotes L2 entries that exceed the access threshold.
    pub fn get(&mut self, key: &str) -> Option<()> {
        if self.l1.get(key).is_some() {
            return Some(());
        }
        if let Some(entry) = self.l2.get(key) {
            if entry.access_count >= self.promote_threshold {
                let entry = entry.clone();
                self.l2.remove(key);
                self.l1.insert(entry);
            }
            return Some(());
        }
        None
    }
    /// Whether a key exists in either tier.
    pub fn contains(&self, key: &str) -> bool {
        self.l1.contains(key) || self.l2.contains(key)
    }
    /// Total entries across both tiers.
    pub fn total_entries(&self) -> usize {
        self.l1.len() + self.l2.len()
    }
    /// L1 utilization [0.0, 1.0].
    pub fn l1_utilization(&self) -> f64 {
        self.l1.utilization()
    }
    /// L2 utilization [0.0, 1.0].
    pub fn l2_utilization(&self) -> f64 {
        self.l2.utilization()
    }
    /// Clear both tiers.
    pub fn clear(&mut self) {
        self.l1.clear();
        self.l2.clear();
    }
}
/// Least Recently Used eviction: evicts entries that have not been accessed
/// for the longest time.
#[derive(Clone, Debug, Default)]
pub struct LruPolicy;
impl LruPolicy {
    pub fn new() -> Self {
        Self
    }
}
/// A random eviction policy — useful as a baseline comparison.
#[derive(Clone, Debug, Default)]
pub struct RandomPolicy {
    pub(super) seed: u64,
}
impl RandomPolicy {
    /// Create a new random policy with the given seed.
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    /// Simple LCG pseudo-random number generator for determinism.
    pub(super) fn lcg_next(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *state
    }
}
/// An eviction policy that uses explicit user-assigned priorities.
/// Lower priority number → evicted first.
#[derive(Clone, Debug, Default)]
pub struct PriorityPolicy {
    /// Priority map: key → priority (higher = more important).
    priorities: HashMap<String, i32>,
    /// Default priority for entries not in the map.
    default_priority: i32,
}
impl PriorityPolicy {
    /// Create a new priority policy with a default priority.
    pub fn new(default_priority: i32) -> Self {
        Self {
            priorities: HashMap::new(),
            default_priority,
        }
    }
    /// Assign a priority to a cache key.
    pub fn set_priority(&mut self, key: &str, priority: i32) {
        self.priorities.insert(key.to_string(), priority);
    }
    /// Get the priority for a key.
    pub fn priority_of(&self, key: &str) -> i32 {
        self.priorities
            .get(key)
            .copied()
            .unwrap_or(self.default_priority)
    }
    /// Remove the priority assignment for a key.
    pub fn clear_priority(&mut self, key: &str) {
        self.priorities.remove(key);
    }
}
/// Metadata for a single cache entry.
#[derive(Clone, Debug)]
pub struct CacheEntry {
    /// Unique key identifying this entry.
    pub key: String,
    /// Size of the cached artifact in bytes.
    pub size_bytes: u64,
    /// Number of times this entry has been accessed.
    pub access_count: u64,
    /// Timestamp of the most recent access.
    pub last_access: Instant,
    /// Timestamp when the entry was first inserted.
    pub created_at: Instant,
    /// Optional payload (opaque bytes for the caller).
    pub data: Vec<u8>,
}
impl CacheEntry {
    /// Create a new cache entry with the given key, size, and data.
    pub fn new(key: impl Into<String>, size_bytes: u64, data: Vec<u8>) -> Self {
        let now = Instant::now();
        Self {
            key: key.into(),
            size_bytes,
            access_count: 0,
            last_access: now,
            created_at: now,
            data,
        }
    }
    /// Record an access, incrementing the counter and updating the timestamp.
    pub fn record_access(&mut self) {
        self.access_count += 1;
        self.last_access = Instant::now();
    }
    /// Age of this entry (time since creation).
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
    /// Time since last access.
    pub fn idle_time(&self) -> Duration {
        self.last_access.elapsed()
    }
}
/// Tracks total cache size and enforces limits.
pub struct CacheSizeTracker {
    pub max_bytes: u64,
    pub max_entries: usize,
    pub used_bytes: u64,
    pub used_entries: usize,
}
impl CacheSizeTracker {
    /// Create a tracker with limits.
    pub fn new(max_bytes: u64, max_entries: usize) -> Self {
        Self {
            max_bytes,
            max_entries,
            used_bytes: 0,
            used_entries: 0,
        }
    }
    /// Whether we can add `bytes` more.
    pub fn can_fit(&self, bytes: u64) -> bool {
        self.used_bytes + bytes <= self.max_bytes && self.used_entries < self.max_entries
    }
    /// Record an addition.
    pub fn add(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_add(bytes);
        self.used_entries += 1;
    }
    /// Record a removal.
    pub fn remove(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
        if self.used_entries > 0 {
            self.used_entries -= 1;
        }
    }
    /// Usage fraction (bytes / max_bytes).
    pub fn byte_utilization(&self) -> f64 {
        if self.max_bytes == 0 {
            0.0
        } else {
            self.used_bytes as f64 / self.max_bytes as f64
        }
    }
    /// Pressure level.
    pub fn pressure(&self) -> CachePressureLevel {
        CachePressureLevel::from_utilization(self.byte_utilization())
    }
}
/// A histogram of evicted entry sizes (bytes) for analysis.
#[derive(Debug, Default)]
pub struct EvictionHistogram {
    /// Bucket boundaries in bytes (exclusive upper bounds).
    buckets: Vec<u64>,
    /// Counts per bucket.
    counts: Vec<u64>,
}
impl EvictionHistogram {
    /// Create a histogram with given bucket boundaries.
    /// `buckets` should be sorted ascending.
    pub fn new(buckets: Vec<u64>) -> Self {
        let len = buckets.len() + 1;
        Self {
            buckets,
            counts: vec![0; len],
        }
    }
    /// Create a default histogram with powers-of-two boundaries.
    pub fn default_buckets() -> Self {
        let buckets = vec![1024, 4096, 16384, 65536, 262144, 1_048_576];
        Self::new(buckets)
    }
    /// Record an evicted entry of `size_bytes`.
    pub fn record(&mut self, size_bytes: u64) {
        let idx = self
            .buckets
            .iter()
            .position(|&b| size_bytes < b)
            .unwrap_or(self.buckets.len());
        self.counts[idx] += 1;
    }
    /// Return the bucket count at the given bucket index.
    pub fn count_at(&self, idx: usize) -> u64 {
        self.counts.get(idx).copied().unwrap_or(0)
    }
    /// Total recorded entries.
    pub fn total(&self) -> u64 {
        self.counts.iter().sum()
    }
    /// Return a human-readable summary of the histogram.
    pub fn to_summary(&self) -> String {
        let mut s = String::new();
        for (i, &count) in self.counts.iter().enumerate() {
            let label = if i == 0 && !self.buckets.is_empty() {
                format!("< {} B", self.buckets[0])
            } else if i < self.buckets.len() {
                format!("{} - {} B", self.buckets[i - 1], self.buckets[i])
            } else {
                format!(">= {} B", self.buckets.last().copied().unwrap_or(0))
            };
            s.push_str(&format!("{}: {}\n", label, count));
        }
        s
    }
}
/// Workload hint used to guide adaptive policy selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkloadHint {
    /// Lots of repeated accesses to the same files (temporal locality).
    TemporalLocality,
    /// Each file accessed roughly once (streaming).
    Streaming,
    /// Large artifacts dominate (size matters most).
    LargeBlobHeavy,
    /// No specific hint; use balanced default.
    Balanced,
}
/// Combined eviction policy that uses a weighted score across multiple factors.
///
/// Each entry receives a score:
///   `score = w_recency * recency_rank + w_frequency * frequency_rank
///          + w_size * size_rank + w_age * age_rank`
///
/// Entries with the highest score (worst combined rank) are evicted first.
#[derive(Clone, Debug)]
pub struct CombinedPolicy {
    pub weights: CombinedWeights,
}
impl CombinedPolicy {
    pub fn new(weights: CombinedWeights) -> Self {
        Self { weights }
    }
    /// Compute eviction scores for each entry. Higher score = evict sooner.
    pub(super) fn compute_scores(&self, entries: &[&CacheEntry]) -> Vec<(String, f64)> {
        let n = entries.len();
        if n == 0 {
            return Vec::new();
        }
        let mut recency_ranked: Vec<(usize, Instant)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.last_access))
            .collect();
        recency_ranked.sort_by(|a, b| a.1.cmp(&b.1));
        let mut recency_rank = vec![0.0f64; n];
        for (rank, &(idx, _)) in recency_ranked.iter().enumerate() {
            recency_rank[idx] = (n - 1 - rank) as f64;
        }
        let mut freq_ranked: Vec<(usize, u64)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.access_count))
            .collect();
        freq_ranked.sort_by(|a, b| a.1.cmp(&b.1));
        let mut freq_rank = vec![0.0f64; n];
        for (rank, &(idx, _)) in freq_ranked.iter().enumerate() {
            freq_rank[idx] = (n - 1 - rank) as f64;
        }
        let mut size_ranked: Vec<(usize, u64)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.size_bytes))
            .collect();
        size_ranked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut size_rank = vec![0.0f64; n];
        for (rank, &(idx, _)) in size_ranked.iter().enumerate() {
            size_rank[idx] = (n - 1 - rank) as f64;
        }
        let mut age_ranked: Vec<(usize, Instant)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.created_at))
            .collect();
        age_ranked.sort_by(|a, b| a.1.cmp(&b.1));
        let mut age_rank = vec![0.0f64; n];
        for (rank, &(idx, _)) in age_ranked.iter().enumerate() {
            age_rank[idx] = (n - 1 - rank) as f64;
        }
        let w = &self.weights;
        let max_rank = (n - 1).max(1) as f64;
        entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let score = w.recency * (recency_rank[i] / max_rank)
                    + w.frequency * (freq_rank[i] / max_rank)
                    + w.size * (size_rank[i] / max_rank)
                    + w.age * (age_rank[i] / max_rank);
                (e.key.clone(), score)
            })
            .collect()
    }
}
/// An adaptive eviction policy that selects the best sub-policy
/// based on a workload hint.
#[derive(Debug)]
pub struct AdaptivePolicy {
    pub(super) hint: WorkloadHint,
    pub(super) lru: LruPolicy,
    pub(super) lfu: LfuPolicy,
    pub(super) size: SizeBasedPolicy,
}
impl AdaptivePolicy {
    /// Create a new adaptive policy with the given workload hint.
    pub fn new(hint: WorkloadHint) -> Self {
        Self {
            hint,
            lru: LruPolicy::new(),
            lfu: LfuPolicy::new(),
            size: SizeBasedPolicy::new(),
        }
    }
    /// Update the workload hint.
    pub fn set_hint(&mut self, hint: WorkloadHint) {
        self.hint = hint;
    }
}
/// Statistics collected across multiple eviction rounds.
#[derive(Debug, Default, Clone)]
pub struct EvictionStats {
    /// Total bytes freed by evictions.
    pub total_bytes_freed: u64,
    /// Total number of entries evicted.
    pub total_entries_evicted: u64,
    /// Number of eviction rounds triggered.
    pub eviction_rounds: u64,
    /// Maximum bytes freed in a single round.
    pub max_bytes_freed_per_round: u64,
    /// Minimum bytes freed in a single round (excluding zero-byte rounds).
    pub min_bytes_freed_per_round: u64,
}
impl EvictionStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one eviction round.
    pub fn record_round(&mut self, entries_evicted: u64, bytes_freed: u64) {
        self.total_bytes_freed += bytes_freed;
        self.total_entries_evicted += entries_evicted;
        self.eviction_rounds += 1;
        if bytes_freed > self.max_bytes_freed_per_round {
            self.max_bytes_freed_per_round = bytes_freed;
        }
        if bytes_freed > 0 {
            if self.min_bytes_freed_per_round == 0 || bytes_freed < self.min_bytes_freed_per_round {
                self.min_bytes_freed_per_round = bytes_freed;
            }
        }
    }
    /// Average bytes freed per eviction round.
    pub fn avg_bytes_freed_per_round(&self) -> f64 {
        if self.eviction_rounds == 0 {
            0.0
        } else {
            self.total_bytes_freed as f64 / self.eviction_rounds as f64
        }
    }
    /// Average entries evicted per round.
    pub fn avg_entries_per_round(&self) -> f64 {
        if self.eviction_rounds == 0 {
            0.0
        } else {
            self.total_entries_evicted as f64 / self.eviction_rounds as f64
        }
    }
}
/// A single entry in a cache warmup plan.
#[derive(Debug, Clone)]
pub struct WarmupEntry {
    /// Cache key.
    pub key: String,
    /// Artifact size in bytes.
    pub size_bytes: u64,
    /// Priority hint (used for ordering warmup).
    pub priority: i32,
}
impl WarmupEntry {
    /// Create a new warmup entry.
    pub fn new(key: &str, size_bytes: u64, priority: i32) -> Self {
        Self {
            key: key.to_string(),
            size_bytes,
            priority,
        }
    }
}
/// Groups entries by a label and evicts from the oldest groups first.
#[derive(Clone, Debug, Default)]
pub struct GroupedPolicy {
    /// Map from entry key → group label.
    groups: HashMap<String, String>,
    /// Group creation order (group label → insertion order index).
    pub(super) group_order: HashMap<String, usize>,
    next_order: usize,
}
impl GroupedPolicy {
    /// Create a new grouped policy.
    pub fn new() -> Self {
        Self::default()
    }
    /// Assign an entry to a group.
    pub fn assign_group(&mut self, key: &str, group: &str) {
        self.groups.insert(key.to_string(), group.to_string());
        if !self.group_order.contains_key(group) {
            self.group_order.insert(group.to_string(), self.next_order);
            self.next_order += 1;
        }
    }
    /// Group label for a key (or "default" if not assigned).
    pub fn group_of<'a>(&'a self, key: &str) -> &'a str {
        self.groups
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or("default")
    }
}
/// Plans and executes a cache warmup sequence.
pub struct CacheWarmupPlan {
    pub(crate) entries: Vec<WarmupEntry>,
}
impl CacheWarmupPlan {
    /// Create an empty warmup plan.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Add an entry to the warmup plan.
    pub fn add(&mut self, entry: WarmupEntry) {
        self.entries.push(entry);
    }
    /// Sort entries by priority descending (highest priority warmed up first).
    pub fn sort_by_priority(&mut self) {
        self.entries.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    /// Execute the warmup plan: insert entries into the cache manager.
    /// Returns the count of successfully inserted entries.
    pub fn execute(&self, cache: &mut CacheManager) -> usize {
        let mut inserted = 0;
        for entry in &self.entries {
            let ce = CacheEntry::new(entry.key.clone(), entry.size_bytes, vec![]);
            let evicted = cache.insert(ce);
            if evicted.is_empty() || cache.contains(&entry.key) {
                inserted += 1;
            }
        }
        inserted
    }
    /// Number of entries in the plan.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the plan is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Total bytes needed to warm the entire plan.
    pub fn total_bytes(&self) -> u64 {
        self.entries.iter().map(|e| e.size_bytes).sum()
    }
}
/// Monitors cache hit rate using a sliding window of recent accesses.
#[derive(Debug, Default)]
pub struct CacheHitRateMonitor {
    /// Ring buffer of (ts, is_hit) pairs.
    window: std::collections::VecDeque<(u64, bool)>,
    /// Window size in number of accesses.
    max_window: usize,
}
impl CacheHitRateMonitor {
    /// Create a new monitor with the given window size.
    pub fn new(max_window: usize) -> Self {
        Self {
            window: std::collections::VecDeque::new(),
            max_window: max_window.max(1),
        }
    }
    /// Record an access.
    pub fn record(&mut self, ts: u64, is_hit: bool) {
        self.window.push_back((ts, is_hit));
        while self.window.len() > self.max_window {
            self.window.pop_front();
        }
    }
    /// Current hit rate over the window.
    pub fn hit_rate(&self) -> f64 {
        if self.window.is_empty() {
            return 0.0;
        }
        let hits = self.window.iter().filter(|(_, h)| *h).count();
        hits as f64 / self.window.len() as f64
    }
    /// Number of accesses in the window.
    pub fn window_size(&self) -> usize {
        self.window.len()
    }
    /// Clear the window.
    pub fn clear(&mut self) {
        self.window.clear();
    }
}
/// A point-in-time sample of cache usage for trend analysis.
#[derive(Clone, Debug)]
pub struct CacheUsageSample {
    /// Sample timestamp (seconds since epoch).
    pub timestamp: u64,
    /// Number of entries in the cache.
    pub entry_count: usize,
    /// Total bytes used.
    pub total_bytes: u64,
    /// Hit rate at sample time.
    pub hit_rate: f64,
}
impl CacheUsageSample {
    /// Create a sample.
    pub fn new(timestamp: u64, entry_count: usize, total_bytes: u64, hit_rate: f64) -> Self {
        Self {
            timestamp,
            entry_count,
            total_bytes,
            hit_rate,
        }
    }
}
/// Detects cache pressure conditions.
#[derive(Debug, Default)]
pub struct CachePressureDetector {
    /// Threshold utilization [0.0, 1.0] above which pressure is declared.
    pub utilization_threshold: f64,
    /// Threshold eviction rate (evictions/insert) above which pressure is declared.
    pub eviction_rate_threshold: f64,
    /// Running count of inserts.
    pub insert_count: u64,
    /// Running count of evictions.
    pub eviction_count: u64,
}
impl CachePressureDetector {
    /// Create a new detector with the given thresholds.
    pub fn new(utilization_threshold: f64, eviction_rate_threshold: f64) -> Self {
        Self {
            utilization_threshold,
            eviction_rate_threshold,
            insert_count: 0,
            eviction_count: 0,
        }
    }
    /// Record an insert (and how many entries were evicted as a result).
    pub fn record_insert(&mut self, evictions: u64) {
        self.insert_count += 1;
        self.eviction_count += evictions;
    }
    /// Current eviction rate (evictions / inserts).
    pub fn eviction_rate(&self) -> f64 {
        if self.insert_count == 0 {
            0.0
        } else {
            self.eviction_count as f64 / self.insert_count as f64
        }
    }
    /// Whether the cache is under pressure given its current utilization.
    pub fn is_under_pressure(&self, utilization: f64) -> bool {
        utilization >= self.utilization_threshold
            || self.eviction_rate() >= self.eviction_rate_threshold
    }
    /// Reset counters.
    pub fn reset(&mut self) {
        self.insert_count = 0;
        self.eviction_count = 0;
    }
}
/// TTL-based eviction: evicts entries whose time-to-live has expired.
/// Entries that have lived longer than `max_age` are evicted first; among
/// those still within TTL, entries closest to expiration are evicted first.
#[derive(Clone, Debug)]
pub struct TtlPolicy {
    /// Maximum age for any cache entry.
    pub max_age: Duration,
}
impl TtlPolicy {
    pub fn new(max_age: Duration) -> Self {
        Self { max_age }
    }
    /// Check whether a specific entry has expired.
    pub fn is_expired(&self, entry: &CacheEntry) -> bool {
        entry.age() >= self.max_age
    }
}
/// Rolling history of cache usage samples.
pub struct CacheUsageHistory {
    samples: std::collections::VecDeque<CacheUsageSample>,
    max_samples: usize,
}
impl CacheUsageHistory {
    /// Create with the given maximum sample count.
    pub fn with_capacity(max_samples: usize) -> Self {
        Self {
            samples: std::collections::VecDeque::new(),
            max_samples,
        }
    }
    /// Record a sample.
    pub fn record(&mut self, sample: CacheUsageSample) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }
    /// Number of samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
    /// Average hit rate across all samples.
    pub fn avg_hit_rate(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.samples.iter().map(|s| s.hit_rate).sum();
        sum / self.samples.len() as f64
    }
    /// Maximum entry count seen.
    pub fn peak_entry_count(&self) -> usize {
        self.samples
            .iter()
            .map(|s| s.entry_count)
            .max()
            .unwrap_or(0)
    }
    /// Maximum bytes seen.
    pub fn peak_bytes(&self) -> u64 {
        self.samples
            .iter()
            .map(|s| s.total_bytes)
            .max()
            .unwrap_or(0)
    }
}
/// Records entries that were evicted (for audit/logging purposes).
#[derive(Clone, Debug, Default)]
pub struct EvictionAuditLog {
    evicted_keys: Vec<String>,
}
impl EvictionAuditLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an eviction.
    pub fn record(&mut self, key: &str) {
        self.evicted_keys.push(key.to_string());
    }
    /// Number of evicted entries.
    pub fn len(&self) -> usize {
        self.evicted_keys.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.evicted_keys.is_empty()
    }
    /// All evicted keys.
    pub fn keys(&self) -> &[String] {
        &self.evicted_keys
    }
}
/// Size-based eviction: evicts the largest entries first to free space quickly.
#[derive(Clone, Debug, Default)]
pub struct SizeBasedPolicy;
impl SizeBasedPolicy {
    pub fn new() -> Self {
        Self
    }
}
/// Per-namespace byte quota with eviction thresholds.
#[derive(Clone, Debug)]
pub struct EvictionQuota {
    /// Namespace identifier.
    pub namespace: String,
    /// Maximum bytes allowed.
    pub max_bytes: u64,
    /// Currently used bytes.
    pub used_bytes: u64,
    /// Whether quota is enforced.
    pub enforced: bool,
}
impl EvictionQuota {
    /// Create a quota.
    pub fn new(namespace: &str, max_bytes: u64) -> Self {
        Self {
            namespace: namespace.to_string(),
            max_bytes,
            used_bytes: 0,
            enforced: true,
        }
    }
    /// Add usage.
    pub fn add(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_add(bytes);
    }
    /// Release usage.
    pub fn release(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }
    /// Whether the quota would be exceeded by `additional_bytes`.
    pub fn would_exceed(&self, additional_bytes: u64) -> bool {
        self.enforced && self.used_bytes + additional_bytes > self.max_bytes
    }
    /// Remaining bytes.
    pub fn remaining(&self) -> u64 {
        self.max_bytes.saturating_sub(self.used_bytes)
    }
    /// Pressure level.
    pub fn pressure(&self) -> CachePressureLevel {
        if self.max_bytes == 0 {
            return CachePressureLevel::Critical;
        }
        CachePressureLevel::from_utilization(self.used_bytes as f64 / self.max_bytes as f64)
    }
}
