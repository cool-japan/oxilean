//! # MilnorSymbol - Trait Implementations
//!
//! This module contains trait implementations for `MilnorSymbol`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MilnorSymbol;
use std::fmt;

impl std::fmt::Display for MilnorSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.factors.is_empty() {
            write!(f, "{{1}}")
        } else {
            let parts: Vec<String> = self.factors.iter().map(|x| x.to_string()).collect();
            write!(f, "{{{}}}", parts.join(", "))
        }
    }
}
