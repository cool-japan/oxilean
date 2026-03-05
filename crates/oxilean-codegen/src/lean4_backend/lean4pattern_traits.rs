//! # Lean4Pattern - Trait Implementations
//!
//! This module contains trait implementations for `Lean4Pattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::Lean4Pattern;
use std::fmt;

impl fmt::Display for Lean4Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lean4Pattern::Wildcard => write!(f, "_"),
            Lean4Pattern::Var(name) => write!(f, "{}", name),
            Lean4Pattern::Ctor(name, args) => {
                write!(f, "{}", name)?;
                for arg in args {
                    write!(f, " {}", paren_pattern(arg))?;
                }
                Ok(())
            }
            Lean4Pattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            Lean4Pattern::Lit(s) => write!(f, "{}", s),
            Lean4Pattern::Or(a, b) => write!(f, "{} | {}", a, b),
            Lean4Pattern::Anonymous(pats) => {
                write!(f, "⟨")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "⟩")
            }
        }
    }
}
