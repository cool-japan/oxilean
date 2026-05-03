//! Definitional Equality Cache for OxiLean Kernel.
//!
//! Provides a persistent cache of definitional equality results to avoid
//! redundant re-checking of expression pairs that the kernel has already
//! decided. Typical kernels spend significant time re-checking the same
//! `(t1, t2)` pairs in nested reduction contexts; caching these results
//! can yield substantial speedups.
//!
//! # Design
//!
//! - **Symmetric keys**: `DefEqKey::new(a, b) == DefEqKey::new(b, a)` since
//!   definitional equality is symmetric.
//! - **Hash-based**: Expression hashes (FNV-1a via `proof_cert::hash_expr`) are
//!   used as keys. Hash collisions are benign — they may cause spurious cache
//!   misses but never incorrect results (the hash merely gates a lookup; the
//!   checker is always authoritative).
//! - **Configurable eviction**: Choose `LRU`, `LFU`, or `FIFO` at construction
//!   time via [`DefEqCache::with_eviction`].
//! - **Memoizing wrapper**: [`with_cache`] transparently wraps any checker.

pub mod functions;
pub mod types;

pub use functions::with_cache;
pub use types::{CacheEviction, DefEqCache, DefEqCacheStats, DefEqEntry, DefEqKey};
