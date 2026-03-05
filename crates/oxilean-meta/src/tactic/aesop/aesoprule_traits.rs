//! # AesopRule - Trait Implementations
//!
//! This module contains trait implementations for `AesopRule`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopRule;

impl std::fmt::Debug for AesopRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AesopRule")
            .field("name", &self.name)
            .field("safety", &self.safety)
            .field("kind", &self.kind)
            .field("priority", &self.priority)
            .field("estimated_subgoals", &self.estimated_subgoals)
            .finish()
    }
}
