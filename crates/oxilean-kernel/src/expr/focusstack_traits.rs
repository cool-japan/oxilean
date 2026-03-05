//! # FocusStack - Trait Implementations
//!
//! This module contains trait implementations for `FocusStack`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FocusStack;

impl<T> Default for FocusStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
