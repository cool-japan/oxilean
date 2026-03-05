//! # LinFormula - Trait Implementations
//!
//! This module contains trait implementations for `LinFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinFormula;
use std::fmt;

impl std::fmt::Display for LinFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinFormula::Atom(s) => write!(f, "{}", s),
            LinFormula::One => write!(f, "1"),
            LinFormula::Bot => write!(f, "⊥"),
            LinFormula::Top => write!(f, "⊤"),
            LinFormula::Zero => write!(f, "0"),
            LinFormula::Neg(a) => write!(f, "{}^⊥", a),
            LinFormula::Bang(a) => write!(f, "!{}", a),
            LinFormula::WhyNot(a) => write!(f, "?{}", a),
            LinFormula::Tensor(a, b) => write!(f, "({} ⊗ {})", a, b),
            LinFormula::Par(a, b) => write!(f, "({} ⅋ {})", a, b),
            LinFormula::With(a, b) => write!(f, "({} & {})", a, b),
            LinFormula::Plus(a, b) => write!(f, "({} ⊕ {})", a, b),
        }
    }
}
