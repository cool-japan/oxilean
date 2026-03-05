//! # SignExpansionEncoder - Trait Implementations
//!
//! This module contains trait implementations for `SignExpansionEncoder`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SignExpansionEncoder;
use std::fmt;

impl std::fmt::Display for SignExpansionEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .signs
            .iter()
            .map(|&b| if b { '+' } else { '-' })
            .collect();
        if s.is_empty() {
            write!(f, "ε (zero)")
        } else {
            write!(f, "{}", s)
        }
    }
}
