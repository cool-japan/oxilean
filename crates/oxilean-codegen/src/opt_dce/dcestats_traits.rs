//! # DceStats - Trait Implementations
//!
//! This module contains trait implementations for `DceStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::DceStats;
use std::fmt;

impl fmt::Display for DceStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DceStats {{ lets_elim={}, alts_elim={}, consts_prop={}, \
             copies_prop={}, fns_elim={}, iters={} }}",
            self.lets_eliminated,
            self.alts_eliminated,
            self.constants_propagated,
            self.copies_propagated,
            self.functions_eliminated,
            self.iterations,
        )
    }
}
