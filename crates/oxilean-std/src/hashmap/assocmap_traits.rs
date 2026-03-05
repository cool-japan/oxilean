//! # AssocMap - Trait Implementations
//!
//! This module contains trait implementations for `AssocMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AssocMap;

impl<K: PartialEq + Clone, V: Clone> Default for AssocMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
