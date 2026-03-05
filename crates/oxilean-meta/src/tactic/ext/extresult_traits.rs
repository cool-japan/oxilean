//! # ExtResult - Trait Implementations
//!
//! This module contains trait implementations for `ExtResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtResult;
use std::fmt;

impl fmt::Display for ExtResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lemmas_applied.is_empty() {
            write!(f, "ext: no progress")
        } else {
            write!(
                f,
                "ext: applied {} lemma(s), {} new goal(s), depth {}",
                self.lemmas_applied.len(),
                self.new_goals.len(),
                self.depth_reached
            )
        }
    }
}
