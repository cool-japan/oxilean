//! # BatchExecutionResult - Trait Implementations
//!
//! This module contains trait implementations for `BatchExecutionResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BatchExecutionResult;
use std::fmt;

impl fmt::Display for BatchExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Batch {}: {} successful, {} errors, {}ms",
            self.batch_id,
            self.results.len(),
            self.errors.len(),
            self.batch_time_ms
        )
    }
}
