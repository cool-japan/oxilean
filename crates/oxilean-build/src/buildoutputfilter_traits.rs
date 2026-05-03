//! # BuildOutputFilter - Trait Implementations
//!
//! This module contains trait implementations for `BuildOutputFilter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::num_cpus;
use super::types::BuildOutputFilter;

impl Default for BuildOutputFilter {
    fn default() -> Self {
        Self::new()
    }
}
