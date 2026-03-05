//! # PrintConfig - Trait Implementations
//!
//! This module contains trait implementations for `PrintConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrintConfig;

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            unicode: true,
            show_implicit: false,
            show_universes: false,
            max_width: 100,
            show_binder_info: true,
            show_indices: true,
        }
    }
}
