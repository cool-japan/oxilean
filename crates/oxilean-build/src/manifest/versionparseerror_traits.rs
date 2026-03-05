//! # VersionParseError - Trait Implementations
//!
//! This module contains trait implementations for `VersionParseError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionParseError;
use std::fmt;

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty version string"),
            Self::MissingComponent(name) => {
                write!(f, "missing version component: {}", name)
            }
            Self::InvalidNumber(s) => write!(f, "invalid number in version: {}", s),
            Self::UnexpectedChar(c) => {
                write!(f, "unexpected character in version: {:?}", c)
            }
        }
    }
}
