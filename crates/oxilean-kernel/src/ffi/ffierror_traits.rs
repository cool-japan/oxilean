//! # FfiError - Trait Implementations
//!
//! This module contains trait implementations for `FfiError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FfiError;
use std::fmt;

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::SymbolNotFound(s) => write!(f, "FFI symbol not found: {}", s),
            FfiError::LibraryNotFound(s) => write!(f, "FFI library not found: {}", s),
            FfiError::TypeMismatch(s) => write!(f, "FFI type mismatch: {}", s),
            FfiError::ValueOutOfRange(s) => write!(f, "FFI value out of range: {}", s),
            FfiError::InvalidSignature(s) => write!(f, "FFI invalid signature: {}", s),
            FfiError::DuplicateSymbol(s) => write!(f, "FFI duplicate symbol: {}", s),
            FfiError::ValidationFailed(s) => write!(f, "FFI validation failed: {}", s),
        }
    }
}
