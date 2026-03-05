//! # DoElabError - Trait Implementations
//!
//! This module contains trait implementations for `DoElabError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DoElabError;
use std::fmt;

impl fmt::Display for DoElabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DoElabError::EmptyDoBlock => write!(f, "empty do block"),
            DoElabError::BindAtEnd(msg) => write!(f, "bind at end of do block: {}", msg),
            DoElabError::NoMonadInstance(msg) => {
                write!(f, "no monad instance found: {}", msg)
            }
            DoElabError::TypeMismatch(msg) => {
                write!(f, "type mismatch in do notation: {}", msg)
            }
            DoElabError::UnknownOperation(msg) => {
                write!(f, "unknown monadic operation: {}", msg)
            }
            DoElabError::NotIterable(msg) => write!(f, "not iterable: {}", msg),
            DoElabError::NoExceptionSupport(msg) => {
                write!(f, "no exception support: {}", msg)
            }
            DoElabError::MaxDepthExceeded(depth) => {
                write!(f, "do-notation nesting depth exceeded: {}", depth)
            }
            DoElabError::InternalError(msg) => {
                write!(f, "internal do-notation error: {}", msg)
            }
        }
    }
}
