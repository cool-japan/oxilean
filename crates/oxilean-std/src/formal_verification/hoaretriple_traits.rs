//! # HoareTriple - Trait Implementations
//!
//! This module contains trait implementations for `HoareTriple`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HoareTriple;
use std::fmt;

impl std::fmt::Display for HoareTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}\n  {}\n{{{}}}", self.pre, self.program, self.post)
    }
}
