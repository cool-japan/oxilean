//! # CoqAttribute - Trait Implementations
//!
//! This module contains trait implementations for `CoqAttribute`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqAttribute;
use std::fmt;

impl std::fmt::Display for CoqAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqAttribute::Global => write!(f, "#[global]"),
            CoqAttribute::Local => write!(f, "#[local]"),
            CoqAttribute::Export => write!(f, "#[export]"),
            CoqAttribute::Transparent => write!(f, "Transparent"),
            CoqAttribute::Opaque => write!(f, "Opaque"),
            CoqAttribute::Polymorphic => write!(f, "#[universes(polymorphic)]"),
            CoqAttribute::Monomorphic => write!(f, "#[universes(monomorphic)]"),
            CoqAttribute::Program => write!(f, "Program"),
            CoqAttribute::Equations => write!(f, "Equations"),
            CoqAttribute::Custom(s) => write!(f, "#[{}]", s),
        }
    }
}
