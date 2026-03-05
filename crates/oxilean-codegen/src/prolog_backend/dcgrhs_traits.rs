//! # DcgRhs - Trait Implementations
//!
//! This module contains trait implementations for `DcgRhs`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::fmt_dcg_seq;
use super::types::DcgRhs;
use std::fmt;

impl fmt::Display for DcgRhs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DcgRhs::NonTerminal(t) => write!(f, "{}", t),
            DcgRhs::Terminals(ts) => {
                write!(f, "[")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "]")
            }
            DcgRhs::Epsilon => write!(f, "[]"),
            DcgRhs::Goal(goals) => {
                write!(f, "{{")?;
                for (i, g) in goals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", g)?;
                }
                write!(f, "}}")
            }
            DcgRhs::Disjunction(a, b) => {
                write!(f, "(")?;
                fmt_dcg_seq(f, a)?;
                write!(f, " ; ")?;
                fmt_dcg_seq(f, b)?;
                write!(f, ")")
            }
            DcgRhs::Seq(parts) => fmt_dcg_seq(f, parts),
        }
    }
}
