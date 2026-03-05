//! # CtfeVersionInfo - Trait Implementations
//!
//! This module contains trait implementations for `CtfeVersionInfo`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeVersionInfo;
use std::fmt;

impl Default for CtfeVersionInfo {
    fn default() -> Self {
        Self {
            pass_version: 1,
            min_fuel: 1,
            max_fuel: 1_000_000,
            supports_memo: true,
            supports_partial_eval: true,
        }
    }
}

impl std::fmt::Display for CtfeVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CtfeVersionInfo {{ v={}, min_fuel={}, max_fuel={}, memo={}, partial={} }}",
            self.pass_version,
            self.min_fuel,
            self.max_fuel,
            self.supports_memo,
            self.supports_partial_eval
        )
    }
}
