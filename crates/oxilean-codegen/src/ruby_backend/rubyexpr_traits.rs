//! # RubyExpr - Trait Implementations
//!
//! This module contains trait implementations for `RubyExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::{RubyExpr, RubyLit};
use std::fmt;

impl fmt::Display for RubyExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RubyExpr::Lit(lit) => write!(f, "{}", lit),
            RubyExpr::Var(name) => write!(f, "{}", name),
            RubyExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            RubyExpr::UnaryOp(op, operand) => write!(f, "{}({})", op, operand),
            RubyExpr::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            RubyExpr::MethodCall(recv, method, args) => {
                write!(f, "{}.{}(", recv, method)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            RubyExpr::Block(params, stmts) => {
                write!(f, "{{ ")?;
                if !params.is_empty() {
                    write!(f, "|{}| ", params.join(", "))?;
                }
                for stmt in stmts {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            RubyExpr::Lambda(params, stmts) => {
                write!(f, "->(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") {{ ")?;
                for stmt in stmts {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            RubyExpr::If(cond, then_e, else_e) => {
                write!(f, "({} ? {} : {})", cond, then_e, else_e)
            }
            RubyExpr::Case(scrutinee, branches, default) => {
                writeln!(f, "case {}", scrutinee)?;
                for (pat, body) in branches {
                    write!(f, "when {}\n  {}\n", pat, body)?;
                }
                if let Some(def) = default {
                    write!(f, "else\n  {}\n", def)?;
                }
                write!(f, "end")
            }
            RubyExpr::Array(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            RubyExpr::Hash(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match k {
                        RubyExpr::Lit(RubyLit::Symbol(name)) => {
                            write!(f, "{}: {}", name, v)?;
                        }
                        _ => {
                            write!(f, "{} => {}", k, v)?;
                        }
                    }
                }
                write!(f, "}}")
            }
            RubyExpr::Assign(name, rhs) => write!(f, "{} = {}", name, rhs),
            RubyExpr::Return(expr) => write!(f, "return {}", expr),
        }
    }
}
