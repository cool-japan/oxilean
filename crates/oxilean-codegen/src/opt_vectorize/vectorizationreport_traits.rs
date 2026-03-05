//! # VectorizationReport - Trait Implementations
//!
//! This module contains trait implementations for `VectorizationReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VectorizationReport;
use std::fmt;

impl fmt::Display for VectorizationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VectorizationReport {{ analyzed={}, vectorized={}, rejected=(dep={}, trip={}, other={}), avg_speedup={:.2}x }}",
            self.loops_analyzed, self.loops_vectorized, self.rejected_dep, self
            .rejected_trip_count, self.rejected_other, self.avg_estimated_speedup,
        )
    }
}
