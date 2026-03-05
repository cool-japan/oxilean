//! # NotationPart - Trait Implementations
//!
//! This module contains trait implementations for `NotationPart`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationPart;

impl std::fmt::Display for NotationPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationPart::Literal(s) => write!(f, "\"{}\"", s),
            NotationPart::Placeholder(s) => write!(f, "{}", s),
            NotationPart::Prec(p) => write!(f, ":{}", p),
        }
    }
}
