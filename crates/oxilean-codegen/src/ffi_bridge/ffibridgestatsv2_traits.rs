//! # FfiBridgeStatsV2 - Trait Implementations
//!
//! This module contains trait implementations for `FfiBridgeStatsV2`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiBridgeStatsV2;
use std::fmt;

impl std::fmt::Display for FfiBridgeStatsV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiBridgeStatsV2 {{ fns={}, structs={}, enums={}, headers={}, bytes={} }}",
            self.functions_bridged,
            self.structs_bridged,
            self.enums_bridged,
            self.headers_generated,
            self.bytes_total,
        )
    }
}
