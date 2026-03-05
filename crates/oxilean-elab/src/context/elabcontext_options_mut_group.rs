//! # ElabContext - options_mut_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabOptions;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Get mutable elaboration options.
    pub fn options_mut(&mut self) -> &mut ElabOptions {
        &mut self.options
    }
}
