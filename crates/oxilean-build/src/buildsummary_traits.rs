//! # BuildSummary - Trait Implementations
//!
//! This module contains trait implementations for `BuildSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildSummary;
use std::fmt;

impl fmt::Display for BuildSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BuildSummary {{ compiled: {}, cached: {}, failed: {}, {}ms }}",
            self.compiled, self.cached, self.failed, self.elapsed_ms
        )
    }
}
