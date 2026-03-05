//! # ShowStats - Trait Implementations
//!
//! This module contains trait implementations for `ShowStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ShowStats;
use std::fmt;

impl fmt::Display for ShowStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShowStats {{ exprs: {}, truncations: {}, chars: {} }}",
            self.exprs_shown, self.depth_truncations, self.chars_produced
        )
    }
}
