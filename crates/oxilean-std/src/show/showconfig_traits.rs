//! # ShowConfig - Trait Implementations
//!
//! This module contains trait implementations for `ShowConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ShowConfig;

impl Default for ShowConfig {
    fn default() -> Self {
        Self {
            compact: false,
            ascii_only: false,
            max_depth: Some(50),
            show_implicit: false,
            show_levels: false,
            show_binder_types: true,
            indent_step: 2,
        }
    }
}
