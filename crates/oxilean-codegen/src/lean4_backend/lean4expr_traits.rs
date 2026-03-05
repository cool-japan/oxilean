//! # Lean4Expr - Trait Implementations
//!
//! This module contains trait implementations for `Lean4Expr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::Lean4Expr;
use std::fmt;

impl fmt::Display for Lean4Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lean4Expr::Var(name) => write!(f, "{}", name),
            Lean4Expr::NatLit(n) => write!(f, "{}", n),
            Lean4Expr::IntLit(n) => write!(f, "{}", n),
            Lean4Expr::BoolLit(b) => write!(f, "{}", b),
            Lean4Expr::StrLit(s) => {
                write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            Lean4Expr::FloatLit(v) => write!(f, "{}", v),
            Lean4Expr::Hole => write!(f, "_"),
            Lean4Expr::Sorry => write!(f, "sorry"),
            Lean4Expr::Panic(msg) => write!(f, "panic! \"{}\"", msg),
            Lean4Expr::App(func, arg) => write!(f, "{} {}", func, paren_expr(arg)),
            Lean4Expr::Lambda(param, ty, body) => {
                if let Some(t) = ty {
                    write!(f, "fun ({} : {}) => {}", param, t, body)
                } else {
                    write!(f, "fun {} => {}", param, body)
                }
            }
            Lean4Expr::Pi(param, ty, body) => {
                write!(f, "({} : {}) → {}", param, ty, body)
            }
            Lean4Expr::Let(name, ty, val, body) => {
                if let Some(t) = ty {
                    write!(f, "let {} : {} := {}\n{}", name, t, val, body)
                } else {
                    write!(f, "let {} := {}\n{}", name, val, body)
                }
            }
            Lean4Expr::LetRec(name, val, body) => {
                write!(f, "let rec {} := {}\n{}", name, val, body)
            }
            Lean4Expr::Match(scrutinee, arms) => {
                writeln!(f, "match {} with", scrutinee)?;
                for (pat, body) in arms {
                    writeln!(f, "  | {} => {}", pat, body)?;
                }
                Ok(())
            }
            Lean4Expr::If(cond, then_e, else_e) => {
                write!(f, "if {} then {} else {}", cond, then_e, else_e)
            }
            Lean4Expr::Do(stmts) => {
                writeln!(f, "do")?;
                for stmt in stmts {
                    writeln!(f, "  {}", stmt)?;
                }
                Ok(())
            }
            Lean4Expr::Have(name, ty, proof, rest) => {
                write!(f, "have {} : {} := {}\n{}", name, ty, proof, rest)
            }
            Lean4Expr::Show(ty, expr) => write!(f, "show {} from {}", ty, expr),
            Lean4Expr::Calc(steps) => {
                writeln!(f, "calc")?;
                for step in steps {
                    writeln!(f, "  {}", step)?;
                }
                Ok(())
            }
            Lean4Expr::ByTactic(tactics) => write!(f, "by {}", tactics.join("; ")),
            Lean4Expr::Ascription(expr, ty) => write!(f, "({} : {})", expr, ty),
            Lean4Expr::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            Lean4Expr::AnonymousCtor(elems) => {
                write!(f, "⟨")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "⟩")
            }
            Lean4Expr::Proj(expr, field) => write!(f, "{}.{}", paren_expr(expr), field),
            Lean4Expr::StructLit(name, fields) => {
                write!(f, "{{ ")?;
                if !name.is_empty() {
                    write!(f, "{} . ", name)?;
                }
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} := {}", k, v)?;
                }
                write!(f, " }}")
            }
        }
    }
}
