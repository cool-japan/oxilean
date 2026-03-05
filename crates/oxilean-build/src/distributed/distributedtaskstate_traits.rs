//! # DistributedTaskState - Trait Implementations
//!
//! This module contains trait implementations for `DistributedTaskState`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DistributedTaskState;

impl std::fmt::Display for DistributedTaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributedTaskState::Pending => write!(f, "pending"),
            DistributedTaskState::Running => write!(f, "running"),
            DistributedTaskState::Done => write!(f, "done"),
            DistributedTaskState::Failed => write!(f, "failed"),
            DistributedTaskState::Cancelled => write!(f, "cancelled"),
        }
    }
}
