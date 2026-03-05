//! # DeadBranchStats - Trait Implementations
//!
//! This module contains trait implementations for `DeadBranchStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeadBranchStats;
use std::fmt;

impl fmt::Display for DeadBranchStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stats {{ cases={}, ctor_matches={}, unreach_defaults={}, single_inlines={}, uniform={} }}",
            self.cases_analyzed, self.known_ctor_matches, self.unreachable_defaults, self
            .single_branch_inlines, self.uniform_folds,
        )
    }
}
