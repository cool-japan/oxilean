//! # DeadBranchConfig - Trait Implementations
//!
//! This module contains trait implementations for `DeadBranchConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeadBranchConfig;
use std::fmt;

impl Default for DeadBranchConfig {
    fn default() -> Self {
        DeadBranchConfig {
            max_passes: 8,
            fold_constants: true,
            use_profiling: false,
            max_alts_analyzed: 256,
        }
    }
}

impl fmt::Display for DeadBranchConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DeadBranchConfig {{ max_passes={}, fold_constants={}, profiling={} }}",
            self.max_passes, self.fold_constants, self.use_profiling
        )
    }
}
