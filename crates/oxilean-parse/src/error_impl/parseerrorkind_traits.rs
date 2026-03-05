//! # ParseErrorKind - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorKind;
use std::fmt;

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnexpectedToken { expected, got } => {
                write!(f, "expected ")?;
                if expected.is_empty() {
                    write!(f, "something")?;
                } else if expected.len() == 1 {
                    write!(f, "{}", expected[0])?;
                } else {
                    write!(f, "one of: {}", expected.join(", "))?;
                }
                write!(f, ", but got {}", got)
            }
            ParseErrorKind::UnexpectedEof { expected } => {
                write!(f, "unexpected end of file, expected ")?;
                if expected.is_empty() {
                    write!(f, "more input")
                } else if expected.len() == 1 {
                    write!(f, "{}", expected[0])
                } else {
                    write!(f, "one of: {}", expected.join(", "))
                }
            }
            ParseErrorKind::InvalidSyntax(msg) => write!(f, "invalid syntax: {}", msg),
            ParseErrorKind::DuplicateDeclaration(name) => {
                write!(f, "duplicate declaration: {}", name)
            }
            ParseErrorKind::InvalidBinder(msg) => write!(f, "invalid binder: {}", msg),
            ParseErrorKind::InvalidPattern(msg) => write!(f, "invalid pattern: {}", msg),
            ParseErrorKind::InvalidUniverse(msg) => {
                write!(f, "invalid universe: {}", msg)
            }
            ParseErrorKind::Other(msg) => write!(f, "{}", msg),
        }
    }
}
