//! # MatchBasicRegistry - Trait Implementations
//!
//! This module contains trait implementations for `MatchBasicRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatchBasicRegistry;

impl Default for MatchBasicRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
