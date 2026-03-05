//! # CacheManager - Trait Implementations
//!
//! This module contains trait implementations for `CacheManager`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheManager;

impl std::fmt::Debug for CacheManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheManager")
            .field("entries", &self.entries.len())
            .field("current_size", &self.current_size)
            .field("max_capacity", &self.max_capacity)
            .field("eviction_count", &self.eviction_count)
            .field("policy", &self.policy.name())
            .finish()
    }
}
