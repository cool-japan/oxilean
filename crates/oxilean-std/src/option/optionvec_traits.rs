//! # OptionVec - Trait Implementations
//!
//! This module contains trait implementations for `OptionVec`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `FromIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OptionVec;

impl<T> Default for OptionVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<Option<T>> for OptionVec<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}
