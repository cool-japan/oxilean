//! # ProofStep - Trait Implementations
//!
//! This module contains trait implementations for `ProofStep`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofStep;
use std::fmt;

impl fmt::Display for ProofStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.success { "OK" } else { "FAIL" };
        write!(
            f,
            "[{}] ({}) {} | goals: {} -> {}",
            self.step, status, self.tactic, self.goals_before, self.goals_after
        )
    }
}
