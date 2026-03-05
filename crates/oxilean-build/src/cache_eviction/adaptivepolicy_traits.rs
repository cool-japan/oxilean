//! # AdaptivePolicy - Trait Implementations
//!
//! This module contains trait implementations for `AdaptivePolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{AdaptivePolicy, CacheEntry, WorkloadHint};

impl EvictionPolicy for AdaptivePolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        match self.hint {
            WorkloadHint::TemporalLocality => self.lru.select_evictions(entries, bytes_needed),
            WorkloadHint::Streaming => self.lfu.select_evictions(entries, bytes_needed),
            WorkloadHint::LargeBlobHeavy => self.size.select_evictions(entries, bytes_needed),
            WorkloadHint::Balanced => self.lru.select_evictions(entries, bytes_needed),
        }
    }
    fn name(&self) -> &str {
        "Adaptive"
    }
}
