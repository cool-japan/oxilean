//! Type definitions for expr_cache

use super::super::functions::*;
use std::collections::{HashMap, VecDeque};

/// A cache entry with adaptive eviction support.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AdaptiveCacheEntry<V> {
    #[allow(missing_docs)]
    pub value: V,
    #[allow(missing_docs)]
    pub priority: CachePriority,
    #[allow(missing_docs)]
    pub access_count: u64,
    #[allow(missing_docs)]
    pub last_access: u64,
    #[allow(missing_docs)]
    pub insert_time: u64,
}

/// Interned symbol with kind.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SymbolInterner {
    pub(crate) symbols: std::collections::HashMap<String, u32>,
    pub(crate) by_id: Vec<String>,
}

/// A multi-level cache: L1 (small, fast), L2 (larger, slower).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MultiLevelCache<V: Clone> {
    pub(crate) l1: WindowCache<u64, V>,
    pub(crate) l2: std::collections::HashMap<u64, V>,
    pub(crate) l2_capacity: usize,
    pub(crate) l1_hits: u64,
    pub(crate) l2_hits: u64,
    pub(crate) misses: u64,
}

/// Versioned cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct VersionedCache<K: std::hash::Hash + Eq, V> {
    pub(crate) entries: std::collections::HashMap<K, (V, u64)>,
    pub(crate) version: u64,
}

/// Token frequency table.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default)]
pub struct TokenFrequencyTable {
    pub(crate) counts: std::collections::HashMap<String, u64>,
}

/// Adaptive LRU cache that self-tunes capacity based on hit rate.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AdaptiveLruCache<V> {
    pub(crate) inner: LruCache<V>,
    pub(crate) min_capacity: usize,
    pub(crate) max_capacity: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
    pub(crate) tune_interval: u64,
    pub(crate) ops: u64,
}

/// Policy cache with pluggable eviction.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PolicyCache<K: std::hash::Hash + Eq, V> {
    pub(crate) entries: std::collections::HashMap<K, (V, u64, u64)>,
    pub(crate) clock: u64,
    pub(crate) capacity: usize,
}

/// String interner — maps strings to compact IDs
#[allow(missing_docs)]
pub struct StringInterner {
    pub(crate) strings: Vec<String>,
    pub(crate) map: HashMap<String, u32>,
}

/// LFU eviction policy.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LfuEviction {
    pub(crate) min_freq: u64,
    pub(crate) age_factor: f64,
}

/// Bloom filter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BloomFilter {
    pub(crate) bits: Vec<u8>,
    pub(crate) size_bits: usize,
    pub(crate) num_hashes: usize,
}

/// An expression "diff" cache: stores the diff between two version of an expression.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ExprDiffCache {
    pub(crate) diffs: std::collections::HashMap<(u64, u64), String>,
    pub(crate) max_size: usize,
}

/// A cache that serialises itself to a byte sequence for persistence.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PersistentCache {
    pub(crate) entries: Vec<(u64, String)>,
}

/// Parse result cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseResultCache {
    pub(crate) entries: std::collections::HashMap<u64, ParseCacheEntry>,
    pub(crate) max_entries: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// LRU parse cache for declaration re-use
#[allow(missing_docs)]
pub struct ParseCache {
    pub(crate) entries: HashMap<DeclHash, CacheEntry>,
    pub(crate) max_entries: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// Expression segment for incremental re-parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ExprSegment {
    #[allow(missing_docs)]
    pub start: usize,
    #[allow(missing_docs)]
    pub end: usize,
    #[allow(missing_docs)]
    pub hash: u64,
    #[allow(missing_docs)]
    pub kind: SegmentKind,
}

/// Cache warmup configuration.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct CacheWarmup {
    #[allow(missing_docs)]
    pub sources: Vec<String>,
    #[allow(missing_docs)]
    pub priority: CachePriority,
    #[allow(missing_docs)]
    pub max_warmup_ms: u64,
}

/// Subexpression frequency map.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default)]
pub struct SubexprFrequencyMap {
    pub(crate) counts: std::collections::HashMap<u64, u32>,
}

/// Cache report with statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug)]
pub struct CacheReport {
    #[allow(missing_docs)]
    pub cache_size: usize,
    #[allow(missing_docs)]
    pub hit_count: u64,
    #[allow(missing_docs)]
    pub miss_count: u64,
    #[allow(missing_docs)]
    pub eviction_count: u64,
    #[allow(missing_docs)]
    pub memory_bytes: usize,
}

/// LRU cache implementation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LruCache<V> {
    pub(crate) capacity: usize,
    pub(crate) map: std::collections::HashMap<u64, V>,
    pub(crate) order: std::collections::VecDeque<u64>,
}

/// Cache prewarmer.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CachePrewarmer {
    pub(crate) sources: Vec<String>,
    pub(crate) warmup_count: usize,
}

/// Windowed cache metrics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug, Clone)]
pub struct WindowedCacheMetrics {
    #[allow(missing_docs)]
    pub window_hits: u64,
    #[allow(missing_docs)]
    pub window_misses: u64,
    #[allow(missing_docs)]
    pub window_evictions: u64,
    #[allow(missing_docs)]
    pub window_inserts: u64,
    #[allow(missing_docs)]
    pub window_size: usize,
}

/// TTL eviction policy.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TtlEviction {
    pub(crate) ttl_ticks: u64,
}

/// Classify cache entry tier.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheTier {
    #[allow(missing_docs)]
    Hot,
    #[allow(missing_docs)]
    Warm,
    #[allow(missing_docs)]
    Cold,
    #[allow(missing_docs)]
    Dead,
}

#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct TypeCheckResult {
    #[allow(missing_docs)]
    pub expr_hash: u64,
    #[allow(missing_docs)]
    pub inferred_type: String,
    #[allow(missing_docs)]
    pub is_valid: bool,
    #[allow(missing_docs)]
    pub check_time_us: u64,
}

/// Macro expansion cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MacroExpansionCache {
    pub(crate) entries: std::collections::HashMap<u64, MacroExpansionEntry>,
    pub(crate) max_size: usize,
}

/// Priority levels for cache entries.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CachePriority {
    #[allow(missing_docs)]
    Low = 0,
    #[allow(missing_docs)]
    Normal = 1,
    #[allow(missing_docs)]
    High = 2,
    #[allow(missing_docs)]
    Pinned = 3,
}

/// Interned string — lightweight identifier for a deduplicated string
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct InternedStr(pub(crate) u32);

/// Parse cache entry
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct CacheEntry {
    /// Hash of the declaration source text
    pub hash: DeclHash,
    /// Original source text
    pub source: String,
    /// Declaration name if known
    pub decl_name: Option<String>,
    /// Number of cache hits for this entry
    pub hit_count: u32,
}

#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentKind {
    #[allow(missing_docs)]
    Atom,
    #[allow(missing_docs)]
    App,
    #[allow(missing_docs)]
    Lambda,
    #[allow(missing_docs)]
    Pi,
    #[allow(missing_docs)]
    Let,
    #[allow(missing_docs)]
    Other,
}

/// Cache pressure monitor.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug)]
pub struct CachePressureMonitor {
    #[allow(missing_docs)]
    pub evictions: u64,
    #[allow(missing_docs)]
    pub inserts: u64,
    #[allow(missing_docs)]
    pub lookups: u64,
    #[allow(missing_docs)]
    pub hits: u64,
    #[allow(missing_docs)]
    pub peak_size: usize,
}

/// Window cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct WindowCache<K: std::hash::Hash + Eq + Clone, V> {
    pub(crate) map: std::collections::HashMap<K, V>,
    pub(crate) order: VecDeque<K>,
    pub(crate) window: usize,
}

/// Namespaced cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NamespacedCache<K: std::hash::Hash + Eq, V> {
    pub(crate) namespaces: std::collections::HashMap<String, std::collections::HashMap<K, V>>,
}

/// Expression pool with reference counting.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ExprPool {
    pub(crate) exprs: std::collections::HashMap<u64, (String, usize)>,
}

/// Segment table for cache invalidation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SegmentTable {
    pub(crate) segments: Vec<ExprSegment>,
    pub(crate) hashes_by_range: std::collections::BTreeMap<(usize, usize), u64>,
}

/// Memo table for parser results.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct MemoTable {
    pub(crate) entries: std::collections::HashMap<(usize, String), MemoEntry>,
}

/// Global expression table (hash-consing).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct GlobalExprTable {
    pub(crate) by_repr: std::collections::HashMap<String, u64>,
    pub(crate) by_hash: std::collections::HashMap<u64, String>,
    pub(crate) next_id: u64,
}

/// String pool for deduplication.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StringPool {
    pub(crate) pool: std::collections::HashSet<String>,
    pub(crate) total_saved_bytes: usize,
}

/// Alpha-equality cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AlphaEqCache {
    pub(crate) known_equal: std::collections::HashSet<(u64, u64)>,
    pub(crate) known_inequal: std::collections::HashSet<(u64, u64)>,
}

/// Token sequence hash for declaration fingerprinting
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct DeclHash(pub(crate) u64);

/// Expression location index: maps hash to source locations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ExprLocationIndex {
    pub(crate) index: std::collections::HashMap<u64, Vec<(usize, usize)>>,
}

/// Cache health report.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug)]
pub struct CacheHealthReport {
    #[allow(missing_docs)]
    pub total_entries: usize,
    #[allow(missing_docs)]
    pub hot_entries: usize,
    #[allow(missing_docs)]
    pub warm_entries: usize,
    #[allow(missing_docs)]
    pub cold_entries: usize,
    #[allow(missing_docs)]
    pub dead_entries: usize,
    #[allow(missing_docs)]
    pub estimated_waste_pct: f64,
}

#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct MemoEntry {
    #[allow(missing_docs)]
    pub end_pos: usize,
    #[allow(missing_docs)]
    pub result: String,
    #[allow(missing_docs)]
    pub success: bool,
}

#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseCacheEntry {
    #[allow(missing_docs)]
    pub source_hash: u64,
    #[allow(missing_docs)]
    pub result_repr: String,
    #[allow(missing_docs)]
    pub parse_time_us: u64,
    #[allow(missing_docs)]
    pub use_count: u64,
}

/// A sliding window for recent token sequences.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenWindow {
    pub(crate) tokens: std::collections::VecDeque<String>,
    pub(crate) capacity: usize,
}

/// Two-queue cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TwoQueueCache<K: std::hash::Hash + Eq + Clone, V> {
    pub(crate) capacity: usize,
    pub(crate) clock: u64,
    pub(crate) main: std::collections::HashMap<K, AdaptiveCacheEntry<V>>,
    pub(crate) probation: std::collections::VecDeque<K>,
    pub(crate) protected: std::collections::VecDeque<K>,
    pub(crate) probation_cap: usize,
}

/// Interning statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug)]
pub struct InterningStats {
    #[allow(missing_docs)]
    pub total_intern_calls: u64,
    #[allow(missing_docs)]
    pub unique_strings: u64,
    #[allow(missing_docs)]
    pub bytes_saved: u64,
}

/// Cache key builder using a chain of hash operations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CacheKeyBuilder {
    pub(crate) hash: u64,
}

/// Nesting depth tracker.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NestingDepthTracker {
    pub(crate) current_depth: usize,
    pub(crate) max_depth: usize,
    pub(crate) peak_depth: usize,
}

/// Rolling hash.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RollingHash {
    pub(crate) base: u64,
    pub(crate) modulus: u64,
    pub(crate) current: u64,
    pub(crate) window_size: usize,
    pub(crate) window: VecDeque<u8>,
    pub(crate) base_pow: u64,
}

/// Cache coverage report.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct CacheCoverageReport {
    #[allow(missing_docs)]
    pub total_source_bytes: usize,
    #[allow(missing_docs)]
    pub cached_bytes: usize,
    #[allow(missing_docs)]
    pub uncached_bytes: usize,
}

/// Hash set 64.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct HashSet64 {
    pub(crate) inner: std::collections::HashSet<u64>,
}

#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct MacroExpansionEntry {
    #[allow(missing_docs)]
    pub macro_hash: u64,
    #[allow(missing_docs)]
    pub arg_hash: u64,
    #[allow(missing_docs)]
    pub expansion: String,
    #[allow(missing_docs)]
    pub expansion_depth: usize,
    #[allow(missing_docs)]
    pub use_count: u32,
}

/// Bump allocator for string storage.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BumpAllocator {
    pub(crate) buffer: Vec<u8>,
    pub(crate) offset: usize,
}

/// Type check cache.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TypeCheckCache {
    pub(crate) cache: std::collections::HashMap<u64, TypeCheckResult>,
    pub(crate) capacity: usize,
}
