//! # JsStmt - Trait Implementations
//!
//! This module contains trait implementations for `JsStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::JsStmt;
use std::fmt;

impl fmt::Display for JsStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_stmt_indented(self, 0))
    }
}
