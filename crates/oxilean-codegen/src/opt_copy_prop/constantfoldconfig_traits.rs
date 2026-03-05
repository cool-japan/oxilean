//! # ConstantFoldConfig - Trait Implementations
//!
//! This module contains trait implementations for `ConstantFoldConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ConstantFoldConfig;

impl Default for ConstantFoldConfig {
    fn default() -> Self {
        ConstantFoldConfig {
            fold_nat_arith: true,
            fold_bool_ops: true,
            max_nat_value: 1 << 32,
        }
    }
}
