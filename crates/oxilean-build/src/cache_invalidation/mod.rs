//! Smart build cache invalidation for the OxiLean build system.
//!
//! Provides content hashing (FNV-1a, DJB2, MurmurHash3), a `BuildCache`
//! with lookup/update/invalidate, dependency-propagating invalidation analysis,
//! topologically-sorted rebuild ordering, stale-entry pruning, a
//! text-based serialisation format, and oxilake.lock-aware invalidation.

pub mod functions;
pub mod lockfile_invalidator;
pub mod types;

pub use functions::*;
pub use lockfile_invalidator::{
    check_lockfile_invalidation, hash_lockfile_content, parse_lockfile_deps, LockfileState,
};
pub use types::*;
