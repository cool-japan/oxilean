//! # LlvmTypeAlias - Trait Implementations
//!
//! This module contains trait implementations for `LlvmTypeAlias`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmTypeAlias;
use std::fmt;

impl fmt::Display for LlvmTypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{} = type {}", self.name, self.ty)
    }
}
