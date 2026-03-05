//! # ReuseExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `ReuseExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseExtEmitStats;
use std::fmt;

impl std::fmt::Display for ReuseExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReuseExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_written, self.items_emitted, self.errors, self.warnings
        )
    }
}
