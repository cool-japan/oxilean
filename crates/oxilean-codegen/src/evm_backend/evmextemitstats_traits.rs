//! # EvmExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `EvmExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmExtEmitStats;
use std::fmt;

impl std::fmt::Display for EvmExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EvmExtEmitStats {{ bytes={}, items={}, fns={}, events={}, errors={} }}",
            self.bytes_written,
            self.items_emitted,
            self.functions_emitted,
            self.events_emitted,
            self.errors,
        )
    }
}
