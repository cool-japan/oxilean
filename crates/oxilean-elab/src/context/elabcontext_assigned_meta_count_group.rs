//! # ElabContext - assigned_meta_count_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Count assigned metavariables.
    pub fn assigned_meta_count(&self) -> usize {
        self.metas.len()
    }
}
