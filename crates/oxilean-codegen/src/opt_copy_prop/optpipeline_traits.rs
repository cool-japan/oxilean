//! # OptPipeline - Trait Implementations
//!
//! This module contains trait implementations for `OptPipeline`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::OptPipeline;

impl Default for OptPipeline {
    fn default() -> Self {
        Self::new()
    }
}
