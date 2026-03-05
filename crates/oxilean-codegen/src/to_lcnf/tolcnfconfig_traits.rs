//! # ToLcnfConfig - Trait Implementations
//!
//! This module contains trait implementations for `ToLcnfConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ToLcnfConfig;

impl Default for ToLcnfConfig {
    fn default() -> Self {
        ToLcnfConfig {
            erase_proofs: true,
            erase_types: true,
            lambda_lift: true,
            max_inline_size: 8,
            debug_names: false,
        }
    }
}
