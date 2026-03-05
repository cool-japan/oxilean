//! # FfiCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `FfiCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiCodeStats;
use std::fmt;

impl std::fmt::Display for FfiCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiCodeStats {{ fns={}, structs={}, enums={}, td={}, consts={}, bytes={} }}",
            self.functions,
            self.structs,
            self.enums,
            self.typedefs,
            self.constants,
            self.total_bytes,
        )
    }
}
