//! # BinderValidationError - Trait Implementations
//!
//! This module contains trait implementations for `BinderValidationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BinderValidationError;
use std::fmt;

impl std::fmt::Display for BinderValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinderValidationError::EmptyName => write!(f, "binder name cannot be empty"),
            BinderValidationError::ReservedName(n) => {
                write!(f, "binder name '{}' is reserved", n)
            }
            BinderValidationError::InvalidTypeAnnotation(msg) => {
                write!(f, "invalid type annotation: {}", msg)
            }
            BinderValidationError::AmbiguousMixedBinders => {
                write!(f, "mixing anonymous and named binders is ambiguous")
            }
            BinderValidationError::InstanceBinderWithoutType => {
                write!(f, "instance binder must have a type annotation")
            }
            BinderValidationError::Other(msg) => write!(f, "binder error: {}", msg),
        }
    }
}
