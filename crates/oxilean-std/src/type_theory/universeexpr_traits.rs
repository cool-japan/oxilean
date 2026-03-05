//! # UniverseExpr - Trait Implementations
//!
//! This module contains trait implementations for `UniverseExpr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UniverseExpr;
use std::fmt;

impl std::fmt::Display for UniverseExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniverseExpr::Lit(n) => write!(f, "{n}"),
            UniverseExpr::Var(v) => write!(f, "{v}"),
            UniverseExpr::Succ(u) => write!(f, "succ({u})"),
            UniverseExpr::Max(a, b) => write!(f, "max({a}, {b})"),
            UniverseExpr::IMax(a, b) => write!(f, "imax({a}, {b})"),
        }
    }
}
