//! # TaskError - Trait Implementations
//!
//! This module contains trait implementations for `TaskError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskError;
use std::fmt;

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::TaskNotFound(id) => write!(f, "task not found: {}", id),
            TaskError::CyclicDependency(cycle) => {
                write!(f, "cyclic dependency: {:?}", cycle)
            }
            TaskError::InvalidStateTransition { from, to } => {
                write!(f, "invalid state transition: {} -> {}", from, to)
            }
            TaskError::DependencyFailed(id) => write!(f, "dependency failed: {}", id),
            TaskError::ExecutionFailed { task_id, reason } => {
                write!(f, "execution failed for {}: {}", task_id, reason)
            }
        }
    }
}
