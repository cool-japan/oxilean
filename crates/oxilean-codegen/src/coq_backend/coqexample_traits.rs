//! # CoqExample - Trait Implementations
//!
//! This module contains trait implementations for `CoqExample`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqExample;
use std::fmt;

impl std::fmt::Display for CoqExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Example {} : {}.\n{}",
            self.name,
            self.statement,
            self.proof.emit()
        )
    }
}
