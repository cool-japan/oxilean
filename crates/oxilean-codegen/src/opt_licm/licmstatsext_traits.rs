//! # LicmStatsExt - Trait Implementations
//!
//! This module contains trait implementations for `LicmStatsExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmStatsExt;
use std::fmt;

impl std::fmt::Display for LicmStatsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LicmStatsExt {{ loops={}, candidates={}, hoisted={}, sunk={}, rejected={} }}",
            self.loops_analyzed, self.candidates_found, self.hoisted, self.sunk, self.rejected,
        )
    }
}
