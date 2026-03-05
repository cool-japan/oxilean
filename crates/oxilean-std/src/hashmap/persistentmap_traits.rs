//! # PersistentMap - Trait Implementations
//!
//! This module contains trait implementations for `PersistentMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PersistentMap;

impl<K: Ord + Clone, V: Clone> Default for PersistentMap<K, V> {
    fn default() -> Self {
        Self::empty()
    }
}
