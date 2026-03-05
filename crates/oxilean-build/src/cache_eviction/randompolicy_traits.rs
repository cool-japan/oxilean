//! # RandomPolicy - Trait Implementations
//!
//! This module contains trait implementations for `RandomPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, RandomPolicy};

impl EvictionPolicy for RandomPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut indices: Vec<usize> = (0..entries.len()).collect();
        let mut state = self.seed;
        for i in 0..indices.len() {
            let j = (Self::lcg_next(&mut state) as usize) % (indices.len() - i) + i;
            indices.swap(i, j);
        }
        let mut freed = 0u64;
        let mut evicted = Vec::new();
        for &idx in &indices {
            if freed >= bytes_needed {
                break;
            }
            evicted.push(entries[idx].key.clone());
            freed += entries[idx].size_bytes;
        }
        evicted
    }
    fn name(&self) -> &str {
        "Random"
    }
}
