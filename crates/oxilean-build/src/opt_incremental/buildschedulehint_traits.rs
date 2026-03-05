//! # BuildScheduleHint - Trait Implementations
//!
//! This module contains trait implementations for `BuildScheduleHint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildScheduleHint;

impl std::fmt::Display for BuildScheduleHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildScheduleHint::Skip => write!(f, "skip"),
            BuildScheduleHint::Rebuild => write!(f, "rebuild"),
            BuildScheduleHint::RelinkOnly => write!(f, "relink-only"),
        }
    }
}
