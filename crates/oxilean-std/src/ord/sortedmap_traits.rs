//! # SortedMap - Trait Implementations
//!
//! This module contains trait implementations for `SortedMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SortedMap;

impl<K: Ord, V> Default for SortedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
