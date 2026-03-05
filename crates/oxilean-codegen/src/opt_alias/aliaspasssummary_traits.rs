//! # AliasPassSummary - Trait Implementations
//!
//! This module contains trait implementations for `AliasPassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasPassSummary;
use std::fmt;

impl std::fmt::Display for AliasPassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AliasPassSummary[{}] {{ funcs={}, queries={}, must_alias={:.1}%, no_alias={:.1}%, {}us }}",
            self.pass_name, self.functions_analyzed, self.queries_answered, self
            .must_alias_rate * 100.0, self.no_alias_rate * 100.0, self.duration_us,
        )
    }
}
