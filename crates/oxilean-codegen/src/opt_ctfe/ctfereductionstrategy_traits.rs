//! # CtfeReductionStrategy - Trait Implementations
//!
//! This module contains trait implementations for `CtfeReductionStrategy`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeReductionStrategy;
use std::fmt;

impl std::fmt::Display for CtfeReductionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtfeReductionStrategy::CallByValue => write!(f, "cbv"),
            CtfeReductionStrategy::CallByName => write!(f, "cbn"),
            CtfeReductionStrategy::CallByNeed => write!(f, "lazy"),
            CtfeReductionStrategy::Normal => write!(f, "normal"),
        }
    }
}
