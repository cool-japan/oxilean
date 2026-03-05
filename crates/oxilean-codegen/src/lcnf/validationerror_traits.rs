//! # ValidationError - Trait Implementations
//!
//! This module contains trait implementations for `ValidationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ValidationError;
use std::fmt;

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::UnboundVariable(id) => write!(f, "Unbound variable: {}", id),
            ValidationError::DuplicateBinding(id) => {
                write!(f, "Duplicate binding: {}", id)
            }
            ValidationError::EmptyCase => write!(f, "Empty case expression"),
            ValidationError::InvalidTag(name, tag) => {
                write!(f, "Invalid tag {} for constructor {}", tag, name)
            }
            ValidationError::NonAtomicArgument => {
                write!(f, "Non-atomic argument (violates ANF invariant)")
            }
        }
    }
}

impl std::error::Error for ValidationError {}
