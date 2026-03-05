//! # WasmGlobal - Trait Implementations
//!
//! This module contains trait implementations for `WasmGlobal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmGlobal;
use std::fmt;

impl fmt::Display for WasmGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mutable {
            write!(
                f,
                "  (global ${} (mut {}) ({}.const {}))",
                self.name, self.ty, self.ty, self.init_value
            )
        } else {
            write!(
                f,
                "  (global ${} {} ({}.const {}))",
                self.name, self.ty, self.ty, self.init_value
            )
        }
    }
}
