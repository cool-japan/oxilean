//! # EvalErrorKind - Trait Implementations
//!
//! This module contains trait implementations for `EvalErrorKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvalErrorKind;
use std::fmt;

impl fmt::Display for EvalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalErrorKind::DivisionByZero => write!(f, "division by zero"),
            EvalErrorKind::StackOverflow { max_depth } => {
                write!(f, "stack overflow (max depth: {})", max_depth)
            }
            EvalErrorKind::TypeMismatch { expected, got } => {
                write!(f, "type mismatch: expected `{}`, got `{}`", expected, got)
            }
            EvalErrorKind::IndexOutOfBounds { index, len } => {
                write!(f, "index {} out of bounds (length {})", index, len)
            }
            EvalErrorKind::SorryReached { name } => {
                write!(f, "sorry reached in `{}`", name)
            }
            EvalErrorKind::FuelExhausted { limit } => {
                write!(f, "evaluation step limit ({}) exceeded", limit)
            }
            EvalErrorKind::UndefinedVariable { name } => {
                write!(f, "undefined variable `{}`", name)
            }
            EvalErrorKind::UndefinedGlobal { name } => {
                write!(f, "undefined global `{}`", name)
            }
            EvalErrorKind::ArithmeticOverflow { op } => {
                write!(f, "arithmetic overflow in `{}`", op)
            }
            EvalErrorKind::NonExhaustiveMatch { value } => {
                write!(f, "non-exhaustive match on `{}`", value)
            }
            EvalErrorKind::Panic { message } => write!(f, "panic: {}", message),
            EvalErrorKind::Unimplemented { feature } => {
                write!(f, "unimplemented: {}", feature)
            }
            EvalErrorKind::Io { message } => write!(f, "I/O error: {}", message),
            EvalErrorKind::BlackHole { thunk_name } => {
                write!(f, "cyclic evaluation (black hole) in `{}`", thunk_name)
            }
        }
    }
}
