//! # DeclKind - Trait Implementations
//!
//! This module contains trait implementations for `DeclKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeclKind;
use std::fmt;

impl std::fmt::Display for DeclKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.keyword())
    }
}
