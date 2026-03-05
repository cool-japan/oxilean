//! # WhnfStats - Trait Implementations
//!
//! This module contains trait implementations for `WhnfStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhnfStats;

impl std::fmt::Display for WhnfStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WhnfStats {{ reductions: {}, cache_hits: {}, stuck: {} }}",
            self.reductions, self.cache_hits, self.stuck_count
        )
    }
}
