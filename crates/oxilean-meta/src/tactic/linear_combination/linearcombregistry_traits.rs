//! # LinearCombRegistry - Trait Implementations
//!
//! This module contains trait implementations for `LinearCombRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearCombRegistry;

impl Default for LinearCombRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
