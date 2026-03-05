//! # VirtualFundamentalClass - Trait Implementations
//!
//! This module contains trait implementations for `VirtualFundamentalClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VirtualFundamentalClass;
use std::fmt;

impl fmt::Display for VirtualFundamentalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]^vir ∈ {}", self.space, self.chow_group)
    }
}
