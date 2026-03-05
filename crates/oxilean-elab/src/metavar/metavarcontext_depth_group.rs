//! # MetaVarContext - depth_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get current depth.
    pub fn depth(&self) -> u32 {
        self.depth
    }
}
