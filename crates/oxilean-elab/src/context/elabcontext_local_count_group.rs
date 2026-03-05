//! # ElabContext - local_count_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Count local variables.
    pub fn local_count(&self) -> usize {
        self.locals.len()
    }
}
