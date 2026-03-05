//! # MetaProgrammingError - Trait Implementations
//!
//! This module contains trait implementations for `MetaProgrammingError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaProgrammingError;
use std::fmt;

impl std::fmt::Display for MetaProgrammingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaProgrammingError::UndefinedVariable(v) => {
                write!(f, "undefined meta variable: {}", v)
            }
            MetaProgrammingError::MacroFailed { name, reason } => {
                write!(f, "macro '{}' failed: {}", name, reason)
            }
            MetaProgrammingError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "type mismatch: expected '{}', found '{}'",
                    expected, found
                )
            }
            MetaProgrammingError::QuotationDepthExceeded(d) => {
                write!(f, "quotation depth {} exceeded", d)
            }
            MetaProgrammingError::SpliceOutsideQuotation => {
                write!(f, "splice used outside a quotation context")
            }
            MetaProgrammingError::EvalError(msg) => write!(f, "eval error: {}", msg),
        }
    }
}
