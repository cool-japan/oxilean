//! # LuaExpr - Trait Implementations
//!
//! This module contains trait implementations for `LuaExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaExpr;
use std::fmt;

impl fmt::Display for LuaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaExpr::Nil => write!(f, "nil"),
            LuaExpr::True => write!(f, "true"),
            LuaExpr::False => write!(f, "false"),
            LuaExpr::Int(n) => write!(f, "{}", n),
            LuaExpr::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            LuaExpr::Str(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            LuaExpr::Var(name) => write!(f, "{}", name),
            LuaExpr::BinOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            LuaExpr::UnaryOp { op, operand } => write!(f, "({} {})", op, operand),
            LuaExpr::Call { func, args } => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            LuaExpr::MethodCall { obj, method, args } => {
                write!(f, "{}:{}(", obj, method)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            LuaExpr::TableConstructor(fields) => {
                write!(f, "{{")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, "}}")
            }
            LuaExpr::IndexAccess { table, key } => write!(f, "{}[{}]", table, key),
            LuaExpr::FieldAccess { table, field } => write!(f, "{}.{}", table, field),
            LuaExpr::Lambda {
                params,
                vararg,
                body,
            } => {
                write!(f, "function(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                if *vararg {
                    if !params.is_empty() {
                        write!(f, ", ")?;
                    }
                    write!(f, "...")?;
                }
                writeln!(f, ")")?;
                for stmt in body {
                    writeln!(f, "  {}", stmt)?;
                }
                write!(f, "end")
            }
            LuaExpr::Ellipsis => write!(f, "..."),
        }
    }
}
