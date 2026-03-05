//! # TaskPriority - Trait Implementations
//!
//! This module contains trait implementations for `TaskPriority`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TaskPriority;
use std::fmt;

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::Low => f.write_str("low"),
            TaskPriority::Normal => f.write_str("normal"),
            TaskPriority::High => f.write_str("high"),
            TaskPriority::Urgent => f.write_str("urgent"),
        }
    }
}
