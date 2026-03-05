//! # WeakTable - Trait Implementations
//!
//! This module contains trait implementations for `WeakTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WeakTable;

impl<T: Clone> Default for WeakTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
