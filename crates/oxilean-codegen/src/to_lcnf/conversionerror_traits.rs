//! # ConversionError - Trait Implementations
//!
//! This module contains trait implementations for `ConversionError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ConversionError;
use std::fmt;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::UnsupportedExpr(msg) => {
                write!(f, "Unsupported expression: {}", msg)
            }
            ConversionError::UnboundVariable(name) => {
                write!(f, "Unbound variable: {}", name)
            }
            ConversionError::DepthLimitExceeded(depth) => {
                write!(f, "Depth limit exceeded: {}", depth)
            }
            ConversionError::InvalidBinder(msg) => write!(f, "Invalid binder: {}", msg),
            ConversionError::TypeConversionError(msg) => {
                write!(f, "Type conversion error: {}", msg)
            }
            ConversionError::LambdaLiftError(msg) => {
                write!(f, "Lambda lift error: {}", msg)
            }
            ConversionError::ClosureConversionError(msg) => {
                write!(f, "Closure conversion error: {}", msg)
            }
            ConversionError::AnfConversionError(msg) => {
                write!(f, "ANF conversion error: {}", msg)
            }
            ConversionError::ProofErasureError(msg) => {
                write!(f, "Proof erasure error: {}", msg)
            }
            ConversionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}
