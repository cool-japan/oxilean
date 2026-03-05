//! # FutharkPassStats - Trait Implementations
//!
//! This module contains trait implementations for `FutharkPassStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkPassStats;
use std::fmt;

impl std::fmt::Display for FutharkPassStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FutharkPassStats {{ fns={}, maps={}, reduces={}, scans={}, kernels={} }}",
            self.functions_processed,
            self.maps_emitted,
            self.reduces_emitted,
            self.scans_emitted,
            self.kernels_generated,
        )
    }
}
