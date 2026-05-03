//! Memoized WHNF reduction with environment-version-aware correctness.
//!
//! Stores the result of WHNF (Weak Head Normal Form) reductions indexed by an
//! FNV-1a hash of the input expression and the current environment version.
//! When the environment grows (a new declaration is added to the kernel
//! `Environment`), [`WhnfMemo::invalidate_all`] bumps the version and clears
//! all entries, guaranteeing that stale results are never returned.
//!
//! # Design
//!
//! - **Version-gated keys**: [`WhnfKey`] pairs `expr_hash` with `env_version`
//!   so entries from a previous environment are never matched.
//! - **Step-threshold caching**: results from trivially cheap reductions
//!   (`steps < config.min_steps_to_cache`) are not stored, trading correctness
//!   for reduced memory pressure.
//! - **Cold eviction**: [`WhnfMemo::evict_cold`] removes entries with low
//!   `access_count`; a FIFO fallback guarantees progress when all entries are
//!   equally hot.
//! - **Memoizing wrapper**: [`with_memo`] transparently wraps any reduction
//!   closure, handling lookup, conditional insertion, and result forwarding.
//!
//! # Example
//!
//! ```
//! use oxilean_kernel::whnf_memo::{WhnfMemo, MemoConfig, hash_bytes, with_memo};
//!
//! let mut memo = WhnfMemo::new(MemoConfig::default());
//! let key = hash_bytes(b"some_expression_bytes");
//!
//! let result = with_memo(&mut memo, key, 0, || {
//!     // expensive WHNF reduction would go here
//!     let result_hash = hash_bytes(b"whnf_result_bytes");
//!     let steps = 42u32;
//!     (result_hash, steps)
//! });
//!
//! let stats = memo.stats();
//! assert_eq!(stats.misses, 1);
//! assert_eq!(stats.hits, 0);
//! ```

pub mod functions;
pub mod types;

pub use functions::{hash_bytes, with_memo};
pub use types::{MemoConfig, MemoStats, WhnfEntry, WhnfKey, WhnfMemo};
