//! # LTLFormula - Trait Implementations
//!
//! This module contains trait implementations for `LTLFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LTLFormula;
use std::fmt;

impl std::fmt::Display for LTLFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LTLFormula::True => write!(f, "⊤"),
            LTLFormula::False => write!(f, "⊥"),
            LTLFormula::Atom(s) => write!(f, "{s}"),
            LTLFormula::Not(p) => write!(f, "¬{p}"),
            LTLFormula::And(p, q) => write!(f, "({p} ∧ {q})"),
            LTLFormula::Or(p, q) => write!(f, "({p} ∨ {q})"),
            LTLFormula::Next(p) => write!(f, "X{p}"),
            LTLFormula::Until(p, q) => write!(f, "({p} U {q})"),
            LTLFormula::Globally(p) => write!(f, "G{p}"),
            LTLFormula::Finally(p) => write!(f, "F{p}"),
        }
    }
}
