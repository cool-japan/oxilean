//! # FuzzyWeights - Trait Implementations
//!
//! This module contains trait implementations for `FuzzyWeights`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FuzzyWeights;
use std::fmt;

impl Default for FuzzyWeights {
    fn default() -> Self {
        FuzzyWeights {
            prefix_bonus: 5.0,
            word_start_bonus: 3.0,
            consecutive_bonus: 2.0,
            gap_penalty: -0.5,
        }
    }
}
