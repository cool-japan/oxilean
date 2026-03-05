//! # StructuralRecParam - Trait Implementations
//!
//! This module contains trait implementations for `StructuralRecParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StructuralRecParam;
use std::fmt;

impl fmt::Display for StructuralRecParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "param '{}' (idx {}) : {}",
            self.param_name, self.param_idx, self.inductive_type
        )
    }
}
