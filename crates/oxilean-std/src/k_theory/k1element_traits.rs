//! # K1Element - Trait Implementations
//!
//! This module contains trait implementations for `K1Element`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::K1Element;
use std::fmt;

impl std::fmt::Display for K1Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "K1[{}]({}x{})",
            self.label, self.matrix_size, self.matrix_size
        )
    }
}
