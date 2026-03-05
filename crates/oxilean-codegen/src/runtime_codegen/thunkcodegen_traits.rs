//! # ThunkCodegen - Trait Implementations
//!
//! This module contains trait implementations for `ThunkCodegen`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::ThunkCodegen;

impl Default for ThunkCodegen {
    fn default() -> Self {
        Self::new()
    }
}
