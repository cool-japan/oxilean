//! # SpecExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `SpecExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SpecExtPassConfig;

impl Default for SpecExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
