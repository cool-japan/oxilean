//! # LicmCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `LicmCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmCodeStats;
use std::fmt;

impl std::fmt::Display for LicmCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LicmCodeStats {{ loops={}, invariant={}, hoisted={}, sunk={}, rejected={} }}",
            self.loops_analyzed, self.invariant_exprs, self.hoisted, self.sunk, self.rejected,
        )
    }
}
