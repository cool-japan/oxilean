//! # PluginPriority - Trait Implementations
//!
//! This module contains trait implementations for `PluginPriority`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PluginPriority;

impl Default for PluginPriority {
    fn default() -> Self {
        PluginPriority::Normal
    }
}

impl std::fmt::Display for PluginPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginPriority::High => write!(f, "high"),
            PluginPriority::Normal => write!(f, "normal"),
            PluginPriority::Low => write!(f, "low"),
        }
    }
}
