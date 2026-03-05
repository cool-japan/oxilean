//! # ReplayError - Trait Implementations
//!
//! This module contains trait implementations for `ReplayError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReplayError;
use std::fmt;

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplayError::InvalidStructure(msg) => write!(f, "Invalid structure: {}", msg),
            ReplayError::TacticFailed(msg) => write!(f, "Tactic failed: {}", msg),
            ReplayError::UnknownHypothesis(msg) => {
                write!(f, "Unknown hypothesis: {}", msg)
            }
            ReplayError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            ReplayError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            ReplayError::InvalidProofState(msg) => {
                write!(f, "Invalid proof state: {}", msg)
            }
            ReplayError::GoalMismatch(msg) => write!(f, "Goal mismatch: {}", msg),
        }
    }
}

impl std::error::Error for ReplayError {}
