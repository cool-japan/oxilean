//! # InductiveError - Trait Implementations
//!
//! This module contains trait implementations for `InductiveError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InductiveError;

impl std::fmt::Display for InductiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InductiveError::AlreadyDefined(n) => {
                write!(f, "type '{}' is already defined", n)
            }
            InductiveError::UndefinedType(n) => write!(f, "undefined type '{}'", n),
            InductiveError::InvalidConstructorType(n) => {
                write!(f, "invalid constructor type for '{}'", n)
            }
            InductiveError::UniverseTooSmall(s) => write!(f, "universe too small: {}", s),
            InductiveError::NonStrictlyPositive(n) => {
                write!(f, "non-strictly-positive occurrence of '{}'", n)
            }
            InductiveError::Other(s) => write!(f, "inductive error: {}", s),
        }
    }
}
