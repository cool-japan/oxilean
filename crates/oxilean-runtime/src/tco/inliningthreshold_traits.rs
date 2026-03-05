//! # InliningThreshold - Trait Implementations
//!
//! This module contains trait implementations for `InliningThreshold`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InliningThreshold;

impl Default for InliningThreshold {
    fn default() -> Self {
        Self {
            max_size: 20,
            max_depth: 8,
            min_call_count: 3,
        }
    }
}
