//! # GoStmt - Trait Implementations
//!
//! This module contains trait implementations for `GoStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::format_stmts;
use super::functions::*;
use super::types::GoStmt;
use std::fmt;

impl fmt::Display for GoStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_stmt(self, 0))
    }
}
