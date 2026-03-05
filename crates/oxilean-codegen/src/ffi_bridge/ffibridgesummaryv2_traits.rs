//! # FfiBridgeSummaryV2 - Trait Implementations
//!
//! This module contains trait implementations for `FfiBridgeSummaryV2`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiBridgeSummaryV2;
use std::fmt;

impl std::fmt::Display for FfiBridgeSummaryV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FFIBridgeSummary({}, {}) {{ fns={}, structs={}, enums={}, bytes={} }}",
            self.lib_name, self.platform, self.funcs, self.structs, self.enums, self.bytes,
        )
    }
}
