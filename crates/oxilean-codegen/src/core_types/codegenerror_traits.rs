//! # CodegenError - Trait Implementations
//!
//! This module contains trait implementations for `CodegenError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CodegenError;
use std::fmt;

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::UnsupportedExpression(msg) => {
                write!(f, "Unsupported expression: {}", msg)
            }
            CodegenError::UnsupportedType(msg) => write!(f, "Unsupported type: {}", msg),
            CodegenError::UnboundVariable(name) => {
                write!(f, "Unbound variable: {}", name)
            }
            CodegenError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            CodegenError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            CodegenError::StructNotFound(name) => write!(f, "Struct not found: {}", name),
            CodegenError::FieldNotFound { struct_name, field } => {
                write!(f, "Field {} not found in struct {}", field, struct_name)
            }
            CodegenError::InternalError(msg) => {
                write!(f, "Internal code generation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CodegenError {}
