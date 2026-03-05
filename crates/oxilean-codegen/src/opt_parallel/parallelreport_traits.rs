//! # ParallelReport - Trait Implementations
//!
//! This module contains trait implementations for `ParallelReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelReport;
use std::fmt;

impl fmt::Display for ParallelReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParallelReport {{ found={}, transformed={}, speedup={:.2}x, races={} }}",
            self.regions_found,
            self.regions_transformed,
            self.estimated_total_speedup,
            self.race_conditions_detected,
        )
    }
}
