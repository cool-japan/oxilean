//! # DivisorClass - Trait Implementations
//!
//! This module contains trait implementations for `DivisorClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DivisorClass;
use std::fmt;

impl std::fmt::Display for DivisorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Div({}, deg={}, g={})",
            self.label, self.degree, self.genus
        )
    }
}
