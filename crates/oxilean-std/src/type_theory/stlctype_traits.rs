//! # STLCType - Trait Implementations
//!
//! This module contains trait implementations for `STLCType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::STLCType;
use std::fmt;

impl std::fmt::Display for STLCType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            STLCType::Base(n) => write!(f, "{n}"),
            STLCType::Fun(a, b) => write!(f, "({a} → {b})"),
            STLCType::Prod(a, b) => write!(f, "({a} × {b})"),
            STLCType::Unit => write!(f, "⊤"),
        }
    }
}
