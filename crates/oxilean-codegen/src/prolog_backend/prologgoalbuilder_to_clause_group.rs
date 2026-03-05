//! # PrologGoalBuilder - to_clause_group Methods
//!
//! This module contains method implementations for `PrologGoalBuilder`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{PrologClause, PrologTerm};

use super::prologgoalbuilder_type::PrologGoalBuilder;

impl PrologGoalBuilder {
    /// Build a clause with the given head.
    #[allow(dead_code)]
    pub fn to_clause(self, head: PrologTerm) -> PrologClause {
        PrologClause::rule(head, self.goals)
    }
}
