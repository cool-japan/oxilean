//! # PrologGoalBuilder - build_group Methods
//!
//! This module contains method implementations for `PrologGoalBuilder`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrologTerm;

use super::prologgoalbuilder_type::PrologGoalBuilder;

impl PrologGoalBuilder {
    /// Build the goal list.
    #[allow(dead_code)]
    pub fn build(self) -> Vec<PrologTerm> {
        self.goals
    }
}
