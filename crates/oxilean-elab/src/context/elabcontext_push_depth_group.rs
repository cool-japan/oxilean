//! # ElabContext - push_depth_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Push elaboration depth.
    pub fn push_depth(&mut self) {
        self.depth += 1;
    }
}
