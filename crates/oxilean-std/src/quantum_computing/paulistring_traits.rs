//! # PauliString - Trait Implementations
//!
//! This module contains trait implementations for `PauliString`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PauliString;
use std::fmt;

impl std::fmt::Display for PauliString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_str = if self.sign < 0 { "-" } else { "+" };
        let paulis_str: String = self.paulis.iter().map(|p| p.to_string()).collect();
        write!(f, "{}{}", sign_str, paulis_str)
    }
}
