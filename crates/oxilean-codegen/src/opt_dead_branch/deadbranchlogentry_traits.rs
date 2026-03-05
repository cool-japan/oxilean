//! # DeadBranchLogEntry - Trait Implementations
//!
//! This module contains trait implementations for `DeadBranchLogEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeadBranchLogEntry;
use std::fmt;

impl std::fmt::Display for DeadBranchLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.function, self.kind, self.detail)
    }
}
