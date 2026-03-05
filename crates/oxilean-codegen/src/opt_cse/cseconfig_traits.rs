//! # CseConfig - Trait Implementations
//!
//! This module contains trait implementations for `CseConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CseConfig;
use std::fmt;

impl Default for CseConfig {
    fn default() -> Self {
        CseConfig {
            max_expr_size: 20,
            track_calls: false,
            max_candidates: 1000,
            pure_functions: vec![],
        }
    }
}

impl fmt::Display for CseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CseConfig {{ max_expr_size={}, track_calls={}, max_candidates={} }}",
            self.max_expr_size, self.track_calls, self.max_candidates
        )
    }
}
