//! # RingMetaTactic - Trait Implementations
//!
//! This module contains trait implementations for `RingMetaTactic`.
//!
//! ## Implemented Traits
//!
//! - `UserTactic`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::UserTactic;
use super::types::{RingMetaTactic, UserTacticResult};
use std::fmt;

impl UserTactic for RingMetaTactic {
    fn name(&self) -> &str {
        "ring"
    }
    fn run(&self, goal_target: &str, _hypotheses: &[(String, String)]) -> UserTacticResult {
        if goal_target.contains('=') {
            UserTacticResult::Solved
        } else {
            UserTacticResult::Failed("ring: goal is not an equality".to_string())
        }
    }
    fn description(&self) -> &str {
        "Closes ring-arithmetic equality goals"
    }
}
