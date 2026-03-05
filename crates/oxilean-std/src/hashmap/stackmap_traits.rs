//! # StackMap - Trait Implementations
//!
//! This module contains trait implementations for `StackMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StackMap;

impl<K: PartialEq + Clone, V: Clone> Default for StackMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
