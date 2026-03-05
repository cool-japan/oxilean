//! # AliasCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `AliasCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasCodeStats;
use std::fmt;

impl std::fmt::Display for AliasCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AliasCodeStats {{ funcs={}, must={}, no={}, escapes={}, promotions={} }}",
            self.functions_analyzed,
            self.must_aliases_found,
            self.no_aliases_found,
            self.escape_candidates,
            self.stack_promotions,
        )
    }
}
