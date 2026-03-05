//! # SynthType - Trait Implementations
//!
//! This module contains trait implementations for `SynthType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SynthType;
use std::fmt;

impl std::fmt::Display for SynthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthType::Base(s) => write!(f, "{}", s),
            SynthType::Unit => write!(f, "()"),
            SynthType::Var(v) => write!(f, "{}", v),
            SynthType::Arrow(a, b) => write!(f, "({} -> {})", a, b),
            SynthType::Product(a, b) => write!(f, "({} * {})", a, b),
            SynthType::Sum(a, b) => write!(f, "({} + {})", a, b),
            SynthType::Forall(v, body) => write!(f, "(∀ {}. {})", v, body),
        }
    }
}
