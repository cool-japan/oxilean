//! # FortranExpr - Trait Implementations
//!
//! This module contains trait implementations for `FortranExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranExpr;
use std::fmt;

impl fmt::Display for FortranExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FortranExpr::Lit(lit) => write!(f, "{}", lit),
            FortranExpr::Var(name) => write!(f, "{}", name),
            FortranExpr::ArrayIndex(arr, idxs) => {
                write!(f, "{}(", arr)?;
                for (i, idx) in idxs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", idx)?;
                }
                write!(f, ")")
            }
            FortranExpr::Component(base, field) => write!(f, "{}%{}", base, field),
            FortranExpr::BinOp(left, op, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
            FortranExpr::UnaryOp(op, expr) => write!(f, "({}{})", op, expr),
            FortranExpr::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            FortranExpr::ArrayCtor(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            FortranExpr::ImpliedDo(expr, var, lo, hi) => {
                write!(f, "({}, {} = {}, {})", expr, var, lo, hi)
            }
            FortranExpr::TypeCtor(name, fields) => {
                write!(f, "{}(", name)?;
                for (i, (fname, fval)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}={}", fname, fval)?;
                }
                write!(f, ")")
            }
            FortranExpr::Merge(a, b, mask) => write!(f, "MERGE({}, {}, {})", a, b, mask),
            FortranExpr::Raw(s) => write!(f, "{}", s),
        }
    }
}
