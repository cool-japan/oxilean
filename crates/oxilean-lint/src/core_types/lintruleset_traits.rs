//! # LintRuleSet - Trait Implementations
//!
//! This module contains trait implementations for `LintRuleSet`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintRuleSet;
use std::fmt;

impl std::fmt::Display for LintRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LintRuleSet[{}]({} rules)", self.name, self.ids.len())
    }
}
