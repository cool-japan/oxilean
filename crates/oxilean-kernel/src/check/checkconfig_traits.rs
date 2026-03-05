//! # CheckConfig - Trait Implementations
//!
//! This module contains trait implementations for `CheckConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CheckConfig;

impl Default for CheckConfig {
    fn default() -> Self {
        CheckConfig {
            check_type_is_sort: true,
            check_value_type: true,
            check_no_free_vars: true,
            max_depth: 0,
            allow_axioms: true,
        }
    }
}
