//! # MemoThunk - Trait Implementations
//!
//! This module contains trait implementations for `MemoThunk`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemoThunk;

impl<T: Clone> Default for MemoThunk<T> {
    fn default() -> Self {
        MemoThunk::new()
    }
}
