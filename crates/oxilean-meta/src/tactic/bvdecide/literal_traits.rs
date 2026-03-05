//! # Literal - Trait Implementations
//!
//! This module contains trait implementations for `Literal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Literal;
use std::fmt;

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.polarity {
            write!(f, "{}", self.var)
        } else {
            write!(f, "~{}", self.var)
        }
    }
}
