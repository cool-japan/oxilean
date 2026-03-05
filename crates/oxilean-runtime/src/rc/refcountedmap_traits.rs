//! # RefcountedMap - Trait Implementations
//!
//! This module contains trait implementations for `RefcountedMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RefcountedMap;

impl<K: Eq + std::hash::Hash + Clone, V: Clone> Default for RefcountedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
