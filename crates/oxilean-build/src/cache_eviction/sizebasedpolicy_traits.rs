//! # SizeBasedPolicy - Trait Implementations
//!
//! This module contains trait implementations for `SizeBasedPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, SizeBasedPolicy};

impl EvictionPolicy for SizeBasedPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut sorted: Vec<&CacheEntry> = entries.to_vec();
        sorted.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
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
        "SizeBased"
    }
}
