//! # JavaExpr - Trait Implementations
//!
//! This module contains trait implementations for `JavaExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JavaExpr;
use std::fmt;

impl fmt::Display for JavaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavaExpr::Lit(lit) => write!(f, "{}", lit),
            JavaExpr::Var(name) => write!(f, "{}", name),
            JavaExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            JavaExpr::UnaryOp(op, operand) => write!(f, "{}{}", op, operand),
            JavaExpr::Call(callee, args) => {
                write!(f, "{}(", callee)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JavaExpr::MethodCall(recv, method, args) => {
                write!(f, "{}.{}(", recv, method)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JavaExpr::New(cls, args) => {
                write!(f, "new {}(", cls)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            JavaExpr::Cast(ty, expr) => write!(f, "(({}) {})", ty, expr),
            JavaExpr::Instanceof(expr, ty) => write!(f, "({} instanceof {})", expr, ty),
            JavaExpr::Ternary(cond, then, else_) => {
                write!(f, "({} ? {} : {})", cond, then, else_)
            }
            JavaExpr::Null => write!(f, "null"),
            JavaExpr::Lambda(params, body) => {
                if params.is_empty() {
                    write!(f, "() -> {}", body)
                } else if params.len() == 1 {
                    write!(f, "{} -> {}", params[0], body)
                } else {
                    write!(f, "(")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ") -> {}", body)
                }
            }
            JavaExpr::MethodRef(cls, method) => write!(f, "{}::{}", cls, method),
            JavaExpr::ArrayAccess(arr, idx) => write!(f, "{}[{}]", arr, idx),
            JavaExpr::FieldAccess(obj, field) => write!(f, "{}.{}", obj, field),
        }
    }
}
