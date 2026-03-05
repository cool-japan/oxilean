//! # AdvDeriveError - Trait Implementations
//!
//! This module contains trait implementations for `AdvDeriveError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AdvDeriveError;
use std::fmt;

impl fmt::Display for AdvDeriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvDeriveError::CannotDerive {
                class,
                type_name,
                reason,
            } => {
                write!(f, "cannot derive {} for {}: {}", class, type_name, reason)
            }
            AdvDeriveError::MissingFieldInstance {
                class,
                field,
                field_type,
            } => {
                write!(
                    f,
                    "missing {} instance for field '{}' of type {}",
                    class, field, field_type
                )
            }
            AdvDeriveError::RecursiveType { class, type_name } => {
                write!(
                    f,
                    "cannot derive {} for recursive type {}",
                    class, type_name
                )
            }
            AdvDeriveError::EmptyType { type_name } => {
                write!(f, "type {} has no constructors", type_name)
            }
            AdvDeriveError::NoHandler { class } => {
                write!(f, "no handler registered for class {}", class)
            }
            AdvDeriveError::Internal(msg) => {
                write!(f, "internal derivation error: {}", msg)
            }
        }
    }
}
