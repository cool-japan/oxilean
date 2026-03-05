//! # CompletionPipeline - Trait Implementations
//!
//! This module contains trait implementations for `CompletionPipeline`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompletionPipeline;
use std::fmt;

impl Default for CompletionPipeline {
    fn default() -> Self {
        CompletionPipeline::new()
    }
}
