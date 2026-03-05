//! # TracePipeline - Trait Implementations
//!
//! This module contains trait implementations for `TracePipeline`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TracePipeline;
use std::fmt;

impl Default for TracePipeline {
    fn default() -> Self {
        TracePipeline::new()
    }
}
