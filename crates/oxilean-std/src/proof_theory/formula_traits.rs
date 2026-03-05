//! # Formula - Trait Implementations
//!
//! This module contains trait implementations for `Formula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Formula;
use std::fmt;

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formula::Atom(s) => write!(f, "{}", s),
            Formula::True_ => write!(f, "⊤"),
            Formula::False_ => write!(f, "⊥"),
            Formula::Neg(inner) => write!(f, "¬{}", inner),
            Formula::And(a, b) => write!(f, "({} ∧ {})", a, b),
            Formula::Or(a, b) => write!(f, "({} ∨ {})", a, b),
            Formula::Implies(a, b) => write!(f, "({} → {})", a, b),
            Formula::Iff(a, b) => write!(f, "({} ↔ {})", a, b),
        }
    }
}
