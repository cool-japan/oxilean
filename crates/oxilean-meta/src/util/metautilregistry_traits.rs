//! # MetaUtilRegistry - Trait Implementations
//!
//! This module contains trait implementations for `MetaUtilRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaUtilRegistry;

impl Default for MetaUtilRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
