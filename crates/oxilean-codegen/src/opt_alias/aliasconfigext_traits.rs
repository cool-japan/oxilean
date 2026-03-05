//! # AliasConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `AliasConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasConfigExt;

impl Default for AliasConfigExt {
    fn default() -> Self {
        Self {
            level: "andersen".to_string(),
            max_iterations: 100,
            max_points_to_size: 1000,
            enable_field_sensitivity: false,
            enable_flow_sensitivity: false,
            enable_context_sensitivity: false,
            track_heap: true,
            track_globals: true,
        }
    }
}
