//! # RuleApplication - Trait Implementations
//!
//! This module contains trait implementations for `RuleApplication`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RuleApplication;
use std::fmt;

impl fmt::Display for RuleApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = if self.forward { "fwd" } else { "bwd" };
        if self.closed_goal() {
            write!(
                f,
                "[{}] {} closed `{}`",
                mode, self.rule_name, self.goal_before
            )
        } else {
            write!(
                f,
                "[{}] {} on `{}` -> {} sub-goal(s)",
                mode,
                self.rule_name,
                self.goal_before,
                self.subgoals_after.len()
            )
        }
    }
}
