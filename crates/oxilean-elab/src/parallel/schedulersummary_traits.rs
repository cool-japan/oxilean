//! # SchedulerSummary - Trait Implementations
//!
//! This module contains trait implementations for `SchedulerSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SchedulerSummary;
use std::fmt;

impl fmt::Display for SchedulerSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SchedulerSummary: {}/{} completed, {} failed, {}ms total",
            self.completed, self.total, self.failed, self.total_time_ms
        )
    }
}
