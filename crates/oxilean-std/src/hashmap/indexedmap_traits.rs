//! # IndexedMap - Trait Implementations
//!
//! This module contains trait implementations for `IndexedMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IndexedMap;

impl<V: Clone> Default for IndexedMap<V> {
    fn default() -> Self {
        Self {
            slots: Vec::new(),
            count: 0,
        }
    }
}
