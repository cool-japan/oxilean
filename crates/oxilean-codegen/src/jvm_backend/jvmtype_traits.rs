//! # JvmType - Trait Implementations
//!
//! This module contains trait implementations for `JvmType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JvmType;
use std::fmt;

impl fmt::Display for JvmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.descriptor())
    }
}
