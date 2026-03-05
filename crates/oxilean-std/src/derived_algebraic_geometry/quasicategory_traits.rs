//! # QuasiCategory - Trait Implementations
//!
//! This module contains trait implementations for `QuasiCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::QuasiCategory;
use std::fmt;

impl fmt::Display for QuasiCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QuasiCat({}, inner_horns={})",
            self.name, self.inner_horns_fill
        )
    }
}
