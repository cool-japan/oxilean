//! # PerfSeverity - Trait Implementations
//!
//! This module contains trait implementations for `PerfSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PerfSeverity;

impl std::fmt::Display for PerfSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerfSeverity::Blocker => write!(f, "blocker"),
            PerfSeverity::Warning => write!(f, "warning"),
            PerfSeverity::Suggestion => write!(f, "suggestion"),
        }
    }
}

