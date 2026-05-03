//! # PluginRegistry - Trait Implementations
//!
//! This module contains trait implementations for `PluginRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::num_cpus;
use super::types::PluginRegistry;

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
