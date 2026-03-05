//! # LcnfLetValue - Trait Implementations
//!
//! This module contains trait implementations for `LcnfLetValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LcnfLetValue;
use std::fmt;

impl fmt::Display for LcnfLetValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcnfLetValue::App(func, args) => {
                write!(f, "{}(", func)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
            LcnfLetValue::Proj(name, idx, var) => write!(f, "{}.{} {}", name, idx, var),
            LcnfLetValue::Ctor(name, tag, args) => {
                write!(f, "{}#{}", name, tag)?;
                if !args.is_empty() {
                    write!(f, "(")?;
                    for (i, a) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", a)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            LcnfLetValue::Lit(lit) => write!(f, "{}", lit),
            LcnfLetValue::Erased => write!(f, "erased"),
            LcnfLetValue::FVar(id) => write!(f, "{}", id),
            LcnfLetValue::Reset(var) => write!(f, "reset({})", var),
            LcnfLetValue::Reuse(slot, name, tag, args) => {
                write!(f, "reuse({}, {}#{}(", slot, name, tag)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, "))")
            }
        }
    }
}
