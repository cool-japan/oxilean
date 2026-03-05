//! # RustPattern - Trait Implementations
//!
//! This module contains trait implementations for `RustPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RustPattern;
use std::fmt;

impl fmt::Display for RustPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustPattern::Wildcard => write!(f, "_"),
            RustPattern::Var(name, mutable) => {
                if *mutable {
                    write!(f, "mut {}", name)
                } else {
                    write!(f, "{}", name)
                }
            }
            RustPattern::Lit(lit) => write!(f, "{}", lit),
            RustPattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                if pats.len() == 1 {
                    write!(f, ",")?;
                }
                write!(f, ")")
            }
            RustPattern::Struct(name, fields) => {
                write!(f, "{} {{", name)?;
                for (i, (fname, fpat)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", fname, fpat)?;
                }
                write!(f, "}}")
            }
            RustPattern::Enum(name, pats) => {
                if pats.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}(", name)?;
                    for (i, p) in pats.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ")")
                }
            }
            RustPattern::Ref(mutable, inner) => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            RustPattern::Or(pats) => {
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
            RustPattern::Range(lo, hi) => write!(f, "{}..={}", lo, hi),
            RustPattern::Guard(pat, cond) => write!(f, "{} if {}", pat, cond),
        }
    }
}
