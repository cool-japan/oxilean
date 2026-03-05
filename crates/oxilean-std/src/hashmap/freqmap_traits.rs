//! # FreqMap - Trait Implementations
//!
//! This module contains trait implementations for `FreqMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AssocMap, FreqMap};

impl<K: PartialEq + Clone> Default for FreqMap<K> {
    fn default() -> Self {
        Self {
            counts: AssocMap::new(),
        }
    }
}
