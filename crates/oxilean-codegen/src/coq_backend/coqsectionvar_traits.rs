//! # CoqSectionVar - Trait Implementations
//!
//! This module contains trait implementations for `CoqSectionVar`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqSectionVar;
use std::fmt;

impl std::fmt::Display for CoqSectionVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable {} : {}.", self.names.join(" "), self.var_type)
    }
}
