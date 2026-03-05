//! # DartExpr - Trait Implementations
//!
//! This module contains trait implementations for `DartExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_args, fmt_typed_params};
use super::types::DartExpr;
use std::fmt;

impl fmt::Display for DartExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DartExpr::Lit(lit) => write!(f, "{}", lit),
            DartExpr::Var(name) => write!(f, "{}", name),
            DartExpr::Field(expr, field) => write!(f, "{}.{}", expr, field),
            DartExpr::MethodCall(recv, method, args) => {
                write!(f, "{}.{}(", recv, method)?;
                fmt_args(f, args)?;
                write!(f, ")")
            }
            DartExpr::Call(callee, args) => {
                write!(f, "{}(", callee)?;
                fmt_args(f, args)?;
                write!(f, ")")
            }
            DartExpr::New(class, named, args) => {
                if let Some(n) = named {
                    write!(f, "{}.{}(", class, n)?;
                } else {
                    write!(f, "{}(", class)?;
                }
                fmt_args(f, args)?;
                write!(f, ")")
            }
            DartExpr::ListLit(elems) => {
                write!(f, "[")?;
                fmt_args(f, elems)?;
                write!(f, "]")
            }
            DartExpr::MapLit(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            DartExpr::SetLit(elems) => {
                write!(f, "{{")?;
                fmt_args(f, elems)?;
                write!(f, "}}")
            }
            DartExpr::Lambda(params, body) => {
                write!(f, "(")?;
                fmt_typed_params(f, params)?;
                write!(f, ") {{ return {}; }}", body)
            }
            DartExpr::Arrow(params, body) => {
                write!(f, "(")?;
                fmt_typed_params(f, params)?;
                write!(f, ") => {}", body)
            }
            DartExpr::BinOp(left, op, right) => write!(f, "({} {} {})", left, op, right),
            DartExpr::UnaryOp(op, expr) => write!(f, "({}{})", op, expr),
            DartExpr::Ternary(cond, then, else_) => {
                write!(f, "({} ? {} : {})", cond, then, else_)
            }
            DartExpr::NullAware(expr, field) => write!(f, "{}?.{}", expr, field),
            DartExpr::NullCoalesce(expr, fallback) => {
                write!(f, "({} ?? {})", expr, fallback)
            }
            DartExpr::Cascade(recv, method, args) => {
                write!(f, "{}..{}(", recv, method)?;
                fmt_args(f, args)?;
                write!(f, ")")
            }
            DartExpr::Await(expr) => write!(f, "await {}", expr),
            DartExpr::As(expr, ty) => write!(f, "({} as {})", expr, ty),
            DartExpr::Is(expr, ty) => write!(f, "({} is {})", expr, ty),
            DartExpr::Throw(expr) => write!(f, "throw {}", expr),
            DartExpr::Spread(expr) => write!(f, "...{}", expr),
            DartExpr::Index(expr, idx) => write!(f, "{}[{}]", expr, idx),
            DartExpr::Raw(s) => write!(f, "{}", s),
        }
    }
}
