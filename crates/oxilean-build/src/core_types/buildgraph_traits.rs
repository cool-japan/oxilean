//! # BuildGraph - Trait Implementations
//!
//! This module contains trait implementations for `BuildGraph`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::num_cpus;
use super::types::BuildGraph;

impl Default for BuildGraph {
    fn default() -> Self {
        Self::new()
    }
}
