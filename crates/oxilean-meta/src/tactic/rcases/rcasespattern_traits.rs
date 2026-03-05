//! # RcasesPattern - Trait Implementations
//!
//! This module contains trait implementations for `RcasesPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RcasesPattern;
use std::fmt;

impl fmt::Display for RcasesPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcasesPattern::One(name) => write!(f, "{}", name),
            RcasesPattern::Clear => write!(f, "_"),
            RcasesPattern::Tuple(pats) => {
                write!(f, "\u{27E8}")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "\u{27E9}")
            }
            RcasesPattern::Alts(pats) => {
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
            RcasesPattern::Typed(p, _ty) => write!(f, "({} : <type>)", p),
            RcasesPattern::Nested(p) => write!(f, "\u{27E8}{}\u{27E9}", p),
        }
    }
}
