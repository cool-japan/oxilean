//! # ExactTriangle - Trait Implementations
//!
//! This module contains trait implementations for `ExactTriangle`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExactTriangle;
use std::fmt;

impl fmt::Display for ExactTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} → {} → {} → {}[1]",
            self.vertex_a, self.vertex_b, self.vertex_c, self.vertex_a
        )
    }
}
