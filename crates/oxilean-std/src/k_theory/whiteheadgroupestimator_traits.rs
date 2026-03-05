//! # WhiteheadGroupEstimator - Trait Implementations
//!
//! This module contains trait implementations for `WhiteheadGroupEstimator`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhiteheadGroupEstimator;
use std::fmt;

impl std::fmt::Display for WhiteheadGroupEstimator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
