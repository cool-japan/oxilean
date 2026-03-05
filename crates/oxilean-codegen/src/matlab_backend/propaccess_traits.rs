//! # PropAccess - Trait Implementations
//!
//! This module contains trait implementations for `PropAccess`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PropAccess;
use std::fmt;

impl fmt::Display for PropAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropAccess::Public => write!(f, "public"),
            PropAccess::Protected => write!(f, "protected"),
            PropAccess::Private => write!(f, "private"),
        }
    }
}
