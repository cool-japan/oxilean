//! # ChiselExpr - Trait Implementations
//!
//! This module contains trait implementations for `ChiselExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChiselExpr;
use std::fmt;

impl fmt::Display for ChiselExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChiselExpr::ULit(v, w) => write!(f, "{v}.U({w}.W)"),
            ChiselExpr::SLit(v, w) => write!(f, "{v}.S({w}.W)"),
            ChiselExpr::BoolLit(b) => {
                write!(f, "{}.B", if *b { "true" } else { "false" })
            }
            ChiselExpr::Var(s) => write!(f, "{s}"),
            ChiselExpr::Io(name) => write!(f, "io.{name}"),
            ChiselExpr::RegField(n) => write!(f, "reg_{n}"),
            ChiselExpr::BinOp(l, op, r) => write!(f, "({l} {op} {r})"),
            ChiselExpr::UnOp(op, e) => write!(f, "({op}({e}))"),
            ChiselExpr::Mux(c, t, e) => write!(f, "Mux({c}, {t}, {e})"),
            ChiselExpr::BitSlice(e, hi, lo) => write!(f, "{e}({hi}, {lo})"),
            ChiselExpr::Cat(parts) => {
                write!(f, "Cat(")?;
                for (i, p) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{p}")?;
                }
                write!(f, ")")
            }
            ChiselExpr::MethodCall(recv, method, args) => {
                write!(f, "{recv}.{method}(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{a}")?;
                }
                write!(f, ")")
            }
        }
    }
}
