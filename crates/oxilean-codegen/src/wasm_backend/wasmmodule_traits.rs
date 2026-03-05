//! # WasmModule - Trait Implementations
//!
//! This module contains trait implementations for `WasmModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmModule;
use std::fmt;

impl fmt::Display for WasmModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_wat())
    }
}
