//! # PluginSandboxConfig - Trait Implementations
//!
//! This module contains trait implementations for `PluginSandboxConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PluginSandboxConfig;

impl Default for PluginSandboxConfig {
    fn default() -> Self {
        Self::strict()
    }
}
