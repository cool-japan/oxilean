//! # SurfaceExpr - Trait Implementations
//!
//! This module contains trait implementations for `SurfaceExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SurfaceExpr;
use std::fmt;

impl fmt::Display for SurfaceExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SurfaceExpr::Sort(sk) => write!(f, "{}", sk),
            SurfaceExpr::Var(name) => write!(f, "{}", name),
            SurfaceExpr::App(fun, arg) => write!(f, "({} {})", fun.value, arg.value),
            SurfaceExpr::Lam(_, body) => write!(f, "(fun ... => {})", body.value),
            SurfaceExpr::Pi(_, body) => write!(f, "(forall ..., {})", body.value),
            SurfaceExpr::Let(name, _, val, body) => {
                write!(f, "(let {} := {} in {})", name, val.value, body.value)
            }
            SurfaceExpr::Lit(lit) => write!(f, "{}", lit),
            SurfaceExpr::Ann(expr, ty) => write!(f, "({} : {})", expr.value, ty.value),
            SurfaceExpr::Hole => write!(f, "_"),
            SurfaceExpr::Proj(expr, field) => write!(f, "{}.{}", expr.value, field),
            SurfaceExpr::If(cond, then_e, else_e) => {
                write!(
                    f,
                    "(if {} then {} else {})",
                    cond.value, then_e.value, else_e.value
                )
            }
            SurfaceExpr::Match(scrut, _arms) => {
                write!(f, "(match {} with ...)", scrut.value)
            }
            SurfaceExpr::Do(_) => write!(f, "(do ...)"),
            SurfaceExpr::Have(name, ty, _, _) => {
                write!(f, "(have {} : {} ...)", name, ty.value)
            }
            SurfaceExpr::Suffices(name, ty, _) => {
                write!(f, "(suffices {} : {} ...)", name, ty.value)
            }
            SurfaceExpr::Show(ty, _) => write!(f, "(show {} ...)", ty.value),
            SurfaceExpr::NamedArg(fun, name, val) => {
                write!(f, "({} ({} := {}))", fun.value, name, val.value)
            }
            SurfaceExpr::AnonymousCtor(fields) => {
                write!(f, "<")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field.value)?;
                }
                write!(f, ">")
            }
            SurfaceExpr::ListLit(elems) => {
                write!(f, "[")?;
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem.value)?;
                }
                write!(f, "]")
            }
            SurfaceExpr::Tuple(elems) => {
                write!(f, "(")?;
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem.value)?;
                }
                write!(f, ")")
            }
            SurfaceExpr::Return(expr) => write!(f, "(return {})", expr.value),
            SurfaceExpr::StringInterp(_parts) => write!(f, "s!\"...\""),
            SurfaceExpr::Range(lo, hi) => {
                if let Some(lo) = lo {
                    write!(f, "{}", lo.value)?;
                }
                write!(f, "..")?;
                if let Some(hi) = hi {
                    write!(f, "{}", hi.value)?;
                }
                Ok(())
            }
            SurfaceExpr::ByTactic(tactics) => {
                write!(f, "(by")?;
                for (i, t) in tactics.iter().enumerate() {
                    if i > 0 {
                        write!(f, ";")?;
                    }
                    write!(f, " {}", t.value)?;
                }
                write!(f, ")")
            }
            SurfaceExpr::Calc(steps) => {
                write!(f, "(calc")?;
                for step in steps {
                    write!(
                        f,
                        " {} {} {} := ...",
                        step.lhs.value, step.rel, step.rhs.value
                    )?;
                }
                write!(f, ")")
            }
        }
    }
}
