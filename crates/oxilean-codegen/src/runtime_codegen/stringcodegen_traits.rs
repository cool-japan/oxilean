//! # StringCodegen - Trait Implementations
//!
//! This module contains trait implementations for `StringCodegen`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::{StringCodegen, StringLayout};

impl Default for StringCodegen {
    fn default() -> Self {
        Self::new(StringLayout::standard())
    }
}
