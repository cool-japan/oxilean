//! # MonotoneChain - Trait Implementations
//!
//! This module contains trait implementations for `MonotoneChain`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MonotoneChain;

impl<T: Ord + Clone> Default for MonotoneChain<T> {
    fn default() -> Self {
        Self::new()
    }
}
