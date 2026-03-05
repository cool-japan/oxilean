//! # MemEffect - Trait Implementations
//!
//! This module contains trait implementations for `MemEffect`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemEffect;
use std::fmt;

impl std::fmt::Display for MemEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemEffect::ReadNone => write!(f, "readnone"),
            MemEffect::ReadOnly => write!(f, "readonly"),
            MemEffect::WriteOnly => write!(f, "writeonly"),
            MemEffect::ReadWrite => write!(f, "readwrite"),
            MemEffect::ArgMemOnly => write!(f, "argmemonly"),
            MemEffect::InaccessibleMemOnly => write!(f, "inaccessiblemem_only"),
        }
    }
}
