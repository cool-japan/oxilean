//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, VecDeque};

/// Combined read+write policy for a cache instance.
#[derive(Clone, Copy, Debug)]
pub struct CachePolicy {
    pub read: CacheReadPolicy,
    pub write: CacheWritePolicy,
}
impl CachePolicy {
    /// Standard read+write policy.
    pub fn read_write() -> Self {
        Self {
            read: CacheReadPolicy::Enabled,
            write: CacheWritePolicy::Synchronous,
        }
    }
    /// Read-only policy (useful for CI inspection).
    pub fn read_only() -> Self {
        Self {
            read: CacheReadPolicy::Enabled,
            write: CacheWritePolicy::ReadOnly,
        }
    }
    /// Fully disabled.
    pub fn disabled() -> Self {
        Self {
            read: CacheReadPolicy::Bypass,
            write: CacheWritePolicy::ReadOnly,
        }
    }
    /// Whether reads are active.
    pub fn reads_enabled(&self) -> bool {
        self.read != CacheReadPolicy::Bypass
    }
    /// Whether writes are active.
    pub fn writes_enabled(&self) -> bool {
        self.write != CacheWritePolicy::ReadOnly
    }
}
/// Exponential-back-off retry configuration for remote cache operations.
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of attempts (including the first try).
    pub max_attempts: u32,
    /// Initial delay in milliseconds.
    pub initial_delay_ms: u64,
    /// Multiplier applied after each failure.
    pub backoff_factor: f64,
    /// Maximum delay cap in milliseconds.
    pub max_delay_ms: u64,
}
impl RetryConfig {
    /// Sensible defaults: 3 attempts, 200 ms initial, 2× backoff, 5 s cap.
    pub fn default_remote() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 200,
            backoff_factor: 2.0,
            max_delay_ms: 5_000,
        }
    }
    /// Compute the delay (ms) for attempt number `n` (0-indexed).
    pub fn delay_for_attempt(&self, n: u32) -> u64 {
        let delay = self.initial_delay_ms as f64 * self.backoff_factor.powi(n as i32);
        (delay as u64).min(self.max_delay_ms)
    }
    /// Whether we should retry given `attempt` (0-indexed, after first failure).
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt + 1 < self.max_attempts
    }
}
/// Local filesystem mirror of a remote cache, with an in-memory index.
pub struct LocalMirrorCache {
    pub cache_dir: std::path::PathBuf,
    pub entries: HashMap<String, CacheEntry>,
}
impl LocalMirrorCache {
    pub fn new(dir: &str) -> Self {
        Self {
            cache_dir: std::path::PathBuf::from(dir),
            entries: HashMap::new(),
        }
    }
    /// Look up an entry by key.
    pub fn lookup(&self, key: &CacheKey) -> Option<&CacheEntry> {
        self.entries.get(&key.to_path_component())
    }
    /// Register a new entry in the in-memory index.
    pub fn register(&mut self, key: CacheKey, size: u64) {
        let path_key = key.to_path_component();
        self.entries.insert(path_key, CacheEntry::new(key, size));
    }
    /// Remove all entries older than `max_age_secs`. Returns count evicted.
    pub fn evict_stale(&mut self, max_age_secs: u64, now_secs: u64) -> usize {
        let before = self.entries.len();
        self.entries
            .retain(|_, entry| !entry.is_stale(max_age_secs, now_secs));
        before - self.entries.len()
    }
    /// Sum of all registered artifact sizes in bytes.
    pub fn total_size_bytes(&self) -> u64 {
        self.entries.values().map(|e| e.artifact_size).sum()
    }
    /// Number of entries in the index.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
/// Controls cache read behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheReadPolicy {
    /// Use cache (default).
    Enabled,
    /// Bypass cache for all reads.
    Bypass,
    /// Use cache but verify content hash on read.
    Verified,
}
/// A unique identifier for a single build-cache session.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CacheSessionId(pub(super) String);
impl CacheSessionId {
    /// Create a deterministic session ID from a seed string.
    pub fn from_seed(seed: &str) -> Self {
        let mut hasher = CacheKeyHasher::new();
        hasher.mix_str(seed);
        Self(hasher.finish_hex())
    }
    /// Return the raw ID string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// A lightweight metadata record in the cache index.
#[derive(Clone, Debug)]
pub struct CacheIndexEntry {
    /// The cached artifact's content hash (from `ContentAddressedStore`).
    pub content_hash: u64,
    /// Artifact size in bytes.
    pub size_bytes: u64,
    /// Unix timestamp (seconds) of the last access.
    pub last_access: u64,
    /// Number of times this entry has been served.
    pub hit_count: u32,
    /// Which backend holds the artifact.
    pub backend: CacheBackendKind,
}
/// Utility for building a `CacheKey` by hashing multiple inputs.
#[derive(Default)]
pub struct CacheKeyHasher {
    state: u64,
}
impl CacheKeyHasher {
    /// Create a new hasher.
    pub fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
    /// Mix a byte slice into the hash state.
    pub fn mix_bytes(&mut self, data: &[u8]) -> &mut Self {
        for &b in data {
            self.state ^= b as u64;
            self.state = self.state.wrapping_mul(0x100000001b3);
        }
        self
    }
    /// Mix a string into the hash state.
    pub fn mix_str(&mut self, s: &str) -> &mut Self {
        self.mix_bytes(s.as_bytes())
    }
    /// Mix a u64 into the hash state.
    pub fn mix_u64(&mut self, v: u64) -> &mut Self {
        self.mix_bytes(&v.to_le_bytes())
    }
    /// Finalize and return a hex-encoded hash string.
    pub fn finish_hex(&self) -> String {
        format!("{:016x}", self.state)
    }
    /// Finalize and return a `CacheKey` for the given module.
    pub fn into_cache_key(self, module: &str) -> CacheKey {
        let hash = format!("{:016x}", self.state);
        CacheKey::new(module, &hash)
    }
}
/// A prioritized list of cache keys to pre-fetch in the background.
pub struct CachePrefetchList {
    /// (priority, key) — higher priority is fetched first.
    entries: std::collections::BinaryHeap<(u32, String)>,
}
impl CachePrefetchList {
    /// Create an empty prefetch list.
    pub fn new() -> Self {
        Self {
            entries: std::collections::BinaryHeap::new(),
        }
    }
    /// Enqueue a key with a given priority.
    pub fn enqueue(&mut self, key: &CacheKey, priority: u32) {
        self.entries.push((priority, key.to_path_component()));
    }
    /// Dequeue the highest-priority key.
    pub fn dequeue(&mut self) -> Option<(u32, String)> {
        self.entries.pop()
    }
    /// Number of queued keys.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A candidate entry for cache eviction, with a computed score.
#[derive(Clone, Debug)]
pub struct EvictionCandidate {
    /// Path component (key) of the entry.
    pub path_component: String,
    /// Eviction score: higher means evict sooner.
    pub score: f64,
    /// Artifact size in bytes.
    pub size_bytes: u64,
}
impl EvictionCandidate {
    /// Create a candidate.
    pub fn new(path_component: &str, score: f64, size_bytes: u64) -> Self {
        Self {
            path_component: path_component.to_string(),
            score,
            size_bytes,
        }
    }
}
/// A time-bounded lease on a cache entry (prevents eviction while held).
#[derive(Clone, Debug)]
pub struct CacheLease {
    /// Path component identifying the cache entry.
    pub path_component: String,
    /// Lease expiry timestamp (seconds).
    pub expires_at: u64,
}
impl CacheLease {
    /// Create a lease that expires `ttl_secs` from `now_secs`.
    pub fn new(path_component: &str, now_secs: u64, ttl_secs: u64) -> Self {
        Self {
            path_component: path_component.to_string(),
            expires_at: now_secs + ttl_secs,
        }
    }
    /// Whether the lease is still valid at `now_secs`.
    pub fn is_valid(&self, now_secs: u64) -> bool {
        now_secs < self.expires_at
    }
    /// Remaining lease duration in seconds.
    pub fn remaining_secs(&self, now_secs: u64) -> u64 {
        self.expires_at.saturating_sub(now_secs)
    }
}
/// A cache that tries multiple backends in priority order (read-through).
pub struct MultiBackendCache {
    /// Ordered list of (backend_kind, is_writable).
    pub backends: Vec<(CacheBackendKind, bool)>,
    /// In-memory store for the local backend.
    pub local_store: ContentAddressedStore,
    /// Unified index of all known entries.
    pub index: CacheIndex,
    /// Retry configuration.
    pub retry: RetryConfig,
    /// Bandwidth statistics.
    pub bandwidth: BandwidthStats,
    /// Total cache hits.
    pub hits: u64,
    /// Total cache misses.
    pub misses: u64,
}
impl MultiBackendCache {
    /// Create a multi-backend cache with a local store.
    pub fn new(local_capacity: usize) -> Self {
        Self {
            backends: vec![(CacheBackendKind::Local, true)],
            local_store: ContentAddressedStore::with_capacity(local_capacity),
            index: CacheIndex::new(),
            retry: RetryConfig::default_remote(),
            bandwidth: BandwidthStats::new(),
            hits: 0,
            misses: 0,
        }
    }
    /// Add a remote backend.
    pub fn add_backend(&mut self, kind: CacheBackendKind, writable: bool) {
        self.backends.push((kind, writable));
    }
    /// Stub fetch: tries local store, otherwise records a miss.
    pub fn fetch(&mut self, key: &CacheKey) -> Option<Vec<u8>> {
        let path_comp = key.to_path_component();
        if let Some(entry) = self.index.get(&path_comp) {
            let content_hash = entry.content_hash;
            if let Some(data) = self.local_store.get(content_hash).cloned() {
                self.hits += 1;
                self.index.record_hit(&path_comp, 0);
                return Some(data);
            }
        }
        self.misses += 1;
        None
    }
    /// Store an artifact in the local backend and update the index.
    pub fn store(&mut self, key: &CacheKey, data: Vec<u8>, now_secs: u64) {
        let path_comp = key.to_path_component();
        let size = data.len() as u64;
        let content_hash = self.local_store.insert(data);
        self.index.upsert(
            &path_comp,
            content_hash,
            size,
            now_secs,
            CacheBackendKind::Local,
        );
    }
    /// Hit rate across all operations.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Total entries in the local store.
    pub fn local_entry_count(&self) -> usize {
        self.local_store.len()
    }
    /// Whether any remote backend is configured.
    pub fn has_remote_backend(&self) -> bool {
        self.backends
            .iter()
            .any(|(k, _)| *k != CacheBackendKind::Local)
    }
}
/// Manages multiple cache segments.
pub struct CacheSegmentManager {
    segments: HashMap<String, CacheSegment>,
}
impl CacheSegmentManager {
    /// Create a segment manager.
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
        }
    }
    /// Ensure a segment exists.
    pub fn ensure_segment(&mut self, id: &str) -> &mut CacheSegment {
        self.segments
            .entry(id.to_string())
            .or_insert_with(|| CacheSegment::new(id))
    }
    /// Record an artifact addition to `segment_id`.
    pub fn record_add(&mut self, segment_id: &str, bytes: u64) {
        self.ensure_segment(segment_id).record_add(bytes);
    }
    /// Record an artifact removal from `segment_id`.
    pub fn record_remove(&mut self, segment_id: &str, bytes: u64) {
        if let Some(seg) = self.segments.get_mut(segment_id) {
            seg.record_remove(bytes);
        }
    }
    /// Total bytes across all segments.
    pub fn total_bytes(&self) -> u64 {
        self.segments.values().map(|s| s.total_bytes).sum()
    }
    /// Total entries across all segments.
    pub fn total_entries(&self) -> u64 {
        self.segments.values().map(|s| s.entry_count).sum()
    }
    /// Number of segments.
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
    /// Deactivate a segment (mark as inactive).
    pub fn deactivate(&mut self, id: &str) {
        if let Some(seg) = self.segments.get_mut(id) {
            seg.active = false;
        }
    }
    /// Get active segment IDs.
    pub fn active_segment_ids(&self) -> Vec<&str> {
        self.segments
            .values()
            .filter(|s| s.active)
            .map(|s| s.id.as_str())
            .collect()
    }
}
/// Configuration for an S3-compatible remote cache backend.
#[derive(Clone, Debug)]
pub struct S3CacheConfig {
    /// S3 endpoint URL.
    pub endpoint: String,
    /// Bucket name.
    pub bucket: String,
    /// Key prefix inside the bucket.
    pub key_prefix: String,
    /// AWS access key ID.
    pub access_key_id: Option<String>,
    /// AWS secret access key.
    pub secret_access_key: Option<String>,
    /// AWS region.
    pub region: String,
    /// Whether to use path-style addressing.
    pub path_style: bool,
}
impl S3CacheConfig {
    /// Create a minimal S3 config.
    pub fn new(endpoint: &str, bucket: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            bucket: bucket.to_string(),
            key_prefix: "oxilean-cache".to_string(),
            access_key_id: None,
            secret_access_key: None,
            region: "us-east-1".to_string(),
            path_style: false,
        }
    }
    /// Builder: set credentials.
    pub fn with_credentials(mut self, access_key: &str, secret: &str) -> Self {
        self.access_key_id = Some(access_key.to_string());
        self.secret_access_key = Some(secret.to_string());
        self
    }
    /// Builder: set region.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = region.to_string();
        self
    }
    /// Builder: use path-style addressing (required for MinIO).
    pub fn with_path_style(mut self) -> Self {
        self.path_style = true;
        self
    }
    /// Compute the S3 object key for a given cache key path component.
    pub fn object_key(&self, path_component: &str) -> String {
        format!("{}/{}", self.key_prefix, path_component)
    }
}
/// Configuration for a GCS (Google Cloud Storage) remote cache backend.
#[derive(Clone, Debug)]
pub struct GcsCacheConfig {
    /// GCS bucket name.
    pub bucket: String,
    /// Object prefix inside the bucket.
    pub prefix: String,
    /// Service account JSON key path (if any).
    pub service_account_path: Option<String>,
    /// Project ID.
    pub project_id: Option<String>,
}
impl GcsCacheConfig {
    /// Create a minimal GCS config.
    pub fn new(bucket: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
            prefix: "oxilean".to_string(),
            service_account_path: None,
            project_id: None,
        }
    }
    /// Builder: set service account key path.
    pub fn with_service_account(mut self, path: &str) -> Self {
        self.service_account_path = Some(path.to_string());
        self
    }
    /// Builder: set project ID.
    pub fn with_project(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_string());
        self
    }
    /// Compute the GCS object name for a given cache key path component.
    pub fn object_name(&self, path_component: &str) -> String {
        format!("{}/{}", self.prefix, path_component)
    }
}
/// Configuration for an HTTP remote cache (Bazel-style remote cache API).
#[derive(Clone, Debug)]
pub struct HttpCacheConfig {
    /// Base URL of the cache server.
    pub base_url: String,
    /// Optional Bearer token for authentication.
    pub bearer_token: Option<String>,
    /// Whether TLS certificate verification is disabled.
    pub insecure: bool,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Maximum upload chunk size in bytes.
    pub chunk_size: usize,
}
impl HttpCacheConfig {
    /// Create a new HTTP cache config.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            bearer_token: None,
            insecure: false,
            timeout_secs: 60,
            chunk_size: 1_048_576,
        }
    }
    /// Builder: set bearer token.
    pub fn with_token(mut self, token: &str) -> Self {
        self.bearer_token = Some(token.to_string());
        self
    }
    /// Builder: disable TLS verification (use with care).
    pub fn insecure(mut self) -> Self {
        self.insecure = true;
        self
    }
    /// Compute the GET/PUT URL for a given path component.
    pub fn artifact_url(&self, path_component: &str) -> String {
        format!(
            "{}/ac/{}",
            self.base_url.trim_end_matches('/'),
            path_component
        )
    }
}
/// Manages active cache leases to prevent eviction of in-use entries.
pub struct CacheLeaseManager {
    leases: HashMap<String, CacheLease>,
}
impl CacheLeaseManager {
    /// Create an empty lease manager.
    pub fn new() -> Self {
        Self {
            leases: HashMap::new(),
        }
    }
    /// Acquire a lease for `path_component` with the given TTL.
    pub fn acquire(&mut self, path_component: &str, now_secs: u64, ttl_secs: u64) {
        let lease = CacheLease::new(path_component, now_secs, ttl_secs);
        self.leases.insert(path_component.to_string(), lease);
    }
    /// Release a lease explicitly.
    pub fn release(&mut self, path_component: &str) {
        self.leases.remove(path_component);
    }
    /// Expire all leases that are no longer valid.
    pub fn expire_old(&mut self, now_secs: u64) -> usize {
        let before = self.leases.len();
        self.leases.retain(|_, l| l.is_valid(now_secs));
        before - self.leases.len()
    }
    /// Whether a given path component has a valid lease.
    pub fn is_leased(&self, path_component: &str, now_secs: u64) -> bool {
        self.leases
            .get(path_component)
            .map(|l| l.is_valid(now_secs))
            .unwrap_or(false)
    }
    /// Number of active leases.
    pub fn active_count(&self) -> usize {
        self.leases.len()
    }
}
/// Tracks cache usage against a capacity budget.
#[derive(Clone, Debug)]
pub struct CacheCapacityBudget {
    /// Maximum allowed bytes.
    pub max_bytes: u64,
    /// Currently used bytes.
    pub used_bytes: u64,
    /// Maximum number of entries.
    pub max_entries: usize,
    /// Currently used entries.
    pub used_entries: usize,
}
impl CacheCapacityBudget {
    /// Create a budget with given limits.
    pub fn new(max_bytes: u64, max_entries: usize) -> Self {
        Self {
            max_bytes,
            used_bytes: 0,
            max_entries,
            used_entries: 0,
        }
    }
    /// Whether the budget allows adding `bytes` more data.
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
    /// Bytes remaining.
    pub fn remaining_bytes(&self) -> u64 {
        self.max_bytes.saturating_sub(self.used_bytes)
    }
    /// Usage as a percentage (0–100).
    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
        }
    }
}
/// Monitors the health of a remote cache backend with periodic ping checks.
#[derive(Debug)]
pub struct CacheHealthChecker {
    /// URL of the cache backend.
    pub endpoint: String,
    /// Whether the last health check passed.
    pub is_healthy: bool,
    /// Total successful health checks.
    pub success_count: u64,
    /// Total failed health checks.
    pub failure_count: u64,
    /// Timestamp of the last check.
    pub last_check_ts: u64,
    /// Interval between checks in seconds.
    pub check_interval_secs: u64,
}
impl CacheHealthChecker {
    /// Create a health checker for a given endpoint.
    pub fn new(endpoint: &str, check_interval_secs: u64) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            is_healthy: true,
            success_count: 0,
            failure_count: 0,
            last_check_ts: 0,
            check_interval_secs,
        }
    }
    /// Simulate a health check (stub: always succeeds).
    pub fn check(&mut self, now_ts: u64) -> bool {
        self.last_check_ts = now_ts;
        self.is_healthy = true;
        self.success_count += 1;
        true
    }
    /// Simulate a failed health check.
    pub fn record_failure(&mut self, now_ts: u64) {
        self.last_check_ts = now_ts;
        self.is_healthy = false;
        self.failure_count += 1;
    }
    /// Whether a check is due given `now_ts`.
    pub fn check_due(&self, now_ts: u64) -> bool {
        now_ts.saturating_sub(self.last_check_ts) >= self.check_interval_secs
    }
    /// Reliability percentage (successes / total * 100).
    pub fn reliability_pct(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            100.0
        } else {
            (self.success_count as f64 / total as f64) * 100.0
        }
    }
}
/// Stub client for a remote artifact cache.
pub struct RemoteCacheClient {
    pub config: RemoteCacheConfig,
    pub hit_count: u64,
    pub miss_count: u64,
    pub upload_count: u64,
}
impl RemoteCacheClient {
    pub fn new(config: RemoteCacheConfig) -> Self {
        Self {
            config,
            hit_count: 0,
            miss_count: 0,
            upload_count: 0,
        }
    }
    /// Attempt to fetch an artifact by key.  Returns `None` (no network in tests).
    pub fn try_fetch(&mut self, _key: &CacheKey) -> Option<Vec<u8>> {
        self.miss_count += 1;
        None
    }
    /// Attempt to upload an artifact.  Stub always returns `true`.
    pub fn try_upload(&mut self, _key: &CacheKey, _artifact: &[u8]) -> bool {
        self.upload_count += 1;
        true
    }
    /// Returns the fraction of lookups that were cache hits.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    /// Human-readable statistics report.
    pub fn stats_report(&self) -> String {
        format!(
            "RemoteCache[{}]: hits={} misses={} uploads={} hit_rate={:.1}%",
            self.config.url,
            self.hit_count,
            self.miss_count,
            self.upload_count,
            self.hit_rate() * 100.0,
        )
    }
}
/// Identifies a build artifact in a remote/local cache.
pub struct CacheKey {
    pub hash: String,
    pub module_name: String,
    pub compiler_version: String,
}
impl CacheKey {
    pub fn new(module: &str, hash: &str) -> Self {
        Self {
            hash: hash.to_string(),
            module_name: module.to_string(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
    /// Returns a URL-safe path component for storage lookups.
    pub fn to_path_component(&self) -> String {
        format!(
            "{}-{}-{}",
            self.module_name.replace(['/', '\\', ' '], "_"),
            self.compiler_version.replace(['.', '+'], "_"),
            self.hash,
        )
    }
}
/// A pool of `RemoteCacheClient` instances, one per backend URL.
pub struct RemoteCachePool {
    /// URL → client.
    clients: HashMap<String, RemoteCacheClient>,
}
impl RemoteCachePool {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    /// Add a client to the pool.
    pub fn add(&mut self, url: &str) {
        let cfg = RemoteCacheConfig::new(url);
        self.clients
            .insert(url.to_string(), RemoteCacheClient::new(cfg));
    }
    /// Attempt to fetch from all clients, returning the first result.
    pub fn try_fetch_any(&mut self, key: &CacheKey) -> Option<(String, Vec<u8>)> {
        let urls: Vec<String> = self.clients.keys().cloned().collect();
        for url in urls {
            if let Some(client) = self.clients.get_mut(&url) {
                if let Some(data) = client.try_fetch(key) {
                    return Some((url, data));
                }
            }
        }
        None
    }
    /// Upload to all writable clients.
    pub fn upload_all(&mut self, key: &CacheKey, data: &[u8]) -> usize {
        let urls: Vec<String> = self.clients.keys().cloned().collect();
        let mut success_count = 0;
        for url in urls {
            if let Some(client) = self.clients.get_mut(&url) {
                if client.try_upload(key, data) {
                    success_count += 1;
                }
            }
        }
        success_count
    }
    /// Number of clients in the pool.
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
    /// Aggregate hit count across all clients.
    pub fn total_hits(&self) -> u64 {
        self.clients.values().map(|c| c.hit_count).sum()
    }
    /// Aggregate miss count across all clients.
    pub fn total_misses(&self) -> u64 {
        self.clients.values().map(|c| c.miss_count).sum()
    }
}
/// A manifest describing all artifacts produced by a single build.
#[derive(Clone, Debug, Default)]
pub struct ArtifactManifest {
    /// List of (module_name, cache_key_hash, size_bytes).
    pub entries: Vec<(String, String, u64)>,
}
impl ArtifactManifest {
    /// Create an empty manifest.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an artifact entry.
    pub fn add(&mut self, module: &str, key: &CacheKey, size: u64) {
        self.entries
            .push((module.to_string(), key.to_path_component(), size));
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the manifest is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Total size of all artifacts.
    pub fn total_size(&self) -> u64 {
        self.entries.iter().map(|(_, _, sz)| sz).sum()
    }
    /// Serialize to a simple line-based format.
    pub fn to_text(&self) -> String {
        self.entries
            .iter()
            .map(|(m, k, sz)| format!("{} {} {}", m, k, sz))
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Parse from the line-based format produced by `to_text`.
    pub fn from_text(text: &str) -> Self {
        let entries = text
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(3, ' ').collect();
                if parts.len() == 3 {
                    let sz = parts[2].parse::<u64>().unwrap_or(0);
                    Some((parts[0].to_string(), parts[1].to_string(), sz))
                } else {
                    None
                }
            })
            .collect();
        Self { entries }
    }
}
/// Wraps an artifact's bytes with optional compression metadata.
#[derive(Clone, Debug)]
pub struct CompressedArtifact {
    /// Compressed bytes (or plain bytes if compression disabled).
    pub data: Vec<u8>,
    /// Uncompressed size in bytes.
    pub uncompressed_size: u64,
    /// Whether the data is actually compressed.
    pub is_compressed: bool,
    /// Compression algorithm tag.
    pub algorithm: &'static str,
}
impl CompressedArtifact {
    /// Create an uncompressed artifact wrapper.
    pub fn uncompressed(data: Vec<u8>) -> Self {
        let size = data.len() as u64;
        Self {
            data,
            uncompressed_size: size,
            is_compressed: false,
            algorithm: "none",
        }
    }
    /// Simulate zstd compression by noting original size (no actual compression).
    pub fn compress_zstd(data: Vec<u8>) -> Self {
        let uncompressed_size = data.len() as u64;
        Self {
            data,
            uncompressed_size,
            is_compressed: true,
            algorithm: "zstd",
        }
    }
    /// Compression ratio (compressed / uncompressed). 1.0 if no compression.
    pub fn compression_ratio(&self) -> f64 {
        if self.uncompressed_size == 0 {
            1.0
        } else {
            self.data.len() as f64 / self.uncompressed_size as f64
        }
    }
    /// Space saving percentage.
    pub fn space_saving_pct(&self) -> f64 {
        (1.0 - self.compression_ratio()) * 100.0
    }
}
/// Eviction strategy used by the local mirror cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least-recently-used.
    Lru,
    /// Least-frequently-used.
    Lfu,
    /// Oldest-first (by timestamp).
    Fifo,
    /// Largest-first (free space greedily).
    LargestFirst,
}
/// Controls when and how artifacts are written to a remote cache.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheWritePolicy {
    /// Write artifacts to cache immediately after build.
    Synchronous,
    /// Queue writes and flush in background.
    Async,
    /// Only write on explicit flush call.
    OnFlush,
    /// Disable all writes (read-only mode).
    ReadOnly,
}
/// Configuration for proactive cache warming.
#[derive(Clone, Debug)]
pub struct CacheWarmingConfig {
    /// Whether warming is enabled.
    pub enabled: bool,
    /// Maximum concurrent warming fetches.
    pub concurrency: usize,
    /// Modules to pre-fetch (by name).
    pub modules: Vec<String>,
    /// Timeout per warming fetch in milliseconds.
    pub fetch_timeout_ms: u64,
}
impl CacheWarmingConfig {
    /// Create a disabled warming config.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            concurrency: 0,
            modules: Vec::new(),
            fetch_timeout_ms: 10_000,
        }
    }
    /// Create a basic warming config.
    pub fn new(concurrency: usize) -> Self {
        Self {
            enabled: true,
            concurrency,
            modules: Vec::new(),
            fetch_timeout_ms: 10_000,
        }
    }
    /// Add a module to warm.
    pub fn with_module(mut self, module: &str) -> Self {
        self.modules.push(module.to_string());
        self
    }
    /// Number of modules queued for warming.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }
}
/// Configuration for connecting to a remote cache server.
pub struct RemoteCacheConfig {
    pub url: String,
    pub timeout_secs: u64,
    pub max_entry_size_mb: u64,
    pub api_key: Option<String>,
}
impl RemoteCacheConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            timeout_secs: 30,
            max_entry_size_mb: 512,
            api_key: None,
        }
    }
    /// Builder-pattern setter for the API key.
    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }
}
/// A point-in-time snapshot of cache statistics for reporting.
#[derive(Clone, Debug, Default)]
pub struct CacheStatsSnapshot {
    /// Total entries at snapshot time.
    pub entry_count: usize,
    /// Total bytes at snapshot time.
    pub total_bytes: u64,
    /// Hit rate at snapshot time.
    pub hit_rate: f64,
    /// Snapshot timestamp (seconds).
    pub timestamp: u64,
}
impl CacheStatsSnapshot {
    /// Create a snapshot.
    pub fn new(entry_count: usize, total_bytes: u64, hit_rate: f64, timestamp: u64) -> Self {
        Self {
            entry_count,
            total_bytes,
            hit_rate,
            timestamp,
        }
    }
}
/// An in-memory index of all known cache entries, keyed by path component.
pub struct CacheIndex {
    /// path_component → entry metadata.
    pub entries: HashMap<String, CacheIndexEntry>,
    /// Maximum number of index entries (0 = unlimited).
    pub max_entries: usize,
}
impl CacheIndex {
    /// Create a new unlimited index.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            max_entries: 0,
        }
    }
    /// Create an index with a maximum entry count.
    pub fn with_limit(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
        }
    }
    /// Insert or update an index entry.
    pub fn upsert(
        &mut self,
        path_component: &str,
        content_hash: u64,
        size_bytes: u64,
        now_secs: u64,
        backend: CacheBackendKind,
    ) {
        if self.max_entries > 0 && self.entries.len() >= self.max_entries {
            self.evict_oldest();
        }
        self.entries.insert(
            path_component.to_string(),
            CacheIndexEntry {
                content_hash,
                size_bytes,
                last_access: now_secs,
                hit_count: 0,
                backend,
            },
        );
    }
    /// Record a cache hit for the given path component.
    pub fn record_hit(&mut self, path_component: &str, now_secs: u64) {
        if let Some(e) = self.entries.get_mut(path_component) {
            e.hit_count = e.hit_count.saturating_add(1);
            e.last_access = now_secs;
        }
    }
    /// Remove all entries whose `last_access` is older than `max_age_secs`.
    pub fn evict_stale(&mut self, max_age_secs: u64, now_secs: u64) -> usize {
        let before = self.entries.len();
        self.entries
            .retain(|_, e| now_secs.saturating_sub(e.last_access) <= max_age_secs);
        before - self.entries.len()
    }
    /// Number of indexed entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Total bytes tracked by the index.
    pub fn total_bytes(&self) -> u64 {
        self.entries.values().map(|e| e.size_bytes).sum()
    }
    /// Remove the entry with the oldest `last_access` timestamp.
    fn evict_oldest(&mut self) {
        if let Some(key) = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.last_access)
            .map(|(k, _)| k.clone())
        {
            self.entries.remove(&key);
        }
    }
    /// Look up an entry.
    pub fn get(&self, path_component: &str) -> Option<&CacheIndexEntry> {
        self.entries.get(path_component)
    }
}
/// Aggregated metrics for a cache session.
#[derive(Clone, Debug, Default)]
pub struct CacheMetrics {
    /// Total artifact fetches attempted.
    pub fetch_attempts: u64,
    /// Successful fetches (hits).
    pub fetch_hits: u64,
    /// Failed fetches (misses or errors).
    pub fetch_misses: u64,
    /// Total artifact uploads.
    pub upload_count: u64,
    /// Total bytes fetched.
    pub bytes_fetched: u64,
    /// Total bytes uploaded.
    pub bytes_uploaded: u64,
    /// Number of retries performed.
    pub retry_count: u64,
    /// Number of evictions triggered.
    pub evictions: u64,
}
impl CacheMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a fetch hit.
    pub fn record_hit(&mut self, bytes: u64) {
        self.fetch_attempts += 1;
        self.fetch_hits += 1;
        self.bytes_fetched += bytes;
    }
    /// Record a fetch miss.
    pub fn record_miss(&mut self) {
        self.fetch_attempts += 1;
        self.fetch_misses += 1;
    }
    /// Record an upload.
    pub fn record_upload(&mut self, bytes: u64) {
        self.upload_count += 1;
        self.bytes_uploaded += bytes;
    }
    /// Record a retry.
    pub fn record_retry(&mut self) {
        self.retry_count += 1;
    }
    /// Record an eviction.
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }
    /// Cache hit rate (0.0–1.0).
    pub fn hit_rate(&self) -> f64 {
        if self.fetch_attempts == 0 {
            0.0
        } else {
            self.fetch_hits as f64 / self.fetch_attempts as f64
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "hits={} misses={} uploads={} hit_rate={:.1}% retries={} evictions={}",
            self.fetch_hits,
            self.fetch_misses,
            self.upload_count,
            self.hit_rate() * 100.0,
            self.retry_count,
            self.evictions,
        )
    }
}
/// Metadata record for a cached artifact.
pub struct CacheEntry {
    pub key: CacheKey,
    pub artifact_size: u64,
    pub timestamp: u64,
    pub hit_count: u32,
}
impl CacheEntry {
    pub fn new(key: CacheKey, size: u64) -> Self {
        Self {
            key,
            artifact_size: size,
            timestamp: 0,
            hit_count: 0,
        }
    }
    /// Returns `true` when the entry is older than `max_age_secs` relative to `now_secs`.
    pub fn is_stale(&self, max_age_secs: u64, now_secs: u64) -> bool {
        now_secs.saturating_sub(self.timestamp) > max_age_secs
    }
}
/// A simple content-addressed in-memory store.
/// Keys are content hashes; values are raw bytes.
pub struct ContentAddressedStore {
    /// hash → artifact bytes
    inner: HashMap<u64, Vec<u8>>,
    /// Maximum number of entries before eviction.
    pub capacity: usize,
    /// Access order for LRU tracking (front = most recent).
    access_order: std::collections::VecDeque<u64>,
}
impl ContentAddressedStore {
    /// Create a store with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::new(),
            capacity,
            access_order: std::collections::VecDeque::new(),
        }
    }
    /// Simple 64-bit hash of a byte slice (FNV-1a).
    pub fn hash_bytes(data: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
    /// Insert artifact bytes; returns its content hash.
    pub fn insert(&mut self, data: Vec<u8>) -> u64 {
        let key = Self::hash_bytes(&data);
        if !self.inner.contains_key(&key) {
            if self.inner.len() >= self.capacity {
                self.evict_lru();
            }
            self.inner.insert(key, data);
            self.access_order.push_front(key);
        } else {
            self.touch(key);
        }
        key
    }
    /// Retrieve artifact bytes by content hash.
    pub fn get(&mut self, hash: u64) -> Option<&Vec<u8>> {
        if self.inner.contains_key(&hash) {
            self.touch(hash);
            self.inner.get(&hash)
        } else {
            None
        }
    }
    /// Number of stored entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Total bytes stored.
    pub fn total_bytes(&self) -> usize {
        self.inner.values().map(|v| v.len()).sum()
    }
    /// Remove the least-recently-used entry.
    fn evict_lru(&mut self) {
        if let Some(key) = self.access_order.pop_back() {
            self.inner.remove(&key);
        }
    }
    /// Move `key` to the front of the access order.
    fn touch(&mut self, key: u64) {
        self.access_order.retain(|&k| k != key);
        self.access_order.push_front(key);
    }
    /// Remove an entry by hash.
    pub fn remove(&mut self, hash: u64) -> bool {
        if self.inner.remove(&hash).is_some() {
            self.access_order.retain(|&k| k != hash);
            true
        } else {
            false
        }
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.inner.clear();
        self.access_order.clear();
    }
}
/// A logical segment of the cache (e.g. by compiler version or target).
#[derive(Clone, Debug)]
pub struct CacheSegment {
    /// Segment identifier.
    pub id: String,
    /// Number of entries in this segment.
    pub entry_count: u64,
    /// Total size in bytes.
    pub total_bytes: u64,
    /// Whether this segment is active.
    pub active: bool,
}
impl CacheSegment {
    /// Create a new active segment.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            entry_count: 0,
            total_bytes: 0,
            active: true,
        }
    }
    /// Record that an artifact was added to this segment.
    pub fn record_add(&mut self, bytes: u64) {
        self.entry_count += 1;
        self.total_bytes += bytes;
    }
    /// Record that an artifact was removed from this segment.
    pub fn record_remove(&mut self, bytes: u64) {
        if self.entry_count > 0 {
            self.entry_count -= 1;
        }
        self.total_bytes = self.total_bytes.saturating_sub(bytes);
    }
    /// Average artifact size in bytes.
    pub fn avg_artifact_size(&self) -> f64 {
        if self.entry_count == 0 {
            0.0
        } else {
            self.total_bytes as f64 / self.entry_count as f64
        }
    }
}
/// Groups cache keys by a namespace (e.g. project name + commit hash).
#[derive(Clone, Debug)]
pub struct CacheKeyNamespace {
    /// Namespace prefix, e.g. "oxilean/abc1234".
    pub prefix: String,
}
impl CacheKeyNamespace {
    /// Create a namespace from a project name and version string.
    pub fn new(project: &str, version: &str) -> Self {
        Self {
            prefix: format!("{}/{}", project.replace(['/', ' '], "_"), version),
        }
    }
    /// Derive a `CacheKey` for `module` within this namespace.
    pub fn key_for(&self, module: &str, content_hash: &str) -> CacheKey {
        let namespaced = format!("{}/{}", self.prefix, module);
        CacheKey::new(&namespaced, content_hash)
    }
    /// Whether a given path component belongs to this namespace.
    pub fn owns(&self, path_component: &str) -> bool {
        path_component.starts_with(&self.prefix.replace(['/', ' '], "_"))
    }
}
/// Identifies which storage backend is in use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheBackendKind {
    /// HTTP-based cache (e.g. Bazel remote cache protocol).
    Http,
    /// Amazon S3 or S3-compatible (e.g. MinIO).
    S3,
    /// Google Cloud Storage.
    Gcs,
    /// Local filesystem mirror.
    Local,
}
/// Running bandwidth statistics for upload/download operations.
#[derive(Clone, Debug, Default)]
pub struct BandwidthStats {
    /// Total bytes downloaded.
    pub bytes_downloaded: u64,
    /// Total bytes uploaded.
    pub bytes_uploaded: u64,
    /// Total download time in milliseconds.
    pub download_ms: u64,
    /// Total upload time in milliseconds.
    pub upload_ms: u64,
}
impl BandwidthStats {
    /// Create zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a completed download.
    pub fn record_download(&mut self, bytes: u64, elapsed_ms: u64) {
        self.bytes_downloaded += bytes;
        self.download_ms += elapsed_ms;
    }
    /// Record a completed upload.
    pub fn record_upload(&mut self, bytes: u64, elapsed_ms: u64) {
        self.bytes_uploaded += bytes;
        self.upload_ms += elapsed_ms;
    }
    /// Average download speed in bytes/second.
    pub fn avg_download_bps(&self) -> f64 {
        if self.download_ms == 0 {
            0.0
        } else {
            self.bytes_downloaded as f64 / (self.download_ms as f64 / 1_000.0)
        }
    }
    /// Average upload speed in bytes/second.
    pub fn avg_upload_bps(&self) -> f64 {
        if self.upload_ms == 0 {
            0.0
        } else {
            self.bytes_uploaded as f64 / (self.upload_ms as f64 / 1_000.0)
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "dl={} bytes ({:.1} KB/s) ul={} bytes ({:.1} KB/s)",
            self.bytes_downloaded,
            self.avg_download_bps() / 1024.0,
            self.bytes_uploaded,
            self.avg_upload_bps() / 1024.0,
        )
    }
}
/// Per-project or per-user cache quota enforcement.
#[derive(Clone, Debug)]
pub struct CacheQuota {
    /// Owner identifier (project or user).
    pub owner: String,
    /// Maximum bytes allowed.
    pub max_bytes: u64,
    /// Currently used bytes.
    pub used_bytes: u64,
    /// Whether quota enforcement is enabled.
    pub enforced: bool,
}
impl CacheQuota {
    /// Create a quota for an owner.
    pub fn new(owner: &str, max_bytes: u64) -> Self {
        Self {
            owner: owner.to_string(),
            max_bytes,
            used_bytes: 0,
            enforced: true,
        }
    }
    /// Check if `additional_bytes` would exceed the quota.
    pub fn would_exceed(&self, additional_bytes: u64) -> bool {
        self.enforced && self.used_bytes + additional_bytes > self.max_bytes
    }
    /// Record usage.
    pub fn add_usage(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_add(bytes);
    }
    /// Release usage.
    pub fn release_usage(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }
    /// Remaining capacity in bytes.
    pub fn remaining(&self) -> u64 {
        self.max_bytes.saturating_sub(self.used_bytes)
    }
    /// Usage as a percentage.
    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
        }
    }
}
