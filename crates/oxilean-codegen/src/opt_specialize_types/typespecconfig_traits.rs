//! # TypeSpecConfig - Trait Implementations
//!
//! This module contains trait implementations for `TypeSpecConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypeSpecConfig;

impl Default for TypeSpecConfig {
    fn default() -> Self {
        TypeSpecConfig {
            max_specializations: 64,
            min_call_count: 2,
        }
    }
}
