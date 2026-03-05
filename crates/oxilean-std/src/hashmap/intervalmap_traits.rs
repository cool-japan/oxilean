//! # IntervalMap - Trait Implementations
//!
//! This module contains trait implementations for `IntervalMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IntervalMap;

impl<T: Ord + Clone, V: Clone> Default for IntervalMap<T, V> {
    fn default() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }
}
