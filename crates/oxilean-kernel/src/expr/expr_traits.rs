//! # Expr - Trait Implementations
//!
//! This module contains trait implementations for `Expr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::functions::*;
use super::types::{Expr, Literal};

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Sort(l) if l.is_zero() => write!(f, "Prop"),
            Expr::Sort(l) => write!(f, "Type {}", l),
            Expr::BVar(n) => write!(f, "#{}", n),
            Expr::FVar(id) => write!(f, "fvar_{}", id.0),
            Expr::Const(name, levels) if levels.is_empty() => write!(f, "{}", name),
            Expr::Const(name, levels) => {
                write!(f, "{}.{{", name)?;
                for (i, l) in levels.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", l)?;
                }
                write!(f, "}}")
            }
            Expr::App(fun, arg) => write!(f, "({} {})", fun, arg),
            Expr::Lam(_, name, ty, body) => write!(f, "(λ {} : {}, {})", name, ty, body),
            Expr::Pi(_, name, ty, body) => {
                if !has_loose_bvar(body, 0) {
                    write!(f, "({} → {})", ty, body)
                } else {
                    write!(f, "(Π {} : {}, {})", name, ty, body)
                }
            }
            Expr::Let(name, ty, val, body) => {
                write!(f, "(let {} : {} := {} in {})", name, ty, val, body)
            }
            Expr::Lit(Literal::Nat(n)) => write!(f, "{}", n),
            Expr::Lit(Literal::Str(s)) => write!(f, "\"{}\"", s),
            Expr::Proj(name, idx, e) => write!(f, "{}.{} {}", name, idx, e),
        }
    }
}
