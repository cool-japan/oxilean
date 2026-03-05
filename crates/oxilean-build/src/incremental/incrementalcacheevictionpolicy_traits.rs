//! # IncrementalCacheEvictionPolicy - Trait Implementations
//!
//! This module contains trait implementations for `IncrementalCacheEvictionPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IncrementalCacheEvictionPolicy;

impl std::fmt::Display for IncrementalCacheEvictionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncrementalCacheEvictionPolicy::Lru => write!(f, "lru"),
            IncrementalCacheEvictionPolicy::Lfu => write!(f, "lfu"),
            IncrementalCacheEvictionPolicy::Fifo => write!(f, "fifo"),
        }
    }
}
