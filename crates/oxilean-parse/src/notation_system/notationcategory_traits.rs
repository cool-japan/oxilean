//! # NotationCategory - Trait Implementations
//!
//! This module contains trait implementations for `NotationCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationCategory;

impl std::fmt::Display for NotationCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationCategory::Term => write!(f, "term"),
            NotationCategory::Tactic => write!(f, "tactic"),
            NotationCategory::Command => write!(f, "command"),
            NotationCategory::Custom(s) => write!(f, "{}", s),
        }
    }
}
