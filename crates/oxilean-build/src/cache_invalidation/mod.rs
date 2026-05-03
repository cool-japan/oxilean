//! Smart build cache invalidation for the OxiLean build system.
//!
//! Provides content hashing (FNV-1a, DJB2, MurmurHash3), a `BuildCache`
//! with lookup/update/invalidate, dependency-propagating invalidation analysis,
//! topologically-sorted rebuild ordering, stale-entry pruning, and a
//! text-based serialisation format.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
