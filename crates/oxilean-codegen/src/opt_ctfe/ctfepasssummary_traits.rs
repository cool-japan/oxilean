//! # CtfePassSummary - Trait Implementations
//!
//! This module contains trait implementations for `CtfePassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfePassSummary;
use std::fmt;

impl std::fmt::Display for CtfePassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CtfePassSummary[{}] {{ processed={}, replaced={}, errors={}, {}us }}",
            self.pass_name, self.funcs_processed, self.replacements, self.errors, self.duration_us
        )
    }
}
