//! # GCongrRelation - Trait Implementations
//!
//! This module contains trait implementations for `GCongrRelation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GCongrRelation;
use std::fmt;

impl fmt::Display for GCongrRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
