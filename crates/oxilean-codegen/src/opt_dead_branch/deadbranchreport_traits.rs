//! # DeadBranchReport - Trait Implementations
//!
//! This module contains trait implementations for `DeadBranchReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeadBranchReport;
use std::fmt;

impl fmt::Display for DeadBranchReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DeadBranchReport {{ eliminated={}, folded={}, iters={} }}",
            self.branches_eliminated, self.cases_folded, self.iterations
        )
    }
}
