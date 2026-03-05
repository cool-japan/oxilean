//! # YulStmt - Trait Implementations
//!
//! This module contains trait implementations for `YulStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::YulStmt;
use std::fmt;

impl std::fmt::Display for YulStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YulStmt::Let(vars, Some(val)) => {
                write!(f, "let {} := {}", vars.join(", "), val)
            }
            YulStmt::Let(vars, None) => write!(f, "let {}", vars.join(", ")),
            YulStmt::Assign(vars, val) => write!(f, "{} := {}", vars.join(", "), val),
            YulStmt::Break => write!(f, "break"),
            YulStmt::Continue => write!(f, "continue"),
            YulStmt::Leave => write!(f, "leave"),
            YulStmt::Return(p, s) => write!(f, "return({}, {})", p, s),
            YulStmt::Revert(p, s) => write!(f, "revert({}, {})", p, s),
            YulStmt::Pop(e) => write!(f, "pop({})", e),
            YulStmt::Expr(e) => write!(f, "{}", e),
            _ => write!(f, "/* yul */"),
        }
    }
}
