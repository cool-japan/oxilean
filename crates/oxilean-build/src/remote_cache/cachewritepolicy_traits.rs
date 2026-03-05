//! # CacheWritePolicy - Trait Implementations
//!
//! This module contains trait implementations for `CacheWritePolicy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheWritePolicy;

impl std::fmt::Display for CacheWritePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheWritePolicy::Synchronous => write!(f, "synchronous"),
            CacheWritePolicy::Async => write!(f, "async"),
            CacheWritePolicy::OnFlush => write!(f, "on-flush"),
            CacheWritePolicy::ReadOnly => write!(f, "read-only"),
        }
    }
}
