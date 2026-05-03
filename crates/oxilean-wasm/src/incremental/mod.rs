//! Incremental type-checking support for OxiLean.
//!
//! Instead of re-checking an entire file on every edit, this module caches
//! checked declarations and only re-checks those whose content or transitive
//! dependencies changed.
//!
//! # Quick start
//!
//! ```no_run
//! use oxilean_wasm::incremental::{incremental_check, cache_stats};
//!
//! // First pass — no prior cache
//! let result = incremental_check("theorem foo : True := trivial", None);
//!
//! // Second pass — reuse the cache; unchanged declarations are skipped
//! let result2 = incremental_check("theorem foo : True := trivial\ndef bar := 42",
//!                                  Some(result.cache));
//! println!("{}", cache_stats(&result2.cache));
//! ```

pub mod functions;
pub mod types;

pub use functions::{
    cache_stats, compute_edit_delta, extract_declarations, hash_declaration, incremental_check,
    invalidate_dependents,
};
pub use types::{
    CheckStatus, DeclHash, DiagnosticInfo, EditDelta, IncrementalCache, IncrementalCheckResult,
    IncrementalEntry,
};
