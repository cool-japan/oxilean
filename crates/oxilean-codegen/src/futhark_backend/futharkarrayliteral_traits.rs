//! # FutharkArrayLiteral - Trait Implementations
//!
//! This module contains trait implementations for `FutharkArrayLiteral`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkArrayLiteral;
use std::fmt;

impl std::fmt::Display for FutharkArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elems: Vec<&str> = self.elements.iter().map(|s| s.as_str()).collect();
        write!(f, "[{}]", elems.join(", "))
    }
}
