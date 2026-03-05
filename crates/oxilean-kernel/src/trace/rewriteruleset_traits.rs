//! # RewriteRuleSet - Trait Implementations
//!
//! This module contains trait implementations for `RewriteRuleSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RewriteRuleSet;
use std::fmt;

impl Default for RewriteRuleSet {
    fn default() -> Self {
        Self::new()
    }
}
