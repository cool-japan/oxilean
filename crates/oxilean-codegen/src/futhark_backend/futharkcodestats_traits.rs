//! # FutharkCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `FutharkCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkCodeStats;
use std::fmt;

impl std::fmt::Display for FutharkCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FutharkCodeStats {{ fns={}, entries={}, types={}, maps={}, reduces={}, scans={} }}",
            self.num_functions,
            self.num_entries,
            self.num_type_defs,
            self.num_map_exprs,
            self.num_reduce_exprs,
            self.num_scan_exprs,
        )
    }
}
