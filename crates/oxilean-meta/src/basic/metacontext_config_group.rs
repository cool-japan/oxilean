//! # MetaContext - config_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConfig;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the configuration.
    pub fn config(&self) -> &MetaConfig {
        &self.config
    }
}
