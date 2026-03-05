//! # AesopSearchState - Trait Implementations
//!
//! This module contains trait implementations for `AesopSearchState`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AesopSearchState;

impl std::fmt::Debug for AesopSearchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AesopSearchState")
            .field("num_nodes", &self.nodes.len())
            .field("queue_len", &self.queue.len())
            .field("finished", &self.finished)
            .field("stats", &self.stats)
            .finish()
    }
}
