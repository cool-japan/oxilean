//! # MetaContext - queries Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{ConstantInfo, Name};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Find a constant in the environment.
    pub fn find_const(&self, name: &Name) -> Option<&ConstantInfo> {
        self.env.find(name)
    }
}
