//! # MemLocation - Trait Implementations
//!
//! This module contains trait implementations for `MemLocation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemLocation;
use std::fmt;

impl std::fmt::Display for MemLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemLocation::Stack(id) => write!(f, "stack#{}", id),
            MemLocation::Heap(id) => write!(f, "heap#{}", id),
            MemLocation::Global(name) => write!(f, "global:{}", name),
            MemLocation::Field(base, field) => write!(f, "{}.{}", base, field),
            MemLocation::Index(base, i) => write!(f, "{}[{}]", base, i),
            MemLocation::Unknown => write!(f, "??"),
        }
    }
}
