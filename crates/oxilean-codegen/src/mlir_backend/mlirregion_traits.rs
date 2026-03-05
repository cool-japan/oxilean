//! # MlirRegion - Trait Implementations
//!
//! This module contains trait implementations for `MlirRegion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MlirRegion;
use std::fmt;

impl fmt::Display for MlirRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " {{")?;
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        write!(f, "  }}")
    }
}
