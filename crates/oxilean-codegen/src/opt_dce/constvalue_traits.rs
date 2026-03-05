//! # ConstValue - Trait Implementations
//!
//! This module contains trait implementations for `ConstValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ConstValue;
use std::fmt;

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstValue::Lit(lit) => write!(f, "const({})", lit),
            ConstValue::Ctor(name, tag, args) => {
                write!(f, "ctor({}#{}", name, tag)?;
                for a in args {
                    write!(f, " {}", a)?;
                }
                write!(f, ")")
            }
            ConstValue::Unknown => write!(f, "unknown"),
        }
    }
}
