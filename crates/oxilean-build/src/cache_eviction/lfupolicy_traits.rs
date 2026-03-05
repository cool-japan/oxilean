//! # LfuPolicy - Trait Implementations
//!
//! This module contains trait implementations for `LfuPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, LfuPolicy};

impl EvictionPolicy for LfuPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut sorted: Vec<&CacheEntry> = entries.to_vec();
        sorted.sort_by(|a, b| {
            a.access_count
                .cmp(&b.access_count)
                .then_with(|| a.last_access.cmp(&b.last_access))
        });
        let mut freed: u64 = 0;
        let mut evicted = Vec::new();
        for entry in sorted {
            if freed >= bytes_needed {
                break;
            }
            evicted.push(entry.key.clone());
            freed += entry.size_bytes;
        }
        evicted
    }
    fn name(&self) -> &str {
        "LFU"
    }
}
