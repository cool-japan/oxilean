//! # PositivityConfig - Trait Implementations
//!
//! This module contains trait implementations for `PositivityConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PositivityConfig;

impl Default for PositivityConfig {
    fn default() -> Self {
        PositivityConfig {
            strict: false,
            max_depth: 50,
        }
    }
}
