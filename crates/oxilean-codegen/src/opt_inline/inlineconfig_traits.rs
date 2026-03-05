//! # InlineConfig - Trait Implementations
//!
//! This module contains trait implementations for `InlineConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{InlineConfig, InlineHeuristics};

impl Default for InlineConfig {
    fn default() -> Self {
        InlineConfig {
            heuristics: InlineHeuristics::default(),
            enable_recursive_inlining: false,
            enable_hot_inlining: true,
            max_passes: 3,
        }
    }
}
