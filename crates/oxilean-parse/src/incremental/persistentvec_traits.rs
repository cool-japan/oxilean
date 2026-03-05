//! # PersistentVec - Trait Implementations
//!
//! This module contains trait implementations for `PersistentVec`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PersistentVec;

impl<T: Clone> Default for PersistentVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
