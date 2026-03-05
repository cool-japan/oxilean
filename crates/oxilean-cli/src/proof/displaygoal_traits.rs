//! # DisplayGoal - Trait Implementations
//!
//! This module contains trait implementations for `DisplayGoal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DisplayGoal;
use std::fmt;

impl fmt::Display for DisplayGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Goal {}:", self.index)?;
        for (name, ty) in &self.hypotheses {
            writeln!(f, "  {} : {}", name, ty)?;
        }
        writeln!(f, "  \u{22a2} {}", self.goal_type)
    }
}
