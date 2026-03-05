//! # CTLFormula - Trait Implementations
//!
//! This module contains trait implementations for `CTLFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CTLFormula;
use std::fmt;

impl std::fmt::Display for CTLFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CTLFormula::True => write!(f, "⊤"),
            CTLFormula::False => write!(f, "⊥"),
            CTLFormula::Atom(s) => write!(f, "{s}"),
            CTLFormula::Not(p) => write!(f, "¬{p}"),
            CTLFormula::And(p, q) => write!(f, "({p} ∧ {q})"),
            CTLFormula::Or(p, q) => write!(f, "({p} ∨ {q})"),
            CTLFormula::EX(p) => write!(f, "EX{p}"),
            CTLFormula::AX(p) => write!(f, "AX{p}"),
            CTLFormula::EF(p) => write!(f, "EF{p}"),
            CTLFormula::AF(p) => write!(f, "AF{p}"),
            CTLFormula::EG(p) => write!(f, "EG{p}"),
            CTLFormula::AG(p) => write!(f, "AG{p}"),
            CTLFormula::EU(p, q) => write!(f, "E({p} U {q})"),
            CTLFormula::AU(p, q) => write!(f, "A({p} U {q})"),
        }
    }
}
