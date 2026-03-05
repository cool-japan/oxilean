//! # JvmInstruction - Trait Implementations
//!
//! This module contains trait implementations for `JvmInstruction`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JvmInstruction;
use std::fmt;

impl fmt::Display for JvmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.opcode)
    }
}
