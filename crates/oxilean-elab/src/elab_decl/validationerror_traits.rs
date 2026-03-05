//! # ValidationError - Trait Implementations
//!
//! This module contains trait implementations for `ValidationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ValidationError;
use std::fmt;

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "declaration has an empty name"),
            ValidationError::TrivialProof(n) => {
                write!(f, "theorem '{}' has a trivial proof", n)
            }
            ValidationError::NoConstructors(n) => {
                write!(f, "inductive type '{}' has no constructors", n)
            }
            ValidationError::ConflictingAttributes(n, msg) => {
                write!(f, "conflicting attributes on '{}': {}", n, msg)
            }
        }
    }
}
