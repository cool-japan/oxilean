//! # CoqNotation - Trait Implementations
//!
//! This module contains trait implementations for `CoqNotation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqNotation;
use std::fmt;

impl std::fmt::Display for CoqNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Notation \"{}\" := ({})", self.pattern, self.body)?;
        let mut has_paren = false;
        if let Some(lvl) = self.level {
            write!(f, " (at level {}", lvl)?;
            has_paren = true;
            if let Some(assoc) = &self.assoc {
                write!(f, ", {} associativity", assoc)?;
            }
            if let Some(scope) = &self.scope {
                write!(f, ", {} scope", scope)?;
            }
            write!(f, ")")?;
        } else if let Some(scope) = &self.scope {
            write!(f, " : {}", scope)?;
        }
        let _ = has_paren;
        write!(f, ".")
    }
}
