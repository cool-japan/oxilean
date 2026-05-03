//! # BuildLogEntry - Trait Implementations
//!
//! This module contains trait implementations for `BuildLogEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildLogEntry;
use std::fmt;

impl std::fmt::Display for BuildLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(t) = &self.target {
            write!(f, "[{}][{}] {}", self.level, t, self.message)
        } else {
            write!(f, "[{}] {}", self.level, self.message)
        }
    }
}
