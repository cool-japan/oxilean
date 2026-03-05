//! # LuaBackend - Trait Implementations
//!
//! This module contains trait implementations for `LuaBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LuaBackend;

impl Default for LuaBackend {
    fn default() -> Self {
        Self::new()
    }
}
