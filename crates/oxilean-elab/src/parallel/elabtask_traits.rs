//! # ElabTask - Trait Implementations
//!
//! This module contains trait implementations for `ElabTask`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabTask;
use std::fmt;

impl fmt::Display for ElabTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task(id={}, name={}, decl={}, deps={:?}, status={}, priority={})",
            self.id, self.name, self.decl_name, self.deps, self.status, self.priority
        )
    }
}
