//! # CacheReadPolicy - Trait Implementations
//!
//! This module contains trait implementations for `CacheReadPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheReadPolicy;

impl std::fmt::Display for CacheReadPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheReadPolicy::Enabled => write!(f, "enabled"),
            CacheReadPolicy::Bypass => write!(f, "bypass"),
            CacheReadPolicy::Verified => write!(f, "verified"),
        }
    }
}
