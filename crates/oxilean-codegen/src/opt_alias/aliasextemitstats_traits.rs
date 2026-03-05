//! # AliasExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `AliasExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasExtEmitStats;
use std::fmt;

impl std::fmt::Display for AliasExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AliasExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_written, self.items_emitted, self.errors, self.warnings
        )
    }
}
