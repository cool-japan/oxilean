//! # CopyPropConfig - Trait Implementations
//!
//! This module contains trait implementations for `CopyPropConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CopyPropConfig;
use std::fmt;

impl Default for CopyPropConfig {
    fn default() -> Self {
        CopyPropConfig {
            max_chain_depth: 16,
            fold_literals: true,
        }
    }
}

impl fmt::Display for CopyPropConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CopyPropConfig {{ max_chain_depth={}, fold_literals={} }}",
            self.max_chain_depth, self.fold_literals
        )
    }
}
