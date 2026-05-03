//! # TtlPolicy - Trait Implementations
//!
//! This module contains trait implementations for `TtlPolicy`.
//!
//! ## Implemented Traits
//!
//! - `EvictionPolicy`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::EvictionPolicy;
use super::types::{CacheEntry, TtlPolicy};

impl EvictionPolicy for TtlPolicy {
    fn select_evictions(&self, entries: &[&CacheEntry], bytes_needed: u64) -> Vec<String> {
        let mut expired: Vec<&CacheEntry> = Vec::new();
        let mut alive: Vec<&CacheEntry> = Vec::new();
        for &e in entries {
            if self.is_expired(e) {
                expired.push(e);
            } else {
                alive.push(e);
            }
        }
        expired.sort_by_key(|a| a.created_at);
        alive.sort_by(|a, b| {
            let rem_a = self.max_age.saturating_sub(a.age());
            let rem_b = self.max_age.saturating_sub(b.age());
            rem_a.cmp(&rem_b)
        });
        let mut freed: u64 = 0;
        let mut evicted = Vec::new();
        for entry in &expired {
            evicted.push(entry.key.clone());
            freed += entry.size_bytes;
        }
        for entry in &alive {
            if freed >= bytes_needed {
                break;
            }
            evicted.push(entry.key.clone());
            freed += entry.size_bytes;
        }
        evicted
    }
    fn name(&self) -> &str {
        "TTL"
    }
}
