//! # LicmConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `LicmConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmConfigExt;

impl Default for LicmConfigExt {
    fn default() -> Self {
        Self {
            enable_hoist: true,
            enable_sink: true,
            enable_speculative_hoist: false,
            max_hoist_cost: 100,
            min_trip_count: 2,
            hoist_stores: false,
            hoist_calls: false,
            max_loop_depth: 16,
        }
    }
}
