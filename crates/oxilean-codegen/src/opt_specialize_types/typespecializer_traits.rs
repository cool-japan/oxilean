//! # TypeSpecializer - Trait Implementations
//!
//! This module contains trait implementations for `TypeSpecializer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{TypeSpecConfig, TypeSpecializer};

impl Default for TypeSpecializer {
    fn default() -> Self {
        TypeSpecializer::new(TypeSpecConfig::default())
    }
}
