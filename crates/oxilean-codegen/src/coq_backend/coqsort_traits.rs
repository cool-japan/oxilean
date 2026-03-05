//! # CoqSort - Trait Implementations
//!
//! This module contains trait implementations for `CoqSort`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqSort;
use std::fmt;

impl fmt::Display for CoqSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoqSort::Prop => write!(f, "Prop"),
            CoqSort::Set => write!(f, "Set"),
            CoqSort::Type(None) => write!(f, "Type"),
            CoqSort::Type(Some(i)) => write!(f, "Type@{{u{}}}", i),
        }
    }
}
