//! # AesopRuleSet - Trait Implementations
//!
//! This module contains trait implementations for `AesopRuleSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopRuleSet;

impl Default for AesopRuleSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for AesopRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AesopRuleSet")
            .field("num_rules", &self.rules.len())
            .field("num_wildcard", &self.wildcard_ids.len())
            .field("num_norm", &self.norm_ids.len())
            .finish()
    }
}
