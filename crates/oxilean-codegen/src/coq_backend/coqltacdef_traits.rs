//! # CoqLtacDef - Trait Implementations
//!
//! This module contains trait implementations for `CoqLtacDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqLtacDef;
use std::fmt;

impl std::fmt::Display for CoqLtacDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rec = if self.is_recursive { " Recursive" } else { "" };
        if self.params.is_empty() {
            write!(f, "Ltac{} {} := {}.", rec, self.name, self.body)
        } else {
            write!(
                f,
                "Ltac{} {} {} := {}.",
                rec,
                self.name,
                self.params.join(" "),
                self.body
            )
        }
    }
}
