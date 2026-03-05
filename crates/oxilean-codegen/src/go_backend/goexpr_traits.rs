//! # GoExpr - Trait Implementations
//!
//! This module contains trait implementations for `GoExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::format_stmts;
use super::types::GoExpr;
use std::fmt;

impl fmt::Display for GoExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoExpr::Lit(lit) => write!(f, "{}", lit),
            GoExpr::Var(name) => write!(f, "{}", name),
            GoExpr::Call(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            GoExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            GoExpr::Unary(op, operand) => write!(f, "({}{})", op, operand),
            GoExpr::Field(obj, field) => write!(f, "{}.{}", obj, field),
            GoExpr::Index(base, idx) => write!(f, "{}[{}]", base, idx),
            GoExpr::TypeAssert(expr, ty) => write!(f, "{}.({}", expr, ty),
            GoExpr::Composite(ty, fields) => {
                write!(f, "{}{{", ty)?;
                for (i, (name, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, val)?;
                }
                write!(f, "}}")
            }
            GoExpr::SliceLit(elem_ty, elems) => {
                write!(f, "[]{}{{", elem_ty)?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "}}")
            }
            GoExpr::AddressOf(inner) => write!(f, "&{}", inner),
            GoExpr::Deref(inner) => write!(f, "*{}", inner),
            GoExpr::FuncLit(params, rets, body) => {
                write!(f, "func(")?;
                for (i, (name, ty)) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", name, ty)?;
                }
                write!(f, ")")?;
                match rets.len() {
                    0 => {}
                    1 => write!(f, " {}", rets[0])?,
                    _ => {
                        write!(f, " (")?;
                        for (i, r) in rets.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", r)?;
                        }
                        write!(f, ")")?;
                    }
                }
                write!(f, " {{")?;
                let body_str = format_stmts(body, 1);
                if !body_str.is_empty() {
                    write!(f, "\n{}\n}}", body_str)?;
                } else {
                    write!(f, "}}")?;
                }
                Ok(())
            }
            GoExpr::Make(ty, args) => {
                write!(f, "make({}", ty)?;
                for a in args {
                    write!(f, ", {}", a)?;
                }
                write!(f, ")")
            }
            GoExpr::New(ty) => write!(f, "new({})", ty),
        }
    }
}
