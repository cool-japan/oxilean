//! # LazyAccumulator - Trait Implementations
//!
//! This module contains trait implementations for `LazyAccumulator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyAccumulator;
use std::fmt;

impl<T: Clone + fmt::Debug> Default for LazyAccumulator<T> {
    fn default() -> Self {
        Self::new()
    }
}
