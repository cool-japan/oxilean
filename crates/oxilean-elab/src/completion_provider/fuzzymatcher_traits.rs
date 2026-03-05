//! # FuzzyMatcher - Trait Implementations
//!
//! This module contains trait implementations for `FuzzyMatcher`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FuzzyMatcher, FuzzyWeights};
use std::fmt;

impl Default for FuzzyMatcher {
    fn default() -> Self {
        FuzzyMatcher {
            weights: FuzzyWeights::default(),
            case_insensitive: true,
        }
    }
}
