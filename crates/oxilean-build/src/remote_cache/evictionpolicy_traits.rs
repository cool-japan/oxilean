//! # EvictionPolicy - Trait Implementations
//!
//! This module contains trait implementations for `EvictionPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvictionPolicy;

impl std::fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionPolicy::Lru => write!(f, "lru"),
            EvictionPolicy::Lfu => write!(f, "lfu"),
            EvictionPolicy::Fifo => write!(f, "fifo"),
            EvictionPolicy::LargestFirst => write!(f, "largest-first"),
        }
    }
}
