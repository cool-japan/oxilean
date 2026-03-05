//! # EvmStorageSlot - Trait Implementations
//!
//! This module contains trait implementations for `EvmStorageSlot`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmStorageSlot;
use std::fmt;

impl std::fmt::Display for EvmStorageSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "slot[{}+{}]: {} {}",
            self.slot, self.offset, self.var_type, self.var_name
        )
    }
}
