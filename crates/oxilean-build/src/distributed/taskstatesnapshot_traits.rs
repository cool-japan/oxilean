//! # TaskStateSnapshot - Trait Implementations
//!
//! This module contains trait implementations for `TaskStateSnapshot`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskStateSnapshot;

impl std::fmt::Display for TaskStateSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pending={} running={} done={} failed={} cancelled={}",
            self.pending, self.running, self.done, self.failed, self.cancelled
        )
    }
}
