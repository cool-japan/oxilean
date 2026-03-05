//! # MonoRelation - Trait Implementations
//!
//! This module contains trait implementations for `MonoRelation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MonoRelation;
use std::fmt;

impl fmt::Display for MonoRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MonoRelation::Le => write!(f, "<="),
            MonoRelation::Lt => write!(f, "<"),
            MonoRelation::Ge => write!(f, ">="),
            MonoRelation::Gt => write!(f, ">"),
            MonoRelation::Dvd => write!(f, "|"),
            MonoRelation::Subset => write!(f, "subset"),
            MonoRelation::Custom(name) => write!(f, "{}", name),
        }
    }
}
