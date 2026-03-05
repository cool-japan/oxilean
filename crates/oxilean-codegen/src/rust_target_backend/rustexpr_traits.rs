//! # RustExpr - Trait Implementations
//!
//! This module contains trait implementations for `RustExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::RustExpr;
use std::fmt;

impl fmt::Display for RustExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", emit_expr(self, 0))
    }
}
