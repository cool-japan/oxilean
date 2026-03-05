//! # AesopStats - Trait Implementations
//!
//! This module contains trait implementations for `AesopStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopStats;

impl std::fmt::Display for AesopStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "nodes_created={} expanded={} rules_tried={} succeeded={} \
             norm_passes={} cache_hits={} depth={} time={}us backtracks={}",
            self.nodes_created,
            self.nodes_expanded,
            self.rules_tried,
            self.rules_succeeded,
            self.norm_passes,
            self.cache_hits,
            self.max_depth_reached,
            self.time_us,
            self.backtracks,
        )
    }
}
