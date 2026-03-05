//! # BaseType - Trait Implementations
//!
//! This module contains trait implementations for `BaseType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BaseType;
use std::fmt;

impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Nat => write!(f, "Nat"),
            BaseType::Bool => write!(f, "Bool"),
            BaseType::Str => write!(f, "String"),
            BaseType::Unit => write!(f, "Unit"),
            BaseType::Named(n) => write!(f, "{}", n),
            BaseType::Pair(a, b) => write!(f, "{} × {}", a, b),
            BaseType::Sum(a, b) => write!(f, "{} + {}", a, b),
        }
    }
}
