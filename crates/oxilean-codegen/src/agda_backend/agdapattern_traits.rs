//! # AgdaPattern - Trait Implementations
//!
//! This module contains trait implementations for `AgdaPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AgdaPattern;
use std::fmt;

impl fmt::Display for AgdaPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgdaPattern::Var(x) => write!(f, "{}", x),
            AgdaPattern::Con(c, args) => {
                if args.is_empty() {
                    write!(f, "{}", c)
                } else {
                    write!(f, "({}", c)?;
                    for a in args {
                        write!(f, " {}", a)?;
                    }
                    write!(f, ")")
                }
            }
            AgdaPattern::Wildcard => write!(f, "_"),
            AgdaPattern::Num(n) => write!(f, "{}", n),
            AgdaPattern::Dot(p) => write!(f, ".{}", p),
            AgdaPattern::Absurd => write!(f, "()"),
            AgdaPattern::As(x, p) => write!(f, "{}@{}", x, p),
            AgdaPattern::Implicit(p) => write!(f, "{{{}}}", p),
        }
    }
}
