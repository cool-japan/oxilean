//! # CoqModuleDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqModuleDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqModuleDef;
use std::fmt;

impl std::fmt::Display for CoqModuleDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kw, end) = if self.is_section {
            ("Section", "End")
        } else {
            ("Module", "End")
        };
        writeln!(f, "{} {}.", kw, self.name)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        write!(f, "{} {}.", end, self.name)
    }
}
