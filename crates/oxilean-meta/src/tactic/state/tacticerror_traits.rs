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

impl std::fmt::Display for TacticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticError::NoGoals => write!(f, "no goals to prove"),
            TacticError::Failed(msg) => write!(f, "tactic failed: {}", msg),
            TacticError::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {:?}, got {:?}", expected, got)
            }
            TacticError::UnknownHyp(name) => write!(f, "unknown hypothesis: {}", name),
            TacticError::GoalMismatch(msg) => write!(f, "goal mismatch: {}", msg),
            TacticError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}
