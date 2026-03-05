//! # IdrisExpr - Trait Implementations
//!
//! This module contains trait implementations for `IdrisExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisExpr;
use std::fmt;

impl fmt::Display for IdrisExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisExpr::Lit(l) => write!(f, "{}", l),
            IdrisExpr::Var(v) => write!(f, "{}", v),
            IdrisExpr::Qualified(parts) => write!(f, "{}", parts.join(".")),
            IdrisExpr::Refl => write!(f, "Refl"),
            IdrisExpr::Hole(h) => write!(f, "?{}", h),
            IdrisExpr::AnonHole => write!(f, "?_"),
            IdrisExpr::App(func, arg) => {
                write!(f, "{} {}", func.fmt_arg(), arg.fmt_arg())
            }
            IdrisExpr::Lam(params, body) => {
                write!(f, "\\{} => {}", params.join(", "), body)
            }
            IdrisExpr::Let(name, val, body) => {
                write!(f, "let {} = {} in {}", name, val, body)
            }
            IdrisExpr::IfThenElse(cond, t, e) => {
                write!(f, "if {} then {} else {}", cond, t, e)
            }
            IdrisExpr::Tuple(exprs) => {
                write!(f, "(")?;
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            IdrisExpr::ListLit(exprs) => {
                write!(f, "[")?;
                for (i, e) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            IdrisExpr::Annot(e, t) => write!(f, "({} : {})", e, t),
            IdrisExpr::Idiom(e) => write!(f, "[| {} |]", e),
            IdrisExpr::ProofTerm(e) => write!(f, "believe_me {}", e.fmt_arg()),
            IdrisExpr::Neg(e) => write!(f, "-{}", e.fmt_arg()),
            IdrisExpr::Infix(op, l, r) => {
                write!(f, "{} {} {}", l.fmt_arg(), op, r.fmt_arg())
            }
            IdrisExpr::RecordUpdate(base, fields) => {
                write!(f, "{{ {} with ", base)?;
                for (i, (name, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", name, val)?;
                }
                write!(f, " }}")
            }
            IdrisExpr::WithPattern(scrutinee, alts) => {
                writeln!(f, "with ({})", scrutinee)?;
                for (pat, body) in alts {
                    writeln!(f, "  ... | {} = {}", pat, body)?;
                }
                Ok(())
            }
            IdrisExpr::CaseOf(scrutinee, alts) => {
                writeln!(f, "case {} of", scrutinee)?;
                for (pat, body) in alts {
                    writeln!(f, "  {} => {}", pat, body)?;
                }
                Ok(())
            }
            IdrisExpr::Do(stmts) => {
                writeln!(f, "do")?;
                for s in stmts {
                    writeln!(f, "  {}", s)?;
                }
                Ok(())
            }
        }
    }
}
