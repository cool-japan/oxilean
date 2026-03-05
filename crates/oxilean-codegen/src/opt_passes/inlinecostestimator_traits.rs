//! # InlineCostEstimator - Trait Implementations
//!
//! This module contains trait implementations for `InlineCostEstimator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InlineCostEstimator;

impl Default for InlineCostEstimator {
    fn default() -> Self {
        InlineCostEstimator {
            always_inline_threshold: 3,
            hot_threshold: 20,
            cold_threshold: 5,
            tail_recursive_penalty: 10,
        }
    }
}
