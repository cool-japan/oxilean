//! # MetalExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `MetalExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalExtEmitStats;
use std::fmt;

impl std::fmt::Display for MetalExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MetalExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_emitted, self.items_emitted, self.errors, self.warnings
        )
    }
}
