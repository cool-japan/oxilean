//! # CacheOptPass - Trait Implementations
//!
//! This module contains trait implementations for `CacheOptPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CacheOptConfig, CacheOptPass};

impl Default for CacheOptPass {
    fn default() -> Self {
        Self::new(CacheOptConfig::default())
    }
}
