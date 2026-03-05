//! # CacheSessionId - Trait Implementations
//!
//! This module contains trait implementations for `CacheSessionId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheSessionId;

impl std::fmt::Display for CacheSessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CacheSession({})", self.0)
    }
}
