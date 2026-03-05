//! # PassStats - Trait Implementations
//!
//! This module contains trait implementations for `PassStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PassStats;
use std::fmt;

impl fmt::Display for PassStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PassStats({}: runs={}, changes={}, avg={:.1})",
            self.name,
            self.run_count,
            self.total_changes,
            self.avg_changes()
        )
    }
}
