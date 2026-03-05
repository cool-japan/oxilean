//! # WorkStack - Trait Implementations
//!
//! This module contains trait implementations for `WorkStack`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WorkStack;

impl<T> Default for WorkStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
