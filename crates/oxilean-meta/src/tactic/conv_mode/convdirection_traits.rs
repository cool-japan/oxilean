//! # ConvDirection - Trait Implementations
//!
//! This module contains trait implementations for `ConvDirection`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConvDirection;
use std::fmt;

impl fmt::Display for ConvDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvDirection::Left => write!(f, "left"),
            ConvDirection::Right => write!(f, "right"),
            ConvDirection::Arg(n) => write!(f, "arg {}", n),
            ConvDirection::Fun => write!(f, "fun"),
            ConvDirection::Ext => write!(f, "ext"),
        }
    }
}
