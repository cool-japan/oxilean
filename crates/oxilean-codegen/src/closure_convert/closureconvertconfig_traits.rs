//! # ClosureConvertConfig - Trait Implementations
//!
//! This module contains trait implementations for `ClosureConvertConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ClosureConvertConfig;

impl Default for ClosureConvertConfig {
    fn default() -> Self {
        ClosureConvertConfig {
            defunctionalize: true,
            stack_alloc_non_escaping: true,
            max_inline_captures: 4,
            merge_closures: true,
        }
    }
}
