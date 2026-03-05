//! # FortranBinOp - Trait Implementations
//!
//! This module contains trait implementations for `FortranBinOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranBinOp;
use std::fmt;

impl fmt::Display for FortranBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FortranBinOp::Add => "+",
            FortranBinOp::Sub => "-",
            FortranBinOp::Mul => "*",
            FortranBinOp::Div => "/",
            FortranBinOp::Pow => "**",
            FortranBinOp::Concat => "//",
            FortranBinOp::Eq => "==",
            FortranBinOp::Ne => "/=",
            FortranBinOp::Lt => "<",
            FortranBinOp::Le => "<=",
            FortranBinOp::Gt => ">",
            FortranBinOp::Ge => ">=",
            FortranBinOp::And => ".AND.",
            FortranBinOp::Or => ".OR.",
            FortranBinOp::Eqv => ".EQV.",
            FortranBinOp::Neqv => ".NEQV.",
        };
        write!(f, "{}", s)
    }
}
