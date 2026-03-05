//! # LtlFormula - Trait Implementations
//!
//! This module contains trait implementations for `LtlFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LtlFormula;
use std::fmt;

impl std::fmt::Display for LtlFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtlFormula::Atom(s) => write!(f, "{}", s),
            LtlFormula::True_ => write!(f, "⊤"),
            LtlFormula::False_ => write!(f, "⊥"),
            LtlFormula::Not(p) => write!(f, "¬{}", p),
            LtlFormula::And(a, b) => write!(f, "({} ∧ {})", a, b),
            LtlFormula::Or(a, b) => write!(f, "({} ∨ {})", a, b),
            LtlFormula::Next(p) => write!(f, "X{}", p),
            LtlFormula::Until(a, b) => write!(f, "({} U {})", a, b),
            LtlFormula::Release(a, b) => write!(f, "({} R {})", a, b),
            LtlFormula::Eventually(p) => write!(f, "F{}", p),
            LtlFormula::Always(p) => write!(f, "G{}", p),
            LtlFormula::WeakUntil(a, b) => write!(f, "({} W {})", a, b),
        }
    }
}
