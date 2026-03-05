//! # DGCategory - Trait Implementations
//!
//! This module contains trait implementations for `DGCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DGCategory;
use std::fmt;

impl fmt::Display for DGCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dgCat({}/{}) pre-triangulated={}",
            self.name, self.base_ring, self.is_pre_triangulated
        )
    }
}
