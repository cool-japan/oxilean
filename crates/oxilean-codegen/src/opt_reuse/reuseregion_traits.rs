//! # ReuseRegion - Trait Implementations
//!
//! This module contains trait implementations for `ReuseRegion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseRegion;
use std::fmt;

impl std::fmt::Display for ReuseRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReuseRegion#{}({}) {{ allocs={}, decisions={} }}",
            self.region_id,
            self.func_name,
            self.allocs.len(),
            self.decisions.len()
        )
    }
}
