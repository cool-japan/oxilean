//! # TsExpr - Trait Implementations
//!
//! This module contains trait implementations for `TsExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::{TsExpr, TsTemplatePart};
use std::fmt;

impl fmt::Display for TsExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsExpr::Lit(lit) => write!(f, "{}", lit),
            TsExpr::Var(name) => write!(f, "{}", name),
            TsExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            TsExpr::UnaryOp(op, expr) => write!(f, "{}{}", op, expr),
            TsExpr::Call(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            TsExpr::MethodCall(obj, method, args) => {
                write!(f, "{}.{}(", obj, method)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            TsExpr::New(ctor, args) => {
                write!(f, "new {}(", ctor)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            TsExpr::Arrow(params, body) => {
                write!(f, "(")?;
                for (i, (name, ty)) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let Some(t) = ty {
                        write!(f, "{}: {}", name, t)?;
                    } else {
                        write!(f, "{}", name)?;
                    }
                }
                write!(f, ") => {}", body)
            }
            TsExpr::Ternary(cond, then_e, else_e) => {
                write!(f, "({}) ? ({}) : ({})", cond, then_e, else_e)
            }
            TsExpr::As(expr, ty) => write!(f, "({} as {})", expr, ty),
            TsExpr::Satisfies(expr, ty) => write!(f, "({} satisfies {})", expr, ty),
            TsExpr::TypeAssert(expr) => write!(f, "{}!", expr),
            TsExpr::ObjectLit(fields) => {
                write!(f, "{{ ")?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
            TsExpr::ArrayLit(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            TsExpr::Template(parts) => {
                write!(f, "`")?;
                for part in parts {
                    match part {
                        TsTemplatePart::Text(s) => write!(f, "{}", s)?,
                        TsTemplatePart::Expr(e) => write!(f, "${{{}}}", e)?,
                    }
                }
                write!(f, "`")
            }
            TsExpr::Await(expr) => write!(f, "await {}", expr),
            TsExpr::Nullish(lhs, rhs) => write!(f, "({} ?? {})", lhs, rhs),
            TsExpr::OptChain(obj, prop) => write!(f, "{}?.{}", obj, prop),
        }
    }
}
