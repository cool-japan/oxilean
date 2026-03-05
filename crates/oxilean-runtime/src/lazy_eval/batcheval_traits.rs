//! # BatchEval - Trait Implementations
//!
//! This module contains trait implementations for `BatchEval`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BatchEval;
use std::fmt;

impl<T: Clone + fmt::Debug> Default for BatchEval<T> {
    fn default() -> Self {
        Self::new()
    }
}
