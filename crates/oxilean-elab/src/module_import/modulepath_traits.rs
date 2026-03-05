//! # ModulePath - Trait Implementations
//!
//! This module contains trait implementations for `ModulePath`.
//!
//! ## Implemented Traits
//!
//! - `FromStr`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ModulePath;
use std::fmt;

impl std::str::FromStr for ModulePath {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ModulePath(s.split('.').map(String::from).collect()))
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("."))
    }
}
