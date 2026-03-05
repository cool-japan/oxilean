//! # ResolutionStats - Trait Implementations
//!
//! This module contains trait implementations for `ResolutionStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ResolutionStats;
use std::fmt;

impl fmt::Display for ResolutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "resolved={} backtracks={} queries={} conflicts={} time={}ms",
            self.resolved_packages,
            self.backtrack_count,
            self.registry_queries,
            self.conflicts_resolved,
            self.resolution_time_ms
        )
    }
}
