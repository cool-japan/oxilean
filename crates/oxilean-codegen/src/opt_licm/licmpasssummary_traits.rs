//! # LicmPassSummary - Trait Implementations
//!
//! This module contains trait implementations for `LicmPassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmPassSummary;
use std::fmt;

impl std::fmt::Display for LicmPassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LicmPassSummary[{}] {{ fns={}, hoisted={}, sunk={}, {}us }}",
            self.pass_name, self.functions_processed, self.hoisted, self.sunk, self.duration_us,
        )
    }
}
