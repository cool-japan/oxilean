//! # WriteOnce - Trait Implementations
//!
//! This module contains trait implementations for `WriteOnce`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WriteOnce;
use std::fmt;

impl<T: Copy> Default for WriteOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}
