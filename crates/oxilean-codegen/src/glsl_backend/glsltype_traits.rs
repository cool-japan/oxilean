//! # GLSLType - Trait Implementations
//!
//! This module contains trait implementations for `GLSLType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GLSLType;
use std::fmt;

impl fmt::Display for GLSLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GLSLType::Array(elem, len) => write!(f, "{}[{}]", elem.keyword(), len),
            other => write!(f, "{}", other.keyword()),
        }
    }
}
