//! # WorkerStats - Trait Implementations
//!
//! This module contains trait implementations for `WorkerStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Worker, WorkerStats};
use std::fmt;

impl fmt::Display for WorkerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Worker {} (completed={}, stolen={}, queue={}, idle={})",
            self.id, self.tasks_completed, self.tasks_stolen, self.queue_length, self.idle
        )
    }
}
