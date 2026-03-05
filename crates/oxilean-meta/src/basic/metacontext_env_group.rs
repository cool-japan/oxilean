//! # MetaContext - env_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Environment;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the environment.
    pub fn env(&self) -> &Environment {
        &self.env
    }
}
