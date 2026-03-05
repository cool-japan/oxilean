//! # TsStmt - Trait Implementations
//!
//! This module contains trait implementations for `TsStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsStmt;
use std::fmt;

impl fmt::Display for TsStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ts_stmt(self, 0))
    }
}
