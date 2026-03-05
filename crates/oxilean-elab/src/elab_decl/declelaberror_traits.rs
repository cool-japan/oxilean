//! # DeclElabError - Trait Implementations
//!
//! This module contains trait implementations for `DeclElabError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::elaborate::{elaborate_expr, elaborate_with_expected_type, ElabError};
use std::fmt;

use super::types::DeclElabError;

impl fmt::Display for DeclElabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeclElabError::ElabError(msg) => write!(f, "elaboration error: {}", msg),
            DeclElabError::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {}, got {}", expected, got)
            }
            DeclElabError::DuplicateName(name) => write!(f, "duplicate name: {}", name),
            DeclElabError::InvalidRecursion(msg) => {
                write!(f, "invalid recursion: {}", msg)
            }
            DeclElabError::MissingType(msg) => write!(f, "missing type: {}", msg),
            DeclElabError::UnsupportedDecl(msg) => {
                write!(f, "unsupported declaration: {}", msg)
            }
        }
    }
}

impl From<ElabError> for DeclElabError {
    fn from(e: ElabError) -> Self {
        DeclElabError::ElabError(format!("{:?}", e))
    }
}
