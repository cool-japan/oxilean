//! # CoqUniverse - Trait Implementations
//!
//! This module contains trait implementations for `CoqUniverse`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqUniverse;
use std::fmt;

impl std::fmt::Display for CoqUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqUniverse::Prop => write!(f, "Prop"),
            CoqUniverse::Set => write!(f, "Set"),
            CoqUniverse::Type(None) => write!(f, "Type"),
            CoqUniverse::Type(Some(i)) => write!(f, "Type@{{{}}}", i),
            CoqUniverse::SProp => write!(f, "SProp"),
        }
    }
}
