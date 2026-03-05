//! # FfiTypedef - Trait Implementations
//!
//! This module contains trait implementations for `FfiTypedef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiTypedef;
use std::fmt;

impl std::fmt::Display for FfiTypedef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef {} {};", self.base_type, self.name)
    }
}
