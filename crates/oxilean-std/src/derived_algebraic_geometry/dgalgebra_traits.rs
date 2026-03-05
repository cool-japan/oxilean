//! # DGAlgebra - Trait Implementations
//!
//! This module contains trait implementations for `DGAlgebra`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DGAlgebra;
use std::fmt;

impl fmt::Display for DGAlgebra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DGA({}, {}-graded over {})",
            self.name, self.grading, self.base_field
        )
    }
}
