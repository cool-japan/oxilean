//! # FredholmData - Trait Implementations
//!
//! This module contains trait implementations for `FredholmData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FredholmData;
use std::fmt;

impl std::fmt::Display for FredholmData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fredholm[{}](ker={}, coker={}, index={})",
            self.label,
            self.kernel_dim,
            self.cokernel_dim,
            self.index()
        )
    }
}
