//! # AliasStatsExt - Trait Implementations
//!
//! This module contains trait implementations for `AliasStatsExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasStatsExt;
use std::fmt;

impl std::fmt::Display for AliasStatsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AliasStatsExt {{ queries={}, must={}, no={}, may={}, partial={} }}",
            self.queries_total,
            self.must_alias_count,
            self.no_alias_count,
            self.may_alias_count,
            self.partial_alias_count,
        )
    }
}
