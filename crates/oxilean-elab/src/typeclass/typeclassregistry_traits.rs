//! # TypeClassRegistry - Trait Implementations
//!
//! This module contains trait implementations for `TypeClassRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypeClassRegistry;
use std::fmt;

impl Default for TypeClassRegistry {
    fn default() -> Self {
        Self::new()
    }
}
