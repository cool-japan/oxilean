//! # ClosureSizeEstimator - Trait Implementations
//!
//! This module contains trait implementations for `ClosureSizeEstimator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ClosureSizeEstimator;

impl Default for ClosureSizeEstimator {
    fn default() -> Self {
        Self::new(8)
    }
}
