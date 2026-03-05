//! # ElabContext - options_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabOptions;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Get elaboration options.
    pub fn options(&self) -> &ElabOptions {
        &self.options
    }
}
