//! # NamePrefixFilter - Trait Implementations
//!
//! This module contains trait implementations for `NamePrefixFilter`.
//!
//! ## Implemented Traits
//!
//! - `HintFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};
use std::fmt;

use super::functions::HintFilter;
use super::types::NamePrefixFilter;

impl HintFilter for NamePrefixFilter {
    fn filter_name(&self) -> &'static str {
        "name_prefix"
    }
    fn accept(&self, hint: &str, _goal: &Goal) -> bool {
        hint.starts_with(&self.prefix)
    }
}
