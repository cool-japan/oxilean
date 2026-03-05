//! # LcnfType - Trait Implementations
//!
//! This module contains trait implementations for `LcnfType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LcnfType;
use std::fmt;

impl fmt::Display for LcnfType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcnfType::Erased => write!(f, "erased"),
            LcnfType::Var(name) => write!(f, "{}", name),
            LcnfType::Fun(params, ret) => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            LcnfType::Ctor(name, args) => {
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "<")?;
                    for (i, a) in args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", a)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            LcnfType::Object => write!(f, "object"),
            LcnfType::Nat => write!(f, "nat"),
            LcnfType::LcnfString => write!(f, "string"),
            LcnfType::Unit => write!(f, "unit"),
            LcnfType::Irrelevant => write!(f, "irrelevant"),
        }
    }
}
