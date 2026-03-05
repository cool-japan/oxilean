//! # GLSLVersion - Trait Implementations
//!
//! This module contains trait implementations for `GLSLVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GLSLVersion;
use std::fmt;

impl fmt::Display for GLSLVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GLSL {}", self.number())
    }
}
