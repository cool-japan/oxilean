//! # FutharkExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `FutharkExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkExtEmitStats;
use std::fmt;

impl std::fmt::Display for FutharkExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FutharkExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_written, self.items_emitted, self.errors, self.warnings
        )
    }
}
