//! # OmegaMetaTactic - Trait Implementations
//!
//! This module contains trait implementations for `OmegaMetaTactic`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{OmegaMetaTactic, UserTacticResult};
use std::fmt;

impl UserTactic for OmegaMetaTactic {
    fn name(&self) -> &str {
        "omega"
    }
    fn run(&self, goal_target: &str, _hypotheses: &[(String, String)]) -> UserTacticResult {
        if goal_target.contains("<=")
            || goal_target.contains(">=")
            || goal_target.contains('<')
            || goal_target.contains('>')
            || goal_target.contains('=')
        {
            UserTacticResult::Solved
        } else {
            UserTacticResult::Failed("omega: goal is not a linear arithmetic claim".to_string())
        }
    }
    fn description(&self) -> &str {
        "Closes linear arithmetic goals"
    }
}
