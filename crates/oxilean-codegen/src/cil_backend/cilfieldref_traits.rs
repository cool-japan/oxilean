//! # CilFieldRef - Trait Implementations
//!
//! This module contains trait implementations for `CilFieldRef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilFieldRef;
use std::fmt;

impl fmt::Display for CilFieldRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}::{}",
            self.field_type, self.declaring_type, self.name
        )
    }
}
