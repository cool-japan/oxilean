//! # RustStmt - Trait Implementations
//!
//! This module contains trait implementations for `RustStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::RustStmt;
use std::fmt;

impl fmt::Display for RustStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", emit_stmt(self, 0))
    }
}
