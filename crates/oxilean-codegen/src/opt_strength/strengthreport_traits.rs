//! # StrengthReport - Trait Implementations
//!
//! This module contains trait implementations for `StrengthReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::StrengthReport;
use std::fmt;

impl fmt::Display for StrengthReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StrengthReport {{ mul={}, div={}, pow={}, iv={}, inc={}, neg={} }}",
            self.mul_reduced,
            self.div_reduced,
            self.pow_reduced,
            self.iv_reductions,
            self.inc_reduced,
            self.neg_reduced,
        )
    }
}
