//! # CongrConfig - Trait Implementations
//!
//! This module contains trait implementations for `CongrConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CongrConfig;

impl Default for CongrConfig {
    fn default() -> Self {
        CongrConfig {
            max_depth: 20,
            use_hyps: true,
        }
    }
}
