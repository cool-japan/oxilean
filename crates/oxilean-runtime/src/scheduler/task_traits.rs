//! # Task - Trait Implementations
//!
//! This module contains trait implementations for `Task`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Task;
use std::fmt;

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name.as_deref().unwrap_or("anonymous");
        write!(
            f,
            "Task({}, name={}, priority={}, state={})",
            self.id, name, self.priority, self.state
        )
    }
}
