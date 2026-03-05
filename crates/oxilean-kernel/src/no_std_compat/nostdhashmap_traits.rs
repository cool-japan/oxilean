//! # NoStdHashMap - Trait Implementations
//!
//! This module contains trait implementations for `NoStdHashMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NoStdHashMap;

impl<K, V> Default for NoStdHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
