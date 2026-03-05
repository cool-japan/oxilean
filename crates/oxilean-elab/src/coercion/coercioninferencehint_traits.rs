//! # CoercionInferenceHint - Trait Implementations
//!
//! This module contains trait implementations for `CoercionInferenceHint`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoercionInferenceHint;
use std::fmt;

impl Default for CoercionInferenceHint {
    fn default() -> Self {
        CoercionInferenceHint::new()
    }
}
