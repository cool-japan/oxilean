//! # TimelineEventState - Trait Implementations
//!
//! This module contains trait implementations for `TimelineEventState`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TimelineEventState;

impl std::fmt::Display for TimelineEventState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineEventState::Pending => write!(f, "pending"),
            TimelineEventState::Running => write!(f, "running"),
            TimelineEventState::Done => write!(f, "done"),
            TimelineEventState::Failed => write!(f, "failed"),
            TimelineEventState::Skipped => write!(f, "skipped"),
        }
    }
}
