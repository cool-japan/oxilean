//! # RcPool - Trait Implementations
//!
//! This module contains trait implementations for `RcPool`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcPool;

impl<T: Clone> Default for RcPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
