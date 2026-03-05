//! # JsExpr - Trait Implementations
//!
//! This module contains trait implementations for `JsExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::{JsExpr, JsStmt};
use std::fmt;

impl fmt::Display for JsExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsExpr::Lit(lit) => write!(f, "{}", lit),
            JsExpr::Var(name) => write!(f, "{}", name),
            JsExpr::Call(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JsExpr::Method(obj, method, args) => {
                write!(f, "{}.{}(", obj, method)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JsExpr::Field(obj, field) => write!(f, "{}.{}", obj, field),
            JsExpr::Index(arr, idx) => write!(f, "{}[{}]", arr, idx),
            JsExpr::Arrow(params, body) => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") => ")?;
                match body.as_ref() {
                    JsStmt::Block(stmts) => {
                        write!(f, "{{")?;
                        let inner = display_indented(stmts, 2);
                        if !inner.is_empty() {
                            write!(f, "\n{}\n}}", inner)?;
                        } else {
                            write!(f, "}}")?;
                        }
                    }
                    other => write!(f, "{}", other)?,
                }
                Ok(())
            }
            JsExpr::Ternary(cond, then_e, else_e) => {
                write!(f, "({}) ? ({}) : ({})", cond, then_e, else_e)
            }
            JsExpr::BinOp(op, lhs, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            JsExpr::UnOp(op, expr) => write!(f, "{}{}", op, expr),
            JsExpr::Await(expr) => write!(f, "await {}", expr),
            JsExpr::New(ctor, args) => {
                write!(f, "new {}(", ctor)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JsExpr::Spread(expr) => write!(f, "...{}", expr),
            JsExpr::Object(fields) => {
                write!(f, "{{")?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            JsExpr::Array(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
        }
    }
}
