//! # InferError - Trait Implementations
//!
//! This module contains trait implementations for `InferError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InferError;
use std::fmt;

impl std::fmt::Display for InferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferError::UnboundFVar(id) => write!(f, "Unbound free variable #{}", id),
            InferError::UnknownConst(n) => write!(f, "Unknown constant: {}", n),
            InferError::ExpectedFunctionType(s) => {
                write!(f, "Expected function type: {}", s)
            }
            InferError::ExpectedSort(s) => write!(f, "Expected sort: {}", s),
            InferError::ProjectionError(s) => write!(f, "Projection error: {}", s),
            InferError::UnificationFailure(s) => write!(f, "Unification failure: {}", s),
            InferError::UnsolvedMetavar(id) => write!(f, "Unsolved metavariable #{}", id),
            InferError::FuelExhausted => {
                write!(f, "Fuel exhausted during type inference")
            }
        }
    }
}

impl std::error::Error for InferError {}
