//! Types for the memoized WHNF reduction layer.
//!
//! Provides cache key/entry/configuration/statistics types for the WHNF memo
//! table.  Cache correctness is guaranteed by including the current environment
//! version in every key: when the environment grows (a new declaration is
//! added), `invalidate_all` bumps `env_version`, rendering all prior entries
//! stale without requiring an explicit scan.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// WhnfKey
// ---------------------------------------------------------------------------

/// Cache key for a single WHNF reduction result.
///
/// Including `env_version` ensures that cached reductions are invalidated
/// automatically when the environment changes: a key computed under version `v`
/// will never match an entry stored under version `v'` where `v' != v`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WhnfKey {
    /// FNV-1a hash of the expression bytes to be reduced.
    pub expr_hash: u64,
    /// Monotonically increasing environment version at query time.
    pub env_version: u32,
}

// ---------------------------------------------------------------------------
// WhnfEntry
// ---------------------------------------------------------------------------

/// A single cached WHNF reduction result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhnfEntry {
    /// FNV-1a hash of the WHNF-reduced result expression.
    pub result_hash: u64,
    /// Number of reduction steps performed to reach WHNF (used for eviction
    /// decisions: entries that required few steps are cheap to recompute).
    pub reduction_steps: u32,
    /// Number of times this entry has been returned from the cache.
    pub access_count: u32,
}

// ---------------------------------------------------------------------------
// MemoConfig
// ---------------------------------------------------------------------------

/// Configuration parameters for a [`WhnfMemo`] instance.
#[derive(Clone, Debug)]
pub struct MemoConfig {
    /// Maximum number of entries in the cache before eviction is triggered.
    pub max_entries: usize,
    /// Minimum number of reduction steps required before a result is cached.
    ///
    /// Results achieved in fewer steps are cheap to recompute and not worth
    /// the memory overhead.
    pub min_steps_to_cache: u32,
    /// Fraction of `max_entries` used as the access-count threshold during
    /// cold eviction.  Entries whose `access_count` is below
    /// `floor(max_entries * eviction_threshold)` are considered cold and may
    /// be removed.
    pub eviction_threshold: f64,
}

impl Default for MemoConfig {
    fn default() -> Self {
        MemoConfig {
            max_entries: 4096,
            min_steps_to_cache: 2,
            eviction_threshold: 0.1,
        }
    }
}

// ---------------------------------------------------------------------------
// WhnfMemo
// ---------------------------------------------------------------------------

/// Memoization table for WHNF reduction results.
///
/// # Correctness
///
/// Every lookup and insertion is tied to `env_version`.  Calling
/// `invalidate_all` bumps the version and clears all entries, guaranteeing
/// that stale results (computed against an older environment) are never
/// returned.
///
/// # Eviction
///
/// When the table reaches `config.max_entries`, `evict_cold` is called
/// automatically during `insert`.  Cold entries — those whose `access_count`
/// falls below a threshold derived from `config.eviction_threshold` — are
/// removed.  If no cold entries exist the oldest-inserted entry is dropped.
pub struct WhnfMemo {
    /// Cached entries, keyed by (expression hash, env version).
    pub(crate) entries: HashMap<WhnfKey, WhnfEntry>,
    /// Total number of successful cache lookups.
    pub hits: u64,
    /// Total number of cache misses.
    pub misses: u64,
    /// Total number of entries removed by eviction.
    pub evictions: u64,
    /// Current environment version.
    pub env_version: u32,
    /// Configuration controlling capacity, caching threshold, and eviction.
    pub(crate) config: MemoConfig,
    /// Monotonically increasing insertion counter (used to implement FIFO
    /// fallback eviction when no cold entries are found).
    pub(crate) insert_order: Vec<WhnfKey>,
}

// ---------------------------------------------------------------------------
// MemoStats
// ---------------------------------------------------------------------------

/// A snapshot of [`WhnfMemo`] performance statistics.
#[derive(Clone, Debug)]
pub struct MemoStats {
    /// Total cache hits.
    pub hits: u64,
    /// Total cache misses.
    pub misses: u64,
    /// Total evictions.
    pub evictions: u64,
    /// Hit rate in [0.0, 1.0].
    pub hit_rate: f64,
    /// Current number of cached entries.
    pub size: usize,
    /// Current environment version.
    pub env_version: u32,
}

impl std::fmt::Display for MemoStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoStats {{ hits: {}, misses: {}, evictions: {}, hit_rate: {:.2}%, size: {}, env_version: {} }}",
            self.hits,
            self.misses,
            self.evictions,
            self.hit_rate * 100.0,
            self.size,
            self.env_version
        )
    }
}
