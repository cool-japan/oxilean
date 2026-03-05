//! # InlineReport - Trait Implementations
//!
//! This module contains trait implementations for `InlineReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::InlineReport;
use std::fmt;

impl fmt::Display for InlineReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InlineReport {{ inlined={}, considered={} }}",
            self.inlines_performed, self.functions_considered
        )
    }
}
