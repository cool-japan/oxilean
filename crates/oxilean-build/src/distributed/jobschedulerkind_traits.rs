//! # JobSchedulerKind - Trait Implementations
//!
//! This module contains trait implementations for `JobSchedulerKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::JobSchedulerKind;

impl std::fmt::Display for JobSchedulerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobSchedulerKind::LeastLoaded => write!(f, "least-loaded"),
            JobSchedulerKind::RoundRobin => write!(f, "round-robin"),
            JobSchedulerKind::PriorityWeighted => write!(f, "priority-weighted"),
        }
    }
}
