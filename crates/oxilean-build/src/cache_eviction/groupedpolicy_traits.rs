//! # GroupedPolicy - Trait Implementations
//!
//! This module contains trait implementations for `GroupedPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, GroupedPolicy};

impl EvictionPolicy for GroupedPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut sorted: Vec<&CacheEntry> = entries.to_vec();
        sorted.sort_by(|a, b| {
            let ga = self
                .group_order
                .get(self.group_of(&a.key))
                .copied()
                .unwrap_or(usize::MAX);
            let gb = self
                .group_order
                .get(self.group_of(&b.key))
                .copied()
                .unwrap_or(usize::MAX);
            ga.cmp(&gb).then_with(|| a.last_access.cmp(&b.last_access))
        });
        let mut freed = 0u64;
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
        "Grouped"
    }
}
