//! # TerminationError - Trait Implementations
//!
//! This module contains trait implementations for `TerminationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TerminationError;
use std::fmt;

impl fmt::Display for TerminationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TerminationError::NoDecreasingArg(name) => {
                write!(
                    f,
                    "failed to prove termination for '{}': no structurally decreasing argument found",
                    name
                )
            }
            TerminationError::CallNotDecreasing {
                caller,
                callee,
                reason,
            } => {
                write!(
                    f,
                    "recursive call from '{}' to '{}' does not decrease: {}",
                    caller, callee, reason
                )
            }
            TerminationError::WellFoundedFailure(msg) => {
                write!(f, "well-founded recursion check failed: {}", msg)
            }
            TerminationError::InvalidRelation(msg) => {
                write!(f, "invalid well-founded relation: {}", msg)
            }
            TerminationError::MutualNoDecrease(names) => {
                write!(f, "mutual recursion group has no decreasing argument: ")?;
                for (i, name) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", name)?;
                }
                Ok(())
            }
            TerminationError::EmptyMutualGroup => {
                write!(f, "empty mutual recursion group")
            }
            TerminationError::NestedRecursion(msg) => {
                write!(f, "nested recursion not supported: {}", msg)
            }
            TerminationError::NonInductiveRecursion(msg) => {
                write!(f, "recursion through non-inductive type: {}", msg)
            }
            TerminationError::MaxDepthExceeded(depth) => {
                write!(f, "maximum analysis depth exceeded: {}", depth)
            }
            TerminationError::UnsupportedPattern(msg) => {
                write!(f, "unsupported recursion pattern: {}", msg)
            }
            TerminationError::InternalError(msg) => {
                write!(f, "internal error in termination checker: {}", msg)
            }
        }
    }
}
