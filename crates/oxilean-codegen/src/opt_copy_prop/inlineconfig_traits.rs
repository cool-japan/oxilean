//! # InlineConfig - Trait Implementations
//!
//! This module contains trait implementations for `InlineConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::DEFAULT_INLINE_THRESHOLD;
use super::types::InlineConfig;

impl Default for InlineConfig {
    fn default() -> Self {
        InlineConfig {
            threshold: DEFAULT_INLINE_THRESHOLD,
            inline_recursive: false,
        }
    }
}
