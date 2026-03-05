//! # IdrisPattern - Trait Implementations
//!
//! This module contains trait implementations for `IdrisPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisPattern;
use std::fmt;

impl fmt::Display for IdrisPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisPattern::Wildcard => write!(f, "_"),
            IdrisPattern::Var(x) => write!(f, "{}", x),
            IdrisPattern::Lit(l) => write!(f, "{}", l),
            IdrisPattern::Nil => write!(f, "[]"),
            IdrisPattern::Tuple(ps) => {
                write!(f, "(")?;
                for (i, p) in ps.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            IdrisPattern::Cons(h, t) => write!(f, "{} :: {}", h, t),
            IdrisPattern::As(n, p) => write!(f, "{}@{}", n, p),
            IdrisPattern::Implicit(p) => write!(f, "{{{}}}", p),
            IdrisPattern::Dot(e) => write!(f, ".{}", e),
            IdrisPattern::Con(c, args) => {
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
        }
    }
}
