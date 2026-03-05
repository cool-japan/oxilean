//! # ParallelScheduler - Trait Implementations
//!
//! This module contains trait implementations for `ParallelScheduler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelScheduler;
use std::fmt;

impl Default for ParallelScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ParallelScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let summary = self.summary();
        write!(f, "{}", summary)
    }
}
