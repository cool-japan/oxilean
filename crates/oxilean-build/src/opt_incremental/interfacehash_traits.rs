//! # InterfaceHash - Trait Implementations
//!
//! This module contains trait implementations for `InterfaceHash`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InterfaceHash;

impl std::fmt::Display for InterfaceHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterfaceHash({:016x})", self.0)
    }
}
