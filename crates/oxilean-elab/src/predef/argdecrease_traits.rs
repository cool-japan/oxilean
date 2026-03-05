//! # ArgDecrease - Trait Implementations
//!
//! This module contains trait implementations for `ArgDecrease`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArgDecrease;
use std::fmt;

impl fmt::Display for ArgDecrease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgDecrease::Decreasing => write!(f, "<"),
            ArgDecrease::Equal => write!(f, "="),
            ArgDecrease::Unknown => write!(f, "?"),
            ArgDecrease::Missing => write!(f, "-"),
        }
    }
}
