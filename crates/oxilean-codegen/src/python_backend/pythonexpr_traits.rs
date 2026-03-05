//! # PythonExpr - Trait Implementations
//!
//! This module contains trait implementations for `PythonExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{FStringPart, PythonExpr};
use std::fmt;

impl fmt::Display for PythonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonExpr::Lit(lit) => write!(f, "{}", lit),
            PythonExpr::Var(name) => write!(f, "{}", name),
            PythonExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            PythonExpr::UnaryOp(op, expr) => {
                if *op == "not" {
                    write!(f, "not {}", expr)
                } else {
                    write!(f, "{}{}", op, expr)
                }
            }
            PythonExpr::Call(func, args, kwargs) => {
                write!(f, "{}(", func)?;
                let mut first = true;
                for a in args {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                    first = false;
                }
                for (k, v) in kwargs {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}={}", k, v)?;
                    first = false;
                }
                write!(f, ")")
            }
            PythonExpr::Attr(obj, field) => write!(f, "{}.{}", obj, field),
            PythonExpr::Subscript(obj, idx) => write!(f, "{}[{}]", obj, idx),
            PythonExpr::Lambda(params, body) => {
                write!(f, "lambda ")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ": {}", body)
            }
            PythonExpr::IfExpr(then_e, cond, else_e) => {
                write!(f, "{} if {} else {}", then_e, cond, else_e)
            }
            PythonExpr::ListComp(expr, var, iter, cond) => {
                write!(f, "[{} for {} in {}", expr, var, iter)?;
                if let Some(c) = cond {
                    write!(f, " if {}", c)?;
                }
                write!(f, "]")
            }
            PythonExpr::DictComp(key, val, k_var, v_var, iter) => {
                write!(
                    f,
                    "{{{}: {} for {}, {} in {}}}",
                    key, val, k_var, v_var, iter
                )
            }
            PythonExpr::SetComp(expr, var, iter, cond) => {
                write!(f, "{{{} for {} in {}", expr, var, iter)?;
                if let Some(c) = cond {
                    write!(f, " if {}", c)?;
                }
                write!(f, "}}")
            }
            PythonExpr::GenExpr(expr, var, iter, cond) => {
                write!(f, "({} for {} in {}", expr, var, iter)?;
                if let Some(c) = cond {
                    write!(f, " if {}", c)?;
                }
                write!(f, ")")
            }
            PythonExpr::Tuple(elems) => {
                if elems.is_empty() {
                    write!(f, "()")
                } else if elems.len() == 1 {
                    write!(f, "({},)", elems[0])
                } else {
                    write!(f, "(")?;
                    for (i, e) in elems.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", e)?;
                    }
                    write!(f, ")")
                }
            }
            PythonExpr::List(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            PythonExpr::Dict(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            PythonExpr::Set(elems) => {
                if elems.is_empty() {
                    write!(f, "set()")
                } else {
                    write!(f, "{{")?;
                    for (i, e) in elems.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", e)?;
                    }
                    write!(f, "}}")
                }
            }
            PythonExpr::Await(expr) => write!(f, "await {}", expr),
            PythonExpr::Yield(expr) => match expr {
                Some(e) => write!(f, "yield {}", e),
                None => write!(f, "yield"),
            },
            PythonExpr::YieldFrom(expr) => write!(f, "yield from {}", expr),
            PythonExpr::Match(expr) => write!(f, "{}", expr),
            PythonExpr::FString(parts) => {
                write!(f, "f\"")?;
                for part in parts {
                    match part {
                        FStringPart::Literal(s) => {
                            for c in s.chars() {
                                match c {
                                    '"' => write!(f, "\\\"")?,
                                    '{' => write!(f, "{{")?,
                                    '}' => write!(f, "}}")?,
                                    '\\' => write!(f, "\\\\")?,
                                    '\n' => write!(f, "\\n")?,
                                    c => write!(f, "{}", c)?,
                                }
                            }
                        }
                        FStringPart::Expr(e) => write!(f, "{{{}}}", e)?,
                        FStringPart::ExprWithFormat(e, spec) => write!(f, "{{{}:{}}}", e, spec)?,
                    }
                }
                write!(f, "\"")
            }
            PythonExpr::Walrus(name, expr) => write!(f, "({} := {})", name, expr),
            PythonExpr::Star(expr) => write!(f, "*{}", expr),
            PythonExpr::DoubleStar(expr) => write!(f, "**{}", expr),
            PythonExpr::Slice(start, stop, step) => {
                if let Some(s) = start {
                    write!(f, "{}", s)?;
                }
                write!(f, ":")?;
                if let Some(s) = stop {
                    write!(f, "{}", s)?;
                }
                if let Some(s) = step {
                    write!(f, ":{}", s)?;
                }
                Ok(())
            }
        }
    }
}
