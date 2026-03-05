//! # IncrementalBuildProgress - Trait Implementations
//!
//! This module contains trait implementations for `IncrementalBuildProgress`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IncrementalBuildProgress;

impl std::fmt::Display for IncrementalBuildProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({:.1}%)", self.completed, self.total, self.pct())
    }
}
