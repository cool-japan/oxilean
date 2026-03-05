//! # EscapeAnalysis - Trait Implementations
//!
//! This module contains trait implementations for `EscapeAnalysis`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::EscapeAnalysis;

impl Default for EscapeAnalysis {
    fn default() -> Self {
        Self::new()
    }
}
