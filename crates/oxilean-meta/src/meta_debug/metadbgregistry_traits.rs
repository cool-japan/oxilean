//! # MetaDbgRegistry - Trait Implementations
//!
//! This module contains trait implementations for `MetaDbgRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaDbgRegistry;

impl Default for MetaDbgRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
