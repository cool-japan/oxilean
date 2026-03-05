//! # KotlinFunc - Trait Implementations
//!
//! This module contains trait implementations for `KotlinFunc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinFunc;
use std::fmt;

impl fmt::Display for KotlinFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_tailrec {
            write!(f, "tailrec ")?;
        }
        write!(f, "fun {}(", self.name)?;
        for (i, (name, ty)) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, ty)?;
        }
        writeln!(f, "): {} {{", self.return_type)?;
        fmt_stmts(&self.body, "    ", f)?;
        writeln!(f, "}}")
    }
}
