//! # DeriveStats - Trait Implementations
//!
//! This module contains trait implementations for `DeriveStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DeriveStats;
use std::fmt;

impl fmt::Display for DeriveStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DeriveStats {{ attempted={}, succeeded={}, failed={}, success_rate={:.1}% }}",
            self.attempted,
            self.succeeded,
            self.failed,
            self.success_rate() * 100.0
        )
    }
}
