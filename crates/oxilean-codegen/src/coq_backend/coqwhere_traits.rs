//! # CoqWhere - Trait Implementations
//!
//! This module contains trait implementations for `CoqWhere`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqWhere;
use std::fmt;

impl std::fmt::Display for CoqWhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binds: Vec<String> = self
            .vars
            .iter()
            .map(|(n, t)| format!("{} := {}", n, t))
            .collect();
        write!(f, "where\n  {}", binds.join("\n  and "))
    }
}
