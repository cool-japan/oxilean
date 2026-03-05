//! # KotlinExpr - Trait Implementations
//!
//! This module contains trait implementations for `KotlinExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_stmt, fmt_stmts};
use super::types::KotlinExpr;
use std::fmt;

impl fmt::Display for KotlinExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KotlinExpr::Var(name) => write!(f, "{}", name),
            KotlinExpr::Lit(lit) => write!(f, "{}", lit),
            KotlinExpr::Call(callee, args) => {
                write!(f, "{}(", callee)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            KotlinExpr::BinOp(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            KotlinExpr::Member(base, field) => write!(f, "{}.{}", base, field),
            KotlinExpr::Index(arr, idx) => write!(f, "{}[{}]", arr, idx),
            KotlinExpr::Unary(op, operand) => write!(f, "{}({})", op, operand),
            KotlinExpr::Lambda(params, body) => {
                write!(f, "{{ ")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                if !params.is_empty() {
                    write!(f, " -> ")?;
                }
                write!(f, "{} }}", body)
            }
            KotlinExpr::When(scrutinee, branches, default) => {
                writeln!(f, "when ({}) {{", scrutinee)?;
                for branch in branches {
                    writeln!(f, "        {} -> {}", branch.condition, branch.body)?;
                }
                if let Some(def) = default {
                    writeln!(f, "        else -> {}", def)?;
                }
                write!(f, "    }}")
            }
            KotlinExpr::Elvis(lhs, rhs) => write!(f, "({} ?: {})", lhs, rhs),
        }
    }
}
