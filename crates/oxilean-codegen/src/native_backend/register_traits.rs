//! # Register - Trait Implementations
//!
//! This module contains trait implementations for `Register`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::Register;
use std::fmt;

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_virtual() {
            write!(f, "v{}", self.0)
        } else {
            write!(f, "r{}", self.0 - super::types::VIRT_PHYS_BOUNDARY)
        }
    }
}
