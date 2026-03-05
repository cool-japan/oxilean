//! # NamespacedCache - Trait Implementations
//!
//! This module contains trait implementations for `NamespacedCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NamespacedCache;

impl<K: std::hash::Hash + Eq, V> Default for NamespacedCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
