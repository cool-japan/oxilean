//! # EnvironmentSnapshot - Trait Implementations
//!
//! This module contains trait implementations for `EnvironmentSnapshot`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EnvironmentSnapshot;
use std::fmt;

impl Default for EnvironmentSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
