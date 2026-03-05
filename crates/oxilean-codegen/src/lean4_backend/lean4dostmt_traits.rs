//! # Lean4DoStmt - Trait Implementations
//!
//! This module contains trait implementations for `Lean4DoStmt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Lean4DoStmt;
use std::fmt;

impl fmt::Display for Lean4DoStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lean4DoStmt::Bind(name, ty, expr) => {
                if let Some(t) = ty {
                    write!(f, "let ({} : {}) ← {}", name, t, expr)
                } else {
                    write!(f, "let {} ← {}", name, expr)
                }
            }
            Lean4DoStmt::LetBind(name, ty, expr) => {
                if let Some(t) = ty {
                    write!(f, "let {} : {} := {}", name, t, expr)
                } else {
                    write!(f, "let {} := {}", name, expr)
                }
            }
            Lean4DoStmt::Expr(e) => write!(f, "{}", e),
            Lean4DoStmt::Return(e) => write!(f, "return {}", e),
            Lean4DoStmt::Pure(e) => write!(f, "pure {}", e),
            Lean4DoStmt::If(cond, then_stmts, else_stmts) => {
                write!(f, "if {} then", cond)?;
                for s in then_stmts {
                    write!(f, "\n    {}", s)?;
                }
                if !else_stmts.is_empty() {
                    write!(f, "\n  else")?;
                    for s in else_stmts {
                        write!(f, "\n    {}", s)?;
                    }
                }
                Ok(())
            }
        }
    }
}
