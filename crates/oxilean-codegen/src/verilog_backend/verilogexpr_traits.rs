//! # VerilogExpr - Trait Implementations
//!
//! This module contains trait implementations for `VerilogExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VerilogExpr;
use std::fmt;

impl fmt::Display for VerilogExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerilogExpr::Lit(v, w) => {
                if *w == 0 || *w == 1 {
                    write!(f, "1'b{}", if *v != 0 { 1 } else { 0 })
                } else {
                    write!(f, "{w}'h{v:X}")
                }
            }
            VerilogExpr::Var(s) => write!(f, "{s}"),
            VerilogExpr::BinOp(l, op, r) => write!(f, "({l} {op} {r})"),
            VerilogExpr::UnOp(op, e) => write!(f, "({op}{e})"),
            VerilogExpr::Concat(parts) => {
                write!(f, "{{")?;
                for (i, p) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{p}")?;
                }
                write!(f, "}}")
            }
            VerilogExpr::Replicate(n, e) => write!(f, "{{{n}{{{e}}}}}"),
            VerilogExpr::Index(e, bit) => write!(f, "{e}[{bit}]"),
            VerilogExpr::Slice(e, hi, lo) => write!(f, "{e}[{hi}:{lo}]"),
            VerilogExpr::Ternary(c, t, e) => write!(f, "({c} ? {t} : {e})"),
            VerilogExpr::Call(name, args) => {
                write!(f, "{name}(")?;
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
