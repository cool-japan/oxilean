//! # MutualElabError - Trait Implementations
//!
//! This module contains trait implementations for `MutualElabError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MutualElabError;
use std::fmt;

impl std::fmt::Display for MutualElabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutualElabError::TypeMismatch(msg) => write!(f, "type mismatch: {}", msg),
            MutualElabError::InvalidRecursion(msg) => {
                write!(f, "invalid recursion: {}", msg)
            }
            MutualElabError::MissingDefinition(msg) => {
                write!(f, "missing definition: {}", msg)
            }
            MutualElabError::CyclicType(msg) => write!(f, "cyclic type: {}", msg),
            MutualElabError::TerminationFailure(msg) => {
                write!(f, "termination failure: {}", msg)
            }
            MutualElabError::Other(msg) => write!(f, "mutual elaboration error: {}", msg),
        }
    }
}
