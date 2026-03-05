//! # TypeClassCoercionRegistry - Trait Implementations
//!
//! This module contains trait implementations for `TypeClassCoercionRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypeClassCoercionRegistry;
use std::fmt;

impl Default for TypeClassCoercionRegistry {
    fn default() -> Self {
        TypeClassCoercionRegistry::new()
    }
}
