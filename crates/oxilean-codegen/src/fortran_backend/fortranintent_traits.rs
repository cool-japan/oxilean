//! # FortranIntent - Trait Implementations
//!
//! This module contains trait implementations for `FortranIntent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FortranIntent;
use std::fmt;

impl fmt::Display for FortranIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FortranIntent::In => write!(f, "IN"),
            FortranIntent::Out => write!(f, "OUT"),
            FortranIntent::InOut => write!(f, "INOUT"),
        }
    }
}
