//! # SchedulerEvent - Trait Implementations
//!
//! This module contains trait implementations for `SchedulerEvent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SchedulerEvent;

impl std::fmt::Display for SchedulerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TaskSubmitted { item_id, name } => {
                write!(f, "[SUBMIT] task {} ({})", item_id, name)
            }
            Self::TaskStarted { item_id, worker_id, name } => {
                write!(f, "[START] task {} ({}) on worker {}", item_id, name, worker_id)
            }
            Self::TaskSucceeded { item_id, worker_id, elapsed_ms } => {
                write!(
                    f, "[DONE] task {} on worker {} in {}ms", item_id, worker_id,
                    elapsed_ms
                )
            }
            Self::TaskFailed { item_id, exit_code, elapsed_ms } => {
                write!(
                    f, "[FAIL] task {} exit={} after {}ms", item_id, exit_code,
                    elapsed_ms
                )
            }
            Self::TaskCancelled { item_id } => write!(f, "[CANCEL] task {}", item_id),
            Self::TaskTimedOut { item_id, timeout_ms } => {
                write!(f, "[TIMEOUT] task {} after {}ms", item_id, timeout_ms)
            }
            Self::AllDone { total_ms } => write!(f, "[ALL_DONE] in {}ms", total_ms),
        }
    }
}

