//! # GoalDisplayConfig - Trait Implementations
//!
//! This module contains trait implementations for `GoalDisplayConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GoalDisplayConfig;
use std::fmt;

impl Default for GoalDisplayConfig {
    fn default() -> Self {
        Self {
            max_type_width: 80,
            show_mvar_ids: false,
            show_tags: true,
            show_let_values: true,
        }
    }
}
