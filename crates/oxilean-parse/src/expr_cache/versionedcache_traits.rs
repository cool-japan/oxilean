//! # VersionedCache - Trait Implementations
//!
//! This module contains trait implementations for `VersionedCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionedCache;

impl<K: std::hash::Hash + Eq, V> Default for VersionedCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
