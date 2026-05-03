//! `InternPool` — backing storage for the string interning table.

use std::collections::HashMap;

/// Statistics snapshot for an `InternPool`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternPoolStats {
    /// Number of distinct strings stored.
    pub len: usize,
    /// Sum of byte lengths of all stored strings (not including null terminators).
    pub total_bytes: usize,
}

/// The raw storage for interned strings.
///
/// - `strings` is append-only; indices are stable for the lifetime of the pool.
/// - `map` maps each string's content to its index for O(1) deduplication.
/// - `static_ptrs` caches the `&'static str` obtained by leaking each string
///   exactly once, so subsequent `resolve` calls are O(1) slice lookups.
pub struct InternPool {
    /// All interned strings in insertion order.
    pub(super) strings: Vec<String>,
    /// Reverse map: content → index.
    pub(super) map: HashMap<String, u32>,
    /// Leaked static pointers, one per interned string (lazy-filled).
    pub(super) static_ptrs: Vec<Option<&'static str>>,
}

impl InternPool {
    /// Creates a new, empty `InternPool`.
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: HashMap::new(),
            static_ptrs: Vec::new(),
        }
    }

    /// Interns `s`, returning a stable index.
    ///
    /// If `s` is already present the existing index is returned without
    /// any allocation. Otherwise a new `String` is pushed and indexed.
    pub fn intern_str(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.static_ptrs.push(None);
        self.map.insert(s.to_string(), id);
        id
    }

    /// Resolves index `id` to a `&'static str`.
    ///
    /// The string is leaked into a `Box<str>` on the first call for `id` and
    /// the resulting pointer is cached. Subsequent calls for the same `id`
    /// return the cached pointer without any allocation.
    ///
    /// Returns `None` if `id` is out of range.
    pub fn resolve_str(&mut self, id: u32) -> Option<&'static str> {
        let idx = id as usize;
        if idx >= self.strings.len() {
            return None;
        }
        // Use cached static pointer if available.
        if let Some(ptr) = self.static_ptrs[idx] {
            return Some(ptr);
        }
        // Leak a copy as a Box<str> → &'static str.
        let leaked: &'static str = Box::leak(self.strings[idx].clone().into_boxed_str());
        self.static_ptrs[idx] = Some(leaked);
        Some(leaked)
    }

    /// Returns the number of distinct strings stored.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns `true` when no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Returns the total byte count of all interned strings.
    pub fn total_bytes(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Returns a `InternPoolStats` snapshot.
    pub fn stats(&self) -> InternPoolStats {
        InternPoolStats {
            len: self.len(),
            total_bytes: self.total_bytes(),
        }
    }
}

impl Default for InternPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_intern_returns_same_index_for_same_string() {
        let mut pool = InternPool::new();
        let id1 = pool.intern_str("hello");
        let id2 = pool.intern_str("hello");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_pool_intern_returns_different_indices_for_different_strings() {
        let mut pool = InternPool::new();
        let id1 = pool.intern_str("hello");
        let id2 = pool.intern_str("world");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_pool_resolve_returns_correct_string() {
        let mut pool = InternPool::new();
        let id = pool.intern_str("kernel");
        let s = pool.resolve_str(id);
        assert_eq!(s, Some("kernel"));
    }

    #[test]
    fn test_pool_resolve_out_of_range_returns_none() {
        let mut pool = InternPool::new();
        assert!(pool.resolve_str(99).is_none());
    }

    #[test]
    fn test_pool_len_counts_unique_strings() {
        let mut pool = InternPool::new();
        pool.intern_str("a");
        pool.intern_str("b");
        pool.intern_str("a"); // duplicate
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_pool_total_bytes() {
        let mut pool = InternPool::new();
        pool.intern_str("abc"); // 3 bytes
        pool.intern_str("de"); // 2 bytes
        assert_eq!(pool.total_bytes(), 5);
    }

    #[test]
    fn test_pool_stats_snapshot() {
        let mut pool = InternPool::new();
        pool.intern_str("hello");
        pool.intern_str("world");
        let stats = pool.stats();
        assert_eq!(stats.len, 2);
        assert_eq!(stats.total_bytes, 10);
    }

    #[test]
    fn test_pool_is_empty_initially() {
        let pool = InternPool::new();
        assert!(pool.is_empty());
    }

    #[test]
    fn test_pool_default_is_empty() {
        let pool = InternPool::default();
        assert!(pool.is_empty());
    }

    #[test]
    fn test_pool_sequential_indices() {
        let mut pool = InternPool::new();
        let ids: Vec<u32> = ["x", "y", "z"].iter().map(|s| pool.intern_str(s)).collect();
        assert_eq!(ids, vec![0, 1, 2]);
    }

    #[test]
    fn test_pool_empty_string_internable() {
        let mut pool = InternPool::new();
        let id = pool.intern_str("");
        assert_eq!(pool.resolve_str(id), Some(""));
    }

    #[test]
    fn test_pool_unicode_string() {
        let mut pool = InternPool::new();
        let id = pool.intern_str("日本語");
        assert_eq!(pool.resolve_str(id), Some("日本語"));
    }

    #[test]
    fn test_pool_resolve_same_pointer_on_repeated_calls() {
        let mut pool = InternPool::new();
        let id = pool.intern_str("stable");
        let ptr1 = pool.resolve_str(id).map(|s| s.as_ptr());
        let ptr2 = pool.resolve_str(id).map(|s| s.as_ptr());
        assert_eq!(
            ptr1, ptr2,
            "resolve must return the same static pointer each time"
        );
    }
}
