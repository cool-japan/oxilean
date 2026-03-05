//! # DceConfig - Trait Implementations
//!
//! This module contains trait implementations for `DceConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::DceConfig;
use std::fmt;

impl Default for DceConfig {
    fn default() -> Self {
        DceConfig {
            eliminate_unused_lets: true,
            eliminate_unreachable_alts: true,
            propagate_constants: true,
            propagate_copies: true,
            fold_known_calls: true,
            max_iterations: 10,
        }
    }
}

impl fmt::Display for DceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DceConfig {{ unused_lets={}, unreachable_alts={}, const_prop={}, \
             copy_prop={}, fold_known={}, max_iter={} }}",
            self.eliminate_unused_lets,
            self.eliminate_unreachable_alts,
            self.propagate_constants,
            self.propagate_copies,
            self.fold_known_calls,
            self.max_iterations,
        )
    }
}
