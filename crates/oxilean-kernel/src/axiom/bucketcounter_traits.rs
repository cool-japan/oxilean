//! # BucketCounter - Trait Implementations
//!
//! This module contains trait implementations for `BucketCounter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BucketCounter;

impl<const N: usize> Default for BucketCounter<N> {
    fn default() -> Self {
        Self::new()
    }
}
