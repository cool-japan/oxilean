//! # FunctionEntry - Trait Implementations
//!
//! This module contains trait implementations for `FunctionEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FunctionEntry;
use std::fmt;

impl fmt::Display for FunctionEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (arity={}, convention={}, recursive={})",
            self.name, self.arity, self.convention, self.is_recursive
        )
    }
}
