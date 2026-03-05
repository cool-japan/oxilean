//! # SpecializationConfig - Trait Implementations
//!
//! This module contains trait implementations for `SpecializationConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SpecializationConfig;

impl Default for SpecializationConfig {
    fn default() -> Self {
        SpecializationConfig {
            max_specializations: 8,
            specialize_closures: true,
            specialize_numerics: true,
            size_threshold: 200,
            growth_factor: 3.0,
            allow_recursive: true,
            specialize_type_params: true,
            max_recursive_depth: 3,
        }
    }
}
