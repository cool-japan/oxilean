//! # MetaContext - pop_depth_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Decrease the depth.
    pub fn pop_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
}
