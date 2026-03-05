//! # ScopeDecl - Trait Implementations
//!
//! This module contains trait implementations for `ScopeDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScopeDecl;
use std::fmt;

impl fmt::Display for ScopeDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeDecl::Section(n) => write!(f, "section {}", n),
            ScopeDecl::Namespace(n) => write!(f, "namespace {}", n),
            ScopeDecl::End(n) => write!(f, "end {}", n),
            ScopeDecl::Open(names) => write!(f, "open {}", names.join(" ")),
        }
    }
}
