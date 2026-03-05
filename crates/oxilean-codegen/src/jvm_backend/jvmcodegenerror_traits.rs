//! # JvmCodegenError - Trait Implementations
//!
//! This module contains trait implementations for `JvmCodegenError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JvmCodegenError;
use std::fmt;

impl fmt::Display for JvmCodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JvmCodegenError::Unsupported(msg) => {
                write!(f, "JVM codegen unsupported: {}", msg)
            }
            JvmCodegenError::UnknownVar(v) => write!(f, "JVM codegen unknown var: {}", v),
            JvmCodegenError::Internal(msg) => write!(f, "JVM codegen internal: {}", msg),
        }
    }
}
