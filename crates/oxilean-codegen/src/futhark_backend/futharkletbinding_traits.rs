//! # FutharkLetBinding - Trait Implementations
//!
//! This module contains trait implementations for `FutharkLetBinding`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkLetBinding;
use std::fmt;

impl std::fmt::Display for FutharkLetBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ty) = &self.type_ann {
            write!(
                f,
                "let ({}: {}) = {} in\n{}",
                self.pattern, ty, self.value, self.body
            )
        } else {
            write!(f, "let {} = {} in\n{}", self.pattern, self.value, self.body)
        }
    }
}
