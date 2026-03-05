//! # HaskellExpr - Trait Implementations
//!
//! This module contains trait implementations for `HaskellExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{HaskellDoStmt, HaskellExpr, HsListQual};
use std::fmt;

impl fmt::Display for HaskellExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaskellExpr::Lit(lit) => write!(f, "{}", lit),
            HaskellExpr::Var(v) => write!(f, "{}", v),
            HaskellExpr::App(func, args) => {
                write!(f, "({}", func)?;
                for a in args {
                    write!(f, " {}", a)?;
                }
                write!(f, ")")
            }
            HaskellExpr::Lambda(pats, body) => {
                write!(f, "(\\")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, " -> {})", body)
            }
            HaskellExpr::Let(name, val, body) => {
                write!(f, "(let {} = {} in {})", name, val, body)
            }
            HaskellExpr::Where(expr, _defs) => write!(f, "{}", expr),
            HaskellExpr::If(cond, then_e, else_e) => {
                write!(f, "(if {} then {} else {})", cond, then_e, else_e)
            }
            HaskellExpr::Case(scrut, alts) => {
                write!(f, "(case {} of {{ ", scrut)?;
                for (i, alt) in alts.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", alt.pattern)?;
                    if let Some(body) = &alt.body {
                        write!(f, " -> {}", body)?;
                    }
                }
                write!(f, " }})")
            }
            HaskellExpr::Do(stmts) => {
                write!(f, "do {{ ")?;
                for (i, stmt) in stmts.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    match stmt {
                        HaskellDoStmt::Bind(name, expr) => write!(f, "{} <- {}", name, expr)?,
                        HaskellDoStmt::Stmt(expr) => write!(f, "{}", expr)?,
                        HaskellDoStmt::LetBind(name, expr) => write!(f, "let {} = {}", name, expr)?,
                    }
                }
                write!(f, " }}")
            }
            HaskellExpr::ListComp(body, quals) => {
                write!(f, "[ {} | ", body)?;
                for (i, q) in quals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match q {
                        HsListQual::Generator(v, e) => write!(f, "{} <- {}", v, e)?,
                        HsListQual::Guard(e) => write!(f, "{}", e)?,
                        HsListQual::LetBind(v, e) => write!(f, "let {} = {}", v, e)?,
                    }
                }
                write!(f, " ]")
            }
            HaskellExpr::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            HaskellExpr::List(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            HaskellExpr::Neg(e) => write!(f, "(negate {})", e),
            HaskellExpr::InfixApp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            HaskellExpr::Operator(op) => write!(f, "({})", op),
            HaskellExpr::TypeAnnotation(expr, ty) => write!(f, "({} :: {})", expr, ty),
        }
    }
}
