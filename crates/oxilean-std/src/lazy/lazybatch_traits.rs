//! # LazyBatch - Trait Implementations
//!
//! This module contains trait implementations for `LazyBatch`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyBatch;

impl<A: 'static> Default for LazyBatch<A> {
    fn default() -> Self {
        Self::new()
    }
}
