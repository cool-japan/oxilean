//! # CEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `CEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CEmitStats;
use std::fmt;

impl fmt::Display for CEmitStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CEmitStats {{ fns={}, structs={}, rc_inc={}, rc_dec={}, closures={}, switches={}, lines={} }}",
            self.functions_emitted, self.structs_emitted, self.rc_inc_calls, self
            .rc_dec_calls, self.closures_emitted, self.switches_emitted, self
            .total_lines,
        )
    }
}
