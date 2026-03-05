//! # SurrealGame - Trait Implementations
//!
//! This module contains trait implementations for `SurrealGame`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SurrealGame;
use std::fmt;

impl std::fmt::Display for SurrealGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l: Vec<String> = self.left_options.iter().map(|x| x.to_string()).collect();
        let r: Vec<String> = self.right_options.iter().map(|x| x.to_string()).collect();
        write!(f, "{{ {} | {} }}", l.join(", "), r.join(", "))
    }
}
