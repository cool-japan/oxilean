//! # MetaContext - predicates Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Check if a name is a recursor.
    pub fn is_recursor(&self, name: &Name) -> bool {
        self.env.is_recursor(name)
    }
}
