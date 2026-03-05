//! # DistributedTask - Trait Implementations
//!
//! This module contains trait implementations for `DistributedTask`.
//!
//! ## Implemented Traits
//!
//! - `PartialEq`
//! - `Eq`
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DistributedTask;

impl PartialEq for DistributedTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DistributedTask {}

impl PartialOrd for DistributedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistributedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
