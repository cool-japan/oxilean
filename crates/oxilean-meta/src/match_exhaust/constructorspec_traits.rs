//! # ConstructorSpec - Trait Implementations
//!
//! This module contains trait implementations for `ConstructorSpec`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConstructorSpec;

impl std::fmt::Display for ConstructorSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
