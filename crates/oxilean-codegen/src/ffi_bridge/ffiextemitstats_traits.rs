//! # FfiExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `FfiExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiExtEmitStats;
use std::fmt;

impl std::fmt::Display for FfiExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_emitted, self.items_emitted, self.errors, self.warnings
        )
    }
}
