//! # FalseEliminator - Trait Implementations
//!
//! This module contains trait implementations for `FalseEliminator`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{FalseEliminator, UserTacticResult};
use std::fmt;

impl UserTactic for FalseEliminator {
    fn name(&self) -> &str {
        "false_elim"
    }
    fn run(&self, _goal_target: &str, hypotheses: &[(String, String)]) -> UserTacticResult {
        let has_false = hypotheses.iter().any(|(_, ty)| ty.trim() == "False");
        if has_false {
            UserTacticResult::Solved
        } else {
            UserTacticResult::Failed("no False hypothesis found".to_string())
        }
    }
    fn description(&self) -> &str {
        "Closes any goal when a False hypothesis is present"
    }
}
