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
use std::fmt;

impl<T> Default for FocusStack<T> {
    fn default() -> Self {
        Self::new()
    }
}
