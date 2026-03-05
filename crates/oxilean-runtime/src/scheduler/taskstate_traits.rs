//! # TaskState - Trait Implementations
//!
//! This module contains trait implementations for `TaskState`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskState;
use std::fmt;

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskState::Created => write!(f, "created"),
            TaskState::Queued => write!(f, "queued"),
            TaskState::Running { worker_id } => {
                write!(f, "running(worker={})", worker_id)
            }
            TaskState::Suspended { waiting_on } => {
                write!(f, "suspended(waiting={})", waiting_on.len())
            }
            TaskState::Completed { .. } => write!(f, "completed"),
            TaskState::Failed { error } => write!(f, "failed({})", error),
            TaskState::Cancelled => write!(f, "cancelled"),
        }
    }
}
