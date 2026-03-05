//! # LayoutComputer - Trait Implementations
//!
//! This module contains trait implementations for `LayoutComputer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::LayoutComputer;

impl Default for LayoutComputer {
    fn default() -> Self {
        Self::new()
    }
}
