//! # ScalaPattern - Trait Implementations
//!
//! This module contains trait implementations for `ScalaPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaPattern;
use std::fmt;

impl fmt::Display for ScalaPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaPattern::Wildcard => write!(f, "_"),
            ScalaPattern::Var(v) => write!(f, "{}", v),
            ScalaPattern::Lit(lit) => write!(f, "{}", lit),
            ScalaPattern::Typed(name, ty) => write!(f, "{}: {}", name, ty),
            ScalaPattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            ScalaPattern::Extractor(name, args) => {
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "(")?;
                    for (i, a) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", a)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            ScalaPattern::Alt(pats) => {
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
        }
    }
}
