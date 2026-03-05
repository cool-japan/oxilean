//! # BiMap - Trait Implementations
//!
//! This module contains trait implementations for `BiMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BiMap;

impl<K: PartialEq + Clone, V: PartialEq + Clone> Default for BiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
