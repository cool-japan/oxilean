//! Types for the Definitional Equality Cache.
//!
//! Caches the results of definitional equality checks to avoid redundant
//! re-checking of expression pairs that have already been decided.

use std::collections::HashMap;

/// A symmetric key for a definitional equality query.
///
/// The two expression hashes are stored in canonical order (`lhs_hash <= rhs_hash`)
/// so that `is_def_eq(a, b)` and `is_def_eq(b, a)` map to the same cache entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DefEqKey {
    /// Hash of the lexicographically smaller expression.
    pub lhs_hash: u64,
    /// Hash of the lexicographically larger expression.
    pub rhs_hash: u64,
}

/// A single cached definitional equality result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefEqEntry {
    /// The key identifying the pair of expressions.
    pub key: DefEqKey,
    /// Whether the two expressions are definitionally equal.
    pub result: bool,
    /// How many times this entry has been used to answer a query.
    pub checked_count: u32,
    /// Logical timestamp of last access (used by LRU eviction).
    pub(crate) last_access: u64,
}

/// Eviction policy applied when the cache exceeds `max_size` entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheEviction {
    /// Least-recently-used: evict the entry accessed furthest in the past.
    LRU,
    /// Least-frequently-used: evict the entry with the smallest `checked_count`.
    LFU,
    /// First-in, first-out: evict the entry with the smallest insertion order.
    FIFO,
}

impl std::fmt::Display for CacheEviction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheEviction::LRU => write!(f, "LRU"),
            CacheEviction::LFU => write!(f, "LFU"),
            CacheEviction::FIFO => write!(f, "FIFO"),
        }
    }
}

/// Persistent cache of definitional equality results.
///
/// Maintains hit/miss statistics and enforces a configurable capacity bound,
/// evicting entries according to the chosen [`CacheEviction`] policy.
pub struct DefEqCache {
    /// Number of successful cache lookups.
    pub hits: u64,
    /// Number of cache misses (query not found).
    pub misses: u64,
    /// Stored entries, keyed by [`DefEqKey`].
    pub(crate) entries: HashMap<DefEqKey, DefEqEntry>,
    /// Maximum number of entries before eviction kicks in.
    pub max_size: usize,
    /// Eviction policy.
    pub(crate) eviction: CacheEviction,
    /// Monotonically increasing logical clock.
    pub(crate) clock: u64,
    /// Insertion-order counter per entry (for FIFO).
    pub(crate) insertion_order: HashMap<DefEqKey, u64>,
    /// Global insertion counter.
    pub(crate) insert_clock: u64,
}

/// A snapshot of cache performance statistics.
#[derive(Clone, Debug)]
pub struct DefEqCacheStats {
    /// Total cache hits.
    pub hits: u64,
    /// Total cache misses.
    pub misses: u64,
    /// Hit rate in [0.0, 1.0].
    pub hit_rate: f64,
    /// Current number of cached entries.
    pub size: usize,
}

impl std::fmt::Display for DefEqCacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DefEqCacheStats {{ hits: {}, misses: {}, hit_rate: {:.2}%, size: {} }}",
            self.hits,
            self.misses,
            self.hit_rate * 100.0,
            self.size
        )
    }
}
