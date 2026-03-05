//! # RuleRegistry - Trait Implementations
//!
//! This module contains trait implementations for `RuleRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RuleRegistry;
use std::fmt;

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
