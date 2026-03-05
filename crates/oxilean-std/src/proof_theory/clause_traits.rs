//! # Clause - Trait Implementations
//!
//! This module contains trait implementations for `Clause`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Clause;
use std::fmt;

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "□")
        } else {
            let lits: Vec<String> = self
                .0
                .iter()
                .map(|&l| {
                    if l > 0 {
                        format!("x{}", l)
                    } else {
                        format!("¬x{}", -l)
                    }
                })
                .collect();
            write!(f, "{{{}}}", lits.join(", "))
        }
    }
}
