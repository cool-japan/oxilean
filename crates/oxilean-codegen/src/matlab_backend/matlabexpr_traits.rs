//! # MatlabExpr - Trait Implementations
//!
//! This module contains trait implementations for `MatlabExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatlabExpr;
use std::fmt;

impl std::fmt::Display for MatlabExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatlabExpr::Lit(lit) => write!(f, "{}", lit),
            MatlabExpr::Var(name) => write!(f, "{}", name),
            MatlabExpr::MatrixLit(rows) => {
                write!(f, "[")?;
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    let elems: Vec<String> = row.iter().map(|e| e.to_string()).collect();
                    write!(f, "{}", elems.join(", "))?;
                }
                write!(f, "]")
            }
            MatlabExpr::CellLit(rows) => {
                write!(f, "{{")?;
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    let elems: Vec<String> = row.iter().map(|e| e.to_string()).collect();
                    write!(f, "{}", elems.join(", "))?;
                }
                write!(f, "}}")
            }
            MatlabExpr::ColonRange { start, step, end } => {
                if let Some(step_expr) = step {
                    write!(f, "{}:{}:{}", start, step_expr, end)
                } else {
                    write!(f, "{}:{}", start, end)
                }
            }
            MatlabExpr::Call(func, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", func, args_str.join(", "))
            }
            MatlabExpr::Index {
                obj,
                indices,
                cell_index,
            } => {
                let idx_str: Vec<String> = indices.iter().map(|i| i.to_string()).collect();
                let (open, close) = if *cell_index { ("{", "}") } else { ("(", ")") };
                write!(f, "{}{}{}{}", obj, open, idx_str.join(", "), close)
            }
            MatlabExpr::FieldAccess(obj, field) => write!(f, "{}.{}", obj, field),
            MatlabExpr::BinaryOp(op, lhs, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            MatlabExpr::UnaryOp(op, operand, postfix) => {
                if *postfix {
                    write!(f, "{}{}", operand, op)
                } else {
                    write!(f, "{}{}", op, operand)
                }
            }
            MatlabExpr::IfExpr(cond, then_expr, else_expr) => {
                write!(f, "({{{0}; {1}}}{{{2}+1}})", else_expr, then_expr, cond)
            }
            MatlabExpr::AnonFunc(params, body) => {
                write!(f, "@({}) {}", params.join(", "), body)
            }
            MatlabExpr::End => write!(f, "end"),
            MatlabExpr::Colon => write!(f, ":"),
            MatlabExpr::Nargin => write!(f, "nargin"),
            MatlabExpr::Nargout => write!(f, "nargout"),
        }
    }
}
