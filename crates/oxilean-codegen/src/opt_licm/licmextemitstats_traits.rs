//! # LicmExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `LicmExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmExtEmitStats;
use std::fmt;

impl std::fmt::Display for LicmExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LicmExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_written, self.items_emitted, self.errors, self.warnings
        )
    }
}
