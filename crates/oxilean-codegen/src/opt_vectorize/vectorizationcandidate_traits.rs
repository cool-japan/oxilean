//! # VectorizationCandidate - Trait Implementations
//!
//! This module contains trait implementations for `VectorizationCandidate`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VectorizationCandidate;
use std::fmt;

impl fmt::Display for VectorizationCandidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Candidate {{ func={}, var={}, bound={:?}, inner={}, dep={} }}",
            self.func_name,
            self.loop_var,
            self.loop_bound,
            self.is_inner_loop,
            self.has_loop_carried_dep
        )
    }
}
