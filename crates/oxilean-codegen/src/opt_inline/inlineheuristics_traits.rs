//! # InlineHeuristics - Trait Implementations
//!
//! This module contains trait implementations for `InlineHeuristics`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InlineHeuristics;

impl Default for InlineHeuristics {
    fn default() -> Self {
        InlineHeuristics {
            max_inline_size: 40,
            max_inline_depth: 4,
            always_inline_threshold: 100,
            never_inline_size: 200,
        }
    }
}
