//! # WGSLTypeError - Trait Implementations
//!
//! This module contains trait implementations for `WGSLTypeError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLTypeError;
use std::fmt;

impl fmt::Display for WGSLTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WGSLTypeError::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            WGSLTypeError::InvalidOperandType { op, ty } => {
                write!(f, "invalid operand type {} for operation '{}'", ty, op)
            }
            WGSLTypeError::SwizzleOutOfRange { component, ty } => {
                write!(
                    f,
                    "swizzle component '{}' out of range for {}",
                    component, ty
                )
            }
            WGSLTypeError::NonShareableBinding { ty } => {
                write!(f, "type {} cannot be used in a resource binding", ty)
            }
        }
    }
}
