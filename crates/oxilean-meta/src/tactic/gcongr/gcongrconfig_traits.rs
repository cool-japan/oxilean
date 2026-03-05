//! # GCongrConfig - Trait Implementations
//!
//! This module contains trait implementations for `GCongrConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GCongrConfig;

impl Default for GCongrConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            relation_filter: None,
            try_refl: true,
            use_hyps: true,
            recursive: false,
        }
    }
}
