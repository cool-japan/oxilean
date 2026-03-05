//! # ThunkVec - Trait Implementations
//!
//! This module contains trait implementations for `ThunkVec`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThunkVec;
use std::fmt;

impl<T: Clone + fmt::Debug> Default for ThunkVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
