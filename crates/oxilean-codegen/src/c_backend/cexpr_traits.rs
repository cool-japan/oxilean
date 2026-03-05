//! # CExpr - Trait Implementations
//!
//! This module contains trait implementations for `CExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CExpr;
use std::fmt;

impl fmt::Display for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CExpr::Var(name) => write!(f, "{}", name),
            CExpr::IntLit(n) => write!(f, "{}LL", n),
            CExpr::UIntLit(n) => write!(f, "{}ULL", n),
            CExpr::StringLit(s) => {
                write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            CExpr::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            CExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            CExpr::UnaryOp(op, expr) => write!(f, "({}{})", op, expr),
            CExpr::FieldAccess(expr, field, is_arrow) => {
                if *is_arrow {
                    write!(f, "{}->{}", expr, field)
                } else {
                    write!(f, "{}.{}", expr, field)
                }
            }
            CExpr::ArrayAccess(arr, idx) => write!(f, "{}[{}]", arr, idx),
            CExpr::Cast(ty, expr) => write!(f, "(({})({})", ty, expr),
            CExpr::SizeOf(ty) => write!(f, "sizeof({})", ty),
            CExpr::Null => write!(f, "NULL"),
            CExpr::Ternary(cond, t, e) => write!(f, "({} ? {} : {})", cond, t, e),
            CExpr::Initializer(ty, fields) => {
                write!(f, "({})", ty)?;
                write!(f, "{{")?;
                for (i, (name, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, ".{} = {}", name, val)?;
                }
                write!(f, "}}")
            }
        }
    }
}
