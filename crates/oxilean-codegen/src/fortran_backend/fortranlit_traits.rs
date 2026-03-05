//! # FortranLit - Trait Implementations
//!
//! This module contains trait implementations for `FortranLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranLit;
use std::fmt;

impl fmt::Display for FortranLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FortranLit::Int(n) => write!(f, "{}_8", n),
            FortranLit::Real(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{:.1}_8", v)
                } else {
                    write!(f, "{}_8", v)
                }
            }
            FortranLit::Logical(true) => write!(f, ".TRUE."),
            FortranLit::Logical(false) => write!(f, ".FALSE."),
            FortranLit::Char(s) => {
                write!(f, "'")?;
                for c in s.chars() {
                    if c == '\'' {
                        write!(f, "''")?;
                    } else {
                        write!(f, "{}", c)?;
                    }
                }
                write!(f, "'")
            }
        }
    }
}
