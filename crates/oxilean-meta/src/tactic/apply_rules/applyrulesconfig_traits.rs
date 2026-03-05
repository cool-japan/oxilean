//! # ApplyRulesConfig - Trait Implementations
//!
//! This module contains trait implementations for `ApplyRulesConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ApplyRulesConfig, ReasoningMode};

impl Default for ApplyRulesConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            mode: ReasoningMode::Backward,
            safe_only: false,
            exhaustive: true,
            tag_filter: None,
            trace: true,
        }
    }
}
