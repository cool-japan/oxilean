//! # LayoutConfig - Trait Implementations
//!
//! This module contains trait implementations for `LayoutConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LayoutConfig;

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            max_width: 100,
            indent_size: 2,
            use_tabs: false,
            tab_width: 4,
        }
    }
}
