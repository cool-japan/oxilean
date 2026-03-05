//! # UnrollConfig - Trait Implementations
//!
//! This module contains trait implementations for `UnrollConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnrollConfig;

impl Default for UnrollConfig {
    fn default() -> Self {
        Self {
            factor: 4,
            full_unroll_limit: 16,
        }
    }
}
