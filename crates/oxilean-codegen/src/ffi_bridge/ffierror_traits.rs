//! # FfiError - Trait Implementations
//!
//! This module contains trait implementations for `FfiError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiError;
use std::fmt;

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::UnsupportedType(ty) => {
                write!(f, "Unsupported type at FFI boundary: {}", ty)
            }
            FfiError::InvalidCallingConvention(cc, reason) => {
                write!(f, "Invalid calling convention {}: {}", cc, reason)
            }
            FfiError::ParamCountMismatch { expected, found } => {
                write!(
                    f,
                    "Parameter count mismatch: expected {}, found {}",
                    expected, found
                )
            }
            FfiError::TypeMismatch {
                param,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Type mismatch for parameter {}: expected {}, found {}",
                    param, expected, found
                )
            }
            FfiError::StructTooLarge { name, size } => {
                write!(
                    f,
                    "Struct {} is too large ({} bytes) to pass by value",
                    name, size
                )
            }
            FfiError::RecursiveType(name) => {
                write!(f, "Recursive type {} cannot cross FFI boundary", name)
            }
            FfiError::Other(msg) => write!(f, "FFI error: {}", msg),
        }
    }
}

impl std::error::Error for FfiError {}
