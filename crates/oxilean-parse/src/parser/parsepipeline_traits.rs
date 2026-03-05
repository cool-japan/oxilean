//! # ParsePipeline - Trait Implementations
//!
//! This module contains trait implementations for `ParsePipeline`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParsePipeline;

impl std::fmt::Display for ParsePipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParsePipeline({} stages)", self.stages.len())
    }
}
