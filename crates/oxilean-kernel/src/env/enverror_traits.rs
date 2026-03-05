//! # EnvError - Trait Implementations
//!
//! This module contains trait implementations for `EnvError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EnvError;

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::DuplicateDeclaration(name) => {
                write!(f, "duplicate declaration: {}", name)
            }
            EnvError::NotFound(name) => write!(f, "declaration not found: {}", name),
        }
    }
}

impl std::error::Error for EnvError {}
