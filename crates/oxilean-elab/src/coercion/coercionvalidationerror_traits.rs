//! # CoercionValidationError - Trait Implementations
//!
//! This module contains trait implementations for `CoercionValidationError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionValidationError;
use std::fmt;

#[allow(dead_code)]
impl std::fmt::Display for CoercionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoercionValidationError::CyclicCoercion(names) => {
                write!(f, "cyclic coercion chain: ")?;
                for (i, n) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, " -> ")?;
                    }
                    write!(f, "{}", n)?;
                }
                Ok(())
            }
            CoercionValidationError::AmbiguousPath { from, to, count } => {
                write!(
                    f,
                    "ambiguous coercion path from {} to {}: {} paths found",
                    from, to, count
                )
            }
            CoercionValidationError::IncompatibleTypes {
                coerce,
                expected,
                got,
            } => {
                write!(
                    f,
                    "coercion {} has incompatible type: expected {}, got {}",
                    coerce, expected, got
                )
            }
            CoercionValidationError::MissingCoercionFn(name) => {
                write!(f, "coercion function not found: {}", name)
            }
            CoercionValidationError::InvalidPriority { coerce, priority } => {
                write!(
                    f,
                    "coercion {} has invalid priority {}: must be >= 0",
                    coerce, priority
                )
            }
        }
    }
}
