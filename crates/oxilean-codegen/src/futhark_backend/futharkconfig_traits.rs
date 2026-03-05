//! # FutharkConfig - Trait Implementations
//!
//! This module contains trait implementations for `FutharkConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FutharkConfig, FutharkType};

impl Default for FutharkConfig {
    fn default() -> Self {
        FutharkConfig {
            indent_width: 2,
            annotate_lets: true,
            default_int: FutharkType::I64,
            default_float: FutharkType::F64,
        }
    }
}
