//! # TaskGraph - Trait Implementations
//!
//! This module contains trait implementations for `TaskGraph`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskGraph;
use std::fmt;

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TaskGraph with {} tasks", self.tasks.len())?;
        for task in self.tasks.values() {
            writeln!(f, "  {}", task)?;
        }
        Ok(())
    }
}
