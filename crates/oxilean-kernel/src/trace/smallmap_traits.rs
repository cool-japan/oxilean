//! # SmallMap - Trait Implementations
//!
//! This module contains trait implementations for `SmallMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SmallMap;
use std::fmt;

impl<K: Ord + Clone, V: Clone> Default for SmallMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
