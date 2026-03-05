//! # TrueDecider - Trait Implementations
//!
//! This module contains trait implementations for `TrueDecider`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{TrueDecider, UserTacticResult};
use std::fmt;

impl UserTactic for TrueDecider {
    fn name(&self) -> &str {
        "decide_true"
    }
    fn run(&self, goal_target: &str, _hypotheses: &[(String, String)]) -> UserTacticResult {
        if goal_target.trim() == "True" {
            UserTacticResult::Solved
        } else {
            UserTacticResult::Failed("goal is not True".to_string())
        }
    }
    fn description(&self) -> &str {
        "Decides goals of the form True"
    }
}
