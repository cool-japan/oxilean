//! # InferError - Trait Implementations
//!
//! This module contains trait implementations for `InferError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InferError;

impl std::fmt::Display for InferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferError::UnboundBVar(i) => write!(f, "unbound bound variable #{}", i),
            InferError::UnknownFVar(id) => write!(f, "unknown free variable fvar_{}", id),
            InferError::UnknownConst(n) => write!(f, "unknown constant '{}'", n),
            InferError::NotAFunction(s) => write!(f, "not a function: {}", s),
            InferError::ExpectedSort(s) => write!(f, "expected sort, got: {}", s),
            InferError::DepthExceeded => write!(f, "type inference depth exceeded"),
            InferError::Other(s) => write!(f, "{}", s),
        }
    }
}
