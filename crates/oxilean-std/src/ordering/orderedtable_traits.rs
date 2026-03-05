//! # OrderedTable - Trait Implementations
//!
//! This module contains trait implementations for `OrderedTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OrderedTable;

impl<K: std::cmp::Ord, V> Default for OrderedTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
