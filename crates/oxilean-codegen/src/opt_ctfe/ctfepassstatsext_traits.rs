//! # CtfePassStatsExt - Trait Implementations
//!
//! This module contains trait implementations for `CtfePassStatsExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfePassStatsExt;
use std::fmt;

impl std::fmt::Display for CtfePassStatsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CtfePassStatsExt {{ attempted={}, evaluated={}, replaced={}, \
             folded={}, memo_hits={}, fuel_used={}, errors={} }}",
            self.functions_attempted,
            self.functions_evaluated,
            self.calls_replaced,
            self.constants_folded,
            self.memo_hits,
            self.fuel_used,
            self.errors,
        )
    }
}
