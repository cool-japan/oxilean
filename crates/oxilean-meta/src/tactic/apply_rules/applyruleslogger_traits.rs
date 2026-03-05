//! # ApplyRulesLogger - Trait Implementations
//!
//! This module contains trait implementations for `ApplyRulesLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ApplyRulesLogger;

impl Default for ApplyRulesLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
