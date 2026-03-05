//! # RefinementType - Trait Implementations
//!
//! This module contains trait implementations for `RefinementType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RefinementType;
use std::fmt;

impl std::fmt::Display for RefinementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x : {} | {} }}", self.base_type, self.predicate)
    }
}
