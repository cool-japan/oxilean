//! # RelFormula - Trait Implementations
//!
//! This module contains trait implementations for `RelFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RelFormula;
use std::fmt;

impl std::fmt::Display for RelFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelFormula::Atom(s) => write!(f, "{}", s),
            RelFormula::Neg(a) => write!(f, "¬{}", a),
            RelFormula::And(a, b) => write!(f, "({} ∧ {})", a, b),
            RelFormula::Or(a, b) => write!(f, "({} ∨ {})", a, b),
            RelFormula::Implies(a, b) => write!(f, "({} → {})", a, b),
            RelFormula::Fusion(a, b) => write!(f, "({} ∘ {})", a, b),
        }
    }
}
