//! # GoalDiff - Trait Implementations
//!
//! This module contains trait implementations for `GoalDiff`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GoalDiff;

impl std::fmt::Display for GoalDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GoalDiff(closed={}, opened={})",
            self.num_closed(),
            self.num_opened()
        )
    }
}
