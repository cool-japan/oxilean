//! # CodegenTarget - Trait Implementations
//!
//! This module contains trait implementations for `CodegenTarget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CodegenTarget;
use std::fmt;

impl fmt::Display for CodegenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenTarget::Rust => write!(f, "Rust"),
            CodegenTarget::C => write!(f, "C"),
            CodegenTarget::LlvmIr => write!(f, "LLVM IR"),
            CodegenTarget::Interpreter => write!(f, "Interpreter"),
        }
    }
}
