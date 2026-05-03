//! Functions for the Definitional Equality Cache.
//!
//! Provides the core cache operations: construction, lookup, insertion,
//! eviction, statistics, and a higher-order memoizing wrapper.

use crate::proof_cert::hash_expr;
use crate::Expr;

use super::types::{CacheEviction, DefEqCache, DefEqCacheStats, DefEqEntry, DefEqKey};

// ---------------------------------------------------------------------------
// DefEqKey construction
// ---------------------------------------------------------------------------

impl DefEqKey {
    /// Construct a canonical `DefEqKey` for the given expression pair.
    ///
    /// The two hashes are sorted so that `new(a, b) == new(b, a)`, enabling
    /// symmetric reuse of cache entries regardless of argument order.
    pub fn new(lhs: &Expr, rhs: &Expr) -> Self {
        let h_lhs = hash_expr(lhs);
        let h_rhs = hash_expr(rhs);
        if h_lhs <= h_rhs {
            DefEqKey {
                lhs_hash: h_lhs,
                rhs_hash: h_rhs,
            }
        } else {
            DefEqKey {
                lhs_hash: h_rhs,
                rhs_hash: h_lhs,
            }
        }
    }

    /// Construct a `DefEqKey` directly from two hashes (already in canonical order).
    pub fn from_hashes(a: u64, b: u64) -> Self {
        if a <= b {
            DefEqKey {
                lhs_hash: a,
                rhs_hash: b,
            }
        } else {
            DefEqKey {
                lhs_hash: b,
                rhs_hash: a,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DefEqCache construction and core operations
// ---------------------------------------------------------------------------

impl DefEqCache {
    /// Create a new empty cache with the given capacity and LRU eviction.
    pub fn new(max_size: usize) -> Self {
        DefEqCache {
            hits: 0,
            misses: 0,
            entries: std::collections::HashMap::new(),
            max_size,
            eviction: CacheEviction::LRU,
            clock: 0,
            insertion_order: std::collections::HashMap::new(),
            insert_clock: 0,
        }
    }

    /// Create a new cache with an explicit eviction policy.
    pub fn with_eviction(max_size: usize, eviction: CacheEviction) -> Self {
        DefEqCache {
            eviction,
            ..DefEqCache::new(max_size)
        }
    }

    /// Look up whether `(lhs, rhs)` is in the cache.
    ///
    /// Updates `hits`/`misses` and refreshes the LRU timestamp on a hit.
    pub fn lookup(&mut self, lhs: &Expr, rhs: &Expr) -> Option<bool> {
        let key = DefEqKey::new(lhs, rhs);
        self.clock = self.clock.wrapping_add(1);
        let now = self.clock;
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.checked_count = entry.checked_count.saturating_add(1);
            entry.last_access = now;
            self.hits += 1;
            Some(entry.result)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Insert the result of a definitional equality check into the cache.
    ///
    /// If the cache is full, one entry is evicted according to the policy
    /// before insertion.
    pub fn insert(&mut self, lhs: &Expr, rhs: &Expr, result: bool) {
        let key = DefEqKey::new(lhs, rhs);
        // If entry already exists, update it in-place.
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.result = result;
            return;
        }
        // Evict if at capacity.
        if self.entries.len() >= self.max_size && self.max_size > 0 {
            self.evict_by_policy();
        }
        self.clock = self.clock.wrapping_add(1);
        self.insert_clock = self.insert_clock.wrapping_add(1);
        let now = self.clock;
        let ins = self.insert_clock;
        self.insertion_order.insert(key, ins);
        self.entries.insert(
            key,
            DefEqEntry {
                key,
                result,
                checked_count: 0,
                last_access: now,
            },
        );
    }

    /// Evict one entry according to the configured eviction policy.
    fn evict_by_policy(&mut self) {
        match self.eviction {
            CacheEviction::LRU => self.evict_lru(),
            CacheEviction::LFU => self.evict_lfu(),
            CacheEviction::FIFO => self.evict_fifo(),
        }
    }

    /// Remove the least-recently-used entry.
    pub fn evict_lru(&mut self) {
        let victim = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.last_access)
            .map(|(k, _)| *k);
        if let Some(k) = victim {
            self.entries.remove(&k);
            self.insertion_order.remove(&k);
        }
    }

    /// Remove the least-frequently-used entry.
    pub fn evict_lfu(&mut self) {
        let victim = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.checked_count)
            .map(|(k, _)| *k);
        if let Some(k) = victim {
            self.entries.remove(&k);
            self.insertion_order.remove(&k);
        }
    }

    /// Remove the oldest-inserted entry (FIFO).
    pub fn evict_fifo(&mut self) {
        let victim = self
            .insertion_order
            .iter()
            .min_by_key(|(_, &ord)| ord)
            .map(|(k, _)| *k);
        if let Some(k) = victim {
            self.entries.remove(&k);
            self.insertion_order.remove(&k);
        }
    }

    /// Return a snapshot of current cache statistics.
    pub fn stats(&self) -> DefEqCacheStats {
        let total = self.hits + self.misses;
        let hit_rate = if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        };
        DefEqCacheStats {
            hits: self.hits,
            misses: self.misses,
            hit_rate,
            size: self.entries.len(),
        }
    }

    /// Clear all entries and reset statistics.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.insertion_order.clear();
        self.hits = 0;
        self.misses = 0;
        self.clock = 0;
        self.insert_clock = 0;
    }

    /// Return the number of currently cached entries.
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Return true if the cache has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Memoizing wrapper
// ---------------------------------------------------------------------------

/// Memoizing wrapper that checks the cache before invoking the checker.
///
/// If `(lhs, rhs)` is already in `cache`, returns the cached result immediately.
/// Otherwise, calls `check()`, stores the result in the cache, and returns it.
///
/// This is the primary way to integrate `DefEqCache` into a definitional
/// equality checker without modifying the checker itself.
///
/// # Example
///
/// ```ignore
/// let result = with_cache(&mut cache, &expr_a, &expr_b, || {
///     expensive_def_eq_check(&expr_a, &expr_b)
/// });
/// ```
pub fn with_cache<F>(cache: &mut DefEqCache, lhs: &Expr, rhs: &Expr, check: F) -> bool
where
    F: FnOnce() -> bool,
{
    if let Some(cached) = cache.lookup(lhs, rhs) {
        return cached;
    }
    let result = check();
    cache.insert(lhs, rhs, result);
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Level, Name};

    fn prop() -> Expr {
        Expr::Sort(Level::Zero)
    }

    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::Zero))
    }

    fn const_expr(name: &str) -> Expr {
        Expr::Const(Name::from_str(name), vec![])
    }

    fn bvar(n: u32) -> Expr {
        Expr::BVar(n)
    }

    // --- DefEqKey ---

    #[test]
    fn test_key_symmetric() {
        let a = prop();
        let b = type0();
        let k1 = DefEqKey::new(&a, &b);
        let k2 = DefEqKey::new(&b, &a);
        assert_eq!(k1, k2, "DefEqKey must be symmetric");
    }

    #[test]
    fn test_key_same_expr() {
        let a = prop();
        let k = DefEqKey::new(&a, &a);
        assert_eq!(k.lhs_hash, k.rhs_hash);
    }

    #[test]
    fn test_key_from_hashes_canonical() {
        let k1 = DefEqKey::from_hashes(10, 20);
        let k2 = DefEqKey::from_hashes(20, 10);
        assert_eq!(k1, k2);
        assert_eq!(k1.lhs_hash, 10);
        assert_eq!(k1.rhs_hash, 20);
    }

    #[test]
    fn test_key_distinct_exprs() {
        let a = prop();
        let b = type0();
        let ka = DefEqKey::new(&a, &a);
        let kb = DefEqKey::new(&b, &b);
        let kab = DefEqKey::new(&a, &b);
        // All three should be distinct (modulo hash collision, which is unlikely).
        assert_ne!(ka, kb);
        assert_ne!(ka, kab);
    }

    // --- DefEqCache construction ---

    #[test]
    fn test_new_cache_empty() {
        let cache = DefEqCache::new(128);
        assert!(cache.is_empty());
        assert_eq!(cache.hits, 0);
        assert_eq!(cache.misses, 0);
    }

    #[test]
    fn test_new_cache_zero_capacity() {
        // A zero-capacity cache should not panic.
        let cache = DefEqCache::new(0);
        assert_eq!(cache.max_size, 0);
    }

    // --- lookup / insert ---

    #[test]
    fn test_miss_on_empty() {
        let mut cache = DefEqCache::new(64);
        let result = cache.lookup(&prop(), &type0());
        assert!(result.is_none());
        assert_eq!(cache.misses, 1);
        assert_eq!(cache.hits, 0);
    }

    #[test]
    fn test_hit_after_insert() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        cache.insert(&a, &b, true);
        let result = cache.lookup(&a, &b);
        assert_eq!(result, Some(true));
        assert_eq!(cache.hits, 1);
    }

    #[test]
    fn test_symmetric_hit() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        cache.insert(&a, &b, false);
        // Lookup with reversed arguments should also hit.
        let result = cache.lookup(&b, &a);
        assert_eq!(result, Some(false));
    }

    #[test]
    fn test_insert_same_entry_twice() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        cache.insert(&a, &b, true);
        cache.insert(&a, &b, false); // overwrite
        let result = cache.lookup(&a, &b);
        assert_eq!(result, Some(false));
    }

    #[test]
    fn test_size_tracking() {
        let mut cache = DefEqCache::new(64);
        assert_eq!(cache.size(), 0);
        cache.insert(&prop(), &type0(), true);
        assert_eq!(cache.size(), 1);
        cache.insert(&bvar(0), &bvar(1), false);
        assert_eq!(cache.size(), 2);
    }

    // --- clear ---

    #[test]
    fn test_clear_resets_everything() {
        let mut cache = DefEqCache::new(64);
        cache.insert(&prop(), &type0(), true);
        let _ = cache.lookup(&prop(), &type0());
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.hits, 0);
        assert_eq!(cache.misses, 0);
    }

    // --- stats ---

    #[test]
    fn test_stats_zero_queries() {
        let cache = DefEqCache::new(64);
        let s = cache.stats();
        assert_eq!(s.hits, 0);
        assert_eq!(s.misses, 0);
        assert_eq!(s.hit_rate, 0.0);
        assert_eq!(s.size, 0);
    }

    #[test]
    fn test_stats_hit_rate() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        cache.insert(&a, &b, true);
        let _ = cache.lookup(&a, &b); // hit
        let _ = cache.lookup(&bvar(0), &bvar(1)); // miss
        let s = cache.stats();
        assert_eq!(s.hits, 1);
        assert_eq!(s.misses, 1);
        assert!((s.hit_rate - 0.5).abs() < 1e-9);
    }

    // --- eviction ---

    #[test]
    fn test_lru_eviction_capacity_respected() {
        let mut cache = DefEqCache::new(2);
        cache.insert(&prop(), &type0(), true);
        cache.insert(&bvar(0), &bvar(1), false);
        // Access first entry to make it recently used.
        let _ = cache.lookup(&prop(), &type0());
        // Insert a third entry — should evict the LRU (bvar(0)/bvar(1)).
        cache.insert(&const_expr("Nat"), &const_expr("Int"), true);
        assert_eq!(cache.size(), 2, "cache should not exceed max_size");
    }

    #[test]
    fn test_lfu_eviction() {
        let mut cache = DefEqCache::with_eviction(2, CacheEviction::LFU);
        cache.insert(&prop(), &type0(), true);
        cache.insert(&bvar(0), &bvar(1), false);
        // Access first entry many times.
        for _ in 0..5 {
            let _ = cache.lookup(&prop(), &type0());
        }
        // Insert third entry — should evict the less-frequently-used bvar pair.
        cache.insert(&const_expr("Nat"), &const_expr("Int"), true);
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_fifo_eviction() {
        let mut cache = DefEqCache::with_eviction(2, CacheEviction::FIFO);
        cache.insert(&prop(), &type0(), true); // inserted first
        cache.insert(&bvar(0), &bvar(1), false); // inserted second
                                                 // Access first entry to make it recently used (should NOT affect FIFO).
        let _ = cache.lookup(&prop(), &type0());
        // Third insertion should evict the first-inserted (prop/type0).
        cache.insert(&const_expr("Nat"), &const_expr("Int"), true);
        assert_eq!(cache.size(), 2);
        // After FIFO eviction of the first entry, its lookup should miss.
        let result = cache.lookup(&prop(), &type0());
        assert!(result.is_none(), "FIFO should have evicted the first entry");
    }

    #[test]
    fn test_evict_lru_explicit() {
        let mut cache = DefEqCache::new(4);
        cache.insert(&prop(), &type0(), true);
        cache.insert(&bvar(0), &bvar(1), false);
        // Access second entry to make it recently used.
        let _ = cache.lookup(&bvar(0), &bvar(1));
        cache.evict_lru();
        assert_eq!(cache.size(), 1);
        // The LRU (prop/type0, never accessed after insert) should be gone.
        assert!(cache.lookup(&prop(), &type0()).is_none());
    }

    // --- with_cache ---

    #[test]
    fn test_with_cache_miss_calls_check() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        let mut called = false;
        let result = with_cache(&mut cache, &a, &b, || {
            called = true;
            true
        });
        assert!(called, "checker should have been invoked on a miss");
        assert!(result);
    }

    #[test]
    fn test_with_cache_hit_skips_check() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        cache.insert(&a, &b, false);
        let mut called = false;
        let result = with_cache(&mut cache, &a, &b, || {
            called = true;
            true // would return a different value if called
        });
        assert!(!called, "checker must not be invoked on a cache hit");
        assert!(!result, "cached value (false) must be returned");
    }

    #[test]
    fn test_with_cache_stores_result() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        let _ = with_cache(&mut cache, &a, &b, || true);
        // Second call should be a cache hit.
        let result = with_cache(&mut cache, &a, &b, || false);
        assert!(result, "second call should return the cached true");
        assert_eq!(cache.hits, 1);
    }

    #[test]
    fn test_with_cache_symmetric_hit() {
        let mut cache = DefEqCache::new(64);
        let a = prop();
        let b = type0();
        let _ = with_cache(&mut cache, &a, &b, || true);
        // Reverse argument order.
        let mut called = false;
        let result = with_cache(&mut cache, &b, &a, || {
            called = true;
            false
        });
        assert!(!called);
        assert!(result);
    }

    // --- CacheEviction display ---

    #[test]
    fn test_eviction_display() {
        assert_eq!(format!("{}", CacheEviction::LRU), "LRU");
        assert_eq!(format!("{}", CacheEviction::LFU), "LFU");
        assert_eq!(format!("{}", CacheEviction::FIFO), "FIFO");
    }

    // --- DefEqCacheStats display ---

    #[test]
    fn test_stats_display() {
        let cache = DefEqCache::new(64);
        let s = cache.stats();
        let text = format!("{}", s);
        assert!(text.contains("DefEqCacheStats"));
    }
}
