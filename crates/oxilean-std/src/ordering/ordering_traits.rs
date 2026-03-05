//! # Ordering - Trait Implementations
//!
//! This module contains trait implementations for `Ordering`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Ordering;
use std::fmt;

impl std::fmt::Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ordering::Less => write!(f, "lt"),
            Ordering::Equal => write!(f, "eq"),
            Ordering::Greater => write!(f, "gt"),
        }
    }
}
