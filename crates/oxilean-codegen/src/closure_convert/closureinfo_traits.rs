//! # ClosureInfo - Trait Implementations
//!
//! This module contains trait implementations for `ClosureInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ClosureInfo;
use std::fmt;

impl fmt::Display for ClosureInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClosureInfo {{ arity={}, captured={}, escaping={}, side_effects={} }}",
            self.arity,
            self.free_vars.len(),
            self.is_escaping,
            self.has_side_effects,
        )
    }
}
