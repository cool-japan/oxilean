//! # MetaContext - push_depth_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Increase the depth (for scoped operations).
    pub fn push_depth(&mut self) {
        self.depth += 1;
    }
}
