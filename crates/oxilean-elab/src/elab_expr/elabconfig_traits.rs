//! # ElabConfig - Trait Implementations
//!
//! This module contains trait implementations for `ElabConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabConfig;
use std::fmt;

impl Default for ElabConfig {
    fn default() -> Self {
        ElabConfig {
            max_depth: 256,
            insert_implicits: true,
            infer_universes: true,
            auto_coercions: true,
            trace_enabled: false,
            allow_sorry: true,
        }
    }
}
