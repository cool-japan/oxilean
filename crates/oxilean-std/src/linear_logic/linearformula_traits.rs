//! # LinearFormula - Trait Implementations
//!
//! This module contains trait implementations for `LinearFormula`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearFormula;
use std::fmt;

impl std::fmt::Display for LinearFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinearFormula::Atom(s) => write!(f, "{s}"),
            LinearFormula::Tensor(a, b) => write!(f, "({a} ⊗ {b})"),
            LinearFormula::Par(a, b) => write!(f, "({a} ⅋ {b})"),
            LinearFormula::With(a, b) => write!(f, "({a} & {b})"),
            LinearFormula::Plus(a, b) => write!(f, "({a} ⊕ {b})"),
            LinearFormula::OfCourse(a) => write!(f, "!{a}"),
            LinearFormula::WhyNot(a) => write!(f, "?{a}"),
            LinearFormula::One => write!(f, "1"),
            LinearFormula::Bottom => write!(f, "⊥"),
            LinearFormula::Top => write!(f, "⊤"),
            LinearFormula::Zero => write!(f, "0"),
            LinearFormula::Dual(a) => write!(f, "{a}^⊥"),
        }
    }
}
