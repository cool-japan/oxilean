//! # CtfeError - Trait Implementations
//!
//! This module contains trait implementations for `CtfeError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeError;
use std::fmt;

impl fmt::Display for CtfeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtfeError::DivisionByZero => write!(f, "division by zero"),
            CtfeError::IndexOutOfBounds { index, length } => {
                write!(f, "index {} out of bounds (length {})", index, length)
            }
            CtfeError::StackOverflow { depth } => {
                write!(f, "stack overflow at depth {}", depth)
            }
            CtfeError::NonConstant { reason } => write!(f, "non-constant: {}", reason),
            CtfeError::Timeout { fuel_used } => {
                write!(f, "timeout after {} steps", fuel_used)
            }
            CtfeError::Overflow { op } => write!(f, "integer overflow in {}", op),
            CtfeError::BadProjection { field } => {
                write!(f, "cannot project field {} from non-constructor", field)
            }
            CtfeError::NonExhaustiveMatch => write!(f, "non-exhaustive pattern match"),
        }
    }
}
