//! # InlinePass - Trait Implementations
//!
//! This module contains trait implementations for `InlinePass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{InlineConfig, InlinePass};

impl Default for InlinePass {
    fn default() -> Self {
        Self::new(InlineConfig::default())
    }
}
