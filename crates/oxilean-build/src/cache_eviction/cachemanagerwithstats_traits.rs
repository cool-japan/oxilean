//! # CacheManagerWithStats - Trait Implementations
//!
//! This module contains trait implementations for `CacheManagerWithStats`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheManagerWithStats;

impl std::fmt::Debug for CacheManagerWithStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheManagerWithStats")
            .field("current_size", &self.inner.current_size())
            .field("max_capacity", &self.inner.max_capacity())
            .field("eviction_rounds", &self.stats.eviction_rounds)
            .finish()
    }
}
