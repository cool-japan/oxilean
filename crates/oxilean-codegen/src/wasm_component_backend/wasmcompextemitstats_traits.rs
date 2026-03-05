//! # WasmCompExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `WasmCompExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WasmCompExtEmitStats;
use std::fmt;

impl std::fmt::Display for WasmCompExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WasmCompExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_emitted, self.items_emitted, self.errors, self.warnings
        )
    }
}
