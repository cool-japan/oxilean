//! # LcnfExpr - Trait Implementations
//!
//! This module contains trait implementations for `LcnfExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LcnfExpr;
use std::fmt;

impl fmt::Display for LcnfExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                writeln!(f, "let {} ({}) : {} := {};", id, name, ty, value)?;
                write!(f, "{}", body)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                writeln!(f, "case {} of", scrutinee)?;
                for alt in alts {
                    write!(f, "  | {}", alt.ctor_name)?;
                    for p in &alt.params {
                        write!(f, " {}", p.id)?;
                    }
                    writeln!(f, " => {}", alt.body)?;
                }
                if let Some(def) = default {
                    writeln!(f, "  | _ => {}", def)?;
                }
                Ok(())
            }
            LcnfExpr::Return(arg) => write!(f, "return {}", arg),
            LcnfExpr::Unreachable => write!(f, "unreachable"),
            LcnfExpr::TailCall(func, args) => {
                write!(f, "tailcall {}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
        }
    }
}
