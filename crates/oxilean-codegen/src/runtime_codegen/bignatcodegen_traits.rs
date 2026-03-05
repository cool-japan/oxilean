//! # BigNatCodegen - Trait Implementations
//!
//! This module contains trait implementations for `BigNatCodegen`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::BigNatCodegen;

impl Default for BigNatCodegen {
    fn default() -> Self {
        Self::new()
    }
}
