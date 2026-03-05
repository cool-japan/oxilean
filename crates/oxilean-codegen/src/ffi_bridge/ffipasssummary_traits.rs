//! # FfiPassSummary - Trait Implementations
//!
//! This module contains trait implementations for `FfiPassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiPassSummary;
use std::fmt;

impl std::fmt::Display for FfiPassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiPassSummary[{}] {{ symbols={}, headers={}, rust_bindings={}, {}us }}",
            self.pass_name,
            self.symbols_bridged,
            self.headers_emitted,
            self.rust_bindings_emitted,
            self.duration_us,
        )
    }
}
