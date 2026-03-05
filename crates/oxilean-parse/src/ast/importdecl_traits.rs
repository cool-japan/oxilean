//! # ImportDecl - Trait Implementations
//!
//! This module contains trait implementations for `ImportDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImportDecl;
use std::fmt;

impl fmt::Display for ImportDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "import {}", self.dotted_path())
    }
}
