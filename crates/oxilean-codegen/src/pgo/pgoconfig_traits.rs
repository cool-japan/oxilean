//! # PgoConfig - Trait Implementations
//!
//! This module contains trait implementations for `PgoConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PgoConfig;

impl Default for PgoConfig {
    fn default() -> Self {
        Self {
            hot_threshold: 100,
            inline_hot: true,
            specialize_hot: true,
            max_inline_size: 50,
        }
    }
}
