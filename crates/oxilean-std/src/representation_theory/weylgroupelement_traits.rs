//! # WeylGroupElement - Trait Implementations
//!
//! This module contains trait implementations for `WeylGroupElement`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WeylGroupElement;
use std::fmt;

impl std::fmt::Display for WeylGroupElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.reduced_word.is_empty() {
            write!(f, "e")
        } else {
            let parts: Vec<String> = self
                .reduced_word
                .iter()
                .map(|i| format!("s{}", i + 1))
                .collect();
            write!(f, "{}", parts.join(""))
        }
    }
}
