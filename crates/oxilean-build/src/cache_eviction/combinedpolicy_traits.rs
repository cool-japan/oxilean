//! # CombinedPolicy - Trait Implementations
//!
//! This module contains trait implementations for `CombinedPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, CombinedPolicy};

impl EvictionPolicy for CombinedPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut scores = self.compute_scores(entries);
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let entry_map: HashMap<&str, &CacheEntry> =
            entries.iter().map(|e| (e.key.as_str(), *e)).collect();
        let mut freed: u64 = 0;
        let mut evicted = Vec::new();
        for (key, _score) in &scores {
            if freed >= bytes_needed {
                break;
            }
            if let Some(entry) = entry_map.get(key.as_str()) {
                evicted.push(key.clone());
                freed += entry.size_bytes;
            }
        }
        evicted
    }
    fn name(&self) -> &str {
        "Combined"
    }
}
