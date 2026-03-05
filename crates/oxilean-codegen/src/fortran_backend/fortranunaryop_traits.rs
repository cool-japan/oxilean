//! # FortranUnaryOp - Trait Implementations
//!
//! This module contains trait implementations for `FortranUnaryOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranUnaryOp;
use std::fmt;

impl fmt::Display for FortranUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FortranUnaryOp::Neg => write!(f, "-"),
            FortranUnaryOp::Not => write!(f, ".NOT. "),
            FortranUnaryOp::Pos => write!(f, "+"),
        }
    }
}
