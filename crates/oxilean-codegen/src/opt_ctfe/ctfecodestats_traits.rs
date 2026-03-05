//! # CtfeCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `CtfeCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeCodeStats;
use std::fmt;

impl std::fmt::Display for CtfeCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CtfeCodeStats {{ constants={}, folds={}, calls_elim={}, loops_unrolled={}, conditions={} }}",
            self.constants_discovered, self.folds_applied, self.calls_eliminated, self
            .loops_unrolled, self.conditions_resolved,
        )
    }
}
