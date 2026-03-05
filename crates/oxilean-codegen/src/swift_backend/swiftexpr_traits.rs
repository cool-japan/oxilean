//! # SwiftExpr - Trait Implementations
//!
//! This module contains trait implementations for `SwiftExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SwiftExpr;
use std::fmt;

impl fmt::Display for SwiftExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwiftExpr::SwiftLitExpr(lit) => write!(f, "{}", lit),
            SwiftExpr::SwiftVar(name) => write!(f, "{}", name),
            SwiftExpr::SwiftSelf => write!(f, "self"),
            SwiftExpr::SwiftSuper => write!(f, "super"),
            SwiftExpr::SwiftCall { callee, args } => {
                write!(f, "{}(", callee)?;
                for (i, (label, expr)) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if label.is_empty() {
                        write!(f, "{}", expr)?;
                    } else {
                        write!(f, "{}: {}", label, expr)?;
                    }
                }
                write!(f, ")")
            }
            SwiftExpr::SwiftBinOp { op, lhs, rhs } => {
                write!(f, "({} {} {})", lhs, op, rhs)
            }
            SwiftExpr::SwiftMember(obj, field) => write!(f, "{}.{}", obj, field),
            SwiftExpr::SwiftSubscript(arr, idx) => write!(f, "{}[{}]", arr, idx),
            SwiftExpr::SwiftUnary(op, operand) => write!(f, "{}{}", op, operand),
            SwiftExpr::SwiftTernary(cond, then_e, else_e) => {
                write!(f, "({} ? {} : {})", cond, then_e, else_e)
            }
            SwiftExpr::SwiftOptionalChain(obj, field) => write!(f, "{}?.{}", obj, field),
            SwiftExpr::SwiftForceUnwrap(expr) => write!(f, "{}!", expr),
            SwiftExpr::SwiftArrayLit(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            SwiftExpr::SwiftDictLit(pairs) => {
                if pairs.is_empty() {
                    return write!(f, "[:]");
                }
                write!(f, "[")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "]")
            }
            SwiftExpr::SwiftTupleLit(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            SwiftExpr::SwiftAs(expr, ty) => write!(f, "({} as {})", expr, ty),
            SwiftExpr::SwiftTry(expr) => write!(f, "try {}", expr),
            SwiftExpr::SwiftAwait(expr) => write!(f, "await {}", expr),
            SwiftExpr::SwiftClosure {
                params,
                return_type,
                body,
            } => {
                write!(f, "{{ ")?;
                if !params.is_empty() {
                    for (i, (name, ty)) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        if let Some(t) = ty {
                            write!(f, "{}: {}", name, t)?;
                        } else {
                            write!(f, "{}", name)?;
                        }
                    }
                    if let Some(ret) = return_type {
                        write!(f, " -> {} ", ret)?;
                    }
                    writeln!(f, "in")?;
                }
                for stmt in body {
                    writeln!(f, "    {}", stmt)?;
                }
                write!(f, "}}")
            }
            SwiftExpr::SwiftSwitchExpr { subject, arms } => {
                writeln!(f, "switch {} {{", subject)?;
                for (pat, expr) in arms {
                    writeln!(f, "    case {}: {}", pat, expr)?;
                }
                write!(f, "}}")
            }
        }
    }
}
