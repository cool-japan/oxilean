//! # Deque - Trait Implementations
//!
//! This module contains trait implementations for `Deque`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Deque;

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
