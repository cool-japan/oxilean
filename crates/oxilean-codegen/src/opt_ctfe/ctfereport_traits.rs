//! # CtfeReport - Trait Implementations
//!
//! This module contains trait implementations for `CtfeReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeReport;
use std::fmt;

impl fmt::Display for CtfeReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CtfeReport {{ evaluated={}, replaced={}, propagated={}, fuel_exhausted={} }}",
            self.functions_evaluated,
            self.calls_replaced,
            self.constants_propagated,
            self.fuel_exhausted_count,
        )
    }
}
