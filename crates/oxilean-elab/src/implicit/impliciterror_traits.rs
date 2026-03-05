//! # ImplicitError - Trait Implementations
//!
//! This module contains trait implementations for `ImplicitError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImplicitError;
use std::fmt;

impl std::fmt::Display for ImplicitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImplicitError::CannotInfer(s) => write!(f, "cannot infer implicit: {}", s),
            ImplicitError::InstanceNotFound(s) => write!(f, "instance not found: {}", s),
            ImplicitError::TooManyImplicits(n) => {
                write!(f, "too many implicit arguments: {}", n)
            }
            ImplicitError::CircularDependency(i, j) => {
                write!(f, "circular dependency between implicits {} and {}", i, j)
            }
        }
    }
}
