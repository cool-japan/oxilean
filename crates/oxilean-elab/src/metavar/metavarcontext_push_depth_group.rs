//! # MetaVarContext - push_depth_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Increment elaboration depth.
    pub fn push_depth(&mut self) {
        self.depth += 1;
    }
}
