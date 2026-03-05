//! # SchedulingStrategy - Trait Implementations
//!
//! This module contains trait implementations for `SchedulingStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SchedulingStrategy;

impl std::fmt::Display for SchedulingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PriorityFirst => write!(f, "priority-first"),
            Self::ShortestJobFirst => write!(f, "shortest-job-first"),
            Self::LongestJobFirst => write!(f, "longest-job-first"),
            Self::CriticalPath => write!(f, "critical-path"),
            Self::RoundRobin => write!(f, "round-robin"),
        }
    }
}

