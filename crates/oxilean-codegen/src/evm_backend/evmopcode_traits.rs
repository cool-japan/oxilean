//! # EvmOpcode - Trait Implementations
//!
//! This module contains trait implementations for `EvmOpcode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmOpcode;
use std::fmt;

impl fmt::Display for EvmOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mnemonic())
    }
}
