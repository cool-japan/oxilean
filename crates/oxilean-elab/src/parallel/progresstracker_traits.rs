//! # ProgressTracker - Trait Implementations
//!
//! This module contains trait implementations for `ProgressTracker`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProgressTracker;
use std::fmt;

impl fmt::Display for ProgressTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_progress())
    }
}
