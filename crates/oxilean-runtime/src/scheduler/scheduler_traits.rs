//! # Scheduler - Trait Implementations
//!
//! This module contains trait implementations for `Scheduler`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Scheduler, SchedulerConfig};
use std::fmt;

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler::new(SchedulerConfig::default())
    }
}

impl fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scheduler")
            .field("num_workers", &self.workers.len())
            .field("active_tasks", &self.active_task_count())
            .field("completed", &self.completed.len())
            .field("global_queue", &self.global_queue.len())
            .finish()
    }
}
