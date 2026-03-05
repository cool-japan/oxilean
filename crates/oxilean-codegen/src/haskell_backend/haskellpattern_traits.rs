//! # HaskellPattern - Trait Implementations
//!
//! This module contains trait implementations for `HaskellPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::HaskellPattern;
use std::fmt;

impl fmt::Display for HaskellPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaskellPattern::Wildcard => write!(f, "_"),
            HaskellPattern::Var(v) => write!(f, "{}", v),
            HaskellPattern::Lit(lit) => write!(f, "{}", lit),
            HaskellPattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            HaskellPattern::List(pats) => {
                write!(f, "[")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
            HaskellPattern::Cons(head, tail) => write!(f, "({} : {})", head, tail),
            HaskellPattern::Constructor(name, args) => {
                write!(f, "{}", name)?;
                for a in args {
                    write!(f, " {}", paren_pattern(a))?;
                }
                Ok(())
            }
            HaskellPattern::As(name, pat) => write!(f, "{}@{}", name, paren_pattern(pat)),
            HaskellPattern::LazyPat(pat) => write!(f, "~{}", paren_pattern(pat)),
        }
    }
}
