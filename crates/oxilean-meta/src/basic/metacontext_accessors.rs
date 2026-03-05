//! # MetaContext - accessors Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConfig;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Set the configuration.
    pub fn set_config(&mut self, config: MetaConfig) {
        self.config = config;
    }
}
