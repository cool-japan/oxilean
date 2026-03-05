//! # TacticStateInfo - Trait Implementations
//!
//! This module contains trait implementations for `TacticStateInfo`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacticStateInfo;
use std::fmt;

impl Default for TacticStateInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TacticStateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.goals.is_empty() {
            write!(f, "no goals")?;
        } else {
            write!(f, "{} goal(s)", self.goals.len())?;
            for (i, goal) in self.goals.iter().enumerate() {
                if i == self.focus {
                    write!(f, "\n>>> ")?;
                } else {
                    write!(f, "\n    ")?;
                }
                write!(f, "{}", goal)?;
            }
        }
        Ok(())
    }
}
