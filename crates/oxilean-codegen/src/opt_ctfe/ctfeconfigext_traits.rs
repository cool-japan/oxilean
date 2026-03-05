//! # CtfeConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `CtfeConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeConfigExt;

impl Default for CtfeConfigExt {
    fn default() -> Self {
        Self {
            fuel: 10_000,
            max_depth: 256,
            max_list_size: 10_000,
            max_string_size: 1_000_000,
            enable_memoization: true,
            enable_logging: false,
            replace_calls: true,
            propagate_constants: true,
            fold_arithmetic: true,
            fold_boolean: true,
            fold_string: true,
            fold_comparison: true,
        }
    }
}
