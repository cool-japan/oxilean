//! # LuaExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `LuaExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaExtEmitStats;
use std::fmt;

impl std::fmt::Display for LuaExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LuaExtEmitStats {{ bytes={}, items={}, errors={}, warnings={} }}",
            self.bytes_emitted, self.items_emitted, self.errors, self.warnings
        )
    }
}
