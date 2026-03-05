//! # KernelError - Trait Implementations
//!
//! This module contains trait implementations for `KernelError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//! - `From`
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KernelError;
use std::fmt;

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::TypeMismatch {
                expected,
                got,
                context,
            } => {
                write!(
                    f,
                    "type mismatch in {}: expected {}, got {}",
                    context, expected, got
                )
            }
            KernelError::UnboundVariable(idx) => write!(f, "unbound variable: #{}", idx),
            KernelError::UnknownConstant(name) => write!(f, "unknown constant: {}", name),
            KernelError::UniverseInconsistency { lhs, rhs } => {
                write!(f, "universe inconsistency: {} vs {}", lhs, rhs)
            }
            KernelError::InvalidInductive(msg) => {
                write!(f, "invalid inductive type: {}", msg)
            }
            KernelError::InvalidRecursor(msg) => write!(f, "invalid recursor: {}", msg),
            KernelError::NotASort(expr) => write!(f, "not a sort: {}", expr),
            KernelError::NotAFunction(expr) => write!(f, "not a function type: {}", expr),
            KernelError::InductiveError(msg) => write!(f, "inductive error: {}", msg),
            KernelError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for KernelError {}

impl From<String> for KernelError {
    fn from(s: String) -> Self {
        KernelError::Other(s)
    }
}

impl From<&str> for KernelError {
    fn from(s: &str) -> Self {
        KernelError::Other(s.to_string())
    }
}
