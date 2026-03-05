//! # ClassConstraint - Trait Implementations
//!
//! This module contains trait implementations for `ClassConstraint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ClassConstraint;
use std::fmt;

impl std::fmt::Display for ClassConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.class, self.ty)
    }
}
