//! # QualifiedNameExt - Trait Implementations
//!
//! This module contains trait implementations for `QualifiedNameExt`.
//!
//! ## Implemented Traits
//!
//! - `FromStr`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::QualifiedNameExt;

impl std::str::FromStr for QualifiedNameExt {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            parts: s.split('.').map(|p| p.to_string()).collect(),
        })
    }
}

impl std::fmt::Display for QualifiedNameExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}
