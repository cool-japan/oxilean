//! # GrindStats - Trait Implementations
//!
//! This module contains trait implementations for `GrindStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GrindStats;
use std::fmt;

impl fmt::Display for GrindStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GrindStats {{ rounds: {}, merges: {}, ematches: {}, instances: {}, \
             splits: {}, max_eclass: {}, nodes: {} }}",
            self.rounds,
            self.merges,
            self.ematches,
            self.instances,
            self.splits,
            self.max_eclass,
            self.total_nodes
        )
    }
}
