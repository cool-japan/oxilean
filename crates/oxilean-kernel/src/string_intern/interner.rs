//! Global `StringInterner` singleton and public API (`intern` / `resolve`).

use super::pool::InternPool;
use std::sync::{Arc, Mutex, OnceLock};

// ---------------------------------------------------------------------------
// InternedStr — lightweight handle
// ---------------------------------------------------------------------------

/// A lightweight, `Copy` handle to an interned string.
///
/// The index refers to a slot in the global `InternPool`. Use [`resolve`] to
/// obtain the `&'static str` for this handle.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InternedStr(u32);

impl InternedStr {
    /// Create an `InternedStr` from a raw pool index.
    ///
    /// In normal usage callers should obtain handles via [`intern`] rather
    /// than constructing them manually.
    #[inline]
    pub fn from_raw(idx: u32) -> Self {
        Self(idx)
    }

    /// Return the raw pool index.
    #[inline]
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for InternedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempt resolution; fall back to showing the raw index.
        match try_resolve(*self) {
            Some(s) => f.write_str(s),
            None => write!(f, "<InternedStr#{}>", self.0),
        }
    }
}

// ---------------------------------------------------------------------------
// StringInterner — thin wrapper owning the Arc<Mutex<InternPool>>
// ---------------------------------------------------------------------------

/// A thread-safe string interning pool.
///
/// Wraps an `Arc<Mutex<InternPool>>` so multiple owners can share the same
/// underlying storage. The global singleton is accessible via [`intern`] and
/// [`resolve`]; `StringInterner` can also be used as a standalone local
/// interner in tests or as an injected dependency.
pub struct StringInterner {
    pool: Arc<Mutex<InternPool>>,
}

impl StringInterner {
    /// Creates a new `StringInterner` backed by a fresh `InternPool`.
    pub fn new() -> Self {
        Self {
            pool: Arc::new(Mutex::new(InternPool::new())),
        }
    }

    /// Creates a `StringInterner` sharing the backing pool with `other`.
    pub fn shared_with(other: &StringInterner) -> Self {
        Self {
            pool: Arc::clone(&other.pool),
        }
    }

    /// Interns `s` and returns an `InternedStr` handle.
    ///
    /// Acquiring the lock returns an error only if the mutex is poisoned (a
    /// panic occurred while the lock was held). In that case this method
    /// panics with a descriptive message rather than silently continuing.
    pub fn intern(&self, s: &str) -> InternedStr {
        let mut guard = self
            .pool
            .lock()
            .expect("StringInterner pool mutex was poisoned");
        InternedStr::from_raw(guard.intern_str(s))
    }

    /// Resolves `handle` to its `&'static str` content.
    ///
    /// Returns `None` if the handle was not produced by this interner.
    pub fn resolve(&self, handle: InternedStr) -> Option<&'static str> {
        let mut guard = self
            .pool
            .lock()
            .expect("StringInterner pool mutex was poisoned");
        guard.resolve_str(handle.raw())
    }

    /// Returns the number of distinct strings stored.
    pub fn len(&self) -> usize {
        let guard = self
            .pool
            .lock()
            .expect("StringInterner pool mutex was poisoned");
        guard.len()
    }

    /// Returns `true` when no strings have been interned.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total byte count of all interned strings.
    pub fn total_bytes(&self) -> usize {
        let guard = self
            .pool
            .lock()
            .expect("StringInterner pool mutex was poisoned");
        guard.total_bytes()
    }

    /// Clones the underlying `Arc` so the pool can be shared across threads.
    pub fn arc_clone(&self) -> Arc<Mutex<InternPool>> {
        Arc::clone(&self.pool)
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Global singleton
// ---------------------------------------------------------------------------

/// Global `InternPool` guarded by a `Mutex`.
///
/// Initialised exactly once on the first call to [`intern`] or [`resolve`].
static GLOBAL_POOL: OnceLock<Arc<Mutex<InternPool>>> = OnceLock::new();

fn global_pool() -> &'static Arc<Mutex<InternPool>> {
    GLOBAL_POOL.get_or_init(|| Arc::new(Mutex::new(InternPool::new())))
}

/// Interns `s` into the global pool and returns an [`InternedStr`] handle.
///
/// Identical strings always return the same handle.
///
/// # Panics
///
/// Panics if the global pool mutex is poisoned (another thread panicked while
/// holding the lock, which should never happen under normal usage).
pub fn intern(s: &str) -> InternedStr {
    let mut guard = global_pool()
        .lock()
        .expect("global StringInterner mutex was poisoned");
    InternedStr::from_raw(guard.intern_str(s))
}

/// Resolves `handle` to its `&'static str` content using the global pool.
///
/// The returned slice has `'static` lifetime because the string is leaked
/// on first resolution and thereafter cached as a static pointer.
///
/// # Panics
///
/// Panics if the global pool mutex is poisoned or if `handle` was not
/// produced by [`intern`] (i.e., the raw index is out of range).
pub fn resolve(handle: InternedStr) -> &'static str {
    let mut guard = global_pool()
        .lock()
        .expect("global StringInterner mutex was poisoned");
    guard
        .resolve_str(handle.raw())
        .expect("InternedStr handle is out of range in the global pool")
}

/// Like [`resolve`] but returns `None` instead of panicking when the handle
/// is out of range.
pub fn try_resolve(handle: InternedStr) -> Option<&'static str> {
    let mut guard = global_pool().lock().ok()?;
    guard.resolve_str(handle.raw())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- InternedStr ---

    #[test]
    fn test_interned_str_is_copy() {
        let h = InternedStr::from_raw(0);
        let h2 = h; // copy
        assert_eq!(h, h2);
    }

    #[test]
    fn test_interned_str_raw_round_trip() {
        let h = InternedStr::from_raw(42);
        assert_eq!(h.raw(), 42);
    }

    #[test]
    fn test_interned_str_ordering() {
        let h0 = InternedStr::from_raw(0);
        let h1 = InternedStr::from_raw(1);
        assert!(h0 < h1);
    }

    // --- StringInterner (local, isolated) ---

    #[test]
    fn test_local_interner_deduplicates() {
        let si = StringInterner::new();
        let h1 = si.intern("apple");
        let h2 = si.intern("apple");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_local_interner_unique_for_distinct_strings() {
        let si = StringInterner::new();
        let h1 = si.intern("cat");
        let h2 = si.intern("dog");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_local_interner_resolve() {
        let si = StringInterner::new();
        let h = si.intern("oxilean");
        assert_eq!(si.resolve(h), Some("oxilean"));
    }

    #[test]
    fn test_local_interner_len() {
        let si = StringInterner::new();
        si.intern("x");
        si.intern("y");
        si.intern("x"); // duplicate
        assert_eq!(si.len(), 2);
    }

    #[test]
    fn test_local_interner_total_bytes() {
        let si = StringInterner::new();
        si.intern("abc"); // 3
        si.intern("de"); // 2
        assert_eq!(si.total_bytes(), 5);
    }

    #[test]
    fn test_local_interner_is_empty_initially() {
        let si = StringInterner::new();
        assert!(si.is_empty());
    }

    #[test]
    fn test_local_interner_shared_with() {
        let si1 = StringInterner::new();
        let h1 = si1.intern("shared");
        let si2 = StringInterner::shared_with(&si1);
        let h2 = si2.intern("shared");
        assert_eq!(h1, h2, "shared interners must return identical handles");
    }

    #[test]
    fn test_local_interner_resolve_out_of_range() {
        let si = StringInterner::new();
        assert!(si.resolve(InternedStr::from_raw(999)).is_none());
    }

    #[test]
    fn test_local_interner_empty_string() {
        let si = StringInterner::new();
        let h = si.intern("");
        assert_eq!(si.resolve(h), Some(""));
    }

    #[test]
    fn test_local_interner_unicode() {
        let si = StringInterner::new();
        let h = si.intern("αβγ");
        assert_eq!(si.resolve(h), Some("αβγ"));
    }

    #[test]
    fn test_local_interner_many_strings() {
        let si = StringInterner::new();
        let strings: Vec<String> = (0..100).map(|i| format!("str_{}", i)).collect();
        let handles: Vec<InternedStr> = strings.iter().map(|s| si.intern(s)).collect();
        // All unique
        let unique: std::collections::HashSet<InternedStr> = handles.iter().copied().collect();
        assert_eq!(unique.len(), 100);
        // All resolve correctly
        for (i, h) in handles.iter().enumerate() {
            let resolved = si.resolve(*h).expect("should resolve");
            assert_eq!(resolved, strings[i].as_str());
        }
    }

    // --- Global API ---

    #[test]
    fn test_global_intern_deduplicates() {
        let h1 = intern("__global_test_string_A__");
        let h2 = intern("__global_test_string_A__");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_global_resolve_returns_correct_content() {
        let h = intern("__global_test_string_B__");
        assert_eq!(resolve(h), "__global_test_string_B__");
    }

    #[test]
    fn test_try_resolve_missing_handle_returns_none() {
        // A fresh local interner produces handles that are unknown to the
        // global pool (unless the raw index happens to collide).
        // We test try_resolve with a very large index instead.
        let result = try_resolve(InternedStr::from_raw(u32::MAX));
        assert!(result.is_none());
    }

    #[test]
    fn test_global_intern_concurrent() {
        use std::thread;
        let handles: Vec<_> = (0..8)
            .map(|_| thread::spawn(|| intern("concurrent_intern_test")))
            .collect();
        let results: Vec<InternedStr> = handles
            .into_iter()
            .map(|h| h.join().expect("thread should not panic"))
            .collect();
        // All threads must get the same handle.
        let first = results[0];
        assert!(results.iter().all(|&h| h == first));
    }

    #[test]
    fn test_display_interned_str() {
        let h = intern("display_test");
        let s = format!("{}", h);
        assert_eq!(s, "display_test");
    }
}
