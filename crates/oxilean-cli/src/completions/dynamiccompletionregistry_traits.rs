//! # DynamicCompletionRegistry - Trait Implementations
//!
//! This module contains trait implementations for `DynamicCompletionRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DynamicCompletionRegistry;
use std::fmt;

impl Default for DynamicCompletionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
