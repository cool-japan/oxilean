//! # CoercionRegistry - Trait Implementations
//!
//! This module contains trait implementations for `CoercionRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionRegistry;
use std::fmt;

impl Default for CoercionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
