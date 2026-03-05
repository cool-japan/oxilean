//! # ClosureCodegen - Trait Implementations
//!
//! This module contains trait implementations for `ClosureCodegen`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::ClosureCodegen;

impl Default for ClosureCodegen {
    fn default() -> Self {
        Self::new()
    }
}
