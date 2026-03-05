//! # DecideConfig - Trait Implementations
//!
//! This module contains trait implementations for `DecideConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DecideConfig;

impl Default for DecideConfig {
    fn default() -> Self {
        DecideConfig {
            max_depth: 100,
            timeout_ms: 5000,
        }
    }
}
