//! # DependenceInfo - Trait Implementations
//!
//! This module contains trait implementations for `DependenceInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DependenceInfo;
use std::fmt;

impl fmt::Display for DependenceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DependenceInfo {{ true={}, anti={}, output={}, loop_carried={}, parallelizable={} }}",
            self.true_deps.len(),
            self.anti_deps.len(),
            self.output_deps.len(),
            self.loop_carried_deps.len(),
            self.is_parallelizable()
        )
    }
}
