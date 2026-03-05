//! # MultiMap - Trait Implementations
//!
//! This module contains trait implementations for `MultiMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MultiMap;

impl<K: PartialEq + Clone, V: Clone> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
