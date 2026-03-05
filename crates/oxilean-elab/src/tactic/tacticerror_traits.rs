//! # TacticError - Trait Implementations
//!
//! This module contains trait implementations for `TacticError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacticError;
use std::fmt;

impl fmt::Display for TacticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TacticError::GoalNotFound(name) => write!(f, "goal '{}' not found", name),
            TacticError::NoGoals => write!(f, "no goals to solve"),
            TacticError::TooManyGoals => write!(f, "too many goals"),
            TacticError::TypeMismatch(msg) => write!(f, "type mismatch: {}", msg),
            TacticError::UnknownTactic(name) => write!(f, "unknown tactic: {}", name),
            TacticError::InvalidArg(msg) => write!(f, "invalid argument: {}", msg),
            TacticError::InternalError(msg) => {
                write!(f, "internal tactic error: {}", msg)
            }
        }
    }
}
