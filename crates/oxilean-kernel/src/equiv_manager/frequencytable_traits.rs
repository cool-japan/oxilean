//! # FrequencyTable - Trait Implementations
//!
//! This module contains trait implementations for `FrequencyTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FrequencyTable;

impl<T: std::hash::Hash + Eq + Clone> Default for FrequencyTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
