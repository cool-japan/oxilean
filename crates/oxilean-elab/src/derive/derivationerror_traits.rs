//! # DerivationError - Trait Implementations
//!
//! This module contains trait implementations for `DerivationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DerivationError;
use std::fmt;

impl fmt::Display for DerivationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DerivationError::CannotDerive(s) => write!(f, "cannot derive: {}", s),
            DerivationError::MissingInstance(s) => write!(f, "missing instance: {}", s),
            DerivationError::RecursiveType(s) => write!(f, "recursive type: {}", s),
            DerivationError::Other(s) => write!(f, "derivation error: {}", s),
        }
    }
}
