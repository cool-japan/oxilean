//! # PassDependency - Trait Implementations
//!
//! This module contains trait implementations for `PassDependency`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PassDependency;
use std::fmt;

impl fmt::Display for PassDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.depends_on, self.pass)
    }
}
