//! # IoErrorKind - Trait Implementations
//!
//! This module contains trait implementations for `IoErrorKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoErrorKind;
use std::fmt;

impl std::fmt::Display for IoErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IoErrorKind::NotFound => "not found",
            IoErrorKind::PermissionDenied => "permission denied",
            IoErrorKind::ConnectionRefused => "connection refused",
            IoErrorKind::TimedOut => "timed out",
            IoErrorKind::UnexpectedEof => "unexpected end of file",
            IoErrorKind::WriteZero => "write zero",
            IoErrorKind::InvalidData => "invalid data",
            IoErrorKind::Other => "other",
        };
        write!(f, "{}", s)
    }
}
