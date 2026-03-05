//! # LcnfArg - Trait Implementations
//!
//! This module contains trait implementations for `LcnfArg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LcnfArg;
use std::fmt;

impl fmt::Display for LcnfArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcnfArg::Var(id) => write!(f, "{}", id),
            LcnfArg::Lit(lit) => write!(f, "{}", lit),
            LcnfArg::Erased => write!(f, "erased"),
            LcnfArg::Type(ty) => write!(f, "@{}", ty),
        }
    }
}
