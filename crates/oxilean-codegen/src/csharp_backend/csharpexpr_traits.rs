//! # CSharpExpr - Trait Implementations
//!
//! This module contains trait implementations for `CSharpExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{CSharpExpr, CSharpInterpolationPart};
use std::fmt;

impl fmt::Display for CSharpExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSharpExpr::Lit(lit) => write!(f, "{}", lit),
            CSharpExpr::Var(name) => write!(f, "{}", name),
            CSharpExpr::Null => write!(f, "null"),
            CSharpExpr::Default(None) => write!(f, "default"),
            CSharpExpr::Default(Some(ty)) => write!(f, "default({})", ty),
            CSharpExpr::NameOf(name) => write!(f, "nameof({})", name),
            CSharpExpr::TypeOf(ty) => write!(f, "typeof({})", ty),
            CSharpExpr::BinOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            CSharpExpr::UnaryOp { op, operand } => write!(f, "{}({})", op, operand),
            CSharpExpr::Call { callee, args } => {
                write!(f, "{}(", callee)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            CSharpExpr::MethodCall {
                receiver,
                method,
                type_args,
                args,
            } => {
                write!(f, "{}.{}", receiver, method)?;
                if !type_args.is_empty() {
                    write!(f, "<")?;
                    for (i, t) in type_args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", t)?;
                    }
                    write!(f, ">")?;
                }
                write!(f, "(")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            CSharpExpr::New { ty, args } => {
                write!(f, "new {}(", ty)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            CSharpExpr::Lambda { params, body } => {
                if params.len() == 1 && params[0].1.is_none() {
                    write!(f, "{} => {}", params[0].0, body)
                } else {
                    write!(f, "(")?;
                    for (i, (name, ty)) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        if let Some(t) = ty {
                            write!(f, "{} {}", t, name)?;
                        } else {
                            write!(f, "{}", name)?;
                        }
                    }
                    write!(f, ") => {}", body)
                }
            }
            CSharpExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                write!(f, "({} ? {} : {})", cond, then_expr, else_expr)
            }
            CSharpExpr::Await(expr) => write!(f, "await {}", expr),
            CSharpExpr::Throw(expr) => write!(f, "throw {}", expr),
            CSharpExpr::Is { expr, pattern } => write!(f, "({} is {})", expr, pattern),
            CSharpExpr::As { expr, ty } => write!(f, "({} as {})", expr, ty),
            CSharpExpr::Member(base, field) => write!(f, "{}.{}", base, field),
            CSharpExpr::Index(base, idx) => write!(f, "{}[{}]", base, idx),
            CSharpExpr::SwitchExpr { scrutinee, arms } => {
                write!(f, "{} switch\n    {{", scrutinee)?;
                for arm in arms {
                    if let Some(guard) = &arm.guard {
                        write!(
                            f,
                            "\n        {} when {} => {}",
                            arm.pattern, guard, arm.body
                        )?;
                    } else {
                        write!(f, "\n        {} => {}", arm.pattern, arm.body)?;
                    }
                    write!(f, ",")?;
                }
                write!(f, "\n    }}")
            }
            CSharpExpr::Interpolated(parts) => {
                write!(f, "$\"")?;
                for part in parts {
                    match part {
                        CSharpInterpolationPart::Text(s) => write!(f, "{}", s)?,
                        CSharpInterpolationPart::Expr(e) => write!(f, "{{{}}}", e)?,
                        CSharpInterpolationPart::ExprFmt(e, fmt_spec) => {
                            write!(f, "{{{}:{}}}", e, fmt_spec)?
                        }
                    }
                }
                write!(f, "\"")
            }
            CSharpExpr::CollectionExpr(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
        }
    }
}
