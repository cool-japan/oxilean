//! # ArrayCodegen - Trait Implementations
//!
//! This module contains trait implementations for `ArrayCodegen`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::{AllocStrategy, ArrayCodegen};

impl Default for ArrayCodegen {
    fn default() -> Self {
        Self::new(AllocStrategy::LeanRuntime)
    }
}
