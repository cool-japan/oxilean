//! # ExecutionStats - Trait Implementations
//!
//! This module contains trait implementations for `ExecutionStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExecutionStats;
use std::fmt;

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExecutionStats: avg={}ms, min={}ms, max={}ms, parallelism={}x",
            self.avg_time_ms, self.min_time_ms, self.max_time_ms, self.parallelism_factor
        )
    }
}
