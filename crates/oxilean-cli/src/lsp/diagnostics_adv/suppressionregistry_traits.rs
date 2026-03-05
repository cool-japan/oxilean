//! # SuppressionRegistry - Trait Implementations
//!
//! This module contains trait implementations for `SuppressionRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SuppressionRegistry;
use std::fmt;

impl Default for SuppressionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
