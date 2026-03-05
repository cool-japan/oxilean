//! # SimpMetaTactic - Trait Implementations
//!
//! This module contains trait implementations for `SimpMetaTactic`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{SimpMetaTactic, UserTacticResult};
use std::fmt;

impl UserTactic for SimpMetaTactic {
    fn name(&self) -> &str {
        "simp"
    }
    fn run(&self, goal_target: &str, _hypotheses: &[(String, String)]) -> UserTacticResult {
        if goal_target.trim() == "True" {
            return UserTacticResult::Solved;
        }
        if let Some((lhs, rhs)) = goal_target.split_once('=') {
            if lhs.trim() == rhs.trim() {
                return UserTacticResult::Solved;
            }
        }
        UserTacticResult::Failed("simp: could not simplify".to_string())
    }
    fn description(&self) -> &str {
        "Simplification tactic"
    }
}
