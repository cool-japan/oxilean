//! # LuaExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `LuaExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaExtPassConfig;

impl Default for LuaExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
