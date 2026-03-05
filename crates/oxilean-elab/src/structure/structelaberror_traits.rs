//! # StructElabError - Trait Implementations
//!
//! This module contains trait implementations for `StructElabError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StructElabError;
use std::fmt;

impl std::fmt::Display for StructElabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructElabError::ParentNotFound(s) => write!(f, "parent not found: {}", s),
            StructElabError::DuplicateField(s) => write!(f, "duplicate field: {}", s),
            StructElabError::FieldTypeMismatch(s) => {
                write!(f, "field type mismatch: {}", s)
            }
            StructElabError::CircularInheritance(s) => {
                write!(f, "circular inheritance: {}", s)
            }
            StructElabError::InvalidClass(s) => write!(f, "invalid class: {}", s),
            StructElabError::Other(s) => write!(f, "{}", s),
        }
    }
}
