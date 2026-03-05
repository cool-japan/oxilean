//! # ExecError - Trait Implementations
//!
//! This module contains trait implementations for `ExecError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExecError;
use std::fmt;

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecError::StackUnderflow => write!(f, "stack underflow"),
            ExecError::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected {expected}, got {got}")
            }
            ExecError::DivisionByZero => write!(f, "division by zero"),
            ExecError::Unreachable => write!(f, "unreachable executed"),
            ExecError::OutOfBoundsMemory(addr) => {
                write!(f, "out-of-bounds memory access at {addr}")
            }
            ExecError::UndefinedLocal(idx) => write!(f, "undefined local {idx}"),
            ExecError::UndefinedGlobal(idx) => write!(f, "undefined global {idx}"),
            ExecError::CallStackOverflow => write!(f, "call stack overflow"),
            ExecError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}
