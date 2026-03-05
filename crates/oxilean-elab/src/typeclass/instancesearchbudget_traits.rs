//! # InstanceSearchBudget - Trait Implementations
//!
//! This module contains trait implementations for `InstanceSearchBudget`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InstanceSearchBudget;
use std::fmt;

impl Default for InstanceSearchBudget {
    fn default() -> Self {
        Self {
            max_candidates: 128,
            max_chain_length: 8,
            max_goals: 64,
        }
    }
}
