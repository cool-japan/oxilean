//! # LazyMap - Trait Implementations
//!
//! This module contains trait implementations for `LazyMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyMap;
use std::fmt;

impl<K: Eq + std::hash::Hash + Clone, V: Clone + fmt::Debug> Default for LazyMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
