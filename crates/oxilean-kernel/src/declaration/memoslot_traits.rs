//! # MemoSlot - Trait Implementations
//!
//! This module contains trait implementations for `MemoSlot`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemoSlot;

impl<T: Clone> Default for MemoSlot<T> {
    fn default() -> Self {
        Self::new()
    }
}
