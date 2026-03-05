//! # Visibility - Trait Implementations
//!
//! This module contains trait implementations for `Visibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Visibility;
use std::fmt;

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Private => write!(f, "private"),
            Visibility::Internal => write!(f, "internal"),
            Visibility::External => write!(f, "external"),
        }
    }
}
