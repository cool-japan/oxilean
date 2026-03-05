//! # PrologGoalBuilder - new_group Methods
//!
//! This module contains method implementations for `PrologGoalBuilder`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::prologgoalbuilder_type::PrologGoalBuilder;
use super::types::PrologTerm;

impl PrologGoalBuilder {
    /// Create a new empty goal builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        PrologGoalBuilder { goals: vec![] }
    }
    /// Add `=..` (univ): `Term =.. List`.
    #[allow(dead_code)]
    pub fn univ(self, term: PrologTerm, list: PrologTerm) -> Self {
        self.goal(PrologTerm::Op(
            "=..".to_string(),
            Box::new(term),
            Box::new(list),
        ))
    }
}
