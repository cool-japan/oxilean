//! # IdrisDoStmt - Trait Implementations
//!
//! This module contains trait implementations for `IdrisDoStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisDoStmt;
use std::fmt;

impl fmt::Display for IdrisDoStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisDoStmt::Bind(x, e) => write!(f, "{} <- {}", x, e),
            IdrisDoStmt::Let(x, e) => write!(f, "let {} = {}", x, e),
            IdrisDoStmt::LetTyped(x, t, e) => write!(f, "let {} : {} = {}", x, t, e),
            IdrisDoStmt::Expr(e) => write!(f, "{}", e),
            IdrisDoStmt::Ignore(e) => write!(f, "ignore {}", e),
        }
    }
}
