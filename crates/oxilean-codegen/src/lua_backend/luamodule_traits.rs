//! # LuaModule - Trait Implementations
//!
//! This module contains trait implementations for `LuaModule`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaModule;

impl Default for LuaModule {
    fn default() -> Self {
        Self::new()
    }
}
