//! # DeclSummary - Trait Implementations
//!
//! This module contains trait implementations for `DeclSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeclSummary;
use std::fmt;

impl fmt::Display for DeclSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} : {}", self.kind, self.name, self.type_sig)
    }
}
