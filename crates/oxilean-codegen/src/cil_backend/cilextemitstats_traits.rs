//! # CilExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `CilExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilExtEmitStats;
use std::fmt;

impl std::fmt::Display for CilExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CilExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_emitted, self.items_emitted, self.errors, self.warnings
        )
    }
}
