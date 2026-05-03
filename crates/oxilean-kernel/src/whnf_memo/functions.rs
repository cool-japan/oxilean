//! Functions for the memoized WHNF reduction layer.
//!
//! Provides the core operations on [`WhnfMemo`]: construction, lookup,
//! insertion, invalidation, cold-entry eviction, statistics, and a
//! higher-order memoizing wrapper.

use std::collections::HashMap;

use super::types::{MemoConfig, MemoStats, WhnfEntry, WhnfKey, WhnfMemo};

// ---------------------------------------------------------------------------
// FNV-1a hash helper
// ---------------------------------------------------------------------------

/// Compute a 64-bit FNV-1a hash of `data`.
///
/// FNV-1a is a non-cryptographic hash chosen for its speed and avalanche
/// behaviour on small inputs, which is typical for serialised expression bytes.
pub fn hash_bytes(data: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
    const PRIME: u64 = 1_099_511_628_211;

    data.iter()
        .fold(OFFSET_BASIS, |h, &b| (h ^ b as u64).wrapping_mul(PRIME))
}

// ---------------------------------------------------------------------------
// WhnfMemo construction and core operations
// ---------------------------------------------------------------------------

impl WhnfMemo {
    /// Create a new, empty `WhnfMemo` with the given configuration.
    pub fn new(config: MemoConfig) -> Self {
        WhnfMemo {
            entries: HashMap::new(),
            hits: 0,
            misses: 0,
            evictions: 0,
            env_version: 0,
            config,
            insert_order: Vec::new(),
        }
    }

    /// Look up the cached WHNF result hash for `expr_hash` under the current
    /// environment version.
    ///
    /// Returns `Some(result_hash)` on a hit and increments `hits`.
    /// Returns `None` on a miss and increments `misses`.
    pub fn lookup(&mut self, expr_hash: u64) -> Option<u64> {
        let key = WhnfKey {
            expr_hash,
            env_version: self.env_version,
        };
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.access_count = entry.access_count.saturating_add(1);
            self.hits += 1;
            Some(entry.result_hash)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Insert a WHNF reduction result into the cache.
    ///
    /// The result is only stored if `steps >= config.min_steps_to_cache`,
    /// since trivially cheap reductions are not worth caching.
    ///
    /// If the cache is at capacity, `evict_cold` is called first to make room.
    pub fn insert(&mut self, expr_hash: u64, result_hash: u64, steps: u32) {
        if steps < self.config.min_steps_to_cache {
            return;
        }
        let key = WhnfKey {
            expr_hash,
            env_version: self.env_version,
        };
        // Update existing entry in-place rather than evicting and re-inserting.
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.result_hash = result_hash;
            entry.reduction_steps = steps;
            return;
        }
        let max = self.config.max_entries;
        if max > 0 && self.entries.len() >= max {
            self.evict_cold();
        }
        self.entries.insert(
            key,
            WhnfEntry {
                result_hash,
                reduction_steps: steps,
                access_count: 0,
            },
        );
        self.insert_order.push(key);
    }

    /// Invalidate the entire cache by bumping the environment version.
    ///
    /// All previously stored entries are cleared; their keys will no longer
    /// match any future lookup (which uses the new `env_version`).
    pub fn invalidate_all(&mut self) {
        self.entries.clear();
        self.insert_order.clear();
        self.env_version = self.env_version.wrapping_add(1);
    }

    /// Evict cold entries — those whose `access_count` is below a threshold
    /// computed from `config.eviction_threshold * max_entries`.
    ///
    /// If no cold entries are found (all entries are hot), the oldest-inserted
    /// entry is removed instead (FIFO fallback), guaranteeing that at least one
    /// slot is freed.
    pub fn evict_cold(&mut self) {
        let threshold = (self.config.max_entries as f64 * self.config.eviction_threshold) as u32;

        // Collect cold keys.
        let cold: Vec<WhnfKey> = self
            .entries
            .iter()
            .filter(|(_, e)| e.access_count <= threshold)
            .map(|(k, _)| *k)
            .collect();

        if !cold.is_empty() {
            let removed = cold.len() as u64;
            for k in &cold {
                self.entries.remove(k);
            }
            // Rebuild insert_order without the evicted keys.
            self.insert_order.retain(|k| !cold.contains(k));
            self.evictions += removed;
        } else {
            // FIFO fallback: remove the oldest-inserted entry.
            if let Some(oldest) = self.insert_order.first().copied() {
                self.insert_order.remove(0);
                self.entries.remove(&oldest);
                self.evictions += 1;
            }
        }
    }

    /// Return a snapshot of current memo statistics.
    pub fn stats(&self) -> MemoStats {
        let total = self.hits + self.misses;
        let hit_rate = if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        };
        MemoStats {
            hits: self.hits,
            misses: self.misses,
            evictions: self.evictions,
            hit_rate,
            size: self.entries.len(),
            env_version: self.env_version,
        }
    }
}

// ---------------------------------------------------------------------------
// Memoizing wrapper
// ---------------------------------------------------------------------------

/// Memoizing wrapper for WHNF reduction.
///
/// Checks `memo` for a cached result before invoking `compute`.  If the cache
/// misses, `compute` is called; its `(result_hash, steps)` return value is
/// stored in `memo` (subject to `min_steps_to_cache`) before being returned.
///
/// # Parameters
///
/// * `memo` — mutable reference to the memo table.
/// * `key` — FNV-1a hash of the expression to be reduced.
/// * `min_steps` — overriding minimum steps threshold for this call site
///   (the greater of this value and `memo.config.min_steps_to_cache` is used).
/// * `compute` — closure that performs the actual reduction and returns
///   `(result_hash, reduction_steps)`.
pub fn with_memo<F>(memo: &mut WhnfMemo, key: u64, min_steps: u32, compute: F) -> u64
where
    F: FnOnce() -> (u64, u32),
{
    if let Some(cached) = memo.lookup(key) {
        return cached;
    }
    let (result_hash, steps) = compute();
    let effective_min = memo.config.min_steps_to_cache.max(min_steps);
    if steps >= effective_min {
        memo.insert(key, result_hash, steps);
    }
    result_hash
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn default_memo() -> WhnfMemo {
        WhnfMemo::new(MemoConfig::default())
    }

    fn memo_with_capacity(n: usize) -> WhnfMemo {
        WhnfMemo::new(MemoConfig {
            max_entries: n,
            min_steps_to_cache: 1,
            eviction_threshold: 0.0,
        })
    }

    // --- hash_bytes ---

    #[test]
    fn test_hash_bytes_empty() {
        let h = hash_bytes(&[]);
        // FNV-1a of empty input is the offset basis.
        assert_eq!(h, 14_695_981_039_346_656_037);
    }

    #[test]
    fn test_hash_bytes_deterministic() {
        let h1 = hash_bytes(b"hello");
        let h2 = hash_bytes(b"hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_bytes_distinct() {
        let ha = hash_bytes(b"Nat.add");
        let hb = hash_bytes(b"List.map");
        assert_ne!(ha, hb);
    }

    // --- WhnfMemo construction ---

    #[test]
    fn test_new_memo_empty() {
        let m = default_memo();
        assert_eq!(m.hits, 0);
        assert_eq!(m.misses, 0);
        assert_eq!(m.evictions, 0);
        assert_eq!(m.env_version, 0);
        assert!(m.entries.is_empty());
    }

    // --- lookup / insert ---

    #[test]
    fn test_miss_on_empty() {
        let mut m = default_memo();
        assert_eq!(m.lookup(42), None);
        assert_eq!(m.misses, 1);
    }

    #[test]
    fn test_hit_after_insert() {
        let mut m = default_memo();
        m.insert(100, 200, 5);
        let result = m.lookup(100);
        assert_eq!(result, Some(200));
        assert_eq!(m.hits, 1);
    }

    #[test]
    fn test_insert_below_min_steps_not_cached() {
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 64,
            min_steps_to_cache: 3,
            eviction_threshold: 0.1,
        });
        m.insert(1, 999, 2); // steps < min_steps_to_cache
        assert_eq!(m.lookup(1), None);
    }

    #[test]
    fn test_insert_at_min_steps_cached() {
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 64,
            min_steps_to_cache: 3,
            eviction_threshold: 0.1,
        });
        m.insert(1, 999, 3); // steps == min_steps_to_cache
        assert_eq!(m.lookup(1), Some(999));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut m = default_memo();
        m.insert(1, 100, 5);
        m.insert(1, 200, 7); // overwrite
        assert_eq!(m.lookup(1), Some(200));
    }

    // --- invalidate_all ---

    #[test]
    fn test_invalidate_all_clears_entries() {
        let mut m = default_memo();
        m.insert(1, 10, 5);
        m.invalidate_all();
        assert!(m.entries.is_empty());
    }

    #[test]
    fn test_invalidate_all_bumps_version() {
        let mut m = default_memo();
        assert_eq!(m.env_version, 0);
        m.invalidate_all();
        assert_eq!(m.env_version, 1);
        m.invalidate_all();
        assert_eq!(m.env_version, 2);
    }

    #[test]
    fn test_invalidate_all_prior_entries_miss() {
        let mut m = default_memo();
        m.insert(42, 99, 5);
        m.invalidate_all();
        // After version bump, the old entry is gone.
        assert_eq!(m.lookup(42), None);
    }

    #[test]
    fn test_insert_after_invalidate_uses_new_version() {
        let mut m = default_memo();
        m.insert(42, 1, 5);
        m.invalidate_all();
        m.insert(42, 2, 5);
        assert_eq!(m.lookup(42), Some(2));
    }

    // --- evict_cold ---

    #[test]
    fn test_evict_cold_removes_unaccessed() {
        let mut m = memo_with_capacity(4);
        m.insert(1, 10, 5);
        m.insert(2, 20, 5);
        // Access entry 2 to make it hot.
        let _ = m.lookup(2);
        let _ = m.lookup(2);
        // Evict cold (threshold = 0, so entries with access_count 0 are cold).
        m.evict_cold();
        // Entry 1 was never accessed → cold → evicted.
        assert_eq!(m.lookup(1), None);
        // Entry 2 was accessed → hot → kept.
        assert_eq!(m.lookup(2), Some(20));
        assert!(m.evictions > 0);
    }

    #[test]
    fn test_evict_cold_fifo_fallback() {
        // Set eviction_threshold to 1.0 so all entries are "hot"
        // (threshold = floor(2 * 1.0) = 2, but access_count starts at 0
        // so after one access it becomes 1, still <= 1).
        // Use threshold just below access_count to force FIFO path:
        // set threshold to -inf effectively by making min_steps very high
        // and access every entry multiple times.
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 2,
            min_steps_to_cache: 1,
            eviction_threshold: 0.0, // threshold = 0 → access_count <= 0 → only unaccessed
        });
        m.insert(1, 10, 5);
        m.insert(2, 20, 5);
        // Access both to make them "hot" (access_count = 1 > 0).
        let _ = m.lookup(1);
        let _ = m.lookup(2);
        // Now no cold entries exist → FIFO fallback: oldest (key 1) evicted.
        m.evict_cold();
        assert!(m.evictions > 0);
    }

    // --- stats ---

    #[test]
    fn test_stats_zero() {
        let m = default_memo();
        let s = m.stats();
        assert_eq!(s.hits, 0);
        assert_eq!(s.misses, 0);
        assert_eq!(s.hit_rate, 0.0);
        assert_eq!(s.size, 0);
        assert_eq!(s.env_version, 0);
    }

    #[test]
    fn test_stats_hit_rate() {
        let mut m = default_memo();
        m.insert(1, 10, 5);
        let _ = m.lookup(1); // hit
        let _ = m.lookup(2); // miss
        let s = m.stats();
        assert_eq!(s.hits, 1);
        assert_eq!(s.misses, 1);
        assert!((s.hit_rate - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_stats_display() {
        let m = default_memo();
        let s = m.stats();
        let text = format!("{}", s);
        assert!(text.contains("MemoStats"));
    }

    // --- with_memo ---

    #[test]
    fn test_with_memo_miss_calls_compute() {
        let mut m = default_memo();
        let mut called = false;
        let result = with_memo(&mut m, 42, 0, || {
            called = true;
            (99, 5)
        });
        assert!(called);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_with_memo_hit_skips_compute() {
        let mut m = default_memo();
        m.insert(42, 99, 5);
        let mut called = false;
        let result = with_memo(&mut m, 42, 0, || {
            called = true;
            (0, 10)
        });
        assert!(!called);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_with_memo_stores_result() {
        let mut m = default_memo();
        let _ = with_memo(&mut m, 77, 0, || (55, 5));
        // Second call should hit the cache.
        let mut called = false;
        let result = with_memo(&mut m, 77, 0, || {
            called = true;
            (0, 5)
        });
        assert!(!called);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_with_memo_respects_min_steps() {
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 64,
            min_steps_to_cache: 10,
            eviction_threshold: 0.1,
        });
        // compute returns only 3 steps → below threshold → not cached
        let _ = with_memo(&mut m, 5, 0, || (7, 3));
        assert_eq!(m.lookup(5), None);
    }

    #[test]
    fn test_with_memo_min_steps_override() {
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 64,
            min_steps_to_cache: 2,
            eviction_threshold: 0.1,
        });
        // with_memo min_steps=10 overrides config min_steps=2 for this call.
        let _ = with_memo(&mut m, 5, 10, || (7, 3));
        assert_eq!(m.lookup(5), None);
    }

    #[test]
    fn test_capacity_triggers_eviction() {
        let mut m = memo_with_capacity(2);
        m.insert(1, 10, 5);
        m.insert(2, 20, 5);
        // Third insert exceeds capacity → eviction.
        m.insert(3, 30, 5);
        assert!(m.entries.len() <= 2);
        assert!(m.evictions > 0);
    }

    #[test]
    fn test_zero_capacity_no_panic() {
        let mut m = WhnfMemo::new(MemoConfig {
            max_entries: 0,
            min_steps_to_cache: 1,
            eviction_threshold: 0.0,
        });
        // With max_entries=0 the capacity check is skipped; insert still stores.
        m.insert(1, 99, 5);
        // No panic is the pass criterion.
    }

    #[test]
    fn test_eviction_count_tracked() {
        let mut m = memo_with_capacity(1);
        m.insert(1, 10, 5);
        m.insert(2, 20, 5); // triggers eviction
        assert!(m.evictions >= 1);
    }
}
