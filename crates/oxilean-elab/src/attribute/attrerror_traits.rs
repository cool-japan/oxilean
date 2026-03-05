//! # AttrError - Trait Implementations
//!
//! This module contains trait implementations for `AttrError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AttrError;
use std::fmt;

impl std::fmt::Display for AttrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrError::UnknownAttribute(name) => write!(f, "unknown attribute: {}", name),
            AttrError::InvalidArgs(msg) => {
                write!(f, "invalid attribute arguments: {}", msg)
            }
            AttrError::DuplicateAttribute(name) => {
                write!(f, "duplicate attribute: {}", name)
            }
            AttrError::IncompatibleAttributes(a, b) => {
                write!(f, "incompatible attributes: {} and {}", a, b)
            }
            AttrError::Other(msg) => write!(f, "attribute error: {}", msg),
        }
    }
}
