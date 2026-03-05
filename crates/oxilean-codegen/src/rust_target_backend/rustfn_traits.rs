//! # RustFn - Trait Implementations
//!
//! This module contains trait implementations for `RustFn`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RustFn;
use std::fmt;

impl fmt::Display for RustFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.emit())
    }
}
