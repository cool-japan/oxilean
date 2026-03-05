//! # ScalaExpr - Trait Implementations
//!
//! This module contains trait implementations for `ScalaExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{ScalaEnumerator, ScalaExpr};
use std::fmt;

impl fmt::Display for ScalaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaExpr::Lit(lit) => write!(f, "{}", lit),
            ScalaExpr::Var(v) => write!(f, "{}", v),
            ScalaExpr::App(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            ScalaExpr::Infix(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            ScalaExpr::Prefix(op, operand) => write!(f, "{}({})", op, operand),
            ScalaExpr::If(cond, then_e, else_e) => {
                write!(f, "if {} then {} else {}", cond, then_e, else_e)
            }
            ScalaExpr::Match(scrut, arms) => {
                writeln!(f, "{} match {{", scrut)?;
                for arm in arms {
                    write!(f, "  case {}", arm.pattern)?;
                    if let Some(guard) = &arm.guard {
                        write!(f, " if {}", guard)?;
                    }
                    writeln!(f, " => {}", arm.body)?;
                }
                write!(f, "}}")
            }
            ScalaExpr::For(enumerators, body) => {
                writeln!(f, "for {{")?;
                for e in enumerators {
                    match e {
                        ScalaEnumerator::Generator(v, expr) => {
                            writeln!(f, "  {} <- {}", v, expr)?;
                        }
                        ScalaEnumerator::Guard(expr) => {
                            writeln!(f, "  if {}", expr)?;
                        }
                        ScalaEnumerator::Definition(v, expr) => {
                            writeln!(f, "  {} = {}", v, expr)?;
                        }
                    }
                }
                write!(f, "}} yield {}", body)
            }
            ScalaExpr::Try(body, catches, finally) => {
                write!(f, "try {{ {} }}", body)?;
                if !catches.is_empty() {
                    writeln!(f, " catch {{")?;
                    for c in catches {
                        writeln!(f, "  case {} => {}", c.pattern, c.body)?;
                    }
                    write!(f, "}}")?;
                }
                if let Some(fin) = finally {
                    write!(f, " finally {{ {} }}", fin)?;
                }
                Ok(())
            }
            ScalaExpr::Lambda(params, body) => {
                if params.len() == 1 {
                    write!(f, "{} => {}", params[0], body)
                } else {
                    write!(f, "(")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ") => {}", body)
                }
            }
            ScalaExpr::Block(stmts, last) => {
                writeln!(f, "{{")?;
                for s in stmts {
                    writeln!(f, "  {}", s)?;
                }
                write!(f, "  {}\n}}", last)
            }
            ScalaExpr::New(cls, args) => {
                write!(f, "new {}(", cls)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            ScalaExpr::This => write!(f, "this"),
            ScalaExpr::Super => write!(f, "super"),
            ScalaExpr::Assign(name, val) => write!(f, "{} = {}", name, val),
            ScalaExpr::TypeAnnotation(expr, ty) => write!(f, "({}: {})", expr, ty),
            ScalaExpr::Throw(expr) => write!(f, "throw {}", expr),
        }
    }
}
