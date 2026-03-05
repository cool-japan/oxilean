//! # TaskStatus - Trait Implementations
//!
//! This module contains trait implementations for `TaskStatus`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskStatus;

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Ready => write!(f, "ready"),
            Self::Running { worker_id } => write!(f, "running(worker={})", worker_id),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Failed { exit_code } => write!(f, "failed(exit={})", exit_code),
            Self::Cancelled => write!(f, "cancelled"),
            Self::TimedOut => write!(f, "timed-out"),
            Self::Skipped => write!(f, "skipped"),
        }
    }
}

