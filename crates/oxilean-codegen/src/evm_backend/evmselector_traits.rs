//! # EvmSelector - Trait Implementations
//!
//! This module contains trait implementations for `EvmSelector`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmSelector;
use std::fmt;

impl std::fmt::Display for EvmSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(0x{})", self.signature, self.hex())
    }
}
