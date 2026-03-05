//! # CoercionValidator - Trait Implementations
//!
//! This module contains trait implementations for `CoercionValidator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionValidator;
use std::fmt;

impl Default for CoercionValidator {
    fn default() -> Self {
        CoercionValidator::new()
    }
}
