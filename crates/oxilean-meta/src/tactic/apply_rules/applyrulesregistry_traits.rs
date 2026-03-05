//! # ApplyRulesRegistry - Trait Implementations
//!
//! This module contains trait implementations for `ApplyRulesRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ApplyRulesRegistry;

impl Default for ApplyRulesRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
