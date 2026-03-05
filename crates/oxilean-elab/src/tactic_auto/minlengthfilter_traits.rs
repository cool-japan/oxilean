//! # MinLengthFilter - Trait Implementations
//!
//! This module contains trait implementations for `MinLengthFilter`.
//!
//! ## Implemented Traits
//!
//! - `HintFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};
use std::fmt;

use super::functions::HintFilter;
use super::types::MinLengthFilter;

impl HintFilter for MinLengthFilter {
    fn filter_name(&self) -> &'static str {
        "min_length"
    }
    fn accept(&self, hint: &str, _goal: &Goal) -> bool {
        hint.len() >= self.min_len
    }
}
