//! # TacCasesRegistry - Trait Implementations
//!
//! This module contains trait implementations for `TacCasesRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacCasesRegistry;

impl Default for TacCasesRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
