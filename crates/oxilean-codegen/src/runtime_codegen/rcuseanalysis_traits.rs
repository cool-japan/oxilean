//! # RcUseAnalysis - Trait Implementations
//!
//! This module contains trait implementations for `RcUseAnalysis`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::RcUseAnalysis;

impl Default for RcUseAnalysis {
    fn default() -> Self {
        Self::new()
    }
}
