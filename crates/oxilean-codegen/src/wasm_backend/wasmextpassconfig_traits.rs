//! # WasmExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `WasmExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmExtPassConfig;

impl Default for WasmExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
