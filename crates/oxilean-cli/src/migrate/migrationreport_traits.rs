//! # MigrationReport - Trait Implementations
//!
//! This module contains trait implementations for `MigrationReport`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MigrationReport;
use std::fmt;

impl Default for MigrationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MigrationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}
